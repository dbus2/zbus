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

mod serializer;
pub use serializer::*;

mod deserializer;
pub use deserializer::*;

mod signature;
pub use crate::signature::*;

mod structure;
pub use crate::structure::*;

mod variant;
pub use variant::*;

mod error;
pub use error::*;

mod variant_value;
pub use variant_value::*;

mod from_variant;
pub use from_variant::*;

mod into_variant;
pub use into_variant::*;

mod utils;
pub use utils::*;

mod signature_parser;

// TODO: Tests for all *serde* types and import all existing ones from zvariant.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use byteorder::{self, ByteOrder, BE, LE};

    use crate::{from_slice, from_slice_for_signature};
    use crate::{to_bytes, to_bytes_for_signature};

    use crate::{Array, Dict, EncodingContext as Context, EncodingFormat as Format};
    use crate::{FromVariant, IntoVariant, Variant, VariantValue};
    use crate::{ObjectPath, Signature};

    // Test through both generic and specific API (wrt byte order)
    macro_rules! basic_type_test {
        ($trait:ty, $test_value:expr, $expected_len:expr, $expected_ty:ty) => {{
            let ctxt = Context::<$trait>::new(Format::DBus, 0);
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
    fn u8_variant() {
        basic_type_test!(LE, 77_u8, 1, u8);

        // As Variant
        let v = 77_u8.into_variant();
        assert!(v.value_signature() == "y");
        assert!(v == Variant::U8(77));
        basic_type_test!(LE, v, 4, Variant);
    }

    #[test]
    fn u16_variant() {
        basic_type_test!(BE, 0xABBA_u16, 2, u16);

        // As Variant
        let v = 0xFEFE_u16.into_variant();
        assert!(v.value_signature() == "q");
        assert!(v == Variant::U16(0xFEFE));
        basic_type_test!(LE, v, 6, Variant);
    }

    #[test]
    fn i16_variant() {
        let encoded = basic_type_test!(BE, -0xAB0_i16, 2, i16);
        assert!(LE::read_i16(&encoded) == 0x50F5_i16);

        // As Variant
        let v = 0xAB_i16.into_variant();
        assert!(v.value_signature() == "n");
        assert!(v == Variant::I16(0xAB));
        basic_type_test!(LE, v, 6, Variant);
    }

    #[test]
    fn u32_variant() {
        basic_type_test!(BE, 0xABBA_ABBA_u32, 4, u32);

        // As Variant
        let v = 0xABBA_ABBA_u32.into_variant();
        assert!(v.value_signature() == "u");
        assert!(v == Variant::U32(0xABBA_ABBA));
        basic_type_test!(LE, v, 8, Variant);
    }

    #[test]
    fn i32_variant() {
        let encoded = basic_type_test!(BE, -0xABBA_AB0_i32, 4, i32);
        assert!(LE::read_i32(&encoded) == 0x5055_44F5_i32);

        // As Variant
        let v = 0xABBA_AB0_i32.into_variant();
        assert!(v.value_signature() == "i");
        assert!(v == Variant::I32(0xABBA_AB0));
        basic_type_test!(LE, v, 8, Variant);
    }

    // u64 is covered by `variant_variant` test below

    #[test]
    fn i64_variant() {
        let encoded = basic_type_test!(BE, -0xABBA_ABBA_ABBA_AB0_i64, 8, i64);
        assert!(LE::read_i64(&encoded) == 0x5055_4455_4455_44F5_i64);

        // As Variant
        let v = 0xABBA_AB0i64.into_variant();
        assert!(v.value_signature() == "x");
        assert!(v == Variant::I64(0xABBA_AB0));
        basic_type_test!(LE, v, 16, Variant);
    }

    #[test]
    fn f64_variant() {
        let encoded = basic_type_test!(BE, 99999.99999_f64, 8, f64);
        assert!(LE::read_f64(&encoded) == -5759340900185448e-143);

        // As Variant
        let v = 99999.99999_f64.into_variant();
        assert!(v.value_signature() == "d");
        assert!(v == Variant::F64(99999.99999));
        basic_type_test!(LE, v, 16, Variant);
    }

    #[test]
    fn str_variant() {
        let string = "hello world";
        basic_type_test!(LE, string, 16, &str);

        // As Variant
        let v = string.into_variant();
        assert!(v.value_signature() == "s");
        assert!(v == Variant::Str("hello world"));
        basic_type_test!(LE, v, 20, Variant);

        // Characters are treated as strings
        basic_type_test!(LE, 'c', 6, char);

        // As Variant
        let v = 'c'.into_variant();
        assert!(v.value_signature() == "s");
        let ctxt = Context::new(Format::DBus, 0);
        let encoded = to_bytes::<LE, _>(ctxt, &v).unwrap();
        assert!(encoded.len() == 10);
        let v = from_slice::<LE, Variant>(&encoded, ctxt).unwrap();
        assert!(v == Variant::Str("c"));
    }

    #[test]
    fn signature_variant() {
        let sig = Signature::from("yys");
        basic_type_test!(LE, sig, 5, Signature);

        // As Variant
        let v = sig.into_variant();
        assert!(v.value_signature() == "g");
        let encoded = basic_type_test!(LE, v, 8, Variant);
        let ctxt = Context::new(Format::DBus, 0);
        let v = from_slice::<LE, Variant>(&encoded, ctxt).unwrap();
        assert!(v == Variant::Signature(Signature::from("yys")));
    }

    #[test]
    fn object_path_variant() {
        let o = ObjectPath::from("/hello/world");
        basic_type_test!(LE, o, 17, ObjectPath);

        // As Variant
        let v = o.into_variant();
        assert!(v.value_signature() == "o");
        let encoded = basic_type_test!(LE, v, 21, Variant);
        let ctxt = Context::new(Format::DBus, 0);
        let v = from_slice::<LE, Variant>(&encoded, ctxt).unwrap();
        assert!(v == Variant::ObjectPath(ObjectPath::from("/hello/world")));
    }

    #[test]
    fn unit() {
        basic_type_test!(BE, (), 0, ());
    }

    #[test]
    fn array_variant() {
        // Let's use D-Bus/GVariant terms

        //
        // Array of u8
        //
        let ay = [77u8, 88];
        // Array itself is treated like a tuple by serde & that translates to a structure in our
        // case so gotta make it a slice for serde to treat it as seq type.
        let ctxt = Context::<LE>::new(Format::DBus, 0);
        let encoded = to_bytes(ctxt, &ay[..]).unwrap();
        assert!(encoded.len() == 6);
        // FIXME: We shouldn't need to use a Vec here but we have to. Maybe array can still be
        // serialized and deserialized as D-Bus array?
        let decoded: Vec<u8> = from_slice(&encoded, ctxt).unwrap();
        assert!(decoded == &[77u8, 88]);

        // As Variant
        let v = &ay[..].into_variant();
        assert!(v.value_signature() == "ay");
        let encoded = to_bytes::<LE, _>(ctxt, v).unwrap();
        assert!(encoded.len() == 10);
        let v = from_slice::<LE, Variant>(&encoded, ctxt).unwrap();
        if let Variant::Array(array) = v {
            assert!(*array.element_signature() == "y");
            assert!(array.len() == 2);
            assert!(array.get()[0] == Variant::U8(77));
            assert!(array.get()[1] == Variant::U8(88));
        } else {
            panic!();
        }

        // Now try as Vec
        let vec = ay.to_vec();
        let encoded = to_bytes::<LE, _>(ctxt, &vec).unwrap();
        assert!(encoded.len() == 6);

        // Vec as Variant
        let v = Array::from(&vec).into_variant();
        assert!(v.value_signature() == "ay");
        let encoded = to_bytes::<LE, _>(ctxt, &v).unwrap();
        assert!(encoded.len() == 10);

        // Emtpy array
        let at: [u64; 0] = [];
        let encoded = to_bytes::<LE, _>(ctxt, &at[..]).unwrap();
        assert!(encoded.len() == 8);

        // As Variant
        let v = &at[..].into_variant();
        assert!(v.value_signature() == "at");
        let encoded = to_bytes::<LE, _>(ctxt, v).unwrap();
        assert!(encoded.len() == 8);
        let v = from_slice::<LE, Variant>(&encoded, ctxt).unwrap();
        if let Variant::Array(array) = v {
            assert!(*array.element_signature() == "t");
            assert!(array.len() == 0);
        } else {
            panic!();
        }

        //
        // Array of strings
        //
        // Can't use 'as' as it's a keyword
        let as_ = ["Hello", "World", "Now", "Bye!"];
        let encoded = to_bytes::<LE, _>(ctxt, &as_[..]).unwrap();
        assert!(encoded.len() == 45);
        let decoded = from_slice::<LE, Vec<&str>>(&encoded, ctxt).unwrap();
        assert!(decoded.len() == 4);
        assert!(decoded[0] == "Hello");
        assert!(decoded[1] == "World");

        // Decode just the second string
        let ctxt = Context::<LE>::new(Format::DBus, 14);
        let decoded: &str = from_slice(&encoded[14..], ctxt).unwrap();
        assert!(decoded == "World");
        let ctxt = Context::<LE>::new(Format::DBus, 0);

        // As Variant
        let v = &as_[..].into_variant();
        assert!(v.value_signature() == "as");
        let encoded = to_bytes(ctxt, v).unwrap();
        assert!(encoded.len() == 49);
        let v = from_slice(&encoded, ctxt).unwrap();
        if let Variant::Array(array) = v {
            assert!(*array.element_signature() == "s");
            assert!(array.len() == 4);
            assert!(array.get()[0] == Variant::Str("Hello"));
            assert!(array.get()[1] == Variant::Str("World"));
        } else {
            panic!();
        }

        // Array of Struct, which in turn containin an Array (We gotta go deeper!)
        // Signature: "a(yu(xbxas)s)");
        let ar = [(
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
        )];
        let encoded = to_bytes(ctxt, &ar[..]).unwrap();
        assert!(encoded.len() == 78);
        let decoded =
            from_slice::<LE, Vec<(u8, u32, (i64, bool, i64, Vec<&str>), &str)>>(&encoded, ctxt)
                .unwrap();
        assert!(decoded.len() == 1);
        let r = &decoded[0];
        assert!(r.0 == u8::max_value());
        assert!(r.1 == u32::max_value());
        let inner_r = &r.2;
        assert!(inner_r.0 == i64::max_value());
        assert!(inner_r.1 == true);
        assert!(inner_r.2 == i64::max_value());
        let as_ = &inner_r.3;
        assert!(as_.len() == 2);
        assert!(as_[0] == "Hello");
        assert!(as_[1] == "World");
        assert!(r.3 == "hello");

        // As Variant
        let v = &ar[..].into_variant();
        assert!(v.value_signature() == "a(yu(xbxas)s)");
        let encoded = to_bytes::<LE, _>(ctxt, v).unwrap();
        assert!(encoded.len() == 94);
        let v = from_slice::<LE, Variant>(&encoded, ctxt).unwrap();
        if let Variant::Array(array) = v {
            assert!(*array.element_signature() == "(yu(xbxas)s)");
            assert!(array.len() == 1);
            let r = &array.get()[0];
            if let Variant::Structure(r) = r {
                let fields = r.fields();
                assert!(fields[0] == Variant::U8(u8::max_value()));
                assert!(fields[1] == Variant::U32(u32::max_value()));
                if let Variant::Structure(r) = &fields[2] {
                    let fields = r.fields();
                    assert!(fields[0] == Variant::I64(i64::max_value()));
                    assert!(fields[1] == Variant::Bool(true));
                    assert!(fields[2] == Variant::I64(i64::max_value()));
                    if let Variant::Array(as_) = &fields[3] {
                        assert!(as_.len() == 2);
                        assert!(as_.get()[0] == Variant::Str("Hello"));
                        assert!(as_.get()[1] == Variant::Str("World"));
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
                assert!(fields[3] == Variant::Str("hello"));
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn dict_variant() {
        let mut map: HashMap<i64, &str> = HashMap::new();
        map.insert(1, "123");
        map.insert(2, "456");
        let ctxt = Context::<LE>::new(Format::DBus, 0);
        let encoded = to_bytes(ctxt, &map).unwrap();
        assert!(dbg!(encoded.len()) == 40);
        let decoded: HashMap<i64, &str> = from_slice(&encoded, ctxt).unwrap();
        assert!(decoded[&1] == "123");
        assert!(decoded[&2] == "456");

        // As Variant
        let v = Dict::from(map).into_variant();
        assert!(v.value_signature() == "a{xs}");
        let encoded = to_bytes(ctxt, &v).unwrap();
        assert!(encoded.len() == 48);
        // Convert it back
        let dict = Dict::from_variant(v).unwrap();
        let map: HashMap<i64, &str> = HashMap::try_from(dict).unwrap();
        assert!(map[&1] == "123");
        assert!(map[&2] == "456");
        // Also decode it back
        let v = from_slice(&encoded, ctxt).unwrap();
        if let Variant::Dict(dict) = v {
            assert!(dict.get::<i64, &str>(&1).unwrap().unwrap() == &"123");
            assert!(dict.get::<i64, &str>(&2).unwrap().unwrap() == &"456");
        } else {
            panic!();
        }

        // Now a hand-crafted Dict Variant but with a Variant as value
        let mut dict = Dict::new(<&str>::signature(), Variant::signature());
        dict.add("hello", "there".into_variant()).unwrap();
        dict.add("bye", "now".into_variant()).unwrap();
        let v = dict.into_variant();
        assert!(v.value_signature() == "a{sv}");
        let encoded = to_bytes(ctxt, &v).unwrap();
        assert!(dbg!(encoded.len()) == 68);
        let v = from_slice(&encoded, ctxt).unwrap();
        if let Variant::Dict(dict) = v {
            assert!(*dict.get::<_, Variant>(&"hello").unwrap().unwrap() == Variant::Str("there"));
            assert!(*dict.get::<_, Variant>(&"bye").unwrap().unwrap() == Variant::Str("now"));
        } else {
            panic!();
        }
    }

    #[test]
    fn variant_variant() {
        let ctxt = Context::<BE>::new(Format::DBus, 0);
        let encoded = to_bytes(ctxt, &0xABBA_ABBA_ABBA_ABBA_u64).unwrap();
        assert!(encoded.len() == 8);
        assert!(LE::read_u64(&encoded) == 0xBAAB_BAAB_BAAB_BAAB_u64);
        let decoded: u64 = from_slice(&encoded, ctxt).unwrap();
        assert!(decoded == 0xABBA_ABBA_ABBA_ABBA);

        // Lie about there being bytes before
        let ctxt = Context::<LE>::new(Format::DBus, 2);
        let encoded = to_bytes(ctxt, &0xABBA_ABBA_ABBA_ABBA_u64).unwrap();
        assert!(encoded.len() == 14);
        let decoded: u64 = from_slice(&encoded, ctxt).unwrap();
        assert_eq!(decoded, 0xABBA_ABBA_ABBA_ABBA_u64);
        let ctxt = Context::<LE>::new(Format::DBus, 0);

        // As Variant
        let v = 0xFEFE_u64.into_variant();
        assert!(v.value_signature() == "t");
        let encoded = to_bytes(ctxt, &v).unwrap();
        assert!(encoded.len() == 16);
        let v = from_slice(&encoded, ctxt).unwrap();
        assert!(v == Variant::U64(0xFEFE));

        // And now as Variant in a Variant
        let v = Variant::Variant(Box::new(v));
        let encoded = to_bytes(ctxt, &v).unwrap();
        assert!(encoded.len() == 16);
        let v = from_slice(&encoded, ctxt).unwrap();
        if let Variant::Variant(v) = v {
            assert!(v.value_signature() == "t");
            assert!(*v == Variant::U64(0xFEFE));
        } else {
            panic!();
        }

        // Ensure Variant works with other Serializer & Deserializer
        let v = 0xFEFE_u64.into_variant();
        let encoded = serde_json::to_string(&v).unwrap();
        let v = serde_json::from_str::<Variant>(&encoded).unwrap();
        assert!(v == Variant::U64(0xFEFE));
    }

    #[test]
    fn enums() {
        // TODO: Document enum handling.
        //
        // 1. `Variant`.
        // 2. custom (de)serialize impl.
        // 3. to/from_*_for_signature()
        use serde_derive::{Deserialize, Serialize};

        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        enum Test {
            Unit,
            NewType(u8),
            Tuple(u8, u64),
            Struct { y: u8, t: u64 },
        }

        let ctxt = Context::<BE>::new(Format::DBus, 0);
        let encoded = to_bytes_for_signature(ctxt, "", &Test::Unit).unwrap();
        assert!(encoded.len() == 4);
        let decoded: Test = from_slice_for_signature(&encoded, ctxt, "").unwrap();
        assert!(decoded == Test::Unit);

        let encoded = to_bytes_for_signature(ctxt, "y", &Test::NewType(42)).unwrap();
        assert!(encoded.len() == 5);
        let decoded: Test = from_slice_for_signature(&encoded, ctxt, "y").unwrap();
        assert!(decoded == Test::NewType(42));

        // TODO: Provide convenience API to create complex signatures
        let encoded = to_bytes_for_signature(ctxt, "(yt)", &Test::Tuple(42, 42)).unwrap();
        assert!(encoded.len() == 24);
        let decoded: Test = from_slice_for_signature(&encoded, ctxt, "(yt)").unwrap();
        assert!(decoded == Test::Tuple(42, 42));

        let s = Test::Struct { y: 42, t: 42 };
        let encoded = to_bytes_for_signature(ctxt, "(yt)", &s).unwrap();
        assert!(encoded.len() == 24);
        let decoded: Test = from_slice_for_signature(&encoded, ctxt, "(yt)").unwrap();
        assert!(decoded == Test::Struct { y: 42, t: 42 });
    }
}
