use std::{error, fmt, io};
use zvariant::Error as VariantError;

use crate::{AddressError, Message, MessageError, MessageType};

#[derive(Debug)]
pub enum Error {
    Address(AddressError),
    IO(io::Error),
    Message(MessageError),
    Variant(VariantError),
    Handshake,
    InvalidReply,
    // According to the spec, there can be all kinds of details in D-Bus errors but nobody adds anything more than a
    // string description.
    MethodError(String, Option<String>, Message),
    Unsupported,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Address(e) => Some(e),
            Error::IO(e) => Some(e),
            Error::Handshake => None,
            Error::Message(e) => Some(e),
            Error::Variant(e) => Some(e),
            Error::InvalidReply => None,
            Error::MethodError(_, _, _) => None,
            Error::Unsupported => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Address(e) => write!(f, "address error: {}", e),
            Error::IO(e) => write!(f, "I/O error: {}", e),
            Error::Handshake => write!(f, "D-Bus handshake failed"),
            Error::Message(e) => write!(f, "Message creation error: {}", e),
            Error::Variant(e) => write!(f, "{}", e),
            Error::InvalidReply => write!(f, "Invalid D-Bus method reply"),
            Error::MethodError(name, detail, _reply) => write!(
                f,
                "{}: {}",
                name,
                detail.as_ref().map(|s| s.as_str()).unwrap_or("no details")
            ),
            Error::Unsupported => write!(f, "Connection support is lacking"),
        }
    }
}

impl From<AddressError> for Error {
    fn from(val: AddressError) -> Self {
        Error::Address(val)
    }
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Error::IO(val)
    }
}

impl From<MessageError> for Error {
    fn from(val: MessageError) -> Self {
        Error::Message(val)
    }
}

impl From<VariantError> for Error {
    fn from(val: VariantError) -> Self {
        Error::Variant(val)
    }
}

// For messages that are D-Bus error returns
impl From<Message> for Error {
    fn from(message: Message) -> Error {
        // FIXME: Instead of checking this, we should have Method as trait and specific types for
        // each message type.
        let header = match message.header() {
            Ok(header) => header,
            Err(e) => {
                return Error::Message(e);
            }
        };
        if header.primary().msg_type() != MessageType::Error {
            return Error::InvalidReply;
        }

        if let Ok(Some(name)) = header.error_name() {
            let name = String::from(name);
            match message.body::<&str>() {
                Ok(detail) => Error::MethodError(name, Some(String::from(detail)), message),
                Err(_) => Error::MethodError(name, None, message),
            }
        } else {
            Error::InvalidReply
        }
    }
}

/// Alias for a `Result` with the error type `zbus::Error`.
pub type Result<T> = std::result::Result<T, Error>;
