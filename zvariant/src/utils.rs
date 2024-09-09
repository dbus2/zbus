use std::slice::SliceIndex;

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
pub(crate) const DICT_ENTRY_ALIGNMENT_DBUS: usize = 8;
/// The VARIANT type signature. Provided for manual signature creation.
pub const VARIANT_SIGNATURE_CHAR: char = 'v';
/// The VARIANT type signature, as a string. Provided for manual signature creation.
pub const VARIANT_SIGNATURE_STR: &str = "v";
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
        value <= (u32::MAX as usize),
        "{} too large for `u32`",
        value,
    );

    value as u32
}

pub(crate) fn usize_to_u8(value: usize) -> u8 {
    assert!(value <= (u8::MAX as usize), "{} too large for `u8`", value,);

    value as u8
}

pub(crate) fn f64_to_f32(value: f64) -> f32 {
    assert!(value <= (f32::MAX as f64), "{} too large for `f32`", value,);

    value as f32
}

/// Slice the given slice of bytes safely and return an error if the slice is too small.
pub(crate) fn subslice<I, T>(input: &[T], index: I) -> Result<&I::Output>
where
    I: SliceIndex<[T]>,
{
    input.get(index).ok_or(Error::OutOfBounds)
}
