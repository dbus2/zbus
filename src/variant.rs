use std::borrow::Cow;
use std::str;

use crate::{ObjectPath, Signature, VariantError, VariantType};

pub struct Variant<'a> {
    signature: String,
    value: Cow<'a, [u8]>,
}

impl<'a> Variant<'a> {
    pub fn from_data(data: &'a [u8], signature: &str) -> Result<Self, VariantError>
    where
        Self: 'a,
    {
        let value = match signature {
            // FIXME: There has to be a shorter way to do this
            u32::SIGNATURE_STR => u32::extract_slice(data)?,
            <(&str)>::SIGNATURE_STR => <(&str)>::extract_slice(data)?,
            ObjectPath::SIGNATURE_STR => ObjectPath::extract_slice(data)?,
            Signature::SIGNATURE_STR => Signature::extract_slice(data)?,
            _ => return Err(VariantError::UnsupportedType),
        };

        Ok(Self {
            value: Cow::from(value),
            signature: String::from(signature),
        })
    }

    pub fn from<T: 'a + VariantType<'a>>(value: T) -> Self
    where
        Self: 'a,
    {
        Self {
            value: Cow::from(value.encode()),
            signature: String::from(T::SIGNATURE_STR),
        }
    }

    pub fn get_signature(&self) -> &str {
        &self.signature
    }

    pub fn get<T: 'a + VariantType<'a>>(&'a self) -> Result<T, VariantError> {
        VariantType::extract(&self.value)
    }

    // FIXME: Return a slice
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
