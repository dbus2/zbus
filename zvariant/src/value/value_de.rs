use std::os::fd::AsRawFd;

use serde::de::IntoDeserializer;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserializer;
use crate::Array;
use crate::Dict;
use crate::ObjectPath;
use crate::Signature;
use crate::Structure;
use crate::Value;

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

pub struct ValueDeserializer<'de> {
    value: Value<'de>
}

impl<'de> ValueDeserializer<'de> {
    pub fn new(value: Value<'de>) -> Self {
        Self {
            value
        }
    }
}

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = crate::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de> {
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
            Value::Structure(structure) => visitor.visit_seq(StructureAccess::new(structure)),
            #[cfg(feature = "gvariant")]
            Value::Maybe(value) => todo!()
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
    deserialize_method!(deserialize_char());
    deserialize_method!(deserialize_str());
    deserialize_method!(deserialize_string());
    deserialize_method!(deserialize_bytes());
    deserialize_method!(deserialize_byte_buf());
    deserialize_method!(deserialize_option());
    deserialize_method!(deserialize_unit());
    deserialize_method!(deserialize_unit_struct(n: &'static str));
    deserialize_method!(deserialize_newtype_struct(n: &'static str));
    deserialize_method!(deserialize_seq());
    deserialize_method!(deserialize_map());
    deserialize_method!(deserialize_tuple(n: usize));
    deserialize_method!(deserialize_tuple_struct(n: &'static str, l: usize));
    deserialize_method!(deserialize_struct(n: &'static str, f: &'static [&'static str]));
    deserialize_method!(deserialize_enum(n: &'static str, f: &'static [&'static str]));
    deserialize_method!(deserialize_identifier());
    deserialize_method!(deserialize_ignored_any());
}

enum ValueAccessState {
    ReadingSignature,
    ReadingValue,
    Done
}

struct ValueAccess<'de> {
    value: Value<'de>,
    state: ValueAccessState
}

impl<'de> ValueAccess<'de> {
    pub fn new(value: Value<'de>) -> Self {
        Self {
            value,
            state: ValueAccessState::ReadingSignature
        }
    }
}

impl<'de> MapAccess<'de> for ValueAccess<'de> {
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de> {
        match self.state {
            ValueAccessState::ReadingSignature => {
                seed.deserialize("zvariant::Value::Signature".into_deserializer()).map(Some)
            },
            ValueAccessState::ReadingValue => {
                seed.deserialize("zvariant::Value::Value".into_deserializer()).map(Some)
            },
            ValueAccessState::Done => Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de> {
        match self.state {
            ValueAccessState::ReadingSignature => {
                self.state = ValueAccessState::ReadingValue;
                seed.deserialize(self.value.value_signature().into_deserializer())
            },
            ValueAccessState::ReadingValue => {
                self.state = ValueAccessState::Done;
                seed.deserialize(self.value.try_clone()?.into_deserializer())
            },
            ValueAccessState::Done => unreachable!()
        }
    }
}

struct ArrayAccess<'de> {
    array: Array<'de>
}

impl<'de> ArrayAccess<'de> {
    pub fn new(array: Array<'de>) -> Self {
        Self {
            array
        }
    }
}

impl<'de> SeqAccess<'de> for ArrayAccess<'de> {
    type Error = crate::Error;
    
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> {

        if let Some(item) = self.array.remove() {
            seed.deserialize(item.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
}

struct DictAccess<'de> {
    dict: Dict<'de, 'de>,
    next_value: Option<Value<'de>>
}

impl<'de> DictAccess<'de> {
    pub fn new(dict: Dict<'de, 'de>) -> Self {
        Self {
            dict,
            next_value: None
        }
    }
}

impl<'de> MapAccess<'de> for DictAccess<'de> {
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de> {
        
        if let Some((key, value)) = self.dict.remove() {
            self.next_value = Some(value);
            seed.deserialize(key.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de> {
        if let Some(value) = self.next_value.take() {
            seed.deserialize(value.into_deserializer())
        } else {
            unreachable!()
        }
    }
}

struct StructureAccess<'de> {
    fields: Vec<Value<'de>>
}

impl<'de> StructureAccess<'de> {
    pub fn new(structure: Structure<'de>) -> Self {
        Self {
            fields: structure.into_fields()
        }
    }
}

impl<'de> SeqAccess<'de> for StructureAccess<'de> {
    type Error = crate::Error;
    
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de> {

        if let Some(item) = self.fields.pop() {
            seed.deserialize(item.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::OwnedObjectPath;
    use crate::OwnedSignature;
    use super::*;
    use serde::Deserialize;
    use serde::Serialize;

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
        let result: i64 = i64::deserialize(de).expect("Should find an i32");
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
        let result: ObjectPath<'_> = ObjectPath::deserialize(de).expect("Should find an object path");
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
            Signature::from_str_unchecked("i")
        );

        dict.add("hello", 42).expect("Should append");
        dict.add("world", 43).expect("Should append");


        let de = Value::Dict(dict).into_deserializer();
        let result: std::collections::HashMap<String, u32> = std::collections::HashMap::deserialize(de).expect("Should find a dict");
        assert_eq!(result.get("hello"), Some(&42));
        assert_eq!(result.get("world"), Some(&43));
    }

    #[test]
    fn deserialize_dict_struct() {
        #[derive(Deserialize)]
        struct MyStruct {
            a: OwnedObjectPath,
            b: OwnedSignature
        }

        let mut dict = Dict::new(
            Signature::from_str_unchecked("s"),
            Signature::from_str_unchecked("v")
        );

        dict.add("a", Value::new(ObjectPath::from_static_str_unchecked("/hello"))).expect("Should append");
        dict.add("b", Value::new(Signature::from_static_str_unchecked("s"))).expect("Should append");

        let de = Value::Dict(dict).into_deserializer();
        let result: MyStruct = MyStruct::deserialize(de).expect("Should find a dict");
        assert_eq!(result.a.as_str(), "/hello");
        assert_eq!(result.b.as_str(), "s");
    }
}
