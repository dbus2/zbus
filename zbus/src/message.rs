use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::io::{Cursor, Error as IOError};
use std::os::unix::io::{IntoRawFd, RawFd};

use zvariant::{EncodingContext, Error as VariantError};
use zvariant::{Signature, Type};

use crate::owned_fd::OwnedFd;
use crate::utils::padding_for_8_bytes;
use crate::{EndianSig, MessageHeader, MessagePrimaryHeader, MessageType};
use crate::{MessageField, MessageFieldCode, MessageFieldError, MessageFields};
use crate::{MIN_MESSAGE_SIZE, NATIVE_ENDIAN_SIG};

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
    MessageField(MessageFieldError),
    Variant(VariantError),
}

impl error::Error for MessageError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            MessageError::Io(e) => Some(e),
            MessageError::MessageField(e) => Some(e),
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
            MessageError::MessageField(e) => write!(f, "{}", e),
            MessageError::Variant(e) => write!(f, "{}", e),
        }
    }
}

impl From<MessageFieldError> for MessageError {
    fn from(val: MessageFieldError) -> MessageError {
        MessageError::MessageField(val)
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
    fn new(ty: MessageType, body: &'a B) -> Result<Self, MessageError> {
        let (body_len, fds_len) = zvariant::serialized_size(body)?;
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
        let mut bytes: Vec<u8> = Vec::with_capacity(MIN_MESSAGE_SIZE);
        let mut fds = vec![];

        let MessageBuilder {
            ty,
            body,
            body_len,
            mut fields,
            reply_to,
        } = self;
        let mut cursor = Cursor::new(&mut bytes);
        let ctxt = dbus_context!(0);

        if let Some(reply_to) = reply_to.as_ref() {
            let destination = reply_to.sender()?.ok_or(MessageError::MissingSender)?;
            let serial = reply_to.primary().serial_num();

            fields.add(MessageField::destination(destination));
            fields.add(MessageField::reply_serial(serial));
        }

        let primary = MessagePrimaryHeader::new(ty, body_len);
        let header = MessageHeader::new(primary, fields);

        zvariant::to_write(&mut cursor, ctxt, &header)?;
        zvariant::to_write_fds(&mut cursor, &mut fds, ctxt, body)?;

        Ok(Message { bytes, fds })
    }

    fn set_reply_to(mut self, reply_to: &'a Message) -> Result<Self, MessageError> {
        self.reply_to = Some(reply_to.header()?);
        Ok(self)
    }

    fn set_field(mut self, field: MessageField<'a>) -> Result<Self, MessageError> {
        self.fields.add(field);
        Ok(self)
    }

    fn reply(reply_to: &'a Message, body: &'a B) -> Result<Self, MessageError> {
        Self::new(MessageType::MethodReturn, body)?.set_reply_to(reply_to)
    }

    fn error(
        reply_to: &'a Message,
        error_name: &'a str,
        body: &'a B,
    ) -> Result<Self, MessageError> {
        Self::new(MessageType::Error, body)?
            .set_reply_to(reply_to)?
            .set_field(MessageField::error_name(error_name))
    }

    fn method(path: &'a str, method_name: &'a str, body: &'a B) -> Result<Self, MessageError> {
        let path = path.try_into()?;

        Self::new(MessageType::MethodCall, body)?
            .set_field(MessageField::path(path))?
            .set_field(MessageField::member(method_name))
    }
}

/// A DBus Message
///
/// The content of the serialized Message is in `bytes`.
///
/// *Note*: The owner of the message is responsible for closing the
/// `fds`.
pub struct Message {
    bytes: Vec<u8>,
    fds: Vec<RawFd>,
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
        let mut b = MessageBuilder::method(path, method_name, body)?;
        if let Some(sender) = sender {
            b = b.set_field(MessageField::sender(sender))?;
        }
        if let Some(destination) = destination {
            b = b.set_field(MessageField::destination(destination))?;
        }
        if let Some(iface) = iface {
            b = b.set_field(MessageField::interface(iface))?;
        }
        b.build()
    }

    pub fn method_reply<B>(call: &Self, body: &B) -> Result<Self, MessageError>
    where
        B: serde::ser::Serialize + Type,
    {
        MessageBuilder::reply(call, body)?.build()
    }

    pub fn method_error<B>(call: &Self, name: &str, body: &B) -> Result<Self, MessageError>
    where
        B: serde::ser::Serialize + Type,
    {
        MessageBuilder::error(call, name, body)?.build()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError> {
        if bytes.len() < MIN_MESSAGE_SIZE {
            return Err(MessageError::InsufficientData);
        }

        if EndianSig::try_from(bytes[0])? != NATIVE_ENDIAN_SIG {
            return Err(MessageError::IncorrectEndian);
        }

        let bytes = bytes.to_vec();
        let fds = vec![];
        Ok(Self { bytes, fds })
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) -> Result<(), MessageError> {
        if bytes.len() > self.bytes_to_completion()? {
            return Err(MessageError::ExcessData);
        }

        self.bytes.extend(bytes);

        Ok(())
    }

    pub(crate) fn set_fds(&mut self, fds: Vec<OwnedFd>) {
        assert_eq!(self.fds.len(), 0);
        // From now on, it's the caller responsability to close the fds
        self.fds = fds.into_iter().map(|fd| fd.into_raw_fd()).collect();
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
        zvariant::to_write(&mut cursor, dbus_context!(0), &primary)
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

        zvariant::from_slice_fds(&self.bytes[header_len..], Some(&self.fds), dbus_context!(0))
            .map_err(MessageError::from)
    }

    pub fn fds(&self) -> &[RawFd] {
        &self.fds
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
        if !self.fds.is_empty() {
            msg.field("fds", &self.fds);
        }
        msg.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Message;
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
        assert_eq!(m.fds, vec![stdout.as_raw_fd()]);

        Message::method_reply(&m, &("all fine!")).unwrap();
        Message::method_error(&m, "org.freedesktop.zbus.Error", &("kaboom!", 32)).unwrap();
    }
}
