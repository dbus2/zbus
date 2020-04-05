use crate::{Basic, EncodingFormat, Error, ObjectPath, Signature};

pub const ARRAY_SIGNATURE_CHAR: char = 'a';
pub const ARRAY_ALIGNMENT: usize = 4;
pub const STRUCT_SIG_START_CHAR: char = '(';
pub const STRUCT_SIG_END_CHAR: char = ')';
pub const STRUCT_SIG_START_STR: &str = "(";
pub const STRUCT_SIG_END_STR: &str = ")";
pub const STRUCT_ALIGNMENT: usize = 8;
pub const DICT_ENTRY_SIG_START_CHAR: char = '{';
pub const DICT_ENTRY_SIG_END_CHAR: char = '}';
pub const DICT_ENTRY_ALIGNMENT: usize = 8;
pub const VARIANT_SIGNATURE_CHAR: char = 'v';
pub const VARIANT_SIGNATURE_STR: &str = "v";
pub const VARIANT_ALIGNMENT: usize = 1;

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

pub(crate) fn alignment_for_signature_char(signature_char: char, _format: EncodingFormat) -> usize {
    match signature_char {
        u8::SIGNATURE_CHAR => u8::ALIGNMENT,
        bool::SIGNATURE_CHAR => bool::ALIGNMENT,
        i16::SIGNATURE_CHAR => i16::ALIGNMENT,
        u16::SIGNATURE_CHAR => u16::ALIGNMENT,
        i32::SIGNATURE_CHAR => i32::ALIGNMENT,
        u32::SIGNATURE_CHAR => u32::ALIGNMENT,
        i64::SIGNATURE_CHAR => i64::ALIGNMENT,
        u64::SIGNATURE_CHAR => u64::ALIGNMENT,
        f64::SIGNATURE_CHAR => f64::ALIGNMENT,
        <&str>::SIGNATURE_CHAR => <&str>::ALIGNMENT,
        ObjectPath::SIGNATURE_CHAR => ObjectPath::ALIGNMENT,
        Signature::SIGNATURE_CHAR => Signature::ALIGNMENT,
        VARIANT_SIGNATURE_CHAR => VARIANT_ALIGNMENT,
        ARRAY_SIGNATURE_CHAR => ARRAY_ALIGNMENT,
        STRUCT_SIG_START_CHAR => STRUCT_ALIGNMENT,
        DICT_ENTRY_SIG_START_CHAR => DICT_ENTRY_ALIGNMENT,
        _ => {
            println!("WARNING: Unsupported signature: {}", signature_char);

            0
        }
    }
}

pub(crate) fn slice_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    match signature.chars().next().ok_or(Error::InsufficientData)? {
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
        | VARIANT_SIGNATURE_CHAR => Ok(Signature::from(&signature[0..1])),
        ARRAY_SIGNATURE_CHAR => slice_array_signature(signature),
        STRUCT_SIG_START_CHAR => slice_structure_signature(signature),
        DICT_ENTRY_SIG_START_CHAR => slice_dict_entry_signature(signature),
        _ => Err(Error::UnsupportedType(signature.to_string())),
    }
}

fn slice_array_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    if signature.len() < 2 {
        return Err(Error::InsufficientData);
    }
    if !signature.starts_with(ARRAY_SIGNATURE_CHAR) {
        return Err(Error::IncorrectType);
    }

    // There should be a valid complete signature after 'a' but not more than 1
    let slice_len = slice_signature(&Signature::from(&signature[1..]))?.len();

    Ok(Signature::from(&signature[0..=slice_len]))
}

fn slice_structure_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    if !signature.starts_with(STRUCT_SIG_START_CHAR) {
        return Err(Error::IncorrectType);
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
    if &signature[i..=i] != STRUCT_SIG_END_STR {
        return Err(Error::IncorrectType);
    }

    Ok(Signature::from(&signature[0..=i]))
}

fn slice_dict_entry_signature<'a>(signature: &'a Signature<'a>) -> Result<Signature<'a>, Error> {
    if !signature.starts_with(DICT_ENTRY_SIG_START_CHAR) {
        return Err(Error::IncorrectType);
    }
    if signature.len() < 4 {
        return Err(Error::InsufficientData);
    }

    // Key's signature will always be just 1 character so no need to slice for that.
    // There should be one valid complete signature for value.
    let slice_len = slice_signature(&Signature::from(&signature[2..]))?.len();

    // signature of value + `{` + 1 char of the key signature + `}`
    Ok((&signature[0..slice_len + 3]).into())
}
