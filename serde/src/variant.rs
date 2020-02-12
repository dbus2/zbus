use std::convert::TryFrom;
use std::str;

use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

use crate::utils::VARIANT_SIGNATURE_STR;
use crate::VariantValue;
use crate::{Array, Dict, Error};
use crate::{ObjectPath, Signature, Structure};

/// A generic container, in the form of an enum that holds exactly one value of any of the other
/// types.
///
/// Note that this type is defined by the [D-Bus specification] and as such, its encoding is not the
/// same as that of the enclosed value.
///
/// # Example
///
/// TODO
///
/// [D-Bus specification]: https://dbus.freedesktop.org/doc/dbus-specification.html
#[derive(Debug, Clone)]
pub enum Variant<'a> {
    // Simple types
    U8(u8),
    Bool(bool),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F64(f64),
    Str(&'a str),
    Signature(Signature<'a>),
    ObjectPath(ObjectPath<'a>),
    Variant(Box<Variant<'a>>),

    // Container types
    Array(Array<'a>),
    Dict(Dict<'a, 'a>),
    Structure(Structure<'a>),
}

impl<'a> Variant<'a> {
    /// Get the signature of the enclosed value.
    pub fn value_signature(&self) -> Signature {
        match self {
            Variant::U8(_) => u8::signature(),
            Variant::Bool(_) => bool::signature(),
            Variant::I16(_) => i16::signature(),
            Variant::U16(_) => u16::signature(),
            Variant::I32(_) => i32::signature(),
            Variant::U32(_) => u32::signature(),
            Variant::I64(_) => i64::signature(),
            Variant::U64(_) => u64::signature(),
            Variant::F64(_) => f64::signature(),
            Variant::Str(_) => <&str>::signature(), // TODO: Optimize later!
            Variant::Signature(_) => Signature::signature(),
            Variant::ObjectPath(_) => ObjectPath::signature(),
            Variant::Variant(_) => Signature::from("v"),

            // Container types
            Variant::Array(value) => value.signature(),
            Variant::Dict(value) => value.signature(),
            Variant::Structure(value) => value.signature(),
        }
    }

    pub(crate) fn serialize_value_as_struct_field<S>(
        &self,
        name: &'static str,
        serializer: &mut S,
    ) -> Result<(), S::Error>
    where
        S: SerializeStruct,
    {
        match self {
            Variant::U8(value) => serializer.serialize_field(name, value),
            Variant::Bool(value) => serializer.serialize_field(name, value),
            Variant::I16(value) => serializer.serialize_field(name, value),
            Variant::U16(value) => serializer.serialize_field(name, value),
            Variant::I32(value) => serializer.serialize_field(name, value),
            Variant::U32(value) => serializer.serialize_field(name, value),
            Variant::I64(value) => serializer.serialize_field(name, value),
            Variant::U64(value) => serializer.serialize_field(name, value),
            Variant::F64(value) => serializer.serialize_field(name, value),
            Variant::Str(value) => serializer.serialize_field(name, value),
            Variant::Signature(value) => serializer.serialize_field(name, value),
            Variant::ObjectPath(value) => serializer.serialize_field(name, value),
            Variant::Variant(value) => serializer.serialize_field(name, value),

            // Container types
            Variant::Array(value) => serializer.serialize_field(name, value),
            Variant::Dict(value) => serializer.serialize_field(name, value),
            Variant::Structure(value) => serializer.serialize_field(name, value),
        }
    }

    // Really crappy that we need to do this separately for struct and seq cases. :(
    pub(crate) fn serialize_value_as_seq_element<S>(
        &self,
        serializer: &mut S,
    ) -> Result<(), S::Error>
    where
        S: SerializeSeq,
    {
        match self {
            Variant::U8(value) => serializer.serialize_element(value),
            Variant::Bool(value) => serializer.serialize_element(value),
            Variant::I16(value) => serializer.serialize_element(value),
            Variant::U16(value) => serializer.serialize_element(value),
            Variant::I32(value) => serializer.serialize_element(value),
            Variant::U32(value) => serializer.serialize_element(value),
            Variant::I64(value) => serializer.serialize_element(value),
            Variant::U64(value) => serializer.serialize_element(value),
            Variant::F64(value) => serializer.serialize_element(value),
            Variant::Str(value) => serializer.serialize_element(value),
            Variant::Signature(value) => serializer.serialize_element(value),
            Variant::ObjectPath(value) => serializer.serialize_element(value),
            Variant::Variant(value) => serializer.serialize_element(value),

            // Container types
            Variant::Array(value) => serializer.serialize_element(value),
            Variant::Dict(value) => serializer.serialize_element(value),
            Variant::Structure(value) => serializer.serialize_element(value),
        }
    }
}

impl<'a> Serialize for Variant<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serializer implementation needs to ensure padding isn't added for Variant.
        let mut structure = serializer.serialize_struct("zvariant::Variant", 2)?;

        let signature = self.value_signature();
        structure.serialize_field("zvariant::Signature", &signature)?;

        self.serialize_value_as_struct_field("zvariant::Value", &mut structure)?;

        structure.end()
    }
}

impl<'a> VariantValue for Variant<'a> {
    fn signature() -> Signature<'static> {
        Signature::from(VARIANT_SIGNATURE_STR)
    }
}

//
// Conversions from encodable types to `Variant`
//

impl<'a> From<u8> for Variant<'a> {
    fn from(value: u8) -> Self {
        Variant::U8(value)
    }
}

impl<'a> From<i8> for Variant<'a> {
    fn from(value: i8) -> Self {
        Variant::I16(value as i16)
    }
}

impl<'a> From<bool> for Variant<'a> {
    fn from(value: bool) -> Self {
        Variant::Bool(value)
    }
}

impl<'a> From<u16> for Variant<'a> {
    fn from(value: u16) -> Self {
        Variant::U16(value)
    }
}

impl<'a> From<i16> for Variant<'a> {
    fn from(value: i16) -> Self {
        Variant::I16(value)
    }
}

impl<'a> From<u32> for Variant<'a> {
    fn from(value: u32) -> Self {
        Variant::U32(value)
    }
}

impl<'a> From<i32> for Variant<'a> {
    fn from(value: i32) -> Self {
        Variant::I32(value)
    }
}

impl<'a> From<u64> for Variant<'a> {
    fn from(value: u64) -> Self {
        Variant::U64(value)
    }
}

impl<'a> From<i64> for Variant<'a> {
    fn from(value: i64) -> Self {
        Variant::I64(value)
    }
}

impl<'a> From<f32> for Variant<'a> {
    fn from(value: f32) -> Self {
        Variant::F64(value as f64)
    }
}

impl<'a> From<f64> for Variant<'a> {
    fn from(value: f64) -> Self {
        Variant::F64(value)
    }
}

impl<'a> From<&'a str> for Variant<'a> {
    fn from(value: &'a str) -> Self {
        Variant::Str(value)
    }
}

impl<'a> From<Signature<'a>> for Variant<'a> {
    fn from(value: Signature<'a>) -> Self {
        Variant::Signature(value)
    }
}

impl<'a> From<ObjectPath<'a>> for Variant<'a> {
    fn from(value: ObjectPath<'a>) -> Self {
        Variant::ObjectPath(value)
    }
}

impl<'a> From<Array<'a>> for Variant<'a> {
    fn from(value: Array<'a>) -> Self {
        Variant::Array(value)
    }
}

impl<'a> From<Dict<'a, 'a>> for Variant<'a> {
    fn from(value: Dict<'a, 'a>) -> Self {
        Variant::Dict(value)
    }
}

impl<'a, S> From<S> for Variant<'a>
where
    S: Into<Structure<'a>>,
{
    fn from(value: S) -> Self {
        Variant::Structure(value.into())
    }
}

impl<'a, V> From<&[V]> for Variant<'a>
where
    V: VariantValue + Into<Variant<'a>> + Clone,
{
    fn from(values: &[V]) -> Self {
        Variant::Array(Array::from(values))
    }
}

impl<'a, V> From<Vec<V>> for Variant<'a>
where
    V: VariantValue + Into<Variant<'a>> + Clone,
{
    fn from(values: Vec<V>) -> Self {
        Variant::Array(Array::from(values))
    }
}

//
// Conversions from `Variant` to encodable types
//

// u8

impl<'a> TryFrom<Variant<'a>> for u8 {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::U8(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a u8 {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::U8(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// bool

impl<'a> TryFrom<Variant<'a>> for bool {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::Bool(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a bool {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::Bool(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i16

impl<'a> TryFrom<Variant<'a>> for i16 {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::I16(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a i16 {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::I16(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u16

impl<'a> TryFrom<Variant<'a>> for u16 {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::U16(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a u16 {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::U16(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i32

impl<'a> TryFrom<Variant<'a>> for i32 {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::I32(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a i32 {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::I32(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u32

impl<'a> TryFrom<Variant<'a>> for u32 {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::U32(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a u32 {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::U32(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i64

impl<'a> TryFrom<Variant<'a>> for i64 {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::I64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a i64 {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::I64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u64

impl<'a> TryFrom<Variant<'a>> for u64 {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::U64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a u64 {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::U64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// f64

impl<'a> TryFrom<Variant<'a>> for f64 {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::F64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a f64 {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::F64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// &str

impl<'a> TryFrom<Variant<'a>> for &'a str {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::Str(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a str {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::Str(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Signature

impl<'a> TryFrom<Variant<'a>> for Signature<'a> {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::Signature(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a Signature<'a> {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::Signature(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// ObjectPath

impl<'a> TryFrom<Variant<'a>> for ObjectPath<'a> {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::ObjectPath(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a ObjectPath<'a> {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::ObjectPath(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Array

impl<'a> TryFrom<Variant<'a>> for Array<'a> {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::Array(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a Array<'a> {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::Array(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// DictEntry

impl<'a> TryFrom<Variant<'a>> for Dict<'a, 'a> {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::Dict(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a Dict<'a, 'a> {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::Dict(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Structure

impl<'a> TryFrom<Variant<'a>> for Structure<'a> {
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Error> {
        if let Variant::Structure(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl<'a> TryFrom<&'a Variant<'a>> for &'a Structure<'a> {
    type Error = Error;

    fn try_from(value: &'a Variant<'a>) -> Result<Self, Error> {
        if let Variant::Structure(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}
