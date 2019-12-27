use std::str;

use crate::{Decode, Encode, EncodingFormat};
use crate::{SharedData, Signature, SimpleDecode};
use crate::{Variant, VariantError};

// FIXME: Implement for owned string cause decode() needs that. Let's make it efficient later.
impl Encode for String {
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
        Variant::Str(self)
    }
}

impl Decode for String {
    fn slice_data(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;

        let len_slice = u32::slice_data_simple(&data, format)?;
        let last_index = u32::decode_simple(&len_slice, format)? as usize + len_slice.len() + 1;

        Ok(data.head(last_index))
    }

    fn decode(
        data: &SharedData,
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

    fn is(variant: &Variant) -> bool {
        if let Variant::Str(_) = variant {
            true
        } else {
            false
        }
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
