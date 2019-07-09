use std::str;

use crate::{VariantError, VariantType};

pub struct Signature<'a>(&'a str);

impl<'a> Signature<'a> {
    pub fn new(signature: &'a str) -> Self {
        Self(signature)
    }

    pub fn as_str(&'a self) -> &str {
        self.0
    }
}

// FIXME: Find a way to share code with &str implementation in `variant_type.rs`
impl<'a> VariantType<'a> for Signature<'a> {
    const SIGNATURE: char = 'g';
    const SIGNATURE_STR: &'static str = "g";

    fn encode(&self) -> Vec<u8> {
        let len = self.0.len();
        let mut bytes = Vec::with_capacity(2 + len);

        bytes.push(len as u8);
        bytes.extend(self.0.as_bytes());
        bytes.push(b'\0');

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        if bytes.len() < 1 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = bytes[0] as usize + 2;
        if bytes.len() < last_index {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..last_index])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        if bytes.len() < 1 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = bytes.len() - 1;
        str::from_utf8(&bytes[1..last_index])
            .map(|s| Self(s))
            .map_err(|_| VariantError::InvalidUtf8)
    }
}
