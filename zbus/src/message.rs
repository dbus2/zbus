use byteorder::ByteOrder;
use std::error;
use std::fmt;
use std::io::Error as IOError;

use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};

use zvariant::{EncodingFormat, Error as VariantError, FromVariant};
use zvariant::{Signature, VariantValue};

use crate::utils::padding_for_8_bytes;
use crate::{MessageField, MessageFieldCode, MessageFieldError, MessageFields};

const PRIMARY_HEADER_SIZE: usize = 12;
pub const MIN_MESSAGE_SIZE: usize = PRIMARY_HEADER_SIZE + 4;

const BODY_LEN_START_OFFSET: usize = 4;

const SERIAL_START_OFFSET: usize = 8;
const SERIAL_END_OFFSET: usize = 12;

const FIELDS_LEN_START_OFFSET: usize = 12;

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

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        u8::deserialize(deserializer).map(MessageType::from)
    }
}

// FIXME: Use derive macro when it's available
impl VariantValue for MessageType {
    fn signature() -> Signature<'static> {
        u8::signature()
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePrimaryHeader {
    endian_sig: u8,
    msg_type: MessageType,
    flags: u8,
    protocol_version: u8,
    body_len: u32,
    serial_num: u32,
}

// FIXME: Use derive macro when it's available
impl<'m> VariantValue for MessagePrimaryHeader {
    fn signature() -> Signature<'static> {
        Signature::from(format!(
            "({}{}{}{}{}{})",
            u8::signature(),
            MessageType::signature(),
            u8::signature(),
            u8::signature(),
            u32::signature(),
            u32::signature(),
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageHeader<'m> {
    primary: MessagePrimaryHeader,
    #[serde(borrow)]
    fields: MessageFields<'m>,
    end: ((),), // To ensure header end on 8-byte boundry
}

impl MessagePrimaryHeader {
    pub fn endian_sig(&self) -> u8 {
        self.endian_sig
    }

    pub fn msg_type(&self) -> MessageType {
        self.msg_type
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    pub fn body_len(&self) -> u32 {
        self.body_len
    }

    pub fn serial_num(&self) -> u32 {
        self.serial_num
    }
}

impl<'m> MessageHeader<'m> {
    pub fn primary(&self) -> &MessagePrimaryHeader {
        &self.primary
    }

    pub fn fields(&self) -> &MessageFields {
        &self.fields
    }
}

// FIXME: Use derive macro when it's available
impl<'m> VariantValue for MessageHeader<'m> {
    fn signature() -> Signature<'static> {
        Signature::from(format!(
            "({}{}())",
            MessagePrimaryHeader::signature(),
            MessageFields::signature(),
        ))
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
        let mut bytes: Vec<u8> = Vec::with_capacity(MIN_MESSAGE_SIZE);
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
        let header = MessageHeader {
            primary: MessagePrimaryHeader {
                endian_sig: ENDIAN_SIG,
                msg_type: MessageType::MethodCall,
                flags: 0,
                protocol_version: 1,
                body_len: 0,   // set to 0 at first
                serial_num: 1, // FIXME: managed by connection
            },
            fields,
            end: ((),),
        };
        zvariant::to_write_ne(&mut cursor, format, &header)?;

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
        if bytes.len() < MIN_MESSAGE_SIZE {
            return Err(MessageError::InsufficientData);
        }

        if bytes[0] != ENDIAN_SIG {
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
        let fields = self.header().map(|header| header.fields)?;
        for field in fields.get() {
            if field.code() == MessageFieldCode::Signature {
                let sig = Signature::from_variant_ref(field.value())?;

                // FIXME: Can we avoid the copy?
                return Ok(Signature::from(String::from(sig)));
            }
        }

        Err(MessageError::NoBodySignature)
    }

    pub fn primary_header(&self) -> Result<MessagePrimaryHeader, MessageError> {
        zvariant::from_slice_ne(&self.0, EncodingFormat::DBus).map_err(MessageError::from)
    }

    pub fn header(&self) -> Result<MessageHeader, MessageError> {
        zvariant::from_slice_ne(&self.0, EncodingFormat::DBus).map_err(MessageError::from)
    }

    pub fn fields(&self) -> Result<MessageFields, MessageError> {
        self.header().map(|header| header.fields)
    }

    pub fn body<'d, 'm: 'd, B>(&'m self) -> Result<B, MessageError>
    where
        B: serde::de::Deserialize<'d> + VariantValue,
    {
        if self.bytes_to_completion()? != 0 {
            return Err(MessageError::InsufficientData);
        }

        let mut header_len = MIN_MESSAGE_SIZE + self.fields_len()?;
        header_len = header_len + padding_for_8_bytes(header_len);

        zvariant::from_slice_ne(&self.0[header_len..], EncodingFormat::DBus)
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

    fn fields_len(&self) -> Result<usize, MessageError> {
        zvariant::from_slice_ne::<u32>(&self.0[FIELDS_LEN_START_OFFSET..], EncodingFormat::DBus)
            .map(|v| v as usize)
            .map_err(MessageError::from)
    }
}
