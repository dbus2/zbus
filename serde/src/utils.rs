use crate::{Basic, EncodingFormat, ObjectPath, Signature};

pub(crate) const ARRAY_SIGNATURE_CHAR: char = 'a';
pub(crate) const ARRAY_ALIGNMENT: usize = 4;
pub(crate) const STRUCT_SIG_START_CHAR: char = '(';
pub(crate) const STRUCT_SIG_END_CHAR: char = ')';
pub(crate) const STRUCT_SIG_START_STR: &'static str = "(";
pub(crate) const STRUCT_SIG_END_STR: &'static str = ")";
pub(crate) const STRUCT_ALIGNMENT: usize = 8;
pub(crate) const DICT_ENTRY_SIG_START_CHAR: char = '{';
pub(crate) const DICT_ENTRY_SIG_END_CHAR: char = '}';
pub(crate) const DICT_ENTRY_ALIGNMENT: usize = 8;
pub(crate) const VARIANT_SIGNATURE_CHAR: char = 'v';
pub(crate) const VARIANT_SIGNATURE_STR: &'static str = "v";
pub(crate) const VARIANT_ALIGNMENT: usize = 1;

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

pub(crate) fn padding_for_signature_char(
    n_bytes_before: usize,
    signature_char: char,
    _format: EncodingFormat,
) -> usize {
    match signature_char {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, u8::ALIGNMENT),
        bool::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, bool::ALIGNMENT),
        i16::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, i16::ALIGNMENT),
        u16::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, u16::ALIGNMENT),
        i32::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, i32::ALIGNMENT),
        u32::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, u32::ALIGNMENT),
        i64::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, i64::ALIGNMENT),
        u64::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, u64::ALIGNMENT),
        f64::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, f64::ALIGNMENT),
        <&str>::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, <&str>::ALIGNMENT),
        ObjectPath::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, ObjectPath::ALIGNMENT),
        Signature::SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, Signature::ALIGNMENT),
        VARIANT_SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, VARIANT_ALIGNMENT),
        ARRAY_SIGNATURE_CHAR => padding_for_n_bytes(n_bytes_before, ARRAY_ALIGNMENT),
        STRUCT_SIG_START_CHAR => padding_for_n_bytes(n_bytes_before, STRUCT_ALIGNMENT),
        DICT_ENTRY_SIG_START_CHAR => padding_for_n_bytes(n_bytes_before, DICT_ENTRY_ALIGNMENT),
        _ => {
            println!("WARNING: Unsupported signature: {}", signature_char);

            0
        }
    }
}
