use std::str;

use crate::VariantError;
use crate::{Array, DictEntry, Encode, EncodingFormat};
use crate::{Decode, ObjectPath};
use crate::{SharedData, Signature, SimpleDecode, Structure};

#[derive(Debug, Clone)]
pub enum Variant {
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
    Str(String), // TODO: Optimize later!
    Signature(Signature),
    ObjectPath(ObjectPath),
    Variant(Box<Variant>),

    // Container types
    Array(Array),
    DictEntry(DictEntry),
    Structure(Structure),
}

impl Variant {
    pub fn from_data(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let signature = signature.into();
        // slice_data() ensures a valid signature
        let slice = crate::decode::slice_data(data, signature.clone(), format)?;

        match signature
            .chars()
            .next()
            .ok_or(VariantError::InsufficientData)?
        {
            // FIXME: There has to be a shorter way to do this
            u8::SIGNATURE_CHAR => u8::decode_simple(&slice, format).map(|value| Variant::U8(value)),
            bool::SIGNATURE_CHAR => {
                bool::decode_simple(&slice, format).map(|value| Variant::Bool(value))
            }
            i16::SIGNATURE_CHAR => {
                i16::decode_simple(&slice, format).map(|value| Variant::I16(value))
            }
            u16::SIGNATURE_CHAR => {
                u16::decode_simple(&slice, format).map(|value| Variant::U16(value))
            }
            i32::SIGNATURE_CHAR => {
                i32::decode_simple(&slice, format).map(|value| Variant::I32(value))
            }
            u32::SIGNATURE_CHAR => {
                u32::decode_simple(&slice, format).map(|value| Variant::U32(value))
            }
            i64::SIGNATURE_CHAR => {
                i64::decode_simple(&slice, format).map(|value| Variant::I64(value))
            }
            u64::SIGNATURE_CHAR => {
                u64::decode_simple(&slice, format).map(|value| Variant::U64(value))
            }
            f64::SIGNATURE_CHAR => {
                f64::decode_simple(&slice, format).map(|value| Variant::F64(value))
            }
            String::SIGNATURE_CHAR => {
                String::decode_simple(&slice, format).map(|value| Variant::Str(value))
            }
            Array::SIGNATURE_CHAR => {
                Array::decode(&slice, signature, format).map(|value| Variant::Array(value))
            }
            ObjectPath::SIGNATURE_CHAR => {
                ObjectPath::decode_simple(&slice, format).map(|value| Variant::ObjectPath(value))
            }
            Signature::SIGNATURE_CHAR => {
                Signature::decode_simple(&slice, format).map(|value| Variant::Signature(value))
            }
            Structure::SIGNATURE_CHAR => {
                Structure::decode(&slice, signature, format).map(|value| Variant::Structure(value))
            }
            Variant::SIGNATURE_CHAR => Variant::decode_simple(&slice, format)
                .map(|value| Variant::Variant(Box::new(value))),
            DictEntry::SIGNATURE_CHAR => {
                DictEntry::decode(&slice, signature, format).map(|value| Variant::DictEntry(value))
            }
            _ => return Err(VariantError::UnsupportedType(signature)),
        }
    }

    // Only use for standalone or the first data in a message
    pub fn encode_value(&self, format: EncodingFormat) -> Vec<u8> {
        let mut bytes = vec![];
        self.encode_value_into(&mut bytes, format);

        bytes
    }

    pub fn encode_value_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        match self {
            // Simple types
            Variant::U8(value) => value.encode_into(bytes, format),
            Variant::Bool(value) => value.encode_into(bytes, format),
            Variant::I16(value) => value.encode_into(bytes, format),
            Variant::U16(value) => value.encode_into(bytes, format),
            Variant::I32(value) => value.encode_into(bytes, format),
            Variant::U32(value) => value.encode_into(bytes, format),
            Variant::I64(value) => value.encode_into(bytes, format),
            Variant::U64(value) => value.encode_into(bytes, format),
            Variant::F64(value) => value.encode_into(bytes, format),
            Variant::Str(value) => value.encode_into(bytes, format), // TODO: Optimize later!
            Variant::Signature(value) => value.encode_into(bytes, format),
            Variant::ObjectPath(value) => value.encode_into(bytes, format),
            Variant::Variant(value) => value.encode_into(bytes, format),

            // Container types
            Variant::Array(value) => value.encode_into(bytes, format),
            Variant::DictEntry(value) => value.encode_into(bytes, format),
            Variant::Structure(value) => value.encode_into(bytes, format),
        }
    }

    pub fn value_padding(&self, n_bytes_before: usize, format: EncodingFormat) -> usize {
        match self {
            // Simple types
            Variant::U8(_) => u8::padding(n_bytes_before, format),
            Variant::Bool(_) => bool::padding(n_bytes_before, format),
            Variant::I16(_) => i16::padding(n_bytes_before, format),
            Variant::U16(_) => u16::padding(n_bytes_before, format),
            Variant::I32(_) => i32::padding(n_bytes_before, format),
            Variant::U32(_) => u32::padding(n_bytes_before, format),
            Variant::I64(_) => i64::padding(n_bytes_before, format),
            Variant::U64(_) => u64::padding(n_bytes_before, format),
            Variant::F64(_) => f64::padding(n_bytes_before, format),
            Variant::Str(_) => String::padding(n_bytes_before, format),
            Variant::Signature(_) => Signature::padding(n_bytes_before, format),
            Variant::ObjectPath(_) => ObjectPath::padding(n_bytes_before, format),
            Variant::Variant(_) => Variant::padding(n_bytes_before, format),

            // Container types
            Variant::Array(_) => Array::padding(n_bytes_before, format),
            Variant::DictEntry(_) => DictEntry::padding(n_bytes_before, format),
            Variant::Structure(_) => Structure::padding(n_bytes_before, format),
        }
    }

    pub fn value_signature(&self) -> Signature {
        match self {
            Variant::U8(value) => value.signature(),
            Variant::Bool(value) => value.signature(),
            Variant::I16(value) => value.signature(),
            Variant::U16(value) => value.signature(),
            Variant::I32(value) => value.signature(),
            Variant::U32(value) => value.signature(),
            Variant::I64(value) => value.signature(),
            Variant::U64(value) => value.signature(),
            Variant::F64(value) => value.signature(),
            Variant::Str(value) => value.signature(), // TODO: Optimize later!
            Variant::Signature(value) => value.signature(),
            Variant::ObjectPath(value) => value.signature(),
            Variant::Variant(value) => value.signature(),

            // Container types
            Variant::Array(value) => value.signature(),
            Variant::DictEntry(value) => value.signature(),
            Variant::Structure(value) => value.signature(),
        }
    }
}

impl Encode for Variant {
    const SIGNATURE_CHAR: char = 'v';
    const SIGNATURE_STR: &'static str = "v";
    const ALIGNMENT: usize = Signature::ALIGNMENT;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        self.value_signature().encode_into(bytes, format);

        self.encode_value_into(bytes, format)
    }

    // In case of Variant, this create a Variant::Variant(self) (i-e deflattens)
    fn to_variant(self) -> Variant {
        Variant::Variant(Box::new(self))
    }
}

impl Decode for Variant {
    fn slice_data(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;

        // Variant is made of signature of the value followed by the actual value. So we gotta
        // extract the signature slice first and then the value slice. Once we know the sizes of
        // both, we can just slice the whole thing.
        let sign_slice = Signature::slice_data_simple(&data, format)?;
        let sign_size = sign_slice.len();
        let sign = Signature::decode_simple(&sign_slice, format)?;

        let value_slice = crate::decode::slice_data(&data.tail(sign_size), sign, format)?;
        let total_size = sign_size + value_slice.len();

        Ok(data.head(total_size))
    }

    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;

        let sign_slice = Signature::slice_data_simple(&data, format)?;
        let sign_size = sign_slice.len();
        let sign = Signature::decode_simple(&sign_slice, format)?;

        Variant::from_data(&data.tail(sign_size), sign, format)
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::Variant(_) = variant {
            true
        } else {
            false
        }
    }

    // In case of Variant, this gets the inner Variant (i-e flattens)
    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::Variant(value) = variant {
            Ok(*value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::Variant(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}
impl SimpleDecode for Variant {}

#[cfg(test)]
mod tests {
    use core::convert::{TryFrom, TryInto};
    use std::collections::HashMap;

    use crate::{Array, Dict, DictEntry};
    use crate::{Decode, Encode, EncodingFormat, SimpleDecode};
    use crate::{SharedData, Structure};

    #[test]
    fn u8_variant() {
        let v = u8::max_value().to_variant();
        assert!(u8::is(&v));
        assert!(*u8::from_variant(&v).unwrap() == u8::max_value());

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 1);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*u8::from_variant(&v).unwrap() == u8::max_value());
    }

    #[test]
    fn bool_variant() {
        let v = true.to_variant();
        assert!(bool::from_variant(&v).unwrap());
        assert!(bool::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 4);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(bool::from_variant(&v).unwrap());
    }

    #[test]
    fn i16_variant() {
        let v = i16::max_value().to_variant();
        assert!(*i16::from_variant(&v).unwrap() == i16::max_value());
        assert!(i16::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 2);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*i16::from_variant(&v).unwrap() == i16::max_value());
    }

    #[test]
    fn u16_variant() {
        let v = u16::max_value().to_variant();
        assert!(*u16::from_variant(&v).unwrap() == u16::max_value());
        assert!(u16::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 2);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*u16::from_variant(&v).unwrap() == u16::max_value());
    }

    #[test]
    fn i32_variant() {
        let v = i32::max_value().to_variant();
        assert!(*i32::from_variant(&v).unwrap() == i32::max_value());
        assert!(i32::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 4);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*i32::from_variant(&v).unwrap() == i32::max_value());
    }

    #[test]
    fn u32_variant() {
        let v = u32::max_value().to_variant();
        assert!(*u32::from_variant(&v).unwrap() == u32::max_value());
        assert!(u32::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 4);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*u32::from_variant(&v).unwrap() == u32::max_value());
    }

    #[test]
    fn i64_variant() {
        let v = i64::max_value().to_variant();
        assert!(*i64::from_variant(&v).unwrap() == i64::max_value());
        assert!(i64::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 8);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*i64::from_variant(&v).unwrap() == i64::max_value());
    }

    #[test]
    fn u64_variant() {
        let v = u64::max_value().to_variant();
        assert!(*u64::from_variant(&v).unwrap() == u64::max_value());
        assert!(u64::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 8);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*u64::from_variant(&v).unwrap() == u64::max_value());
    }

    #[test]
    fn f64_variant() {
        let v = 117.112f64.to_variant();
        assert!(*f64::from_variant(&v).unwrap() == 117.112);
        assert!(f64::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 8);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*f64::from_variant(&v).unwrap() == 117.112);
    }

    #[test]
    fn str_variant() {
        let v = String::from("Hello world!").to_variant();
        assert!(*String::from_variant(&v).unwrap() == "Hello world!");
        assert!(String::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 17);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(*String::from_variant(&v).unwrap() == "Hello world!");
    }

    #[test]
    fn object_path_variant() {
        let v = crate::ObjectPath::new("Hello world!").to_variant();
        assert!(crate::ObjectPath::from_variant(&v).unwrap().as_str() == "Hello world!");
        assert!(crate::ObjectPath::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 17);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(crate::ObjectPath::from_variant(&v).unwrap().as_str() == "Hello world!");
    }

    #[test]
    fn signature_variant() {
        let v = crate::Signature::new("Hello world!").to_variant();
        assert!(crate::Signature::from_variant(&v).unwrap().as_str() == "Hello world!");
        assert!(crate::Signature::is(&v));

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 14);
        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(crate::Signature::from_variant(&v).unwrap().as_str() == "Hello world!");
    }

    #[test]
    fn variant_variant() {
        let v = 7u8.to_variant();
        // The argument to encode here shouln't matter cause variants are 1-byte aligned so just
        // keep an arbitrary odd number of bytes before the variant and encoding shouldn't add
        // any padding (i-e we should end up with 7 bytes only).
        let mut encoded = vec![0u8; 3];
        let format = EncodingFormat::default();
        v.encode_into(&mut encoded, format);
        assert!(encoded.len() == 7);

        // Add some extra bytes to the encoded data to test the slicing
        encoded.push(0);
        encoded.push(1);
        encoded.push(7);

        let encoded = SharedData::new(encoded).tail(3);
        let slice = crate::Variant::slice_data_simple(&encoded, format).unwrap();

        let decoded = crate::Variant::decode_simple(&slice, format).unwrap();
        assert!(decoded.signature() == crate::Variant::SIGNATURE_STR);
        assert!(decoded.value_signature() == u8::SIGNATURE_STR);
        assert!(*u8::from_variant(&decoded).unwrap() == 7u8);
    }

    #[test]
    fn struct_variant() {
        let mut map: HashMap<i64, String> = HashMap::new();
        map.insert(1, String::from("123"));
        map.insert(2, String::from("456"));
        let dict: Dict = map.into();
        let array = Array::try_from(dict).unwrap();

        let s = Structure::new()
            .add_field(u8::max_value())
            .add_field(u32::max_value())
            .add_field(
                Structure::new()
                    .add_field(i64::max_value())
                    .add_field(true)
                    .add_field(
                        Structure::new()
                            .add_field(i64::max_value())
                            .add_field(std::f64::MAX),
                    ),
            )
            .add_field(String::from("hello"))
            .add_field(array);
        let v = s.to_variant();
        assert!(Structure::is(&v));
        let s = Structure::from_variant(&v).unwrap();
        let fields = s.fields();
        assert!(u8::is(&fields[0]));
        assert!(*u8::from_variant(&fields[0]).unwrap() == u8::max_value());
        assert!(u32::is(&fields[1]));
        assert!(*u32::from_variant(&fields[1]).unwrap() == u32::max_value());

        assert!(Structure::is(&fields[2]));
        let inner = Structure::from_variant(&fields[2]).unwrap();
        let inner_fields = inner.fields();
        assert!(i64::is(&inner_fields[0]));
        assert!(*i64::from_variant(&inner_fields[0]).unwrap() == i64::max_value());
        assert!(bool::is(&inner_fields[1]));
        assert!(bool::from_variant(&inner_fields[1]).unwrap());
        assert!(Structure::is(&inner_fields[2]));
        let inner = Structure::from_variant(&inner_fields[2]).unwrap();
        let inner_fields = inner.fields();
        assert!(i64::is(&inner_fields[0]));
        assert!(*i64::from_variant(&inner_fields[0]).unwrap() == i64::max_value());
        assert!(f64::is(&inner_fields[1]));
        assert!(*f64::from_variant(&inner_fields[1]).unwrap() == std::f64::MAX);

        assert!(String::is(&fields[3]));
        assert!(String::from_variant(&fields[3]).unwrap() == "hello");

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        // The HashMap is unordered so we can't rely on items to be in a specific order during the transformation to
        // Vec, and size depends on the order of items because of padding rules.
        assert!(encoding.len() == 88 || encoding.len() == 92);

        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(Structure::is(&v));
        let s = Structure::from_variant(&v).unwrap();
        let fields = s.fields();
        assert!(*u8::from_variant(&fields[0]).unwrap() == u8::max_value());
        assert!(u8::is(&fields[0]));
        assert!(*u32::from_variant(&fields[1]).unwrap() == u32::max_value());
        assert!(u32::is(&fields[1]));

        assert!(Structure::is(&fields[2]));
        let inner = Structure::from_variant(&fields[2]).unwrap();
        let inner_fields = inner.fields();
        assert!(i64::is(&inner_fields[0]));
        assert!(*i64::from_variant(&inner_fields[0]).unwrap() == i64::max_value());
        assert!(bool::is(&inner_fields[1]));
        assert!(*bool::from_variant(&inner_fields[1]).unwrap());

        assert!(String::from_variant(&fields[3]).unwrap() == "hello");
        assert!(String::is(&fields[3]));
    }

    #[test]
    fn array_variant() {
        // Let's use D-Bus/GVariant terms

        // Array of u8
        let ay = vec![u8::max_value(), 0u8, 47u8];
        let array = Array::from(ay);
        assert!(array.signature() == "ay");
        for element in array.inner() {
            assert!(u8::is(&element));
        }

        let format = EncodingFormat::default();
        let encoding = SharedData::new(array.encode(format));
        assert!(encoding.len() == 7);
        let v = crate::Variant::from_data(&encoding, array.signature(), format).unwrap();
        let array = Array::take_from_variant(v).unwrap();
        let ay: Vec<u8> = array.try_into().unwrap();
        assert!(ay == [u8::max_value(), 0u8, 47u8]);

        // Array of strings
        // Can't use 'as' as it's a keyword
        let as_ = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("Now"),
            String::from("Bye!"),
        ];
        let array = Array::from(as_);
        assert!(array.signature() == "as");
        for element in array.inner() {
            assert!(String::is(&element));
        }

        let encoding = SharedData::new(array.encode(format));
        assert!(encoding.len() == 45);
        let v = crate::Variant::from_data(&encoding, array.signature(), format).unwrap();
        let array = Array::take_from_variant(v).unwrap();
        let as_: Vec<String> = array.try_into().unwrap();
        assert!(as_ == ["Hello", "World", "Now", "Bye!"]);

        // Array of Struct, which in turn containin an Array (We gotta go deeper!)
        let ar = vec![Structure::new()
            // top-most simple fields
            .add_field(u8::max_value())
            .add_field(u32::max_value())
            // top-most inner structure
            .add_field(
                Structure::new()
                    // 2nd level simple fields
                    .add_field(i64::max_value())
                    .add_field(true)
                    .add_field(i64::max_value())
                    // 2nd level array field
                    .add_field(Array::from(vec![
                        String::from("Hello"),
                        String::from("World"),
                    ])),
            )
            // one more top-most simple field
            .add_field(String::from("hello"))];
        let array = Array::from(ar);
        assert!(array.signature() == "a(yu(xbxas)s)");
        for element in array.inner() {
            assert!(Structure::is(&element));
        }

        let encoding = SharedData::new(array.encode(format));
        assert!(encoding.len() == 78);
        let v = crate::Variant::from_data(&encoding, array.signature(), format).unwrap();
        let array = Array::take_from_variant(v).unwrap();
        let mut ar: Vec<Structure> = array.try_into().unwrap();

        // top-most structure
        let s = ar.remove(0);
        let mut fields = s.take_fields();

        // top-most simple fields
        assert!(u8::is(&fields[0]));
        assert!(*u8::from_variant(&fields[0]).unwrap() == u8::max_value());
        assert!(u32::is(&fields[1]));
        assert!(*u32::from_variant(&fields[1]).unwrap() == u32::max_value());
        assert!(String::is(&fields[3]));
        assert!(String::from_variant(&fields[3]).unwrap() == "hello");

        // top-most inner structure
        let inner = Structure::take_from_variant(fields.remove(2)).unwrap();
        let mut inner_fields = inner.take_fields();

        // 2nd level simple fields
        assert!(i64::is(&inner_fields[0]));
        assert!(*i64::from_variant(&inner_fields[0]).unwrap() == i64::max_value());
        assert!(bool::is(&inner_fields[1]));
        assert!(*bool::from_variant(&inner_fields[1]).unwrap() == true);

        // 2nd level array field
        let array = Array::take_from_variant(inner_fields.remove(3)).unwrap();
        let as_: Vec<String> = array.try_into().unwrap();
        assert!(as_ == [String::from("Hello"), String::from("World")]);
    }

    #[test]
    fn dict_entry_variant() {
        // Simple type value
        let entry = DictEntry::new(2u8, String::from("world"));
        assert!(entry.signature() == "{ys}");
        let v = entry.to_variant();

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));

        let v = crate::Variant::from_data(&encoding, v.value_signature(), format).unwrap();
        assert!(encoding.len() == 14);
        assert!(DictEntry::is(&v));
        let entry = DictEntry::from_variant(&v).unwrap();
        assert!(*entry.key::<u8>().unwrap() == 2u8);
        assert!(entry.value::<String>().unwrap() == "world");

        // STRUCT value
        let entry = DictEntry::new(
            String::from("hello"),
            Structure::new()
                .add_field(u8::max_value())
                .add_field(u32::max_value()),
        );
        assert!(entry.signature() == "{s(yu)}");
        let v = entry.to_variant();

        let format = EncodingFormat::default();
        let encoding = SharedData::new(v.encode_value(format));
        assert!(encoding.len() == 24);
        assert!(DictEntry::is(&v));
        let entry = DictEntry::from_variant(&v).unwrap();
        assert!(entry.key::<String>().unwrap() == "hello");
        let s = entry.value::<Structure>().unwrap();
        let fields = s.fields();
        assert!(u8::is(&fields[0]));
        assert!(*u8::from_variant(&fields[0]).unwrap() == u8::max_value());
        assert!(u32::is(&fields[1]));
        assert!(*u32::from_variant(&fields[1]).unwrap() == u32::max_value());
    }
}
