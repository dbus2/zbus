use serde::{de, ser};
use std::{error, fmt, result};

#[derive(Debug)]
pub enum Error {
    // Generic error needed by Serde
    Message(String),

    Io(std::io::Error),
    IncorrectType,
    Utf8(std::str::Utf8Error),
    PaddingNot0(u8),
    UnknownFd,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Utf8(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(s) => write!(f, "{}", s),
            Error::Io(e) => e.fmt(f),
            Error::IncorrectType => write!(f, "incorrect type"),
            Error::Utf8(e) => write!(f, "{}", e),
            Error::PaddingNot0(b) => write!(f, "Unexpected non-0 padding byte `{}`", b),
            Error::UnknownFd => write!(f, "File descriptor not in the given FD index"),
        }
    }
}

impl de::Error for Error {
    // TODO: Add more specific error variants to Error enum above so we can implement other methods
    // here too.
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
