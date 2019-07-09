use std::borrow::Cow;
use std::str;

use crate::{ObjectPath, Signature, VariantError, VariantType};

pub struct Variant<'a> {
    signature: Cow<'a, str>,
    value: Cow<'a, [u8]>,
}

impl<'a> Variant<'a> {
    pub fn from_data(data: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        let value = extract_slice_from_data(data, signature)?;

        Ok(Self {
            value: Cow::from(value),
            signature: Cow::from(String::from(signature)),
        })
    }

    pub fn from<T: 'a + VariantType<'a>>(value: T) -> Self
    where
        Self: 'a,
    {
        Self {
            value: Cow::from(value.encode()),
            signature: Cow::from(T::SIGNATURE_STR),
        }
    }

    pub fn get_signature(&self) -> &str {
        &self.signature
    }

    pub fn get<T: 'a + VariantType<'a>>(&'a self) -> Result<T, VariantError> {
        VariantType::decode(&self.value)
    }

    pub fn get_bytes(&self) -> &[u8] {
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
        T::SIGNATURE_STR == self.signature
    }
}

impl<'a> VariantType<'a> for Variant<'a> {
    const SIGNATURE: char = 'v';
    const SIGNATURE_STR: &'static str = "v";

    fn encode(&self) -> Vec<u8> {
        let mut bytes = Signature::new(&self.signature).encode();
        bytes.extend_from_slice(&self.value);

        bytes
    }

    fn extract_slice(bytes: &'a [u8]) -> Result<&'a [u8], VariantError> {
        // Variant is made of signature of the value followed by the actual value. So we gotta
        // extract the signature slice first and then the value slice. Once we know the sizes of
        // both, we can just slice the whole thing.
        let sign_slice = Signature::extract_slice(bytes)?;
        let sign_size = sign_slice.len();
        let sign = Signature::decode(sign_slice)?;

        let value_slice = extract_slice_from_data(&bytes[sign_size..], sign.as_str())?;
        let total_size = sign_size + value_slice.len();

        Ok(&bytes[0..total_size])
    }

    fn decode(bytes: &'a [u8]) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        let sign_slice = Signature::extract_slice(bytes)?;
        let sign_size = sign_slice.len();
        let sign = Signature::decode(sign_slice)?;

        Variant::from_data(&bytes[sign_size..], sign.as_str())
    }
}

fn extract_slice_from_data<'a>(data: &'a [u8], signature: &str) -> Result<&'a [u8], VariantError> {
    match signature {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_STR => u8::extract_slice(data),
        bool::SIGNATURE_STR => bool::extract_slice(data),
        i16::SIGNATURE_STR => i16::extract_slice(data),
        u16::SIGNATURE_STR => u16::extract_slice(data),
        i32::SIGNATURE_STR => i32::extract_slice(data),
        u32::SIGNATURE_STR => u32::extract_slice(data),
        i64::SIGNATURE_STR => i64::extract_slice(data),
        u64::SIGNATURE_STR => u64::extract_slice(data),
        f64::SIGNATURE_STR => f64::extract_slice(data),
        <(&str)>::SIGNATURE_STR => <(&str)>::extract_slice(data),
        ObjectPath::SIGNATURE_STR => ObjectPath::extract_slice(data),
        Signature::SIGNATURE_STR => Signature::extract_slice(data),
        _ => return Err(VariantError::UnsupportedType),
    }
}

#[cfg(test)]
mod tests {
    use crate::VariantType;

    #[test]
    fn u8_variant() {
        let v = crate::Variant::from(u8::max_value());
        assert!(v.len() == 1);
        assert!(v.get::<u8>().unwrap() == u8::max_value());
        assert!(v.is::<u8>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 1);
        assert!(v.get::<u8>().unwrap() == u8::max_value());
    }

    #[test]
    fn bool_variant() {
        let v = crate::Variant::from(true);
        assert!(v.len() == 4);
        assert!(v.get::<bool>().unwrap());
        assert!(v.is::<bool>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 4);
        assert!(v.get::<bool>().unwrap());
    }

    #[test]
    fn i16_variant() {
        let v = crate::Variant::from(i16::max_value());
        assert!(v.len() == 2);
        assert!(v.get::<i16>().unwrap() == i16::max_value());
        assert!(v.is::<i16>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 2);
        assert!(v.get::<i16>().unwrap() == i16::max_value());
    }

    #[test]
    fn u16_variant() {
        let v = crate::Variant::from(u16::max_value());
        assert!(v.len() == 2);
        assert!(v.get::<u16>().unwrap() == u16::max_value());
        assert!(v.is::<u16>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 2);
        assert!(v.get::<u16>().unwrap() == u16::max_value());
    }

    #[test]
    fn i32_variant() {
        let v = crate::Variant::from(i32::max_value());
        assert!(v.len() == 4);
        assert!(v.get::<i32>().unwrap() == i32::max_value());
        assert!(v.is::<i32>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 4);
        assert!(v.get::<i32>().unwrap() == i32::max_value());
    }

    #[test]
    fn u32_variant() {
        let v = crate::Variant::from(u32::max_value());
        assert!(v.len() == 4);
        assert!(v.get::<u32>().unwrap() == u32::max_value());
        assert!(v.is::<u32>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 4);
        assert!(v.get::<u32>().unwrap() == u32::max_value());
    }

    #[test]
    fn i64_variant() {
        let v = crate::Variant::from(i64::max_value());
        assert!(v.len() == 8);
        assert!(v.get::<i64>().unwrap() == i64::max_value());
        assert!(v.is::<i64>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 8);
        assert!(v.get::<i64>().unwrap() == i64::max_value());
    }

    #[test]
    fn u64_variant() {
        let v = crate::Variant::from(u64::max_value());
        assert!(v.len() == 8);
        assert!(v.get::<u64>().unwrap() == u64::max_value());
        assert!(v.is::<u64>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 8);
        assert!(v.get::<u64>().unwrap() == u64::max_value());
    }

    #[test]
    fn f64_variant() {
        let v = crate::Variant::from(117.112f64);
        assert!(v.len() == 8);
        assert!(v.get::<f64>().unwrap() == 117.112);
        assert!(v.is::<f64>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 8);
        assert!(v.get::<f64>().unwrap() == 117.112);
    }

    #[test]
    fn str_variant() {
        let v = crate::Variant::from("Hello world!");
        assert!(v.len() == 17);
        assert!(v.get::<(&str)>().unwrap() == "Hello world!");
        assert!(v.is::<(&str)>());

        let v = crate::Variant::from_data(v.get_bytes(), v.get_signature()).unwrap();
        assert!(v.len() == 17);
        assert!(v.get::<(&str)>().unwrap() == "Hello world!");
    }

    #[test]
    fn variant_variant() {
        let v = crate::Variant::from(7u8);
        let mut encoded = v.encode();
        assert!(encoded.len() == 4);

        // Add some extra bytes to the encoded data to test the slicing
        encoded.push(0);
        encoded.push(1);
        encoded.push(7);

        let slice = crate::Variant::extract_slice(&encoded).unwrap();

        let decoded = crate::Variant::decode(slice).unwrap();
        assert!(decoded.get_signature() == u8::SIGNATURE_STR);
        assert!(decoded.get::<u8>().unwrap() == 7u8);
    }
}
