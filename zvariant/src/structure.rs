use serde::ser::{Serialize, SerializeTupleStruct, Serializer};

use crate::{OwnedValue, Signature, Type, Value};

/// Use this to efficiently build a [`Structure`].
///
/// [`Structure`]: struct.Structure.html
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StructureBuilder<'a>(Vec<Value<'a>>);

impl<'a> StructureBuilder<'a> {
    /// Create a new `StructureBuilder`.
    ///
    /// Same as `StructureBuilder::default()`.
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

    /// Build the `Structure`.
    ///
    /// [`Structure`]: struct.Structure.html
    pub fn build(self) -> Structure<'a> {
        let signature = create_signature_from_fields(&self.0);

        Structure {
            fields: self.0,
            signature,
        }
    }

    /// Same as `build` except Signature is provided.
    pub(crate) fn build_with_signature<'s: 'a>(self, signature: Signature<'s>) -> Structure<'a> {
        Structure {
            fields: self.0,
            signature,
        }
    }
}

/// A helper type to wrap structs in [`Value`].
///
/// API is provided to convert from, and to tuples.
///
/// [`Value`]: enum.Value.html
#[derive(Debug, Clone, PartialEq)]
pub struct Structure<'a> {
    fields: Vec<Value<'a>>,
    signature: Signature<'a>,
}

impl<'a> Structure<'a> {
    /// Get a reference to all the fields of `self`.
    pub fn fields(&self) -> &[Value<'a>] {
        &self.fields
    }

    /// Converts `self` to a `Vec` containing all its fields.
    pub fn into_fields(self) -> Vec<Value<'a>> {
        self.fields
    }

    /// Create a new `Structure`.
    ///
    /// Same as `Structure::default()`.
    #[deprecated(
        since = "2.3.0",
        note = "Please use `StructureBuilder` to create a `Structure` instead."
    )]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append `field` to `self`.
    ///
    /// This method returns `Self` so that you can use the builder pattern to create a complex
    /// structure.
    #[deprecated(
        since = "2.3.0",
        note = "Please use `StructureBuilder` to create a `Structure` instead."
    )]
    pub fn add_field<T>(self, field: T) -> Self
    where
        T: Type + Into<Value<'a>>,
    {
        #[allow(deprecated)]
        self.append_field(Value::new(field))
    }

    /// Append `field` to `self`.
    ///
    /// Identical to `add_field`, except the field must be in the form of a `Value`.
    #[deprecated(
        since = "2.3.0",
        note = "Please use `StructureBuilder` to create a `Structure` instead."
    )]
    pub fn append_field<'e: 'a>(mut self, field: Value<'e>) -> Self {
        self.fields.push(field);
        self.signature = create_signature_from_fields(&self.fields);

        self
    }

    /// Get the signature of this `Structure`.
    ///
    /// NB: This method potentially allocates and copies. Use [`full_signature`] if you'd like to
    /// avoid that.
    ///
    /// [`full_signature`]: #method.full_signature
    pub fn signature(&self) -> Signature<'static> {
        self.signature.to_owned()
    }

    /// Get the signature of this `Structure`.
    pub fn full_signature(&self) -> &Signature<'_> {
        &self.signature
    }

    pub(crate) fn to_owned(&self) -> Structure<'static> {
        Structure {
            fields: self.fields.iter().map(|v| v.to_owned()).collect(),
            signature: self.signature.to_owned(),
        }
    }
}

impl<'a> Default for Structure<'a> {
    fn default() -> Self {
        let signature = Signature::from_str_unchecked("()");

        Self {
            fields: vec![],
            signature,
        }
    }
}

impl<'a> Serialize for Structure<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut structure =
            serializer.serialize_tuple_struct("zvariant::Structure", self.fields.len())?;
        for field in &self.fields {
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
                    StructureBuilder::new()
                    $(
                        .add_field(value. $n)
                    )+
                    .build()
                }
            }

            impl<'a, E, $($name),+> std::convert::TryFrom<Structure<'a>> for ($($name),+,)
            where
                $($name: std::convert::TryFrom<Value<'a>, Error = E>,)+
                crate::Error: From<E>,

            {
                type Error = crate::Error;

                fn try_from(mut s: Structure<'a>) -> core::result::Result<Self, Self::Error> {
                    Ok((
                    $(
                         $name::try_from(s.fields.remove(0))?,
                    )+
                    ))
                }
            }

            impl<'a, E, $($name),+> std::convert::TryFrom<Value<'a>> for ($($name),+,)
            where
                $($name: std::convert::TryFrom<Value<'a>, Error = E>,)+
                crate::Error: From<E>,

            {
                type Error = crate::Error;

                fn try_from(v: Value<'a>) -> core::result::Result<Self, Self::Error> {
                    Self::try_from(Structure::try_from(v)?)
                }
            }

            impl<E, $($name),+> std::convert::TryFrom<OwnedValue> for ($($name),+,)
            where
                $($name: std::convert::TryFrom<Value<'static>, Error = E>,)+
                crate::Error: From<E>,

            {
                type Error = crate::Error;

                fn try_from(v: OwnedValue) -> core::result::Result<Self, Self::Error> {
                    Self::try_from(Value::from(v))
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

fn create_signature_from_fields(fields: &[Value<'_>]) -> Signature<'static> {
    let mut signature = String::with_capacity(255);
    signature.push('(');
    for field in fields {
        signature.push_str(&field.value_signature());
    }
    signature.push(')');

    Signature::from_string_unchecked(signature)
}
