use std::str;

use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

use crate::utils::VARIANT_SIGNATURE_STR;
use crate::VariantValue;
use crate::{Array, Dict};
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
#[derive(Debug, Clone, PartialEq)]
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
        structure.serialize_field("zvariant::Variant::Signature", &signature)?;

        self.serialize_value_as_struct_field("zvariant::Variant::Value", &mut structure)?;

        structure.end()
    }
}

impl<'a> VariantValue for Variant<'a> {
    fn signature() -> Signature<'static> {
        Signature::from(VARIANT_SIGNATURE_STR)
    }
}
