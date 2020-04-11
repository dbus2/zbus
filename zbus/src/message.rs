use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::io::{Cursor, Error as IOError};

use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use zvariant::{EncodingFormat, Error as VariantError, FromVariant};
use zvariant::{Signature, VariantValue};

use crate::utils::padding_for_8_bytes;
use crate::{MessageField, MessageFieldCode, MessageFieldError, MessageFields};

const PRIMARY_HEADER_SIZE: usize = 12;
pub const MIN_MESSAGE_SIZE: usize = PRIMARY_HEADER_SIZE + 4;

const FIELDS_LEN_START_OFFSET: usize = 12;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize_repr, PartialEq, Serialize_repr)]
pub enum EndianSig {
    Big = b'B',
    Little = b'l',
}

// Such a shame I've to do this manually
impl TryFrom<u8> for EndianSig {
    type Error = MessageError;

    fn try_from(val: u8) -> Result<EndianSig, MessageError> {
        match val {
            b'B' => Ok(EndianSig::Big),
            b'l' => Ok(EndianSig::Little),
            _ => Err(MessageError::IncorrectEndian),
        }
    }
}

// FIXME: Use derive macro when it's available
impl VariantValue for EndianSig {
    fn signature() -> Signature<'static> {
        u8::signature()
    }
}

#[cfg(target_endian = "big")]
const NATIVE_ENDIAN_SIG: EndianSig = EndianSig::Big;
#[cfg(target_endian = "little")]
const NATIVE_ENDIAN_SIG: EndianSig = EndianSig::Little;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize_repr, PartialEq, Serialize_repr)]
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

// FIXME: Use derive macro when it's available
impl VariantValue for MessageType {
    fn signature() -> Signature<'static> {
        u8::signature()
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize_repr, PartialEq, Serialize_repr)]
pub enum MessageFlags {
    /// No flags.
    None = 0x0,
    /// This message does not expect method return replies or error replies, even if it is of a type
    /// that can have a reply; the reply should be omitted.
    ///
    /// Note that `MessageType::MethodCall` is the only message type currently defined in the
    /// specification that can expect a reply, so the presence or absence of this flag in the other
    /// three message types that are currently documented is meaningless: replies to those message
    /// types should not be sent, whether this flag is present or not.
    NoReplyExpected = 0x1,
    /// The bus must not launch an owner for the destination name in response to this message.
    NoAutoStart = 0x2,
    /// This flag may be set on a method call message to inform the receiving side that the caller
    /// is prepared to wait for interactive authorization, which might take a considerable time to
    /// complete. For instance, if this flag is set, it would be appropriate to query the user for
    /// passwords or confirmation via Polkit or a similar framework.
    AllowInteractiveAuth = 0x4,
}

// Such a shame I've to do this manually
impl From<u8> for MessageFlags {
    fn from(val: u8) -> MessageFlags {
        match val {
            0x1 => MessageFlags::NoReplyExpected,
            0x2 => MessageFlags::NoAutoStart,
            0x4 => MessageFlags::AllowInteractiveAuth,
            // According to the spec, unknown flags must be ignored
            _ => MessageFlags::None,
        }
    }
}

// FIXME: Use derive macro when it's available
impl VariantValue for MessageFlags {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagePrimaryHeader {
    endian_sig: EndianSig,
    msg_type: MessageType,
    flags: MessageFlags,
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
    pub fn endian_sig(&self) -> EndianSig {
        self.endian_sig
    }

    pub fn set_endian_sig(&mut self, sig: EndianSig) {
        self.endian_sig = sig;
    }

    pub fn msg_type(&self) -> MessageType {
        self.msg_type
    }

    pub fn set_msg_type(&mut self, msg_type: MessageType) {
        self.msg_type = msg_type;
    }

    pub fn flags(&self) -> MessageFlags {
        self.flags
    }

    pub fn set_flags(&mut self, flags: MessageFlags) {
        self.flags = flags;
    }

    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    pub fn set_protocol_version(&mut self, version: u8) {
        self.protocol_version = version;
    }

    pub fn body_len(&self) -> u32 {
        self.body_len
    }

    pub fn set_body_len(&mut self, len: u32) {
        self.body_len = len;
    }

    pub fn serial_num(&self) -> u32 {
        self.serial_num
    }

    pub fn set_serial_num(&mut self, serial: u32) {
        self.serial_num = serial;
    }
}

impl<'m> MessageHeader<'m> {
    pub fn primary(&self) -> &MessagePrimaryHeader {
        &self.primary
    }

    pub fn primary_mut(&mut self) -> &mut MessagePrimaryHeader {
        &mut self.primary
    }

    pub fn fields(&self) -> &MessageFields {
        &self.fields
    }

    pub fn fields_mut<'s: 'm>(&'s mut self) -> &'s mut MessageFields {
        &mut self.fields
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
        B: serde::ser::Serialize + VariantValue,
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
                signature = Signature::from(String::from(&signature[1..signature.len() - 1]));
            }
            fields.add(MessageField::signature(signature));
        }
        fields.add(MessageField::path(path));
        fields.add(MessageField::member(method_name));

        let format = EncodingFormat::DBus;
        let mut header = MessageHeader {
            primary: MessagePrimaryHeader {
                endian_sig: NATIVE_ENDIAN_SIG,
                msg_type: MessageType::MethodCall,
                flags: MessageFlags::None,
                protocol_version: 1,
                body_len: 0, // set to 0 at first
                serial_num: u32::max_value(),
            },
            fields,
            end: ((),),
        };
        zvariant::to_write_ne(&mut cursor, format, &header)?;

        let body_len = zvariant::to_write_ne(&mut cursor, format, body)?;
        if body_len > u32::max_value() as usize {
            return Err(MessageError::ExcessData);
        }
        let primary = header.primary_mut();
        primary.set_body_len(body_len as u32);
        cursor.set_position(0);
        zvariant::to_write_ne(&mut cursor, format, primary)?;

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

    pub fn modify_primary_header<F>(&mut self, mut modifier: F) -> Result<(), MessageError>
    where
        F: FnMut(&mut MessagePrimaryHeader) -> Result<(), MessageError>,
    {
        let mut primary = self.primary_header()?;
        modifier(&mut primary)?;

        let mut cursor = Cursor::new(&mut self.0);
        zvariant::to_write_ne(&mut cursor, EncodingFormat::DBus, &primary)
            .map(|_| ())
            .map_err(MessageError::from)
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

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    fn fields_len(&self) -> Result<usize, MessageError> {
        zvariant::from_slice_ne::<u32>(&self.0[FIELDS_LEN_START_OFFSET..], EncodingFormat::DBus)
            .map(|v| v as usize)
            .map_err(MessageError::from)
    }
}
