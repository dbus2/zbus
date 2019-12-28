use byteorder::ByteOrder;
use core::convert::TryInto;

use crate::{Decode, Encode, EncodingFormat};
use crate::{SharedData, Signature, SimpleDecode};
use crate::{Variant, VariantError};

// Since neither `From` trait nor `Vec` is from this crate, we need this intermediate type.
//
#[derive(Debug, Clone, Default)]
pub struct Array(Vec<Variant>);

impl Array {
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an `Array` from given variants.
    ///
    /// All variants must all contain the same `VariantType`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use zvariant::{Array, Encode, Variant};
    ///
    /// let variants = vec![42u8.to_variant(), 45u8.to_variant()];
    /// let array = Array::new_from_vec(variants).unwrap();
    ///
    /// let variants = vec![42u8.to_variant(), 45u32.to_variant()];
    /// assert!(Array::new_from_vec(variants).is_err());
    /// ```
    pub fn new_from_vec(vec: Vec<Variant>) -> Result<Self, VariantError> {
        // Ensure all elements are of the same type
        if let Some(first) = vec.first() {
            let first_sig = first.value_signature();

            for element in &vec[1..] {
                if element.value_signature() != first_sig {
                    return Err(VariantError::IncorrectType);
                }
            }
        }

        Ok(Array(vec))
    }

    // Caller ensures all variants in the `vec` are of the same type
    pub fn new_from_vec_unchecked(vec: Vec<Variant>) -> Self {
        Array(vec)
    }

    /// Adds the given `element` to `self`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use zvariant::{Array, Decode, Variant};
    ///
    /// let mut array = Array::new();
    /// array.add_element(42u8).unwrap();
    /// assert!(array.add_element("hi").is_err());
    /// array.add_element(45u8).unwrap();
    /// assert!(array.add_element(42u32).is_err());
    /// ```
    pub fn add_element<T: Encode>(&mut self, element: T) -> Result<(), VariantError> {
        // Ensure we only add elements of the same type
        if self.0.last().map(|v| !T::is(v)).unwrap_or(false) {
            return Err(VariantError::IncorrectType);
        }

        self.0.push(element.to_variant());

        Ok(())
    }

    pub fn inner(&self) -> &Vec<Variant> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Variant> {
        &mut self.0
    }

    pub fn take_inner(self) -> Vec<Variant> {
        self.0
    }
}

impl Encode for Array {
    const SIGNATURE_CHAR: char = 'a';
    const SIGNATURE_STR: &'static str = "a";
    const ALIGNMENT: usize = 4;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);

        let len_position = bytes.len();
        bytes.extend(&0u32.to_ne_bytes());
        let n_bytes_before = bytes.len();
        let mut first_element = true;
        let mut first_padding = 0;

        for element in self.inner() {
            if first_element {
                // Length we report doesn't include padding for the first element
                first_padding = element.value_padding(bytes.len(), format);
                first_element = false;
            }

            // Deep copying, nice!!! 🙈
            element.encode_value_into(bytes, format);
        }

        // Set size of array in bytes
        let len = crate::utils::usize_to_u32(bytes.len() - n_bytes_before - first_padding);
        byteorder::NativeEndian::write_u32(&mut bytes[len_position..len_position + 4], len);
    }

    fn signature(&self) -> Signature {
        let signature = format!("a{}", self.inner()[0].value_signature().as_str());

        Signature::from(signature)
    }

    fn to_variant(self) -> Variant {
        Variant::Array(self)
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::Array(_) = variant {
            true
        } else {
            false
        }
    }
}

impl Decode for Array {
    fn slice_data(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        let signature = Self::ensure_correct_signature(signature)?;

        // Child signature
        let child_signature = crate::decode::slice_signature(&signature[1..])?;

        // Array size in bytes
        let len_slice = u32::slice_data_simple(&data, format)?;
        let mut extracted = len_slice.len();
        let mut len = u32::decode_simple(&len_slice, format)? as usize + extracted;
        let mut first_element = true;
        while extracted < len {
            let element_data = data.tail(extracted);

            if first_element {
                // Length we got from array doesn't include padding for the first element
                len += crate::encode::padding_for_signature(
                    element_data.position(),
                    child_signature.as_str(),
                    format,
                );
                first_element = false;
            }

            let slice = crate::decode::slice_data(&element_data, child_signature.as_str(), format)?;
            extracted += slice.len();
            if extracted > len {
                return Err(VariantError::InsufficientData);
            }
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
        }

        Ok(data.head(extracted as usize))
    }

    fn decode(
        data: &SharedData,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let padding = Self::padding(data.position(), format);
        if data.len() < padding + 4 {
            return Err(VariantError::InsufficientData);
        }
        let signature = Self::ensure_correct_signature(signature)?;

        // Child signature
        let child_signature = crate::decode::slice_signature(&signature[1..])?;

        // Array size in bytes
        let mut extracted = padding + 4;
        let mut len =
            u32::decode_simple(&data.subset(padding, extracted), format)? as usize + extracted;
        let mut elements = vec![];

        let mut first_element = true;
        while extracted < len {
            let element_data = data.tail(extracted);

            if first_element {
                // Length we got from array doesn't include padding for the first element
                len += crate::encode::padding_for_signature(
                    element_data.position(),
                    child_signature.as_str(),
                    format,
                );
                first_element = false;
            }

            let slice = crate::decode::slice_data(&element_data, child_signature.as_str(), format)?;
            extracted += slice.len();
            if extracted > len {
                return Err(VariantError::InsufficientData);
            }

            let element = Variant::from_data(&slice, child_signature.as_str(), format)?;
            elements.push(element);
        }
        if extracted == 0 {
            return Err(VariantError::ExcessData);
        }

        // Not using Array::new_from_vec() as that will entail redundant (in this context) type
        // checks
        Ok(Array::new_from_vec_unchecked(elements))
    }

    fn ensure_correct_signature(
        signature: impl Into<Signature>,
    ) -> Result<Signature, VariantError> {
        let signature = signature.into();
        let len = signature.len();
        let slice = Self::slice_signature(signature)?;
        if slice.len() != len {
            return Err(VariantError::IncorrectType);
        }

        Ok(slice)
    }

    fn slice_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError> {
        let signature = signature.into();
        if signature.len() < 2 {
            return Err(VariantError::InsufficientData);
        }
        if !signature.starts_with('a') {
            return Err(VariantError::IncorrectType);
        }

        // There should be a valid complete signature after 'a' but not more than 1
        let slice = crate::decode::slice_signature(&signature[1..])?;

        Ok(Signature::from(&signature[0..=slice.len()]))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::Array(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::Array(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl<T> TryInto<Vec<T>> for Array
where
    T: Decode,
{
    type Error = VariantError;

    fn try_into(self) -> Result<Vec<T>, VariantError> {
        let mut v = vec![];

        for value in self.take_inner() {
            v.push(T::take_from_variant(value)?);
        }

        Ok(v)
    }
}

impl<T> From<Vec<T>> for Array
where
    T: Encode,
{
    fn from(values: Vec<T>) -> Self {
        let v = values.into_iter().map(|value| value.to_variant()).collect();

        Array::new_from_vec_unchecked(v)
    }
}

impl From<crate::Dict> for Array {
    fn from(value: crate::Dict) -> Self {
        Array::from(value.take_inner())
    }
}
