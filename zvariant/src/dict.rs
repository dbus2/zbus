use core::convert::{TryFrom, TryInto};
use std::collections::HashMap;

use crate::VariantError;
use crate::{Array, Basic, Decode, DictEntry, Encode};

/// A dictionary as an [`Array`] of [`DictEntry`].
///
/// It would have been best to implement [`From`]`<`[`Vec`]`<`[`DictEntry`]`>>` for [`HashMap`]
/// but since both [`From`] trait and [`HashMap`] are external to our crate, we need this
/// intermediate type. We can't implement [`Into`] either as [`Vec`] isn't our type either.
/// API is provided to transform this into, and from a [`HashMap`] though.
///
/// # Example:
///
/// ```
/// use core::convert::{TryFrom, TryInto};
/// use std::collections::HashMap;
///
/// use zvariant::{Array, Dict};
/// use zvariant::{Decode, Encode, EncodingFormat};
///
/// // Create a Dict from a HashMap
/// let mut map: HashMap<i64, &str> = HashMap::new();
/// map.insert(1, "123");
/// map.insert(2, "456");
/// let dict: Dict = map.into();
///
/// // Then we turn it into an Array so we can encode it
/// let array = Array::try_from(dict).unwrap();
/// let format = EncodingFormat::default();
/// let encoding = array.encode(format);
/// assert!(encoding.len() == 40);
///
/// // Then we do the opposite
/// let array = Array::decode(encoding, array.signature(), format).unwrap();
/// let dict = Dict::try_from(array).unwrap();
/// let map: HashMap<i64, String> = dict.try_into().unwrap();
///
/// // Check we got the right thing back
/// assert!(map[&1] == "123");
/// assert!(map[&2] == "456");
/// ```
///
/// [`Array`]: struct.Array.html
/// [`DictEntry`]: struct.DictEntry.html
/// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [`Into`]: https://doc.rust-lang.org/std/convert/trait.Into.html
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
#[derive(Default)]
pub struct Dict(Vec<DictEntry>);

impl Dict {
    /// Create a new `Dict`.
    ///
    /// Same as calling `Dict::default()`.
    pub fn new() -> Self {
        Dict::default()
    }

    /// Create a new `Dict` from `vec`.
    ///
    /// Same as calling `Dict::from(vec)`.
    pub fn new_from_vec(vec: Vec<DictEntry>) -> Self {
        Dict(vec)
    }

    /// Get a reference to the underlying `Vec<DictEntry>`.
    pub fn get(&self) -> &Vec<DictEntry> {
        &self.0
    }

    /// Get a mutable reference to the underlying `Vec<DictEntry>`.
    pub fn get_mut(&mut self) -> &mut Vec<DictEntry> {
        &mut self.0
    }

    /// Unwraps the `Dict`, returning the underlying `Vec<DictEntry>`.
    pub fn into_inner(self) -> Vec<DictEntry> {
        self.0
    }
}

// Conversion of Dict to HashMap
impl<K, V> TryInto<HashMap<K, V>> for Dict
where
    K: Decode + Basic,
    V: Decode,
{
    type Error = VariantError;

    fn try_into(self) -> Result<HashMap<K, V>, VariantError> {
        let mut map = HashMap::new();

        for entry in self.0 {
            let (key, value) = entry.take()?;

            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<'a, K, V> TryInto<HashMap<&'a K, &'a V>> for &'a Dict
where
    K: Decode + Basic,
    V: Decode,
{
    type Error = VariantError;

    fn try_into(self) -> Result<HashMap<&'a K, &'a V>, VariantError> {
        let mut map = HashMap::new();

        for entry in &self.0 {
            map.insert(entry.key()?, entry.value()?);
        }

        Ok(map)
    }
}

// Conversion of Hashmap to Dict
impl<K, V> From<HashMap<K, V>> for Dict
where
    K: Encode + Basic,
    V: Encode,
{
    fn from(value: HashMap<K, V>) -> Self {
        let vec = value
            .into_iter()
            .map(|(key, value)| DictEntry::new(key, value))
            .collect();

        Dict(vec)
    }
}

// Conversion of ARRAY of DICT_ENTRY to Dict
impl TryFrom<Array> for Dict {
    type Error = VariantError;

    fn try_from(value: Array) -> Result<Self, VariantError> {
        value.try_into().map(Dict::new_from_vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Dict, DictEntry};
    use core::convert::TryInto;
    use std::collections::HashMap;

    #[test]
    fn hashmap_to_vec() {
        let mut hash: HashMap<i64, &str> = HashMap::new();
        hash.insert(1, "123");
        hash.insert(2, "456");
        hash.insert(3, "789");

        let dict: Dict = hash.into();
        for entry in dict.get() {
            match entry.key::<i64>().unwrap() {
                1 => assert!(entry.value::<String>().unwrap() == "123"),
                2 => assert!(entry.value::<String>().unwrap() == "456"),
                3 => assert!(entry.value::<String>().unwrap() == "789"),
                _ => panic!("Encountered a key that wasn't supposed to be in the hashmap"),
            }
        }
    }

    #[test]
    fn vec_to_hashmap() {
        let mut v = vec![];
        v.push(DictEntry::new(1i64, "123"));
        v.push(DictEntry::new(2i64, "456"));
        v.push(DictEntry::new(3i64, "789"));
        let dict = Dict::new_from_vec(v);

        let hash: HashMap<i64, String> = dict.try_into().unwrap();
        for (key, value) in hash.iter() {
            match key {
                1 => assert!(*value == "123"),
                2 => assert!(*value == "456"),
                3 => assert!(*value == "789"),
                _ => panic!("Encountered a key that wasn't supposed to be in the hashmap"),
            }
        }
    }
}
