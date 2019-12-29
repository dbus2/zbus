use byteorder::ByteOrder;

use crate::SharedData;
use crate::{Array, DictEntry, Encode, EncodingFormat};
use crate::{ObjectPath, Signature, SimpleDecode, Structure};
use crate::{Variant, VariantError};

/// Trait for decoding of varius types from encoded form.
///
/// All data types that implement [`Encode`] also implement `Decode`. As decoding require
/// allocation, one exception here is `&str`. It only implements [`Encode`], while its owned
/// sibling, `String` implements both traits.
///
/// [`Encode`]: trait.Encode.html
pub trait Decode: Encode + std::fmt::Debug {
    /// Get the slice of `data` that belongs to implementing type.
    ///
    /// Unless `data` only contains one item of implementing type, you need to call this function
    /// to get the slice before you can call [`decode`]. For simple types, you might want to use
    /// [`SimpleDecode::slice_data_simple`] instead.
    ///
    /// The default implementation works for constant-sized types whose size is the same as their
    /// alignment
    ///
    /// [`decode`]: trait.Decode.html#tymethod.decode
    /// [`SimpleDecode::slice_data_simple`]: trait.SimpleDecode.html#method.slice_data_simple
    fn slice_data(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError>
    where
        Self: Sized,
    {
        Self::ensure_correct_signature(signature)?;
        let data = data.into();
        let padding = Self::padding(data.position(), format);
        let len = Self::ALIGNMENT + padding;
        ensure_sufficient_bytes(data.bytes(), len)?;

        Ok(data.subset(0, len))
    }

    /// Ensure given `signature` is correct and of the implementing type.
    ///
    /// The default implementation works for simple types whose signature is constant.
    fn ensure_correct_signature(
        signature: impl Into<Signature>,
    ) -> Result<Signature, VariantError> {
        let signature = signature.into();

        if signature != Self::SIGNATURE_STR {
            return Err(VariantError::IncorrectType);
        }

        Ok(signature)
    }

    /// Decode instance of implementing type from the `data` slice.
    ///
    /// Unless `data` only contains one item of implementing type, you need to call [`slice_data`]
    /// to get the slice before you can call this function. For simple types, you might want to use
    /// [`SimpleDecode::decode_simple`] instead.
    ///
    /// [`slice_data`]: trait.Decode.html#method.slice_data
    /// [`SimpleDecode::decode_simple`]: trait.SimpleDecode.html#method.decode_simple
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError>
    where
        Self: Sized;

    /// Get the slice of `signature` that belongs to implementing type.
    ///
    /// This is used for getting individual signatures from container signatures. The default implementation works for
    /// simple types with single-character signatures.
    ///
    /// # Example:
    ///
    /// ```
    /// use zvariant::{Decode, Encode};
    ///
    /// let container_sig = "sux";
    /// let str_sig = String::slice_signature(container_sig).unwrap();
    /// assert!(str_sig == String::SIGNATURE_STR);
    /// let mut parsed = str_sig.len();
    ///
    /// let u32_sig = u32::slice_signature(&container_sig[parsed..]).unwrap();
    /// assert!(u32_sig == u32::SIGNATURE_STR);
    /// parsed += u32_sig.len();
    ///
    /// let i64_sig = i64::slice_signature(&container_sig[parsed..]).unwrap();
    /// assert!(i64_sig == i64::SIGNATURE_STR);
    /// ```
    fn slice_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError> {
        let slice: Signature = signature.into()[0..1].into();

        Self::ensure_correct_signature(slice)
    }

    /// A helper for [`decode`] implementation. Removes any leading padding bytes.
    ///
    /// [`decode`]: trait.Decode.html#tymethod.decode
    fn slice_for_decoding(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<SharedData, VariantError> {
        Self::ensure_correct_signature(signature)?;
        let data = data.into();
        let padding = Self::padding(data.position(), format);
        let len = Self::ALIGNMENT + padding;
        ensure_sufficient_bytes(data.bytes(), len)?;

        Ok(data.tail(padding))
    }

    /// Flatten the `variant` to enclosed value of the implementing type.
    ///
    /// `TryFrom<Variant>` bound on `Decode` trait would have been better but we can't do that
    /// unfortunately since [`Variant`] implements `Decode`.
    ///
    /// [`Variant`]: enum.Variant.html
    fn take_from_variant(variant: Variant) -> Result<Self, VariantError>
    where
        Self: Sized;

    /// Flatten the `variant` reference to a reference of the enclosed value of the implementing
    /// type.
    fn from_variant(variant: &Variant) -> Result<&Self, VariantError>;
}

impl Decode for u8 {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(slice.bytes()[0])
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::U8(u) = variant {
            Ok(u)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::U8(ref u) = variant {
            Ok(u)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Decode for bool {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        match byteorder::NativeEndian::read_u32(slice.bytes()) {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(VariantError::IncorrectValue),
        }
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::Bool(u) = variant {
            Ok(u)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::Bool(u) = variant {
            Ok(u)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Decode for i16 {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_i16(slice.bytes()))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::I16(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::I16(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Decode for u16 {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_u16(slice.bytes()))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::U16(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::U16(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Decode for i32 {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_i32(slice.bytes()))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::I32(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::I32(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Decode for u32 {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_u32(slice.bytes()))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::U32(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::U32(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Decode for i64 {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_i64(slice.bytes()))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::I64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::I64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Decode for u64 {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_u64(slice.bytes()))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::U64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::U64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

impl Decode for f64 {
    fn decode(
        data: impl Into<SharedData>,
        signature: impl Into<Signature>,
        format: EncodingFormat,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, format)?;

        Ok(byteorder::NativeEndian::read_f64(slice.bytes()))
    }

    fn take_from_variant(variant: Variant) -> Result<Self, VariantError> {
        if let Variant::F64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError> {
        if let Variant::F64(value) = variant {
            Ok(value)
        } else {
            Err(VariantError::IncorrectType)
        }
    }
}

pub(crate) fn ensure_sufficient_bytes(bytes: &[u8], size: usize) -> Result<(), VariantError> {
    if bytes.len() < size {
        return Err(VariantError::InsufficientData);
    }

    Ok(())
}

pub(crate) fn slice_data(
    data: impl Into<SharedData>,
    signature: impl Into<Signature>,
    format: EncodingFormat,
) -> Result<SharedData, VariantError> {
    let signature = signature.into();

    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => u8::slice_data_simple(data, format),
        bool::SIGNATURE_CHAR => bool::slice_data_simple(data, format),
        i16::SIGNATURE_CHAR => i16::slice_data_simple(data, format),
        u16::SIGNATURE_CHAR => u16::slice_data_simple(data, format),
        i32::SIGNATURE_CHAR => i32::slice_data_simple(data, format),
        u32::SIGNATURE_CHAR => u32::slice_data_simple(data, format),
        i64::SIGNATURE_CHAR => i64::slice_data_simple(data, format),
        u64::SIGNATURE_CHAR => u64::slice_data_simple(data, format),
        f64::SIGNATURE_CHAR => f64::slice_data_simple(data, format),
        String::SIGNATURE_CHAR => String::slice_data_simple(data, format),
        Array::SIGNATURE_CHAR => Array::slice_data(data, signature, format),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::slice_data_simple(data, format),
        Signature::SIGNATURE_CHAR => Signature::slice_data_simple(data, format),
        Structure::SIGNATURE_CHAR => Structure::slice_data(data, signature, format),
        Variant::SIGNATURE_CHAR => Variant::slice_data(data, signature, format),
        DictEntry::SIGNATURE_CHAR => DictEntry::slice_data(data, signature, format),
        _ => Err(VariantError::UnsupportedType(signature)),
    }
}

pub(crate) fn slice_signature(signature: impl Into<Signature>) -> Result<Signature, VariantError> {
    let signature = signature.into();

    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => u8::slice_signature(signature),
        bool::SIGNATURE_CHAR => bool::slice_signature(signature),
        i16::SIGNATURE_CHAR => i16::slice_signature(signature),
        u16::SIGNATURE_CHAR => u16::slice_signature(signature),
        i32::SIGNATURE_CHAR => i32::slice_signature(signature),
        u32::SIGNATURE_CHAR => u32::slice_signature(signature),
        i64::SIGNATURE_CHAR => i64::slice_signature(signature),
        u64::SIGNATURE_CHAR => u64::slice_signature(signature),
        f64::SIGNATURE_CHAR => f64::slice_signature(signature),
        String::SIGNATURE_CHAR => String::slice_signature(signature),
        Array::SIGNATURE_CHAR => Array::slice_signature(signature),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::slice_signature(signature),
        Signature::SIGNATURE_CHAR => Signature::slice_signature(signature),
        Structure::SIGNATURE_CHAR => Structure::slice_signature(signature),
        Variant::SIGNATURE_CHAR => Variant::slice_signature(signature),
        DictEntry::SIGNATURE_CHAR => DictEntry::slice_signature(signature),
        _ => Err(VariantError::UnsupportedType(signature)),
    }
}
