use std::{
    convert::{TryFrom, TryInto},
    fmt,
    io::Cursor,
    os::unix::io::{AsRawFd, RawFd},
    sync::{Arc, RwLock},
};

use static_assertions::assert_impl_all;
use zbus_names::{BusName, ErrorName, InterfaceName, MemberName, UniqueName};
use zvariant::{DynamicType, EncodingContext, ObjectPath, Signature, Type};

use crate::{
    utils::padding_for_8_bytes, EndianSig, Error, MessageField, MessageFieldCode, MessageFields,
    MessageHeader, MessagePrimaryHeader, MessageType, OwnedFd, Result, MIN_MESSAGE_SIZE,
    NATIVE_ENDIAN_SIG, PRIMARY_HEADER_SIZE,
};

const FIELDS_LEN_START_OFFSET: usize = 12;
const LOCK_PANIC_MSG: &str = "lock poisoned";

macro_rules! dbus_context {
    ($n_bytes_before: expr) => {
        EncodingContext::<byteorder::NativeEndian>::new_dbus($n_bytes_before)
    };
}

#[derive(Debug)]
struct MessageBuilder<'a, B> {
    ty: MessageType,
    body: &'a B,
    body_len: u32,
    reply_to: Option<MessageHeader<'a>>,
    fields: MessageFields<'a>,
}

impl<'a, B> MessageBuilder<'a, B>
where
    B: serde::ser::Serialize + DynamicType,
{
    fn new(ty: MessageType, sender: Option<UniqueName<'a>>, body: &'a B) -> Result<Self> {
        let ctxt = dbus_context!(0);
        let (body_len, fds_len) = zvariant::serialized_size_fds(ctxt, body)?;
        let body_len = u32::try_from(body_len).map_err(|_| Error::ExcessData)?;

        let mut fields = MessageFields::new();

        let mut signature = body.dynamic_signature();
        if !signature.is_empty() {
            if signature.starts_with(zvariant::STRUCT_SIG_START_STR) {
                // Remove leading and trailing STRUCT delimiters
                signature = signature.slice(1..signature.len() - 1);
            }
            fields.add(MessageField::Signature(signature));
        }
        if let Some(sender) = sender {
            fields.add(MessageField::Sender(sender));
        }

        if fds_len > 0 {
            fields.add(MessageField::UnixFDs(fds_len as u32));
        }

        Ok(Self {
            ty,
            body,
            body_len,
            fields,
            reply_to: None,
        })
    }

    fn build(self) -> Result<Message> {
        let MessageBuilder {
            ty,
            body,
            body_len,
            mut fields,
            reply_to,
        } = self;

        if let Some(reply_to) = reply_to.as_ref() {
            let serial = reply_to.primary().serial_num().ok_or(Error::MissingField)?;
            fields.add(MessageField::ReplySerial(*serial));

            if let Some(sender) = reply_to.sender()? {
                fields.add(MessageField::Destination(BusName::Unique(sender.clone())));
            }
        }

        let primary = MessagePrimaryHeader::new(ty, body_len);
        let header = MessageHeader::new(primary, fields);

        let ctxt = dbus_context!(0);
        // 1K for all the fields should be enough for most messages?
        let mut bytes: Vec<u8> =
            Vec::with_capacity(PRIMARY_HEADER_SIZE + 1024 + (body_len as usize));
        let mut cursor = Cursor::new(&mut bytes);

        zvariant::to_writer(&mut cursor, ctxt, &header)?;
        let (_, fds) = zvariant::to_writer_fds(&mut cursor, ctxt, body)?;

        Ok(Message {
            primary_header: header.into_primary(),
            bytes,
            fds: Arc::new(RwLock::new(Fds::Raw(fds))),
        })
    }

    fn set_reply_to(mut self, reply_to: &'a Message) -> Result<Self> {
        self.reply_to = Some(reply_to.header()?);
        Ok(self)
    }

    fn set_field(mut self, field: MessageField<'a>) -> Self {
        self.fields.add(field);
        self
    }

    fn reply(sender: Option<UniqueName<'a>>, reply_to: &'a Message, body: &'a B) -> Result<Self> {
        Self::new(MessageType::MethodReturn, sender, body)?.set_reply_to(reply_to)
    }

    fn error(
        sender: Option<UniqueName<'a>>,
        reply_to: &'a Message,
        error_name: ErrorName<'a>,
        body: &'a B,
    ) -> Result<Self> {
        Ok(Self::new(MessageType::Error, sender, body)?
            .set_reply_to(reply_to)?
            .set_field(MessageField::ErrorName(error_name)))
    }

    fn method(
        sender: Option<UniqueName<'a>>,
        path: ObjectPath<'a>,
        method_name: MemberName<'a>,
        body: &'a B,
    ) -> Result<Self> {
        Ok(Self::new(MessageType::MethodCall, sender, body)?
            .set_field(MessageField::Path(path))
            .set_field(MessageField::Member(method_name)))
    }

    fn signal(
        sender: Option<UniqueName<'a>>,
        path: ObjectPath<'a>,
        iface: InterfaceName<'a>,
        signal_name: MemberName<'a>,
        body: &'a B,
    ) -> Result<Self> {
        Ok(Self::new(MessageType::Signal, sender, body)?
            .set_field(MessageField::Path(path))
            .set_field(MessageField::Interface(iface))
            .set_field(MessageField::Member(signal_name)))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Fds {
    Owned(Vec<OwnedFd>),
    Raw(Vec<RawFd>),
}

impl Clone for Fds {
    fn clone(&self) -> Self {
        Fds::Raw(match self {
            Fds::Raw(v) => v.clone(),
            Fds::Owned(v) => v.iter().map(|fd| fd.as_raw_fd()).collect(),
        })
    }
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
/// [`disown_fds`] after deserializing to `RawFD` using [`body`] if you want to take the ownership.
/// Moreover, a clone of a message with owned FDs will only receive unowned copies of the FDs.
///
/// [`body`]: #method.body
/// [`disown_fds`]: #method.disown_fds
/// [`Connection`]: struct.Connection#method.call_method
#[derive(Clone)]
pub struct Message {
    primary_header: MessagePrimaryHeader,
    bytes: Vec<u8>,
    fds: Arc<RwLock<Fds>>,
}

assert_impl_all!(Message: Send, Sync, Unpin);

// TODO: Handle non-native byte order: https://gitlab.freedesktop.org/dbus/zbus/-/issues/19
impl Message {
    /// Create a message of type [`MessageType::MethodCall`].
    ///
    /// [`MessageType::MethodCall`]: enum.MessageType.html#variant.MethodCall
    pub fn method<'s, 'd, 'p, 'i, 'm, S, D, P, I, M, B>(
        sender: Option<S>,
        destination: Option<D>,
        path: P,
        iface: Option<I>,
        method_name: M,
        body: &B,
    ) -> Result<Self>
    where
        S: TryInto<UniqueName<'s>>,
        D: TryInto<BusName<'d>>,
        P: TryInto<ObjectPath<'p>>,
        I: TryInto<InterfaceName<'i>>,
        M: TryInto<MemberName<'m>>,
        S::Error: Into<Error>,
        D::Error: Into<Error>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + DynamicType,
    {
        let sender = match sender {
            Some(sender) => Some(sender.try_into().map_err(Into::into)?),
            None => None,
        };
        let mut b = MessageBuilder::method(
            sender,
            path.try_into().map_err(Into::into)?,
            method_name.try_into().map_err(Into::into)?,
            body,
        )?;
        if let Some(destination) = destination {
            b = b.set_field(MessageField::Destination(
                destination.try_into().map_err(Into::into)?,
            ));
        }
        if let Some(iface) = iface {
            b = b.set_field(MessageField::Interface(
                iface.try_into().map_err(Into::into)?,
            ));
        }
        b.build()
    }

    /// Create a message of type [`MessageType::Signal`].
    ///
    /// [`MessageType::Signal`]: enum.MessageType.html#variant.Signal
    pub fn signal<'s, 'd, 'p, 'i, 'm, S, D, P, I, M, B>(
        sender: Option<S>,
        destination: Option<D>,
        path: P,
        iface: I,
        signal_name: M,
        body: &B,
    ) -> Result<Self>
    where
        S: TryInto<UniqueName<'s>>,
        D: TryInto<BusName<'d>>,
        P: TryInto<ObjectPath<'p>>,
        I: TryInto<InterfaceName<'i>>,
        M: TryInto<MemberName<'m>>,
        S::Error: Into<Error>,
        D::Error: Into<Error>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + DynamicType,
    {
        let sender = match sender {
            Some(sender) => Some(sender.try_into().map_err(Into::into)?),
            None => None,
        };
        let mut b = MessageBuilder::signal(
            sender,
            path.try_into().map_err(Into::into)?,
            iface.try_into().map_err(Into::into)?,
            signal_name.try_into().map_err(Into::into)?,
            body,
        )?;
        if let Some(destination) = destination {
            b = b.set_field(MessageField::Destination(
                destination.try_into().map_err(Into::into)?,
            ));
        }
        b.build()
    }

    /// Create a message of type [`MessageType::MethodReturn`].
    ///
    /// [`MessageType::MethodReturn`]: enum.MessageType.html#variant.MethodReturn
    pub fn method_reply<'s, S, B>(sender: Option<S>, call: &Self, body: &B) -> Result<Self>
    where
        S: TryInto<UniqueName<'s>>,
        S::Error: Into<Error>,
        B: serde::ser::Serialize + DynamicType,
    {
        let sender = match sender {
            Some(sender) => Some(sender.try_into().map_err(Into::into)?),
            None => None,
        };
        MessageBuilder::reply(sender, call, body)?.build()
    }

    /// Create a message of type [`MessageType::MethodError`].
    ///
    /// [`MessageType::MethodError`]: enum.MessageType.html#variant.MethodError
    pub fn method_error<'s, 'e, S, E, B>(
        sender: Option<S>,
        call: &Self,
        name: E,
        body: &B,
    ) -> Result<Self>
    where
        S: TryInto<UniqueName<'s>>,
        S::Error: Into<Error>,
        E: TryInto<ErrorName<'e>>,
        E::Error: Into<Error>,
        B: serde::ser::Serialize + DynamicType,
    {
        let sender = match sender {
            Some(sender) => Some(sender.try_into().map_err(Into::into)?),
            None => None,
        };
        let name = name.try_into().map_err(Into::into)?;
        MessageBuilder::error(sender, call, name, body)?.build()
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < MIN_MESSAGE_SIZE {
            return Err(Error::InsufficientData);
        }

        if EndianSig::try_from(bytes[0])? != NATIVE_ENDIAN_SIG {
            return Err(Error::IncorrectEndian);
        }

        let primary_header = zvariant::from_slice(bytes, dbus_context!(0)).map_err(Error::from)?;
        let bytes = bytes.to_vec();
        let fds = Arc::new(RwLock::new(Fds::Raw(vec![])));
        Ok(Self {
            primary_header,
            bytes,
            fds,
        })
    }

    pub(crate) fn add_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() > self.bytes_to_completion()? {
            return Err(Error::ExcessData);
        }

        self.bytes.extend(bytes);

        Ok(())
    }

    pub(crate) fn set_owned_fds(&self, fds: Vec<OwnedFd>) {
        *self.fds.write().expect(LOCK_PANIC_MSG) = Fds::Owned(fds);
    }

    /// Disown the associated file descriptors from the message.
    ///
    /// When a message is received over a AF_UNIX socket, it may contain associated FDs. To prevent
    /// the message from closing those FDs on drop, call this method that returns all the received
    /// FDs with their ownership.
    ///
    /// Note: the message will continue to reference the files, so you must keep them open for as
    /// long as the message itself.
    pub fn disown_fds(&self) -> Vec<OwnedFd> {
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

    pub(crate) fn bytes_to_completion(&self) -> Result<usize> {
        let header_len = MIN_MESSAGE_SIZE + self.fields_len()?;
        let body_padding = padding_for_8_bytes(header_len);
        let body_len = self.primary_header().body_len();
        let required = header_len + body_padding + body_len as usize;

        Ok(required - self.bytes.len())
    }

    /// The signature of the body.
    ///
    /// **Note:** While zbus treats multiple arguments as a struct (to allow you to use the tuple
    /// syntax), D-Bus does not. Since this method gives you the signature expected on the wire by
    /// D-Bus, the trailing and leading STRUCT signature parenthesis will not be present in case of
    /// multiple arguments.
    pub fn body_signature(&self) -> Result<Signature<'_>> {
        match self
            .header()?
            .into_fields()
            .into_field(MessageFieldCode::Signature)
            .ok_or(Error::NoBodySignature)?
        {
            MessageField::Signature(signature) => Ok(signature),
            _ => Err(Error::InvalidField),
        }
    }

    pub fn primary_header(&self) -> &MessagePrimaryHeader {
        &self.primary_header
    }

    pub(crate) fn modify_primary_header<F>(&mut self, mut modifier: F) -> Result<()>
    where
        F: FnMut(&mut MessagePrimaryHeader) -> Result<()>,
    {
        modifier(&mut self.primary_header)?;

        let mut cursor = Cursor::new(&mut self.bytes);
        zvariant::to_writer(&mut cursor, dbus_context!(0), &self.primary_header)
            .map(|_| ())
            .map_err(Error::from)
    }

    /// Deserialize the header.
    pub fn header(&self) -> Result<MessageHeader<'_>> {
        zvariant::from_slice(&self.bytes, dbus_context!(0)).map_err(Error::from)
    }

    /// Deserialize the fields.
    pub fn fields(&self) -> Result<MessageFields<'_>> {
        let ctxt = dbus_context!(crate::PRIMARY_HEADER_SIZE);
        zvariant::from_slice(&self.bytes[crate::PRIMARY_HEADER_SIZE..], ctxt).map_err(Error::from)
    }

    /// Deserialize the body (without checking signature matching).
    pub fn body_unchecked<'d, 'm: 'd, B>(&'m self) -> Result<B>
    where
        B: serde::de::Deserialize<'d> + Type,
    {
        if self.bytes_to_completion()? != 0 {
            return Err(Error::InsufficientData);
        }

        let mut header_len = MIN_MESSAGE_SIZE + self.fields_len()?;
        header_len = header_len + padding_for_8_bytes(header_len);

        zvariant::from_slice_fds(
            &self.bytes[header_len..],
            Some(&self.fds()),
            dbus_context!(0),
        )
        .map_err(Error::from)
    }

    /// Deserialize the body using the contained signature.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::convert::TryInto;
    /// # use zbus::Message;
    /// # (|| -> zbus::Result<()> {
    /// let send_body = (7i32, (2i32, "foo"), vec!["bar"]);
    /// let message = Message::method(None::<&str>, Some("zbus.test"), "/", Some("zbus.test"), "ping", &send_body)?;
    /// let body : zvariant::Structure = message.body()?;
    /// let fields = body.fields();
    /// assert!(matches!(fields[0], zvariant::Value::I32(7)));
    /// assert!(matches!(fields[1], zvariant::Value::Structure(_)));
    /// assert!(matches!(fields[2], zvariant::Value::Array(_)));
    ///
    /// let reply_msg = Message::method_reply(None::<&str>, &message, &body)?;
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
        let body_sig = match self.body_signature() {
            Ok(sig) => sig,
            Err(Error::NoBodySignature) => Signature::from_static_str_unchecked(""),
            Err(e) => return Err(e),
        };

        if self.bytes_to_completion()? != 0 {
            return Err(Error::InsufficientData);
        }

        let mut header_len = MIN_MESSAGE_SIZE + self.fields_len()?;
        header_len = header_len + padding_for_8_bytes(header_len);

        zvariant::from_slice_fds_for_dynamic_signature(
            &self.bytes[header_len..],
            Some(&self.fds()),
            dbus_context!(0),
            &body_sig,
        )
        .map_err(Error::from)
    }

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
        if self.bytes_to_completion()? != 0 {
            return Err(Error::InsufficientData);
        }

        let mut header_len = MIN_MESSAGE_SIZE + self.fields_len()?;
        header_len = header_len + padding_for_8_bytes(header_len);

        Ok(&self.bytes[header_len..])
    }

    fn fields_len(&self) -> Result<usize> {
        zvariant::from_slice(&self.bytes[FIELDS_LEN_START_OFFSET..], dbus_context!(0))
            .map(|v: u32| v as usize)
            .map_err(Error::from)
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = f.debug_struct("Msg");
        let _ = self.header().map(|h| {
            if let Ok(t) = h.message_type() {
                msg.field("type", &t);
            }
            if let Ok(Some(sender)) = h.sender() {
                msg.field("sender", &sender);
            }
            if let Ok(Some(serial)) = h.reply_serial() {
                msg.field("reply-serial", &serial);
            }
            if let Ok(Some(path)) = h.path() {
                msg.field("path", &path);
            }
            if let Ok(Some(iface)) = h.interface() {
                msg.field("iface", &iface);
            }
            if let Ok(Some(member)) = h.member() {
                msg.field("member", &member);
            }
        });
        if let Ok(s) = self.body_signature() {
            msg.field("body", &s);
        }
        let fds = self.fds();
        if !fds.is_empty() {
            msg.field("fds", &fds);
        }
        msg.finish()
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = self.header();
        let (ty, error_name, sender, member) = if let Ok(h) = header.as_ref() {
            (
                h.message_type().ok(),
                h.error_name().ok().flatten(),
                h.sender().ok().flatten(),
                h.member().ok().flatten(),
            )
        } else {
            (None, None, None, None)
        };

        match ty {
            Some(MessageType::MethodCall) => {
                write!(f, "Method call")?;
                if let Some(m) = member {
                    write!(f, " {}", m)?;
                }
            }
            Some(MessageType::MethodReturn) => {
                write!(f, "Method return")?;
            }
            Some(MessageType::Error) => {
                write!(f, "Error")?;
                if let Some(e) = error_name {
                    write!(f, " {}", e)?;
                }

                let msg = self.body_unchecked::<&str>();
                if let Ok(msg) = msg {
                    write!(f, ": {}", msg)?;
                }
            }
            Some(MessageType::Signal) => {
                write!(f, "Signal")?;
                if let Some(m) = member {
                    write!(f, " {}", m)?;
                }
            }
            _ => {
                write!(f, "Unknown message")?;
            }
        }

        if let Some(s) = sender {
            write!(f, " from {}", s)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;

    use super::{Fds, Message};
    use std::os::unix::io::AsRawFd;
    use test_env_log::test;
    use zvariant::Fd;

    #[test]
    fn test() {
        let stdout = std::io::stdout();
        let m = Message::method(
            Some(":1.72"),
            None::<()>,
            "/",
            None::<()>,
            "do",
            &(Fd::from(&stdout), "foo"),
        )
        .unwrap();
        assert_eq!(m.body_signature().unwrap().to_string(), "hs");
        assert_eq!(*m.fds.read().unwrap(), Fds::Raw(vec![stdout.as_raw_fd()]));

        let body: Result<u32, Error> = m.body();
        assert!(matches!(
            body.unwrap_err(),
            Error::Variant(zvariant::Error::SignatureMismatch { .. })
        ));

        assert_eq!(m.to_string(), "Method call do from :1.72");
        let r = Message::method_reply(None::<()>, &m, &("all fine!")).unwrap();
        assert_eq!(r.to_string(), "Method return");
        let e = Message::method_error(
            None::<()>,
            &m,
            "org.freedesktop.zbus.Error",
            &("kaboom!", 32),
        )
        .unwrap();
        assert_eq!(e.to_string(), "Error org.freedesktop.zbus.Error: kaboom!");
    }
}
