use byteorder::ByteOrder;
use std::{error, fmt, str};

#[derive(Debug)]
pub enum VariantError {
    IncorrectType,
    InvalidUtf8,
    InsufficientData,
    UnsupportedType,
}

impl error::Error for VariantError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantError::IncorrectType => write!(f, "incorrect type"),
            VariantError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            VariantError::InsufficientData => write!(f, "insufficient data"),
            VariantError::UnsupportedType => write!(f, "unsupported type"),
        }
    }
}

pub trait VariantType<'a>: Sized {
    const SIGNATURE: char;
    const SIGNATURE_STR: &'static str;
    fn encode(&self) -> Vec<u8>;
    fn extract_slice(data: &'a [u8]) -> Result<&'a [u8], VariantError>;

    fn extract(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a;
}

impl<'a> VariantType<'a> for u32 {
    const SIGNATURE: char = 'u';
    const SIGNATURE_STR: &'static str = "u";

    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4);
        bytes.extend(&self.to_ne_bytes());

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..4])
    }

    fn extract(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        Ok(byteorder::NativeEndian::read_u32(bytes))
    }
}

impl<'a> VariantType<'a> for &'a str {
    const SIGNATURE: char = 's';
    const SIGNATURE_STR: &'static str = "s";

    fn encode(&self) -> Vec<u8> {
        let len = self.len();
        let mut bytes = Vec::with_capacity(5 + len);

        bytes.extend(&(len as u32).to_ne_bytes());
        bytes.extend(self.as_bytes());
        bytes.push(b'\0');

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = byteorder::NativeEndian::read_u32(bytes) as usize + 5;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn extract(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        if bytes.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        str::from_utf8(&bytes[4..]).map_err(|_| VariantError::InvalidUtf8)
    }
}
