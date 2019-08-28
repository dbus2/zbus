use byteorder::ByteOrder;
use std::str;

use crate::utils::padding_for_n_bytes;
use crate::{SimpleVariantType, VariantError, VariantType};

impl<'a> VariantType<'a> for &'a str {
    const SIGNATURE: char = 's';
    const SIGNATURE_STR: &'static str = "s";
    const ALIGNMENT: u32 = 4;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let len = self.len();
        let padding = padding_for_n_bytes(n_bytes_before as u32, Self::ALIGNMENT);
        let mut bytes = Vec::with_capacity(padding as usize + 5 + len);

        bytes.extend(std::iter::repeat(0).take(padding as usize));

        bytes.extend(&(len as u32).to_ne_bytes());
        bytes.extend(self.as_bytes());
        bytes.push(b'\0');

        bytes
    }

    fn slice_data<'b>(
        bytes: &'b [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        let padding = Self::padding(n_bytes_before);
        let len = Self::ALIGNMENT as usize + padding;
        crate::ensure_sufficient_bytes(bytes, len)?;

        let last_index = len + byteorder::NativeEndian::read_u32(&bytes[padding..]) as usize + 1;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(bytes, signature, n_bytes_before)?;

        let last_index = slice.len() - 1;
        str::from_utf8(&slice[4..last_index]).map_err(|_| VariantError::InvalidUtf8)
    }
}
impl<'a> SimpleVariantType<'a> for &'a str {}

#[derive(Debug)]
pub struct ObjectPath<'a>(&'a str);

impl<'a> ObjectPath<'a> {
    pub fn new(path: &'a str) -> Self {
        Self(path)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

// FIXME: Find a way to share code with &str implementation above
impl<'a> VariantType<'a> for ObjectPath<'a> {
    const SIGNATURE: char = 'o';
    const SIGNATURE_STR: &'static str = "o";
    const ALIGNMENT: u32 = 4;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        self.0.encode(n_bytes_before)
    }

    fn slice_data<'b>(
        bytes: &'b [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        <(&str)>::slice_data_simple(bytes, n_bytes_before)
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;
        <(&str)>::decode(bytes, <(&str)>::SIGNATURE_STR, n_bytes_before).map(|s| Self(s))
    }
}
impl<'a> SimpleVariantType<'a> for ObjectPath<'a> {}

#[derive(Debug)]
pub struct Signature<'a>(&'a str);

impl<'a> Signature<'a> {
    pub fn new(signature: &'a str) -> Self {
        Self(signature)
    }

    pub fn as_str(&self) -> &str {
        self.0
    }
}

// FIXME: Find a way to share code with &str implementation in `variant_type.rs`
impl<'a> VariantType<'a> for Signature<'a> {
    const SIGNATURE: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
    const ALIGNMENT: u32 = 1;

    // No padding needed because of 1-byte alignment and hence n_bytes_before is ignored everywhere.

    fn encode(&self, _n_bytes_before: usize) -> Vec<u8> {
        let len = self.0.len();
        let mut bytes = Vec::with_capacity(2 + len);

        bytes.push(len as u8);
        bytes.extend(self.0.as_bytes());
        bytes.push(b'\0');

        bytes
    }

    fn slice_data<'b>(
        bytes: &'b [u8],
        signature: &str,
        _n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        if bytes.len() < 1 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = bytes[0] as usize + 2;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        _n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;
        if bytes.len() < 1 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = bytes.len() - 1;
        str::from_utf8(&bytes[1..last_index])
            .map(|s| Self(s))
            .map_err(|_| VariantError::InvalidUtf8)
    }
}
impl<'a> SimpleVariantType<'a> for Signature<'a> {}
