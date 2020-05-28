use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::io::{Cursor, Error as IOError};
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};

use zvariant::{EncodingContext, Error as VariantError};
use zvariant::{Signature, Type};

use crate::owned_fd::OwnedFd;
use crate::utils::padding_for_8_bytes;
use crate::{EndianSig, MessageHeader, MessagePrimaryHeader, MessageType};
use crate::{MessageField, MessageFieldCode, MessageFields};
use crate::{MIN_MESSAGE_SIZE, NATIVE_ENDIAN_SIG, PRIMARY_HEADER_SIZE};

const FIELDS_LEN_START_OFFSET: usize = 12;
macro_rules! dbus_context {
    ($n_bytes_before: expr) => {
        EncodingContext::<byteorder::NativeEndian>::new_dbus($n_bytes_before)
    };
}

#[derive(Debug)]
pub enum MessageError {
    StrTooLarge,
    InsufficientData,
    ExcessData,
    IncorrectEndian,
    Io(IOError),
    NoBodySignature,
    MissingSender,
    Variant(VariantError),
}

impl error::Error for MessageError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            MessageError::Io(e) => Some(e),
            MessageError::Variant(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageError::StrTooLarge => write!(f, "string too large"),
            MessageError::InsufficientData => write!(f, "insufficient data"),
            MessageError::Io(e) => e.fmt(f),
            MessageError::ExcessData => write!(f, "excess data"),
            MessageError::IncorrectEndian => write!(f, "incorrect endian"),
            MessageError::NoBodySignature => write!(f, "missing body signature"),
            MessageError::MissingSender => write!(f, "missing sender"),
            MessageError::Variant(e) => write!(f, "{}", e),
        }
    }
}

impl From<VariantError> for MessageError {
    fn from(val: VariantError) -> MessageError {
        MessageError::Variant(val)
    }
}

impl From<IOError> for MessageError {
    fn from(val: IOError) -> MessageError {
        MessageError::Io(val)
    }
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
    B: serde::ser::Serialize + Type,
{
    fn new(ty: MessageType, sender: Option<&'a str>, body: &'a B) -> Result<Self, MessageError> {
        let ctxt = dbus_context!(0);
        let (body_len, fds_len) = zvariant::serialized_size_fds(ctxt, body)?;
        let body_len = u32::try_from(body_len).map_err(|_| MessageError::ExcessData)?;

        let mut fields = MessageFields::new();

        let mut signature = B::signature();
        if signature != "" {
            if signature.starts_with(zvariant::STRUCT_SIG_START_STR) {
                // Remove leading and trailing STRUCT delimiters
                signature = Signature::from_string_unchecked(String::from(
                    &signature[1..signature.len() - 1],
                ));
            }
            fields.add(MessageField::signature(signature));
        }
        if let Some(sender) = sender {
            fields.add(MessageField::sender(sender));
        }

        if fds_len > 0 {
            fields.add(MessageField::unix_fds(fds_len as u32));
        }

        Ok(Self {
            ty,
            body,
            body_len,
            fields,
            reply_to: None,
        })
    }

    fn build(self) -> Result<Message, MessageError> {
        let MessageBuilder {
            ty,
            body,
            body_len,
            mut fields,
            reply_to,
        } = self;

        if let Some(reply_to) = reply_to.as_ref() {
            let destination = reply_to.sender()?.ok_or(MessageError::MissingSender)?;
            let serial = reply_to.primary().serial_num();

            fields.add(MessageField::destination(destination));
            fields.add(MessageField::reply_serial(serial));
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
            bytes,
            fds: Fds::Raw(fds),
        })
    }

    fn set_reply_to(mut self, reply_to: &'a Message) -> Result<Self, MessageError> {
        self.reply_to = Some(reply_to.header()?);
        Ok(self)
    }

    fn set_field(mut self, field: MessageField<'a>) -> Result<Self, MessageError> {
        self.fields.add(field);
        Ok(self)
    }

    fn reply(
        sender: Option<&'a str>,
        reply_to: &'a Message,
        body: &'a B,
    ) -> Result<Self, MessageError> {
        Self::new(MessageType::MethodReturn, sender, body)?.set_reply_to(reply_to)
    }

    fn error(
        sender: Option<&'a str>,
        reply_to: &'a Message,
        error_name: &'a str,
        body: &'a B,
    ) -> Result<Self, MessageError> {
        Self::new(MessageType::Error, sender, body)?
            .set_reply_to(reply_to)?
            .set_field(MessageField::error_name(error_name))
    }

    fn method(
        sender: Option<&'a str>,
        path: &'a str,
        method_name: &'a str,
        body: &'a B,
    ) -> Result<Self, MessageError> {
        let path = path.try_into()?;

        Self::new(MessageType::MethodCall, sender, body)?
            .set_field(MessageField::path(path))?
            .set_field(MessageField::member(method_name))
    }

    fn signal(
        sender: Option<&'a str>,
        path: &'a str,
        iface: &'a str,
        signal_name: &'a str,
        body: &'a B,
    ) -> Result<Self, MessageError> {
        let path = path.try_into()?;

        Self::new(MessageType::Signal, sender, body)?
            .set_field(MessageField::path(path))?
            .set_field(MessageField::interface(iface))?
            .set_field(MessageField::member(signal_name))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Fds {
    Owned(Vec<OwnedFd>),
    Raw(Vec<RawFd>),
}

/// A DBus Message
///
/// The content of the serialized Message is in `bytes`. To
/// deserialize the body of the message, use the [`body`] method. You
/// may also access the header and other details with the various
/// other getters.
///
/// *Note*: The message owns the received FDs and will close them when
/// dropped. You may call [`disown_fds`] after deserializing to
/// `RawFD` with [`body`] if you want to.
///
/// [`body`]: #method.body
/// [`disown_fds`]: #method.disown_fds
pub struct Message {
    bytes: Vec<u8>,
    fds: Fds,
}

// TODO: Make generic over byteorder
// TODO: Document
//
// * multiple args needing to be a tuple or struct
// * pass unit ref for empty body
// * Only primary header can be modified after creation.
impl Message {
    pub fn method<B>(
        sender: Option<&str>,
        destination: Option<&str>,
        path: &str,
        iface: Option<&str>,
        method_name: &str,
        body: &B,
    ) -> Result<Self, MessageError>
    where
        B: serde::ser::Serialize + Type,
    {
        let mut b = MessageBuilder::method(sender, path, method_name, body)?;
        if let Some(destination) = destination {
            b = b.set_field(MessageField::destination(destination))?;
        }
        if let Some(iface) = iface {
            b = b.set_field(MessageField::interface(iface))?;
        }
        b.build()
    }

    pub fn signal<B>(
        sender: Option<&str>,
        destination: Option<&str>,
        path: &str,
        iface: &str,
        signal_name: &str,
        body: &B,
    ) -> Result<Self, MessageError>
    where
        B: serde::ser::Serialize + Type,
    {
        let mut b = MessageBuilder::signal(sender, path, iface, signal_name, body)?;
        if let Some(destination) = destination {
            b = b.set_field(MessageField::destination(destination))?;
        }
        b.build()
    }

    pub fn method_reply<B>(
        sender: Option<&str>,
        call: &Self,
        body: &B,
    ) -> Result<Self, MessageError>
    where
        B: serde::ser::Serialize + Type,
    {
        MessageBuilder::reply(sender, call, body)?.build()
    }

    pub fn method_error<B>(
        sender: Option<&str>,
        call: &Self,
        name: &str,
        body: &B,
    ) -> Result<Self, MessageError>
    where
        B: serde::ser::Serialize + Type,
    {
        MessageBuilder::error(sender, call, name, body)?.build()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError> {
        if bytes.len() < MIN_MESSAGE_SIZE {
            return Err(MessageError::InsufficientData);
        }

        if EndianSig::try_from(bytes[0])? != NATIVE_ENDIAN_SIG {
            return Err(MessageError::IncorrectEndian);
        }

        let bytes = bytes.to_vec();
        let fds = Fds::Raw(vec![]);
        Ok(Self { bytes, fds })
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) -> Result<(), MessageError> {
        if bytes.len() > self.bytes_to_completion()? {
            return Err(MessageError::ExcessData);
        }

        self.bytes.extend(bytes);

        Ok(())
    }

    pub(crate) fn set_owned_fds(&mut self, fds: Vec<OwnedFd>) {
        self.fds = Fds::Owned(fds);
    }

    /// Disown the associated file descriptors.
    ///
    /// When a message is received over a AF_UNIX socket, it may
    /// contain associated FDs. To prevent the message from closing
    /// those FDs on drop, you may remove the ownership thanks to this
    /// method, after that you are responsible for closing them.
    pub fn disown_fds(&mut self) {
        if let Fds::Owned(ref mut fds) = &mut self.fds {
            // From now on, it's the caller responsability to close the fds
            self.fds = Fds::Raw(fds.drain(..).map(|fd| fd.into_raw_fd()).collect());
        }
    }

    pub fn bytes_to_completion(&self) -> Result<usize, MessageError> {
        let header_len = MIN_MESSAGE_SIZE + self.fields_len()?;
        let body_padding = padding_for_8_bytes(header_len);
        let body_len = self.primary_header()?.body_len();
        let required = header_len + body_padding + body_len as usize;

        Ok(required - self.bytes.len())
    }

    pub fn body_signature(&self) -> Result<Signature, MessageError> {
        let field = self
            .header()?
            .into_fields()
            .into_field(MessageFieldCode::Signature)
            .ok_or(MessageError::NoBodySignature)?;

        Ok(field.into_value().try_into()?)
    }

    pub fn primary_header(&self) -> Result<MessagePrimaryHeader, MessageError> {
        zvariant::from_slice(&self.bytes, dbus_context!(0)).map_err(MessageError::from)
    }

    pub fn modify_primary_header<F>(&mut self, mut modifier: F) -> Result<(), MessageError>
    where
        F: FnMut(&mut MessagePrimaryHeader) -> Result<(), MessageError>,
    {
        let mut primary = self.primary_header()?;
        modifier(&mut primary)?;

        let mut cursor = Cursor::new(&mut self.bytes);
        zvariant::to_writer(&mut cursor, dbus_context!(0), &primary)
            .map(|_| ())
            .map_err(MessageError::from)
    }

    pub fn header(&self) -> Result<MessageHeader, MessageError> {
        zvariant::from_slice(&self.bytes, dbus_context!(0)).map_err(MessageError::from)
    }

    pub fn fields(&self) -> Result<MessageFields, MessageError> {
        let ctxt = dbus_context!(crate::PRIMARY_HEADER_SIZE);
        zvariant::from_slice(&self.bytes[crate::PRIMARY_HEADER_SIZE..], ctxt)
            .map_err(MessageError::from)
    }

    pub fn body<'d, 'm: 'd, B>(&'m self) -> Result<B, MessageError>
    where
        B: serde::de::Deserialize<'d> + Type,
    {
        if self.bytes_to_completion()? != 0 {
            return Err(MessageError::InsufficientData);
        }

        let mut header_len = MIN_MESSAGE_SIZE + self.fields_len()?;
        header_len = header_len + padding_for_8_bytes(header_len);

        zvariant::from_slice_fds(
            &self.bytes[header_len..],
            Some(&self.fds()),
            dbus_context!(0),
        )
        .map_err(MessageError::from)
    }

    pub(crate) fn fds(&self) -> Vec<RawFd> {
        match &self.fds {
            Fds::Raw(fds) => fds.clone(),
            Fds::Owned(fds) => fds.iter().map(|f| f.as_raw_fd()).collect(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn fields_len(&self) -> Result<usize, MessageError> {
        zvariant::from_slice(&self.bytes[FIELDS_LEN_START_OFFSET..], dbus_context!(0))
            .map(|v: u32| v as usize)
            .map_err(MessageError::from)
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = f.debug_struct("Msg");
        let _ = self.header().and_then(|h| {
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
            Ok(())
        });
        if let Ok(s) = self.body_signature() {
            msg.field("body", &s);
        }
        if !self.fds().is_empty() {
            msg.field("fds", &self.fds);
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

                let msg = self.body::<&str>();
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
    use super::{Fds, Message};
    use std::os::unix::io::AsRawFd;
    use zvariant::Fd;

    #[test]
    fn test() {
        let stdout = std::io::stdout();
        let m = Message::method(
            Some(":1.72"),
            None,
            "/",
            None,
            "do",
            &(Fd::from(&stdout), "foo"),
        )
        .unwrap();
        assert_eq!(m.body_signature().unwrap().to_string(), "hs");
        assert_eq!(m.fds, Fds::Raw(vec![stdout.as_raw_fd()]));

        assert_eq!(m.to_string(), "Method call do from :1.72");
        let r = Message::method_reply(None, &m, &("all fine!")).unwrap();
        assert_eq!(r.to_string(), "Method return");
        let e = Message::method_error(None, &m, "org.freedesktop.zbus.Error", &("kaboom!", 32))
            .unwrap();
        assert_eq!(e.to_string(), "Error org.freedesktop.zbus.Error: kaboom!");
    }
}
