use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{Signature, Variant, VariantValue};

/// An ordered collection of items of arbitrary types.
///
/// This is mostly just a way to support custom data structures. You only use this for structures
/// inside [`Variant`].
///
/// # Example
///
/// TODO
///
/// [`Variant`]: enum.Variant.html
#[derive(Debug, Clone, Default)]
pub struct Structure<'a>(Vec<Variant<'a>>);

impl<'a> Structure<'a> {
    /// Get all the fields, consuming `self`.
    pub fn take_fields(self) -> Vec<Variant<'a>> {
        self.0
    }

    /// Get a reference to all the fields of `self`.
    pub fn fields(&self) -> &[Variant<'a>] {
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
    pub fn add_field<V>(mut self, field: V) -> Self
    where
        V: VariantValue + Into<Variant<'a>>,
    {
        self.0.push(field.into());

        self
    }

    pub fn signature(&self) -> Signature<'static> {
        let mut signature = String::from("(");
        for field in &self.0 {
            signature.push_str(&field.value_signature());
        }
        signature.push_str(")");

        Signature::from(signature)
    }
}

impl<'a> Serialize for Structure<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut structure = serializer.serialize_struct("zvariant::Structure", self.0.len())?;
        for field in &self.0 {
            // FIXME: field names should be unique within the structure.
            field.serialize_value_as_struct_field("zvariant::Structure::field", &mut structure)?;
        }
        structure.end()
    }
}
