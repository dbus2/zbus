use std::marker::PhantomData;
use std::str;

use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess, Unexpected, Visitor,
};
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

use crate::utils::*;
use crate::{Array, Dict};
use crate::{Basic, IntoVariant, VariantValue};
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
    String(String),
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
            Variant::String(_) | Variant::Str(_) => <&str>::signature(),
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
            Variant::String(value) => serializer.serialize_field(name, value),
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
            Variant::String(value) => serializer.serialize_element(value),
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

impl<'de: 'a, 'a> Deserialize<'de> for Variant<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = VariantVisitor;

        deserializer.deserialize_any(visitor)
    }
}

struct VariantVisitor;

impl<'de> Visitor<'de> for VariantVisitor {
    type Value = Variant<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Variant")
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Variant<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let signature = visitor.next_element::<Signature>()?.ok_or_else(|| {
            Error::invalid_value(Unexpected::Other("nothing"), &"a Variant signature")
        })?;
        let seed = VariantSeed::<Variant> {
            signature,
            phantom: PhantomData,
        };

        visitor
            .next_element_seed(seed)?
            .ok_or_else(|| Error::invalid_value(Unexpected::Other("nothing"), &"a Variant value"))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Variant<'de>, V::Error>
    where
        V: MapAccess<'de>,
    {
        let (_, signature) = visitor.next_entry::<&str, Signature>()?.ok_or_else(|| {
            Error::invalid_value(Unexpected::Other("nothing"), &"a Variant signature")
        })?;
        let _ = visitor.next_key::<&str>()?;

        let seed = VariantSeed::<Variant> {
            signature,
            phantom: PhantomData,
        };
        visitor.next_value_seed(seed)
    }
}

struct VariantSeed<'de, T> {
    signature: Signature<'de>,
    phantom: PhantomData<T>,
}

impl<'de, T> VariantSeed<'de, T>
where
    T: Deserialize<'de>,
{
    #[inline]
    fn visit_array<V>(self, mut visitor: V) -> Result<Variant<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        // TODO: Why do we need String here?
        let signature = Signature::from(String::from(&self.signature[1..]));
        let mut array = Array::new(signature.clone());

        while let Some(elem) = visitor.next_element_seed(VariantSeed::<Variant> {
            signature: signature.clone(),
            phantom: PhantomData,
        })? {
            array.append(elem).map_err(Error::custom)?;
        }

        Ok(Variant::Array(array))
    }

    #[inline]
    fn visit_struct<V>(self, mut visitor: V) -> Result<Variant<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut i = 1;
        let signature_end = self.signature.len() - 1;
        let mut structure = Structure::new();
        while i < signature_end {
            let fields_signature = Signature::from(&self.signature[i..signature_end]);
            let field_signature = slice_signature(&fields_signature).map_err(Error::custom)?;
            i += field_signature.len();
            // FIXME: Any way to avoid this allocation?
            let field_signature = Signature::from(String::from(field_signature.as_str()));

            if let Some(field) = visitor.next_element_seed(VariantSeed::<Variant> {
                signature: field_signature,
                phantom: PhantomData,
            })? {
                structure = structure.append_field(field);
            }
        }

        Ok(Variant::Structure(structure))
    }

    #[inline]
    fn visit_variant<V>(self, visitor: V) -> Result<Variant<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        VariantVisitor
            .visit_seq(visitor)
            .map(|v| Variant::Variant(Box::new(v)))
    }
}

macro_rules! variant_seed_basic_method {
    ($name:ident, $type:ty) => {
        #[inline]
        fn $name<E>(self, value: $type) -> Result<Variant<'de>, E>
        where
            E: serde::de::Error,
        {
            Ok(value.into_variant())
        }
    };
}

macro_rules! variant_seed_str_method {
    ($name:ident, $type:ty, $variant:ident) => {
        #[inline]
        fn $name<E>(self, value: $type) -> Result<Variant<'de>, E>
        where
            E: serde::de::Error,
        {
            match self.signature.as_str() {
                <&str>::SIGNATURE_STR => Ok(Variant::$variant(value)),
                Signature::SIGNATURE_STR => Ok(Variant::Signature(Signature::from(value))),
                ObjectPath::SIGNATURE_STR => Ok(Variant::ObjectPath(ObjectPath::from(value))),
                _ => {
                    let expected = format!(
                        "`{}`, `{}` or `{}`",
                        <&str>::SIGNATURE_STR,
                        Signature::SIGNATURE_STR,
                        ObjectPath::SIGNATURE_STR,
                    );
                    Err(Error::invalid_type(
                        Unexpected::Str(self.signature.as_str()),
                        &expected.as_str(),
                    ))
                }
            }
        }
    };
}

impl<'de, T> Visitor<'de> for VariantSeed<'de, T>
where
    T: Deserialize<'de>,
{
    type Value = Variant<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Variant value")
    }

    variant_seed_basic_method!(visit_bool, bool);
    variant_seed_basic_method!(visit_i16, i16);
    variant_seed_basic_method!(visit_i32, i32);
    variant_seed_basic_method!(visit_i64, i64);
    variant_seed_basic_method!(visit_u8, u8);
    variant_seed_basic_method!(visit_u16, u16);
    variant_seed_basic_method!(visit_u32, u32);
    variant_seed_basic_method!(visit_u64, u64);
    variant_seed_basic_method!(visit_f64, f64);

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Variant<'de>, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(String::from(value))
    }

    variant_seed_str_method!(visit_borrowed_str, &'de str, Str);
    variant_seed_str_method!(visit_string, String, String);

    #[inline]
    fn visit_seq<V>(self, visitor: V) -> Result<Variant<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        match self.signature.chars().next().ok_or_else(|| {
            Error::invalid_value(
                Unexpected::Other("nothing"),
                &"Array or Struct signature character",
            )
        })? {
            // For some reason rustc doesn't like us using ARRAY_SIGNATURE_CHAR const
            'a' => self.visit_array(visitor),
            '(' => self.visit_struct(visitor),
            'v' => self.visit_variant(visitor),
            c => Err(Error::invalid_value(
                Unexpected::Char(c),
                &"a Variant signature",
            )),
        }
    }

    #[inline]
    fn visit_map<V>(self, mut visitor: V) -> Result<Variant<'de>, V::Error>
    where
        V: MapAccess<'de>,
    {
        // TODO: Why do we need String here?
        println!("signature: {}", self.signature.as_str());
        let key_signature = Signature::from(String::from(&self.signature[2..3]));
        let signature_end = self.signature.len() - 1;
        let value_signature = Signature::from(String::from(&self.signature[3..signature_end]));
        let mut dict = Dict::new(key_signature.clone(), value_signature.clone());

        while let Some((key, value)) = visitor.next_entry_seed(
            VariantSeed::<Variant> {
                signature: key_signature.clone(),
                phantom: PhantomData,
            },
            VariantSeed::<Variant> {
                signature: value_signature.clone(),
                phantom: PhantomData,
            },
        )? {
            dict.append(key, value).map_err(Error::custom)?;
        }

        Ok(Variant::Dict(dict))
    }
}

impl<'de, T> DeserializeSeed<'de> for VariantSeed<'de, T>
where
    T: Deserialize<'de>,
{
    type Value = Variant<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'a> VariantValue for Variant<'a> {
    fn signature() -> Signature<'static> {
        Signature::from(VARIANT_SIGNATURE_STR)
    }
}
