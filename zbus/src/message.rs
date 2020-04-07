use byteorder::ByteOrder;
use std::error;
use std::fmt;
use std::io::{Error as IOError, Write};

use zvariant::{EncodingFormat, Error as VariantError, FromVariant};
use zvariant::{Signature, VariantValue};

use crate::utils::padding_for_8_bytes;
use crate::{MessageField, MessageFieldCode, MessageFieldError, MessageFields};

/// Size of primary message header
pub const PRIMARY_HEADER_SIZE: usize = 16;

const MESSAGE_TYPE_OFFSET: usize = 1;

const BODY_LEN_START_OFFSET: usize = 4;
const BODY_LEN_END_OFFSET: usize = 8;

const SERIAL_START_OFFSET: usize = 8;
const SERIAL_END_OFFSET: usize = 12;

const FIELDS_LEN_START_OFFSET: usize = 12;
const FIELDS_LEN_END_OFFSET: usize = 16;

#[cfg(target_endian = "big")]
const ENDIAN_SIG: u8 = b'B';
#[cfg(target_endian = "little")]
const ENDIAN_SIG: u8 = b'l';

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MessageType {
    Invalid = 0,
    MethodCall = 1,
    MethodReturn = 2,
    Error = 3,
    Signal = 4,
}

// Such a shame I've to do this manually
impl From<u8> for MessageType {
    fn from(val: u8) -> MessageType {
        match val {
            1 => MessageType::MethodCall,
            2 => MessageType::MethodReturn,
            3 => MessageType::Error,
            4 => MessageType::Signal,
            _ => MessageType::Invalid,
        }
    }
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
impl Message {
    pub fn method<B>(
        destination: Option<&str>,
        path: &str,
        iface: Option<&str>,
        method_name: &str,
        body: &B,
    ) -> Result<Self, MessageError>
    where
        B: serde::ser::Serialize + VariantValue,
    {
        let mut bytes: Vec<u8> = Vec::with_capacity(PRIMARY_HEADER_SIZE);
        let mut cursor = std::io::Cursor::new(&mut bytes);

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
                signature = Signature::from(String::from(&signature[1..signature.len() - 1]));
            }
            fields.add(MessageField::signature(signature));
        }

        fields.add(MessageField::path(path));
        fields.add(MessageField::member(method_name));

        let format = EncodingFormat::DBus;
        zvariant::to_write_ne(
            &mut cursor,
            format,
            &(
                ENDIAN_SIG,                    // Endianness
                MessageType::MethodCall as u8, // Message type
                0u8,                           // Flags
                1u8,                           // Major version of D-Bus protocol
                0u32,                          // Message body encoding set to 0 at first
                1u32,                          // Serial number. FIXME: managed by connection
                fields,
            ),
        )?; // Array of fields

        // Do we need to do this if body is None?
        let padding = padding_for_8_bytes(cursor.position() as usize);
        if padding > 0 {
            for _ in 0..padding {
                cursor.write_all(&[0u8; 1])?;
            }
        }

        let pos = cursor.position();
        zvariant::to_write_ne(&mut cursor, format, body)?;

        let body_len = cursor.position() - pos;
        if body_len > (u32::max_value() as u64) {
            return Err(MessageError::ExcessData);
        }
        cursor.set_position(BODY_LEN_START_OFFSET as u64);
        zvariant::to_write_ne(&mut cursor, format, &(body_len as u32))?;

        Ok(Message(bytes))
    }

    pub fn set_serial(mut self, serial: u32) -> Self {
        byteorder::NativeEndian::write_u32(
            &mut self.0[SERIAL_START_OFFSET..SERIAL_END_OFFSET],
            serial,
        );

        self
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError> {
        if bytes.len() < PRIMARY_HEADER_SIZE {
            return Err(MessageError::InsufficientData);
        }

        if bytes[0] != ENDIAN_SIG {
            return Err(MessageError::IncorrectEndian);
        }

        Ok(Message(bytes.to_vec()))
    }

    pub fn add_bytes(&mut self, bytes: &[u8]) -> Result<(), MessageError> {
        if bytes.len() > self.bytes_to_completion() as usize {
            return Err(MessageError::ExcessData);
        }

        self.0.extend(bytes);

        Ok(())
    }

    pub fn bytes_to_completion(&self) -> usize {
        let header_len = PRIMARY_HEADER_SIZE + self.fields_len();

        (header_len + padding_for_8_bytes(header_len) + self.body_len()) - self.0.len()
    }

    pub fn fields_len(&self) -> usize {
        byteorder::NativeEndian::read_u32(&self.0[FIELDS_LEN_START_OFFSET..FIELDS_LEN_END_OFFSET])
            as usize
    }

    pub fn body_len(&self) -> usize {
        byteorder::NativeEndian::read_u32(&self.0[BODY_LEN_START_OFFSET..BODY_LEN_END_OFFSET])
            as usize
    }

    pub fn body_signature(&self) -> Result<Signature, MessageError> {
        for field in self.fields()?.get() {
            if field.code() == MessageFieldCode::Signature {
                let sig = Signature::from_variant_ref(field.value())?;

                // FIXME: Can we avoid the copy?
                return Ok(Signature::from(String::from(sig)));
            }
        }

        Err(MessageError::NoBodySignature)
    }

    pub fn message_type(&self) -> MessageType {
        MessageType::from(self.0[MESSAGE_TYPE_OFFSET])
    }

    pub fn fields(&self) -> Result<MessageFields, MessageError> {
        if self.bytes_to_completion() != 0 {
            return Err(MessageError::InsufficientData);
        }

        zvariant::from_slice_ne::<(u8, u8, u8, u8, u32, u32, MessageFields)>(
            &self.0,
            EncodingFormat::DBus,
        )
        .map(|(_, _, _, _, _, _, fields)| fields)
        .map_err(MessageError::from)
    }

    pub fn body<'d, 'm: 'd, B>(&'m self) -> Result<Option<B>, MessageError>
    where
        B: serde::de::Deserialize<'d> + VariantValue,
    {
        if self.bytes_to_completion() != 0 {
            return Err(MessageError::InsufficientData);
        }

        let mut header_len = PRIMARY_HEADER_SIZE + self.fields_len();
        header_len = header_len + padding_for_8_bytes(header_len);
        if self.body_len() == 0 {
            return Ok(None);
        }

        zvariant::from_slice_ne(&self.0[header_len..], EncodingFormat::DBus)
            .map(Some)
            .map_err(MessageError::from)
    }

    pub fn no_reply_expected(mut self) -> Self {
        self.0[3] |= 0x01;

        self
    }

    pub fn no_auto_start(mut self) -> Self {
        self.0[3] |= 0x02;

        self
    }

    pub fn allow_interactive_auth(mut self) -> Self {
        self.0[3] |= 0x03;

        self
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}
