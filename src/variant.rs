use std::str;

use crate::{SharedData, Signature, SimpleVariantType};
use crate::{VariantError, VariantType, VariantTypeConstants};

#[derive(Debug)]
pub struct Variant {
    signature: String,
    value: SharedData,
}

impl Variant {
    pub fn from_data(data: &SharedData, signature: &str) -> Result<Self, VariantError> {
        // slice_data() ensures a valid signature
        let value = crate::variant_type::slice_data(data, signature)?;

        Ok(Self {
            value: value,
            signature: String::from(signature),
        })
    }

    pub fn from<T: VariantType>(value: T) -> Self {
        Self {
            value: SharedData::new(value.encode()),
            signature: String::from(value.signature()),
        }
    }

    pub fn signature(&self) -> &str {
        &self.signature
    }

    pub fn get<T: VariantType>(&self) -> Result<T, VariantError> {
        T::decode(&self.value, &self.signature)
    }

    pub fn bytes(&self) -> &SharedData {
        &self.value
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Checks if contained value is of the generic type `T`
    ///
    /// # Examples
    ///
    /// ```
    /// let v = zbus::Variant::from(String::froM("hello"));
    /// assert!(!v.is::<u32>());
    /// assert!(v.is::<String>());
    /// ```
    ///
    /// ```
    /// let v = zbus::Variant::from(147u32);
    /// assert!(!v.is::<String>());
    /// assert!(v.is::<u32>());
    /// ```
    pub fn is<T: VariantType>(&self) -> bool {
        self.signature.starts_with(T::signature_str())
    }
}

impl VariantTypeConstants for Variant {
    const SIGNATURE_CHAR: char = 'v';
    const SIGNATURE_STR: &'static str = "v";
    const ALIGNMENT: usize = Signature::ALIGNMENT;
}

impl VariantType for Variant {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>) {
        Signature::new(&self.signature).encode_into(bytes);
        self.value.apply_mut(|b| bytes.extend_from_slice(b));
    }

    fn slice_data(data: &SharedData, signature: &str) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;

        // Variant is made of signature of the value followed by the actual value. So we gotta
        // extract the signature slice first and then the value slice. Once we know the sizes of
        // both, we can just slice the whole thing.
        let sign_slice = Signature::slice_data_simple(&data)?;
        let sign_size = sign_slice.len();
        let sign = Signature::decode_simple(&sign_slice)?;

        let value_slice = crate::variant_type::slice_data(&data.tail(sign_size), sign.as_str())?;
        let total_size = sign_size + value_slice.len();

        Ok(data.head(total_size))
    }

    fn decode(data: &SharedData, signature: &str) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;

        let sign_slice = Signature::slice_data_simple(&data)?;
        let sign_size = sign_slice.len();
        let sign = Signature::decode_simple(&sign_slice)?;

        Variant::from_data(&data.tail(sign_size), sign.as_str())
    }
}
impl SimpleVariantType for Variant {}

#[cfg(test)]
mod tests {
    use crate::{Dict, DictEntry, SimpleVariantType};
    use crate::{SharedData, Structure, StructureBuilder};
    use crate::{VariantType, VariantTypeConstants};

    #[test]
    fn u8_variant() {
        let v = crate::Variant::from(u8::max_value());
        assert!(v.len() == 1);
        assert!(v.get::<u8>().unwrap() == u8::max_value());
        assert!(v.is::<u8>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 1);
        assert!(v.get::<u8>().unwrap() == u8::max_value());
    }

    #[test]
    fn bool_variant() {
        let v = crate::Variant::from(true);
        assert!(v.len() == 4);
        assert!(v.get::<bool>().unwrap());
        assert!(v.is::<bool>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 4);
        assert!(v.get::<bool>().unwrap());
    }

    #[test]
    fn i16_variant() {
        let v = crate::Variant::from(i16::max_value());
        assert!(v.len() == 2);
        assert!(v.get::<i16>().unwrap() == i16::max_value());
        assert!(v.is::<i16>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 2);
        assert!(v.get::<i16>().unwrap() == i16::max_value());
    }

    #[test]
    fn u16_variant() {
        let v = crate::Variant::from(u16::max_value());
        assert!(v.len() == 2);
        assert!(v.get::<u16>().unwrap() == u16::max_value());
        assert!(v.is::<u16>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 2);
        assert!(v.get::<u16>().unwrap() == u16::max_value());
    }

    #[test]
    fn i32_variant() {
        let v = crate::Variant::from(i32::max_value());
        assert!(v.len() == 4);
        assert!(v.get::<i32>().unwrap() == i32::max_value());
        assert!(v.is::<i32>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 4);
        assert!(v.get::<i32>().unwrap() == i32::max_value());
    }

    #[test]
    fn u32_variant() {
        let v = crate::Variant::from(u32::max_value());
        assert!(v.len() == 4);
        assert!(v.get::<u32>().unwrap() == u32::max_value());
        assert!(v.is::<u32>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 4);
        assert!(v.get::<u32>().unwrap() == u32::max_value());
    }

    #[test]
    fn i64_variant() {
        let v = crate::Variant::from(i64::max_value());
        assert!(v.len() == 8);
        assert!(v.get::<i64>().unwrap() == i64::max_value());
        assert!(v.is::<i64>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 8);
        assert!(v.get::<i64>().unwrap() == i64::max_value());
    }

    #[test]
    fn u64_variant() {
        let v = crate::Variant::from(u64::max_value());
        assert!(v.len() == 8);
        assert!(v.get::<u64>().unwrap() == u64::max_value());
        assert!(v.is::<u64>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 8);
        assert!(v.get::<u64>().unwrap() == u64::max_value());
    }

    #[test]
    fn f64_variant() {
        let v = crate::Variant::from(117.112f64);
        assert!(v.len() == 8);
        assert!(v.get::<f64>().unwrap() == 117.112);
        assert!(v.is::<f64>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 8);
        assert!(v.get::<f64>().unwrap() == 117.112);
    }

    #[test]
    fn str_variant() {
        let v = crate::Variant::from(String::from("Hello world!"));
        assert!(v.len() == 17);
        assert!(v.get::<String>().unwrap() == "Hello world!");
        assert!(v.is::<String>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 17);
        assert!(v.get::<String>().unwrap() == "Hello world!");
    }

    #[test]
    fn object_path_variant() {
        let v = crate::Variant::from(crate::ObjectPath::new("Hello world!"));
        assert!(v.len() == 17);
        assert!(v.get::<crate::ObjectPath>().unwrap().as_str() == "Hello world!");
        assert!(v.is::<crate::ObjectPath>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 17);
        assert!(v.get::<crate::ObjectPath>().unwrap().as_str() == "Hello world!");
    }

    #[test]
    fn signature_variant() {
        let v = crate::Variant::from(crate::Signature::new("Hello world!"));
        assert!(v.len() == 14);
        assert!(v.get::<crate::Signature>().unwrap().as_str() == "Hello world!");
        assert!(v.is::<crate::Signature>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 14);
        assert!(v.get::<crate::Signature>().unwrap().as_str() == "Hello world!");
    }

    #[test]
    fn variant_variant() {
        let v = crate::Variant::from(7u8);
        // The argument to encode here shouln't matter cause variants are 1-byte aligned so just
        // keep an arbitrary odd number of bytes before the variant and encoding shouldn't add
        // any padding (i-e we should end up with 7 bytes only).
        let mut encoded = vec![0u8; 3];
        v.encode_into(&mut encoded);
        assert!(encoded.len() == 7);

        // Add some extra bytes to the encoded data to test the slicing
        encoded.push(0);
        encoded.push(1);
        encoded.push(7);

        let encoded = SharedData::new(encoded).tail(3);
        let slice = crate::Variant::slice_data_simple(&encoded).unwrap();

        let decoded = crate::Variant::decode_simple(&slice).unwrap();
        assert!(decoded.signature() == u8::SIGNATURE_STR);
        assert!(decoded.get::<u8>().unwrap() == 7u8);
    }

    #[test]
    fn struct_variant() {
        let mut dict: Dict<i64, String> = Dict::new();
        dict.insert(1, String::from("123"));
        dict.insert(2, String::from("456"));
        let dict_vec: Vec<DictEntry<i64, String>> = dict.into();

        let s = StructureBuilder::new()
            .add_field(u8::max_value())
            .add_field(u32::max_value())
            .add_field(
                StructureBuilder::new()
                    .add_field(i64::max_value())
                    .add_field(true)
                    .add_field(
                        StructureBuilder::new()
                            .add_field(i64::max_value())
                            .add_field(std::f64::MAX)
                            .create(),
                    )
                    .create(),
            )
            .add_field(String::from("hello"))
            .add_field(dict_vec)
            .create();
        let v = crate::Variant::from(s);
        // The HashMap is unordered so we can't rely on items to be in a specific order during the transformation to
        // Vec, and size depends on the order of items because of padding rules.
        assert!(v.len() == 88 || v.len() == 92);

        assert!(v.is::<Structure>());
        let s = v.get::<Structure>().unwrap();
        let fields = s.fields();
        assert!(fields[0].is::<u8>());
        assert!(fields[0].get::<u8>().unwrap() == u8::max_value());
        assert!(fields[1].is::<u32>());
        assert!(fields[1].get::<u32>().unwrap() == u32::max_value());

        assert!(fields[2].is::<Structure>());
        let inner = fields[2].get::<Structure>().unwrap();
        let inner_fields = inner.fields();
        assert!(inner_fields[0].is::<i64>());
        assert!(inner_fields[0].get::<i64>().unwrap() == i64::max_value());
        assert!(inner_fields[1].is::<bool>());
        assert!(inner_fields[1].get::<bool>().unwrap());
        assert!(inner_fields[2].is::<Structure>());
        let inner = inner_fields[2].get::<Structure>().unwrap();
        let inner_fields = inner.fields();
        assert!(inner_fields[0].is::<i64>());
        assert!(inner_fields[0].get::<i64>().unwrap() == i64::max_value());
        assert!(inner_fields[1].is::<f64>());
        assert!(inner_fields[1].get::<f64>().unwrap() == std::f64::MAX);

        assert!(fields[3].is::<String>());
        assert!(fields[3].get::<String>().unwrap() == "hello");

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 88 || v.len() == 92);

        assert!(v.is::<Structure>());
        let s = v.get::<Structure>().unwrap();
        let fields = s.fields();
        assert!(fields[0].get::<u8>().unwrap() == u8::max_value());
        assert!(fields[0].is::<u8>());
        assert!(fields[1].get::<u32>().unwrap() == u32::max_value());
        assert!(fields[1].is::<u32>());

        assert!(fields[2].is::<Structure>());
        let inner = fields[2].get::<Structure>().unwrap();
        let inner_fields = inner.fields();
        assert!(inner_fields[0].is::<i64>());
        assert!(inner_fields[0].get::<i64>().unwrap() == i64::max_value());
        assert!(inner_fields[1].is::<bool>());
        assert!(inner_fields[1].get::<bool>().unwrap());

        assert!(fields[3].get::<String>().unwrap() == "hello");
        assert!(fields[3].is::<String>());
    }

    #[test]
    fn array_variant() {
        // Let's use D-Bus/GVariant terms

        // Array of u8
        let ay = vec![u8::max_value(), 0u8, 47u8];
        assert!(ay.signature() == "ay");
        let v = crate::Variant::from(ay);
        assert!(v.len() == 7);
        assert!(v.is::<Vec::<u8>>());
        let v = v.get::<Vec<u8>>().unwrap();
        assert!(v == [u8::max_value(), 0u8, 47u8]);

        // Array of strings
        // Can't use 'as' as it's a keyword
        let as_ = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("Now"),
            String::from("Bye!"),
        ];
        assert!(as_.signature() == "as");
        let v = crate::Variant::from(as_);
        assert!(v.len() == 45);
        let v = v.get::<Vec<String>>().unwrap();
        assert!(v == ["Hello", "World", "Now", "Bye!"]);

        // Array of Struct, which in turn containin an Array (We gotta go deeper!)
        let ar = vec![StructureBuilder::new()
            .add_field(u8::max_value())
            .add_field(u32::max_value())
            .add_field(
                StructureBuilder::new()
                    .add_field(i64::max_value())
                    .add_field(true)
                    .add_field(vec![String::from("Hello"), String::from("World")])
                    .create(),
            )
            .add_field(String::from("hello"))
            .create()];
        assert!(ar.signature() == "a(yu(xbas)s)");
        let v = crate::Variant::from(ar);
        assert!(v.len() == 66);

        assert!(v.is::<Vec::<Structure>>());
        let ar = v.get::<Vec<Structure>>().unwrap();
        let s = &ar[0];
        let fields = s.fields();
        assert!(fields[0].is::<u8>());
        assert!(fields[0].get::<u8>().unwrap() == u8::max_value());
        assert!(fields[1].is::<u32>());
        assert!(fields[1].get::<u32>().unwrap() == u32::max_value());

        let inner = fields[2].get::<Structure>().unwrap();
        let inner_fields = inner.fields();
        assert!(inner_fields[0].is::<i64>());
        assert!(inner_fields[0].get::<i64>().unwrap() == i64::max_value());
        assert!(inner_fields[1].is::<bool>());
        assert!(inner_fields[1].get::<bool>().unwrap() == true);
        assert!(inner_fields[2].is::<Vec::<String>>());
        let as_ = inner_fields[2].get::<Vec<String>>().unwrap();
        assert!(as_ == [String::from("Hello"), String::from("World")]);

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 66);

        assert!(v.is::<Vec::<Structure>>());
        let ar = v.get::<Vec<Structure>>().unwrap();
        let s = &ar[0];
        let fields = s.fields();
        assert!(fields[0].is::<u8>());
        assert!(fields[0].get::<u8>().unwrap() == u8::max_value());
        assert!(fields[1].is::<u32>());
        assert!(fields[1].get::<u32>().unwrap() == u32::max_value());

        let inner = fields[2].get::<Structure>().unwrap();
        let inner_fields = inner.fields();
        assert!(inner_fields[0].is::<i64>());
        assert!(inner_fields[0].get::<i64>().unwrap() == i64::max_value());
        assert!(inner_fields[1].is::<bool>());
        assert!(inner_fields[1].get::<bool>().unwrap() == true);
        assert!(inner_fields[2].is::<Vec::<String>>());
        let as_ = inner_fields[2].get::<Vec<String>>().unwrap();
        assert!(as_ == [String::from("Hello"), String::from("World")]);
    }

    #[test]
    fn dict_entry_variant() {
        // Simple type value
        let entry = DictEntry::new(2u8, String::from("world"));
        assert!(entry.signature() == "{ys}");
        let v = crate::Variant::from(entry);
        assert!(v.len() == 14);
        assert!(v.is::<DictEntry::<u8, String>>());
        let entry = v.get::<DictEntry<u8, String>>().unwrap();
        assert!(*entry.key() == 2u8);
        assert!(*entry.value() == "world");

        // STRUCT value
        let entry = DictEntry::new(
            String::from("hello"),
            StructureBuilder::new()
                .add_field(u8::max_value())
                .add_field(u32::max_value())
                .create(),
        );
        assert!(entry.signature() == "{s(yu)}");
        let v = crate::Variant::from(entry);
        assert!(v.len() == 24);
        assert!(v.is::<DictEntry::<String, Structure>>());
        let entry = v.get::<DictEntry<String, Structure>>().unwrap();
        assert!(*entry.key() == "hello");
        let s = entry.value();
        let fields = s.fields();
        assert!(fields[0].is::<u8>());
        assert!(fields[0].get::<u8>().unwrap() == u8::max_value());
        assert!(fields[1].is::<u32>());
        assert!(fields[1].get::<u32>().unwrap() == u32::max_value());
    }
}
