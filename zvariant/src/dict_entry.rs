use crate::{Basic, Decode, Encode, EncodingFormat};
use crate::{SharedData, Signature};
use crate::{Variant, VariantError};

/// A dictionary entry as a key-value pair.
///
/// This is not very useful on its own but since D-Bus defines it as its own type, a hashmap in
/// D-Bus is encoded as an array of dictionary entries and [GVariant] even allows this to be
/// used outside of an array, we provide this data type.
///
/// The key must be a [basic data type].
///
/// [GVariant]: https://developer.gnome.org/glib/stable/glib-GVariant.html
/// [basic data type]: trait.Basic.html
///
/// # Example:
///
/// ```
/// use zvariant::{Decode, DictEntry};
/// use zvariant::{Encode, EncodingFormat, Structure};
///
/// let entry = DictEntry::new(
///     // String key
///     "hello",
///     // Structure value
///     Structure::new()
///         .add_field(u8::max_value())
///         .add_field(u32::max_value()),
/// );
/// assert!(entry.signature() == "{s(yu)}");
///
/// let format = EncodingFormat::default();
/// let encoding = entry.encode(format);
/// assert!(encoding.len() == 24);
///
/// let entry = DictEntry::decode(encoding, entry.signature(), format).unwrap();
/// assert!(entry.key::<String>().unwrap() == "hello");
/// let structure = entry.value::<Structure>().unwrap();
/// let fields = structure.fields();
/// assert!(u8::is(&fields[0]));
/// assert!(*u8::from_variant(&fields[0]).unwrap() == u8::max_value());
/// assert!(u32::is(&fields[1]));
/// assert!(*u32::from_variant(&fields[1]).unwrap() == u32::max_value());
/// ```
#[derive(Debug, Clone)]
pub struct DictEntry {
    key: Box<Variant>,
    value: Box<Variant>,
}

impl DictEntry {
    /// Create a new `DictEntry`
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Encode + Basic,
        V: Encode,
    {
        Self {
            key: Box::new(key.to_variant()),
            value: Box::new(value.to_variant()),
        }
    }

    /// Get a reference to the key.
    pub fn key<K>(&self) -> Result<&K, VariantError>
    where
        K: Decode + Basic,
    {
        K::from_variant(&self.key)
    }

    /// Get a reference to the value.
    pub fn value<V>(&self) -> Result<&V, VariantError>
    where
        V: Decode,
    {
        V::from_variant(&self.value)
    }

    /// Take the key and value, consuming `self`.
    pub fn take<K, V>(self) -> Result<(K, V), VariantError>
    where
        K: Decode + Basic,
        V: Decode,
    {
        Ok((
            K::take_from_variant(*self.key)?,
            V::take_from_variant(*self.value)?,
        ))
    }
}

impl Encode for DictEntry {
    // The real single character signature for DICT_ENTRY is `e` but that's not actually used in practice for D-Bus at
    // least (the spec clearly states that this signature must never appear on the bus). The openning and closing curly
    // braces are used in practice and that's why we'll declare the opening curly brace as the signature for this type.
    const SIGNATURE_CHAR: char = '{';
    const SIGNATURE_STR: &'static str = "{";
    const ALIGNMENT: usize = 8;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);

        self.key.encode_value_into(bytes, format);
        self.value.encode_value_into(bytes, format);
    }

    fn signature(&self) -> Signature {
        Signature::from(format!(
            "{{{}{}}}",
            self.key.value_signature().as_str(),
            self.value.value_signature().as_str(),
        ))
    }

    fn to_variant(self) -> Variant {
        Variant::DictEntry(self)
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::DictEntry(_) = variant {
            true
        } else {
            false
        }
    }
}

impl Decode for DictEntry {
    fn slice_data(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        let data = data.into();
        let padding = Self::padding(data.position(), format);
        if data.len() < padding {
            return Err(VariantError::InsufficientData);
        }
        let signature = Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        // Key's signature will always be just 1 character so no need to slice for that.
        let key_signature = &signature[1..2];
        let key_slice =
            crate::decode::slice_data(data.tail(extracted as usize), key_signature, format)?;
        extracted += key_slice.len();

        let value_signature = crate::decode::slice_signature(&signature[2..])?;
        let value_slice =
            crate::decode::slice_data(data.tail(extracted as usize), value_signature, format)?;
        extracted += value_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }

        Ok(data.head(extracted))
    }

    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        // Similar to slice_data, except we create variants.
        let data = data.into();
        let padding = Self::padding(data.position(), format);
        if data.len() < padding {
            return Err(VariantError::InsufficientData);
        }
        let signature = Self::ensure_correct_signature(signature)?;

        let mut extracted = padding;
        // Key's signature will always be just 1 character so no need to slice for that.
        let key_signature = &signature[1..2];
        let key_slice =
            crate::decode::slice_data(data.tail(extracted as usize), key_signature, format)?;
        extracted += key_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }
        let key = Variant::from_data(key_slice, key_signature, format)?;

        let value_signature = crate::decode::slice_signature(&signature[2..])?;
        let value_slice = crate::decode::slice_data(
            data.tail(extracted as usize),
            value_signature.as_str(),
            format,
        )?;
        extracted += value_slice.len();
        if extracted > data.len() {
            return Err(VariantError::InsufficientData);
        }
        let value = Variant::from_data_slice(value_slice, value_signature, format)?;

        Ok(Self {
            key: Box::new(key),
            value: Box::new(value),
        })
    }

    // Kept independent of K and V so that it can be used from generic code
    fn ensure_correct_signature(
        signature: impl Into<Signature>,
    ) -> Result<Signature, VariantError> {
        let signature = signature.into();
        if !signature.starts_with('{') || !signature.ends_with('}') {
            return Err(VariantError::IncorrectType);
        }
        if signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        // Don't need the alignments but no errors here means we've valid signatures
        let _ = crate::alignment_for_signature(&signature[1..2])?;
        let value_signature = crate::decode::slice_signature(&signature[2..])?;
        let _ = crate::alignment_for_signature(value_signature)?;

        Ok(signature)
    }

    // Kept independent of K and V so that it can be used from generic code
    fn slice_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError> {
        let signature = signature.into();

        if !signature.starts_with('{') {
            return Err(VariantError::IncorrectType);
        }
        if signature.len() < 4 {
            return Err(VariantError::InsufficientData);
        }

        // Key's signature will always be just 1 character so no need to slice for that.
        // There should be one valid complete signature for value.
        let slice = crate::decode::slice_signature(&signature[2..])?;

        // signature of value + `{` + 1 char of the key signature + `}`
        Ok((&signature[0..slice.len() + 3]).into())
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::DictEntry(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::DictEntry(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}
