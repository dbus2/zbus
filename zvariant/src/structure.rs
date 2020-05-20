use serde::ser::{Serialize, SerializeTupleStruct, Serializer};

use crate::{Signature, Type, Value};

/// An ordered collection of items of arbitrary types.
///
/// This is mostly just a way to support custom data structures. You only use this for structures
/// inside [`Value`]. API is provided to convert from, and to tuples.
///
/// [`Value`]: enum.Value.html
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Structure<'a>(Vec<Value<'a>>);

impl<'a> Structure<'a> {
    /// Get a reference to all the fields of `self`.
    pub fn fields(&self) -> &[Value<'a>] {
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
    pub fn add_field<T>(self, field: T) -> Self
    where
        T: Type + Into<Value<'a>>,
    {
        self.append_field(Value::new(field))
    }

    /// Append `field` to `self`.
    ///
    /// Identical to `add_field`, except the field must be in the form of a `Value`.
    pub fn append_field<'e: 'a>(mut self, field: Value<'e>) -> Self {
        self.0.push(field);

        self
    }

    /// Get the signature of this `Structure`.
    pub fn signature(&self) -> Signature<'static> {
        let mut signature = String::from("(");
        for field in &self.0 {
            signature.push_str(&field.value_signature());
        }
        signature.push_str(")");

        Signature::from_string_unchecked(signature)
    }

    pub(crate) fn to_owned(&self) -> Structure<'static> {
        Structure(self.0.iter().map(|v| v.to_owned()).collect())
    }
}

impl<'a> Serialize for Structure<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut structure =
            serializer.serialize_tuple_struct("zvariant::Structure", self.0.len())?;
        for field in &self.0 {
            field.serialize_value_as_tuple_struct_field(&mut structure)?;
        }
        structure.end()
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<'a, $($name),+> From<($($name),+,)> for Structure<'a>
            where
                $($name: Type + Into<Value<'a>>,)+
            {
                #[inline]
                fn from(value: ($($name),+,)) -> Self {
                    Structure::new()
                    $(
                        .add_field(value. $n)
                    )+
                }
            }

            impl<'a, $($name),+> std::convert::TryFrom<Structure<'a>> for ($($name),+,)
            where
                $($name: std::convert::TryFrom<Value<'a>>,)+
            {
                type Error = crate::Error;

                fn try_from(mut s: Structure<'a>) -> core::result::Result<Self, Self::Error> {
                    Ok((
                    $(
                        s.0.remove(0).downcast::<$name>().ok_or(crate::Error::IncorrectType)?,
                    )+
                    ))
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
