use std::fmt::Display;

/// The error type for `zbus_polkit`.
///
/// The various errors that can be reported by this crate.
#[derive(Debug)]
pub enum Error {
    /// I/O errors.
    Io(std::io::Error),

    /// Could not parse a number for a Process ID or an User ID.
    ParseInt(std::num::ParseIntError),

    /// Could not retrieve/deserialize sender header of the message.
    BadSender(zbus::MessageError),

    /// Missing sender header in the message.
    MissingSender,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseInt(error)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::ParseInt(e) => Some(e),
            Error::BadSender(e) => Some(e),
            Error::MissingSender => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => e.fmt(f),
            Error::ParseInt(e) => e.fmt(f),
            Error::BadSender(e) => e.fmt(f),
            Error::MissingSender => write!(f, "sender header field missing in the message",),
        }
    }
}
