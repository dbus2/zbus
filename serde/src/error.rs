use serde::ser;
use std::{error, fmt, result};

#[derive(Debug)]
pub enum Error {
    // Generic error needed by Serde
    Message(String),

    Io(std::io::Error),
    ExcessData,
    IncorrectType,
    IncorrectValue,
    InvalidUtf8,
    InsufficientData,
    InvalidSignature(String),
    UnsupportedType(String),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(s) => write!(f, "{}", s),
            Error::Io(e) => e.fmt(f),
            Error::ExcessData => write!(f, "excess data"),
            Error::IncorrectType => write!(f, "incorrect type"),
            Error::IncorrectValue => write!(f, "incorrect value"),
            Error::InvalidUtf8 => write!(f, "invalid UTF-8"),
            Error::InsufficientData => write!(f, "insufficient data"),
            Error::InvalidSignature(s) => write!(f, "invalid signature: \"{}\"", s.as_str()),
            Error::UnsupportedType(s) => {
                write!(f, "unsupported type (signature: \"{}\")", s.as_str())
            }
        }
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
