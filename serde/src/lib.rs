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

    use crate::to_bytes;
    use crate::{Array, Dict, EncodingFormat};
    use crate::{FromVariant, IntoVariant, Variant, VariantValue};
    use crate::{ObjectPath, Signature};

    #[test]
    fn u8_variant() {
        let encoded = to_bytes::<_, LE>(&77_u8, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 1);

        // As Variant
        let v = 77_u8.into_variant();
        assert!(v.value_signature() == "y");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 4);
    }

    #[test]
    fn u16_variant() {
        let encoded = to_bytes::<_, BE>(&0xABBA_u16, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 2);
        assert!(LE::read_u16(&encoded) == 0xBAAB_u16);

        // As Variant
        let v = 0xFEFE_u16.into_variant();
        assert!(v.value_signature() == "q");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 6);
    }

    #[test]
    fn i16_variant() {
        let encoded = to_bytes::<_, BE>(&-0xAB0_i16, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 2);
        assert!(LE::read_i16(&encoded) == 0x50F5_i16);

        // As Variant
        let v = 0xAB_i16.into_variant();
        assert!(v.value_signature() == "n");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 6);
    }

    #[test]
    fn u32_variant() {
        let encoded = to_bytes::<_, BE>(&0xABBA_ABBA_u32, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 4);
        assert!(LE::read_u32(&encoded) == 0xBAAB_BAAB_u32);

        // As Variant
        let v = 0xABBA_ABBA_u32.into_variant();
        assert!(v.value_signature() == "u");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);
    }

    #[test]
    fn i32_variant() {
        let encoded = to_bytes::<_, BE>(&-0xABBA_AB0_i32, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 4);
        assert!(LE::read_i32(&encoded) == 0x5055_44F5_i32);

        // As Variant
        let v = 0xABBA_AB0_i32.into_variant();
        assert!(v.value_signature() == "i");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);
    }

    #[test]
    fn u64_variant() {
        let encoded = to_bytes::<_, BE>(&0xABBA_ABBA_ABBA_ABBA_u64, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);
        assert!(LE::read_u64(&encoded) == 0xBAAB_BAAB_BAAB_BAAB_u64);

        // As Variant
        let v = 0xFEFE_u64.into_variant();
        assert!(v.value_signature() == "t");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 16);
    }

    #[test]
    fn i64_variant() {
        let encoded = to_bytes::<_, BE>(&-0xABBA_ABBA_ABBA_AB0_i64, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);
        assert!(LE::read_i64(&encoded) == 0x5055_4455_4455_44F5_i64);

        // As Variant
        let v = 0xABBA_AB0i64.into_variant();
        assert!(v.value_signature() == "x");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 16);
    }

    #[test]
    fn f64_variant() {
        let encoded = to_bytes::<_, BE>(&99999.99999_f64, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);
        assert!(LE::read_f64(&encoded) == -5759340900185448e-143);

        // As Variant
        let v = 99999.99999_f64.into_variant();
        assert!(v.value_signature() == "d");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 16);
    }

    #[test]
    fn str_variant() {
        let string = "hello world";
        let encoded = to_bytes::<_, LE>(&string, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 16);

        // As Variant
        let v = string.into_variant();
        assert!(v.value_signature() == "s");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 20);
    }

    #[test]
    fn signature_variant() {
        let sig = Signature::from("yys");
        let encoded = to_bytes::<_, LE>(&sig, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 5);

        // As Variant
        let v = sig.into_variant();
        assert!(v.value_signature() == "g");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);
    }

    #[test]
    fn object_path_variant() {
        let o = ObjectPath::from("/hello/world");
        let encoded = to_bytes::<_, LE>(&o, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 17);

        // As Variant
        let v = o.into_variant();
        assert!(v.value_signature() == "o");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 21);
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
        let encoded = to_bytes::<_, LE>(&ay[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 6);

        // As Variant
        let v = &ay[..].into_variant();
        assert!(v.value_signature() == "ay");
        let encoded = to_bytes::<_, LE>(v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 10);

        // Now try as Vec
        let vec = ay.to_vec();
        let encoded = to_bytes::<_, LE>(&vec, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 6);

        // Vec as Variant
        let v = Array::from(&vec).into_variant();
        assert!(v.value_signature() == "ay");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 10);

        // Emtpy array
        let ay: [u64; 0] = [];
        let encoded = to_bytes::<_, LE>(&ay[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);

        //
        // Array of strings
        //
        // Can't use 'as' as it's a keyword
        let as_ = ["Hello", "World", "Now", "Bye!"];
        let encoded = to_bytes::<_, LE>(&as_[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 45);

        // As Variant
        let v = &as_[..].into_variant();
        assert!(v.value_signature() == "as");
        let encoded = to_bytes::<_, LE>(v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 49);

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
        let encoded = to_bytes::<_, LE>(&ar[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 78);

        // As Variant
        let v = &ar[..].into_variant();
        assert!(v.value_signature() == "a(yu(xbxas)s)");
        let encoded = to_bytes::<_, LE>(v, EncodingFormat::DBus).unwrap();
        assert!(dbg!(encoded.len()) == 94);
    }

    #[test]
    fn dict_variant() {
        let mut map: HashMap<i64, &str> = HashMap::new();
        map.insert(1, "123");
        map.insert(2, "456");
        let encoded = to_bytes::<_, LE>(&map, EncodingFormat::DBus).unwrap();
        assert!(dbg!(encoded.len()) == 40);

        // As Variant
        let v = Dict::from(map).into_variant();
        assert!(v.value_signature() == "a{xs}");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 48);
        // Convert it back
        let dict = Dict::from_variant(v).unwrap();
        let map: HashMap<i64, &str> = HashMap::try_from(dict).unwrap();
        assert!(map[&1] == "123");
        assert!(map[&2] == "456");

        // Now a hand-crafted Dict Variant but with a Variant as value
        let mut dict = Dict::new(&<&str>::signature(), &Variant::signature());
        dict.add("hello", "there".into_variant()).unwrap();
        dict.add("bye", "now".into_variant()).unwrap();
        let v = dict.into_variant();
        assert!(v.value_signature() == "a{sv}");
        let encoded = to_bytes::<_, LE>(&v, EncodingFormat::DBus).unwrap();
        assert!(dbg!(encoded.len()) == 68);
    }
}
