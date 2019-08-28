use byteorder::ByteOrder;
use std::{borrow::Cow, error, fmt, str};

use crate::utils::padding_for_n_bytes;
use crate::{ObjectPath, Signature, Structure, Variant};

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

pub trait VariantType<'a>: Sized + std::fmt::Debug {
    const SIGNATURE: char;
    const SIGNATURE_STR: &'static str;
    const ALIGNMENT: u32;

    // FIXME: Would be nice if this returned a slice
    fn encode(&self, n_bytes_before: usize) -> Vec<u8>;

    // Default implementation works for constant-sized types where size is the same as their
    // alignment
    fn slice_data<'b>(
        bytes: &'b [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        let len = (Self::ALIGNMENT + padding_for_n_bytes(n_bytes_before as u32, Self::ALIGNMENT))
            as usize;
        ensure_sufficient_bytes(bytes, len)?;

        Ok(&bytes[0..len])
    }
    fn ensure_correct_signature(signature: &str) -> Result<(), VariantError> {
        if signature != Self::SIGNATURE_STR {
            return Err(VariantError::IncorrectType);
        }

        Ok(())
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError>;

    fn signature<'b>(&'b self) -> Cow<'b, str> {
        Cow::from(Self::SIGNATURE_STR)
    }

    fn slice_signature(signature: &str) -> Result<&str, VariantError> {
        let slice = &signature[0..1];
        Self::ensure_correct_signature(slice)?;

        Ok(slice)
    }

    fn create_bytes_vec(n_bytes_before: usize) -> Vec<u8> {
        let padding = padding_for_n_bytes(n_bytes_before as u32, Self::ALIGNMENT);

        std::iter::repeat(0).take((padding) as usize).collect()
    }

    // Helper for decode() implementation
    fn slice_for_decoding<'b>(
        bytes: &'b [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        let padding = Self::padding(n_bytes_before);
        let len = Self::ALIGNMENT as usize + padding;
        ensure_sufficient_bytes(bytes, len)?;

        Ok(&bytes[padding..])
    }

    fn padding(n_bytes_before: usize) -> usize {
        padding_for_n_bytes(n_bytes_before as u32, Self::ALIGNMENT) as usize
    }
}

pub trait SimpleVariantType<'a>: VariantType<'a> {
    fn slice_data_simple<'b>(
        data: &'b [u8],
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        Self::slice_data(data, Self::SIGNATURE_STR, n_bytes_before)
    }

    fn decode_simple(bytes: &'a [u8], n_bytes_before: usize) -> Result<Self, VariantError> {
        Self::decode(bytes, Self::SIGNATURE_STR, n_bytes_before)
    }
}

impl<'a> VariantType<'a> for u8 {
    const SIGNATURE: char = 'y';
    const SIGNATURE_STR: &'static str = "y";
    const ALIGNMENT: u32 = 1;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        v.extend(&self.to_ne_bytes());

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        Ok(slice[0])
    }
}
impl<'a> SimpleVariantType<'a> for u8 {}

impl<'a> VariantType<'a> for bool {
    const SIGNATURE: char = 'b';
    const SIGNATURE_STR: &'static str = "b";
    const ALIGNMENT: u32 = 4;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        v.extend(&(*self as u32).to_ne_bytes());

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        match byteorder::NativeEndian::read_u32(&slice) {
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

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        v.extend(&self.to_ne_bytes());

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        Ok(byteorder::NativeEndian::read_i16(slice))
    }
}
impl<'a> SimpleVariantType<'a> for i16 {}

impl<'a> VariantType<'a> for u16 {
    const SIGNATURE: char = 'q';
    const SIGNATURE_STR: &'static str = "q";
    const ALIGNMENT: u32 = 2;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        v.extend(&self.to_ne_bytes());

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        Ok(byteorder::NativeEndian::read_u16(slice))
    }
}
impl<'a> SimpleVariantType<'a> for u16 {}

impl<'a> VariantType<'a> for i32 {
    const SIGNATURE: char = 'i';
    const SIGNATURE_STR: &'static str = "i";
    const ALIGNMENT: u32 = 4;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        v.extend(&self.to_ne_bytes());

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        Ok(byteorder::NativeEndian::read_i32(slice))
    }
}
impl<'a> SimpleVariantType<'a> for i32 {}

impl<'a> VariantType<'a> for u32 {
    const SIGNATURE: char = 'u';
    const SIGNATURE_STR: &'static str = "u";
    const ALIGNMENT: u32 = 4;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        v.extend(&self.to_ne_bytes());

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        Ok(byteorder::NativeEndian::read_u32(slice))
    }
}
impl<'a> SimpleVariantType<'a> for u32 {}

impl<'a> VariantType<'a> for i64 {
    const SIGNATURE: char = 'x';
    const SIGNATURE_STR: &'static str = "x";
    const ALIGNMENT: u32 = 8;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        v.extend(&self.to_ne_bytes());

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        Ok(byteorder::NativeEndian::read_i64(slice))
    }
}
impl<'a> SimpleVariantType<'a> for i64 {}

impl<'a> VariantType<'a> for u64 {
    const SIGNATURE: char = 't';
    const SIGNATURE_STR: &'static str = "t";
    const ALIGNMENT: u32 = 8;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        v.extend(&self.to_ne_bytes());

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        Ok(byteorder::NativeEndian::read_u64(slice))
    }
}
impl<'a> SimpleVariantType<'a> for u64 {}

impl<'a> VariantType<'a> for f64 {
    const SIGNATURE: char = 'd';
    const SIGNATURE_STR: &'static str = "d";
    const ALIGNMENT: u32 = 8;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);
        let mut bytes = [0; 8];
        byteorder::NativeEndian::write_f64(&mut bytes, *self);
        v.extend(&bytes);

        v
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        Ok(byteorder::NativeEndian::read_f64(slice))
    }
}
impl<'a> SimpleVariantType<'a> for f64 {}

pub(crate) fn ensure_sufficient_bytes(bytes: &[u8], size: usize) -> Result<(), VariantError> {
    if bytes.len() < size {
        return Err(VariantError::InsufficientData);
    }

    Ok(())
}

pub(crate) fn slice_data<'a>(
    data: &'a [u8],
    signature: &str,
    n_bytes_before: usize,
) -> Result<&'a [u8], VariantError> {
    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE => u8::slice_data_simple(data, n_bytes_before),
        bool::SIGNATURE => bool::slice_data_simple(data, n_bytes_before),
        i16::SIGNATURE => i16::slice_data_simple(data, n_bytes_before),
        u16::SIGNATURE => u16::slice_data_simple(data, n_bytes_before),
        i32::SIGNATURE => i32::slice_data_simple(data, n_bytes_before),
        u32::SIGNATURE => u32::slice_data_simple(data, n_bytes_before),
        i64::SIGNATURE => i64::slice_data_simple(data, n_bytes_before),
        u64::SIGNATURE => u64::slice_data_simple(data, n_bytes_before),
        f64::SIGNATURE => f64::slice_data_simple(data, n_bytes_before),
        <(&str)>::SIGNATURE => <(&str)>::slice_data_simple(data, n_bytes_before),
        // Doesn't matter what type for T we use here, signature is the same but we're also assuming `slice_data` to
        // be independent of `T` (an internal detail).
        Vec::<bool>::SIGNATURE => Vec::<bool>::slice_data(data, signature, n_bytes_before),
        ObjectPath::SIGNATURE => ObjectPath::slice_data_simple(data, n_bytes_before),
        Signature::SIGNATURE => Signature::slice_data_simple(data, n_bytes_before),
        Structure::SIGNATURE => Structure::slice_data(data, signature, n_bytes_before),
        Variant::SIGNATURE => Variant::slice_data(data, signature, n_bytes_before),
        _ => return Err(VariantError::UnsupportedType(String::from(signature))),
    }
}

pub(crate) fn alignment_for_signature(signature: &str) -> Result<u32, VariantError> {
    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE => Ok(u8::ALIGNMENT),
        bool::SIGNATURE => Ok(bool::ALIGNMENT),
        i16::SIGNATURE => Ok(i16::ALIGNMENT),
        u16::SIGNATURE => Ok(u16::ALIGNMENT),
        i32::SIGNATURE => Ok(i32::ALIGNMENT),
        u32::SIGNATURE => Ok(u32::ALIGNMENT),
        i64::SIGNATURE => Ok(i64::ALIGNMENT),
        u64::SIGNATURE => Ok(u64::ALIGNMENT),
        f64::SIGNATURE => Ok(f64::ALIGNMENT),
        <(&str)>::SIGNATURE => Ok(<(&str)>::ALIGNMENT),
        // Doesn't matter what type for T we use here, alignment is the same
        Vec::<bool>::SIGNATURE => Ok(Vec::<bool>::ALIGNMENT),
        ObjectPath::SIGNATURE => Ok(ObjectPath::ALIGNMENT),
        Signature::SIGNATURE => Ok(Signature::ALIGNMENT),
        Structure::SIGNATURE => Ok(Structure::ALIGNMENT),
        Variant::SIGNATURE => Ok(Variant::ALIGNMENT),
        _ => return Err(VariantError::UnsupportedType(String::from(signature))),
    }
}

pub(crate) fn slice_signature(signature: &str) -> Result<&str, VariantError> {
    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE => u8::slice_signature(signature),
        bool::SIGNATURE => bool::slice_signature(signature),
        i16::SIGNATURE => i16::slice_signature(signature),
        u16::SIGNATURE => u16::slice_signature(signature),
        i32::SIGNATURE => i32::slice_signature(signature),
        u32::SIGNATURE => u32::slice_signature(signature),
        i64::SIGNATURE => i64::slice_signature(signature),
        u64::SIGNATURE => u64::slice_signature(signature),
        f64::SIGNATURE => f64::slice_signature(signature),
        <(&str)>::SIGNATURE => <(&str)>::slice_signature(signature),
        // Doesn't matter what type for T we use here, signature is the same but we're also assuming `slice_signature`
        // to be independent of `T` (an internal detail).
        Vec::<bool>::SIGNATURE => Vec::<bool>::slice_signature(signature),
        ObjectPath::SIGNATURE => ObjectPath::slice_signature(signature),
        Signature::SIGNATURE => Signature::slice_signature(signature),
        Structure::SIGNATURE => Structure::slice_signature(signature),
        Variant::SIGNATURE => Variant::slice_signature(signature),
        _ => return Err(VariantError::UnsupportedType(String::from(signature))),
    }
}
