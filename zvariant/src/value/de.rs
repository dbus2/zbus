//! Deserialization of `Value` into Rust types.
//!
//! This modules provides a `Deserializer` implementation for
//! `Value` which allows for deserializing a `Value` into
//! Rust types.
//!
//! Normally this module is not used directly, but is instead
//! invoked via the `zvariant::from_value` method provied
//! by this create. This deserializer can also be constructed
//! by calling `Value::into_deserializer`.

use crate::{Array, Dict, Value};
use serde::{
    de::{
        EnumAccess, Error, IntoDeserializer, MapAccess, SeqAccess, Unexpected, VariantAccess,
        Visitor,
    },
    Deserializer,
};
use std::{collections::VecDeque, os::fd::AsRawFd};

macro_rules! deserialize_method {
    ($method:ident($($arg:ident: $type:ty),*)) => {
        #[inline]
        fn $method<V>(self, $($arg: $type,)* visitor: V) -> Result<V::Value, crate::Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Value::Value(value) => value.into_deserializer().$method($($arg,)* visitor),
                _ => self.deserialize_any(visitor)
            }
        }
    }
}

/// Deserialize a `Value` into a Rust type.
pub struct ValueDeserializer<'de> {
    value: Value<'de>,
}

impl<'de> ValueDeserializer<'de> {
    pub fn new(value: Value<'de>) -> Self {
        Self { value }
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            Value::U8(v) => visitor.visit_u8(v),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::I16(v) => visitor.visit_i16(v),
            Value::U16(v) => visitor.visit_u16(v),
            Value::I32(v) => visitor.visit_i32(v),
            Value::Fd(v) => visitor.visit_i32(v.as_raw_fd()),
            Value::U32(v) => visitor.visit_u32(v),
            Value::I64(v) => visitor.visit_i64(v),
            Value::U64(v) => visitor.visit_u64(v),
            Value::F64(v) => visitor.visit_f64(v),
            Value::Str(v) => visitor.visit_str(v.as_str()),
            Value::Signature(sig) => visitor.visit_str(sig.as_str()),
            Value::ObjectPath(path) => visitor.visit_str(path.as_str()),
            Value::Value(value) => visitor.visit_map(ValueAccess::new(*value)),
            Value::Array(array) => visitor.visit_seq(ArrayAccess::new(array)),
            Value::Dict(dict) => visitor.visit_map(DictAccess::new(dict)),
            Value::Structure(structure) => {
                visitor.visit_seq(StructureAccess::new(structure.into_fields().into()))
            }
            #[cfg(feature = "gvariant")]
            Value::Maybe(value) => {
                if let Some(value) = value.into_inner() {
                    visitor.visit_some(value.into_deserializer())
                } else {
                    visitor.visit_none()
                }
            }
        }
    }

    deserialize_method!(deserialize_bool());
    deserialize_method!(deserialize_i8());
    deserialize_method!(deserialize_i16());
    deserialize_method!(deserialize_i32());
    deserialize_method!(deserialize_i64());
    deserialize_method!(deserialize_u8());
    deserialize_method!(deserialize_u16());
    deserialize_method!(deserialize_u32());
    deserialize_method!(deserialize_u64());
    deserialize_method!(deserialize_f32());
    deserialize_method!(deserialize_f64());
    deserialize_method!(deserialize_str());
    deserialize_method!(deserialize_string());
    deserialize_method!(deserialize_bytes());
    deserialize_method!(deserialize_byte_buf());
    deserialize_method!(deserialize_unit());
    deserialize_method!(deserialize_unit_struct(n: &'static str));
    deserialize_method!(deserialize_seq());
    deserialize_method!(deserialize_map());
    deserialize_method!(deserialize_tuple(n: usize));
    deserialize_method!(deserialize_struct(n: &'static str, f: &'static [&'static str]));
    deserialize_method!(deserialize_identifier());
    deserialize_method!(deserialize_ignored_any());

    fn deserialize_option<V>(self, #[allow(unused_variables)] visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            #[cfg(feature = "gvariant")]
            Value::Maybe(maybe) => {
                if let Some(value) = maybe.into_inner() {
                    visitor.visit_some(value.into_deserializer())
                } else {
                    visitor.visit_none()
                }
            }

            #[cfg(feature = "option-as-array")]
            Value::Array(mut array) => {
                if array.is_empty() {
                    visitor.visit_none()
                } else if array.len() == 1 {
                    let elem = array.remove().unwrap();
                    visitor.visit_some(elem.into_deserializer())
                } else {
                    Err(crate::Error::invalid_value(
                        Unexpected::Seq,
                        &"0 or 1 elements for an optional type",
                    ))
                }
            }

            value => Err(crate::Error::invalid_value(
                value.unexpected(),
                &"an optional type",
            )),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Value::Value(inner) = self.value {
            return inner
                .into_deserializer()
                .deserialize_newtype_struct(name, visitor);
        }

        match name {
            "zvariant::ObjectPath" => match self.value {
                Value::ObjectPath(object_path) => {
                    visitor.visit_newtype_struct(object_path.into_deserializer())
                }
                value => Err(Error::invalid_value(value.unexpected(), &"an object path")),
            },
            "zvariant::Signature" => match self.value {
                Value::Signature(sig) => visitor.visit_newtype_struct(sig.into_deserializer()),
                value => Err(Error::invalid_value(value.unexpected(), &"a signature")),
            },
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::U8(byte) => visitor.visit_char(byte as char),
            Value::Str(string) => {
                if string.len() == 1 {
                    visitor.visit_char(string.chars().next().unwrap())
                } else {
                    Err(Error::invalid_value(
                        Unexpected::Str(string.as_str()),
                        &"a single character",
                    ))
                }
            }
            Value::Value(value) => value.into_deserializer().deserialize_char(visitor),
            value => Err(crate::Error::invalid_value(value.unexpected(), &"a char")),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Structure(structure) => {
                visitor.visit_enum(StructureEnumAccess::new(structure.into_fields().into()))
            }
            value => Err(Error::invalid_value(value.unexpected(), &"an enum")),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Structure(structure) => {
                visitor.visit_seq(StructureAccess::new(structure.into_fields().into()))
            }
            Value::Array(array) => visitor.visit_seq(ArrayAccess::new(array)),
            value => Err(Error::invalid_value(value.unexpected(), &"a tuple struct")),
        }
    }
}

/// The internal state of a `ValueAccess`
enum ValueAccessState {
    /// The signature is being read
    ReadingSignature,

    /// The value is being read
    ReadingValue,

    /// The deserialization is done and
    /// there are no more values to read
    Done,
}

/// Deserialize a `Value` from a DBus/GVariant variant.
struct ValueAccess<'de> {
    value: Value<'de>,
    state: ValueAccessState,
}

impl<'de> ValueAccess<'de> {
    pub fn new(value: Value<'de>) -> Self {
        Self {
            value,
            state: ValueAccessState::ReadingSignature,
        }
    }
}

impl<'de> MapAccess<'de> for ValueAccess<'de> {
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.state {
            ValueAccessState::ReadingSignature => seed
                .deserialize("zvariant::Value::Signature".into_deserializer())
                .map(Some),
            ValueAccessState::ReadingValue => seed
                .deserialize("zvariant::Value::Value".into_deserializer())
                .map(Some),
            ValueAccessState::Done => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.state {
            ValueAccessState::ReadingSignature => {
                self.state = ValueAccessState::ReadingValue;
                seed.deserialize(self.value.value_signature().into_deserializer())
            }
            ValueAccessState::ReadingValue => {
                self.state = ValueAccessState::Done;
                seed.deserialize(self.value.try_clone()?.into_deserializer())
            }
            ValueAccessState::Done => unreachable!(),
        }
    }
}

/// Deserialize a sequence of values from a given `Array`.
struct ArrayAccess<'de> {
    array: Array<'de>,
}

impl<'de> ArrayAccess<'de> {
    pub fn new(array: Array<'de>) -> Self {
        Self { array }
    }
}

impl<'de> SeqAccess<'de> for ArrayAccess<'de> {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(item) = self.array.remove() {
            seed.deserialize(item.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
}

/// Deserialize a map of values from a given `Dict`
struct DictAccess<'de> {
    dict: Dict<'de, 'de>,
    next_value: Option<Value<'de>>,
}

impl<'de> DictAccess<'de> {
    pub fn new(dict: Dict<'de, 'de>) -> Self {
        Self {
            dict,
            next_value: None,
        }
    }
}

impl<'de> MapAccess<'de> for DictAccess<'de> {
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.dict.remove() {
            self.next_value = Some(value);
            seed.deserialize(key.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.next_value.take() {
            seed.deserialize(value.into_deserializer())
        } else {
            unreachable!()
        }
    }
}

/// Deserialize a given sequence of fields taken from a `Structure`
/// into a rust sequence
struct StructureAccess<'de> {
    fields: VecDeque<Value<'de>>,
}

impl<'de> StructureAccess<'de> {
    pub fn new(fields: VecDeque<Value<'de>>) -> Self {
        Self { fields }
    }
}

impl<'de> SeqAccess<'de> for StructureAccess<'de> {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(item) = self.fields.pop_front() {
            seed.deserialize(item.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
}

/// Deserialize a given sequence of fields taken from a `Structure`
/// into a rust Enum
struct StructureEnumAccess<'de> {
    fields: VecDeque<Value<'de>>,
}

impl<'de> StructureEnumAccess<'de> {
    pub fn new(fields: VecDeque<Value<'de>>) -> Self {
        Self { fields }
    }
}

impl<'de> EnumAccess<'de> for StructureEnumAccess<'de> {
    type Error = crate::Error;
    type Variant = StructureEnumAccess<'de>;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(name_field) = self.fields.pop_front() {
            let result = seed.deserialize(name_field.into_deserializer())?;
            Ok((result, self))
        } else {
            Err(crate::Error::missing_field("Enum Discriminator"))
        }
    }
}

impl<'de> VariantAccess<'de> for StructureEnumAccess<'de> {
    type Error = crate::Error;

    fn unit_variant(mut self) -> Result<(), Self::Error> {
        if let Some(field) = self.fields.pop_front() {
            Err(crate::Error::invalid_value(
                field.unexpected(),
                &"Expected a unit variant",
            ))
        } else {
            Ok(())
        }
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(field) = self.fields.pop_front() {
            seed.deserialize(field.into_deserializer())
        } else {
            Err(crate::Error::missing_field("expected enum data"))
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(StructureAccess::new(self.fields))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(StructureAccess::new(self.fields))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;
    use crate::{ObjectPath, OwnedObjectPath, OwnedSignature, Signature, StructureBuilder};
    use serde::Deserialize;

    #[test]
    fn deserialize_i16() {
        let de = Value::I16(42).into_deserializer();
        let result: i16 = i16::deserialize(de).expect("Should find an i16");
        assert_eq!(result, 42);
    }

    #[test]
    fn deserialize_i32() {
        let de = Value::I32(42).into_deserializer();
        let result: i32 = i32::deserialize(de).expect("Should find an i32");
        assert_eq!(result, 42);
    }

    #[test]
    fn deserialize_i64() {
        let de = Value::I64(42).into_deserializer();
        let result: i64 = i64::deserialize(de).expect("Should find an i64");
        assert_eq!(result, 42);
    }

    #[test]
    fn deserialize_u8() {
        let de = Value::U8(42).into_deserializer();
        let result: u8 = u8::deserialize(de).expect("Should find an u8");
        assert_eq!(result, 42);
    }

    #[test]
    fn deserialize_u16() {
        let de = Value::U16(42).into_deserializer();
        let result: u16 = u16::deserialize(de).expect("Should find an u16");
        assert_eq!(result, 42);
    }

    #[test]
    fn deserialize_u32() {
        let de = Value::U32(42).into_deserializer();
        let result: u32 = u32::deserialize(de).expect("Should find an u32");
        assert_eq!(result, 42);
    }

    #[test]
    fn deserialize_u64() {
        let de = Value::U64(42).into_deserializer();
        let result: u64 = u64::deserialize(de).expect("Should find an u64");
        assert_eq!(result, 42);
    }

    #[test]
    fn deserialize_f64() {
        let de = Value::F64(3.14).into_deserializer();
        let result: f64 = f64::deserialize(de).expect("Should find an f64");
        assert_eq!(result, 3.14);
    }

    #[test]
    fn deserialize_char() {
        let de = Value::U8(b'a').into_deserializer();
        let result: char = char::deserialize(de).expect("Should find a char");
        assert_eq!(result, 'a');
    }

    #[test]
    fn deserialize_str() {
        let de = Value::Str("hello".into()).into_deserializer();
        let result: String = String::deserialize(de).expect("Should find a string");
        assert_eq!(result, "hello");
    }

    #[test]
    fn deserialize_signature() {
        let de = Value::Signature(Signature::from_str_unchecked("a{sv}")).into_deserializer();
        let result: Signature<'_> = Signature::deserialize(de).expect("Should find a signature");
        assert_eq!(result.as_str(), "a{sv}");
    }

    #[test]
    fn deserialize_object_path() {
        let de = Value::ObjectPath(ObjectPath::from_str_unchecked("/hello")).into_deserializer();
        let result: ObjectPath<'_> =
            ObjectPath::deserialize(de).expect("Should find an object path");
        assert_eq!(result.as_str(), "/hello");
    }

    #[test]
    fn deserialize_array() {
        let array = Array::from(vec![42, 43]);
        let de = Value::Array(array).into_deserializer();
        let result: Vec<u32> = Vec::deserialize(de).expect("Should find an array");
        assert_eq!(result, vec![42, 43]);
    }

    #[test]
    fn deserialize_dict() {
        let mut dict = Dict::new(
            Signature::from_str_unchecked("s"),
            Signature::from_str_unchecked("i"),
        );

        dict.add("hello", 42).expect("Should append");
        dict.add("world", 43).expect("Should append");

        let de = Value::Dict(dict).into_deserializer();
        let result: std::collections::HashMap<String, u32> =
            std::collections::HashMap::deserialize(de).expect("Should find a dict");
        assert_eq!(result.get("hello"), Some(&42));
        assert_eq!(result.get("world"), Some(&43));
    }

    #[test]
    fn deserialize_dict_struct() {
        #[derive(Deserialize)]
        struct MyStruct {
            a: OwnedObjectPath,
            b: OwnedSignature,
        }

        let mut dict = Dict::new(
            Signature::from_str_unchecked("s"),
            Signature::from_str_unchecked("v"),
        );

        dict.add(
            "a",
            Value::new(ObjectPath::from_static_str_unchecked("/hello")),
        )
        .expect("Should append");
        dict.add("b", Value::new(Signature::from_static_str_unchecked("s")))
            .expect("Should append");

        let de = Value::Dict(dict).into_deserializer();
        let result: MyStruct = MyStruct::deserialize(de).expect("Should find a struct");
        assert_eq!(result.a.as_str(), "/hello");
        assert_eq!(result.b.as_str(), "s");
    }

    #[test]
    fn deserialize_dict_map() {
        let mut dict = Dict::new(
            Signature::from_str_unchecked("s"),
            Signature::from_str_unchecked("s"),
        );

        dict.add("a", "hello").expect("Should append");
        dict.add("b", "world").expect("Should append");

        let de = Value::Dict(dict).into_deserializer();
        let result: HashMap<String, String> =
            HashMap::<String, String>::deserialize(de).expect("Should find a dict");
        assert_eq!(result["a"].as_str(), "hello");
        assert_eq!(result["b"].as_str(), "world");
    }

    #[test]
    fn deserialize_newtype_struct() {
        #[derive(Deserialize)]
        struct MyStruct(i32);

        let structure = StructureBuilder::new().add_field(42i32).build();

        let de = Value::Structure(structure).into_deserializer();
        let result: MyStruct = MyStruct::deserialize(de).unwrap();
        assert_eq!(result.0, 42);
    }

    #[test]
    fn deserialize_newtype_variant() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum MyEnum {
            A(i32),
            B(i32),
        }

        {
            let structure = StructureBuilder::new()
                .add_field("A")
                .add_field(1i32)
                .build();

            let input = Value::Structure(structure);

            let de = input.into_deserializer();
            let output: MyEnum = MyEnum::deserialize(de).unwrap();

            assert_eq!(output, MyEnum::A(1));
        }

        {
            let structure = StructureBuilder::new()
                .add_field("B")
                .add_field(2i32)
                .build();

            let input = Value::Structure(structure);

            let de = input.into_deserializer();
            let output: MyEnum = MyEnum::deserialize(de).unwrap();

            assert_eq!(output, MyEnum::B(2));
        }
    }

    #[test]
    fn deserialize_unit_variant() {
        #[derive(Deserialize, PartialEq, Debug)]
        enum MyEnum {
            A,
            B,
        }

        {
            let structure = StructureBuilder::new().add_field("A").build();

            let input = Value::Structure(structure);

            let de = input.into_deserializer();
            let output: MyEnum = MyEnum::deserialize(de).unwrap();

            assert_eq!(output, MyEnum::A);
        }

        {
            let structure = StructureBuilder::new().add_field("B").build();

            let input = Value::Structure(structure);

            let de = input.into_deserializer();
            let output: MyEnum = MyEnum::deserialize(de).unwrap();

            assert_eq!(output, MyEnum::B);
        }
    }

    #[test]
    #[cfg(feature = "gvariant")]
    fn deserialize_maybe_some() {
        use crate::Maybe;
        let input = Value::Maybe(Maybe::just(Value::U8(1)));
        let de = input.into_deserializer();
        let output: Option<u8> = Option::deserialize(de).unwrap();
        assert_eq!(output, Some(1));
    }

    #[test]
    #[cfg(feature = "gvariant")]
    fn deserialize_maybe_none() {
        use crate::Maybe;
        let input = Value::Maybe(Maybe::nothing(signature_string!("y")));
        let de = input.into_deserializer();
        let output: Option<u8> = Option::deserialize(de).unwrap();
        assert_eq!(output, None);
    }

    #[test]
    #[cfg(feature = "option-as-array")]
    fn deserialize_array_some() {
        let input = Value::Array(Array::from(vec![Value::U8(1)]));
        let de = input.into_deserializer();
        let output: Option<u8> = Option::deserialize(de).unwrap();
        assert_eq!(output, Some(1));
    }

    #[test]
    #[cfg(feature = "option-as-array")]
    fn deserialize_array_none() {
        let input = Value::Array(Array::new(signature_string!("y")));
        let de = input.into_deserializer();
        let output: Option<u8> = Option::deserialize(de).unwrap();
        assert_eq!(output, None);
    }
}
