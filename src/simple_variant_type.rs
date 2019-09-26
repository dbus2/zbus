use crate::{VariantError, VariantType};

pub trait SimpleVariantType<'a>: VariantType<'a> {
    fn slice_data_simple<'b>(
        data: &'b [u8],
        n_bytes_before: usize,
    ) -> Result<&'b [u8], VariantError>
    where
        Self: Sized,
    {
        Self::slice_data(data, Self::signature_str(), n_bytes_before)
    }

    fn decode_simple(bytes: &'a [u8], n_bytes_before: usize) -> Result<Self, VariantError>
    where
        Self: Sized,
    {
        Self::decode(bytes, Self::signature_str(), n_bytes_before)
    }
}

impl<'a> SimpleVariantType<'a> for u8 {}
impl<'a> SimpleVariantType<'a> for bool {}
impl<'a> SimpleVariantType<'a> for i16 {}
impl<'a> SimpleVariantType<'a> for u16 {}
impl<'a> SimpleVariantType<'a> for i32 {}
impl<'a> SimpleVariantType<'a> for u32 {}
impl<'a> SimpleVariantType<'a> for i64 {}
impl<'a> SimpleVariantType<'a> for u64 {}
impl<'a> SimpleVariantType<'a> for f64 {}
