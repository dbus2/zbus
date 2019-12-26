use byteorder::ByteOrder;
use std::str;

use crate::utils::padding_for_n_bytes;
use crate::{Array, DictEntry, ObjectPath, Signature, Structure};
use crate::{Variant, VariantTypeConstants};

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
    fn signature_char() -> char
    where
        Self: Sized;
    fn signature_str() -> &'static str
    where
        Self: Sized;
    fn alignment() -> usize
    where
        Self: Sized;

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
        Signature::new(Self::signature_str())
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
        padding_for_n_bytes(n_bytes_before, Self::alignment())
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
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U8(self)
    }
}

impl Encode for bool {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&(*self as u32).to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::Bool(self)
    }
}

impl Encode for i16 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I16(self)
    }
}

impl Encode for u16 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U16(self)
    }
}

impl Encode for i32 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I32(self)
    }
}

impl Encode for u32 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U32(self)
    }
}

impl Encode for i64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::I64(self)
    }
}

impl Encode for u64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, format: EncodingFormat) {
        Self::add_padding(bytes, format);
        bytes.extend(&self.to_ne_bytes());
    }

    fn to_variant(self) -> Variant {
        Variant::U64(self)
    }
}

impl Encode for f64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

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

#[cfg(test)]
mod tests {
    use crate::{Encode, EncodingFormat, SharedData, SimpleDecode};

    // Ensure Encode can be used as Boxed type
    #[test]
    fn trait_object() {
        let boxed = Box::new(42u8);

        let format = EncodingFormat::default();
        let encoded = SharedData::new(encode_u8(boxed, format));
        assert!(u8::decode_simple(&encoded, format).unwrap() == 42u8);
    }

    fn encode_u8(boxed: Box<dyn Encode>, format: EncodingFormat) -> Vec<u8> {
        boxed.encode(format)
    }
}
