use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::BuildHasher;

use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};

use crate::{Basic, Error, Signature};
use crate::{Type, Value};

/// A helper type to wrap dictionaries in a [`Value`].
///
/// API is provided to convert from, and to a [`HashMap`].
///
/// [`Value`]: enum.Value.html#variant.Dict
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
#[derive(Debug, Clone, PartialEq)]
pub struct Dict<'k, 'v> {
    entries: Vec<DictEntry<'k, 'v>>,
    key_signature: Signature<'k>,
    value_signature: Signature<'v>,
}

impl<'k, 'v> Dict<'k, 'v> {
    /// Create a new empty `Dict`, given the signature of the keys and values.
    pub fn new(key_signature: Signature<'k>, value_signature: Signature<'v>) -> Self {
        Self {
            entries: vec![],
            key_signature,
            value_signature,
        }
    }

    /// Append `key` and `value` as a new entry.
    ///
    /// # Errors
    ///
    /// * if [`key.value_signature()`] doesn't match the key signature `self` was created for.
    /// * if [`value.value_signature()`] doesn't match the value signature `self` was created for.
    ///
    /// [`key.value_signature()`]: enum.Value.html#method.value_signature
    /// [`value.value_signature()`]: enum.Value.html#method.value_signature
    pub fn append<'kv: 'k, 'vv: 'v>(
        &mut self,
        key: Value<'kv>,
        value: Value<'vv>,
    ) -> Result<(), Error> {
        check_child_value_signature!(self.key_signature, key.value_signature(), "key");
        check_child_value_signature!(self.value_signature, value.value_signature(), "value");

        self.entries.push(DictEntry { key, value });

        Ok(())
    }

    /// Add a new entry.
    pub fn add<K, V>(&mut self, key: K, value: V) -> Result<(), Error>
    where
        K: Basic + Into<Value<'k>> + std::hash::Hash + std::cmp::Eq,
        V: Into<Value<'v>> + Type,
    {
        check_child_value_signature!(self.key_signature, K::signature(), "key");
        check_child_value_signature!(self.value_signature, V::signature(), "value");

        self.entries.push(DictEntry {
            key: Value::new(key),
            value: Value::new(value),
        });

        Ok(())
    }

    /// Get the value for the given key.
    pub fn get<'d, K, V>(&'d self, key: &K) -> Result<Option<&'v V>, Error>
    where
        'd: 'k + 'v,
        K: ?Sized + std::cmp::Eq + 'k,
        V: ?Sized,
        &'k K: TryFrom<&'k Value<'k>>,
        &'v V: TryFrom<&'v Value<'v>>,
    {
        for entry in &self.entries {
            let entry_key = entry.key.downcast_ref::<K>().ok_or(Error::IncorrectType)?;
            if *entry_key == *key {
                return entry
                    .value
                    .downcast_ref()
                    .ok_or(Error::IncorrectType)
                    .map(Some);
            }
        }

        Ok(None)
    }

    /// Get the signature of this `Dict`.
    pub fn signature(&self) -> Signature<'static> {
        Signature::from_string_unchecked(format!(
            "a{{{}{}}}",
            self.key_signature, self.value_signature,
        ))
    }

    pub(crate) fn to_owned(&self) -> Dict<'static, 'static> {
        Dict {
            key_signature: self.key_signature.to_owned(),
            value_signature: self.value_signature.to_owned(),
            entries: self.entries.iter().map(|v| v.to_owned()).collect(),
        }
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
    K: Basic + TryFrom<Value<'k>> + std::hash::Hash + std::cmp::Eq,
    V: TryFrom<Value<'v>>,
    H: BuildHasher + Default,
{
    type Error = Error;

    fn try_from(v: Dict<'k, 'v>) -> Result<Self, Self::Error> {
        let mut map = HashMap::default();
        for e in v.entries.into_iter() {
            let key = e.key.downcast().ok_or(Error::IncorrectType)?;
            let value = e.value.downcast().ok_or(Error::IncorrectType)?;

            map.insert(key, value);
        }
        Ok(map)
    }
}

// TODO: this could be useful
// impl<'d, 'k, 'v, K, V, H> TryFrom<&'d Dict<'k, 'v>> for HashMap<&'k K, &'v V, H>

// Conversion of Hashmap to Dict
impl<'k, 'v, K, V> From<HashMap<K, V>> for Dict<'k, 'v>
where
    K: Type + Into<Value<'k>> + std::hash::Hash + std::cmp::Eq,
    V: Type + Into<Value<'v>>,
{
    fn from(value: HashMap<K, V>) -> Self {
        let entries = value
            .into_iter()
            .map(|(key, value)| DictEntry {
                key: Value::new(key),
                value: Value::new(value),
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

#[derive(Debug, Clone, PartialEq)]
struct DictEntry<'k, 'v> {
    key: Value<'k>,
    value: Value<'v>,
}

impl<'k, 'v> DictEntry<'k, 'v> {
    fn to_owned(&self) -> DictEntry<'static, 'static> {
        DictEntry {
            key: self.key.to_owned(),
            value: self.value.to_owned(),
        }
    }
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
