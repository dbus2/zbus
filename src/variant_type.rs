use byteorder::ByteOrder;
use std::{borrow::Cow, error, fmt, str};

use crate::utils::padding_for_n_bytes;
use crate::SharedData;
use crate::{Array, DictEntry, ObjectPath, Signature, Structure};
use crate::{SimpleVariantType, Variant, VariantTypeConstants};

#[derive(Debug)]
pub enum VariantError {
    ExcessData,
    IncorrectType,
    IncorrectValue,
    InvalidUtf8,
    InsufficientData,
    UnsupportedType(String),
}

impl error::Error for VariantError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VariantError::ExcessData => write!(f, "excess data"),
            VariantError::IncorrectType => write!(f, "incorrect type"),
            VariantError::IncorrectValue => write!(f, "incorrect value"),
            VariantError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            VariantError::InsufficientData => write!(f, "insufficient data"),
            VariantError::UnsupportedType(s) => {
                write!(f, "unsupported type (signature: \"{}\")", s)
            }
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum AlignmentKind {
    All,
    ChildOnly,
}

impl Default for AlignmentKind {
    fn default() -> Self {
        AlignmentKind::All
    }
}

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

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct EncodingContext {
    pub format: EncodingFormat,
    pub alignment: AlignmentKind,
}

impl EncodingContext {
    pub fn copy_for_child(self) -> Self {
        let mut copy = self;
        if copy.alignment == AlignmentKind::ChildOnly {
            copy.alignment = AlignmentKind::All;
        }

        copy
    }
}

// As trait-object, you can only use the `encode` method but you can downcast it to the concrete
// type to get the full API back.
pub trait VariantType: std::fmt::Debug {
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
    fn encode(&self, context: EncodingContext) -> Vec<u8> {
        let mut bytes = vec![];
        self.encode_into(&mut bytes, context);

        bytes
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext);

    // Default implementation works for constant-sized types where size is the same as their
    // alignment
    fn slice_data(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<SharedData, VariantError>
    where
        Self: Sized,
    {
        Self::ensure_correct_signature(signature)?;
        let padding = Self::padding(data.position(), context);
        let len = Self::alignment() + padding;
        data.apply(|bytes| ensure_sufficient_bytes(bytes, len))?;

        Ok(data.subset(0, len))
    }

    fn ensure_correct_signature(signature: &str) -> Result<(), VariantError>
    where
        Self: Sized,
    {
        if signature != Self::signature_str() {
            return Err(VariantError::IncorrectType);
        }

        Ok(())
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError>
    where
        Self: Sized;

    fn signature<'a>(&'a self) -> Cow<'a, str>
    where
        Self: Sized,
    {
        Cow::from(Self::signature_str())
    }

    fn slice_signature(signature: &str) -> Result<&str, VariantError>
    where
        Self: Sized,
    {
        let slice = &signature[0..1];
        Self::ensure_correct_signature(slice)?;

        Ok(slice)
    }

    fn add_padding(bytes: &mut Vec<u8>, context: EncodingContext)
    where
        Self: Sized,
    {
        let padding = Self::padding(bytes.len(), context);
        if padding > 0 {
            bytes.resize(bytes.len() + padding, 0);
        }
    }

    // Mostly a helper for decode() implementation. Removes any leading padding bytes.
    fn slice_for_decoding(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<SharedData, VariantError>
    where
        Self: Sized,
    {
        Self::ensure_correct_signature(signature)?;
        let padding = Self::padding(data.position(), context);
        let len = Self::alignment() + padding;
        data.apply(|bytes| ensure_sufficient_bytes(bytes, len))?;

        Ok(data.tail(padding))
    }

    fn padding(n_bytes_before: usize, context: EncodingContext) -> usize
    where
        Self: Sized,
    {
        match context.alignment {
            AlignmentKind::All => padding_for_n_bytes(n_bytes_before, Self::alignment()),
            AlignmentKind::ChildOnly => 0,
        }
    }

    /// Checks if variant value is of the generic type `T`
    ///
    /// # Examples
    ///
    /// ```
    /// use zbus::VariantType;
    ///
    /// let v = String::from("hello").to_variant();
    /// assert!(!u32::is(&v));
    /// assert!(String::is(&v));
    /// ```
    ///
    /// ```
    /// use zbus::VariantType;
    ///
    /// let v = 147u32.to_variant();
    /// assert!(u32::is(&v));
    /// assert!(!String::is(&v));
    /// ```
    fn is(variant: &Variant) -> bool
    where
        Self: Sized;

    // `TryFrom<Variant>` trait bound would have been better but we can't use that unfortunately
    // since Variant implements VariantType.
    fn take_from_variant(variant: Variant) -> Result<Self, VariantError>
    where
        Self: Sized;

    fn from_variant(variant: &Variant) -> Result<&Self, VariantError>
    where
        Self: Sized;

    // Into<Variant> trait bound would have been better and it's possible but since `Into<T> for T`
    // is provided implicitly, the default no-op implementation for `Variant` won't do the right
    // thing: unflatten it.
    // `TryFrom<Variant>`.
    fn to_variant(self) -> Variant
    where
        Self: Sized;
}

impl VariantType for u8 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        bytes.extend(&self.to_ne_bytes());
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| Ok(bytes[0]))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::U8(_) = variant {
            true
        } else {
            false
        }
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

    fn to_variant(self) -> Variant {
        Variant::U8(self)
    }
}

impl VariantType for bool {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        bytes.extend(&(*self as u32).to_ne_bytes());
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| match byteorder::NativeEndian::read_u32(bytes) {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(VariantError::IncorrectValue),
        })
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::Bool(_) = variant {
            true
        } else {
            false
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

    fn to_variant(self) -> Variant {
        Variant::Bool(self)
    }
}

impl VariantType for i16 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        bytes.extend(&self.to_ne_bytes());
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| Ok(byteorder::NativeEndian::read_i16(bytes)))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::I16(_) = variant {
            true
        } else {
            false
        }
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

    fn to_variant(self) -> Variant {
        Variant::I16(self)
    }
}

impl VariantType for u16 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        bytes.extend(&self.to_ne_bytes());
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| Ok(byteorder::NativeEndian::read_u16(bytes)))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::U16(_) = variant {
            true
        } else {
            false
        }
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

    fn to_variant(self) -> Variant {
        Variant::U16(self)
    }
}

impl VariantType for i32 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        bytes.extend(&self.to_ne_bytes());
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| Ok(byteorder::NativeEndian::read_i32(bytes)))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::I32(_) = variant {
            true
        } else {
            false
        }
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

    fn to_variant(self) -> Variant {
        Variant::I32(self)
    }
}

impl VariantType for u32 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        bytes.extend(&self.to_ne_bytes());
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| Ok(byteorder::NativeEndian::read_u32(bytes)))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::U32(_) = variant {
            true
        } else {
            false
        }
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

    fn to_variant(self) -> Variant {
        Variant::U32(self)
    }
}

impl VariantType for i64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        bytes.extend(&self.to_ne_bytes());
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| Ok(byteorder::NativeEndian::read_i64(bytes)))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::I64(_) = variant {
            true
        } else {
            false
        }
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

    fn to_variant(self) -> Variant {
        Variant::I64(self)
    }
}

impl VariantType for u64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        bytes.extend(&self.to_ne_bytes());
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| Ok(byteorder::NativeEndian::read_u64(bytes)))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::U64(_) = variant {
            true
        } else {
            false
        }
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

    fn to_variant(self) -> Variant {
        Variant::U64(self)
    }
}

impl VariantType for f64 {
    fn signature_char() -> char {
        Self::SIGNATURE_CHAR
    }
    fn signature_str() -> &'static str {
        Self::SIGNATURE_STR
    }
    fn alignment() -> usize {
        Self::ALIGNMENT
    }

    fn encode_into(&self, bytes: &mut Vec<u8>, context: EncodingContext) {
        Self::add_padding(bytes, context);
        let mut buf = [0; 8];
        byteorder::NativeEndian::write_f64(&mut buf, *self);
        bytes.extend_from_slice(&buf);
    }

    fn decode(
        data: &SharedData,
        signature: &str,
        context: EncodingContext,
    ) -> Result<Self, VariantError> {
        let slice = Self::slice_for_decoding(data, signature, context)?;

        slice.apply(|bytes| Ok(byteorder::NativeEndian::read_f64(bytes)))
    }

    fn is(variant: &Variant) -> bool {
        if let Variant::F64(_) = variant {
            true
        } else {
            false
        }
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

    fn to_variant(self) -> Variant {
        Variant::F64(self)
    }
}

pub(crate) fn ensure_sufficient_bytes(bytes: &[u8], size: usize) -> Result<(), VariantError> {
    if bytes.len() < size {
        return Err(VariantError::InsufficientData);
    }

    Ok(())
}

pub(crate) fn slice_data(
    data: &SharedData,
    signature: &str,
    context: EncodingContext,
) -> Result<SharedData, VariantError> {
    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => u8::slice_data_simple(data, context),
        bool::SIGNATURE_CHAR => bool::slice_data_simple(data, context),
        i16::SIGNATURE_CHAR => i16::slice_data_simple(data, context),
        u16::SIGNATURE_CHAR => u16::slice_data_simple(data, context),
        i32::SIGNATURE_CHAR => i32::slice_data_simple(data, context),
        u32::SIGNATURE_CHAR => u32::slice_data_simple(data, context),
        i64::SIGNATURE_CHAR => i64::slice_data_simple(data, context),
        u64::SIGNATURE_CHAR => u64::slice_data_simple(data, context),
        f64::SIGNATURE_CHAR => f64::slice_data_simple(data, context),
        String::SIGNATURE_CHAR => String::slice_data_simple(data, context),
        Array::SIGNATURE_CHAR => Array::slice_data(data, signature, context),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::slice_data_simple(data, context),
        Signature::SIGNATURE_CHAR => Signature::slice_data_simple(data, context),
        Structure::SIGNATURE_CHAR => Structure::slice_data(data, signature, context),
        Variant::SIGNATURE_CHAR => Variant::slice_data(data, signature, context),
        DictEntry::SIGNATURE_CHAR => DictEntry::slice_data(data, signature, context),
        _ => return Err(VariantError::UnsupportedType(String::from(signature))),
    }
}

pub(crate) fn padding_for_signature(
    n_bytes_before: usize,
    signature: &str,
    context: EncodingContext,
) -> usize {
    match signature.chars().next().unwrap_or('\0') {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => u8::padding(n_bytes_before, context),
        bool::SIGNATURE_CHAR => bool::padding(n_bytes_before, context),
        i16::SIGNATURE_CHAR => i16::padding(n_bytes_before, context),
        u16::SIGNATURE_CHAR => u16::padding(n_bytes_before, context),
        i32::SIGNATURE_CHAR => i32::padding(n_bytes_before, context),
        u32::SIGNATURE_CHAR => u32::padding(n_bytes_before, context),
        i64::SIGNATURE_CHAR => i64::padding(n_bytes_before, context),
        u64::SIGNATURE_CHAR => u64::padding(n_bytes_before, context),
        f64::SIGNATURE_CHAR => f64::padding(n_bytes_before, context),
        String::SIGNATURE_CHAR => String::padding(n_bytes_before, context),
        Array::SIGNATURE_CHAR => Array::padding(n_bytes_before, context),
        ObjectPath::SIGNATURE_CHAR => ObjectPath::padding(n_bytes_before, context),
        Signature::SIGNATURE_CHAR => Signature::padding(n_bytes_before, context),
        Structure::SIGNATURE_CHAR => Structure::padding(n_bytes_before, context),
        Variant::SIGNATURE_CHAR => Variant::padding(n_bytes_before, context),
        DictEntry::SIGNATURE_CHAR => DictEntry::padding(n_bytes_before, context),
        _ => {
            println!("WARNING: Unsupported signature: {}", signature);

            0
        }
    }
}

pub(crate) fn slice_signature(signature: &str) -> Result<&str, VariantError> {
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
        _ => return Err(VariantError::UnsupportedType(String::from(signature))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{EncodingContext, SharedData, SimpleVariantType, VariantType};

    // Ensure VariantType can be used as Boxed type
    #[test]
    fn trait_object() {
        let boxed = Box::new(42u8);

        let context = EncodingContext::default();
        let encoded = SharedData::new(encode_u8(boxed, context));
        assert!(u8::decode_simple(&encoded, context).unwrap() == 42u8);
    }

    fn encode_u8(boxed: Box<dyn VariantType>, context: EncodingContext) -> Vec<u8> {
        boxed.encode(context)
    }
}
