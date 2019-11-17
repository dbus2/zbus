use std::borrow::Cow;

use crate::utils::padding_for_n_bytes;
use crate::SharedData;
use crate::{Variant, VariantError, VariantType, VariantTypeConstants};

#[derive(Debug)]
pub struct Structure {
    encoding: SharedData,
    signature: String,
    fields: Vec<Variant>,
}

impl Structure {
    pub fn fields(&self) -> &[Variant] {
        &self.fields
    }
}

fn variants_from_struct_data(
    data: &SharedData,
    signature: &str,
) -> Result<Vec<Variant>, VariantError> {
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
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        // Parse data
        let variant = Variant::from_data(&data.tail(extracted), child_signature)?;
        extracted += variant.len();
        if extracted > data.len() {
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

    pub fn add_field<T>(mut self, field: T) -> Self
    where
        T: VariantType,
    {
        field.encode_into(&mut self.encoding);
        self.signature
            .insert_str(self.signature.len() - 1, &field.signature());

        self
    }

    pub fn create(self) -> Structure {
        let (encoding, signature) = (self.encoding, self.signature);
        let encoding = SharedData::new(encoding);
        let fields = variants_from_struct_data(&encoding, &signature)
            .unwrap_or_else(|e| {
                eprintln!("An error occured getting fields from a Structure (signature: '{}'). This should NOT happen \
                           since this structure is created manually. Here is the error: {}", signature, e);

                vec![]
            });
        Structure {
            encoding: encoding,
            signature: signature,
            fields: fields,
        }
    }
}

impl VariantTypeConstants for Structure {
    // The real single character signature for STRUCT is `r` but that's not actually used in practice for D-Bus at least
    // (the spec clearly states that this signature must never appear on the bus). The openning and closing braces are
    // used in practice and that's why we'll declare the opening brace as the signature for this type.
    const SIGNATURE_CHAR: char = '(';
    const SIGNATURE_STR: &'static str = "(";
    const ALIGNMENT: usize = 8;
}

impl VariantType for Structure {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>) {
        Self::add_padding(bytes);

        // Since a Structure always starts at 8-byte boundry, the fields and their children are
        // already aligned correctly.
        self.encoding.apply_mut(|b| bytes.extend_from_slice(b));
    }

    fn slice_data(data: &SharedData, signature: &str) -> Result<SharedData, VariantError> {
        let padding = Self::padding(data.position());
        if data.len() < padding || signature.len() < 3 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        let mut i = 1;
        let last_index = signature.len() - 1;
        while i < last_index {
            let child_signature = crate::variant_type::slice_signature(&signature[i..last_index])?;
            let slice =
                crate::variant_type::slice_data(&data.tail(extracted as usize), child_signature)?;
            extracted += slice.len();
            if extracted > data.len() {
                return Err(VariantError::InsufficientData);
            }

            i += child_signature.len();
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
        }

        Ok(data.head(extracted))
    }

    fn decode(data: &SharedData, signature: &str) -> Result<Self, VariantError> {
        // Similar to slice_data, except we create variants.
        let padding = Self::padding(data.position());
        if data.len() < padding || signature.len() < 3 {
            return Err(VariantError::InsufficientData);
        }
        Self::ensure_correct_signature(signature)?;

        let encoding = data.tail(padding);
        let fields = variants_from_struct_data(&encoding, signature)?;

        Ok(Self {
            encoding: encoding,
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
