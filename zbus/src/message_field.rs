use core::convert::TryFrom;

use std::error;
use std::fmt;

use zvariant::{Decode, Encode};
use zvariant::{ObjectPath, Signature, Structure};
use zvariant::{Variant, VariantError};

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
pub struct MessageField(Structure);

impl MessageField {
    pub fn code(&self) -> Result<MessageFieldCode, MessageFieldError> {
        let fields = self.0.fields();
        if fields.len() < 2 {
            return Err(MessageFieldError::InsufficientData);
        }

        Ok(u8::from_variant(&self.0.fields()[0])
            .map(|c| MessageFieldCode::from(*c))
            .unwrap_or(MessageFieldCode::Invalid))
    }

    pub fn value(&self) -> Result<&Variant, MessageFieldError> {
        Ok(Variant::from_variant(&self.0.fields()[1])?)
    }

    pub fn inner(&self) -> &Structure {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Structure {
        &mut self.0
    }

    pub fn into_inner(self) -> Structure {
        self.0
    }

    pub fn path(path: impl Into<ObjectPath>) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::Path as u8)
                .add_field(path.into().to_variant()),
        )
    }

    pub fn interface(interface: &str) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::Interface as u8)
                .add_field(String::from(interface).to_variant()),
        )
    }

    pub fn member(member: &str) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::Member as u8)
                .add_field(String::from(member).to_variant()),
        )
    }

    pub fn error_name(error_name: &str) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::ErrorName as u8)
                .add_field(String::from(error_name).to_variant()),
        )
    }

    pub fn reply_serial(serial: u32) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::ReplySerial as u8)
                .add_field(serial.to_variant()),
        )
    }

    pub fn destination(destination: &str) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::Destination as u8)
                .add_field(String::from(destination).to_variant()),
        )
    }

    pub fn sender(sender: &str) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::Sender as u8)
                .add_field(String::from(sender).to_variant()),
        )
    }

    pub fn signature(signature: impl Into<Signature>) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::Signature as u8)
                .add_field(signature.into().to_variant()),
        )
    }

    pub fn unix_fds(fd: u32) -> Self {
        Self(
            Structure::new()
                .add_field(MessageFieldCode::UnixFDs as u8)
                .add_field(fd.to_variant()),
        )
    }
}

impl std::ops::Deref for MessageField {
    type Target = Structure;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl std::ops::DerefMut for MessageField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl TryFrom<Structure> for MessageField {
    type Error = MessageFieldError;

    fn try_from(structure: Structure) -> Result<Self, MessageFieldError> {
        let field = MessageField(structure);

        // Ensure there is a valid code & variant payload
        let _ = field.code()?;
        let _ = field.value()?;

        Ok(field)
    }
}
