use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{IntoValue, Signature, Type, Value};

/// An ordered collection of items of arbitrary types.
///
/// This is mostly just a way to support custom data structures. You only use this for structures
/// inside [`Value`].
///
/// # Example
///
/// TODO
///
/// [`Value`]: enum.Value.html
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Structure<'a>(Vec<Value<'a>>);

impl<'a> Structure<'a> {
    /// Get all the fields, consuming `self`.
    pub fn take_fields(self) -> Vec<Value<'a>> {
        self.0
    }

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
    pub fn add_field<T>(mut self, field: T) -> Self
    where
        T: Type + IntoValue<'a>,
    {
        self.0.push(field.into_value());

        self
    }

    pub fn append_field<'e: 'a>(mut self, field: Value<'e>) -> Self {
        self.0.push(field);

        self
    }

    pub fn signature(&self) -> Signature<'static> {
        let mut signature = String::from("(");
        for field in &self.0 {
            signature.push_str(&field.value_signature());
        }
        signature.push_str(")");

        Signature::from_string_unchecked(signature)
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

// Arrays are serialized as tupples by Serde and that's strange. Let's just not support it at all.
// TODO: Mention this fact in the module docs.

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<'a, $($name),+> From<($($name),+,)> for Structure<'a>
            where
                $($name: Type + IntoValue<'a>,)+
            {
                #[inline]
                fn from(value: ($($name),+,)) -> Self {
                    Structure::new()
                    $(
                        .add_field(value. $n)
                    )+
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
