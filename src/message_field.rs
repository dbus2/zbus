use std::fmt;

use crate::variant::{ObjectPath, Signature, Variant, VariantError};

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

pub struct MessageField<'a> {
    pub code: MessageFieldCode,
    pub value: Variant<'a>,
}

impl<'a> MessageField<'a> {
    pub fn path(path: &'a str) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::Path,
            value: Variant::from(ObjectPath(path)),
        }
    }

    pub fn interface(interface: &'a str) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::Interface,
            value: Variant::from(interface),
        }
    }

    pub fn member(member: &'a str) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::Member,
            value: Variant::from(member),
        }
    }

    pub fn error_name(error_name: &'a str) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::ErrorName,
            value: Variant::from(error_name),
        }
    }

    pub fn reply_serial(serial: u32) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::ReplySerial,
            value: Variant::from(serial),
        }
    }

    pub fn destination(destination: &'a str) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::Destination,
            value: Variant::from(destination),
        }
    }

    pub fn sender(sender: &'a str) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::Sender,
            value: Variant::from(sender),
        }
    }

    pub fn signature(signature: &'a str) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::Signature,
            value: Variant::from(Signature(signature)),
        }
    }

    pub fn unix_fds(fd: u32) -> Self
    where
        Self: 'a,
    {
        Self {
            code: MessageFieldCode::UnixFDs,
            value: Variant::from(fd),
        }
    }

    pub fn from_data(data: &'a [u8]) -> Result<(Self, usize), MessageFieldError> {
        let code = MessageFieldCode::from(data[0]);
        let signature =
            String::from_utf8(data[2..3].into()).map_err(|_| MessageFieldError::InvalidUtf8)?;

        let value = Variant::from_data(&data[4..], &signature)
            .map_err(|e| MessageFieldError::Variant(e))?;
        let len = 4 + value.len();

        Ok((Self { code, value }, len))
    }

    pub fn encode(&self) -> Result<Vec<u8>, MessageFieldError> {
        let mut bytes = Vec::with_capacity(4 + self.value.len());

        // Signature
        bytes.push(self.code as u8);
        bytes.push(1);
        bytes.push(
            self.value
                .get_signature()
                .chars()
                .nth(0)
                .ok_or_else(|| MessageFieldError::InsufficientData)? as u8,
        );
        bytes.push(b'\0');

        // Value
        bytes.extend(self.value.get_bytes());

        Ok(bytes)
    }
}
