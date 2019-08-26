use byteorder::ByteOrder;
use std::error;
use std::{borrow::Cow, fmt};

use crate::utils::padding_for_8_bytes;
use crate::Signature;
use crate::Structure;
use crate::{MessageField, MessageFieldCode, MessageFieldError};
use crate::{Variant, VariantError, VariantType};

/// Size of primary message header
pub const PRIMARY_HEADER_SIZE: usize = 16;

const MESSAGE_TYPE_OFFSET: usize = 1;

const BODY_LEN_START_OFFSET: usize = 4;
const BODY_LEN_END_OFFSET: usize = 8;

const SERIAL_START_OFFSET: usize = 8;
const SERIAL_END_OFFSET: usize = 12;

const FIELDS_LEN_START_OFFSET: usize = 12;
const FIELDS_LEN_END_OFFSET: usize = 16;

const FIELDS_START_OFFSET: usize = 16;

#[cfg(target_endian = "big")]
const ENDIAN_SIG: u8 = b'B';
#[cfg(target_endian = "little")]
const ENDIAN_SIG: u8 = b'l';

// It's actually 10 (and even not that) but let's round it to next 8-byte alignment
const MAX_FIELDS_IN_MESSAGE: usize = 16;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
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

#[derive(Debug)]
pub struct Message(Vec<u8>);

impl Message {
    pub fn method(
        destination: Option<&str>,
        path: &str,
        iface: Option<&str>,
        method_name: &str,
        body: Option<Vec<Variant>>,
    ) -> Result<Self, MessageError> {
        let mut m = Message::new(MessageType::MethodCall);

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

        // Message body structure and its encoding
        let (body_encoding, body_structure) = body
            .map(|variants| {
                let structure = Structure::new(variants);

                (structure.encode(0), Some(structure))
            })
            .unwrap_or((vec![], None));
        m.0.extend(&(body_encoding.len() as u32).to_ne_bytes());

        // Serial number. FIXME: managed by connection
        m.0.extend(&1u32.to_ne_bytes());

        // Now array of fields, which starts with length of the array (but lets start with 0)
        let mut array_len = 0u32;
        m.0.extend(&array_len.to_ne_bytes());

        if let Some(destination) = destination {
            array_len += m.push_field(&MessageField::destination(destination), 0)?;
        }
        if let Some(iface) = iface {
            let padding = padding_for_8_bytes(array_len);
            array_len += m.push_field(&MessageField::interface(iface), padding)?;
        }
        if let Some(structure) = body_structure {
            let padding = padding_for_8_bytes(array_len);
            let signature = structure.signature();
            // Remove the leading and trailing STRUCT delimiter
            let signature = &signature[1..signature.len() - 1];
            array_len += m.push_field(&MessageField::signature(signature), padding)?;
        }
        let padding = padding_for_8_bytes(array_len);
        array_len += m.push_field(&MessageField::path(path), padding)?;
        let padding = padding_for_8_bytes(array_len);
        array_len += m.push_field(&MessageField::member(method_name), padding)?;
        byteorder::NativeEndian::write_u32(
            &mut m.0[FIELDS_LEN_START_OFFSET..FIELDS_LEN_END_OFFSET],
            array_len,
        );
        let padding = padding_for_8_bytes(array_len);
        if padding > 0 {
            m.push_padding(padding);
        }

        if body_encoding.len() > 0 {
            m.0.extend(body_encoding);
        }

        Ok(m)
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

    pub fn bytes_to_completion(&self) -> u32 {
        let header_len = PRIMARY_HEADER_SIZE as u32 + self.fields_len();

        (header_len + padding_for_8_bytes(header_len) + self.body_len()) - (self.0.len() as u32)
    }

    pub fn fields_len(&self) -> u32 {
        byteorder::NativeEndian::read_u32(&self.0[FIELDS_LEN_START_OFFSET..FIELDS_LEN_END_OFFSET])
    }

    pub fn body_len(&self) -> u32 {
        byteorder::NativeEndian::read_u32(&self.0[BODY_LEN_START_OFFSET..BODY_LEN_END_OFFSET])
    }

    pub fn body_signature(&self) -> Result<String, MessageError> {
        for field in self.fields()? {
            if field.code() == MessageFieldCode::Signature {
                let value = field.value()?;
                let sig = value.get::<Signature>()?;

                return Ok(String::from(sig.as_str()));
            }
        }

        Err(MessageError::NoBodySignature)
    }

    pub fn message_type(&self) -> MessageType {
        MessageType::from(self.0[MESSAGE_TYPE_OFFSET])
    }

    pub fn fields(&self) -> Result<Vec<MessageField>, MessageError> {
        if self.bytes_to_completion() != 0 {
            return Err(MessageError::InsufficientData);
        }

        let fields_len = self.fields_len() as usize;
        if fields_len == 0 {
            // FIXME: Should this be an error? Can there be a message w/o any fields?
            return Ok(vec![]);
        }

        let mut v = Vec::with_capacity(MAX_FIELDS_IN_MESSAGE);

        let mut i = FIELDS_START_OFFSET;
        while i < FIELDS_START_OFFSET + fields_len {
            let (field, len) = MessageField::from_data(&self.0[i..])?;

            // According to the spec, we should ignore unkown fields.
            if field.code() != MessageFieldCode::Invalid {
                v.push(field);
            }

            i += len;
            i += padding_for_8_bytes(i as u32) as usize;
        }

        Ok(v)
    }

    pub fn body(&self, body_signature: Option<&str>) -> Result<Vec<Variant>, MessageError> {
        if self.bytes_to_completion() != 0 {
            return Err(MessageError::InsufficientData);
        }

        let mut header_len = PRIMARY_HEADER_SIZE as u32 + self.fields_len();
        header_len = header_len + padding_for_8_bytes(header_len);
        if self.body_len() == 0 {
            return Ok(vec![]);
        }

        let signature = body_signature
            .map(|s| Cow::from(s))
            .unwrap_or(Cow::from(self.body_signature()?));
        // Add () for Structure
        let signature = format!("({})", signature);
        let structure = Structure::decode(&self.0[(header_len as usize)..], &signature)?;

        Ok(structure.take_fields())
    }

    pub fn no_reply_expected(mut self) -> Self {
        self.0[3] = self.0[3] | 0x01;

        self
    }

    pub fn no_auto_start(mut self) -> Self {
        self.0[3] = self.0[3] | 0x02;

        self
    }

    pub fn allow_interactive_auth(mut self) -> Self {
        self.0[3] = self.0[3] | 0x03;

        self
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn new(mtype: MessageType) -> Self {
        Message(vec![
            ENDIAN_SIG,  // Endianness
            mtype as u8, // Message type
            0,           // Flags
            1,           // Major version of D-Bus protocol
        ])
    }

    fn push_padding(&mut self, len: u32) {
        self.0.extend(std::iter::repeat(0).take(len as usize));
    }

    fn push_field(&mut self, field: &MessageField, padding: u32) -> Result<u32, MessageError> {
        self.push_padding(padding);

        let encoded = field.encode();
        self.0.extend(&encoded);

        Ok(encoded.len() as u32 + padding)
    }
}
