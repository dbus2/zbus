use serde::{de, ser};
use std::{error, fmt, result};

// TODO: See if we can avoid allocated String
#[derive(Debug)]
pub enum Error {
    // Generic error needed by Serde
    Message(String),

    Io(std::io::Error),
    IncorrectType,
    Utf8(std::str::Utf8Error),
    InsufficientData,
    PaddingNot0,
    UnknownFd,
    InvalidSignature(String),
    InvalidObjectPath(String),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // FIXME: is it true for Error::Io as well?
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(s) => write!(f, "{}", s),
            Error::Io(e) => e.fmt(f),
            Error::IncorrectType => write!(f, "incorrect type"),
            Error::Utf8(e) => write!(f, "{}", e),
            Error::InsufficientData => write!(f, "insufficient data"),
            Error::PaddingNot0 => write!(f, "non-0 padding byte(s)"),
            Error::UnknownFd => write!(f, "File descriptor not in the given FD index"),
            Error::InvalidSignature(s) => write!(f, "invalid signature: \"{}\"", s),
            Error::InvalidObjectPath(s) => write!(f, "invalid object path: \"{}\"", s),
        }
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Error
    where
        T: fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Error
    where
        T: fmt::Display,
    {
        Error::Message(msg.to_string())
    }
}

/// Alias for a `Result` with the error type `zvariant::Error`.
pub type Result<T> = result::Result<T, Error>;
