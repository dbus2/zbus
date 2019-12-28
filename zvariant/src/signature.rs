use std::str;

use crate::{Basic, Decode, Encode, EncodingFormat};
use crate::{SharedData, SimpleDecode};
use crate::{Variant, VariantError};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Signature(String);

impl Signature {
    pub fn new(signature: impl Into<Signature>) -> Self {
        signature.into()
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// FIXME: Find a way to share code with String implementation.
impl Encode for Signature {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
    const ALIGNMENT: usize = 1;

    // No padding needed because of 1-byte format and hence encoding format is ignored everywhere.

    fn encode_into(&self, bytes: &mut Vec<u8>, _format: EncodingFormat) {
        let len = self.0.len();

        bytes.push(len as u8);
        bytes.extend(self.0.as_bytes());
        bytes.push(b'\0');
    }

    fn to_variant(self) -> Variant {
        Variant::Signature(self)
    }
}

impl Decode for Signature {
    fn slice_data(
        data: &SharedData,
        signature: impl Into<Signature>,
        _format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;
        if data.len() < 1 {
            return Err(VariantError::InsufficientData);
        }

        let bytes = data.bytes();
        let last_index = bytes[0] as usize + 2;
        crate::ensure_sufficient_bytes(bytes, last_index)?;

        Ok(data.head(last_index))
    }

    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        _format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;

        let last_index = data.len() - 1;
        let bytes = data.bytes();
        crate::ensure_sufficient_bytes(bytes, last_index)?;

        str::from_utf8(&bytes[1..last_index])
            .map(Self::new)
            .map_err(|_| VariantError::InvalidUtf8)
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::Signature(_) = variant {
            true
        } else {
            false
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::Signature(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::Signature(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}
impl SimpleDecode for Signature {}
impl Basic for Signature {}

impl From<&str> for Signature {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for Signature {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for Signature {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl PartialEq<&str> for Signature {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
