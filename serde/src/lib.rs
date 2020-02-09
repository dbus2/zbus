mod array;
pub use array::*;

mod basic;
pub use basic::*;

mod dict_entry;
pub use dict_entry::*;

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

mod utils;

// TODO: Tests for all *serde* types and import all existing ones from zvariant

#[cfg(test)]
mod tests {
    use crate::to_bytes;
    use crate::{Array, EncodingFormat, ObjectPath, Signature, Variant};

    #[test]
    fn u8_variant() {
        let encoded = to_bytes(&77u8, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 1);

        // As Variant
        let v = Variant::from(77u8);
        assert!(v.value_signature().as_str() == "y");
        let encoded = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 4);
    }

    #[test]
    fn str_variant() {
        let string = "hello world";
        let encoded = to_bytes(&string, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 16);

        // As Variant
        let v = Variant::from(string);
        assert!(v.value_signature().as_str() == "s");
        let encoded = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 20);
    }

    #[test]
    fn signature_variant() {
        let sig = Signature::from("yys");
        let encoded = to_bytes(&sig, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 5);

        // As Variant
        let v = Variant::from(sig);
        assert!(v.value_signature().as_str() == "g");
        let encoded = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);
    }

    #[test]
    fn object_path_variant() {
        let o = ObjectPath::from("/hello/world");
        let encoded = to_bytes(&o, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 17);

        // As Variant
        let v = Variant::from(o);
        assert!(v.value_signature().as_str() == "o");
        let encoded = to_bytes(&v, EncodingFormat::DBus).unwrap();
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
        let encoded = to_bytes(&ay[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 6);

        // As Variant
        let v = Variant::from(&ay[..]);
        assert!(v.value_signature().as_str() == "ay");
        let encoded = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 10);

        // Now try as Vec
        let vec = ay.to_vec();
        let encoded = to_bytes(&vec, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 6);

        // Vec as Variant
        let v = Variant::from(Array::from(&vec));
        assert!(v.value_signature().as_str() == "ay");
        let encoded = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 10);

        // Emtpy array
        let ay: [u8; 0] = [];
        let encoded = to_bytes(&ay[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 4);

        //
        // Array of strings
        //
        // Can't use 'as' as it's a keyword
        let as_ = ["Hello", "World", "Now", "Bye!"];
        let encoded = to_bytes(&as_[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 45);

        // As Variant
        let v = Variant::from(&as_[..]);
        assert!(v.value_signature().as_str() == "as");
        let encoded = to_bytes(&v, EncodingFormat::DBus).unwrap();
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
        let encoded = to_bytes(&ar[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 78);

        // As Variant
        let v = Variant::from(&ar[..]);
        assert!(v.value_signature().as_str() == "a(yu(xbxas)s)");
        let encoded = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(dbg!(encoded.len()) == 94);
    }
}
