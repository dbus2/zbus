use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str;

use crate::Basic;

/// String that identifies objects at a given destination on the D-Bus bus.
///
/// Mostly likely this is only useful in the D-Bus context.
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename(
    serialize = "zvariant::ObjectPath",
    deserialize = "zvariant::ObjectPath"
))]
pub struct ObjectPath<'a>(Cow<'a, str>);

impl<'a> ObjectPath<'a> {
    /// The object path as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a> Basic for ObjectPath<'a> {
    const SIGNATURE_CHAR: char = 'o';
    const SIGNATURE_STR: &'static str = "o";
    const ALIGNMENT: usize = <&str>::ALIGNMENT;
}

impl<'a> From<&'a str> for ObjectPath<'a> {
    fn from(value: &'a str) -> Self {
        Self(Cow::from(value))
    }
}

impl<'a> From<String> for ObjectPath<'a> {
    fn from(value: String) -> Self {
        Self(Cow::from(value))
    }
}

impl<'a> std::ops::Deref for ObjectPath<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> PartialEq<&str> for ObjectPath<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> std::fmt::Display for ObjectPath<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
