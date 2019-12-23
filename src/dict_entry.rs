use std::borrow::Cow;

use crate::EncodingContext;
use crate::Variant;
use crate::{SharedData, SimpleVariantType};
use crate::{VariantError, VariantType, VariantTypeConstants};

#[derive(Debug, Clone)]
pub struct DictEntry {
    key: Box<Variant>,
    value: Box<Variant>,
}

impl DictEntry {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: SimpleVariantType + std::hash::Hash,
        V: VariantType,
    {
        Self {
            key: Box::new(key.to_variant()),
            value: Box::new(value.to_variant()),
        }
    }

    // FIXME: Tryo to optimize (this should be returing a reference ideally
    pub fn key<K>(&self) -> Result<&K, VariantError>
    where
        K: SimpleVariantType + std::hash::Hash,
    {
        K::from_variant(&self.key)
    }

    pub fn value<V>(&self) -> Result<&V, VariantError>
    where
        V: VariantType,
    {
        V::from_variant(&self.value)
    }

    pub fn take_inner<K, V>(self) -> Result<(K, V), VariantError>
    where
        K: SimpleVariantType + std::hash::Hash,
        V: VariantType,
    {
        Ok((
            K::take_from_variant(*self.key)?,
            V::take_from_variant(*self.value)?,
        ))
    }
}

impl VariantTypeConstants for DictEntry {
    // The real single character signature for DICT_ENTRY is `e` but that's not actually used in practice for D-Bus at
    // least (the spec clearly states that this signature must never appear on the bus). The openning and closing curly
    // braces are used in practice and that's why we'll declare the opening curly brace as the signature for this type.
    const SIGNATURE_CHAR: char = '{';
    const SIGNATURE_STR: &'static str = "{";
    const ALIGNMENT: usize = 8;
}

impl VariantType for DictEntry {
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
        Self::add_padding(bytes, context);

        let child_enc_context = context.copy_for_child();
        self.key.encode_value_into(bytes, child_enc_context);
        self.value.encode_value_into(bytes, child_enc_context);
    }

    fn slice_data(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<SharedData, VariantError> {
        let padding = Self::padding(data.position(), context);
        if data.len() < padding || signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        let child_enc_context = context.copy_for_child();
        // Key's signature will always be just 1 character so no need to slice for that.
        let key_signature = &signature[1..2];
        let key_slice = crate::variant_type::slice_data(
            &data.tail(extracted as usize),
            key_signature,
            child_enc_context,
        )?;
        extracted += key_slice.len();

        let value_signature = crate::variant_type::slice_signature(&signature[2..])?;
        let value_slice = crate::variant_type::slice_data(
            &data.tail(extracted as usize),
            value_signature,
            child_enc_context,
        )?;
        extracted += value_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        Ok(data.head(extracted))
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        // Similar to slice_data, except we create variants.
        let padding = Self::padding(data.position(), context);
        if data.len() < padding || signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        let child_enc_context = context.copy_for_child();
        // Key's signature will always be just 1 character so no need to slice for that.
        let key_signature = &signature[1..2];
        let key_slice = crate::variant_type::slice_data(
            &data.tail(extracted as usize),
            key_signature,
            child_enc_context,
        )?;
        let key = Variant::from_data(&key_slice, key_signature, child_enc_context)?;
        extracted += key_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        let value_signature = crate::variant_type::slice_signature(&signature[2..])?;
        let value_slice = crate::variant_type::slice_data(
            &data.tail(extracted as usize),
            value_signature,
            child_enc_context,
        )?;
        let value = Variant::from_data(&value_slice, value_signature, child_enc_context)?;
        extracted += value_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        Ok(Self {
            key: Box::new(key),
            value: Box::new(value),
        })
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
            self.key.value_signature(),
            self.value.value_signature()
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

    fn is(variant: &Variant) -> bool {
        if let Variant::DictEntry(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::DictEntry(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::DictEntry(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn to_variant(self) -> Variant {
        Variant::DictEntry(self)
    }
}
