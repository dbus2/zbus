use byteorder::ByteOrder;
use std::borrow::Cow;

use crate::{SharedData, SimpleVariantType};
use crate::{VariantError, VariantType, VariantTypeConstants};

impl<T: VariantType> VariantTypeConstants for Vec<T> {
    const SIGNATURE_CHAR: char = 'a';
    const SIGNATURE_STR: &'static str = "a";
    const ALIGNMENT: usize = 4;
}

impl<T: VariantType> VariantType for Vec<T> {
    fn signature_char() -> char {
        'a'
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>) {
        Self::add_padding(bytes);

        let len_position = bytes.len();
        bytes.extend(&0u32.to_ne_bytes());
        let n_bytes_before = bytes.len();
        for element in self {
            // Deep copying, nice!!! 🙈
            element.encode_into(bytes);
        }

        // Set size of array in bytes
        let len = crate::utils::usize_to_u32(bytes.len() - n_bytes_before);
        byteorder::NativeEndian::write_u32(&mut bytes[len_position..len_position + 4], len);
    }

    fn slice_data(data: &SharedData, signature: &str) -> Result<SharedData, VariantError> {
        if signature.len() < 2 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        // Child signature
        let child_signature = crate::variant_type::slice_signature(&signature[1..])?;

        // Array size in bytes
        let len_slice = u32::slice_data_simple(&data)?;
        let mut extracted = len_slice.len();
        let len = u32::decode_simple(&len_slice)? as usize + 4;
        while extracted < len {
            let slice = crate::variant_type::slice_data(&data.tail(extracted), child_signature)?;
            extracted += slice.len();
            if extracted > len {
                return Err(VariantError::InsufficientData);
            }
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
        }

        Ok(data.head(extracted as usize))
    }

    fn decode(data: &SharedData, signature: &str) -> Result<Self, VariantError> {
        let padding = Self::padding(data.position());
        if data.len() < padding + 4 || signature.len() < 2 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        // Child signature
        let child_signature = crate::variant_type::slice_signature(&signature[1..])?;

        // Array size in bytes
        let mut extracted = padding + 4;
        let len = u32::decode_simple(&data.subset(padding, extracted))? as usize + 4;
        let mut elements = vec![];

        while extracted < len {
            let slice =
                crate::variant_type::slice_data(&data.tail(extracted as usize), child_signature)?;
            extracted += slice.len();
            if extracted > len {
                return Err(VariantError::InsufficientData);
            }

            let element = T::decode(&slice, child_signature)?;
            elements.push(element);
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
        }

        Ok(elements)
    }

    fn ensure_correct_signature(signature: &str) -> Result<(), VariantError> {
        let slice = Self::slice_signature(&signature)?;
        if slice.len() != signature.len() {
            return Err(VariantError::IncorrectType);
        }

        Ok(())
    }

    fn signature<'b>(&'b self) -> Cow<'b, str> {
        let signature = format!("a{}", self[0].signature());

        Cow::from(signature)
    }

    fn slice_signature(signature: &str) -> Result<&str, VariantError> {
        if !signature.starts_with("a") {
            return Err(VariantError::IncorrectType);
        }

        // There should be a valid complete signature after 'a' but not more than 1
        let slice = crate::variant_type::slice_signature(&signature[1..])?;

        Ok(&signature[0..slice.len() + 1])
    }
}
