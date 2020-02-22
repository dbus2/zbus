use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::BuildHasher;

use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

use crate::{Basic, Error, Signature, Variant, VariantValue};

/// A dictionary type to be used with [`Variant`].
///
/// TODO
///
/// [`Variant`]: enum.Variant.html
#[derive(Debug, Clone)]
pub struct Dict<'k, 'v> {
    entries: Vec<DictEntry<'k, 'v>>,
    key_signature: Signature<'k>,
    value_signature: Signature<'v>,
}

impl<'k, 'v> Dict<'k, 'v> {
    pub fn new(key_signature: &Signature<'k>, value_signature: &Signature<'v>) -> Self {
        Self {
            entries: vec![],
            key_signature: key_signature.clone(),
            value_signature: value_signature.clone(),
        }
    }

    /// Add a new entry.
    pub fn add<K, V>(&mut self, key: K, value: V) -> Result<(), Error>
    where
        K: Basic + Into<Variant<'k>> + std::hash::Hash + std::cmp::Eq,
        V: Into<Variant<'v>> + VariantValue,
    {
        if K::signature() != self.key_signature || V::signature() != self.value_signature {
            return Err(Error::IncorrectType);
        }

        self.entries.push(DictEntry {
            key: key.into(),
            value: value.into(),
        });

        Ok(())
    }

    /// Get the value for the given key.
    pub fn get<'d, K, V>(&'d self, key: &'k K) -> Result<Option<&'v V>, Error>
    where
        'd: 'k + 'v,
        &'k K: TryFrom<&'k Variant<'k>, Error = Error>,
        K: std::cmp::Eq,
        &'v V: TryFrom<&'v Variant<'v>, Error = Error>,
    {
        for entry in &self.entries {
            let k = <&K>::try_from(&entry.key)?;
            if *k == *key {
                return <&V>::try_from(&entry.value).map(Some);
            }
        }

        Ok(None)
    }

    pub fn signature(&self) -> Signature<'static> {
        Signature::from(format!(
            "a{{{}{}}}",
            self.key_signature.as_str(),
            self.value_signature.as_str(),
        ))
    }

    // TODO: Provide more API like https://docs.rs/toml/0.5.5/toml/map/struct.Map.html
}

impl<'k, 'v> Serialize for Dict<'k, 'v> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.entries.len()))?;
        for entry in &self.entries {
            seq.serialize_element(entry)?;
        }

        seq.end()
    }
}

// Conversion of Dict to HashMap
impl<'k, 'v, K, V, H> TryFrom<Dict<'k, 'v>> for HashMap<K, V, H>
where
    K: Basic + TryFrom<Variant<'k>, Error = Error> + std::hash::Hash + std::cmp::Eq,
    V: TryFrom<Variant<'v>, Error = Error>,
    H: BuildHasher + Default,
{
    type Error = Error;

    fn try_from(value: Dict<'k, 'v>) -> Result<HashMap<K, V, H>, Error> {
        let mut map = HashMap::default();

        for entry in value.entries {
            let (key, value) = (K::try_from(entry.key)?, V::try_from(entry.value)?);

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'d, 'k, 'v, K, V, H> TryFrom<&'d Dict<'k, 'v>> for HashMap<&'k K, &'v V, H>
where
    'd: 'k + 'v,
    &'k K: TryFrom<&'k Variant<'k>, Error = Error>,
    K: std::cmp::Eq + std::hash::Hash,
    &'v V: TryFrom<&'v Variant<'v>, Error = Error>,
    H: BuildHasher + Default,
{
    type Error = Error;

    fn try_from(value: &'d Dict<'k, 'v>) -> Result<HashMap<&'k K, &'v V, H>, Error> {
        let mut map = HashMap::default();

        for entry in &value.entries {
            let (key, value) = (<&K>::try_from(&entry.key)?, <&V>::try_from(&entry.value)?);

            map.insert(key, value);
        }

        Ok(map)
    }
}

// Conversion of Hashmap to Dict
impl<'k, 'v, K, V> From<HashMap<K, V>> for Dict<'k, 'v>
where
    K: VariantValue + Into<Variant<'k>> + std::hash::Hash + std::cmp::Eq,
    V: VariantValue + Into<Variant<'v>>,
{
    fn from(value: HashMap<K, V>) -> Self {
        let entries = value
            .into_iter()
            .map(|(key, value)| DictEntry {
                key: key.into(),
                value: value.into(),
            })
            .collect();

        Self {
            entries,
            key_signature: K::signature(),
            value_signature: V::signature(),
        }
    }
}

// TODO: Conversion of Dict from/to BTreeMap

#[derive(Debug, Clone)]
struct DictEntry<'k, 'v> {
    key: Variant<'k>,
    value: Variant<'v>,
}

impl<'k, 'v> Serialize for DictEntry<'k, 'v> {
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
