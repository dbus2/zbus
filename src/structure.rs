use std::borrow::Cow;

use crate::utils::padding_for_n_bytes;
use crate::{Variant, VariantError, VariantType, VariantTypeConstants};

#[derive(Debug)]
pub struct Structure<'a> {
    encoding: Cow<'a, [u8]>,
    signature: String,
    fields: Vec<Variant>,
}

impl<'a> Structure<'a> {
    pub fn fields(&self) -> &[Variant] {
        &self.fields
    }
}

fn variants_from_struct_data(bytes: &[u8], signature: &str) -> Result<Vec<Variant>, VariantError> {
    // Assuming simple types here but it's OK to have more capacity than needed
    let mut fields = Vec::with_capacity(signature.len());
    let mut extracted = 0;
    let mut i = 1;
    let last_index = signature.len() - 1;
    while i < last_index {
        let child_signature = crate::slice_signature(&signature[i..last_index])?;

        // Parse child padding ourselves since we'll create Variant from it and Variant doesn't
        // handle padding.
        let alignment = crate::alignment_for_signature(child_signature)?;
        extracted += padding_for_n_bytes(extracted, alignment);
        if extracted > bytes.len() {
            return Err(VariantError::InsufficientData);
        }

        // Parse data
        let variant = Variant::from_data(&bytes[extracted..], child_signature)?;
        extracted += variant.len();
        if extracted > bytes.len() {
            return Err(VariantError::InsufficientData);
        }
        fields.push(variant);

        i += child_signature.len();
    }
    if extracted == 0 {
        return Err(VariantError::ExcessData);
    }

    Ok(fields)
}

#[derive(Debug)]
pub struct StructureBuilder {
    encoding: Vec<u8>,
    signature: String,
}

impl StructureBuilder {
    pub fn new() -> Self {
        Self {
            encoding: vec![],
            signature: String::from("()"),
        }
    }

    pub fn add_field<'b, T>(mut self, field: T) -> Self
    where
        T: VariantType<'b>,
    {
        let encoding = field.encode(self.encoding.len());
        self.encoding.extend(encoding);
        self.signature
            .insert_str(self.signature.len() - 1, &field.signature());

        self
    }

    pub fn create(self) -> Structure<'static> {
        let fields = variants_from_struct_data(&self.encoding, &self.signature)
            .unwrap_or_else(|e| {
                eprintln!("An error occured getting fields from a Structure (signature: '{}'). This should NOT happen \
                           since this structure is created manually. Here is the error: {}", self.signature, e);

                vec![]
            });
        Structure {
            encoding: Cow::from(self.encoding),
            signature: self.signature,
            fields: fields,
        }
    }
}

impl<'a> VariantTypeConstants for Structure<'a> {
    // The real single character signature for STRUCT is `r` but that's not actually used in practice for D-Bus at least
    // (the spec clearly states that this signature must never appear on the bus). The openning and closing braces are
    // used in practice and that's why we'll declare the opening brace as the signature for this type.
    const SIGNATURE_CHAR: char = '(';
    const SIGNATURE_STR: &'static str = "(";
    const ALIGNMENT: usize = 8;
}

impl<'a> VariantType<'a> for Structure<'a> {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode(&self, n_bytes_before: usize) -> Vec<u8> {
        let padding = Self::padding(n_bytes_before);
        if padding == 0 {
            return self.encoding.to_vec();
        }

        let mut v: Vec<u8> = std::iter::repeat(0).take(padding).collect();
        // Since a Structure always starts at 8-byte boundry, the fields and their children are
        // already aligned correctly.
        v.extend_from_slice(&self.encoding);

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

        let encoding = &bytes[padding..];
        let fields = variants_from_struct_data(encoding, signature)?;

        Ok(Self {
            encoding: Cow::from(encoding),
            signature: String::from(signature),
            fields: fields,
        })
    }

    fn ensure_correct_signature(signature: &str) -> Result<(), VariantError> {
        if !signature.starts_with("(") || !signature.ends_with(")") {
            return Err(VariantError::IncorrectType);
        }

        let mut i = 1;
        while i < signature.len() - 1 {
            // Ensure we've only valid child signatures
            let child_signature = crate::variant_type::slice_signature(&signature[i..])?;
            i += child_signature.len();
        }

        Ok(())
    }

    fn signature<'b>(&'b self) -> Cow<'b, str> {
        Cow::from(&self.signature)
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
