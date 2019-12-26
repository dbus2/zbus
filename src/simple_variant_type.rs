use crate::{Decode, EncodingFormat, SharedData, VariantError};

pub trait SimpleDecode: Decode {
    fn slice_data_simple(
        data: &SharedData,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError>
    where
        Self: Sized,
    {
        Self::slice_data(data, Self::SIGNATURE_STR, format)
    }

    fn decode_simple(bytes: &SharedData, format: EncodingFormat) -> Result<Self, VariantError>
    where
        Self: Sized,
    {
        Self::decode(bytes, Self::SIGNATURE_STR, format)
    }
}

impl SimpleDecode for u8 {}
impl SimpleDecode for bool {}
impl SimpleDecode for i16 {}
impl SimpleDecode for u16 {}
impl SimpleDecode for i32 {}
impl SimpleDecode for u32 {}
impl SimpleDecode for i64 {}
impl SimpleDecode for u64 {}
impl SimpleDecode for f64 {}
