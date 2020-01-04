use serde_derive::Serialize;
use std::borrow::Cow;
use std::str;

use crate::Basic;

/// String that identifies the type of an encoded value.
///
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone)]
#[serde(rename(serialize = "zvariant::Signature", deserialize = "zvariant::Signature"))]
pub struct Signature<'a>(Cow<'a, str>);

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
