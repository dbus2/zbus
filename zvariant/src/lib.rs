mod array;
pub use array::*;

mod basic;
pub use basic::*;

mod dict;
pub use dict::*;

mod encoding_context;
pub use encoding_context::*;

mod object_path;
pub use crate::object_path::*;

mod ser;
pub use ser::*;

mod de;
pub use de::*;

mod signature;
pub use crate::signature::*;

mod structure;
pub use crate::structure::*;

mod value;
pub use value::*;

mod error;
pub use error::*;

mod r#type;
pub use r#type::*;

mod from_value;
pub use from_value::*;

mod into_value;
pub use into_value::*;

mod utils;
pub use utils::*;

mod signature_parser;

// TODO: Tests for all *serde* types and import all existing ones from zvariant.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::{TryFrom, TryInto};

    use arrayvec::ArrayVec;
    use byteorder::{self, ByteOrder, BE, LE};

    use zvariant_derive::Type;

    use crate::{from_slice, from_slice_for_signature};
    use crate::{to_bytes, to_bytes_for_signature};

    use crate::{Array, Dict, EncodingContext as Context};
    use crate::{FromValue, IntoValue, Type, Value};
    use crate::{ObjectPath, Signature};

    // Test through both generic and specific API (wrt byte order)
    macro_rules! basic_type_test {
        ($trait:ty, $test_value:expr, $expected_len:expr, $expected_ty:ty) => {{
            let ctxt = Context::<$trait>::new_dbus(0);
            let encoded = to_bytes(ctxt, &$test_value).unwrap();
            assert_eq!(
                encoded.len(),
                $expected_len,
                "invalid encoding using `to_bytes`"
            );
            let decoded: $expected_ty = from_slice(&encoded, ctxt).unwrap();
            assert!(
                decoded == $test_value,
                "invalid decoding using `from_slice`"
            );

            encoded
        }};
    }

    #[test]
    fn u8_value() {
        basic_type_test!(LE, 77_u8, 1, u8);

        // As Value
        let v = 77_u8.into_value();
        assert_eq!(v.value_signature(), "y");
        assert_eq!(v, Value::U8(77));
        basic_type_test!(LE, v, 4, Value);
    }

    #[test]
    fn u16_value() {
        basic_type_test!(BE, 0xABBA_u16, 2, u16);

        // As Value
        let v = 0xFEFE_u16.into_value();
        assert_eq!(v.value_signature(), "q");
        assert_eq!(v, Value::U16(0xFEFE));
        basic_type_test!(LE, v, 6, Value);
    }

    #[test]
    fn i16_value() {
        let encoded = basic_type_test!(BE, -0xAB0_i16, 2, i16);
        assert_eq!(LE::read_i16(&encoded), 0x50F5_i16);

        // As Value
        let v = 0xAB_i16.into_value();
        assert_eq!(v.value_signature(), "n");
        assert_eq!(v, Value::I16(0xAB));
        basic_type_test!(LE, v, 6, Value);
    }

    #[test]
    fn u32_value() {
        basic_type_test!(BE, 0xABBA_ABBA_u32, 4, u32);

        // As Value
        let v = 0xABBA_ABBA_u32.into_value();
        assert_eq!(v.value_signature(), "u");
        assert_eq!(v, Value::U32(0xABBA_ABBA));
        basic_type_test!(LE, v, 8, Value);
    }

    #[test]
    fn i32_value() {
        let encoded = basic_type_test!(BE, -0xABBA_AB0_i32, 4, i32);
        assert_eq!(LE::read_i32(&encoded), 0x5055_44F5_i32);

        // As Value
        let v = 0xABBA_AB0_i32.into_value();
        assert_eq!(v.value_signature(), "i");
        assert_eq!(v, Value::I32(0xABBA_AB0));
        basic_type_test!(LE, v, 8, Value);
    }

    // u64 is covered by `value_value` test below

    #[test]
    fn i64_value() {
        let encoded = basic_type_test!(BE, -0xABBA_ABBA_ABBA_AB0_i64, 8, i64);
        assert_eq!(LE::read_i64(&encoded), 0x5055_4455_4455_44F5_i64);

        // As Value
        let v = 0xABBA_AB0i64.into_value();
        assert_eq!(v.value_signature(), "x");
        assert_eq!(v, Value::I64(0xABBA_AB0));
        basic_type_test!(LE, v, 16, Value);
    }

    #[test]
    fn f64_value() {
        let encoded = basic_type_test!(BE, 99999.99999_f64, 8, f64);
        assert_eq!(LE::read_f64(&encoded), -5759340900185448e-143);

        // As Value
        let v = 99999.99999_f64.into_value();
        assert_eq!(v.value_signature(), "d");
        assert_eq!(v, Value::F64(99999.99999));
        basic_type_test!(LE, v, 16, Value);
    }

    #[test]
    fn str_value() {
        let string = String::from("hello world");
        basic_type_test!(LE, string, 16, String);
        basic_type_test!(LE, string, 16, &str);

        let string = "hello world";
        basic_type_test!(LE, string, 16, &str);
        basic_type_test!(LE, string, 16, String);

        // As Value
        let v = string.into_value();
        assert_eq!(v.value_signature(), "s");
        assert_eq!(v, Value::Str("hello world"));
        basic_type_test!(LE, v, 20, Value);

        // Characters are treated as strings
        basic_type_test!(LE, 'c', 6, char);

        // As Value
        let v = "c".into_value();
        assert_eq!(v.value_signature(), "s");
        let ctxt = Context::new_dbus(0);
        let encoded = to_bytes::<LE, _>(ctxt, &v).unwrap();
        assert_eq!(encoded.len(), 10);
        let v = from_slice::<LE, Value>(&encoded, ctxt).unwrap();
        assert_eq!(v, Value::Str("c"));
    }

    #[test]
    fn signature_value() {
        let sig = Signature::try_from("yys").unwrap();
        basic_type_test!(LE, sig, 5, Signature);

        // As Value
        let v = sig.into_value();
        assert_eq!(v.value_signature(), "g");
        let encoded = basic_type_test!(LE, v, 8, Value);
        let ctxt = Context::new_dbus(0);
        let v = from_slice::<LE, Value>(&encoded, ctxt).unwrap();
        assert_eq!(v, Value::Signature(Signature::try_from("yys").unwrap()));
    }

    #[test]
    fn object_path_value() {
        let o = ObjectPath::try_from("/hello/world").unwrap();
        basic_type_test!(LE, o, 17, ObjectPath);

        // As Value
        let v = o.into_value();
        assert_eq!(v.value_signature(), "o");
        let encoded = basic_type_test!(LE, v, 21, Value);
        let ctxt = Context::new_dbus(0);
        let v = from_slice::<LE, Value>(&encoded, ctxt).unwrap();
        assert_eq!(
            v,
            Value::ObjectPath(ObjectPath::try_from("/hello/world").unwrap())
        );
    }

    #[test]
    fn unit() {
        basic_type_test!(BE, (), 0, ());
    }

    #[test]
    fn array_value() {
        // Let's use D-Bus/GVariant terms

        //
        // Array of u8
        //
        let ay = ArrayVec::from([77u8, 88]);
        let ctxt = Context::<LE>::new_dbus(0);
        let encoded = to_bytes(ctxt, &ay[..]).unwrap();
        assert_eq!(encoded.len(), 6);
        let decoded: ArrayVec<[u8; 2]> = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(&decoded.as_slice(), &[77u8, 88]);

        // As Value
        let v = &ay.into_value();
        assert_eq!(v.value_signature(), "ay");
        let encoded = to_bytes::<LE, _>(ctxt, v).unwrap();
        assert_eq!(encoded.len(), 10);
        let v = from_slice::<LE, Value>(&encoded, ctxt).unwrap();
        if let Value::Array(array) = v {
            assert_eq!(*array.element_signature(), "y");
            assert_eq!(array.len(), 2);
            assert_eq!(array.get()[0], Value::U8(77));
            assert_eq!(array.get()[1], Value::U8(88));
        } else {
            panic!();
        }

        // Now try as Vec
        let vec = ay.to_vec();
        let encoded = to_bytes::<LE, _>(ctxt, &vec).unwrap();
        assert_eq!(encoded.len(), 6);

        // Vec as Value
        let v = Array::from(&vec).into_value();
        assert_eq!(v.value_signature(), "ay");
        let encoded = to_bytes::<LE, _>(ctxt, &v).unwrap();
        assert_eq!(encoded.len(), 10);

        // Emtpy array
        let at = ArrayVec::<[u64; 0]>::new();
        let encoded = to_bytes::<LE, _>(ctxt, &at).unwrap();
        assert_eq!(encoded.len(), 8);

        // As Value
        let v = &at.into_value();
        assert_eq!(v.value_signature(), "at");
        let encoded = to_bytes::<LE, _>(ctxt, v).unwrap();
        assert_eq!(encoded.len(), 8);
        let v = from_slice::<LE, Value>(&encoded, ctxt).unwrap();
        if let Value::Array(array) = v {
            assert_eq!(*array.element_signature(), "t");
            assert_eq!(array.len(), 0);
        } else {
            panic!();
        }

        //
        // Array of strings
        //
        // Can't use 'as' as it's a keyword
        let as_ = ArrayVec::from(["Hello", "World", "Now", "Bye!"]);
        let encoded = to_bytes::<LE, _>(ctxt, &as_).unwrap();
        assert_eq!(encoded.len(), 45);
        let decoded = from_slice::<LE, ArrayVec<[&str; 4]>>(&encoded, ctxt).unwrap();
        assert_eq!(decoded.len(), 4);
        assert_eq!(decoded[0], "Hello");
        assert_eq!(decoded[1], "World");

        let decoded = from_slice::<LE, ArrayVec<[String; 4]>>(&encoded, ctxt).unwrap();
        assert_eq!(decoded.as_slice(), as_.as_slice());

        // Decode just the second string
        let ctxt = Context::<LE>::new_dbus(14);
        let decoded: &str = from_slice(&encoded[14..], ctxt).unwrap();
        assert_eq!(decoded, "World");
        let ctxt = Context::<LE>::new_dbus(0);

        // As Value
        let v = &as_.into_value();
        assert_eq!(v.value_signature(), "as");
        let encoded = to_bytes(ctxt, v).unwrap();
        assert_eq!(encoded.len(), 49);
        let v = from_slice(&encoded, ctxt).unwrap();
        if let Value::Array(array) = v {
            assert_eq!(*array.element_signature(), "s");
            assert_eq!(array.len(), 4);
            assert_eq!(array.get()[0], Value::Str("Hello"));
            assert_eq!(array.get()[1], Value::Str("World"));
        } else {
            panic!();
        }

        // Array of Struct, which in turn containin an Array (We gotta go deeper!)
        // Signature: "a(yu(xbxas)s)");
        let ar = ArrayVec::from([(
            // top-most simple fields
            u8::max_value(),
            u32::max_value(),
            (
                // 2nd level simple fields
                i64::max_value(),
                true,
                i64::max_value(),
                // 2nd level array field
                &["Hello", "World"][..],
            ),
            // one more top-most simple field
            "hello",
        )]);
        let encoded = to_bytes(ctxt, &ar).unwrap();
        assert_eq!(encoded.len(), 78);
        let decoded = from_slice::<
            LE,
            ArrayVec<[(u8, u32, (i64, bool, i64, ArrayVec<[&str; 2]>), &str); 1]>,
        >(&encoded, ctxt)
        .unwrap();
        assert_eq!(decoded.len(), 1);
        let r = &decoded[0];
        assert_eq!(r.0, u8::max_value());
        assert_eq!(r.1, u32::max_value());
        let inner_r = &r.2;
        assert_eq!(inner_r.0, i64::max_value());
        assert_eq!(inner_r.1, true);
        assert_eq!(inner_r.2, i64::max_value());
        let as_ = &inner_r.3;
        assert_eq!(as_.len(), 2);
        assert_eq!(as_[0], "Hello");
        assert_eq!(as_[1], "World");
        assert_eq!(r.3, "hello");

        // As Value
        let v = &ar[..].into_value();
        assert_eq!(v.value_signature(), "a(yu(xbxas)s)");
        let encoded = to_bytes::<LE, _>(ctxt, v).unwrap();
        assert_eq!(encoded.len(), 94);
        let v = from_slice::<LE, Value>(&encoded, ctxt).unwrap();
        if let Value::Array(array) = v {
            assert_eq!(*array.element_signature(), "(yu(xbxas)s)");
            assert_eq!(array.len(), 1);
            let r = &array.get()[0];
            if let Value::Structure(r) = r {
                let fields = r.fields();
                assert_eq!(fields[0], Value::U8(u8::max_value()));
                assert_eq!(fields[1], Value::U32(u32::max_value()));
                if let Value::Structure(r) = &fields[2] {
                    let fields = r.fields();
                    assert_eq!(fields[0], Value::I64(i64::max_value()));
                    assert_eq!(fields[1], Value::Bool(true));
                    assert_eq!(fields[2], Value::I64(i64::max_value()));
                    if let Value::Array(as_) = &fields[3] {
                        assert_eq!(as_.len(), 2);
                        assert_eq!(as_.get()[0], Value::Str("Hello"));
                        assert_eq!(as_.get()[1], Value::Str("World"));
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
                assert_eq!(fields[3], Value::Str("hello"));
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn dict_value() {
        let mut map: HashMap<i64, &str> = HashMap::new();
        map.insert(1, "123");
        map.insert(2, "456");
        let ctxt = Context::<LE>::new_dbus(0);
        let encoded = to_bytes(ctxt, &map).unwrap();
        assert_eq!(dbg!(encoded.len()), 40);
        let decoded: HashMap<i64, &str> = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(decoded[&1], "123");
        assert_eq!(decoded[&2], "456");

        // As Value
        let v = Dict::from(map).into_value();
        assert_eq!(v.value_signature(), "a{xs}");
        let encoded = to_bytes(ctxt, &v).unwrap();
        assert_eq!(encoded.len(), 48);
        // Convert it back
        let dict = Dict::from_value(v).unwrap();
        let map: HashMap<i64, &str> = HashMap::try_from(dict).unwrap();
        assert_eq!(map[&1], "123");
        assert_eq!(map[&2], "456");
        // Also decode it back
        let v = from_slice(&encoded, ctxt).unwrap();
        if let Value::Dict(dict) = v {
            assert_eq!(dict.get::<i64, &str>(&1).unwrap().unwrap(), &"123");
            assert_eq!(dict.get::<i64, &str>(&2).unwrap().unwrap(), &"456");
        } else {
            panic!();
        }

        // Now a hand-crafted Dict Value but with a Value as value
        let mut dict = Dict::new(<&str>::signature(), Value::signature());
        dict.add("hello", "there".into_value()).unwrap();
        dict.add("bye", "now".into_value()).unwrap();
        let v = dict.into_value();
        assert_eq!(v.value_signature(), "a{sv}");
        let encoded = to_bytes(ctxt, &v).unwrap();
        assert_eq!(dbg!(encoded.len()), 68);
        let v = from_slice(&encoded, ctxt).unwrap();
        if let Value::Dict(dict) = v {
            assert_eq!(
                *dict.get::<_, Value>(&"hello").unwrap().unwrap(),
                Value::Str("there")
            );
            assert_eq!(
                *dict.get::<_, Value>(&"bye").unwrap().unwrap(),
                Value::Str("now")
            );
        } else {
            panic!();
        }
    }

    #[test]
    fn value_value() {
        let ctxt = Context::<BE>::new_dbus(0);
        let encoded = to_bytes(ctxt, &0xABBA_ABBA_ABBA_ABBA_u64).unwrap();
        assert_eq!(encoded.len(), 8);
        assert_eq!(LE::read_u64(&encoded), 0xBAAB_BAAB_BAAB_BAAB_u64);
        let decoded: u64 = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(decoded, 0xABBA_ABBA_ABBA_ABBA);

        // Lie about there being bytes before
        let ctxt = Context::<LE>::new_dbus(2);
        let encoded = to_bytes(ctxt, &0xABBA_ABBA_ABBA_ABBA_u64).unwrap();
        assert_eq!(encoded.len(), 14);
        let decoded: u64 = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(decoded, 0xABBA_ABBA_ABBA_ABBA_u64);
        let ctxt = Context::<LE>::new_dbus(0);

        // As Value
        let v = 0xFEFE_u64.into_value();
        assert_eq!(v.value_signature(), "t");
        let encoded = to_bytes(ctxt, &v).unwrap();
        assert_eq!(encoded.len(), 16);
        let v = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(v, Value::U64(0xFEFE));

        // And now as Value in a Value
        let v = Value::Value(Box::new(v));
        let encoded = to_bytes(ctxt, &v).unwrap();
        assert_eq!(encoded.len(), 16);
        let v = from_slice(&encoded, ctxt).unwrap();
        if let Value::Value(v) = v {
            assert_eq!(v.value_signature(), "t");
            assert_eq!(*v, Value::U64(0xFEFE));
        } else {
            panic!();
        }

        // Ensure Value works with other Serializer & Deserializer
        let v = 0xFEFE_u64.into_value();
        let encoded = serde_json::to_string(&v).unwrap();
        let v = serde_json::from_str::<Value>(&encoded).unwrap();
        assert_eq!(v, Value::U64(0xFEFE));
    }

    #[test]
    fn enums() {
        // TODO: Document enum handling.
        //
        // 1. `Value`.
        // 2. custom (de)serialize impl.
        // 3. to/from_*_for_signature()
        use serde::{Deserialize, Serialize};

        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        enum Test {
            Unit,
            NewType(u8),
            Tuple(u8, u64),
            Struct { y: u8, t: u64 },
        }

        let ctxt = Context::<BE>::new_dbus(0);
        let signature = "u".try_into().unwrap();
        let encoded = to_bytes_for_signature(ctxt, &signature, &Test::Unit).unwrap();
        assert_eq!(encoded.len(), 4);
        let decoded: Test = from_slice_for_signature(&encoded, ctxt, &signature).unwrap();
        assert_eq!(decoded, Test::Unit);

        let signature = "y".try_into().unwrap();
        let encoded = to_bytes_for_signature(ctxt, &signature, &Test::NewType(42)).unwrap();
        assert_eq!(encoded.len(), 5);
        let decoded: Test = from_slice_for_signature(&encoded, ctxt, &signature).unwrap();
        assert_eq!(decoded, Test::NewType(42));

        // TODO: Provide convenience API to create complex signatures
        let signature = "(yt)".try_into().unwrap();
        let encoded = to_bytes_for_signature(ctxt, &signature, &Test::Tuple(42, 42)).unwrap();
        assert_eq!(encoded.len(), 24);
        let decoded: Test = from_slice_for_signature(&encoded, ctxt, &signature).unwrap();
        assert_eq!(decoded, Test::Tuple(42, 42));

        let s = Test::Struct { y: 42, t: 42 };
        let encoded = to_bytes_for_signature(ctxt, &signature, &s).unwrap();
        assert_eq!(encoded.len(), 24);
        let decoded: Test = from_slice_for_signature(&encoded, ctxt, &signature).unwrap();
        assert_eq!(decoded, Test::Struct { y: 42, t: 42 });
    }

    #[test]
    fn derive() {
        use crate as zvariant;

        use serde::{Deserialize, Serialize};
        use serde_repr::{Deserialize_repr, Serialize_repr};

        #[derive(Deserialize, Serialize, Type)]
        struct Struct<'s> {
            field1: u16,
            field2: i64,
            field3: &'s str,
        }

        assert_eq!(Struct::signature(), "(qxs)");
        let s = Struct {
            field1: 0xFF_FF,
            field2: 0xFF_FF_FF_FF_FF_FF,
            field3: "hello",
        };
        let ctxt = Context::<LE>::new_dbus(0);
        let encoded = to_bytes(ctxt, &s).unwrap();
        assert_eq!(encoded.len(), 26);
        let decoded: Struct = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(decoded.field1, 0xFF_FF);
        assert_eq!(decoded.field2, 0xFF_FF_FF_FF_FF_FF);
        assert_eq!(decoded.field3, "hello");

        #[derive(Deserialize, Serialize, Type)]
        struct UnitStruct;

        assert_eq!(UnitStruct::signature(), <()>::signature());
        let encoded = to_bytes(ctxt, &UnitStruct).unwrap();
        assert_eq!(encoded.len(), 0);
        let _: UnitStruct = from_slice(&encoded, ctxt).unwrap();

        #[repr(u8)]
        #[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
        enum Enum {
            Variant1,
            Variant2,
            Variant3,
        }

        assert_eq!(Enum::signature(), u8::signature());
        let encoded = to_bytes(ctxt, &Enum::Variant3).unwrap();
        assert_eq!(encoded.len(), 1);
        let decoded: Enum = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(decoded, Enum::Variant3);

        #[repr(i64)]
        #[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
        enum Enum2 {
            Variant1,
            Variant2,
            Variant3,
        }

        assert_eq!(Enum2::signature(), i64::signature());
        let encoded = to_bytes(ctxt, &Enum2::Variant2).unwrap();
        assert_eq!(encoded.len(), 8);
        let decoded: Enum2 = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(decoded, Enum2::Variant2);

        #[derive(Deserialize, Serialize, Type, Debug, PartialEq)]
        enum NoReprEnum {
            Variant1,
            Variant2,
            Variant3,
        }

        assert_eq!(NoReprEnum::signature(), u32::signature());
        let encoded = to_bytes(ctxt, &NoReprEnum::Variant2).unwrap();
        assert_eq!(encoded.len(), 4);
        let decoded: NoReprEnum = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(decoded, NoReprEnum::Variant2);
    }
}
