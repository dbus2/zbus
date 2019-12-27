use byteorder::ByteOrder;
use core::convert::TryInto;
use std::error;
use std::fmt;

use crate::utils::padding_for_8_bytes;
use crate::Signature;
use crate::VariantError;
use crate::{Array, Decode, Encode, EncodingFormat};
use crate::{MessageField, MessageFieldCode, MessageFieldError, MessageFields};
use crate::{SharedData, Structure};

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
        body: Option<Structure>,
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

        // Message body encoding set to 0 at first
        let body_len_position = m.0.len();
        m.0.extend(&0u32.to_ne_bytes());

        // Serial number. FIXME: managed by connection
        m.0.extend(&1u32.to_ne_bytes());

        // Now array of fields
        let mut fields = MessageFields::new();

        if let Some(destination) = destination {
            fields.add(MessageField::destination(destination));
        }
        if let Some(iface) = iface {
            fields.add(MessageField::interface(iface));
        }
        let body_signature = body.as_ref().map(|structure| {
            let signature = structure.signature();
            // Remove the leading and trailing STRUCT delimiter
            let signature = &signature[1..signature.len() - 1];

            String::from(signature)
        });
        if let Some(body_signature) = body_signature {
            fields.add(MessageField::signature(body_signature));
        }

        fields.add(MessageField::path(path));
        fields.add(MessageField::member(method_name));

        let format = EncodingFormat::DBus;
        let array = Array::from(fields);
        array.encode_into(&mut m.0, format);

        // Do we need to do this if body is None?
        let padding = padding_for_8_bytes(m.0.len());
        if padding > 0 {
            m.push_padding(padding);
        }

        if let Some(body) = body {
            let n_bytes_before = m.0.len();
            body.encode_into(&mut m.0, format);

            let len = crate::utils::usize_to_u32(m.0.len() - n_bytes_before);
            byteorder::NativeEndian::write_u32(
                &mut m.0[body_len_position..body_len_position + 4],
                len,
            );
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
        for field in self.fields()?.inner() {
            if field.code()? == MessageFieldCode::Signature {
                let value = field.value()?;
                let sig = Signature::from_variant(value)?;

                return Ok(Signature::from(sig.as_str()));
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

        // FIXME: We can avoid this deep copy (perhaps if we have builder pattern for Message?)
        let encoding = SharedData::new(self.0.clone());
        let format = EncodingFormat::DBus;
        let slice = Array::slice_data(&encoding.tail(FIELDS_LEN_START_OFFSET), "a(yv)", format)?;

        let array = Array::decode(&slice, "a(yv)", format).map_err(MessageError::from)?;

        array.try_into().map_err(MessageError::from)
    }

    pub fn body(&self, body_signature: Option<Signature>) -> Result<Structure, MessageError> {
        if self.bytes_to_completion() != 0 {
            return Err(MessageError::InsufficientData);
        }

        let mut header_len = PRIMARY_HEADER_SIZE + self.fields_len();
        header_len = header_len + padding_for_8_bytes(header_len);
        if self.body_len() == 0 {
            return Ok(Structure::new());
        }

        let signature = body_signature.unwrap_or(self.body_signature()?);
        // Add () for Structure
        let signature = format!("({})", signature.as_str());

        // FIXME: We can avoid this deep copy (perhaps if we have builder pattern for Message?)
        let encoding = SharedData::new(self.0.clone());
        let structure =
            Structure::decode(&encoding.tail(header_len), signature, EncodingFormat::DBus)?;

        Ok(structure)
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

    fn push_padding(&mut self, len: usize) {
        self.0.extend(std::iter::repeat(0).take(len));
    }
}
