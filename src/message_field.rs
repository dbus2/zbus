use std::borrow::Cow;
use std::error;
use std::fmt;

use crate::{ObjectPath, Signature, Structure, VariantError};
use crate::{Variant, VariantType};

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
    pub fn code(&self) -> MessageFieldCode {
        self.0.fields()[0]
            .get::<u8>()
            .map(|c| MessageFieldCode::from(c))
            .unwrap_or(MessageFieldCode::Invalid)
    }

    pub fn value<'b>(&'b self) -> Result<Variant<'b>, MessageFieldError> {
        self.0.fields()[1].get::<Variant>().map_err(|e| e.into())
    }

    pub fn path(path: &'a str) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::Path as u8),
            Variant::from(Variant::from(ObjectPath::new(path))),
        ]))
    }

    pub fn interface(interface: &'a str) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::Interface as u8),
            Variant::from(Variant::from(interface)),
        ]))
    }

    pub fn member(member: &'a str) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::Member as u8),
            Variant::from(Variant::from(member)),
        ]))
    }

    pub fn error_name(error_name: &'a str) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::ErrorName as u8),
            Variant::from(Variant::from(error_name)),
        ]))
    }

    pub fn reply_serial(serial: u32) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::ReplySerial as u8),
            Variant::from(Variant::from(serial)),
        ]))
    }

    pub fn destination(destination: &'a str) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::Destination as u8),
            Variant::from(Variant::from(destination)),
        ]))
    }

    pub fn sender(sender: &'a str) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::Sender as u8),
            Variant::from(Variant::from(sender)),
        ]))
    }

    pub fn signature(signature: &'a str) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::Signature as u8),
            Variant::from(Variant::from(Signature::new(signature))),
        ]))
    }

    pub fn unix_fds(fd: u32) -> Self
    where
        Self: 'a,
    {
        Self(Structure::new(vec![
            Variant::from(MessageFieldCode::UnixFDs as u8),
            Variant::from(Variant::from(fd)),
        ]))
    }
}

// FIXME: Try automating this when we've delegation: https://github.com/rust-lang/rfcs/pull/2393
impl<'a> VariantType<'a> for MessageField<'a> {
    const SIGNATURE: char = Structure::SIGNATURE;
    const SIGNATURE_STR: &'static str = Structure::SIGNATURE_STR;
    const ALIGNMENT: usize = Structure::ALIGNMENT;

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
