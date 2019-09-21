use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::DictEntry;
use crate::{SimpleVariantType, VariantType};

// Since neither `From` trait nor `HashMap` is from this crate, we need this intermediate type.
// We can't implement `Into` either as `Vec` isn't our type either.
pub struct Dict<K, V>(HashMap<K, V>);

impl<'a, K: SimpleVariantType<'a> + Hash + Eq, V: VariantType<'a>> Dict<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn new_from_hashmap(hashmap: HashMap<K, V>) -> Self {
        Self(hashmap)
    }

    pub fn inner(&self) -> &HashMap<K, V> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.0
    }
}

impl<'a, K: SimpleVariantType<'a> + Hash + Eq, V: VariantType<'a>> Deref for Dict<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl<'a, K: SimpleVariantType<'a> + Hash + Eq, V: VariantType<'a>> DerefMut for Dict<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

// Conversion of ARRAY of DICT_ENTRY to DICT
impl<'a, K: SimpleVariantType<'a> + Hash + Eq, V: VariantType<'a>> From<Vec<DictEntry<K, V>>>
    for Dict<K, V>
{
    fn from(val: Vec<DictEntry<K, V>>) -> Self {
        let mut map = HashMap::new();

        for entry in val {
            let (key, value) = entry.take_inner();

            map.insert(key, value);
        }

        Self(map)
    }
}

// Conversion of DICT to ARRAY of DICT_ENTRY
impl<'a, K: SimpleVariantType<'a> + Hash + Eq, V: VariantType<'a>> Into<Vec<DictEntry<K, V>>>
    for Dict<K, V>
{
    fn into(self) -> Vec<DictEntry<K, V>> {
        let mut vec = Vec::new();

        for (key, value) in self.0 {
            vec.push(DictEntry::new(key, value));
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::{Dict, DictEntry};

    #[test]
    fn hashmap_to_vec() {
        let mut dict: Dict<i64, &str> = Dict::new();
        dict.insert(1, "123");
        dict.insert(2, "456");
        dict.insert(3, "789");

        let v: Vec<DictEntry<i64, &str>> = dict.into();
        for entry in v {
            match entry.key() {
                1 => assert!(*entry.value() == "123"),
                2 => assert!(*entry.value() == "456"),
                3 => assert!(*entry.value() == "789"),
                _ => panic!("Encountered a key that wasn't supposed to be in the hashmap"),
            }
        }
    }

    #[test]
    fn vec_to_hashmap() {
        let mut v: Vec<DictEntry<i64, &str>> = Vec::new();
        v.push(DictEntry::new(1, "123"));
        v.push(DictEntry::new(2, "456"));
        v.push(DictEntry::new(3, "789"));

        let dict: Dict<i64, &str> = v.into();
        for (key, value) in dict.iter() {
            match key {
                1 => assert!(*value == "123"),
                2 => assert!(*value == "456"),
                3 => assert!(*value == "789"),
                _ => panic!("Encountered a key that wasn't supposed to be in the hashmap"),
            }
        }
    }
}
