use std::str;

use crate::{Basic, Decode, Encode, EncodingFormat};
use crate::{SharedData, Signature, SimpleDecode};
use crate::{Variant, VariantError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectPath(String);

impl ObjectPath {
    pub fn new(path: impl Into<ObjectPath>) -> Self {
        path.into()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Encode for ObjectPath {
    const SIGNATURE_CHAR: char = 'o';
    const SIGNATURE_STR: &'static str = "o";
    const ALIGNMENT: usize = 4;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        self.0.encode_into(bytes, format);
    }

    fn to_variant(self) -> Variant {
        Variant::ObjectPath(self)
    }
}

impl Decode for ObjectPath {
    fn slice_data<'b>(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;
        String::slice_data_simple(data, format)
    }

    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;
        String::decode(data, String::SIGNATURE_STR, format).map(Self)
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::ObjectPath(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::ObjectPath(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::ObjectPath(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}
impl SimpleDecode for ObjectPath {}
impl Basic for ObjectPath {}

impl From<&str> for ObjectPath {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for ObjectPath {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for ObjectPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl PartialEq<&str> for ObjectPath {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
