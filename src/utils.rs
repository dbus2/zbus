pub(crate) fn padding_for_8_bytes(value: u32) -> u32 {
    padding_for_n_bytes(value, 8)
}

pub(crate) fn padding_for_n_bytes(value: u32, align: u32) -> u32 {
    let len_rounded_up = value.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);

    len_rounded_up.wrapping_sub(value)
}
