//! A DBus/GVariant type signature parser and enum representation.
//!
//! This module provides a recursive-descent parser (based on `nom`)
//! for DBus/GVariant type signatures. The parser returns an enum that
//! represents the full type signature being reprsented by a given string
//! or sequence of bytes.

use std::collections::VecDeque;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::all_consuming,
    multi::many1, IResult,
};

use crate::{
    Basic, Fd, ObjectPath, Signature, Type, ARRAY_SIGNATURE_STR, STRUCT_SIG_END_STR,
    STRUCT_SIG_START_STR, VARIANT_SIGNATURE_CHAR, VARIANT_SIGNATURE_STR,
};

/// A parsed DBus/GVariant type signature.
#[derive(Debug, PartialEq, Clone)]
pub struct ParsedSignature(VecDeque<SignatureEntry>);

impl ParsedSignature {
    /// Get the next entry in the signature to
    /// be processed.
    pub fn next(&mut self) -> Option<SignatureEntry> {
        self.0.pop_front()
    }

    /// Check if the signature matches a given
    /// type `T`.
    pub fn matches<T: Type>(&self) -> bool {
        T::signature() == signature_string!(&self.to_string())
    }

    /// Parse a byte slice into a `ParsedSignature`. If the entire
    /// slice is not consumed by the parsing, an error is returned.
    pub fn parse_bytes(input: &[u8]) -> crate::Result<ParsedSignature> {
        match SignatureEntry::parse(input) {
            Ok(entries) => Ok(ParsedSignature(entries)),
            Err(err) => {
                let reason = format!("Failed to parse signature. Reason: {:?}", err);
                Err(crate::Error::Message(reason))
            }
        }
    }

    /// Parse a string slice into a `ParsedSignature`. If the entire
    /// string is not consumed by the parsing, an error is returned.
    pub fn parse_str(input: &str) -> crate::Result<ParsedSignature> {
        Self::parse_bytes(input.as_bytes())
    }
}

impl std::fmt::Display for ParsedSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.0 {
            write!(f, "{}", entry)?;
        }

        Ok(())
    }
}

impl<'a> From<ParsedSignature> for Signature<'a> {
    fn from(parsed: ParsedSignature) -> Self {
        signature_string!(&parsed.to_string())
    }
}

impl<'a> From<&ParsedSignature> for Signature<'a> {
    fn from(parsed: &ParsedSignature) -> Self {
        signature_string!(&parsed.to_string())
    }
}

impl From<SignatureEntry> for ParsedSignature {
    fn from(entry: SignatureEntry) -> Self {
        ParsedSignature(vec![entry].into())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SignatureEntry {
    U8,
    U16,
    U32,
    U64,
    I16,
    I32,
    I64,
    F64,
    Bool,
    Str,
    ObjectPath,
    Signature,
    Variant,
    Array(Box<SignatureEntry>),
    Struct(VecDeque<SignatureEntry>),
    DictEntry(Box<SignatureEntry>, Box<SignatureEntry>),

    #[cfg(unix)]
    Fd,

    #[cfg(feature = "gvariant")]
    Maybe(Box<SignatureEntry>),
}

impl SignatureEntry {
    pub fn matches<T: Type>(&self) -> bool {
        T::signature() == signature_string!(&self.to_string())
    }

    pub fn parse(input: &[u8]) -> crate::Result<VecDeque<SignatureEntry>> {
        match Self::parse_all(input) {
            Ok((_, parsed)) => Ok(parsed),
            Err(err) => {
                let reason = format!("Failed to parse signature. Reason: {:?}", err);
                Err(crate::Error::Message(reason))
            }
        }
    }

    fn basic(input: &[u8]) -> IResult<&[u8], SignatureEntry> {
        let (input, next) = anychar(input)?;

        match next {
            u8::SIGNATURE_CHAR => Ok((input, SignatureEntry::U8)),
            u16::SIGNATURE_CHAR => Ok((input, SignatureEntry::U16)),
            u32::SIGNATURE_CHAR => Ok((input, SignatureEntry::U32)),
            u64::SIGNATURE_CHAR => Ok((input, SignatureEntry::U64)),
            i16::SIGNATURE_CHAR => Ok((input, SignatureEntry::I16)),
            i32::SIGNATURE_CHAR => Ok((input, SignatureEntry::I32)),
            i64::SIGNATURE_CHAR => Ok((input, SignatureEntry::I64)),
            f64::SIGNATURE_CHAR => Ok((input, SignatureEntry::F64)),
            bool::SIGNATURE_CHAR => Ok((input, SignatureEntry::Bool)),
            String::SIGNATURE_CHAR => Ok((input, SignatureEntry::Str)),
            ObjectPath::SIGNATURE_CHAR => Ok((input, SignatureEntry::ObjectPath)),
            Signature::SIGNATURE_CHAR => Ok((input, SignatureEntry::Signature)),
            VARIANT_SIGNATURE_CHAR => Ok((input, SignatureEntry::Variant)),

            #[cfg(unix)]
            Fd::SIGNATURE_CHAR => Ok((input, SignatureEntry::Fd)),

            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Char,
            ))),
        }
    }

    fn container_array(input: &[u8]) -> IResult<&[u8], SignatureEntry> {
        let (input, _) = tag(ARRAY_SIGNATURE_STR)(input)?;
        let (input, sig) = Self::array_element(input)?;
        Ok((input, SignatureEntry::Array(Box::new(sig))))
    }

    fn container_struct(input: &[u8]) -> IResult<&[u8], SignatureEntry> {
        let (input, _) = tag(STRUCT_SIG_START_STR)(input)?;
        let (input, sigs) = Self::parse_many(input)?;
        let (input, _) = tag(STRUCT_SIG_END_STR)(input)?;

        Ok((input, SignatureEntry::Struct(sigs)))
    }

    fn container_dict_entry(input: &[u8]) -> IResult<&[u8], SignatureEntry> {
        let (input, _) = tag("{")(input)?;
        let (input, key) = Self::parse_one(input)?;
        let (input, value) = Self::parse_one(input)?;
        let (input, _) = tag("}")(input)?;

        Ok((
            input,
            SignatureEntry::DictEntry(Box::new(key), Box::new(value)),
        ))
    }

    fn array_element(input: &[u8]) -> IResult<&[u8], SignatureEntry> {
        alt((
            Self::basic,
            Self::container_array,
            Self::container_struct,
            Self::container_dict_entry,
            #[cfg(feature = "gvariant")]
            Self::maybe,
        ))(input)
    }

    #[cfg(feature = "gvariant")]
    fn maybe(input: &[u8]) -> IResult<&[u8], SignatureEntry> {
        let (input, _) = tag("m")(input)?;
        let (input, sig) = Self::parse_one(input)?;

        Ok((input, SignatureEntry::Maybe(Box::new(sig))))
    }

    fn parse_one(input: &[u8]) -> IResult<&[u8], SignatureEntry> {
        alt((
            Self::basic,
            Self::container_array,
            Self::container_struct,
            #[cfg(feature = "gvariant")]
            Self::maybe,
        ))(input)
    }

    fn parse_many(input: &[u8]) -> IResult<&[u8], VecDeque<SignatureEntry>> {
        let (input, entries) = many1(Self::parse_one)(input)?;
        Ok((input, entries.into()))
    }

    fn parse_all(input: &[u8]) -> IResult<&[u8], VecDeque<SignatureEntry>> {
        all_consuming(Self::parse_many)(input)
    }
}

impl<'a> From<SignatureEntry> for Signature<'a> {
    fn from(parsed: SignatureEntry) -> Self {
        signature_string!(&parsed.to_string())
    }
}

impl<'a> From<&SignatureEntry> for Signature<'a> {
    fn from(parsed: &SignatureEntry) -> Self {
        signature_string!(&parsed.to_string())
    }
}

impl<'a> From<&mut SignatureEntry> for Signature<'a> {
    fn from(parsed: &mut SignatureEntry) -> Self {
        signature_string!(&parsed.to_string())
    }
}

impl std::fmt::Display for SignatureEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignatureEntry::U8 => f.write_str(u8::SIGNATURE_STR),
            SignatureEntry::U16 => f.write_str(u16::SIGNATURE_STR),
            SignatureEntry::U32 => f.write_str(u32::SIGNATURE_STR),
            SignatureEntry::U64 => f.write_str(u64::SIGNATURE_STR),
            SignatureEntry::I16 => f.write_str(i16::SIGNATURE_STR),
            SignatureEntry::I32 => f.write_str(i32::SIGNATURE_STR),
            SignatureEntry::I64 => f.write_str(i64::SIGNATURE_STR),
            SignatureEntry::F64 => f.write_str(f64::SIGNATURE_STR),
            SignatureEntry::Bool => f.write_str(bool::SIGNATURE_STR),
            SignatureEntry::Str => f.write_str(String::SIGNATURE_STR),
            SignatureEntry::ObjectPath => f.write_str(ObjectPath::SIGNATURE_STR),
            SignatureEntry::Signature => f.write_str(Signature::SIGNATURE_STR),
            SignatureEntry::Variant => f.write_str(VARIANT_SIGNATURE_STR),
            SignatureEntry::Array(sig) => write!(f, "a{}", sig),
            SignatureEntry::Struct(fields) => {
                f.write_str("(")?;

                for field in fields {
                    write!(f, "{}", field)?;
                }

                f.write_str(")")
            }
            SignatureEntry::DictEntry(key, value) => write!(f, "{{{}{}}}", key, value),

            #[cfg(unix)]
            SignatureEntry::Fd => f.write_str(Fd::SIGNATURE_STR),

            #[cfg(feature = "gvariant")]
            SignatureEntry::Maybe(sig) => write!(f, "m{}", sig),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Value;

    use super::*;

    #[test]
    fn parse_signature_bool() {
        let parsed = ParsedSignature::parse_str("b").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::Bool].into()));
    }

    #[test]
    fn parse_signature_u8() {
        let parsed = ParsedSignature::parse_str("y").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::U8].into()));
    }

    #[test]
    fn parse_signature_u16() {
        let parsed = ParsedSignature::parse_str("q").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::U16].into()));
    }

    #[test]
    fn parse_signature_u32() {
        let parsed = ParsedSignature::parse_str("u").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::U32].into()));
    }

    #[test]
    fn parse_signature_u64() {
        let parsed = ParsedSignature::parse_str("t").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::U64].into()));
    }

    #[test]
    fn parse_signature_i16() {
        let parsed = ParsedSignature::parse_str("n").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::I16].into()));
    }

    #[test]
    fn parse_signature_i32() {
        let parsed = ParsedSignature::parse_str("i").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::I32].into()));
    }

    #[test]
    fn parse_signature_i64() {
        let parsed = ParsedSignature::parse_str("x").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::I64].into()));
    }

    #[test]
    fn parse_signature_double() {
        let parsed = ParsedSignature::parse_str("d").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::F64].into()));
    }

    #[test]
    #[cfg(unix)]
    fn parse_signature_fd() {
        let parsed = ParsedSignature::parse_str("h").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::Fd].into()));
    }

    #[test]
    fn parse_signature_string() {
        let parsed = ParsedSignature::parse_str("s").unwrap();
        assert_eq!(parsed, ParsedSignature(vec![SignatureEntry::Str].into()));
    }

    #[test]
    fn parse_signature_object_path() {
        let parsed = ParsedSignature::parse_str("o").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(vec![SignatureEntry::ObjectPath].into())
        );
    }

    #[test]
    fn parse_signature_signature() {
        let parsed = ParsedSignature::parse_str("g").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(vec![SignatureEntry::Signature].into())
        );
    }

    #[test]
    fn parse_signature_variant() {
        let parsed = ParsedSignature::parse_str("v").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(vec![SignatureEntry::Variant].into())
        );
    }

    #[test]
    fn parse_signature_array() {
        let parsed = ParsedSignature::parse_str("ai").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(vec![SignatureEntry::Array(Box::new(SignatureEntry::I32))].into())
        );
    }

    #[test]
    fn parse_signature_struct() {
        let parsed = ParsedSignature::parse_str("(iu)").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(
                vec![SignatureEntry::Struct(
                    vec![SignatureEntry::I32, SignatureEntry::U32].into()
                )]
                .into()
            )
        );
    }

    #[test]
    fn parse_nested_struct() {
        let parsed = ParsedSignature::parse_str("(i(iu))").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(
                vec![SignatureEntry::Struct(
                    vec![
                        SignatureEntry::I32,
                        SignatureEntry::Struct(
                            vec![SignatureEntry::I32, SignatureEntry::U32].into()
                        )
                    ]
                    .into()
                )]
                .into()
            )
        );
    }

    #[test]
    fn parse_signature_dict() {
        let parsed = ParsedSignature::parse_str("a{is}").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(
                vec![SignatureEntry::Array(Box::new(SignatureEntry::DictEntry(
                    Box::new(SignatureEntry::I32),
                    Box::new(SignatureEntry::Str)
                )))]
                .into()
            )
        );
    }

    #[test]
    fn parse_nested_dict() {
        let parsed = ParsedSignature::parse_str("a{ia{is}}").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(
                vec![SignatureEntry::Array(Box::new(SignatureEntry::DictEntry(
                    Box::new(SignatureEntry::I32),
                    Box::new(SignatureEntry::Array(Box::new(SignatureEntry::DictEntry(
                        Box::new(SignatureEntry::I32),
                        Box::new(SignatureEntry::Str)
                    ))))
                )))]
                .into()
            )
        );
    }

    #[test]
    fn parse_array_of_structs() {
        let parsed = ParsedSignature::parse_str("a(iu)").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(
                vec![SignatureEntry::Array(Box::new(SignatureEntry::Struct(
                    vec![SignatureEntry::I32, SignatureEntry::U32].into()
                )))]
                .into()
            )
        );
    }

    #[test]
    #[cfg(feature = "gvariant")]
    fn parse_maybe() {
        let parsed = ParsedSignature::parse_str("ms").unwrap();
        assert_eq!(
            parsed,
            ParsedSignature(vec![SignatureEntry::Maybe(Box::new(SignatureEntry::Str))].into())
        );
    }

    #[test]
    fn fails_parse_lone_dictentry() {
        assert!(ParsedSignature::parse_str("{is}").is_err());
    }

    #[test]
    fn fails_parse_unclosed_dict() {
        assert!(ParsedSignature::parse_str("a{is").is_err());
    }

    #[test]
    fn fails_parse_unopened_dict() {
        assert!(ParsedSignature::parse_str("ais}").is_err());
    }

    #[test]
    fn fails_parse_unopened_struct() {
        assert!(ParsedSignature::parse_str("is)").is_err());
    }

    #[test]
    fn fails_parse_invalid_char() {
        assert!(ParsedSignature::parse_str("p").is_err());
    }

    #[test]
    fn next() {
        let mut parsed = ParsedSignature::parse_str("ius").unwrap();
        assert_eq!(parsed.next(), Some(SignatureEntry::I32));
        assert_eq!(parsed.next(), Some(SignatureEntry::U32));
        assert_eq!(parsed.next(), Some(SignatureEntry::Str));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    fn matches_bool() {
        let parsed = ParsedSignature::parse_str("b").unwrap();
        assert!(parsed.matches::<bool>());
        assert!(!parsed.matches::<u8>());
    }

    #[test]
    fn matches_u8() {
        let parsed = ParsedSignature::parse_str("y").unwrap();
        assert!(parsed.matches::<u8>());
        assert!(!parsed.matches::<u16>());
    }

    #[test]
    fn matches_u16() {
        let parsed = ParsedSignature::parse_str("q").unwrap();
        assert!(parsed.matches::<u16>());
        assert!(!parsed.matches::<u8>());
    }

    #[test]
    fn matches_u32() {
        let parsed = ParsedSignature::parse_str("u").unwrap();
        assert!(parsed.matches::<u32>());
        assert!(!parsed.matches::<u16>());
    }

    #[test]
    fn matches_u64() {
        let parsed = ParsedSignature::parse_str("t").unwrap();
        assert!(parsed.matches::<u64>());
        assert!(!parsed.matches::<u32>());
    }

    #[test]
    fn matches_i16() {
        let parsed = ParsedSignature::parse_str("n").unwrap();
        assert!(parsed.matches::<i16>());
        assert!(!parsed.matches::<u16>());
    }

    #[test]
    fn matches_i32() {
        let parsed = ParsedSignature::parse_str("i").unwrap();
        assert!(parsed.matches::<i32>());
        assert!(!parsed.matches::<u32>());
    }

    #[test]
    fn matches_i64() {
        let parsed = ParsedSignature::parse_str("x").unwrap();
        assert!(parsed.matches::<i64>());
        assert!(!parsed.matches::<u64>());
    }

    #[test]
    fn matches_double() {
        let parsed = ParsedSignature::parse_str("d").unwrap();
        assert!(parsed.matches::<f64>());
        assert!(!parsed.matches::<i64>());
    }

    #[test]
    #[cfg(unix)]
    fn matches_fd() {
        let parsed = ParsedSignature::parse_str("h").unwrap();
        assert!(parsed.matches::<Fd<'_>>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    fn matches_string() {
        let parsed = ParsedSignature::parse_str("s").unwrap();
        assert!(parsed.matches::<String>());
        assert!(parsed.matches::<&str>());
        assert!(parsed.matches::<&'static str>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    fn matches_object_path() {
        let parsed = ParsedSignature::parse_str("o").unwrap();
        assert!(parsed.matches::<ObjectPath<'_>>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    fn matches_signature() {
        let parsed = ParsedSignature::parse_str("g").unwrap();
        assert!(parsed.matches::<Signature<'_>>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    fn matches_variant() {
        let parsed = ParsedSignature::parse_str("v").unwrap();
        assert!(parsed.matches::<Value<'_>>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    fn matches_array() {
        let parsed = ParsedSignature::parse_str("ai").unwrap();
        assert!(parsed.matches::<Vec<i32>>());
        assert!(!parsed.matches::<Vec<u8>>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    fn matches_struct() {
        let parsed = ParsedSignature::parse_str("(iu)").unwrap();
        assert!(parsed.matches::<(i32, u32)>());
        assert!(!parsed.matches::<(u32, i32)>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    fn matches_dict() {
        let parsed = ParsedSignature::parse_str("a{is}").unwrap();
        assert!(parsed.matches::<std::collections::HashMap<i32, String>>());
        assert!(!parsed.matches::<std::collections::HashMap<u32, String>>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    #[cfg(all(feature = "gvariant", not(feature = "option-as_array")))]
    fn matches_maybe() {
        let parsed = ParsedSignature::parse_str("ms").unwrap();
        assert_eq!(Option::<String>::signature().as_str(), "ms");
        assert!(parsed.matches::<Option<String>>());
        assert!(!parsed.matches::<Option<u8>>());
        assert!(!parsed.matches::<String>());
        assert!(!parsed.matches::<f64>());
    }

    #[test]
    #[cfg(all(feature = "option-as-array", not(feature = "gvariant")))]
    fn matches_maybe() {
        let parsed = ParsedSignature::parse_str("as").unwrap();
        assert!(parsed.matches::<Option<String>>());
        assert!(!parsed.matches::<Option<u8>>());
        assert!(!parsed.matches::<String>());
        assert!(!parsed.matches::<f64>());
    }
}
