use crate::{EncodingFormat, SharedData, VariantError, VariantType};

pub trait SimpleVariantType: VariantType {
    fn slice_data_simple(
        data: &SharedData,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError>
    where
        Self: Sized,
    {
        Self::slice_data(data, Self::signature_str(), format)
    }

    fn decode_simple(bytes: &SharedData, format: EncodingFormat) -> Result<Self, VariantError>
    where
        Self: Sized,
    {
        Self::decode(bytes, Self::signature_str(), format)
    }
}

impl SimpleVariantType for u8 {}
impl SimpleVariantType for bool {}
impl SimpleVariantType for i16 {}
impl SimpleVariantType for u16 {}
impl SimpleVariantType for i32 {}
impl SimpleVariantType for u32 {}
impl SimpleVariantType for i64 {}
impl SimpleVariantType for u64 {}
impl SimpleVariantType for f64 {}
