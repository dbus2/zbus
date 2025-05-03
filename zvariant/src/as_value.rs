//! Utilities to (de)serialize a value as a [`enum@zvariant::Value`].
//!
//! This is mainly useful for constructing a strongly-typed API for dealing with dictionaries
//! containing string keys and variant values (`a{sv}` in D-Bus language) See the relevant
//! [FAQ entry] in our book for more details and examples.
//!
//! [FAQ entry]: https://dbus2.github.io/zbus/faq.html#how-to-use-a-struct-as-a-dictionary

use crate::{DeserializeValue, SerializeValue, Type};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Utilities to (de)serialize an optional value as a [`enum@zvariant::Value`].
pub mod optional {
    use super::*;

    /// Serialize an optional value as a [`enum@zvariant::Value`].
    pub fn serialize<T, S>(value: &Option<T>, ser: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Type + Serialize,
    {
        super::serialize(value.as_ref().unwrap(), ser)
    }

    /// Deserialize an optional value as a [`enum@zvariant::Value`].
    pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + Type + 'de,
    {
        super::deserialize(deserializer).map(Some)
    }
}

/// Serialize a value as a [`enum@zvariant::Value`].
pub fn serialize<T, S>(value: &T, ser: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Type + Serialize,
{
    SerializeValue(value).serialize(ser)
}

/// Deserialize a value as a [`enum@zvariant::Value`].
pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Type + 'de,
{
    DeserializeValue::deserialize(deserializer).map(|v| v.0)
}
