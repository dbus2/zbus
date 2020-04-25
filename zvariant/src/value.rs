use core::str;
use std::convert::TryFrom;
use std::marker::PhantomData;

use serde::de::{
    Deserialize, DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess, Unexpected, Visitor,
};
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

use crate::utils::*;
use crate::{Array, Dict};
use crate::{Basic, Type};
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
pub enum Value<'a> {
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
    Value(Box<Value<'a>>),

    // Container types
    Array(Array<'a>),
    Dict(Dict<'a, 'a>),
    Structure(Structure<'a>),
}

impl<'a> Value<'a> {
    /// Make a [`Value`] for a given value.
    ///
    /// In general, you can use [`Into`] trait on basic types, except
    /// when you explicitely need to wrap [`Value`] itself, in which
    /// case this constructor comes handy.
    ///
    /// [`Value`]: enum.Value.html
    /// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
    pub fn new<T>(value: T) -> Self
    where
        T: Into<Self> + Type,
    {
        // With specialization, we wouldn't have this
        if T::signature() == VARIANT_SIGNATURE_STR {
            Self::Value(Box::new(value.into()))
        } else {
            value.into()
        }
    }

    /// Get the signature of the enclosed value.
    pub fn value_signature(&self) -> Signature {
        match self {
            Value::U8(_) => u8::signature(),
            Value::Bool(_) => bool::signature(),
            Value::I16(_) => i16::signature(),
            Value::U16(_) => u16::signature(),
            Value::I32(_) => i32::signature(),
            Value::U32(_) => u32::signature(),
            Value::I64(_) => i64::signature(),
            Value::U64(_) => u64::signature(),
            Value::F64(_) => f64::signature(),
            Value::Str(_) => <&str>::signature(),
            Value::Signature(_) => Signature::signature(),
            Value::ObjectPath(_) => ObjectPath::signature(),
            Value::Value(_) => Signature::from_str_unchecked("v"),

            // Container types
            Value::Array(value) => value.signature(),
            Value::Dict(value) => value.signature(),
            Value::Structure(value) => value.signature(),
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
            Value::U8(value) => serializer.serialize_field(name, value),
            Value::Bool(value) => serializer.serialize_field(name, value),
            Value::I16(value) => serializer.serialize_field(name, value),
            Value::U16(value) => serializer.serialize_field(name, value),
            Value::I32(value) => serializer.serialize_field(name, value),
            Value::U32(value) => serializer.serialize_field(name, value),
            Value::I64(value) => serializer.serialize_field(name, value),
            Value::U64(value) => serializer.serialize_field(name, value),
            Value::F64(value) => serializer.serialize_field(name, value),
            Value::Str(value) => serializer.serialize_field(name, value),
            Value::Signature(value) => serializer.serialize_field(name, value),
            Value::ObjectPath(value) => serializer.serialize_field(name, value),
            Value::Value(value) => serializer.serialize_field(name, value),

            // Container types
            Value::Array(value) => serializer.serialize_field(name, value),
            Value::Dict(value) => serializer.serialize_field(name, value),
            Value::Structure(value) => serializer.serialize_field(name, value),
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
            Value::U8(value) => serializer.serialize_element(value),
            Value::Bool(value) => serializer.serialize_element(value),
            Value::I16(value) => serializer.serialize_element(value),
            Value::U16(value) => serializer.serialize_element(value),
            Value::I32(value) => serializer.serialize_element(value),
            Value::U32(value) => serializer.serialize_element(value),
            Value::I64(value) => serializer.serialize_element(value),
            Value::U64(value) => serializer.serialize_element(value),
            Value::F64(value) => serializer.serialize_element(value),
            Value::Str(value) => serializer.serialize_element(value),
            Value::Signature(value) => serializer.serialize_element(value),
            Value::ObjectPath(value) => serializer.serialize_element(value),
            Value::Value(value) => serializer.serialize_element(value),

            // Container types
            Value::Array(value) => serializer.serialize_element(value),
            Value::Dict(value) => serializer.serialize_element(value),
            Value::Structure(value) => serializer.serialize_element(value),
        }
    }

    /// Try to get `&x` from `&Value(x)` for type `T`.
    ///
    /// [`TryFrom`] is implemented for various `Value->T` conversions,
    /// and you can use that, as it is usually the most convenient.
    ///
    /// However, if you need to unwrap [`Value`] explicitely, and
    /// handle the `Value(Value) -> Value` case, then you should use
    /// this function (because [`TryFrom`] is idempotent on [`Value`]
    /// itself).
    ///
    /// [`Value`]: enum.Value.html
    /// [`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
    pub fn downcast_ref<T>(&'a self) -> Option<&'a T>
    where
        &'a T: TryFrom<&'a Value<'a>>,
    {
        if let Value::Value(v) = self {
            <&T>::try_from(v).ok()
        } else {
            <&T>::try_from(&self).ok()
        }
    }
}

impl<'a> Serialize for Value<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serializer implementation needs to ensure padding isn't added for Value.
        let mut structure = serializer.serialize_struct("zvariant::Value", 2)?;

        let signature = self.value_signature();
        structure.serialize_field("zvariant::Value::Signature", &signature)?;

        self.serialize_value_as_struct_field("zvariant::Value::Value", &mut structure)?;

        structure.end()
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for Value<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = ValueVisitor;

        deserializer.deserialize_any(visitor)
    }
}

// Note that the Visitor implementations don't check for validity of the
// signature. That's left to the Deserialize implementation of Signature
// itself.

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Value<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Value")
    }

    #[inline]
    fn visit_seq<V>(self, mut visitor: V) -> Result<Value<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let signature = visitor.next_element::<Signature>()?.ok_or_else(|| {
            Error::invalid_value(Unexpected::Other("nothing"), &"a Value signature")
        })?;
        let seed = ValueSeed::<Value> {
            signature,
            phantom: PhantomData,
        };

        visitor
            .next_element_seed(seed)?
            .ok_or_else(|| Error::invalid_value(Unexpected::Other("nothing"), &"a Value value"))
    }

    fn visit_map<V>(self, mut visitor: V) -> Result<Value<'de>, V::Error>
    where
        V: MapAccess<'de>,
    {
        let (_, signature) = visitor.next_entry::<&str, Signature>()?.ok_or_else(|| {
            Error::invalid_value(Unexpected::Other("nothing"), &"a Value signature")
        })?;
        let _ = visitor.next_key::<&str>()?;

        let seed = ValueSeed::<Value> {
            signature,
            phantom: PhantomData,
        };
        visitor.next_value_seed(seed)
    }
}

struct ValueSeed<'de, T> {
    signature: Signature<'de>,
    phantom: PhantomData<T>,
}

impl<'de, T> ValueSeed<'de, T>
where
    T: Deserialize<'de>,
{
    #[inline]
    fn visit_array<V>(self, mut visitor: V) -> Result<Value<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        // TODO: Why do we need String here?
        let signature = Signature::from_string_unchecked(String::from(&self.signature[1..]));
        let mut array = Array::new(signature.clone());

        while let Some(elem) = visitor.next_element_seed(ValueSeed::<Value> {
            signature: signature.clone(),
            phantom: PhantomData,
        })? {
            array.append(elem).map_err(Error::custom)?;
        }

        Ok(Value::Array(array))
    }

    #[inline]
    fn visit_struct<V>(self, mut visitor: V) -> Result<Value<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut i = 1;
        let signature_end = self.signature.len() - 1;
        let mut structure = Structure::new();
        while i < signature_end {
            let fields_signature = Signature::from_str_unchecked(&self.signature[i..signature_end]);
            let field_signature = slice_signature(&fields_signature).map_err(Error::custom)?;
            i += field_signature.len();
            // FIXME: Any way to avoid this allocation?
            let field_signature = Signature::from_string_unchecked(String::from(&field_signature));

            if let Some(field) = visitor.next_element_seed(ValueSeed::<Value> {
                signature: field_signature,
                phantom: PhantomData,
            })? {
                structure = structure.append_field(field);
            }
        }

        Ok(Value::Structure(structure))
    }

    #[inline]
    fn visit_variant<V>(self, visitor: V) -> Result<Value<'de>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        ValueVisitor
            .visit_seq(visitor)
            .map(|v| Value::Value(Box::new(v)))
    }
}

macro_rules! value_seed_basic_method {
    ($name:ident, $type:ty) => {
        #[inline]
        fn $name<E>(self, value: $type) -> Result<Value<'de>, E>
        where
            E: serde::de::Error,
        {
            Ok(value.into())
        }
    };
}

macro_rules! value_seed_str_method {
    ($name:ident, $type:ty, $variant:ident, $constructor:ident) => {
        #[inline]
        fn $name<E>(self, value: $type) -> Result<Value<'de>, E>
        where
            E: serde::de::Error,
        {
            match self.signature.as_str() {
                <&str>::SIGNATURE_STR => Ok(Value::$variant(value)),
                Signature::SIGNATURE_STR => Ok(Value::Signature(Signature::$constructor(value))),
                ObjectPath::SIGNATURE_STR => Ok(Value::ObjectPath(ObjectPath::$constructor(value))),
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

impl<'de, T> Visitor<'de> for ValueSeed<'de, T>
where
    T: Deserialize<'de>,
{
    type Value = Value<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Value value")
    }

    value_seed_basic_method!(visit_bool, bool);
    value_seed_basic_method!(visit_i16, i16);
    value_seed_basic_method!(visit_i32, i32);
    value_seed_basic_method!(visit_i64, i64);
    value_seed_basic_method!(visit_u8, u8);
    value_seed_basic_method!(visit_u16, u16);
    value_seed_basic_method!(visit_u32, u32);
    value_seed_basic_method!(visit_u64, u64);
    value_seed_basic_method!(visit_f64, f64);

    #[inline]
    fn visit_str<E>(self, value: &str) -> Result<Value<'de>, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(String::from(value))
    }

    value_seed_str_method!(visit_borrowed_str, &'de str, Str, from_str_unchecked);

    #[inline]
    fn visit_seq<V>(self, visitor: V) -> Result<Value<'de>, V::Error>
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
                &"a Value signature",
            )),
        }
    }

    #[inline]
    fn visit_map<V>(self, mut visitor: V) -> Result<Value<'de>, V::Error>
    where
        V: MapAccess<'de>,
    {
        // TODO: Why do we need String here?
        let key_signature = Signature::from_string_unchecked(String::from(&self.signature[2..3]));
        let signature_end = self.signature.len() - 1;
        let value_signature =
            Signature::from_string_unchecked(String::from(&self.signature[3..signature_end]));
        let mut dict = Dict::new(key_signature.clone(), value_signature.clone());

        while let Some((key, value)) = visitor.next_entry_seed(
            ValueSeed::<Value> {
                signature: key_signature.clone(),
                phantom: PhantomData,
            },
            ValueSeed::<Value> {
                signature: value_signature.clone(),
                phantom: PhantomData,
            },
        )? {
            dict.append(key, value).map_err(Error::custom)?;
        }

        Ok(Value::Dict(dict))
    }
}

impl<'de, T> DeserializeSeed<'de> for ValueSeed<'de, T>
where
    T: Deserialize<'de>,
{
    type Value = Value<'de>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

impl<'a> Type for Value<'a> {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked(VARIANT_SIGNATURE_STR)
    }
}
