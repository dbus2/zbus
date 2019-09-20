use std::borrow::Cow;

use crate::{SimpleVariantType, VariantError, VariantType};

#[derive(Debug)]
pub struct DictEntry<K, V> {
    key: K,
    value: V,
}

impl<'a, K: SimpleVariantType<'a> + std::hash::Hash, V: VariantType<'a>> DictEntry<K, V> {
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

impl<'a, K: SimpleVariantType<'a> + std::hash::Hash, V: VariantType<'a>> VariantType<'a>
    for DictEntry<K, V>
{
    // The real single character signature for DICT_ENTRY is `e` but that's not actually used in practice for D-Bus at
    // least (the spec clearly states that this signature must never appear on the bus). The openning and closing curly
    // braces are used in practice and that's why we'll declare the opening curly brace as the signature for this type.
    const SIGNATURE: char = '{';
    const SIGNATURE_STR: &'static str = "{";
    const ALIGNMENT: usize = 8;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);

        v.extend(self.key.encode(v.len() + n_bytes_before));
        v.extend(self.value.encode(v.len() + n_bytes_before));

        v
    }

    // Kept independent of K and V so that it can be used from generic code
    fn slice_data<'b>(
        bytes: &'b [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        let padding = Self::padding(n_bytes_before);
        if bytes.len() < padding || signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        // Key's signature will always be just 1 character so no need to slice for that.
        let key_signature = &signature[1..2];
        let key_slice = crate::variant_type::slice_data(
            &bytes[(extracted as usize)..],
            key_signature,
            n_bytes_before + extracted,
        )?;
        extracted += key_slice.len();

        let value_signature = crate::variant_type::slice_signature(&signature[2..])?;
        let value_slice = crate::variant_type::slice_data(
            &bytes[(extracted as usize)..],
            value_signature,
            n_bytes_before + extracted,
        )?;
        extracted += value_slice.len();
        if extracted > bytes.len() {
            return Err(VariantError::InsufficientData);
        }

        Ok(&bytes[0..extracted])
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        // Similar to slice_data, except we create variants.
        let padding = Self::padding(n_bytes_before);
        if bytes.len() < padding || signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        let slice = K::slice_data_simple(&bytes[extracted..], n_bytes_before + extracted)?;
        let key = K::decode_simple(slice, n_bytes_before + extracted)?;
        extracted += slice.len();
        if extracted > bytes.len() {
            return Err(VariantError::InsufficientData);
        }

        let value_signature = V::slice_signature(&signature[2..])?;
        let slice = V::slice_data(
            &bytes[extracted..],
            value_signature,
            n_bytes_before + extracted,
        )?;
        let value = V::decode(slice, value_signature, n_bytes_before + extracted)?;
        extracted += slice.len();
        if extracted > bytes.len() {
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
        let _ = crate::variant_type::alignment_for_signature(&signature[1..2])?;
        let value_signature = crate::variant_type::slice_signature(&signature[2..])?;
        let _ = crate::variant_type::alignment_for_signature(value_signature)?;

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
