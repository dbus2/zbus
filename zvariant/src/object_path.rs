use core::convert::TryFrom;
use core::str;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::Serialize;
use std::borrow::Cow;

use crate::{Basic, Error, Result, Signature, Type};

/// String that identifies objects at a given destination on the D-Bus bus.
///
/// Mostly likely this is only useful in the D-Bus context.
///
/// # Examples
///
/// ```
/// use core::convert::TryFrom;
/// use zvariant::ObjectPath;
///
/// // Valid object paths
/// let o = ObjectPath::try_from("/").unwrap();
/// assert_eq!(o, "/");
/// let o = ObjectPath::try_from("/Path/t0/0bject").unwrap();
/// assert_eq!(o, "/Path/t0/0bject");
/// let o = ObjectPath::try_from("/a/very/looooooooooooooooooooooooo0000o0ng/path").unwrap();
/// assert_eq!(o, "/a/very/looooooooooooooooooooooooo0000o0ng/path");
///
/// // Invalid object paths
/// ObjectPath::try_from("").unwrap_err();
/// ObjectPath::try_from("/double//slashes/").unwrap_err();
/// ObjectPath::try_from(".").unwrap_err();
/// ObjectPath::try_from("/end/with/slash/").unwrap_err();
/// ObjectPath::try_from("/ha.d").unwrap_err();
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone)]
#[serde(rename(
    serialize = "zvariant::ObjectPath",
    deserialize = "zvariant::ObjectPath"
))]
pub struct ObjectPath<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> ObjectPath<'a> {
    /// The object path as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create a new `ObjectPath` from the given string.
    ///
    /// Since the passed string is not checked for correctness, prefer using the
    /// `TryFrom<&str>` implementation.
    pub fn from_str_unchecked<'s: 'a>(signature: &'s str) -> Self {
        Self(Cow::from(signature))
    }

    /// Same as `from_str_unchecked`, except it takes an owned `String`.
    ///
    /// Since the passed string is not checked for correctness, prefer using the
    /// `TryFrom<String>` implementation.
    pub fn from_string_unchecked(signature: String) -> Self {
        Self(Cow::from(signature))
    }

    pub(crate) fn to_owned(&self) -> ObjectPath<'static> {
        let s = self.0.clone().into_owned();
        ObjectPath(Cow::Owned(s))
    }
}

impl<'a> Basic for ObjectPath<'a> {
    const SIGNATURE_CHAR: char = 'o';
    const SIGNATURE_STR: &'static str = "o";
    const ALIGNMENT: usize = <&str>::ALIGNMENT;
}

impl<'a> Type for ObjectPath<'a> {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked(Self::SIGNATURE_STR)
    }
}

/// Try to create an ObjectPath from a string.
impl<'a> TryFrom<&'a str> for ObjectPath<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        ensure_correct_object_path_str(value)?;

        Ok(Self(Cow::from(value)))
    }
}

impl<'a> TryFrom<String> for ObjectPath<'a> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        ensure_correct_object_path_str(&value)?;

        Ok(Self(Cow::from(value)))
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

impl<'de: 'a, 'a> Deserialize<'de> for ObjectPath<'a> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = ObjectPathVisitor;

        deserializer.deserialize_str(visitor)
    }
}

struct ObjectPathVisitor;

impl<'de> Visitor<'de> for ObjectPathVisitor {
    type Value = ObjectPath<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an ObjectPath")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, value: &'de str) -> core::result::Result<ObjectPath<'de>, E>
    where
        E: serde::de::Error,
    {
        ObjectPath::try_from(value).map_err(serde::de::Error::custom)
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> core::result::Result<ObjectPath<'de>, E>
    where
        E: serde::de::Error,
    {
        ObjectPath::try_from(String::from(value)).map_err(serde::de::Error::custom)
    }
}

fn ensure_correct_object_path_str(path: &str) -> Result<()> {
    let mut prev = '\0';

    // Rules
    //
    // * At least 1 character.
    // * First character must be `/`
    // * No trailing `/`
    // * No `//`
    // * Only ASCII alphanumeric, `_` or '/'
    if path.is_empty() {
        return Err(serde::de::Error::invalid_length(0, &"> 0 character"));
    }

    for (i, c) in path.chars().enumerate() {
        if i == 0 && c != '/' {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Char(c),
                &"/",
            ));
        } else if c == '/' && prev == '/' {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str("//"),
                &"/",
            ));
        } else if path.len() > 1 && i == (path.len() - 1) && c == '/' {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Char('/'),
                &"an alphanumeric character or `_`",
            ));
        } else if !c.is_ascii_alphanumeric() && c != '/' && c != '_' {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Char(c),
                &"an alphanumeric character, `_` or `/`",
            ));
        }
        prev = c;
    }

    Ok(())
}
