use byteorder::ByteOrder;
use std::str;

use crate::{EncodingContext, SharedData, SimpleVariantType};
use crate::{VariantError, VariantType, VariantTypeConstants};

impl VariantTypeConstants for String {
    const SIGNATURE_CHAR: char = 's';
    const SIGNATURE_STR: &'static str = "s";
    const ALIGNMENT: usize = 4;
}

// FIXME: Implement for owned string cause decode() needs that. Let's make it efficient later.
impl VariantType for String {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        let len = self.len();
        Self::add_padding(bytes, context);

        bytes.extend(&crate::utils::usize_to_u32(len).to_ne_bytes());
        bytes.extend(self.as_bytes());
        bytes.push(b'\0');
    }

    fn slice_data(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;
        let padding = Self::padding(data.position(), context);
        let len = Self::ALIGNMENT as usize + padding;
        data.apply(|bytes| crate::ensure_sufficient_bytes(bytes, len))?;

        let last_index = data.apply(|bytes| {
            let last_index =
                len + byteorder::NativeEndian::read_u32(&bytes[padding..]) as usize + 1;
            crate::ensure_sufficient_bytes(bytes, last_index)?;

            Ok(last_index)
        })?;

        Ok(data.head(last_index))
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        let last_index = slice.len() - 1;
        slice.apply(|bytes| {
            str::from_utf8(&bytes[4..last_index])
                .map(|s| s.to_owned())
                .map_err(|_| VariantError::InvalidUtf8)
        })
    }
}
impl SimpleVariantType for String {}

#[derive(Debug)]
pub struct ObjectPath(String);

impl ObjectPath {
    pub fn new(path: &str) -> Self {
        Self(String::from(path))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl VariantTypeConstants for ObjectPath {
    const SIGNATURE_CHAR: char = 'o';
    const SIGNATURE_STR: &'static str = "o";
    const ALIGNMENT: usize = 4;
}

// FIXME: Find a way to share code with &str implementation above
impl VariantType for ObjectPath {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        self.0.encode_into(bytes, context);
    }

    fn slice_data<'b>(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;
        String::slice_data_simple(data, context)
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;
        String::decode(data, String::SIGNATURE_STR, context).map(|s| Self(s))
    }
}
impl SimpleVariantType for ObjectPath {}

#[derive(Debug)]
pub struct Signature(String);

impl Signature {
    pub fn new(signature: &str) -> Self {
        Self(String::from(signature))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl VariantTypeConstants for Signature {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
    const ALIGNMENT: usize = 1;
}

// FIXME: Find a way to share code with &str implementation in `variant_type.rs`
impl VariantType for Signature {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    // No padding needed because of 1-byte context and hence encoding context is ignored everywhere.

    fn encode_into(&self, bytes: &mut Vec<u8>, _context: EncodingContext) {
        let len = self.0.len();

        bytes.push(len as u8);
        bytes.extend(self.0.as_bytes());
        bytes.push(b'\0');
    }

    fn slice_data(
        data: &SharedData,
        signature: &str,
        _context: EncodingContext,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;
        if data.len() < 1 {
            return Err(VariantError::InsufficientData);
        }

        let last_index = data.apply(|bytes| {
            let last_index = bytes[0] as usize + 2;
            crate::ensure_sufficient_bytes(bytes, last_index)?;

            Ok(last_index)
        })?;

        Ok(data.head(last_index))
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        _context: EncodingContext,
    ) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;

        let last_index = data.len() - 1;
        data.apply(|bytes| {
            crate::ensure_sufficient_bytes(bytes, last_index)?;

            str::from_utf8(&bytes[1..last_index])
                .map(|s| Self::new(s))
                .map_err(|_| VariantError::InvalidUtf8)
        })
    }
}
impl SimpleVariantType for Signature {}
