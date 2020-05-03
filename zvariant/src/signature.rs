use core::convert::TryFrom;
use core::str;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::Serialize;
use std::borrow::Cow;

use crate::{Basic, Error, Result};

/// String that identifies the type of an encoded value.
///
#[derive(Debug, PartialEq, Eq, Hash, Serialize, Clone)]
#[serde(rename(serialize = "zvariant::Signature", deserialize = "zvariant::Signature"))]
pub struct Signature<'a>(#[serde(borrow)] Cow<'a, str>);

impl<'a> Signature<'a> {
    /// The signature as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create a new Signature from given string.
    ///
    /// Since the passed string is not checked for correctness, it's provided for ease of
    /// `Type` implementations.
    pub fn from_str_unchecked<'s: 'a>(signature: &'s str) -> Self {
        Self(Cow::from(signature))
    }

    /// Same as `from_str_unchecked`, except it takes an owned `String`.
    pub fn from_string_unchecked(signature: String) -> Self {
        Self(Cow::from(signature))
    }

    pub(crate) fn to_owned(&self) -> Signature<'static> {
        let s = self.0.clone().into_owned();
        Signature(Cow::Owned(s))
    }
}

impl<'a> Basic for Signature<'a> {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
    const ALIGNMENT: usize = 1;
}

/// Try to create a Signature from a string.
///
/// # Examples
///
/// ```
/// use core::convert::TryFrom;
/// use zvariant::Signature;
///
/// // Valid signatures
/// Signature::try_from("").unwrap();
/// Signature::try_from("y").unwrap();
/// Signature::try_from("xs").unwrap();
/// Signature::try_from("(ysa{sd})").unwrap();
/// Signature::try_from("a{sd}").unwrap();
///
/// // Invalid signatures
/// Signature::try_from("z").unwrap_err();
/// Signature::try_from("(xs").unwrap_err();
/// Signature::try_from("xs)").unwrap_err();
/// Signature::try_from("s/").unwrap_err();
/// Signature::try_from("a").unwrap_err();
/// Signature::try_from("a{yz}").unwrap_err();
/// ```
impl<'a> TryFrom<&'a str> for Signature<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        ensure_correct_signature_str(value)?;

        Ok(Self(Cow::from(value)))
    }
}

impl<'a> TryFrom<String> for Signature<'a> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        ensure_correct_signature_str(&value)?;

        Ok(Self(Cow::from(value)))
    }
}

impl<'a> From<Signature<'a>> for String {
    fn from(value: Signature<'a>) -> String {
        String::from(value.as_str())
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

impl<'de: 'a, 'a> Deserialize<'de> for Signature<'a> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = SignatureVisitor;

        deserializer.deserialize_str(visitor)
    }
}

struct SignatureVisitor;

impl<'de> Visitor<'de> for SignatureVisitor {
    type Value = Signature<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a Signature")
    }

    #[inline]
    fn visit_borrowed_str<E>(self, value: &'de str) -> core::result::Result<Signature<'de>, E>
    where
        E: serde::de::Error,
    {
        Signature::try_from(value).map_err(serde::de::Error::custom)
    }

    #[inline]
    fn visit_str<E>(self, value: &str) -> core::result::Result<Signature<'de>, E>
    where
        E: serde::de::Error,
    {
        Signature::try_from(String::from(value)).map_err(serde::de::Error::custom)
    }
}

fn ensure_correct_signature_str(signature: &str) -> Result<()> {
    if signature.len() > 255 {
        return Err(Error::InvalidSignature(String::from(signature)));
    }

    let (mut parsed, end) = match signature.chars().next() {
        Some(crate::ARRAY_SIGNATURE_CHAR) => {
            if signature.len() == 1 {
                return Err(Error::InvalidSignature(String::from(signature)));
            }

            (1, signature.len())
        }
        Some(crate::STRUCT_SIG_START_CHAR) => {
            if !signature.ends_with(crate::STRUCT_SIG_END_CHAR) {
                return Err(Error::InvalidSignature(String::from(signature)));
            }

            (1, signature.len() - 1)
        }
        Some(crate::DICT_ENTRY_SIG_START_CHAR) => {
            if !signature.ends_with(crate::DICT_ENTRY_SIG_END_CHAR) {
                return Err(Error::InvalidSignature(String::from(signature)));
            }

            (1, signature.len() - 1)
        }
        Some(_) | None => (0, signature.len()),
    };

    while parsed < end {
        let rest_of_signature = &signature[parsed..end];
        let signature = Signature::from_str_unchecked(rest_of_signature);
        let slice = crate::utils::slice_signature(&signature)?;

        parsed += slice.len();
    }

    Ok(())
}
