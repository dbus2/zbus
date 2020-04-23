use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str;

use crate::Basic;

/// String that identifies the type of an encoded value.
///
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename(serialize = "zvariant::Signature", deserialize = "zvariant::Signature"))]
pub struct Signature<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> Signature<'a> {
    /// The signature as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a> Basic for Signature<'a> {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
    const ALIGNMENT: usize = 1;
}

// TODO: TryFrom instead and check signature validity.
impl<'a> From<&'a str> for Signature<'a> {
    fn from(value: &'a str) -> Self
    where
        Self: 'a,
    {
        Self(Cow::from(value))
    }
}

impl<'a> From<String> for Signature<'a> {
    fn from(value: String) -> Self {
        Self(Cow::from(value))
    }
}

impl<'a> From<&Signature<'a>> for String {
    fn from(value: &Signature<'a>) -> String {
        String::from(value.as_str())
    }
}

impl<'a> std::ops::Deref for Signature<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> PartialEq<&str> for Signature<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> std::fmt::Display for Signature<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
