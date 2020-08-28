use core::convert::TryFrom;
use core::str;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::borrow::Cow;

use crate::{Basic, EncodingFormat, Error, Result, Type};

/// String that [identifies] the type of an encoded value.
///
/// # Examples
///
/// ```
/// use core::convert::TryFrom;
/// use zvariant::Signature;
///
/// // Valid signatures
/// let s = Signature::try_from("").unwrap();
/// assert_eq!(s, "");
/// let s = Signature::try_from("y").unwrap();
/// assert_eq!(s, "y");
/// let s = Signature::try_from("xs").unwrap();
/// assert_eq!(s, "xs");
/// let s = Signature::try_from("(ysa{sd})").unwrap();
/// assert_eq!(s, "(ysa{sd})");
/// let s = Signature::try_from("a{sd}").unwrap();
/// assert_eq!(s, "a{sd}");
///
/// // Invalid signatures
/// Signature::try_from("z").unwrap_err();
/// Signature::try_from("(xs").unwrap_err();
/// Signature::try_from("xs)").unwrap_err();
/// Signature::try_from("s/").unwrap_err();
/// Signature::try_from("a").unwrap_err();
/// Signature::try_from("a{yz}").unwrap_err();
/// ```
///
/// [identifies]: https://dbus.freedesktop.org/doc/dbus-specification.html#type-system
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Signature<'a>(Cow<'a, [u8]>);

impl<'a> Signature<'a> {
    /// The signature as a string.
    pub fn as_str(&self) -> &str {
        // SAFETY: non-UTF8 characters in Signature should NEVER happen
        unsafe { str::from_utf8_unchecked(&self.0) }
    }

    /// The signature bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Create a new Signature from given bytes.
    ///
    /// Since the passed bytes are not checked for correctness, it's provided for ease of
    /// `Type` implementations.
    pub fn from_bytes_unchecked<'s: 'a>(bytes: &'s [u8]) -> Self {
        Self(Cow::from(bytes))
    }

    /// Same as `from_bytes_unchecked`, except it takes a string reference.
    pub fn from_str_unchecked<'s: 'a>(signature: &'s str) -> Self {
        Self::from_bytes_unchecked(signature.as_bytes())
    }

    /// Same as `from_str_unchecked`, except it takes an owned `String`.
    pub fn from_string_unchecked(signature: String) -> Self {
        Self(Cow::from(signature.as_bytes().to_owned()))
    }

    /// the signature's length.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// if the signature is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Signature<'static> {
        let s = self.0.clone().into_owned();
        Signature(Cow::Owned(s))
    }

    /// Creates an owned clone of `self`.
    pub fn into_owned(self) -> Signature<'static> {
        Signature(Cow::Owned(self.0.into_owned()))
    }
}

impl<'a> Basic for Signature<'a> {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
    const ALIGNMENT: usize = 1;

    fn alignment(format: EncodingFormat) -> usize {
        match format {
            EncodingFormat::DBus => 1,
        }
    }
}

impl<'a> Type for Signature<'a> {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked(Self::SIGNATURE_STR)
    }
}

impl<'a> TryFrom<&'a [u8]> for Signature<'a> {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self> {
        ensure_correct_signature_str(value)?;

        Ok(Self::from_bytes_unchecked(value))
    }
}

/// Try to create a Signature from a string.
impl<'a> TryFrom<&'a str> for Signature<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        Self::try_from(value.as_bytes())
    }
}

impl<'a> TryFrom<String> for Signature<'a> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        ensure_correct_signature_str(value.as_bytes())?;

        Ok(Self::from_string_unchecked(value))
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
        self.as_str().fmt(f)
    }
}

impl<'a> Serialize for Signature<'a> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
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

fn ensure_correct_signature_str(signature: &[u8]) -> Result<()> {
    if signature.len() > 255 {
        return Err(serde::de::Error::invalid_length(
            signature.len(),
            &"<= 255 characters",
        ));
    }

    if signature.is_empty() {
        return Ok(());
    }

    let (mut parsed, end) = match signature[0] as char {
        crate::ARRAY_SIGNATURE_CHAR => {
            if signature.len() == 1 {
                return Err(serde::de::Error::invalid_length(1, &"> 1 character"));
            }

            (1, signature.len())
        }
        crate::STRUCT_SIG_START_CHAR => {
            // We can't get None here cause we already established there is at least 1 char
            let c = *signature.last().expect("empty signature") as char;
            if c != crate::STRUCT_SIG_END_CHAR {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Char(c),
                    &crate::STRUCT_SIG_END_STR,
                ));
            }

            (1, signature.len() - 1)
        }
        crate::DICT_ENTRY_SIG_START_CHAR => {
            // We can't get None here cause we already established there is at least 1 char
            let c = *signature.last().expect("empty signature") as char;
            if c != crate::DICT_ENTRY_SIG_END_CHAR {
                return Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Char(c),
                    &crate::DICT_ENTRY_SIG_END_STR,
                ));
            }

            (1, signature.len() - 1)
        }
        _ => (0, signature.len()),
    };

    while parsed < end {
        let rest_of_signature = &signature[parsed..end];
        let signature = Signature::from_bytes_unchecked(rest_of_signature);
        let slice = crate::utils::slice_signature(&signature)?;

        parsed += slice.len();
    }

    Ok(())
}
