use std::convert::TryFrom;

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{Basic, Signature, Variant};

// TODO: This type can't be used indepentently now that Serializer requires VariantValue trait and
// we can't implement VariantValue on this type since signature is dynamic.

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
/// TODO
#[derive(Debug, Clone)]
pub struct DictEntry<'a, 'b> {
    key: Box<Variant<'a>>,
    value: Box<Variant<'b>>,
}

impl<'a, 'b> DictEntry<'a, 'b> {
    /// Create a new `DictEntry`.
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Basic + Into<Variant<'a>> + std::hash::Hash + std::cmp::Eq,
        V: Into<Variant<'b>>,
    {
        Self {
            key: Box::new(key.into()),
            value: Box::new(value.into()),
        }
    }

    /// Get a reference to the key.
    pub fn key<K>(&'a self) -> Result<K, K::Error>
    where
        K: TryFrom<&'a Variant<'a>> + std::hash::Hash + std::cmp::Eq,
    {
        K::try_from(&self.key)
    }

    /// Get a reference to the value.
    pub fn value<V>(&'b self) -> Result<V, V::Error>
    where
        V: TryFrom<&'b Variant<'b>>,
    {
        V::try_from(&self.value)
    }

    pub fn signature(&self) -> Signature<'static> {
        Signature::from(format!(
            "{{{}{}}}",
            self.key.value_signature().as_str(),
            self.value.value_signature().as_str(),
        ))
    }
}

impl<'a, 'b> Serialize for DictEntry<'a, 'b> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut entry = serializer.serialize_struct("zvariant::DictEntry", 2)?;
        self.key
            .serialize_value_as_struct_field("zvariant::DictEntry::Key", &mut entry)?;
        self.value
            .serialize_value_as_struct_field("zvariant::DictEntry::Value", &mut entry)?;

        entry.end()
    }
}
