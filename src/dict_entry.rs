use crate::{Decode, Encode, EncodingFormat};
use crate::{SharedData, Signature, SimpleDecode};
use crate::{Variant, VariantError};

#[derive(Debug, Clone)]
pub struct DictEntry {
    key: Box<Variant>,
    value: Box<Variant>,
}

impl DictEntry {
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Encode + std::hash::Hash,
        V: Encode,
    {
        Self {
            key: Box::new(key.to_variant()),
            value: Box::new(value.to_variant()),
        }
    }

    // FIXME: Tryo to optimize (this should be returing a reference ideally
    pub fn key<K>(&self) -> Result<&K, VariantError>
    where
        K: SimpleDecode + std::hash::Hash,
    {
        K::from_variant(&self.key)
    }

    pub fn value<V>(&self) -> Result<&V, VariantError>
    where
        V: Decode,
    {
        V::from_variant(&self.value)
    }

    pub fn take_inner<K, V>(self) -> Result<(K, V), VariantError>
    where
        K: SimpleDecode + std::hash::Hash,
        V: Decode,
    {
        Ok((
            K::take_from_variant(*self.key)?,
            V::take_from_variant(*self.value)?,
        ))
    }
}

impl Encode for DictEntry {
    // The real single character signature for DICT_ENTRY is `e` but that's not actually used in practice for D-Bus at
    // least (the spec clearly states that this signature must never appear on the bus). The openning and closing curly
    // braces are used in practice and that's why we'll declare the opening curly brace as the signature for this type.
    const SIGNATURE_CHAR: char = '{';
    const SIGNATURE_STR: &'static str = "{";
    const ALIGNMENT: usize = 8;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);

        self.key.encode_value_into(bytes, format);
        self.value.encode_value_into(bytes, format);
    }

    fn signature(&self) -> Signature {
        Signature::from(format!(
            "{{{}{}}}",
            self.key.value_signature().as_str(),
            self.value.value_signature().as_str(),
        ))
    }

    fn to_variant(self) -> Variant {
        Variant::DictEntry(self)
    }
}

impl Decode for DictEntry {
    fn slice_data(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        let padding = Self::padding(data.position(), format);
        if data.len() < padding {
            return Err(VariantError::InsufficientData);
        }
        let signature = Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        // Key's signature will always be just 1 character so no need to slice for that.
        let key_signature = &signature[1..2];
        let key_slice =
            crate::decode::slice_data(&data.tail(extracted as usize), key_signature, format)?;
        extracted += key_slice.len();

        let value_signature = crate::decode::slice_signature(&signature[2..])?;
        let value_slice =
            crate::decode::slice_data(&data.tail(extracted as usize), value_signature, format)?;
        extracted += value_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        Ok(data.head(extracted))
    }

    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        // Similar to slice_data, except we create variants.
        let padding = Self::padding(data.position(), format);
        if data.len() < padding {
            return Err(VariantError::InsufficientData);
        }
        let signature = Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        // Key's signature will always be just 1 character so no need to slice for that.
        let key_signature = &signature[1..2];
        let key_slice =
            crate::decode::slice_data(&data.tail(extracted as usize), key_signature, format)?;
        let key = Variant::from_data(&key_slice, key_signature, format)?;
        extracted += key_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        let value_signature = crate::decode::slice_signature(&signature[2..])?;
        let value_slice = crate::decode::slice_data(
            &data.tail(extracted as usize),
            value_signature.as_str(),
            format,
        )?;
        let value = Variant::from_data(&value_slice, value_signature, format)?;
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
    fn ensure_correct_signature(
        signature: impl Into<Signature>,
    ) -> Result<Signature, VariantError> {
        let signature = signature.into();
        if !signature.starts_with("{") || !signature.ends_with("}") {
            return Err(VariantError::IncorrectType);
        }
        if signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        // Don't need the alignments but no errors here means we've valid signatures
        let _ = crate::alignment_for_signature(&signature[1..2])?;
        let value_signature = crate::decode::slice_signature(&signature[2..])?;
        let _ = crate::alignment_for_signature(value_signature)?;

        Ok(signature)
    }

    // Kept independent of K and V so that it can be used from generic code
    fn slice_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError> {
        let signature = signature.into();

        if !signature.starts_with("{") {
            return Err(VariantError::IncorrectType);
        }
        if signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        // Key's signature will always be just 1 character so no need to slice for that.
        // There should be one valid complete signature for value.
        let slice = crate::decode::slice_signature(&signature[2..])?;

        // signature of value + `{` + 1 char of the key signature + `}`
        Ok((&signature[0..slice.len() + 3]).into())
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
}
