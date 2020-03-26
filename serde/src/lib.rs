mod array;
pub use array::*;

mod basic;
pub use basic::*;

mod dict;
pub use dict::*;

mod encoding_format;
pub use encoding_format::*;

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

mod signature_parser;
mod utils;

// TODO: Tests for all *serde* types and import all existing ones from zvariant.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryFrom;

    use byteorder::{BigEndian as BE, ByteOrder, LittleEndian as LE};

    use crate::{from_slice, to_bytes};
    use crate::{Array, Dict, EncodingFormat as Format};
    use crate::{FromVariant, IntoVariant, Variant, VariantValue};
    use crate::{ObjectPath, Signature};

    #[test]
    fn u8_variant() {
        let encoded = to_bytes::<LE, _>(Format::DBus, &77_u8).unwrap();
        assert!(encoded.len() == 1);
        let decoded = from_slice::<LE, u8>(&encoded, Format::DBus).unwrap();
        assert!(decoded == 77);

        // As Variant
        let v = 77_u8.into_variant();
        assert!(v.value_signature() == "y");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 4);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::U8(77));
    }

    #[test]
    fn u16_variant() {
        let encoded = to_bytes::<BE, _>(Format::DBus, &0xABBA_u16).unwrap();
        assert!(encoded.len() == 2);
        assert!(LE::read_u16(&encoded) == 0xBAAB_u16);
        let decoded = from_slice::<BE, u16>(&encoded, Format::DBus).unwrap();
        assert!(decoded == 0xABBA);

        // As Variant
        let v = 0xFEFE_u16.into_variant();
        assert!(v.value_signature() == "q");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 6);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::U16(0xFEFE));
    }

    #[test]
    fn i16_variant() {
        let encoded = to_bytes::<BE, _>(Format::DBus, &-0xAB0_i16).unwrap();
        assert!(encoded.len() == 2);
        assert!(LE::read_i16(&encoded) == 0x50F5_i16);
        let decoded = from_slice::<BE, i16>(&encoded, Format::DBus).unwrap();
        assert!(decoded == -0xAB0);

        // As Variant
        let v = 0xAB_i16.into_variant();
        assert!(v.value_signature() == "n");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 6);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::I16(0xAB));
    }

    #[test]
    fn u32_variant() {
        let encoded = to_bytes::<BE, _>(Format::DBus, &0xABBA_ABBA_u32).unwrap();
        assert!(encoded.len() == 4);
        assert!(LE::read_u32(&encoded) == 0xBAAB_BAAB_u32);
        let decoded = from_slice::<BE, u32>(&encoded, Format::DBus).unwrap();
        assert!(decoded == 0xABBA_ABBA);

        // As Variant
        let v = 0xABBA_ABBA_u32.into_variant();
        assert!(v.value_signature() == "u");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 8);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::U32(0xABBA_ABBA));
    }

    #[test]
    fn i32_variant() {
        let encoded = to_bytes::<BE, _>(Format::DBus, &-0xABBA_AB0_i32).unwrap();
        assert!(encoded.len() == 4);
        assert!(LE::read_i32(&encoded) == 0x5055_44F5_i32);
        let decoded = from_slice::<BE, i32>(&encoded, Format::DBus).unwrap();
        assert!(decoded == -0xABBA_AB0);

        // As Variant
        let v = 0xABBA_AB0_i32.into_variant();
        assert!(v.value_signature() == "i");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 8);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::I32(0xABBA_AB0));
    }

    // u64 is covered by `variant_variant` test below

    #[test]
    fn i64_variant() {
        let encoded = to_bytes::<BE, _>(Format::DBus, &-0xABBA_ABBA_ABBA_AB0_i64).unwrap();
        assert!(encoded.len() == 8);
        assert!(LE::read_i64(&encoded) == 0x5055_4455_4455_44F5_i64);
        let decoded = from_slice::<BE, i64>(&encoded, Format::DBus).unwrap();
        assert!(decoded == -0xABBA_ABBA_ABBA_AB0);

        // As Variant
        let v = 0xABBA_AB0i64.into_variant();
        assert!(v.value_signature() == "x");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 16);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::I64(0xABBA_AB0));
    }

    #[test]
    fn f64_variant() {
        let encoded = to_bytes::<BE, _>(Format::DBus, &99999.99999_f64).unwrap();
        assert!(encoded.len() == 8);
        assert!(LE::read_f64(&encoded) == -5759340900185448e-143);
        let decoded = from_slice::<BE, f64>(&encoded, Format::DBus).unwrap();
        assert!(decoded == 99999.99999);

        // As Variant
        let v = 99999.99999_f64.into_variant();
        assert!(v.value_signature() == "d");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 16);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::F64(99999.99999));
    }

    #[test]
    fn str_variant() {
        let string = "hello world";
        let encoded = to_bytes::<LE, _>(Format::DBus, &string).unwrap();
        assert!(encoded.len() == 16);
        let decoded = from_slice::<LE, &str>(&encoded, Format::DBus).unwrap();
        assert!(decoded == "hello world");

        // As Variant
        let v = string.into_variant();
        assert!(v.value_signature() == "s");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 20);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::Str("hello world"));
    }

    #[test]
    fn signature_variant() {
        let sig = Signature::from("yys");
        let encoded = to_bytes::<LE, _>(Format::DBus, &sig).unwrap();
        assert!(encoded.len() == 5);
        let decoded = from_slice::<LE, Signature>(&encoded, Format::DBus).unwrap();
        assert!(decoded == Signature::from("yys"));

        // As Variant
        let v = sig.into_variant();
        assert!(v.value_signature() == "g");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 8);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::Signature(Signature::from("yys")));
    }

    #[test]
    fn object_path_variant() {
        let o = ObjectPath::from("/hello/world");
        let encoded = to_bytes::<LE, _>(Format::DBus, &o).unwrap();
        assert!(encoded.len() == 17);
        let decoded = from_slice::<LE, ObjectPath>(&encoded, Format::DBus).unwrap();
        assert!(decoded == ObjectPath::from("/hello/world"));

        // As Variant
        let v = o.into_variant();
        assert!(v.value_signature() == "o");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 21);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        assert!(v == Variant::ObjectPath(ObjectPath::from("/hello/world")));
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
        let encoded = to_bytes::<LE, _>(Format::DBus, &ay[..]).unwrap();
        assert!(encoded.len() == 6);
        // FIXME: We shouldn't need to use a Vec here but we have to. Maybe array can still be
        // serialized and deserialized as D-Bus array?
        let decoded = from_slice::<LE, Vec<u8>>(&encoded, Format::DBus).unwrap();
        assert!(decoded == &[77u8, 88]);

        // As Variant
        let v = &ay[..].into_variant();
        assert!(v.value_signature() == "ay");
        let encoded = to_bytes::<LE, _>(Format::DBus, v).unwrap();
        assert!(encoded.len() == 10);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
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
        let encoded = to_bytes::<LE, _>(Format::DBus, &vec).unwrap();
        assert!(encoded.len() == 6);

        // Vec as Variant
        let v = Array::from(&vec).into_variant();
        assert!(v.value_signature() == "ay");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 10);

        // Emtpy array
        let at: [u64; 0] = [];
        let encoded = to_bytes::<LE, _>(Format::DBus, &at[..]).unwrap();
        assert!(encoded.len() == 8);

        // As Variant
        let v = &at[..].into_variant();
        assert!(v.value_signature() == "at");
        let encoded = to_bytes::<LE, _>(Format::DBus, v).unwrap();
        assert!(encoded.len() == 8);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
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
        let encoded = to_bytes::<LE, _>(Format::DBus, &as_[..]).unwrap();
        assert!(encoded.len() == 45);
        let decoded = from_slice::<LE, Vec<&str>>(&encoded, Format::DBus).unwrap();
        assert!(decoded.len() == 4);
        assert!(decoded[0] == "Hello");
        assert!(decoded[1] == "World");

        // As Variant
        let v = &as_[..].into_variant();
        assert!(v.value_signature() == "as");
        let encoded = to_bytes::<LE, _>(Format::DBus, v).unwrap();
        assert!(encoded.len() == 49);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
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
        let encoded = to_bytes::<LE, _>(Format::DBus, &ar[..]).unwrap();
        assert!(encoded.len() == 78);
        let decoded = from_slice::<LE, Vec<(u8, u32, (i64, bool, i64, Vec<&str>), &str)>>(
            &encoded,
            Format::DBus,
        )
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
        let encoded = to_bytes::<LE, _>(Format::DBus, v).unwrap();
        assert!(encoded.len() == 94);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
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
        let encoded = to_bytes::<LE, _>(Format::DBus, &map).unwrap();
        assert!(dbg!(encoded.len()) == 40);
        let decoded = from_slice::<LE, HashMap<i64, &str>>(&encoded, Format::DBus).unwrap();
        assert!(decoded[&1] == "123");
        assert!(decoded[&2] == "456");

        // As Variant
        let v = Dict::from(map).into_variant();
        assert!(v.value_signature() == "a{xs}");
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(encoded.len() == 48);
        // Convert it back
        let dict = Dict::from_variant(v).unwrap();
        let map: HashMap<i64, &str> = HashMap::try_from(dict).unwrap();
        assert!(map[&1] == "123");
        assert!(map[&2] == "456");
        // Also decode it back
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
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
        let encoded = to_bytes::<LE, _>(Format::DBus, &v).unwrap();
        assert!(dbg!(encoded.len()) == 68);
        let v = from_slice::<LE, Variant>(&encoded, Format::DBus).unwrap();
        if let Variant::Dict(dict) = v {
            assert!(*dict.get::<_, Variant>(&"hello").unwrap().unwrap() == Variant::Str("there"));
            assert!(*dict.get::<_, Variant>(&"bye").unwrap().unwrap() == Variant::Str("now"));
        } else {
            panic!();
        }
    }
}
