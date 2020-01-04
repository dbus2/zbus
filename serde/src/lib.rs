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

// TODO: Tests for all *serde* types

#[cfg(test)]
mod tests {
    use crate::to_bytes;
    use crate::{Array, EncodingFormat, ObjectPath, Signature, Variant};

    #[test]
    fn u8_variant() {
        let (encoded, s) = to_bytes(&77u8, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 1);
        assert!(s.as_str() == "y");

        // As Variant
        let v = Variant::from(77u8);
        let (encoded, s) = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 4);
        assert!(s.as_str() == "v");
    }

    #[test]
    fn str_variant() {
        let string = "hello world";
        let (encoded, s) = to_bytes(&string, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 16);
        assert!(s.as_str() == "s");

        // As Variant
        let v = Variant::from(string);
        let (encoded, s) = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 20);
        assert!(s.as_str() == "v");
    }

    #[test]
    fn signature_variant() {
        let sig = Signature::from("yys");
        let (encoded, s) = to_bytes(&sig, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 5);
        assert!(s.as_str() == "g");

        // As Variant
        let v = Variant::from(sig);
        let (encoded, s) = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 8);
        assert!(s.as_str() == "v");
    }

    #[test]
    fn object_path_variant() {
        let o = ObjectPath::from("/hello/world");
        let (encoded, s) = to_bytes(&o, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 17);
        assert!(s.as_str() == "o");

        // As Variant
        let v = Variant::from(o);
        let (encoded, signature) = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 21);
        assert!(signature.as_str() == "v");
    }

    #[test]
    fn array_variant() {
        let array = [77u8, 88];
        // Array itself is treated like a tuple by serde & that translates to a structure in our
        // case so gotta make it a slice for serde to treat it as seq type.
        let (encoded, s) = to_bytes(&array[..], EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 6);
        assert!(s.as_str() == "ay");

        // As Variant
        // FIXME: Provide a more direct translation
        let v = Variant::from(Array::from(&array[..]));
        let (encoded, s) = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 10);
        assert!(s.as_str() == "v");

        // Now try as Vec
        let vec = array.to_vec();
        let (encoded, s) = to_bytes(&vec, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 6);
        assert!(s.as_str() == "ay");

        // Vec as Variant
        let v = Variant::from(Array::from(&vec));
        let (encoded, s) = to_bytes(&v, EncodingFormat::DBus).unwrap();
        assert!(encoded.len() == 10);
        assert!(s.as_str() == "v");
    }
}
