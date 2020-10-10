use core::convert::TryFrom;
use core::fmt::{self, Debug, Display, Formatter};
use core::str;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::ops::{Bound, RangeBounds};
use std::sync::Arc;

use crate::{Basic, EncodingFormat, Error, Result, Type};

// A data type similar to Cow and [`bytes::Bytes`] but unlike the former won't allow us to only keep
// the owned bytes in Arc and latter doesn't have a notion of borrowed data and would require API
// breakage.
//
// [`bytes::Bytes`]: https://docs.rs/bytes/0.5.6/bytes/struct.Bytes.html
#[derive(PartialEq, Eq, Hash, Clone)]
enum Bytes<'b> {
    Borrowed(&'b [u8]),
    Owned(Arc<[u8]>),
}

impl<'b> Bytes<'b> {
    fn borrowed<'s: 'b>(bytes: &'s [u8]) -> Self {
        Self::Borrowed(bytes)
    }

    fn owned(bytes: Vec<u8>) -> Self {
        Self::Owned(bytes.into())
    }
}

impl<'b> std::ops::Deref for Bytes<'b> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        match self {
            Bytes::Borrowed(borrowed) => borrowed,
            Bytes::Owned(owned) => &owned,
        }
    }
}

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
/// This is implemented so that multiple instances can share the same underlying signature string.
/// Use [`slice`] method to create new signature that represents a portion of a signature
///
/// [identifies]: https://dbus.freedesktop.org/doc/dbus-specification.html#type-system
/// [`slice`]: #method.slice
#[derive(Eq, Hash, Clone)]
pub struct Signature<'a> {
    bytes: Bytes<'a>,
    pos: usize,
    end: usize,
}

impl<'a> Signature<'a> {
    /// The signature as a string.
    pub fn as_str(&self) -> &str {
        // SAFETY: non-UTF8 characters in Signature should NEVER happen
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// The signature bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[self.pos..self.end]
    }

    /// Create a new Signature from given bytes.
    ///
    /// Since the passed bytes are not checked for correctness, it's provided for ease of
    /// `Type` implementations.
    pub fn from_bytes_unchecked<'s: 'a>(bytes: &'s [u8]) -> Self {
        Self {
            bytes: Bytes::borrowed(bytes),
            pos: 0,
            end: bytes.len(),
        }
    }

    /// Same as `from_bytes_unchecked`, except it takes a string reference.
    pub fn from_str_unchecked<'s: 'a>(signature: &'s str) -> Self {
        Self::from_bytes_unchecked(signature.as_bytes())
    }

    /// Same as `from_str_unchecked`, except it takes an owned `String`.
    pub fn from_string_unchecked(signature: String) -> Self {
        let bytes = signature.into_bytes();
        let end = bytes.len();

        Self {
            bytes: Bytes::owned(bytes),
            pos: 0,
            end,
        }
    }

    /// the signature's length.
    pub fn len(&self) -> usize {
        self.end - self.pos
    }

    /// if the signature is empty.
    pub fn is_empty(&self) -> bool {
        self.as_bytes().is_empty()
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> Signature<'static> {
        let bytes = self.as_bytes().to_vec();
        let end = bytes.len();

        Signature {
            bytes: Bytes::owned(bytes),
            pos: 0,
            end,
        }
    }

    /// Creates an owned clone of `self`.
    pub fn into_owned(self) -> Signature<'static> {
        self.to_owned()
    }

    /// Returns a slice of `self` for the provided range.
    ///
    /// # Panics
    ///
    /// Requires that begin <= end and end <= self.len(), otherwise slicing will panic.
    pub fn slice(&self, range: impl RangeBounds<usize>) -> Self {
        let len = self.len();

        let pos = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };

        assert!(
            pos <= end,
            "range start must not be greater than end: {:?} <= {:?}",
            pos,
            end,
        );
        assert!(
            end <= len,
            "range end out of bounds: {:?} <= {:?}",
            end,
            len,
        );

        if end == pos {
            return Self::from_str_unchecked("");
        }

        let mut clone = self.clone();
        clone.pos += pos;
        clone.end = self.pos + end;

        clone
    }
}

impl<'a> Debug for Signature<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // FIXME: Should we display all the bytes along with self.pos and self.end, instead?
        f.write_str("Signature: [\n")?;
        for byte in self.as_bytes() {
            f.write_fmt(format_args!("\t{} ({}),\n", *byte as char, byte))?;
        }
        f.write_str("]")
    }
}

impl<'a> Basic for Signature<'a> {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
    const ALIGNMENT: usize = 1;

    fn alignment(format: EncodingFormat) -> usize {
        match format {
            EncodingFormat::DBus => 1,
            EncodingFormat::GVariant => 1,
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

impl<'a, 'b> PartialEq<Signature<'a>> for Signature<'b> {
    fn eq(&self, other: &Signature) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<'a> PartialEq<&str> for Signature<'a> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<'a> Display for Signature<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.as_str(), f)
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

/// Owned [`Signature`](struct.Signature.html)
#[derive(Debug, Clone, PartialEq, serde::Serialize, zvariant_derive::Type)]
pub struct OwnedSignature(Signature<'static>);

impl OwnedSignature {
    pub fn into_inner(self) -> Signature<'static> {
        self.0
    }
}

impl std::ops::Deref for OwnedSignature {
    type Target = Signature<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::From<OwnedSignature> for Signature<'static> {
    fn from(o: OwnedSignature) -> Self {
        o.into_inner()
    }
}

impl<'a> std::convert::From<Signature<'a>> for OwnedSignature {
    fn from(o: Signature<'a>) -> Self {
        OwnedSignature(o.into_owned())
    }
}

impl<'de> Deserialize<'de> for OwnedSignature {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = SignatureVisitor;

        deserializer
            .deserialize_string(visitor)
            .map(|v| OwnedSignature(v.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::Signature;

    #[test]
    fn signature_slicing() {
        let sig = Signature::from_str_unchecked("(asta{sv})");
        assert_eq!(sig, "(asta{sv})");

        let slice = sig.slice(1..);
        assert_eq!(slice.len(), sig.len() - 1);
        assert_eq!(slice, &sig[1..]);
        assert_eq!(slice.as_bytes()[1], b's');
        assert_eq!(slice.as_bytes()[2], b't');

        let slice = slice.slice(2..3);
        assert_eq!(slice.len(), 1);
        assert_eq!(slice, "t");
        assert_eq!(slice.slice(1..), "");
    }
}
