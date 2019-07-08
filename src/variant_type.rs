use byteorder::ByteOrder;
use std::{error, fmt, str};

#[derive(Debug)]
pub enum VariantError {
    IncorrectType,
    IncorrectValue,
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
            VariantError::IncorrectValue => write!(f, "incorrect value"),
            VariantError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            VariantError::InsufficientData => write!(f, "insufficient data"),
            VariantError::UnsupportedType => write!(f, "unsupported type"),
        }
    }
}

pub trait VariantType<'a>: Sized {
    const SIGNATURE: char;
    const SIGNATURE_STR: &'static str;

    // FIXME: Would be nice if this returned a slice
    fn encode(&self) -> Vec<u8>;
    fn extract_slice(data: &'a [u8]) -> Result<&'a [u8], VariantError>;

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a;
}

impl<'a> VariantType<'a> for u8 {
    const SIGNATURE: char = 'y';
    const SIGNATURE_STR: &'static str = "y";

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().iter().cloned().collect::<Vec<u8>>()
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 1)?;

        Ok(&bytes[0..1])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 1)?;

        Ok(bytes[0])
    }
}

impl<'a> VariantType<'a> for bool {
    const SIGNATURE: char = 'b';
    const SIGNATURE_STR: &'static str = "b";

    fn encode(&self) -> Vec<u8> {
        (*self as u32).to_ne_bytes().iter().cloned().collect()
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(&bytes[0..4])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 4)?;

        match byteorder::NativeEndian::read_u32(bytes) {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(VariantError::IncorrectValue),
        }
    }
}

impl<'a> VariantType<'a> for i16 {
    const SIGNATURE: char = 'n';
    const SIGNATURE_STR: &'static str = "n";

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().iter().cloned().collect()
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 2)?;

        Ok(&bytes[0..2])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 2)?;

        Ok(byteorder::NativeEndian::read_i16(bytes))
    }
}

impl<'a> VariantType<'a> for u16 {
    const SIGNATURE: char = 'q';
    const SIGNATURE_STR: &'static str = "q";

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().iter().cloned().collect()
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 2)?;

        Ok(&bytes[0..2])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 2)?;

        Ok(byteorder::NativeEndian::read_u16(bytes))
    }
}

impl<'a> VariantType<'a> for i32 {
    const SIGNATURE: char = 'i';
    const SIGNATURE_STR: &'static str = "i";

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().iter().cloned().collect()
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(&bytes[0..4])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(byteorder::NativeEndian::read_i32(bytes))
    }
}

impl<'a> VariantType<'a> for u32 {
    const SIGNATURE: char = 'u';
    const SIGNATURE_STR: &'static str = "u";

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().iter().cloned().collect()
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(&bytes[0..4])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(byteorder::NativeEndian::read_u32(bytes))
    }
}

impl<'a> VariantType<'a> for i64 {
    const SIGNATURE: char = 'x';
    const SIGNATURE_STR: &'static str = "x";

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().iter().cloned().collect()
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(&bytes[0..8])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(byteorder::NativeEndian::read_i64(bytes))
    }
}

impl<'a> VariantType<'a> for u64 {
    const SIGNATURE: char = 't';
    const SIGNATURE_STR: &'static str = "t";

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().iter().cloned().collect()
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(&bytes[0..8])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(byteorder::NativeEndian::read_u64(bytes))
    }
}

impl<'a> VariantType<'a> for f64 {
    const SIGNATURE: char = 'd';
    const SIGNATURE_STR: &'static str = "d";

    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![0; 8];
        byteorder::NativeEndian::write_f64(&mut bytes, *self);

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(&bytes[0..8])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(byteorder::NativeEndian::read_f64(bytes))
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
        ensure_sufficient_bytes(bytes, 4)?;

        let last_index = byteorder::NativeEndian::read_u32(bytes) as usize + 5;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        ensure_sufficient_bytes(bytes, 4)?;

        let last_index = bytes.len() - 1;
        str::from_utf8(&bytes[4..last_index]).map_err(|_| VariantError::InvalidUtf8)
    }
}

fn ensure_sufficient_bytes(bytes: &[u8], size: usize) -> Result<(), VariantError> {
    if bytes.len() < size {
        return Err(VariantError::InsufficientData);
    }

    Ok(())
}
