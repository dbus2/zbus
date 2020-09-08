use crate::{Basic, EncodingFormat, Error, Fd, ObjectPath, Signature};

/// The prefix of ARRAY type signature, as a character. Provided for manual signature creation.
pub const ARRAY_SIGNATURE_CHAR: char = 'a';
/// The prefix of ARRAY type signature, as a string. Provided for manual signature creation.
pub const ARRAY_SIGNATURE_STR: &str = "a";
pub(crate) const ARRAY_ALIGNMENT_DBUS: usize = 4;
/// The opening character of STRUCT type signature. Provided for manual signature creation.
pub const STRUCT_SIG_START_CHAR: char = '(';
/// The closing character of STRUCT type signature. Provided for manual signature creation.
pub const STRUCT_SIG_END_CHAR: char = ')';
/// The opening character of STRUCT type signature, as a string. Provided for manual signature creation.
pub const STRUCT_SIG_START_STR: &str = "(";
/// The closing character of STRUCT type signature, as a string. Provided for manual signature creation.
pub const STRUCT_SIG_END_STR: &str = ")";
pub(crate) const STRUCT_ALIGNMENT_DBUS: usize = 8;
/// The opening character of DICT_ENTRY type signature. Provided for manual signature creation.
pub const DICT_ENTRY_SIG_START_CHAR: char = '{';
/// The closing character of DICT_ENTRY type signature. Provided for manual signature creation.
pub const DICT_ENTRY_SIG_END_CHAR: char = '}';
/// The opening character of DICT_ENTRY type signature, as a string. Provided for manual signature creation.
pub const DICT_ENTRY_SIG_START_STR: &str = "{";
/// The closing character of DICT_ENTRY type signature, as a string. Provided for manual signature creation.
pub const DICT_ENTRY_SIG_END_STR: &str = "}";
pub(crate) const DICT_ENTRY_ALIGNMENT_DBUS: usize = 8;
/// The VARIANT type signature. Provided for manual signature creation.
pub const VARIANT_SIGNATURE_CHAR: char = 'v';
/// The VARIANT type signature, as a string. Provided for manual signature creation.
pub const VARIANT_SIGNATURE_STR: &str = "v";
pub(crate) const VARIANT_ALIGNMENT_DBUS: usize = 1;
pub(crate) const VARIANT_ALIGNMENT_GVARIANT: usize = 8;
/// The prefix of MAYBE (GVariant-specific) type signature, as a character. Provided for manual
/// signature creation.
pub const MAYBE_SIGNATURE_CHAR: char = 'm';
/// The prefix of MAYBE (GVariant-specific) type signature, as a string. Provided for manual
/// signature creation.
pub const MAYBE_SIGNATURE_STR: &str = "m";

pub(crate) fn padding_for_n_bytes(value: usize, align: usize) -> usize {
    let len_rounded_up = value.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);

    len_rounded_up.wrapping_sub(value)
}

pub(crate) fn usize_to_u32(value: usize) -> u32 {
    if value > (std::u32::MAX as usize) {
        panic!("{} too large for `u32`", value);
    }

    value as u32
}

pub(crate) fn usize_to_u8(value: usize) -> u8 {
    if value > (std::u8::MAX as usize) {
        panic!("{} too large for `u8`", value);
    }

    value as u8
}

pub(crate) fn f64_to_f32(value: f64) -> f32 {
    if value > (std::f32::MAX as f64) {
        panic!("{} too large for `f32`", value);
    }

    value as f32
}

// `signature` must be **one** complete and correct signature. Expect panics otherwise!
pub(crate) fn alignment_for_signature(signature: &Signature, format: EncodingFormat) -> usize {
    match signature
        .as_bytes()
        .first()
        .map(|b| *b as char)
        .expect("alignment_for_signature expects **one** complete & correct signature")
    {
        u8::SIGNATURE_CHAR => u8::alignment(format),
        bool::SIGNATURE_CHAR => bool::alignment(format),
        i16::SIGNATURE_CHAR => i16::alignment(format),
        u16::SIGNATURE_CHAR => u16::alignment(format),
        i32::SIGNATURE_CHAR => i32::alignment(format),
        u32::SIGNATURE_CHAR | Fd::SIGNATURE_CHAR => u32::alignment(format),
        i64::SIGNATURE_CHAR => i64::alignment(format),
        u64::SIGNATURE_CHAR => u64::alignment(format),
        f64::SIGNATURE_CHAR => f64::alignment(format),
        <&str>::SIGNATURE_CHAR => <&str>::alignment(format),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::alignment(format),
        Signature::SIGNATURE_CHAR => Signature::alignment(format),
        VARIANT_SIGNATURE_CHAR => match format {
            EncodingFormat::DBus => VARIANT_ALIGNMENT_DBUS,
            EncodingFormat::GVariant => VARIANT_ALIGNMENT_GVARIANT,
        },
        ARRAY_SIGNATURE_CHAR => alignment_for_array_signature(signature, format),
        STRUCT_SIG_START_CHAR => alignment_for_struct_signature(signature, format),
        DICT_ENTRY_SIG_START_CHAR => alignment_for_dict_entry_signature(signature, format),
        MAYBE_SIGNATURE_CHAR => alignment_for_maybe_signature(signature, format),
        _ => {
            println!("WARNING: Unsupported signature: {}", signature);

            0
        }
    }
}

pub(crate) fn slice_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    match signature
        .as_bytes()
        .first()
        .map(|b| *b as char)
        .ok_or_else(|| serde::de::Error::invalid_length(0, &">= 1 character"))?
    {
        u8::SIGNATURE_CHAR
        | bool::SIGNATURE_CHAR
        | i16::SIGNATURE_CHAR
        | u16::SIGNATURE_CHAR
        | i32::SIGNATURE_CHAR
        | u32::SIGNATURE_CHAR
        | i64::SIGNATURE_CHAR
        | u64::SIGNATURE_CHAR
        | f64::SIGNATURE_CHAR
        | <&str>::SIGNATURE_CHAR
        | ObjectPath::SIGNATURE_CHAR
        | Signature::SIGNATURE_CHAR
        | Fd::SIGNATURE_CHAR
        | VARIANT_SIGNATURE_CHAR => Ok(Signature::from_str_unchecked(&signature[0..1])),
        ARRAY_SIGNATURE_CHAR => slice_array_signature(signature),
        STRUCT_SIG_START_CHAR => slice_structure_signature(signature),
        DICT_ENTRY_SIG_START_CHAR => slice_dict_entry_signature(signature),
        MAYBE_SIGNATURE_CHAR => slice_maybe_signature(signature),
        c => Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Char(c),
            &"a valid signature character",
        )),
    }
}

pub(crate) fn is_fixed_sized_signature<'a>(signature: &'a Signature<'a>) -> Result<bool, Error> {
    match signature
        .as_bytes()
        .first()
        .map(|b| *b as char)
        .ok_or_else(|| serde::de::Error::invalid_length(0, &">= 1 character"))?
    {
        u8::SIGNATURE_CHAR
        | bool::SIGNATURE_CHAR
        | i16::SIGNATURE_CHAR
        | u16::SIGNATURE_CHAR
        | i32::SIGNATURE_CHAR
        | u32::SIGNATURE_CHAR
        | i64::SIGNATURE_CHAR
        | u64::SIGNATURE_CHAR
        | f64::SIGNATURE_CHAR
        | Fd::SIGNATURE_CHAR => Ok(true),
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
        if $child_signature != $expected_signature {
            let unexpected = format!("{} with signature `{}`", $child_name, $child_signature,);
            let expected = format!("{} with signature `{}`", $child_name, $expected_signature);

            return Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Str(&unexpected),
                &expected.as_str(),
            ));
        }
    }};
}

fn slice_single_child_type_container_signature<'a>(
    signature: &'a Signature<'a>,
    expected_sig_prefix: char,
) -> Result<Signature<'a>, Error> {
    if signature.len() < 2 {
        return Err(serde::de::Error::invalid_length(
            signature.len(),
            &">= 2 characters",
        ));
    }

    // We can't get None here cause we already established there is are least 2 chars above
    let c = signature
        .as_bytes()
        .first()
        .map(|b| *b as char)
        .expect("empty signature");
    if c != expected_sig_prefix {
        return Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Char(c),
            &expected_sig_prefix.to_string().as_str(),
        ));
    }

    // There should be a valid complete signature after 'a' but not more than 1
    let slice_len = slice_signature(&Signature::from_str_unchecked(&signature[1..]))?.len();

    Ok(Signature::from_str_unchecked(&signature[0..=slice_len]))
}

fn slice_array_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    slice_single_child_type_container_signature(signature, ARRAY_SIGNATURE_CHAR)
}

fn slice_maybe_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    slice_single_child_type_container_signature(signature, MAYBE_SIGNATURE_CHAR)
}

fn slice_structure_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    if signature.len() < 2 {
        return Err(serde::de::Error::invalid_length(
            signature.len(),
            &">= 2 characters",
        ));
    }

    // We can't get None here cause we already established there are at least 2 chars above
    let c = signature
        .as_bytes()
        .first()
        .map(|b| *b as char)
        .expect("empty signature");
    if c != STRUCT_SIG_START_CHAR {
        return Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Char(c),
            &crate::STRUCT_SIG_START_STR,
        ));
    }

    let mut open_braces = 1;
    let mut i = 1;
    while i < signature.len() {
        if &signature[i..=i] == STRUCT_SIG_END_STR {
            open_braces -= 1;

            if open_braces == 0 {
                break;
            }
        } else if &signature[i..=i] == STRUCT_SIG_START_STR {
            open_braces += 1;
        }

        i += 1;
    }
    let end = &signature[i..=i];
    if end != STRUCT_SIG_END_STR {
        return Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(end),
            &crate::STRUCT_SIG_END_STR,
        ));
    }

    Ok(Signature::from_str_unchecked(&signature[0..=i]))
}

fn slice_dict_entry_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    if signature.len() < 4 {
        return Err(serde::de::Error::invalid_length(
            signature.len(),
            &">= 4 characters",
        ));
    }

    // We can't get None here cause we already established there are at least 4 chars above
    let c = signature
        .as_bytes()
        .first()
        .map(|b| *b as char)
        .expect("empty signature");
    if c != DICT_ENTRY_SIG_START_CHAR {
        return Err(serde::de::Error::invalid_value(
            serde::de::Unexpected::Char(c),
            &crate::DICT_ENTRY_SIG_START_STR,
        ));
    }

    // Key's signature will always be just 1 character so no need to slice for that.
    // There should be one valid complete signature for value.
    let slice_len = slice_signature(&Signature::from_str_unchecked(&signature[2..]))?.len();

    // signature of value + `{` + 1 char of the key signature + `}`
    Ok(Signature::from_str_unchecked(&signature[0..slice_len + 3]))
}

fn alignment_for_single_child_type_signature(
    signature: &Signature,
    format: EncodingFormat,
    dbus_align: usize,
) -> usize {
    match format {
        EncodingFormat::DBus => dbus_align,
        EncodingFormat::GVariant => {
            let child_signature = Signature::from_str_unchecked(&signature[1..]);

            alignment_for_signature(&child_signature, format)
        }
    }
}

fn alignment_for_array_signature(signature: &Signature, format: EncodingFormat) -> usize {
    alignment_for_single_child_type_signature(signature, format, ARRAY_ALIGNMENT_DBUS)
}

fn alignment_for_maybe_signature(signature: &Signature, format: EncodingFormat) -> usize {
    alignment_for_single_child_type_signature(signature, format, 1)
}

fn alignment_for_struct_signature(signature: &Signature, format: EncodingFormat) -> usize {
    match format {
        EncodingFormat::DBus => STRUCT_ALIGNMENT_DBUS,
        EncodingFormat::GVariant => {
            let inner_signature = &signature[1..signature.len() - 1];
            let mut parsed = 0;
            let mut alignment = 0;

            while parsed < inner_signature.len() {
                let rest_of_signature = Signature::from_str_unchecked(&inner_signature[parsed..]);
                let child_signature =
                    slice_signature(&rest_of_signature).expect("invalid signature");
                parsed += child_signature.len();

                let child_alignment = alignment_for_signature(&child_signature, format);
                if child_alignment > alignment {
                    alignment = child_alignment;

                    if alignment == 8 {
                        // 8 bytes is max alignment so we can short-circuit here
                        break;
                    }
                }
            }

            alignment
        }
    }
}

fn alignment_for_dict_entry_signature(signature: &Signature, format: EncodingFormat) -> usize {
    match format {
        EncodingFormat::DBus => DICT_ENTRY_ALIGNMENT_DBUS,
        EncodingFormat::GVariant => {
            let key_signature = Signature::from_str_unchecked(&signature[1..2]);
            let key_alignment = alignment_for_signature(&key_signature, format);
            if key_alignment == 8 {
                // 8 bytes is max alignment so we can short-circuit here
                return 8;
            }

            let value_signature = Signature::from_str_unchecked(&signature[2..signature.len() - 1]);
            let value_alignment = alignment_for_signature(&value_signature, format);
            if value_alignment > key_alignment {
                value_alignment
            } else {
                key_alignment
            }
        }
    }
}

fn is_fixed_sized_struct_signature<'a>(signature: &'a Signature<'a>) -> Result<bool, Error> {
    let inner_signature = &signature[1..signature.len() - 1];
    let mut parsed = 0;
    let mut fixed_sized = true;

    while parsed < inner_signature.len() {
        let rest_of_signature = Signature::from_str_unchecked(&inner_signature[parsed..]);
        let child_signature = slice_signature(&rest_of_signature).expect("invalid signature");
        parsed += child_signature.len();

        if !is_fixed_sized_signature(&child_signature)? {
            // STRUCT is fixed-sized only if all its children are
            fixed_sized = false;

            break;
        }
    }

    Ok(fixed_sized)
}

fn is_fixed_sized_dict_entry_signature<'a>(signature: &'a Signature<'a>) -> Result<bool, Error> {
    let key_signature = Signature::from_str_unchecked(&signature[1..2]);
    if !is_fixed_sized_signature(&key_signature)? {
        return Ok(false);
    }

    let value_signature = Signature::from_str_unchecked(&signature[2..signature.len() - 1]);

    is_fixed_sized_signature(&value_signature)
}
