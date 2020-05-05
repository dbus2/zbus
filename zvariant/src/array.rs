use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::convert::TryFrom;

use crate::{Error, Result};
use crate::{Signature, Type, Value};

/// An unordered collection of items of the same type.
///
/// This is used for keeping arrays in a [`Value`]. API is provided to convert from, and to a
/// [`Vec`].
///
/// [`Value`]: enum.Value.html#variant.Array
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[derive(Debug, Clone, PartialEq)]
pub struct Array<'a> {
    element_signature: Signature<'a>,
    elements: Vec<Value<'a>>,
    signature: Signature<'static>,
}

impl<'a> Array<'a> {
    /// Create a new empty `Array`, given the signature of the elements.
    pub fn new(element_signature: Signature) -> Array {
        let signature = create_signature(&element_signature);
        Array {
            element_signature,
            elements: vec![],
            signature,
        }
    }

    /// Append `element`.
    ///
    /// # Errors
    ///
    /// if `element`'s signature doesn't match the element signature `self` was created for.
    pub fn append<'e: 'a>(&mut self, element: Value<'e>) -> Result<()> {
        if element.value_signature() != self.element_signature {
            return Err(Error::IncorrectType);
        }

        self.elements.push(element);

        Ok(())
    }

    /// Get all the elements.
    pub fn get(&self) -> &[Value<'a>] {
        &self.elements
    }

    /// Get the number of elements.
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }

    /// Get the signature of this `Array`.
    pub fn signature(&self) -> Signature<'static> {
        self.signature.clone()
    }

    /// Get the signature of the elements in the `Array`.
    pub fn element_signature(&self) -> &Signature {
        &self.element_signature
    }

    pub(crate) fn to_owned(&self) -> Array<'static> {
        Array {
            element_signature: self.element_signature.to_owned(),
            elements: self.elements.iter().map(|v| v.to_owned()).collect(),
            signature: self.signature.to_owned(),
        }
    }
}

impl<'a> std::ops::Deref for Array<'a> {
    type Target = [Value<'a>];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'a, T> From<Vec<T>> for Array<'a>
where
    T: Type + Into<Value<'a>>,
{
    fn from(values: Vec<T>) -> Self {
        let element_signature = T::signature();
        let elements = values.into_iter().map(Value::new).collect();
        let signature = create_signature(&element_signature);

        Self {
            element_signature,
            elements,
            signature,
        }
    }
}

impl<'a, T> From<&[T]> for Array<'a>
where
    T: Type + Into<Value<'a>> + Clone,
{
    fn from(values: &[T]) -> Self {
        let element_signature = T::signature();
        let elements = values
            .iter()
            .map(|value| Value::new(value.clone()))
            .collect();
        let signature = create_signature(&element_signature);

        Self {
            element_signature,
            elements,
            signature,
        }
    }
}

impl<'a, T> From<&Vec<T>> for Array<'a>
where
    T: Type + Into<Value<'a>> + Clone,
{
    fn from(values: &Vec<T>) -> Self {
        Self::from(&values[..])
    }
}

impl<'a, T> TryFrom<Array<'a>> for Vec<T>
where
    T: TryFrom<Value<'a>, Error = Error>,
{
    type Error = Error;

    fn try_from(v: Array<'a>) -> core::result::Result<Self, Self::Error> {
        // there is no try_map yet..
        let mut res = vec![];
        for e in v.elements.into_iter() {
            res.push(T::try_from(e)?);
        }
        Ok(res)
    }
}

// TODO: this could be useful
// impl<'a, 'b, T> TryFrom<&'a Array<'b>> for Vec<T>

impl<'a> Serialize for Array<'a> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
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

fn create_signature(element_signature: &Signature) -> Signature<'static> {
    Signature::from_string_unchecked(format!("a{}", element_signature))
}
