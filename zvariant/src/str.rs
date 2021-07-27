// FIXME: Drop this when the deprecated `Basic::ALIGNMENT` is dropped in the next API break.
#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use static_assertions::assert_impl_all;
use std::{borrow::Cow, str};

use crate::{Basic, EncodingFormat, Signature, Type};

/// A string wrapper.
///
/// This is used for keeping strings in a [`Value`]. API is provided to convert from, and to a
/// [`&str`] and [`String`].
///
/// [`Value`]: enum.Value.html#variant.Str
/// [`&str`]: https://doc.rust-lang.org/std/str/index.html
/// [`String`]: https://doc.rust-lang.org/std/string/struct.String.html
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
#[serde(rename(serialize = "zvariant::Str", deserialize = "zvariant::Str"))]
pub struct Str<'a>(#[serde(borrow)] Cow<'a, str>);

assert_impl_all!(Str<'_>: Send, Sync, Unpin);

impl<'a> Str<'a> {
    /// The underlying string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Str<'static> {
        let s = self.0.clone().into_owned();
        Str(Cow::Owned(s))
    }
}

impl<'a> Basic for Str<'a> {
    const SIGNATURE_CHAR: char = <&str>::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = <&str>::SIGNATURE_STR;
    const ALIGNMENT: usize = <&str>::ALIGNMENT;

    fn alignment(format: EncodingFormat) -> usize {
        <&str>::alignment(format)
    }
}

impl<'a> Type for Str<'a> {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked(Self::SIGNATURE_STR)
    }
}

impl<'a> From<&'a str> for Str<'a> {
    fn from(value: &'a str) -> Self {
        Self(Cow::from(value))
    }
}

impl<'a> From<&'a String> for Str<'a> {
    fn from(value: &'a String) -> Self {
        Self(Cow::from(value.as_str()))
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

impl<'a> PartialEq<str> for Str<'a> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
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

#[cfg(test)]
mod tests {
    use super::Str;

    #[test]
    fn from_string() {
        let string = String::from("value");
        let v = Str::from(&string);
        assert_eq!(v.as_str(), "value");
    }
}
