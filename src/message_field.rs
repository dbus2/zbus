use std::borrow::Cow;
use std::error;
use std::fmt;

use crate::{ObjectPath, Signature, Structure, StructureBuilder, VariantError};
use crate::{Variant, VariantType, VariantTypeConstants};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum MessageFieldCode {
    Invalid = 0,     // Not a valid field name.
    Path = 1,        // The object to send a call to, or the object a signal is emitted from.
    Interface = 2,   // The interface to invoke a method call on, or that a signal is emitted from.
    Member = 3,      // The member, either the method name or signal name.
    ErrorName = 4,   // The name of the error that occurred, for errors
    ReplySerial = 5, //	The serial number of the message this message is a reply to.
    Destination = 6, // The name of the connection this message is intended for.
    Sender = 7,      // Unique name of the sending connection.
    Signature = 8,   // The signature of the message body.
    UnixFDs = 9,     // The number of Unix file descriptors that accompany the message.
}

impl From<u8> for MessageFieldCode {
    fn from(val: u8) -> MessageFieldCode {
        match val {
            1 => MessageFieldCode::Path,
            2 => MessageFieldCode::Interface,
            3 => MessageFieldCode::Member,
            4 => MessageFieldCode::ErrorName,
            5 => MessageFieldCode::ReplySerial,
            6 => MessageFieldCode::Destination,
            7 => MessageFieldCode::Sender,
            8 => MessageFieldCode::Signature,
            9 => MessageFieldCode::UnixFDs,
            _ => MessageFieldCode::Invalid,
        }
    }
}

#[derive(Debug)]
pub enum MessageFieldError {
    InsufficientData,
    InvalidCode,
    InvalidUtf8,
    Variant(VariantError),
}

impl error::Error for MessageFieldError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            MessageFieldError::Variant(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for MessageFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageFieldError::InsufficientData => write!(f, "insufficient data"),
            MessageFieldError::InvalidCode => write!(f, "invalid field code"),
            MessageFieldError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            MessageFieldError::Variant(e) => write!(f, "{}", e),
        }
    }
}

impl From<VariantError> for MessageFieldError {
    fn from(val: VariantError) -> MessageFieldError {
        MessageFieldError::Variant(val)
    }
}

#[derive(Debug)]
pub struct MessageField<'a>(Structure<'a>);

impl<'a> MessageField<'a> {
    pub fn code(&self) -> Result<MessageFieldCode, MessageFieldError> {
        let fields = self.0.fields();
        if fields.len() < 2 {
            return Err(MessageFieldError::InsufficientData);
        }

        Ok(fields[0]
            .get::<u8>()
            .map(|c| MessageFieldCode::from(c))
            .unwrap_or(MessageFieldCode::Invalid))
    }

    pub fn value(&self) -> Result<Variant, MessageFieldError> {
        self.0.fields()[1].get::<Variant>().map_err(|e| e.into())
    }

    pub fn path(path: &str) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::Path as u8)
                .add_field(Variant::from(ObjectPath::new(path)))
                .create(),
        )
    }

    pub fn interface(interface: &str) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::Interface as u8)
                .add_field(Variant::from(interface))
                .create(),
        )
    }

    pub fn member(member: &str) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::Member as u8)
                .add_field(Variant::from(member))
                .create(),
        )
    }

    pub fn error_name(error_name: &str) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::ErrorName as u8)
                .add_field(Variant::from(error_name))
                .create(),
        )
    }

    pub fn reply_serial(serial: u32) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::ReplySerial as u8)
                .add_field(Variant::from(serial))
                .create(),
        )
    }

    pub fn destination(destination: &str) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::Destination as u8)
                .add_field(Variant::from(destination))
                .create(),
        )
    }

    pub fn sender(sender: &str) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::Sender as u8)
                .add_field(Variant::from(sender))
                .create(),
        )
    }

    pub fn signature(signature: &str) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::Signature as u8)
                .add_field(Variant::from(Signature::new(signature)))
                .create(),
        )
    }

    pub fn unix_fds(fd: u32) -> Self {
        Self(
            StructureBuilder::new()
                .add_field(MessageFieldCode::UnixFDs as u8)
                .add_field(Variant::from(fd))
                .create(),
        )
    }
}

impl<'a> VariantTypeConstants for MessageField<'a> {
    const SIGNATURE_CHAR: char = Structure::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = Structure::SIGNATURE_STR;
    const ALIGNMENT: usize = Structure::ALIGNMENT;
}

// FIXME: Try automating this when we've delegation: https://github.com/rust-lang/rfcs/pull/2393
impl<'a> VariantType<'a> for MessageField<'a> {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        self.0.encode(n_bytes_before)
    }

    fn slice_data<'b>(
        bytes: &'b [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        Structure::slice_data(bytes, signature, n_bytes_before)
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        Structure::decode(bytes, signature, n_bytes_before).map(|s| MessageField(s))
    }

    fn ensure_correct_signature(signature: &str) -> Result<(), VariantError> {
        Structure::ensure_correct_signature(signature)
    }

    fn signature<'b>(&'b self) -> Cow<'b, str> {
        self.0.signature()
    }

    fn slice_signature(signature: &str) -> Result<&str, VariantError> {
        Structure::slice_signature(signature)
    }
}
