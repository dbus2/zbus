use serde::{Deserialize, Serialize};

use super::{child_signature::ChildSignature, fields_signatures::FieldsSignatures};

use core::fmt;
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::serialized::Format;

/// A D-Bus signature in parsed form.
///
/// This is similar to the [`zvariant::Signature`] type, but unlike `zvariant::Signature`, this is a
/// parsed representation of a signature. Our (de)serialization API primarily uses this type for
/// efficiency.
///
/// # Examples
///
/// Typically, you'd create a `Signature` from a string:
///
/// ```
/// use std::str::FromStr;
/// use zvariant::parsed::Signature;
///
/// let sig = Signature::from_str("a{sv}").unwrap();
/// assert_eq!(sig.to_string(), "a{sv}");
///
/// let sig = Signature::from_str("(xa{bs}as)").unwrap();
/// assert_eq!(sig.to_string(), "(xa{bs}as)");
/// ```
///
/// [`zvariant::Signature`]: https://docs.rs/zvariant/latest/zvariant/struct.Signature.html
#[derive(Debug, Clone)]
pub enum Signature {
    // Basic types
    /// The signature for the unit type (`()`). This is not a valid D-Bus signature, but is used to
    /// represnt "no data" (for example, a D-Bus method call without any arguments will have this
    /// as its body signature).
    ///
    /// # Warning
    ///
    /// This variant only exists for convenience and must only be used as a top-level signature. If
    /// used inside container signatures, it will cause errors and in somce cases, panics. It's
    /// best to not use it directly.
    Unit,
    /// The signature for an 8-bit unsigned integer (AKA a byte).
    U8,
    /// The signature for a boolean.
    Bool,
    /// The signature for a 16-bit signed integer.
    I16,
    /// The signature for a 16-bit unsigned integer.
    U16,
    /// The signature for a 32-bit signed integer.
    I32,
    /// The signature for a 32-bit unsigned integer.
    U32,
    /// The signature for a 64-bit signed integer.
    I64,
    /// The signature for a 64-bit unsigned integer.
    U64,
    /// The signature for a 64-bit floating point number.
    F64,
    /// The signature for a string.
    Str,
    /// The signature for a signature.
    Signature,
    /// The signature for an object path.
    ObjectPath,
    /// The signature for a variant.
    Variant,
    /// The signature for a file descriptor.
    #[cfg(unix)]
    Fd,

    // Container types
    /// The signature for an array.
    Array(ChildSignature),
    /// The signature for a dictionary.
    Dict {
        /// The signature for the key.
        key: ChildSignature,
        /// The signature for the value.
        value: ChildSignature,
    },
    /// The signature for a structure.
    Structure(FieldsSignatures),
    /// The signature for a maybe type (gvariant-specific).
    #[cfg(feature = "gvariant")]
    Maybe(ChildSignature),
}

impl Signature {
    /// The size of the string form of `self`.
    pub fn string_len(&self) -> usize {
        match self {
            Signature::Unit => 0,
            Signature::U8
            | Signature::Bool
            | Signature::I16
            | Signature::U16
            | Signature::I32
            | Signature::U32
            | Signature::I64
            | Signature::U64
            | Signature::F64
            | Signature::Str
            | Signature::Signature
            | Signature::ObjectPath
            | Signature::Variant => 1,
            #[cfg(unix)]
            Signature::Fd => 1,
            Signature::Array(child) => 1 + child.string_len(),
            Signature::Dict { key, value } => 3 + key.string_len() + value.string_len(),
            Signature::Structure(fields) => {
                let mut len = 2;
                for field in fields.iter() {
                    len += field.string_len();
                }
                len
            }
            #[cfg(feature = "gvariant")]
            Signature::Maybe(child) => 1 + child.string_len(),
        }
    }

    /// Write the string form of `self` to the given formatter.
    ///
    /// This produces the same output as the `Display::fmt`, unless `self` is a
    /// [`Signature::Structure`], in which case the written string will **not** be wrapped in
    /// parenthesis (`()`).
    pub fn write_as_string_no_parens(&self, write: &mut impl std::fmt::Write) -> fmt::Result {
        self.write_as_string(write, false)
    }

    /// Convert `self` to a string, without any enclosing parenthesis.
    ///
    /// This produces the same output as the `ToString::to_string`, unless `self` is a
    /// [`Signature::Structure`], in which case the written string will **not** be wrapped in
    /// parenthesis (`()`).
    pub fn to_string_no_parens(&self) -> String {
        let mut s = String::with_capacity(self.string_len());
        self.write_as_string(&mut s, false).unwrap();

        s
    }

    /// Parse signature from a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, super::Error> {
        parse(bytes, false)
    }

    /// Create a `Signature::Structure` for a given set of field signatures.
    pub fn structure<F>(fields: F) -> Self
    where
        F: Into<FieldsSignatures>,
    {
        Signature::Structure(fields.into())
    }

    /// Create a `Signature::Structure` for a given set of static field signatures.
    pub const fn static_structure(fields: &'static [&'static Signature]) -> Self {
        Signature::Structure(FieldsSignatures::Static { fields })
    }

    /// Create a `Signature::Array` for a given child signature.
    pub fn array<C>(child: C) -> Self
    where
        C: Into<ChildSignature>,
    {
        Signature::Array(child.into())
    }

    /// Create a `Signature::Array` for a given static child signature.
    pub const fn static_array(child: &'static Signature) -> Self {
        Signature::Array(ChildSignature::Static { child })
    }

    /// Create a `Signature::Dict` for a given key and value signatures.
    pub fn dict<K, V>(key: K, value: V) -> Self
    where
        K: Into<ChildSignature>,
        V: Into<ChildSignature>,
    {
        Signature::Dict {
            key: key.into(),
            value: value.into(),
        }
    }

    /// Create a `Signature::Dict` for a given static key and value signatures.
    pub const fn static_dict(key: &'static Signature, value: &'static Signature) -> Self {
        Signature::Dict {
            key: ChildSignature::Static { child: key },
            value: ChildSignature::Static { child: value },
        }
    }

    /// Create a `Signature::Maybe` for a given child signature.
    #[cfg(feature = "gvariant")]
    pub fn maybe<C>(child: C) -> Self
    where
        C: Into<ChildSignature>,
    {
        Signature::Maybe(child.into())
    }

    /// Create a `Signature::Maybe` for a given static child signature.
    #[cfg(feature = "gvariant")]
    pub const fn static_maybe(child: &'static Signature) -> Self {
        Signature::Maybe(ChildSignature::Static { child })
    }

    /// The required padding alignment for the given format.
    pub fn alignment(&self, format: Format) -> usize {
        match format {
            Format::DBus => self.alignment_dbus(),
            #[cfg(feature = "gvariant")]
            Format::GVariant => self.alignment_gvariant(),
        }
    }

    fn alignment_dbus(&self) -> usize {
        match self {
            Signature::U8 | Signature::Variant | Signature::Signature => 1,
            Signature::I16 | Signature::U16 => 2,
            Signature::I32
            | Signature::U32
            | Signature::Bool
            | Signature::Str
            | Signature::ObjectPath
            | Signature::Array(_)
            | Signature::Dict { .. } => 4,
            Signature::I64
            | Signature::U64
            | Signature::F64
            | Signature::Unit
            | Signature::Structure(_) => 8,
            #[cfg(unix)]
            Signature::Fd => 4,
            #[cfg(feature = "gvariant")]
            Signature::Maybe(_) => unreachable!("Maybe type is not supported in D-Bus"),
        }
    }

    #[cfg(feature = "gvariant")]
    fn alignment_gvariant(&self) -> usize {
        use std::cmp::max;

        match self {
            Signature::Unit
            | Signature::U8
            | Signature::I16
            | Signature::U16
            | Signature::I32
            | Signature::U32
            | Signature::F64
            | Signature::Bool
            | Signature::I64
            | Signature::U64
            | Signature::Signature => self.alignment_dbus(),
            #[cfg(unix)]
            Signature::Fd => self.alignment_dbus(),
            Signature::Str | Signature::ObjectPath => 1,
            Signature::Variant => 8,
            Signature::Array(child) | Signature::Maybe(child) => child.alignment_gvariant(),
            Signature::Dict { key, value } => {
                max(key.alignment_gvariant(), value.alignment_gvariant())
            }
            Signature::Structure(fields) => fields
                .iter()
                .map(Signature::alignment_gvariant)
                .max()
                .unwrap_or(1),
        }
    }

    /// Check if the signature is of a fixed-sized type.
    #[cfg(feature = "gvariant")]
    pub fn is_fixed_sized(&self) -> bool {
        match self {
            Signature::Unit
            | Signature::U8
            | Signature::Bool
            | Signature::I16
            | Signature::U16
            | Signature::I32
            | Signature::U32
            | Signature::I64
            | Signature::U64
            | Signature::F64 => true,
            #[cfg(unix)]
            Signature::Fd => true,
            Signature::Str
            | Signature::Signature
            | Signature::ObjectPath
            | Signature::Variant
            | Signature::Array(_)
            | Signature::Dict { .. }
            | Signature::Maybe(_) => false,
            Signature::Structure(fields) => fields.iter().all(|f| f.is_fixed_sized()),
        }
    }

    fn write_as_string(&self, w: &mut impl std::fmt::Write, outer_parens: bool) -> fmt::Result {
        match self {
            Signature::Unit => write!(w, ""),
            Signature::U8 => write!(w, "y"),
            Signature::Bool => write!(w, "b"),
            Signature::I16 => write!(w, "n"),
            Signature::U16 => write!(w, "q"),
            Signature::I32 => write!(w, "i"),
            Signature::U32 => write!(w, "u"),
            Signature::I64 => write!(w, "x"),
            Signature::U64 => write!(w, "t"),
            Signature::F64 => write!(w, "d"),
            Signature::Str => write!(w, "s"),
            Signature::Signature => write!(w, "g"),
            Signature::ObjectPath => write!(w, "o"),
            Signature::Variant => write!(w, "v"),
            #[cfg(unix)]
            Signature::Fd => write!(w, "h"),
            Signature::Array(array) => write!(w, "a{}", **array),
            Signature::Dict { key, value } => {
                write!(w, "a{{")?;
                write!(w, "{}{}", **key, **value)?;
                write!(w, "}}")
            }
            Signature::Structure(fields) => {
                if outer_parens {
                    write!(w, "(")?;
                }
                for field in fields.iter() {
                    write!(w, "{}", field)?;
                }
                if outer_parens {
                    write!(w, ")")?;
                }

                Ok(())
            }
            #[cfg(feature = "gvariant")]
            Signature::Maybe(maybe) => write!(w, "m{}", **maybe),
        }
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_as_string(f, true)
    }
}

impl FromStr for Signature {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s.as_bytes(), false)
    }
}

impl TryFrom<&str> for Signature {
    type Error = super::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Signature::from_str(value)
    }
}

/// Validate the given signature string.
pub fn validate(bytes: &[u8]) -> Result<(), super::Error> {
    parse(bytes, true).map(|_| ())
}

/// Parse a signature string into a `Signature`.
///
/// When `check_only` is true, the function will not allocate memory for the dynamic types.
/// Instead it will return dummy values in the parsed Signature.
fn parse(bytes: &[u8], check_only: bool) -> Result<Signature, super::Error> {
    use nom::{
        branch::alt,
        combinator::{all_consuming, eof, map},
        multi::{fold_many1, many1_count},
        sequence::{delimited, pair},
    };

    let empty = map(eof, |_| Signature::Unit);

    fn byte<'bytes, Error: nom::error::ParseError<&'bytes [u8]>>(
        b: u8,
    ) -> impl Fn(&'bytes [u8]) -> nom::IResult<&'bytes [u8], &'bytes [u8], Error> {
        move |bytes: &'bytes [u8]| nom::bytes::complete::tag(&[b])(bytes)
    }

    // `many1` allocates so we only want to use it when `check_only == false`
    type ManyError<'b> = nom::Err<nom::error::Error<&'b [u8]>>;
    fn many(
        bytes: &[u8],
        check_only: bool,
        top_level: bool,
    ) -> Result<(&[u8], Signature), ManyError<'_>> {
        let parser = |s| parse_signature(s, check_only);
        if check_only {
            return map(many1_count(parser), |_| Signature::Unit)(bytes);
        }

        // Avoid the allocation of `Vec<Signature>` in case of a single signature on the top-level.
        // This is a a very common case, especially in variants, where the signature needs to be
        // parsed at runtime.
        enum SignatureList {
            Unit,
            One(Signature),
            Structure(Vec<Signature>),
        }

        map(
            fold_many1(
                parser,
                || SignatureList::Unit,
                |acc, signature| match acc {
                    // On the top-level, we want to return the signature directly if there is only
                    // one.
                    SignatureList::Unit if top_level => SignatureList::One(signature),
                    SignatureList::Unit => SignatureList::Structure(vec![signature]),
                    SignatureList::One(one) => SignatureList::Structure(vec![one, signature]),
                    SignatureList::Structure(mut signatures) => {
                        signatures.push(signature);
                        SignatureList::Structure(signatures)
                    }
                },
            ),
            |sig_list| match sig_list {
                SignatureList::Unit => Signature::Unit,
                SignatureList::One(sig) => sig,
                SignatureList::Structure(signatures) => Signature::structure(signatures),
            },
        )(bytes)
    }

    fn parse_signature(bytes: &[u8], check_only: bool) -> nom::IResult<&[u8], Signature> {
        let parse_with_context = |bytes| parse_signature(bytes, check_only);

        let simple_type = alt((
            map(byte(b'y'), |_| Signature::U8),
            map(byte(b'b'), |_| Signature::Bool),
            map(byte(b'n'), |_| Signature::I16),
            map(byte(b'q'), |_| Signature::U16),
            map(byte(b'i'), |_| Signature::I32),
            map(byte(b'u'), |_| Signature::U32),
            map(byte(b'x'), |_| Signature::I64),
            map(byte(b't'), |_| Signature::U64),
            map(byte(b'd'), |_| Signature::F64),
            map(byte(b's'), |_| Signature::Str),
            map(byte(b'g'), |_| Signature::Signature),
            map(byte(b'o'), |_| Signature::ObjectPath),
            map(byte(b'v'), |_| Signature::Variant),
            #[cfg(unix)]
            map(byte(b'h'), |_| Signature::Fd),
        ));

        let dict = map(
            pair(
                byte(b'a'),
                delimited(
                    byte(b'{'),
                    pair(parse_with_context, parse_with_context),
                    byte(b'}'),
                ),
            ),
            |(_, (key, value))| {
                if check_only {
                    return Signature::Dict {
                        key: Signature::Unit.into(),
                        value: Signature::Unit.into(),
                    };
                }

                Signature::Dict {
                    key: key.into(),
                    value: value.into(),
                }
            },
        );

        let array = map(pair(byte(b'a'), parse_with_context), |(_, child)| {
            if check_only {
                return Signature::Array(Signature::Unit.into());
            }

            Signature::Array(child.into())
        });

        let structure = delimited(byte(b'('), |s| many(s, check_only, false), byte(b')'));

        #[cfg(feature = "gvariant")]
        let maybe = map(pair(byte(b'm'), parse_with_context), |(_, child)| {
            if check_only {
                return Signature::Maybe(Signature::Unit.into());
            }

            Signature::Maybe(child.into())
        });

        alt((
            simple_type,
            dict,
            array,
            structure,
            #[cfg(feature = "gvariant")]
            maybe,
        ))(bytes)
    }

    let (_, signature) = all_consuming(alt((empty, |s| many(s, check_only, true))))(bytes)
        .map_err(|_| super::Error::InvalidSignature)?;

    Ok(signature)
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Signature::Unit, Signature::Unit)
            | (Signature::U8, Signature::U8)
            | (Signature::Bool, Signature::Bool)
            | (Signature::I16, Signature::I16)
            | (Signature::U16, Signature::U16)
            | (Signature::I32, Signature::I32)
            | (Signature::U32, Signature::U32)
            | (Signature::I64, Signature::I64)
            | (Signature::U64, Signature::U64)
            | (Signature::F64, Signature::F64)
            | (Signature::Str, Signature::Str)
            | (Signature::Signature, Signature::Signature)
            | (Signature::ObjectPath, Signature::ObjectPath)
            | (Signature::Variant, Signature::Variant) => true,
            #[cfg(unix)]
            (Signature::Fd, Signature::Fd) => true,
            (Signature::Array(a), Signature::Array(b)) => a.eq(&**b),
            (
                Signature::Dict {
                    key: key_a,
                    value: value_a,
                },
                Signature::Dict {
                    key: key_b,
                    value: value_b,
                },
            ) => key_a.eq(&**key_b) && value_a.eq(&**value_b),
            (Signature::Structure(a), Signature::Structure(b)) => a.iter().eq(b.iter()),
            #[cfg(feature = "gvariant")]
            (Signature::Maybe(a), Signature::Maybe(b)) => a.eq(&**b),
            _ => false,
        }
    }
}

impl Eq for Signature {}

impl PartialEq<&str> for Signature {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Signature::Unit => other.is_empty(),
            Self::Bool => *other == "b",
            Self::U8 => *other == "y",
            Self::I16 => *other == "n",
            Self::U16 => *other == "q",
            Self::I32 => *other == "i",
            Self::U32 => *other == "u",
            Self::I64 => *other == "x",
            Self::U64 => *other == "t",
            Self::F64 => *other == "d",
            Self::Str => *other == "s",
            Self::Signature => *other == "g",
            Self::ObjectPath => *other == "o",
            Self::Variant => *other == "v",
            #[cfg(unix)]
            Self::Fd => *other == "h",
            Self::Array(child) => {
                if other.len() < 2 || !other.starts_with('a') {
                    return false;
                }

                child.eq(&other[1..])
            }
            Self::Dict { key, value } => {
                if other.len() < 4 || !other.starts_with("a{") || !other.ends_with('}') {
                    return false;
                }

                let (key_str, value_str) = other[2..other.len() - 1].split_at(1);

                key.eq(key_str) && value.eq(value_str)
            }
            Self::Structure(fields) => {
                let string_len = self.string_len();
                // self.string_len() will always take `()` into account so it can't be a smaller
                // number than `other.len()`.
                if string_len < other.len()
                    // Their length is either equal (i-e `other` has outer `()`) or `other` has no
                    // outer `()`.
                    || (string_len != other.len() && string_len != other.len() + 2)
                {
                    return false;
                }

                let fields_str = if string_len == other.len() {
                    &other[1..other.len() - 1]
                } else {
                    // No outer `()`.
                    if other.is_empty() {
                        return false;
                    }

                    other
                };

                let mut start = 0;
                for field in fields.iter() {
                    let len = field.string_len();
                    let end = start + len;
                    if end > fields_str.len() {
                        return false;
                    }
                    if !field.eq(&fields_str[start..end]) {
                        return false;
                    }

                    start += len;
                }

                true
            }
            #[cfg(feature = "gvariant")]
            Self::Maybe(child) => {
                if other.len() < 2 || !other.starts_with('m') {
                    return false;
                }

                child.eq(&other[1..])
            }
        }
    }
}

impl PartialEq<str> for Signature {
    fn eq(&self, other: &str) -> bool {
        self.eq(&other)
    }
}

impl PartialOrd for Signature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Signature::Unit, Signature::Unit)
            | (Signature::U8, Signature::U8)
            | (Signature::Bool, Signature::Bool)
            | (Signature::I16, Signature::I16)
            | (Signature::U16, Signature::U16)
            | (Signature::I32, Signature::I32)
            | (Signature::U32, Signature::U32)
            | (Signature::I64, Signature::I64)
            | (Signature::U64, Signature::U64)
            | (Signature::F64, Signature::F64)
            | (Signature::Str, Signature::Str)
            | (Signature::Signature, Signature::Signature)
            | (Signature::ObjectPath, Signature::ObjectPath)
            | (Signature::Variant, Signature::Variant) => std::cmp::Ordering::Equal,
            #[cfg(unix)]
            (Signature::Fd, Signature::Fd) => std::cmp::Ordering::Equal,
            (Signature::Array(a), Signature::Array(b)) => a.cmp(b),
            (
                Signature::Dict {
                    key: key_a,
                    value: value_a,
                },
                Signature::Dict {
                    key: key_b,
                    value: value_b,
                },
            ) => match key_a.cmp(key_b) {
                std::cmp::Ordering::Equal => value_a.cmp(value_b),
                other => other,
            },
            (Signature::Structure(a), Signature::Structure(b)) => a.iter().cmp(b.iter()),
            #[cfg(feature = "gvariant")]
            (Signature::Maybe(a), Signature::Maybe(b)) => a.cmp(b),
            (_, _) => std::cmp::Ordering::Equal,
        }
    }
}

impl Serialize for Signature {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&str>::deserialize(deserializer).and_then(|s| {
            Signature::from_str(s).map_err(|e| serde::de::Error::custom(e.to_string()))
        })
    }
}
