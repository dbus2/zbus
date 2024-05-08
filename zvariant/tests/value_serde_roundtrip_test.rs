use serde::{Deserialize, Serialize};
use zvariant::{ObjectPath, OwnedValue, Signature, Type};

#[test]
fn serde_i8() {
    let value: i8 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: i8 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_i16() {
    let value: i16 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: i16 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_i32() {
    let value: i32 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: i32 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_i64() {
    let value: i64 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: i64 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_u8() {
    let value: u8 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: u8 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_u16() {
    let value: u16 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: u16 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_u32() {
    let value: u32 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: u32 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_u64() {
    let value: u64 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: u64 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_f64() {
    let value: f64 = 3.14;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: f64 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_bool() {
    let value: bool = true;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: bool = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_string() {
    let value: String = "Hello, world!".to_string();
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: String = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_byte() {
    let value: u8 = 42;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: u8 = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_char() {
    let value: char = 'a';
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: char = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_array() {
    let value: Vec<i32> = vec![1, 2, 3];
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: Vec<i32> = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_byte_array() {
    let value: Vec<u8> = vec![1, 2, 3];
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: Vec<u8> = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_unit_variant() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    enum UnitVariant {
        A,
        B,
    }
    let value = UnitVariant::A;
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: UnitVariant = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_newtype_struct() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    struct NewtypeStruct(i32);
    let value = NewtypeStruct(42);
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: NewtypeStruct = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_newtype_variant() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    enum NewtypeVariant {
        A(i32),
        B(i32),
    }
    let value = NewtypeVariant::A(42);
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: NewtypeVariant = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_seq() {
    let value: Vec<i32> = vec![1, 2, 3];
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: Vec<i32> = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_tuple() {
    let value: (i32, i32) = (1, 2);
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: (i32, i32) = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_tuple_struct() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    struct TupleStruct(i32, i32);
    let value = TupleStruct(1, 2);
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: TupleStruct = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_tuple_variant() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    enum TupleVariant {
        A(i32, i32),
        B(i32, i32),
    }
    let value = TupleVariant::A(1, 2);
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: TupleVariant = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_map() {
    use std::collections::HashMap;
    let mut value = HashMap::new();
    value.insert("a".to_string(), 1);
    value.insert("b".to_string(), 2);
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: HashMap<String, i32> = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_struct() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    struct Struct {
        a: i32,
        b: i32,
    }
    let value = Struct { a: 1, b: 2 };
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: Struct = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_struct_variant() {
    #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
    enum StructVariant {
        A { a: i32, b: i32 },
        B { a: i32, b: i32 },
    }
    let value = StructVariant::A { a: 1, b: 2 };
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: StructVariant = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_object_path() {
    let value: ObjectPath = ObjectPath::try_from("/org/freedesktop/DBus").unwrap();
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: ObjectPath = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
fn serde_signature() {
    let value: Signature<'_> = Signature::try_from("a{sv}").unwrap();
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: Signature<'_> = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
#[cfg(all(feature = "gvariant", not(feature = "option-as-array")))]
fn test_maybe() {
    let value: Option<i32> = Some(42);
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: Option<i32> = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}

#[test]
#[cfg(all(feature = "option-as-array", not(feature = "gvariant")))]
fn test_maybe() {
    let value: Option<i32> = Some(42);
    let serialized: OwnedValue = zvariant::to_value(&value).unwrap();
    let deserialized: Option<i32> = zvariant::from_value(serialized).unwrap();
    assert_eq!(value, deserialized);
}
