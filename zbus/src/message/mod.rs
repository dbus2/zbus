//! D-Bus Message.
use std::{fmt, io::Cursor, num::NonZeroU32};

#[cfg(unix)]
use std::{
    os::unix::io::{AsRawFd, RawFd},
    sync::{Arc, RwLock},
};

use static_assertions::assert_impl_all;
use zbus_names::{ErrorName, InterfaceName, MemberName};

#[cfg(unix)]
use crate::OwnedFd;
use crate::{
    utils::padding_for_8_bytes,
    zvariant::{EncodingContext, ObjectPath, Signature, Type as VariantType},
    Error, Result,
};

mod builder;
pub use builder::Builder;

mod field;
use field::{Field, FieldCode};

mod fields;
use fields::{Fields, QuickFields};

pub(crate) mod header;
use header::MIN_MESSAGE_SIZE;
pub use header::{EndianSig, Flags, Header, PrimaryHeader, Type, NATIVE_ENDIAN_SIG};

#[cfg(unix)]
const LOCK_PANIC_MSG: &str = "lock poisoned";

macro_rules! dbus_context {
    ($n_bytes_before: expr) => {
        EncodingContext::<byteorder::NativeEndian>::new_dbus($n_bytes_before)
    };
}

#[cfg(unix)]
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Fds {
    Owned(Vec<OwnedFd>),
    Raw(Vec<RawFd>),
}

/// A position in the stream of [`Message`] objects received by a single [`zbus::Connection`].
///
/// Note: the relative ordering of values obtained from distinct [`zbus::Connection`] objects is
/// not specified; only sequence numbers originating from the same connection should be compared.
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Sequence {
    recv_seq: u64,
}

impl Sequence {
    /// A sequence number that is higher than any other; used by errors that terminate a stream.
    pub(crate) const LAST: Self = Self { recv_seq: u64::MAX };
}

/// A D-Bus Message.
///
/// The content of the message are stored in serialized format. To deserialize the body of the
/// message, use the [`body`] method. You may also access the header and other details with the
/// various other getters.
///
/// Also provided are constructors for messages of different types. These will mainly be useful for
/// very advanced use cases as typically you will want to create a message for immediate dispatch
/// and hence use the API provided by [`Connection`], even when using the low-level API.
///
/// **Note**: The message owns the received FDs and will close them when dropped. You can call
/// [`take_fds`] after deserializing to `RawFD` using [`body`] if you want to take the ownership.
///
/// [`body`]: #method.body
/// [`take_fds`]: #method.take_fds
/// [`Connection`]: struct.Connection#method.call_method
#[derive(Clone)]
pub struct Message {
    pub(crate) primary_header: PrimaryHeader,
    pub(crate) quick_fields: QuickFields,
    pub(crate) bytes: Vec<u8>,
    pub(crate) body_offset: usize,
    #[cfg(unix)]
    pub(crate) fds: Arc<RwLock<Fds>>,
    pub(crate) recv_seq: Sequence,
}

assert_impl_all!(Message: Send, Sync, Unpin);

// TODO: Handle non-native byte order: https://github.com/dbus2/zbus/issues/19
impl Message {
    /// Create a builder for message of type [`Type::MethodCall`].
    pub fn method<'b, 'p: 'b, 'm: 'b, P, M>(path: P, method_name: M) -> Result<Builder<'b>>
    where
        P: TryInto<ObjectPath<'p>>,
        M: TryInto<MemberName<'m>>,
        P::Error: Into<Error>,
        M::Error: Into<Error>,
    {
        Builder::method_call(path, method_name)
    }

    /// Create a builder for message of type [`Type::Signal`].
    pub fn signal<'b, 'p: 'b, 'i: 'b, 'm: 'b, P, I, M>(
        path: P,
        iface: I,
        signal_name: M,
    ) -> Result<Builder<'b>>
    where
        P: TryInto<ObjectPath<'p>>,
        I: TryInto<InterfaceName<'i>>,
        M: TryInto<MemberName<'m>>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
        M::Error: Into<Error>,
    {
        Builder::signal(path, iface, signal_name)
    }

    /// Create a builder for message of type [`Type::MethodReturn`].
    pub fn method_reply(call: &Self) -> Result<Builder<'_>> {
        Builder::method_return(&call.header())
    }

    /// Create a builder for message of type [`Type::Error`].
    pub fn method_error<'b, 'e: 'b, E>(call: &Self, name: E) -> Result<Builder<'b>>
    where
        E: TryInto<ErrorName<'e>>,
        E::Error: Into<Error>,
    {
        Builder::error(&call.header(), name)
    }

    /// Create a message from bytes.
    ///
    /// The `fds` parameter is only available on unix. It specifies the file descriptors that
    /// accompany the message. On the wire, values of the UNIX_FD types store the index of the
    /// corresponding file descriptor in this vector. Passing an empty vector on a message that
    /// has UNIX_FD will result in an error.
    ///
    /// **Note:** Since the constructed message is not construct by zbus, the receive sequence,
    /// which can be acquired from [`Message::recv_position`], is not applicable and hence set
    /// to `0`.
    ///
    /// # Safety
    ///
    /// This method is unsafe as bytes may have an invalid encoding.
    pub unsafe fn from_bytes(bytes: Vec<u8>, #[cfg(unix)] fds: Vec<OwnedFd>) -> Result<Self> {
        Self::from_raw_parts(
            bytes,
            #[cfg(unix)]
            fds,
            0,
        )
    }

    /// Create a message from its full contents
    pub(crate) fn from_raw_parts(
        bytes: Vec<u8>,
        #[cfg(unix)] fds: Vec<OwnedFd>,
        recv_seq: u64,
    ) -> Result<Self> {
        if EndianSig::try_from(bytes[0])? != NATIVE_ENDIAN_SIG {
            return Err(Error::IncorrectEndian);
        }

        let (primary_header, fields_len) = PrimaryHeader::read(&bytes)?;
        let (header, _) = zvariant::from_slice(&bytes, dbus_context!(0))?;
        #[cfg(unix)]
        let fds = Arc::new(RwLock::new(Fds::Owned(fds)));

        let header_len = MIN_MESSAGE_SIZE + fields_len as usize;
        let body_offset = header_len + padding_for_8_bytes(header_len);
        let quick_fields = QuickFields::new(&bytes, &header)?;

        Ok(Self {
            primary_header,
            quick_fields,
            bytes,
            body_offset,
            #[cfg(unix)]
            fds,
            recv_seq: Sequence { recv_seq },
        })
    }

    /// Take ownership of the associated file descriptors in the message.
    ///
    /// When a message is received over a AF_UNIX socket, it may contain associated FDs. To prevent
    /// the message from closing those FDs on drop, call this method that returns all the received
    /// FDs with their ownership.
    ///
    /// This function is Unix-specific.
    ///
    /// Note: the message will continue to reference the files, so you must keep them open for as
    /// long as the message itself.
    #[cfg(unix)]
    pub fn take_fds(&self) -> Vec<OwnedFd> {
        let mut fds_lock = self.fds.write().expect(LOCK_PANIC_MSG);
        if let Fds::Owned(ref mut fds) = *fds_lock {
            // From now on, it's the caller responsibility to close the fds
            let fds = std::mem::take(&mut *fds);
            *fds_lock = Fds::Raw(fds.iter().map(|fd| fd.as_raw_fd()).collect());
            fds
        } else {
            vec![]
        }
    }

    /// The signature of the body.
    ///
    /// **Note:** While zbus treats multiple arguments as a struct (to allow you to use the tuple
    /// syntax), D-Bus does not. Since this method gives you the signature expected on the wire by
    /// D-Bus, the trailing and leading STRUCT signature parenthesis will not be present in case of
    /// multiple arguments.
    pub fn body_signature(&self) -> Option<Signature<'_>> {
        self.quick_fields.signature(self)
    }

    pub fn primary_header(&self) -> &PrimaryHeader {
        &self.primary_header
    }

    pub(crate) fn modify_primary_header<F>(&mut self, mut modifier: F) -> Result<()>
    where
        F: FnMut(&mut PrimaryHeader) -> Result<()>,
    {
        modifier(&mut self.primary_header)?;

        let mut cursor = Cursor::new(&mut self.bytes);
        zvariant::to_writer(&mut cursor, dbus_context!(0), &self.primary_header)
            .map(|_| ())
            .map_err(Error::from)
    }

    /// The message header.
    ///
    /// Note: This method does not deserialize the header but it does currently allocate so its not
    /// zero-cost. While the allocation is small and will hopefully be removed in the future, it's
    /// best to keep the header around if you need to access it a lot.
    pub fn header(&self) -> Header<'_> {
        let mut fields = Fields::new();
        let quick_fields = &self.quick_fields;
        if let Some(p) = quick_fields.path(self) {
            fields.add(Field::Path(p));
        }
        if let Some(i) = quick_fields.interface(self) {
            fields.add(Field::Interface(i));
        }
        if let Some(m) = quick_fields.member(self) {
            fields.add(Field::Member(m));
        }
        if let Some(e) = quick_fields.error_name(self) {
            fields.add(Field::ErrorName(e));
        }
        if let Some(r) = quick_fields.reply_serial() {
            fields.add(Field::ReplySerial(r));
        }
        if let Some(d) = quick_fields.destination(self) {
            fields.add(Field::Destination(d));
        }
        if let Some(s) = quick_fields.sender(self) {
            fields.add(Field::Sender(s));
        }
        if let Some(s) = quick_fields.signature(self) {
            fields.add(Field::Signature(s));
        }
        if let Some(u) = quick_fields.unix_fds() {
            fields.add(Field::UnixFDs(u));
        }

        Header::new(self.primary_header.clone(), fields)
    }

    /// The message type.
    pub fn message_type(&self) -> Type {
        self.primary_header.msg_type()
    }

    /// The object to send a call to, or the object a signal is emitted from.
    #[deprecated(note = "Use `Message::header` with `message::Header::path` instead")]
    pub fn path(&self) -> Option<ObjectPath<'_>> {
        self.quick_fields.path(self)
    }

    /// The interface to invoke a method call on, or that a signal is emitted from.
    #[deprecated(note = "Use `Message::header` with `message::Header::interface` instead")]
    pub fn interface(&self) -> Option<InterfaceName<'_>> {
        self.quick_fields.interface(self)
    }

    /// The member, either the method name or signal name.
    #[deprecated(note = "Use `Message::header` with `message::Header::member` instead")]
    pub fn member(&self) -> Option<MemberName<'_>> {
        self.quick_fields.member(self)
    }

    /// The serial number of the message this message is a reply to.
    #[deprecated(note = "Use `Message::header` with `message::Header::reply_serial` instead")]
    pub fn reply_serial(&self) -> Option<NonZeroU32> {
        self.quick_fields.reply_serial()
    }

    /// Deserialize the body (without checking signature matching).
    pub fn body_unchecked<'d, 'm: 'd, B>(&'m self) -> Result<B>
    where
        B: serde::de::Deserialize<'d> + VariantType,
    {
        {
            #[cfg(unix)]
            {
                zvariant::from_slice_fds(
                    &self.bytes[self.body_offset..],
                    Some(&self.fds()),
                    dbus_context!(0),
                )
            }
            #[cfg(not(unix))]
            {
                zvariant::from_slice(&self.bytes[self.body_offset..], dbus_context!(0))
            }
        }
        .map_err(Error::from)
        .map(|b| b.0)
    }

    /// Deserialize the body using the contained signature.
    ///
    /// # Example
    ///
    /// ```
    /// # use zbus::message::Message;
    /// # (|| -> zbus::Result<()> {
    /// let send_body = (7i32, (2i32, "foo"), vec!["bar"]);
    /// let mut message = Message::method("/", "ping")?
    ///     .destination("zbus.test")?
    ///     .interface("zbus.test")?
    ///     .build(&send_body)?;
    /// let conn = zbus::blocking::Connection::session()?;
    /// conn.inner().assign_serial_num(&mut message)?;
    /// let body : zbus::zvariant::Structure = message.body()?;
    /// let fields = body.fields();
    /// assert!(matches!(fields[0], zvariant::Value::I32(7)));
    /// assert!(matches!(fields[1], zvariant::Value::Structure(_)));
    /// assert!(matches!(fields[2], zvariant::Value::Array(_)));
    ///
    /// let reply_msg = Message::method_reply(&message)?.build(&body)?;
    /// let reply_value : (i32, (i32, &str), Vec<String>) = reply_msg.body()?;
    ///
    /// assert_eq!(reply_value.0, 7);
    /// assert_eq!(reply_value.2.len(), 1);
    /// # Ok(()) })().unwrap()
    /// ```
    pub fn body<'d, 'm: 'd, B>(&'m self) -> Result<B>
    where
        B: zvariant::DynamicDeserialize<'d>,
    {
        let body_sig = self
            .body_signature()
            .unwrap_or_else(|| Signature::from_static_str_unchecked(""));

        {
            #[cfg(unix)]
            {
                zvariant::from_slice_fds_for_dynamic_signature(
                    &self.bytes[self.body_offset..],
                    Some(&self.fds()),
                    dbus_context!(0),
                    &body_sig,
                )
            }
            #[cfg(not(unix))]
            {
                zvariant::from_slice_for_dynamic_signature(
                    &self.bytes[self.body_offset..],
                    dbus_context!(0),
                    &body_sig,
                )
            }
        }
        .map_err(Error::from)
        .map(|b| b.0)
    }

    #[cfg(unix)]
    pub(crate) fn fds(&self) -> Vec<RawFd> {
        match &*self.fds.read().expect(LOCK_PANIC_MSG) {
            Fds::Raw(fds) => fds.clone(),
            Fds::Owned(fds) => fds.iter().map(|f| f.as_raw_fd()).collect(),
        }
    }

    /// Get a reference to the byte encoding of the message.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get a reference to the byte encoding of the body of the message.
    pub fn body_as_bytes(&self) -> Result<&[u8]> {
        Ok(&self.bytes[self.body_offset..])
    }

    /// Get the receive ordering of a message.
    ///
    /// This may be used to identify how two events were ordered on the bus.  It only produces a
    /// useful ordering for messages that were produced by the same [`zbus::Connection`].
    ///
    /// This is completely unrelated to the serial number on the message, which is set by the peer
    /// and might not be ordered at all.
    pub fn recv_position(&self) -> Sequence {
        self.recv_seq
    }

    pub(crate) fn set_serial_num(&mut self, serial_num: NonZeroU32) -> Result<()> {
        self.modify_primary_header(|primary| {
            primary.set_serial_num(serial_num);
            Ok(())
        })?;
        self.primary_header.set_serial_num(serial_num);

        Ok(())
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = f.debug_struct("Msg");
        let h = self.header();
        msg.field("type", &h.message_type());
        if let Some(sender) = h.sender() {
            msg.field("sender", &sender);
        }
        if let Some(serial) = h.reply_serial() {
            msg.field("reply-serial", &serial);
        }
        if let Some(path) = h.path() {
            msg.field("path", &path);
        }
        if let Some(iface) = h.interface() {
            msg.field("iface", &iface);
        }
        if let Some(member) = h.member() {
            msg.field("member", &member);
        }
        if let Some(s) = self.body_signature() {
            msg.field("body", &s);
        }
        #[cfg(unix)]
        {
            let fds = self.fds();
            if !fds.is_empty() {
                msg.field("fds", &fds);
            }
        }
        msg.finish()
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = self.header();
        let (ty, error_name, sender, member) = (
            header.message_type(),
            header.error_name(),
            header.sender(),
            header.member(),
        );

        match ty {
            Type::MethodCall => {
                write!(f, "Method call")?;
                if let Some(m) = member {
                    write!(f, " {m}")?;
                }
            }
            Type::MethodReturn => {
                write!(f, "Method return")?;
            }
            Type::Error => {
                write!(f, "Error")?;
                if let Some(e) = error_name {
                    write!(f, " {e}")?;
                }

                let msg = self.body_unchecked::<&str>();
                if let Ok(msg) = msg {
                    write!(f, ": {msg}")?;
                }
            }
            Type::Signal => {
                write!(f, "Signal")?;
                if let Some(m) = member {
                    write!(f, " {m}")?;
                }
            }
        }

        if let Some(s) = sender {
            write!(f, " from {s}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use std::os::unix::io::AsRawFd;
    use test_log::test;
    #[cfg(unix)]
    use zvariant::Fd;

    #[cfg(unix)]
    use super::Fds;
    use super::Message;
    use crate::Error;

    #[test]
    fn test() {
        #[cfg(unix)]
        let stdout = std::io::stdout();
        let mut m = Message::method("/", "do")
            .unwrap()
            .sender(":1.72")
            .unwrap()
            .build(&(
                #[cfg(unix)]
                Fd::from(&stdout),
                "foo",
            ))
            .unwrap();
        m.set_serial_num(1.try_into().unwrap()).unwrap();
        assert_eq!(
            m.body_signature().unwrap().to_string(),
            if cfg!(unix) { "hs" } else { "s" }
        );
        #[cfg(unix)]
        assert_eq!(*m.fds.read().unwrap(), Fds::Raw(vec![stdout.as_raw_fd()]));

        let body: Result<u32, Error> = m.body();
        assert!(matches!(
            body.unwrap_err(),
            Error::Variant(zvariant::Error::SignatureMismatch { .. })
        ));

        assert_eq!(m.to_string(), "Method call do from :1.72");
        let r = Message::method_reply(&m)
            .unwrap()
            .build(&("all fine!"))
            .unwrap();
        assert_eq!(r.to_string(), "Method return");
        let e = Message::method_error(&m, "org.freedesktop.zbus.Error")
            .unwrap()
            .build(&("kaboom!", 32))
            .unwrap();
        assert_eq!(e.to_string(), "Error org.freedesktop.zbus.Error: kaboom!");
    }
}
