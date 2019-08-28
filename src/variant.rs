use std::borrow::Cow;
use std::str;

use crate::{Signature, SimpleVariantType, VariantError, VariantType};

#[derive(Debug)]
pub struct Variant<'a> {
    signature: String,
    value: Cow<'a, [u8]>,
}

impl<'a> Variant<'a> {
    pub fn from_data(data: &'a [u8], signature: &str) -> Result<Self, VariantError> {
        // slice_data() ensures a valid signature
        let value = crate::variant_type::slice_data(data, signature, 0)?;

        Ok(Self {
            value: Cow::from(value),
            signature: String::from(signature),
        })
    }

    pub fn from<T: 'a + VariantType<'a>>(value: T) -> Self {
        Self {
            value: Cow::from(value.encode(0)),
            signature: String::from(value.signature()),
        }
    }

    pub fn signature(&self) -> &str {
        &self.signature
    }

    pub fn get<T: 'a + VariantType<'a>>(&'a self) -> Result<T, VariantError> {
        T::decode(&self.value, &self.signature, 0)
    }

    pub fn bytes(&self) -> &[u8] {
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
    /// let v = zbus::Variant::from("hello");
    /// assert!(!v.is::<u32>());
    /// assert!(v.is::<(&str)>());
    /// ```
    ///
    /// ```
    /// let v = zbus::Variant::from(147u32);
    /// assert!(!v.is::<(&str)>());
    /// assert!(v.is::<u32>());
    /// ```
    pub fn is<T: 'a + VariantType<'a>>(&self) -> bool {
        self.signature.starts_with(T::SIGNATURE_STR)
    }

    // Should this be part of public API?
    pub(crate) fn inner_alignment(&self) -> usize {
        // Constructors ensure that we always have a valid `signature` so `unwrap()` should be fine here.
        crate::variant_type::alignment_for_signature(&self.signature).unwrap()
    }
}

impl<'a> VariantType<'a> for Variant<'a> {
    const SIGNATURE: char = 'v';
    const SIGNATURE_STR: &'static str = "v";
    const ALIGNMENT: usize = Signature::ALIGNMENT;

    // Like Signature, no padding needed because of 1-byte alignment and hence n_bytes_before is ignored everywhere.

    fn encode(&self, _n_bytes_before: usize) -> Vec<u8> {
        let mut bytes = Signature::new(&self.signature).encode(0);
        bytes.extend_from_slice(&self.value);

        bytes
    }

    fn slice_data<'b>(
        bytes: &'b [u8],
        signature: &str,
        _n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError> {
        Self::ensure_correct_signature(signature)?;

        // Variant is made of signature of the value followed by the actual value. So we gotta
        // extract the signature slice first and then the value slice. Once we know the sizes of
        // both, we can just slice the whole thing.
        let sign_slice = Signature::slice_data_simple(bytes, 0)?;
        let sign_size = sign_slice.len();
        let sign = Signature::decode_simple(sign_slice, 0)?;

        let value_slice = crate::variant_type::slice_data(&bytes[sign_size..], sign.as_str(), 0)?;
        let total_size = sign_size + value_slice.len();

        Ok(&bytes[0..total_size])
    }

    fn decode(
        bytes: &'a [u8],
        signature: &str,
        _n_bytes_before: usize,
    ) -> Result<Self, VariantError> {
        Self::ensure_correct_signature(signature)?;

        let sign_slice = Signature::slice_data_simple(bytes, 0)?;
        let sign_size = sign_slice.len();
        let sign = Signature::decode_simple(sign_slice, 0)?;

        Variant::from_data(&bytes[sign_size..], sign.as_str())
    }
}
impl<'a> SimpleVariantType<'a> for Variant<'a> {}

#[cfg(test)]
mod tests {
    use crate::{SimpleVariantType, Structure, VariantType};

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
        let v = crate::Variant::from("Hello world!");
        assert!(v.len() == 17);
        assert!(v.get::<(&str)>().unwrap() == "Hello world!");
        assert!(v.is::<(&str)>());

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 17);
        assert!(v.get::<(&str)>().unwrap() == "Hello world!");
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
        // pass an arbitrary odd number and encoding shouldn't have any padding (i-e we should get
        // 4 bytes only).
        let mut encoded = v.encode(3);
        assert!(encoded.len() == 4);

        // Add some extra bytes to the encoded data to test the slicing
        encoded.push(0);
        encoded.push(1);
        encoded.push(7);

        let slice = crate::Variant::slice_data_simple(&encoded, 0).unwrap();

        let decoded = crate::Variant::decode_simple(slice, 0).unwrap();
        assert!(decoded.signature() == u8::SIGNATURE_STR);
        assert!(decoded.get::<u8>().unwrap() == 7u8);
    }

    #[test]
    fn struct_variant() {
        let s = Structure::new(vec![
            crate::Variant::from(u8::max_value()),
            crate::Variant::from(u32::max_value()),
            crate::Variant::from(Structure::new(vec![
                crate::Variant::from(i64::max_value()),
                crate::Variant::from(true),
                crate::Variant::from(Structure::new(vec![
                    crate::Variant::from(i64::max_value()),
                    crate::Variant::from(std::f64::MAX),
                ])),
            ])),
            crate::Variant::from("hello"),
        ]);
        let v = crate::Variant::from(s);
        assert!(v.len() == 50);

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

        assert!(fields[3].is::<&str>());
        assert!(fields[3].get::<&str>().unwrap() == "hello");

        let v = crate::Variant::from_data(v.bytes(), v.signature()).unwrap();
        assert!(v.len() == 50);

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

        assert!(fields[3].get::<&str>().unwrap() == "hello");
        assert!(fields[3].is::<&str>());
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
        let as_ = vec!["Hello", "World", "Now", "Bye!"];
        assert!(as_.signature() == "as");
        let v = crate::Variant::from(as_);
        assert!(v.len() == 45);
        let v = v.get::<Vec<&str>>().unwrap();
        assert!(v == ["Hello", "World", "Now", "Bye!"]);

        // Array of Struct, which in turn containin an Array (We gotta go deeper!)
        let ar = vec![Structure::new(vec![
            crate::Variant::from(u8::max_value()),
            crate::Variant::from(u32::max_value()),
            crate::Variant::from(Structure::new(vec![
                crate::Variant::from(i64::max_value()),
                crate::Variant::from(true),
                crate::Variant::from(vec!["Hello", "World"]),
            ])),
            crate::Variant::from("hello"),
        ])];
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
        assert!(inner_fields[2].is::<Vec::<&str>>());
        let as_ = inner_fields[2].get::<Vec<&str>>().unwrap();
        assert!(as_ == ["Hello", "World"]);

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
        assert!(inner_fields[2].is::<Vec::<&str>>());
        let as_ = inner_fields[2].get::<Vec<&str>>().unwrap();
        assert!(as_ == ["Hello", "World"]);
    }
}
