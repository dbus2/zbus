use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str;

use crate::{Basic, Signature, Type};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename(serialize = "zvariant::Str", deserialize = "zvariant::Str"))]
pub struct Str<'a>(#[serde(borrow)] pub Cow<'a, str>);

impl<'a> Str<'a> {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn to_owned(&self) -> Str<'static> {
        let s = self.0.clone().into_owned();
        Str(Cow::Owned(s))
    }
}

impl<'a> Basic for Str<'a> {
    const SIGNATURE_CHAR: char = <&str>::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = <&str>::SIGNATURE_STR;
    const ALIGNMENT: usize = <&str>::ALIGNMENT;
}

impl<'a> Type for Str<'a> {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked(Self::SIGNATURE_STR)
    }
}

impl<'a> From<&'a str> for Str<'a> {
    fn from(value: &'a str) -> Self {
        Self(Cow::from(value))
    }
}

impl<'a> From<String> for Str<'a> {
    fn from(value: String) -> Self {
        Self(Cow::from(value))
    }
}

impl<'a> From<Str<'a>> for String {
    fn from(value: Str<'a>) -> String {
        value.0.into_owned()
    }
}

impl<'a> From<&'a Str<'a>> for &'a str {
    fn from(value: &'a Str<'a>) -> &'a str {
        value.as_str()
    }
}

impl<'a> std::ops::Deref for Str<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> PartialEq<&str> for Str<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> std::fmt::Display for Str<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
