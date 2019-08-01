use crate::utils::padding_for_n_bytes;
use crate::{Variant, VariantError, VariantType};

pub struct Structure<'a> {
    fields: Vec<Variant<'a>>,
    signature: String,
}

impl<'a> Structure<'a> {
    pub fn new(fields: Vec<Variant<'a>>) -> Self {
        let mut signature = String::with_capacity(fields.len() + 2);
        signature.push('(');
        for field in &fields {
            signature.push_str(field.signature());
        }
        signature.push(')');

        Self { fields, signature }
    }

    pub fn as_slice<'b>(&'b self) -> &'b [Variant<'b>] {
        &self.fields
    }
}

impl<'a> VariantType<'a> for Structure<'a> {
    // The real single character signature for STRUCT is `r` but that's not actually used in practice for D-Bus at least
    // (the spec clearly states that this signature must never appear on the bus). The openning and closing braces are
    // used in practice and that's why we'll declare the opening brace as the signature for this type.
    const SIGNATURE: char = '(';
    const SIGNATURE_STR: &'static str = "(";
    const ALIGNMENT: u32 = 8;

    fn encode(&self) -> Vec<u8> {
        let mut v = vec![];
        for variant in &self.fields {
            let alignment = variant.inner_alignment();
            let padding = padding_for_n_bytes(v.len() as u32, alignment);
            v.extend(std::iter::repeat(0).take(padding as usize));

            // Deep copying, nice!!! 🙈
            v.extend(variant.get_bytes());
        }

        v
    }

    fn extract_slice(bytes: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
        if bytes.len() == 0 || signature.len() < 3 {
            return Err(VariantError::InsufficientData);
        }

        let mut extracted = 0;
        let mut i = 1;
        let mut open_brace_index = None;
        while i < signature.len() - 1 {
            match open_brace_index {
                Some(_) => {
                    if &signature[i..i + 1] != ")" {
                        i += 1;

                        continue;
                    }
                }
                None => {
                    if &signature[i..i + 1] == "(" {
                        open_brace_index = Some(i);
                        i += 1;

                        continue;
                    }
                }
            }
            let child_signature = &signature[open_brace_index.unwrap_or(i)..i + 1];
            open_brace_index = None;

            // Parse padding
            let alignment = crate::variant_type::get_alignment_for_signature(child_signature)?;
            extracted += padding_for_n_bytes(extracted as u32, alignment) as usize;
            if extracted > bytes.len() {
                return Err(VariantError::InsufficientData);
            }

            // Parse data
            let slice =
                crate::variant_type::extract_slice_from_data(&bytes[extracted..], child_signature)?;
            extracted += slice.len();
            if extracted > bytes.len() {
                return Err(VariantError::InsufficientData);
            }

            i += 1;
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
        }

        Ok(&bytes[0..extracted])
    }

    fn decode(bytes: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        // Similar to extract_slice, except we create variants.
        if bytes.len() == 0 || signature.len() < 3 {
            return Err(VariantError::InsufficientData);
        }

        // Assuming simple types here but it's OK to have more capacity than needed
        let mut variants = Vec::with_capacity(signature.len());
        let mut extracted = 0;
        let mut i = 1;
        let mut open_brace_index = None;
        while i < signature.len() - 1 {
            match open_brace_index {
                Some(_) => {
                    if &signature[i..i + 1] != ")" {
                        i += 1;

                        continue;
                    }
                }
                None => {
                    if &signature[i..i + 1] == "(" {
                        open_brace_index = Some(i);
                        i += 1;

                        continue;
                    }
                }
            }
            let child_signature = &signature[open_brace_index.unwrap_or(i)..i + 1];
            open_brace_index = None;

            // Parse padding
            let alignment = crate::variant_type::get_alignment_for_signature(child_signature)?;
            extracted += padding_for_n_bytes(extracted as u32, alignment) as usize;
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

            i += 1;
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
            let _ = crate::variant_type::get_alignment_for_signature(&signature[i..])?;

            i += 1;
        }

        Ok(())
    }

    fn signature<'b>(&'b self) -> &'b str {
        &self.signature
    }
}
