use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
    fmt,
};

use super::{Error, Result};

/// Percent-encode the value.
pub fn encode_percents(f: &mut dyn fmt::Write, value: &[u8]) -> std::fmt::Result {
    for &byte in value {
        if matches!(byte, b'-' | b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'_' | b'/' | b'.' | b'\\' | b'*')
        {
            // Write the byte directly if it's in the allowed set
            f.write_char(byte as char)?;
        } else {
            // Otherwise, write its percent-encoded form
            write!(f, "%{:02X}", byte)?;
        }
    }

    Ok(())
}

/// Percent-decode the string.
pub fn decode_percents(value: &str) -> Result<Cow<'_, [u8]>> {
    // Check if decoding is necessary
    let needs_decoding = value.chars().any(|c| c == '%' || !is_allowed_char(c));

    if !needs_decoding {
        return Ok(Cow::Borrowed(value.as_bytes()));
    }

    let mut decoded = Vec::with_capacity(value.len());
    let mut chars = value.chars();

    while let Some(c) = chars.next() {
        match c {
            '%' => {
                let high = chars
                    .next()
                    .ok_or_else(|| Error::Encoding("Incomplete percent-encoded sequence".into()))?;
                let low = chars
                    .next()
                    .ok_or_else(|| Error::Encoding("Incomplete percent-encoded sequence".into()))?;
                decoded.push(decode_hex_pair(high, low)?);
            }
            _ if is_allowed_char(c) => decoded.push(c as u8),
            _ => return Err(Error::Encoding("Invalid character in address".into())),
        }
    }

    Ok(Cow::Owned(decoded))
}

// A trait for types that can be percent-encoded and written to a [`fmt::Formatter`].
pub(crate) trait Encodable {
    fn encode(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<T: ToString> Encodable for T {
    fn encode(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        encode_percents(f, self.to_string().as_bytes())
    }
}

pub(crate) struct EncData<T: ?Sized>(pub T);

impl<T: AsRef<[u8]>> Encodable for EncData<T> {
    fn encode(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        encode_percents(f, self.0.as_ref())
    }
}

pub(crate) struct EncOsStr<T: ?Sized>(pub T);

impl Encodable for EncOsStr<&Cow<'_, OsStr>> {
    fn encode(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        encode_percents(f, self.0.to_string_lossy().as_bytes())
    }
}

impl Encodable for EncOsStr<&OsStr> {
    fn encode(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        encode_percents(f, self.0.to_string_lossy().as_bytes())
    }
}

fn is_allowed_char(c: char) -> bool {
    matches!(c, '-' | '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' | '/' | '.' | '\\' | '*')
}

fn decode_hex_pair(high: char, low: char) -> Result<u8> {
    let high_digit = decode_hex(high)?;
    let low_digit = decode_hex(low)?;

    Ok(high_digit << 4 | low_digit)
}

fn decode_hex(c: char) -> Result<u8> {
    match c {
        '0'..='9' => Ok(c as u8 - b'0'),
        'a'..='f' => Ok(c as u8 - b'a' + 10),
        'A'..='F' => Ok(c as u8 - b'A' + 10),

        _ => Err(Error::Encoding(
            "Invalid hexadecimal character in percent-encoded sequence".into(),
        )),
    }
}

pub(super) fn decode_percents_str(value: &str) -> Result<Cow<'_, str>> {
    cow_bytes_to_str(decode_percents(value)?)
}

fn cow_bytes_to_str(cow: Cow<'_, [u8]>) -> Result<Cow<'_, str>> {
    match cow {
        Cow::Borrowed(bytes) => Ok(Cow::Borrowed(
            std::str::from_utf8(bytes).map_err(|e| Error::Encoding(format!("{e}")))?,
        )),
        Cow::Owned(bytes) => Ok(Cow::Owned(
            String::from_utf8(bytes).map_err(|e| Error::Encoding(format!("{e}")))?,
        )),
    }
}

pub(super) fn decode_percents_os_str(value: &str) -> Result<Cow<'_, OsStr>> {
    cow_bytes_to_os_str(decode_percents(value)?)
}

fn cow_bytes_to_os_str(cow: Cow<'_, [u8]>) -> Result<Cow<'_, OsStr>> {
    match cow {
        Cow::Borrowed(bytes) => Ok(Cow::Borrowed(OsStr::new(
            std::str::from_utf8(bytes).map_err(|e| Error::Encoding(format!("{e}")))?,
        ))),
        Cow::Owned(bytes) => Ok(Cow::Owned(OsString::from(
            String::from_utf8(bytes).map_err(|e| Error::Encoding(format!("{e}")))?,
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_ascii() {
        const INPUT: &[u8] = "hello".as_bytes();

        let mut output = String::new();
        encode_percents(&mut output, INPUT).unwrap();
        assert_eq!(output, "hello");

        let result = decode_percents(&output).unwrap();
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, Cow::Borrowed(INPUT));
    }

    #[test]
    fn special_characters() {
        const INPUT: &[u8] = "hello world!".as_bytes();

        let mut output = String::new();
        encode_percents(&mut output, INPUT).unwrap();
        assert_eq!(output, "hello%20world%21");

        let result = decode_percents(&output).unwrap();
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, Cow::Borrowed(INPUT));
    }

    #[test]
    fn empty_input() {
        const INPUT: &[u8] = "".as_bytes();

        let mut output = String::new();
        encode_percents(&mut output, INPUT).unwrap();
        assert_eq!(output, "");

        let result = decode_percents(&output).unwrap();
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, Cow::Borrowed(INPUT));
    }

    #[test]
    fn non_ascii_characters() {
        const INPUT: &[u8] = "😊".as_bytes();

        let mut output = String::new();
        encode_percents(&mut output, INPUT).unwrap();
        assert_eq!(output, "%F0%9F%98%8A");

        let result = decode_percents(&output).unwrap();
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, Cow::Borrowed(INPUT));
    }

    #[test]
    fn incomplete_encoding() {
        let result = decode_percents("incomplete%");
        assert!(result.is_err());
    }

    #[test]
    fn invalid_characters() {
        let result = decode_percents("invalid%2Gchar");
        assert!(result.is_err());
    }
}
