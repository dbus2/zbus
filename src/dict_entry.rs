use std::borrow::Cow;

use crate::{SharedData, SimpleVariantType};
use crate::{VariantError, VariantType, VariantTypeConstants};

#[derive(Debug)]
pub struct DictEntry<K, V> {
    key: K,
    value: V,
}

impl<K: SimpleVariantType + std::hash::Hash, V: VariantType> DictEntry<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn value(&self) -> &V {
        &self.value
    }

    pub fn take_inner(self) -> (K, V) {
        (self.key, self.value)
    }
}

impl<K: SimpleVariantType + std::hash::Hash, V: VariantType> VariantTypeConstants
    for DictEntry<K, V>
{
    // The real single character signature for DICT_ENTRY is `e` but that's not actually used in practice for D-Bus at
    // least (the spec clearly states that this signature must never appear on the bus). The openning and closing curly
    // braces are used in practice and that's why we'll declare the opening curly brace as the signature for this type.
    const SIGNATURE_CHAR: char = '{';
    const SIGNATURE_STR: &'static str = "{";
    const ALIGNMENT: usize = 8;
}

impl<K: SimpleVariantType + std::hash::Hash, V: VariantType> VariantType for DictEntry<K, V> {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>) {
        Self::add_padding(bytes);

        self.key.encode_into(bytes);
        self.value.encode_into(bytes);
    }

    // Kept independent of K and V so that it can be used from generic code
    fn slice_data(data: &SharedData, signature: &str) -> Result<SharedData, VariantError> {
        let padding = Self::padding(data.position());
        if data.len() < padding || signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        // Key's signature will always be just 1 character so no need to slice for that.
        let key_signature = &signature[1..2];
        let key_slice =
            crate::variant_type::slice_data(&data.tail(extracted as usize), key_signature)?;
        extracted += key_slice.len();

        let value_signature = crate::variant_type::slice_signature(&signature[2..])?;
        let value_slice =
            crate::variant_type::slice_data(&data.tail(extracted as usize), value_signature)?;
        extracted += value_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        Ok(data.head(extracted))
    }

    fn decode(data: &SharedData, signature: &str) -> Result<Self, VariantError> {
        // Similar to slice_data, except we create variants.
        let padding = Self::padding(data.position());
        if data.len() < padding || signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        let slice = K::slice_data_simple(&data.tail(extracted))?;
        let key = K::decode_simple(&slice)?;
        extracted += slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        let value_signature = V::slice_signature(&signature[2..])?;
        let slice = V::slice_data(&data.tail(extracted as usize), value_signature)?;
        let value = V::decode(&slice, value_signature)?;
        extracted += slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        Ok(Self::new(key, value))
    }

    // Kept independent of K and V so that it can be used from generic code
    fn ensure_correct_signature(signature: &str) -> Result<(), VariantError> {
        if !signature.starts_with("{") || !signature.ends_with("}") {
            return Err(VariantError::IncorrectType);
        }
        if signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        // Don't need the alignments but no errors here means we've valid signatures
        let _ = crate::alignment_for_signature(&signature[1..2])?;
        let value_signature = crate::variant_type::slice_signature(&signature[2..])?;
        let _ = crate::alignment_for_signature(value_signature)?;

        Ok(())
    }

    fn signature<'b>(&'b self) -> Cow<'b, str> {
        Cow::from(format!(
            "{{{}{}}}",
            self.key.signature(),
            self.value.signature()
        ))
    }

    // Kept independent of K and V so that it can be used from generic code
    fn slice_signature(signature: &str) -> Result<&str, VariantError> {
        if !signature.starts_with("{") {
            return Err(VariantError::IncorrectType);
        }
        if signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        // Key's signature will always be just 1 character so no need to slice for that.
        // There should be one valid complete signature for value.
        let slice = crate::variant_type::slice_signature(&signature[2..])?;

        // signature of value + `{` + 1 char of the key signature + `}`
        Ok(&signature[0..slice.len() + 3])
    }
}
