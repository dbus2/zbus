use byteorder::ByteOrder;
use std::{error, fmt, str};

use crate::{ObjectPath, Signature};

#[derive(Debug)]
pub enum VariantError {
    ExcessData,
    IncorrectType,
    IncorrectValue,
    InvalidUtf8,
    InsufficientData,
    UnsupportedType(String),
}

impl error::Error for VariantError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantError::ExcessData => write!(f, "excess data"),
            VariantError::IncorrectType => write!(f, "incorrect type"),
            VariantError::IncorrectValue => write!(f, "incorrect value"),
            VariantError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            VariantError::InsufficientData => write!(f, "insufficient data"),
            VariantError::UnsupportedType(s) => {
                write!(f, "unsupported type (signature: \"{}\")", s)
            }
        }
    }
}

pub trait VariantType<'a>: Sized {
    const SIGNATURE: char;
    const SIGNATURE_STR: &'static str;
    const ALIGNMENT: u32;

    // FIXME: Would be nice if this returned a slice
    fn encode(&self) -> Vec<u8>;
    fn extract_slice(data: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError>;
    fn ensure_correct_signature(signature: &str) -> Result<(), VariantError> {
        if signature != Self::SIGNATURE_STR {
            return Err(VariantError::IncorrectType);
        }

        Ok(())
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a;

    fn get_signature<'b>(&'b self) -> &'b str {
        Self::SIGNATURE_STR
    }
}

pub trait SimpleVariantType<'a>: VariantType<'a> {
    fn extract_slice_simple(data: &'a [u8]) -> Result<&'a [u8], VariantError> {
        Self::extract_slice(data, Self::SIGNATURE_STR)
    }

    fn decode_simple(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::decode(bytes, Self::SIGNATURE_STR)
    }
}

impl<'a> VariantType<'a> for u8 {
    const SIGNATURE: char = 'y';
    const SIGNATURE_STR: &'static str = "y";
    const ALIGNMENT: u32 = 1;

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 1)?;

        Ok(&bytes[0..1])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 1)?;

        Ok(bytes[0])
    }
}
impl<'a> SimpleVariantType<'a> for u8 {}

impl<'a> VariantType<'a> for bool {
    const SIGNATURE: char = 'b';
    const SIGNATURE_STR: &'static str = "b";
    const ALIGNMENT: u32 = 4;

    fn encode(&self) -> Vec<u8> {
        (*self as u32).to_ne_bytes().to_vec()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(&bytes[0..4])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 4)?;

        match byteorder::NativeEndian::read_u32(bytes) {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(VariantError::IncorrectValue),
        }
    }
}
impl<'a> SimpleVariantType<'a> for bool {}

impl<'a> VariantType<'a> for i16 {
    const SIGNATURE: char = 'n';
    const SIGNATURE_STR: &'static str = "n";
    const ALIGNMENT: u32 = 2;

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 2)?;

        Ok(&bytes[0..2])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 2)?;

        Ok(byteorder::NativeEndian::read_i16(bytes))
    }
}
impl<'a> SimpleVariantType<'a> for i16 {}

impl<'a> VariantType<'a> for u16 {
    const SIGNATURE: char = 'q';
    const SIGNATURE_STR: &'static str = "q";
    const ALIGNMENT: u32 = 2;

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 2)?;

        Ok(&bytes[0..2])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 2)?;

        Ok(byteorder::NativeEndian::read_u16(bytes))
    }
}
impl<'a> SimpleVariantType<'a> for u16 {}

impl<'a> VariantType<'a> for i32 {
    const SIGNATURE: char = 'i';
    const SIGNATURE_STR: &'static str = "i";
    const ALIGNMENT: u32 = 4;

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(&bytes[0..4])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(byteorder::NativeEndian::read_i32(bytes))
    }
}
impl<'a> SimpleVariantType<'a> for i32 {}

impl<'a> VariantType<'a> for u32 {
    const SIGNATURE: char = 'u';
    const SIGNATURE_STR: &'static str = "u";
    const ALIGNMENT: u32 = 4;

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(&bytes[0..4])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 4)?;

        Ok(byteorder::NativeEndian::read_u32(bytes))
    }
}
impl<'a> SimpleVariantType<'a> for u32 {}

impl<'a> VariantType<'a> for i64 {
    const SIGNATURE: char = 'x';
    const SIGNATURE_STR: &'static str = "x";
    const ALIGNMENT: u32 = 8;

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(&bytes[0..8])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(byteorder::NativeEndian::read_i64(bytes))
    }
}
impl<'a> SimpleVariantType<'a> for i64 {}

impl<'a> VariantType<'a> for u64 {
    const SIGNATURE: char = 't';
    const SIGNATURE_STR: &'static str = "t";
    const ALIGNMENT: u32 = 8;

    fn encode(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(&bytes[0..8])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(byteorder::NativeEndian::read_u64(bytes))
    }
}
impl<'a> SimpleVariantType<'a> for u64 {}

impl<'a> VariantType<'a> for f64 {
    const SIGNATURE: char = 'd';
    const SIGNATURE_STR: &'static str = "d";
    const ALIGNMENT: u32 = 8;

    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![0; 8];
        byteorder::NativeEndian::write_f64(&mut bytes, *self);

        bytes
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(&bytes[0..8])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 8)?;

        Ok(byteorder::NativeEndian::read_f64(bytes))
    }
}
impl<'a> SimpleVariantType<'a> for f64 {}

impl<'a> VariantType<'a> for &'a str {
    const SIGNATURE: char = 's';
    const SIGNATURE_STR: &'static str = "s";
    const ALIGNMENT: u32 = 4;

    fn encode(&self) -> Vec<u8> {
        let len = self.len();
        let mut bytes = Vec::with_capacity(5 + len);

        bytes.extend(&(len as u32).to_ne_bytes());
        bytes.extend(self.as_bytes());
        bytes.push(b'\0');

        bytes
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 4)?;

        let last_index = byteorder::NativeEndian::read_u32(bytes) as usize + 5;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        Self::ensure_correct_signature(signature)?;
        ensure_sufficient_bytes(bytes, 4)?;

        let last_index = bytes.len() - 1;
        str::from_utf8(&bytes[4..last_index]).map_err(|_| VariantError::InvalidUtf8)
    }
}
impl<'a> SimpleVariantType<'a> for &'a str {}

fn ensure_sufficient_bytes(bytes: &[u8], size: usize) -> Result<(), VariantError> {
    if bytes.len() < size {
        return Err(VariantError::InsufficientData);
    }

    Ok(())
}

pub(crate) fn extract_slice_from_data<'a>(
    data: &'a [u8],
    signature: &str,
) -> Result<&'a [u8], VariantError> {
    match signature {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_STR => u8::extract_slice_simple(data),
        bool::SIGNATURE_STR => bool::extract_slice_simple(data),
        i16::SIGNATURE_STR => i16::extract_slice_simple(data),
        u16::SIGNATURE_STR => u16::extract_slice_simple(data),
        i32::SIGNATURE_STR => i32::extract_slice_simple(data),
        u32::SIGNATURE_STR => u32::extract_slice_simple(data),
        i64::SIGNATURE_STR => i64::extract_slice_simple(data),
        u64::SIGNATURE_STR => u64::extract_slice_simple(data),
        f64::SIGNATURE_STR => f64::extract_slice_simple(data),
        <(&str)>::SIGNATURE_STR => <(&str)>::extract_slice_simple(data),
        ObjectPath::SIGNATURE_STR => ObjectPath::extract_slice_simple(data),
        Signature::SIGNATURE_STR => Signature::extract_slice_simple(data),
        _ => return Err(VariantError::UnsupportedType(String::from(signature))),
    }
}

pub(crate) fn get_alignment_for_signature(signature: &str) -> Result<u32, VariantError> {
    match signature {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_STR => Ok(u8::ALIGNMENT),
        bool::SIGNATURE_STR => Ok(bool::ALIGNMENT),
        i16::SIGNATURE_STR => Ok(i16::ALIGNMENT),
        u16::SIGNATURE_STR => Ok(u16::ALIGNMENT),
        i32::SIGNATURE_STR => Ok(i32::ALIGNMENT),
        u32::SIGNATURE_STR => Ok(u32::ALIGNMENT),
        i64::SIGNATURE_STR => Ok(i64::ALIGNMENT),
        u64::SIGNATURE_STR => Ok(u64::ALIGNMENT),
        f64::SIGNATURE_STR => Ok(f64::ALIGNMENT),
        <(&str)>::SIGNATURE_STR => Ok(<(&str)>::ALIGNMENT),
        ObjectPath::SIGNATURE_STR => Ok(ObjectPath::ALIGNMENT),
        Signature::SIGNATURE_STR => Ok(Signature::ALIGNMENT),
        _ => return Err(VariantError::UnsupportedType(String::from(signature))),
    }
}
