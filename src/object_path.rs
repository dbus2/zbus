use std::str;

use crate::{SimpleVariantType, VariantError, VariantType};

pub struct ObjectPath<'a>(&'a str);

impl<'a> ObjectPath<'a> {
    pub fn new(path: &'a str) -> Self {
        Self(path)
    }

    pub fn as_str(&'a self) -> &str {
        self.0
    }
}

// FIXME: Find a way to share code with &str implementation in `variant_type.rs`
impl<'a> VariantType<'a> for ObjectPath<'a> {
    const SIGNATURE: char = 'o';
    const SIGNATURE_STR: &'static str = "o";

    fn encode(&self) -> Vec<u8> {
        self.0.encode()
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;
        <(&str)>::extract_slice_simple(bytes)
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        <(&str)>::decode(bytes).map(|s| Self(s))
    }
}
impl<'a> SimpleVariantType<'a> for ObjectPath<'a> {}
