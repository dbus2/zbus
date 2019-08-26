use byteorder::ByteOrder;
use std::borrow::Cow;

use crate::utils::padding_for_n_bytes;
use crate::SimpleVariantType;
use crate::{VariantError, VariantType};

impl<'a, T: VariantType<'a>> VariantType<'a> for Vec<T> {
    const SIGNATURE: char = 'a';
    const SIGNATURE_STR: &'static str = "a";
    const ALIGNMENT: u32 = 4;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);

        v.extend(&0u32.to_ne_bytes());
        for element in self {
            // Deep copying, nice!!! 🙈
            v.extend(element.encode(v.len() + n_bytes_before));
        }

        // Set size of array in bytes
        let len = (v.len() - 4) as u32;
        byteorder::NativeEndian::write_u32(&mut v[0..4], len);

        v
    }

    fn extract_slice<'b>(bytes: &'b [u8], signature: &str) -> Result<&'b [u8], VariantError> {
        if bytes.len() < 4 || signature.len() < 2 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        // Child signature & alignement
        let child_signature = crate::variant_type::slice_signature(&signature[1..])?;
        let alignment = crate::variant_type::alignment_for_signature(child_signature)?;

        // Array size in bytes
        let len = u32::decode_simple(&bytes[0..4])? + 4;
        let mut extracted = 4;
        while extracted < len {
            // Parse padding
            extracted += padding_for_n_bytes(extracted as u32, alignment);
            if extracted > len {
                return Err(VariantError::InsufficientData);
            }

            // Parse data
            let slice = crate::variant_type::extract_slice_from_data(
                &bytes[(extracted as usize)..],
                child_signature,
            )?;
            extracted += slice.len() as u32;
            if extracted > len {
                return Err(VariantError::InsufficientData);
            }
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
        }

        Ok(&bytes[0..(extracted as usize)])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError> {
        if bytes.len() < 4 || signature.len() < 2 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        // Child signature & alignement
        let child_signature = crate::variant_type::slice_signature(&signature[1..])?;
        let alignment = crate::variant_type::alignment_for_signature(child_signature)?;

        // Array size in bytes
        let len = u32::decode_simple(&bytes[0..4])? + 4;
        let mut extracted = 4;
        let mut elements = vec![];

        while extracted < len {
            // Parse padding
            extracted += padding_for_n_bytes(extracted as u32, alignment);
            if extracted > len {
                return Err(VariantError::InsufficientData);
            }

            // Parse data
            let slice = crate::variant_type::extract_slice_from_data(
                &bytes[(extracted as usize)..],
                child_signature,
            )?;
            extracted += slice.len() as u32;
            if extracted > len {
                return Err(VariantError::InsufficientData);
            }

            let element = T::decode(slice, child_signature)?;
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
