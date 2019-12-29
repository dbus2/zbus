use std::str;

use crate::{Basic, Decode, Encode, EncodingFormat};
use crate::{SharedData, Signature, SimpleDecode};
use crate::{Variant, VariantError};

// We can only implement Encode for unowned type as Decode::decode() implementation will need
// require lifetimes and we really want to avoid that now.
impl Encode for &str {
    const SIGNATURE_CHAR: char = 's';
    const SIGNATURE_STR: &'static str = "s";
    const ALIGNMENT: usize = 4;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        let len = self.len();
        Self::add_padding(bytes, format);

        bytes.extend(&crate::utils::usize_to_u32(len).to_ne_bytes());
        bytes.extend(self.as_bytes());
        bytes.push(b'\0');
    }

    fn to_variant(self) -> Variant {
        String::from(self).to_variant()
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::Str(_) = variant {
            true
        } else {
            false
        }
    }
}
impl Basic for &str {}

impl Encode for String {
    const SIGNATURE_CHAR: char = <&str>::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = <&str>::SIGNATURE_STR;
    const ALIGNMENT: usize = <&str>::ALIGNMENT;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        self.as_str().encode_into(bytes, format)
    }

    fn to_variant(self) -> Variant {
        Variant::Str(self)
    }

    fn is(variant: &Variant) -> bool {
        <&str>::is(variant)
    }
}

impl Decode for String {
    fn slice_data(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;

        let data = data.into();
        let len_slice = u32::slice_data_simple(&data, format)?;
        let last_index = u32::decode_simple(&len_slice, format)? as usize + len_slice.len() + 1;

        Ok(data.head(last_index))
    }

    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;
        let last_index = slice.len() - 1;
        let bytes = slice.bytes();

        str::from_utf8(&bytes[4..last_index])
            .map(|s| s.to_owned())
            .map_err(|_| VariantError::InvalidUtf8)
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::Str(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::Str(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}
impl SimpleDecode for String {}
impl Basic for String {}
