use crate::{Decode, Encode, EncodingFormat};
use crate::{SharedData, Signature};
use crate::{Variant, VariantError};

/// An ordered collection of items of arbitrary types.
///
/// This is mostly just a way to support custom data structures. In the future, we'll hopefully
/// provide a way to easily map a Rust struct into a `Structure` (probably through an [attribute
/// macro]).
///
/// # Example
///
/// ```
/// use core::convert::TryInto;
///
/// use zvariant::{Array, Decode, Encode, EncodingFormat};
/// use zvariant::Structure;
///
/// // Create a complex structure
/// let top_struct = Structure::new()
///     // top-most simple fields
///     .add_field(u8::max_value())
///     .add_field(u32::max_value())
///     // top-most inner structure
///     .add_field(
///         Structure::new()
///             // 2nd level simple fields
///             .add_field(i64::max_value())
///             .add_field(true)
///             .add_field(i64::max_value())
///             // 2nd level array field
///             .add_field(Array::from(vec!["Hello", "World"])),
///     )
///     // one more top-most simple field
///     .add_field("hello");
/// assert!(top_struct.signature() == "(yu(xbxas)s)");
///
/// // Encode it
/// let format = EncodingFormat::default();
/// let encoding = top_struct.encode(format);
/// assert!(encoding.len() == 70);
///
/// // Then decode it
/// let top_struct = Structure::decode(encoding, top_struct.signature(), format).unwrap();
///
/// // Now check if everything is as expected
/// let fields = top_struct.fields();
///
/// // top-most simple fields
/// assert!(*u8::from_variant(&fields[0]).unwrap() == u8::max_value());
/// assert!(*u32::from_variant(&fields[1]).unwrap() == u32::max_value());
/// assert!(String::from_variant(&fields[3]).unwrap() == "hello");
///
/// // top-most inner structure
/// let inner = Structure::from_variant(&fields[2]).unwrap();
/// let inner_fields = inner.fields();
///
/// // 2nd level simple fields
/// assert!(*i64::from_variant(&inner_fields[0]).unwrap() == i64::max_value());
/// assert!(*bool::from_variant(&inner_fields[1]).unwrap() == true);
///
/// // 2nd level array field
/// let array = Array::from_variant(&inner_fields[3]).unwrap();
/// let as_: Vec<&String> = array.try_into().unwrap();
/// assert!(as_ == ["Hello", "World"]);
/// ```
///
/// [attribute macro]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[derive(Debug, Clone, Default)]
pub struct Structure(Vec<Variant>);

impl Structure {
    /// Get all the fields, consuming `self`.
    pub fn take_fields(self) -> Vec<Variant> {
        self.0
    }

    /// Get a reference to all the fields of `self`.
    pub fn fields(&self) -> &[Variant] {
        &self.0
    }

    /// Create a new `Structure`.
    ///
    /// Same as `Structure::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append `field` to `self`.
    ///
    /// This method returns `Self` so that you can use the builder pattern to create a complex
    /// structure.
    pub fn add_field<T>(mut self, field: T) -> Self
    where
        T: Encode,
    {
        self.0.push(field.to_variant());

        self
    }
}

impl Encode for Structure {
    // The real single character signature for STRUCT is `r` but that's not actually used in practice for D-Bus at least
    // (the spec clearly states that this signature must never appear on the bus). The openning and closing braces are
    // used in practice and that's why we'll declare the opening brace as the signature for this type.
    const SIGNATURE_CHAR: char = '(';
    const SIGNATURE_STR: &'static str = "(";
    const ALIGNMENT: usize = 8;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);

        // Since a Structure always starts at 8-byte boundry, the fields and their children are
        // already aligned correctly.
        for field in &self.0 {
            field.encode_value_into(bytes, format);
        }
    }

    fn signature(&self) -> Signature {
        let mut signature = String::from("(");
        for field in &self.0 {
            signature.push_str(&field.value_signature());
        }
        signature.push_str(")");
        Signature::from(signature)
    }

    fn to_variant(self) -> Variant {
        Variant::Structure(self)
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::Structure(_) = variant {
            true
        } else {
            false
        }
    }
}

impl Decode for Structure {
    fn slice_data(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        let data = data.into();
        let padding = Self::padding(data.position(), format);
        if data.len() < padding {
            return Err(VariantError::InsufficientData);
        }
        let signature = Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        let mut i = 1;
        let last_index = signature.len() - 1;
        while i < last_index {
            let child_signature = crate::decode::slice_signature(&signature[i..last_index])?;
            let slice = crate::decode::slice_data(
                data.tail(extracted as usize),
                child_signature.as_str(),
                format,
            )?;
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

    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        // Similar to slice_data, except we create variants.
        let data = data.into();
        let padding = Self::padding(data.position(), format);
        let signature = signature.into();
        if data.len() < padding || signature.len() < 3 {
            return Err(VariantError::InsufficientData);
        }
        let signature = Self::ensure_correct_signature(signature)?;

        let encoding = data.tail(padding);
        let fields = variants_from_struct_data(&encoding, signature, format)?;

        Ok(Self(fields))
    }

    fn ensure_correct_signature(
        signature: impl Into<Signature>,
    ) -> Result<Signature, VariantError> {
        let signature = signature.into();
        if signature.len() < 3 {
            return Err(VariantError::InsufficientData);
        }
        if !signature.starts_with('(') || !signature.ends_with(')') {
            return Err(VariantError::IncorrectType);
        }

        let mut i = 1;
        while i < signature.len() - 1 {
            // Ensure we've only valid child signatures
            let child_signature = crate::decode::slice_signature(&signature[i..])?;
            i += child_signature.len();
        }

        Ok(signature)
    }

    fn slice_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError> {
        let signature = signature.into();
        if !signature.starts_with('(') {
            return Err(VariantError::IncorrectType);
        }

        let mut open_braces = 1;
        let mut i = 1;
        while i < signature.len() {
            if &signature[i..=i] == ")" {
                open_braces -= 1;

                if open_braces == 0 {
                    break;
                }
            } else if &signature[i..=i] == "(" {
                open_braces += 1;
            }

            i += 1;
        }
        if &signature[i..=i] != ")" {
            return Err(VariantError::IncorrectType);
        }

        Ok(Signature::from(&signature[0..=i]))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::Structure(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::Structure(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

fn variants_from_struct_data(
    data: &SharedData,
    signature: impl Into<Signature>,
    format: EncodingFormat,
) -> Result<Vec<Variant>, VariantError> {
    // Assuming simple types here but it's OK to have more capacity than needed
    let signature = signature.into();
    let mut fields = Vec::with_capacity(signature.len());
    let mut extracted = 0;
    let mut i = 1;
    let last_index = signature.len() - 1;
    while i < last_index {
        let child_signature = crate::slice_signature(&signature[i..last_index])?;

        let child_slice =
            crate::decode::slice_data(data.tail(extracted), child_signature.as_str(), format)?;
        extracted += child_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }
        let variant = Variant::from_data_slice(child_slice, child_signature.as_str(), format)?;
        fields.push(variant);

        i += child_signature.len();
    }
    if extracted == 0 {
        return Err(VariantError::ExcessData);
    }

    Ok(fields)
}
