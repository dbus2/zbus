use static_assertions::assert_impl_all;
use std::{convert::Infallible, error, fmt, io, sync::Arc};
use zvariant::Error as VariantError;

use crate::{fdo, Message, MessageError, MessageType};

/// The error type for `zbus`.
///
/// The various errors that can be reported by this crate.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Error {
    /// Interface not found
    InterfaceNotFound,
    /// Invalid D-Bus address.
    Address(String),
    /// An I/O error.
    Io(io::Error),
    /// Message parsing error.
    Message(MessageError),
    /// A [zvariant](../zvariant/index.html) error.
    Variant(VariantError),
    /// Initial handshake error.
    Handshake(String),
    /// Unexpected or incorrect reply.
    InvalidReply,
    /// A D-Bus method error reply.
    // According to the spec, there can be all kinds of details in D-Bus errors but nobody adds anything more than a
    // string description.
    MethodError(String, Option<String>, Arc<Message>),
    /// Invalid D-Bus GUID.
    InvalidGUID,
    /// Unsupported function, or support currently lacking.
    Unsupported,
    /// A [`fdo::Error`] transformed into [`Error`].
    FDO(Box<fdo::Error>),
    #[cfg(feature = "xml")]
    /// An XML error
    SerdeXml(serde_xml_rs::Error),
    /// Only exists to allow `TryFrom<T> for T` conversions. You should never actually be getting
    /// this error from any API.
    Infallible,
}

assert_impl_all!(Error: Send, Sync, Unpin);

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Error::Io(_) => false,
            _ => self == other,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::InterfaceNotFound => None,
            Error::Address(_) => None,
            Error::Io(e) => Some(e),
            Error::Handshake(_) => None,
            Error::Message(e) => Some(e),
            Error::Variant(e) => Some(e),
            Error::InvalidReply => None,
            Error::MethodError(_, _, _) => None,
            Error::InvalidGUID => None,
            Error::Unsupported => None,
            Error::FDO(e) => Some(e),
            #[cfg(feature = "xml")]
            Error::SerdeXml(e) => Some(e),
            Error::Infallible => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InterfaceNotFound => write!(f, "Interface not found"),
            Error::Address(e) => write!(f, "address error: {}", e),
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Handshake(e) => write!(f, "D-Bus handshake failed: {}", e),
            Error::Message(e) => write!(f, "Message creation error: {}", e),
            Error::Variant(e) => write!(f, "{}", e),
            Error::InvalidReply => write!(f, "Invalid D-Bus method reply"),
            Error::MethodError(name, detail, _reply) => write!(
                f,
                "{}: {}",
                name,
                detail.as_ref().map(|s| s.as_str()).unwrap_or("no details")
            ),
            Error::InvalidGUID => write!(f, "Invalid GUID"),
            Error::Unsupported => write!(f, "Connection support is lacking"),
            Error::FDO(e) => write!(f, "{}", e),
            #[cfg(feature = "xml")]
            Error::SerdeXml(e) => write!(f, "XML error: {}", e),
            Error::Infallible => write!(f, "Infallible conversion failed"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Error::Io(val)
    }
}

impl From<nix::Error> for Error {
    fn from(val: nix::Error) -> Self {
        val.as_errno()
            .map(|errno| io::Error::from_raw_os_error(errno as i32).into())
            .unwrap_or_else(|| io::Error::new(io::ErrorKind::Other, val).into())
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

impl From<fdo::Error> for Error {
    fn from(val: fdo::Error) -> Self {
        match val {
            fdo::Error::ZBus(e) => e,
            e => Error::FDO(Box::new(e)),
        }
    }
}

#[cfg(feature = "xml")]
impl From<serde_xml_rs::Error> for Error {
    fn from(val: serde_xml_rs::Error) -> Self {
        Error::SerdeXml(val)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Error::Infallible
    }
}

// For messages that are D-Bus error returns
impl From<Message> for Error {
    fn from(message: Message) -> Error {
        Self::from(Arc::new(message))
    }
}

impl From<Arc<Message>> for Error {
    fn from(message: Arc<Message>) -> Error {
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
            match message.body_unchecked::<&str>() {
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
