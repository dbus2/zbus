use std::borrow::Cow;

use crate::utils::padding_for_n_bytes;
use crate::{Variant, VariantError, VariantType};

#[derive(Debug)]
pub struct Structure<'a>(Vec<Variant<'a>>);

impl<'a> Structure<'a> {
    pub fn new(fields: Vec<Variant<'a>>) -> Self {
        Self(fields)
    }

    // FIXME: Can't we just use 'a here?
    pub fn fields<'b>(&'b self) -> &'b [Variant<'b>] {
        &self.0
    }

    pub fn take_fields(self) -> Vec<Variant<'a>> {
        self.0
    }
}

impl<'a> VariantType<'a> for Structure<'a> {
    // The real single character signature for STRUCT is `r` but that's not actually used in practice for D-Bus at least
    // (the spec clearly states that this signature must never appear on the bus). The openning and closing braces are
    // used in practice and that's why we'll declare the opening brace as the signature for this type.
    const SIGNATURE: char = '(';
    const SIGNATURE_STR: &'static str = "(";
    const ALIGNMENT: u32 = 8;

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let mut v = Self::create_bytes_vec(n_bytes_before);

        for variant in &self.0 {
            // Variant doesn't have the padding so we need to add it
            let alignment = variant.inner_alignment();
            let padding = padding_for_n_bytes((v.len() + n_bytes_before) as u32, alignment);
            v.extend(std::iter::repeat(0).take(padding as usize));

            // Deep copying, nice!!! 🙈
            v.extend(variant.bytes());
        }

        v
    }

    fn slice_data<'b>(
        bytes: &'b [u8],
        signature: &str,
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        let padding = Self::padding(n_bytes_before);
        if bytes.len() < padding || signature.len() < 3 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        let mut i = 1;
        let last_index = signature.len() - 1;
        while i < last_index {
            let child_signature = crate::variant_type::slice_signature(&signature[i..last_index])?;
            let slice = crate::variant_type::slice_data(
                &bytes[(extracted as usize)..],
                child_signature,
                n_bytes_before + extracted,
            )?;
            extracted += slice.len();
            if extracted > bytes.len() {
                return Err(VariantError::InsufficientData);
            }

            i += child_signature.len();
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
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
        if bytes.len() < padding || signature.len() < 3 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        // Assuming simple types here but it's OK to have more capacity than needed
        let mut variants = Vec::with_capacity(signature.len());
        let mut extracted = padding;
        let mut i = 1;
        let last_index = signature.len() - 1;
        while i < last_index {
            let child_signature = crate::variant_type::slice_signature(&signature[i..last_index])?;

            // Parse child padding ourselves since we'll create Varint from it and Variant doesn't
            // handle padding.
            let alignment = crate::variant_type::alignment_for_signature(child_signature)?;
            extracted +=
                padding_for_n_bytes((n_bytes_before + extracted) as u32, alignment) as usize;
            if extracted > bytes.len() {
                return Err(VariantError::InsufficientData);
            }

            // Parse data
            let variant = Variant::from_data(&bytes[extracted..], child_signature)?;
            extracted += variant.len();
            if extracted > bytes.len() {
                return Err(VariantError::InsufficientData);
            }
            variants.push(variant);

            i += child_signature.len();
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
        }

        Ok(Self::new(variants))
    }

    fn ensure_correct_signature(signature: &str) -> Result<(), VariantError> {
        if !signature.starts_with("(") || !signature.ends_with(")") {
            return Err(VariantError::IncorrectType);
        }

        let mut i = 1;
        while i < signature.len() - 1 {
            if &signature[i..i + 1] == ")" {
                i += 1;

                continue;
            }
            // We don't need the alignment but not getting an error here means it's a supported type
            let _ = crate::variant_type::alignment_for_signature(&signature[i..])?;

            i += 1;
        }

        Ok(())
    }

    fn signature<'b>(&'b self) -> Cow<'b, str> {
        let mut signature = String::with_capacity(self.0.len() + 2);
        signature.push('(');
        for field in &self.0 {
            signature.push_str(field.signature());
        }
        signature.push(')');

        Cow::from(signature)
    }

    fn slice_signature(signature: &str) -> Result<&str, VariantError> {
        if !signature.starts_with("(") {
            return Err(VariantError::IncorrectType);
        }

        let mut open_braces = 1;
        let mut i = 1;
        while i < signature.len() {
            if &signature[i..i + 1] == ")" {
                open_braces -= 1;

                if open_braces == 0 {
                    break;
                }
            } else if &signature[i..i + 1] == "(" {
                open_braces += 1;
            }

            i += 1;
        }
        if &signature[i..i + 1] != ")" {
            return Err(VariantError::IncorrectType);
        }

        Ok(&signature[0..i + 1])
    }
}
