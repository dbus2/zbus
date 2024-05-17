use std::slice::SliceIndex;

#[cfg(feature = "gvariant")]
use crate::signature_parser::SignatureParser;
use crate::{Error, Result};

/// The prefix of ARRAY type signature, as a character. Provided for manual signature creation.
pub const ARRAY_SIGNATURE_CHAR: char = 'a';
/// The prefix of ARRAY type signature, as a string. Provided for manual signature creation.
pub const ARRAY_SIGNATURE_STR: &str = "a";
pub(crate) const ARRAY_ALIGNMENT_DBUS: usize = 4;
/// The opening character of STRUCT type signature. Provided for manual signature creation.
pub const STRUCT_SIG_START_CHAR: char = '(';
/// The closing character of STRUCT type signature. Provided for manual signature creation.
pub const STRUCT_SIG_END_CHAR: char = ')';
/// The opening character of STRUCT type signature, as a string. Provided for manual signature
/// creation.
pub const STRUCT_SIG_START_STR: &str = "(";
/// The closing character of STRUCT type signature, as a string. Provided for manual signature
/// creation.
pub const STRUCT_SIG_END_STR: &str = ")";
pub(crate) const STRUCT_ALIGNMENT_DBUS: usize = 8;
/// The opening character of DICT_ENTRY type signature. Provided for manual signature creation.
pub const DICT_ENTRY_SIG_START_CHAR: char = '{';
/// The closing character of DICT_ENTRY type signature. Provided for manual signature creation.
pub const DICT_ENTRY_SIG_END_CHAR: char = '}';
/// The opening character of DICT_ENTRY type signature, as a string. Provided for manual signature
/// creation.
pub const DICT_ENTRY_SIG_START_STR: &str = "{";
/// The closing character of DICT_ENTRY type signature, as a string. Provided for manual signature
/// creation.
pub const DICT_ENTRY_SIG_END_STR: &str = "}";
/// The VARIANT type signature. Provided for manual signature creation.
pub const VARIANT_SIGNATURE_CHAR: char = 'v';
/// The VARIANT type signature, as a string. Provided for manual signature creation.
pub const VARIANT_SIGNATURE_STR: &str = "v";
pub(crate) const VARIANT_ALIGNMENT_DBUS: usize = 1;
#[cfg(feature = "gvariant")]
pub(crate) const VARIANT_ALIGNMENT_GVARIANT: usize = 8;
/// The prefix of MAYBE (GVariant-specific) type signature, as a character. Provided for manual
/// signature creation.
#[cfg(feature = "gvariant")]
pub const MAYBE_SIGNATURE_CHAR: char = 'm';
/// The prefix of MAYBE (GVariant-specific) type signature, as a string. Provided for manual
/// signature creation.
#[cfg(feature = "gvariant")]
pub const MAYBE_SIGNATURE_STR: &str = "m";

pub(crate) fn padding_for_n_bytes(value: usize, align: usize) -> usize {
    let len_rounded_up = value.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);

    len_rounded_up.wrapping_sub(value)
}

pub(crate) fn usize_to_u32(value: usize) -> u32 {
    assert!(
        value <= (std::u32::MAX as usize),
        "{} too large for `u32`",
        value,
    );

    value as u32
}

pub(crate) fn usize_to_u8(value: usize) -> u8 {
    assert!(
        value <= (std::u8::MAX as usize),
        "{} too large for `u8`",
        value,
    );

    value as u8
}

#[cfg(feature = "gvariant")]
pub(crate) fn is_fixed_sized_signature<'a>(signature: &'a Signature<'a>) -> Result<bool> {
    match signature
        .as_bytes()
        .first()
        .map(|b| *b as char)
        .ok_or_else(|| -> Error { serde::de::Error::invalid_length(0, &">= 1 character") })?
    {
        u8::SIGNATURE_CHAR
        | bool::SIGNATURE_CHAR
        | i16::SIGNATURE_CHAR
        | u16::SIGNATURE_CHAR
        | i32::SIGNATURE_CHAR
        | u32::SIGNATURE_CHAR
        | i64::SIGNATURE_CHAR
        | u64::SIGNATURE_CHAR
        | f64::SIGNATURE_CHAR => Ok(true),
        #[cfg(unix)]
        Fd::SIGNATURE_CHAR => Ok(true),
        STRUCT_SIG_START_CHAR => is_fixed_sized_struct_signature(signature),
        DICT_ENTRY_SIG_START_CHAR => is_fixed_sized_dict_entry_signature(signature),
        _ => Ok(false),
    }
}

// Given an &str, create an owned (String-based) Signature w/ appropriate capacity
macro_rules! signature_string {
    ($signature:expr) => {{
        let mut s = String::with_capacity(255);
        s.push_str($signature);

        Signature::from_string_unchecked(s)
    }};
}

macro_rules! check_child_value_signature {
    ($expected_signature:expr, $child_signature:expr, $child_name:literal) => {{
        if $expected_signature.as_str() != "v" && $child_signature != $expected_signature {
            let unexpected = format!("{} with signature `{}`", $child_name, $child_signature,);
            let expected = format!("{} with signature `{}`", $child_name, $expected_signature);

            return Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Str(&unexpected),
                &expected.as_str(),
            ));
        }
    }};
}

#[cfg(feature = "gvariant")]
fn alignment_for_maybe_signature(signature: &Signature<'_>, format: Format) -> Result<usize> {
    alignment_for_single_child_type_signature(signature, format, 1)
}

#[cfg(feature = "gvariant")]
fn is_fixed_sized_struct_signature<'a>(signature: &'a Signature<'a>) -> Result<bool> {
    let inner_signature = Signature::from_str_unchecked(&signature[1..signature.len() - 1]);
    let mut sig_parser = SignatureParser::new(inner_signature);
    let mut fixed_sized = true;

    while !sig_parser.done() {
        let child_signature = sig_parser.parse_next_signature()?;

        if !is_fixed_sized_signature(&child_signature)? {
            // STRUCT is fixed-sized only if all its children are
            fixed_sized = false;

            break;
        }
    }

    Ok(fixed_sized)
}

#[cfg(feature = "gvariant")]
fn is_fixed_sized_dict_entry_signature<'a>(signature: &'a Signature<'a>) -> Result<bool> {
    let key_signature = Signature::from_str_unchecked(&signature[1..2]);
    if !is_fixed_sized_signature(&key_signature)? {
        return Ok(false);
    }

    let value_signature = Signature::from_str_unchecked(&signature[2..signature.len() - 1]);

    is_fixed_sized_signature(&value_signature)
}

/// Slice the given slice of bytes safely and return an error if the slice is too small.
pub(crate) fn subslice<I, T>(input: &[T], index: I) -> Result<&I::Output>
where
    I: SliceIndex<[T]>,
{
    input.get(index).ok_or(Error::OutOfBounds)
}
