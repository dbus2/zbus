use core::convert::{TryFrom, TryInto};
use std::collections::HashMap;

use crate::VariantError;
use crate::{Array, Basic, Decode, DictEntry, Encode};

// Since neither `From` trait nor `HashMap` is from this crate, we need this intermediate type.
// We can't implement `Into` either as `Vec` isn't our type either.
#[derive(Default)]
pub struct Dict(Vec<DictEntry>);

impl Dict {
    pub fn new() -> Self {
        Dict::default()
    }

    pub fn new_from_vec(vec: Vec<DictEntry>) -> Self {
        Dict(vec)
    }

    pub fn inner(&self) -> &Vec<DictEntry> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Vec<DictEntry> {
        &mut self.0
    }

    pub fn take_inner(self) -> Vec<DictEntry> {
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
            let (key, value) = entry.take_inner()?;

            map.insert(key, value);
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
        for entry in dict.inner() {
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
