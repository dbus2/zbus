use byteorder::ByteOrder;
use std::str;

use crate::utils::padding_for_n_bytes;
use crate::{Array, DictEntry, ObjectPath};
use crate::{Signature, Structure, Variant, VariantError};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EncodingFormat {
    DBus,
    // TODO: GVariant
}

impl Default for EncodingFormat {
    fn default() -> Self {
        EncodingFormat::DBus
    }
}

pub trait Encode: std::fmt::Debug {
    const SIGNATURE_CHAR: char;
    const SIGNATURE_STR: &'static str;
    const ALIGNMENT: usize;

    // Only use for the first data in a message
    fn encode(&self, format: EncodingFormat) -> Vec<u8> {
        let mut bytes = vec![];
        self.encode_into(&mut bytes, format);

        bytes
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat);

    fn signature(&self) -> Signature
    where
        Self: Sized,
    {
        Signature::new(Self::SIGNATURE_STR)
    }

    fn add_padding(bytes: &mut Vec<u8>, format: EncodingFormat)
    where
        Self: Sized,
    {
        let padding = Self::padding(bytes.len(), format);
        if padding > 0 {
            bytes.resize(bytes.len() + padding, 0);
        }
    }

    fn padding(n_bytes_before: usize, _format: EncodingFormat) -> usize
    where
        Self: Sized,
    {
        padding_for_n_bytes(n_bytes_before, Self::ALIGNMENT)
    }

    // Into<Variant> trait bound would have been better and it's possible but since `Into<T> for T`
    // is provided implicitly, the default no-op implementation for `Variant` won't do the right
    // thing: unflatten it.
    // `TryFrom<Variant>`.
    fn to_variant(self) -> Variant
    where
        Self: Sized;
}

impl Encode for u8 {
    const SIGNATURE_CHAR: char = 'y';
    const SIGNATURE_STR: &'static str = "y";
    const ALIGNMENT: usize = 1;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U8(self)
    }
}

impl Encode for bool {
    const SIGNATURE_CHAR: char = 'b';
    const SIGNATURE_STR: &'static str = "b";
    const ALIGNMENT: usize = 4;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&(*self as u32).to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::Bool(self)
    }
}

impl Encode for i16 {
    const SIGNATURE_CHAR: char = 'n';
    const SIGNATURE_STR: &'static str = "n";
    const ALIGNMENT: usize = 2;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I16(self)
    }
}

impl Encode for u16 {
    const SIGNATURE_CHAR: char = 'q';
    const SIGNATURE_STR: &'static str = "q";
    const ALIGNMENT: usize = 2;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U16(self)
    }
}

impl Encode for i32 {
    const SIGNATURE_CHAR: char = 'i';
    const SIGNATURE_STR: &'static str = "i";
    const ALIGNMENT: usize = 4;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I32(self)
    }
}

impl Encode for u32 {
    const SIGNATURE_CHAR: char = 'u';
    const SIGNATURE_STR: &'static str = "u";
    const ALIGNMENT: usize = 4;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U32(self)
    }
}

impl Encode for i64 {
    const SIGNATURE_CHAR: char = 'x';
    const SIGNATURE_STR: &'static str = "x";
    const ALIGNMENT: usize = 8;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I64(self)
    }
}

impl Encode for u64 {
    const SIGNATURE_CHAR: char = 't';
    const SIGNATURE_STR: &'static str = "t";
    const ALIGNMENT: usize = 8;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U64(self)
    }
}

impl Encode for f64 {
    const SIGNATURE_CHAR: char = 'd';
    const SIGNATURE_STR: &'static str = "d";
    const ALIGNMENT: usize = 8;

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        let mut buf = [0; 8];
        byteorder::NativeEndian::write_f64(&mut buf, *self);
        bytes.extend_from_slice(&buf);
    }

    fn to_variant(self) -> Variant {
        Variant::F64(self)
    }
}

pub(crate) fn padding_for_signature(
    n_bytes_before: usize,
    signature: impl Into<Signature>,
    format: EncodingFormat,
) -> usize {
    let signature = signature.into();

    match signature.chars().next().unwrap_or('\0') {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => u8::padding(n_bytes_before, format),
        bool::SIGNATURE_CHAR => bool::padding(n_bytes_before, format),
        i16::SIGNATURE_CHAR => i16::padding(n_bytes_before, format),
        u16::SIGNATURE_CHAR => u16::padding(n_bytes_before, format),
        i32::SIGNATURE_CHAR => i32::padding(n_bytes_before, format),
        u32::SIGNATURE_CHAR => u32::padding(n_bytes_before, format),
        i64::SIGNATURE_CHAR => i64::padding(n_bytes_before, format),
        u64::SIGNATURE_CHAR => u64::padding(n_bytes_before, format),
        f64::SIGNATURE_CHAR => f64::padding(n_bytes_before, format),
        String::SIGNATURE_CHAR => String::padding(n_bytes_before, format),
        Array::SIGNATURE_CHAR => Array::padding(n_bytes_before, format),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::padding(n_bytes_before, format),
        Signature::SIGNATURE_CHAR => Signature::padding(n_bytes_before, format),
        Structure::SIGNATURE_CHAR => Structure::padding(n_bytes_before, format),
        Variant::SIGNATURE_CHAR => Variant::padding(n_bytes_before, format),
        DictEntry::SIGNATURE_CHAR => DictEntry::padding(n_bytes_before, format),
        _ => {
            println!("WARNING: Unsupported signature: {}", signature.as_str());

            0
        }
    }
}

pub(crate) fn alignment_for_signature(
    signature: impl Into<Signature>,
) -> Result<usize, VariantError> {
    let signature = signature.into();

    match signature
        .as_str()
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => Ok(u8::ALIGNMENT),
        bool::SIGNATURE_CHAR => Ok(bool::ALIGNMENT),
        i16::SIGNATURE_CHAR => Ok(i16::ALIGNMENT),
        u16::SIGNATURE_CHAR => Ok(u16::ALIGNMENT),
        i32::SIGNATURE_CHAR => Ok(i32::ALIGNMENT),
        u32::SIGNATURE_CHAR => Ok(u32::ALIGNMENT),
        i64::SIGNATURE_CHAR => Ok(i64::ALIGNMENT),
        u64::SIGNATURE_CHAR => Ok(u64::ALIGNMENT),
        f64::SIGNATURE_CHAR => Ok(f64::ALIGNMENT),
        <String>::SIGNATURE_CHAR => Ok(<String>::ALIGNMENT),
        Array::SIGNATURE_CHAR => Ok(Array::ALIGNMENT),
        ObjectPath::SIGNATURE_CHAR => Ok(ObjectPath::ALIGNMENT),
        Signature::SIGNATURE_CHAR => Ok(Signature::ALIGNMENT),
        Structure::SIGNATURE_CHAR => Ok(Structure::ALIGNMENT),
        Variant::SIGNATURE_CHAR => Ok(Variant::ALIGNMENT),
        DictEntry::SIGNATURE_CHAR => Ok(DictEntry::ALIGNMENT),
        _ => return Err(VariantError::UnsupportedType(signature)),
    }
}
