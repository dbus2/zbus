use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::io::{Cursor, Error as IOError};

use zvariant::{EncodingContext, Error as VariantError};
use zvariant::{Signature, Type};

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
pub struct Message(Vec<u8>);

// TODO: Make generic over byteorder
// TODO: Document
//
// * multiple args needing to be a tuple or struct
// * pass unit ref for empty body
// * Only primary header can be modified after creation.
impl Message {
    pub fn method<B>(
        destination: Option<&str>,
        path: &str,
        iface: Option<&str>,
        method_name: &str,
        body: &B,
    ) -> Result<Self, MessageError>
    where
        B: serde::ser::Serialize + Type,
    {
        let mut bytes: Vec<u8> = Vec::with_capacity(MIN_MESSAGE_SIZE);
        let mut cursor = Cursor::new(&mut bytes);

        let dest_length = destination.map_or(0, |s| s.len());
        let iface_length = iface.map_or(0, |s| s.len());

        // Checks args
        if dest_length > (u32::max_value() as usize)
            || path.len() > (u32::max_value() as usize)
            || iface_length > (u32::max_value() as usize)
            || method_name.len() > (u32::max_value() as usize)
        {
            return Err(MessageError::StrTooLarge);
        }

        // Construct the array of fields
        let mut fields = MessageFields::new();

        if let Some(destination) = destination {
            fields.add(MessageField::destination(destination));
        }
        if let Some(iface) = iface {
            fields.add(MessageField::interface(iface));
        }
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
        let path = zvariant::ObjectPath::try_from(path)?;
        fields.add(MessageField::path(path));
        fields.add(MessageField::member(method_name));

        let ctxt = dbus_context!(0);
        let mut header =
            MessageHeader::new(MessagePrimaryHeader::new(MessageType::MethodCall), fields);
        zvariant::to_write(&mut cursor, ctxt, &header)?;

        let body_len = zvariant::to_write(&mut cursor, ctxt, body)?;
        if body_len > u32::max_value() as usize {
            return Err(MessageError::ExcessData);
        }
        let primary = header.primary_mut();
        primary.set_body_len(body_len as u32);
        cursor.set_position(0);
        zvariant::to_write(&mut cursor, ctxt, primary)?;

        Ok(Message(bytes))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError> {
        if bytes.len() < MIN_MESSAGE_SIZE {
            return Err(MessageError::InsufficientData);
        }

        if EndianSig::try_from(bytes[0])? != NATIVE_ENDIAN_SIG {
            return Err(MessageError::IncorrectEndian);
        }

        Ok(Message(bytes.to_vec()))
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) -> Result<(), MessageError> {
        if bytes.len() > self.bytes_to_completion()? {
            return Err(MessageError::ExcessData);
        }

        self.0.extend(bytes);

        Ok(())
    }

    pub fn bytes_to_completion(&self) -> Result<usize, MessageError> {
        let header_len = MIN_MESSAGE_SIZE + self.fields_len()?;
        let body_padding = padding_for_8_bytes(header_len);
        let body_len = self.primary_header()?.body_len();
        let required = header_len + body_padding + body_len as usize;

        Ok(required - self.0.len())
    }

    pub fn body_signature(&self) -> Result<Signature, MessageError> {
        let field = self
            .header()?
            .take_fields()
            .take_field(MessageFieldCode::Signature)
            .ok_or(MessageError::NoBodySignature)?;

        Ok(field.take_value().try_into()?)
    }

    pub fn primary_header(&self) -> Result<MessagePrimaryHeader, MessageError> {
        zvariant::from_slice(&self.0, dbus_context!(0)).map_err(MessageError::from)
    }

    pub fn modify_primary_header<F>(&mut self, mut modifier: F) -> Result<(), MessageError>
    where
        F: FnMut(&mut MessagePrimaryHeader) -> Result<(), MessageError>,
    {
        let mut primary = self.primary_header()?;
        modifier(&mut primary)?;

        let mut cursor = Cursor::new(&mut self.0);
        zvariant::to_write(&mut cursor, dbus_context!(0), &primary)
            .map(|_| ())
            .map_err(MessageError::from)
    }

    pub fn header(&self) -> Result<MessageHeader, MessageError> {
        zvariant::from_slice(&self.0, dbus_context!(0)).map_err(MessageError::from)
    }

    pub fn fields(&self) -> Result<MessageFields, MessageError> {
        let ctxt = dbus_context!(crate::PRIMARY_HEADER_SIZE);
        zvariant::from_slice(&self.0[crate::PRIMARY_HEADER_SIZE..], ctxt)
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

        zvariant::from_slice(&self.0[header_len..], dbus_context!(0)).map_err(MessageError::from)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn fields_len(&self) -> Result<usize, MessageError> {
        zvariant::from_slice(&self.0[FIELDS_LEN_START_OFFSET..], dbus_context!(0))
            .map(|v: u32| v as usize)
            .map_err(MessageError::from)
    }
}
