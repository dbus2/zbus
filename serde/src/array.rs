use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::{Signature, Variant, VariantValue};

/// An unordered collection of items of the same type.
///
/// API is provided to create this from a [`Vec`].
///
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[derive(Debug, Clone)]
pub struct Array<'a> {
    element_signature: Signature<'a>,
    elements: Vec<Variant<'a>>,
}

impl<'a> Array<'a> {
    pub fn get(&self) -> &[Variant<'a>] {
        &self.elements
    }

    pub fn signature(&self) -> Signature<'static> {
        Signature::from(format!("a{}", self.element_signature.as_str()))
    }

    pub fn element_signature(&self) -> &Signature {
        &self.element_signature
    }
}

impl<'a> std::ops::Deref for Array<'a> {
    type Target = [Variant<'a>];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, V> From<Vec<V>> for Array<'a>
where
    V: VariantValue + Into<Variant<'a>>,
{
    fn from(values: Vec<V>) -> Self {
        let element_signature = V::signature();
        let elements = values.into_iter().map(|value| value.into()).collect();

        Self {
            element_signature,
            elements,
        }
    }
}

impl<'a, V> From<&[V]> for Array<'a>
where
    V: VariantValue + Into<Variant<'a>> + Clone,
{
    fn from(values: &[V]) -> Self {
        let element_signature = V::signature();
        let elements = values
            .into_iter()
            .map(|value| value.clone().into())
            .collect();

        Self {
            element_signature,
            elements,
        }
    }
}

impl<'a, V> From<&Vec<V>> for Array<'a>
where
    V: VariantValue + Into<Variant<'a>> + Clone,
{
    fn from(values: &Vec<V>) -> Self {
        Self::from(&values[..])
    }
}

impl<'a> Serialize for Array<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.elements.len()))?;
        for element in &self.elements {
            element.serialize_value_as_seq_element(&mut seq)?;
        }

        seq.end()
    }
}
