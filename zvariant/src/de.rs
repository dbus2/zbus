use core::convert::TryFrom;

use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;

use std::ffi::CStr;
use std::os::unix::io::RawFd;
use std::{marker::PhantomData, str};

use crate::framing_offset_size::FramingOffsetSize;
use crate::framing_offsets::FramingOffsets;
use crate::signature_parser::SignatureParser;
use crate::utils::*;
use crate::Type;
use crate::{Basic, EncodingContext, EncodingFormat};
use crate::{Error, Result};
use crate::{Fd, ObjectPath, Signature};

/// Deserialize `T` from a given slice of bytes, containing file descriptor indices.
///
/// Please note that actual file descriptors are not part of the encoding and need to be transferred
/// via an out-of-band platform specific mechanism. The encoding only contain the indices of the
/// file descriptors and hence the reason, caller must pass a slice of file descriptors.
///
/// # Examples
///
/// ```
/// use zvariant::{to_bytes_fds, from_slice_fds};
/// use zvariant::{EncodingContext, Fd};
///
/// let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
/// let (encoded, fds) = to_bytes_fds(ctxt, &Fd::from(42)).unwrap();
/// let decoded: Fd = from_slice_fds(&encoded, Some(&fds), ctxt).unwrap();
/// assert_eq!(decoded, Fd::from(42));
/// ```
///
/// [`from_slice`]: fn.from_slice.html
pub fn from_slice_fds<'d, 'r: 'd, B, T: ?Sized>(
    bytes: &'r [u8],
    fds: Option<&[RawFd]>,
    ctxt: EncodingContext<B>,
) -> Result<T>
where
    B: byteorder::ByteOrder,
    T: Deserialize<'d> + Type,
{
    let signature = T::signature();
    from_slice_fds_for_signature(bytes, fds, ctxt, &signature)
}

/// Deserialize `T` from a given slice of bytes.
///
/// If `T` is an, or (potentially) contains an [`Fd`], use [`from_slice_fds`] instead.
///
/// # Examples
///
/// ```
/// use zvariant::{to_bytes, from_slice};
/// use zvariant::EncodingContext;
///
/// let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
/// let encoded = to_bytes(ctxt, "hello world").unwrap();
/// let decoded: &str = from_slice(&encoded, ctxt).unwrap();
/// assert_eq!(decoded, "hello world");
/// ```
///
/// [`Fd`]: struct.Fd.html
/// [`from_slice_fds`]: fn.from_slice_fds.html
pub fn from_slice<'d, 'r: 'd, B, T: ?Sized>(bytes: &'r [u8], ctxt: EncodingContext<B>) -> Result<T>
where
    B: byteorder::ByteOrder,
    T: Deserialize<'d> + Type,
{
    let signature = T::signature();
    from_slice_for_signature(bytes, ctxt, &signature)
}

/// Deserialize `T` from a given slice of bytes with the given signature.
///
/// Use this function instead of [`from_slice`] if the value being deserialized does not implement
/// [`Type`]. Also, if `T` is an, or (potentially) contains an [`Fd`], use
/// [`from_slice_fds_for_signature`] instead.
///
/// # Examples
///
/// One known case where `Type` implementation isn't possible, is enum types (except simple ones
/// with unit variants only).
///
/// ```
/// use std::convert::TryInto;
/// use serde::{Deserialize, Serialize};
///
/// use zvariant::{to_bytes_for_signature, from_slice_for_signature};
/// use zvariant::EncodingContext;
///
/// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
/// enum Test {
///     Unit,
///     NewType(u8),
///     Tuple(u8, u64),
///     Struct { y: u8, t: u64 },
/// }
///
/// let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
/// let signature = "u".try_into().unwrap();
/// let encoded = to_bytes_for_signature(ctxt, &signature, &Test::Unit).unwrap();
/// let decoded: Test = from_slice_for_signature(&encoded, ctxt, &signature).unwrap();
/// assert_eq!(decoded, Test::Unit);
///
/// let signature = "y".try_into().unwrap();
/// let encoded = to_bytes_for_signature(ctxt, &signature, &Test::NewType(42)).unwrap();
/// let decoded: Test = from_slice_for_signature(&encoded, ctxt, &signature).unwrap();
/// assert_eq!(decoded, Test::NewType(42));
///
/// let signature = "(yt)".try_into().unwrap();
/// let encoded = to_bytes_for_signature(ctxt, &signature, &Test::Tuple(42, 42)).unwrap();
/// let decoded: Test = from_slice_for_signature(&encoded, ctxt, &signature).unwrap();
/// assert_eq!(decoded, Test::Tuple(42, 42));
///
/// let s = Test::Struct { y: 42, t: 42 };
/// let encoded = to_bytes_for_signature(ctxt, &signature, &s).unwrap();
/// let decoded: Test = from_slice_for_signature(&encoded, ctxt, &signature).unwrap();
/// assert_eq!(decoded, Test::Struct { y: 42, t: 42 });
/// ```
///
/// [`Type`]: trait.Type.html
/// [`Fd`]: struct.Fd.html
/// [`from_slice_fds_for_signature`]: fn.from_slice_fds_for_signature.html
// TODO: Return number of bytes parsed?
pub fn from_slice_for_signature<'d, 'r: 'd, B, T: ?Sized>(
    bytes: &'r [u8],
    ctxt: EncodingContext<B>,
    signature: &Signature,
) -> Result<T>
where
    B: byteorder::ByteOrder,
    T: Deserialize<'d>,
{
    from_slice_fds_for_signature(bytes, None, ctxt, signature)
}

/// Deserialize `T` from a given slice of bytes containing file descriptor indices, with the given signature.
///
/// Please note that actual file descriptors are not part of the encoding and need to be transferred
/// via an out-of-band platform specific mechanism. The encoding only contain the indices of the
/// file descriptors and hence the reason, caller must pass a slice of file descriptors.
///
/// [`from_slice`]: fn.from_slice.html
/// [`from_slice_for_signature`]: fn.from_slice_for_signature.html
// TODO: Return number of bytes parsed?
pub fn from_slice_fds_for_signature<'d, 'r: 'd, B, T: ?Sized>(
    bytes: &'r [u8],
    fds: Option<&[RawFd]>,
    ctxt: EncodingContext<B>,
    signature: &Signature,
) -> Result<T>
where
    B: byteorder::ByteOrder,
    T: Deserialize<'d>,
{
    let mut de = Deserializer::new(bytes, fds, signature, ctxt);

    T::deserialize(&mut de)
}

/// Our deserialization implementation.
#[derive(Debug)]
pub struct Deserializer<'de, 'sig, 'f, B> {
    pub(self) ctxt: EncodingContext<B>,
    pub(self) bytes: &'de [u8],
    pub(self) fds: Option<&'f [RawFd]>,
    pub(self) pos: usize,

    pub(self) sig_parser: SignatureParser<'sig>,

    b: PhantomData<B>,
}

impl<'de, 'sig, 'f, B> Deserializer<'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    /// Create a Deserializer struct instance.
    pub fn new<'r: 'de>(
        bytes: &'r [u8],
        fds: Option<&'f [RawFd]>,
        signature: &Signature<'sig>,
        ctxt: EncodingContext<B>,
    ) -> Self {
        let sig_parser = SignatureParser::new(signature.clone());

        Self {
            ctxt,
            sig_parser,
            bytes,
            fds,
            pos: 0,
            b: PhantomData,
        }
    }

    fn get_fd(&self, idx: u32) -> Result<i32> {
        self.fds
            .map(|fds| fds.get(idx as usize))
            .flatten()
            .copied()
            .ok_or(Error::UnknownFd)
    }

    fn parse_padding(&mut self, alignment: usize) -> Result<usize> {
        let padding = padding_for_n_bytes(self.abs_pos(), alignment);
        if padding > 0 {
            if self.pos + padding > self.bytes.len() {
                return Err(serde::de::Error::invalid_length(
                    self.bytes.len(),
                    &format!(">= {}", self.pos + padding).as_str(),
                ));
            }

            for i in 0..padding {
                let byte = self.bytes[self.pos + i];
                if byte != 0 {
                    return Err(Error::PaddingNot0(byte));
                }
            }
            self.pos += padding;
        }

        Ok(padding)
    }

    fn prep_deserialize_basic<T>(&mut self) -> Result<()>
    where
        T: Basic,
    {
        self.sig_parser.skip_char()?;
        self.parse_padding(T::alignment(self.ctxt.format()))?;

        Ok(())
    }

    fn next_slice(&mut self, len: usize) -> Result<&'de [u8]> {
        if self.pos + len > self.bytes.len() {
            return Err(serde::de::Error::invalid_length(
                self.bytes.len(),
                &format!(">= {}", self.pos + len).as_str(),
            ));
        }

        let slice = &self.bytes[self.pos..self.pos + len];
        self.pos += len;

        Ok(slice)
    }

    fn next_const_size_slice<T>(&mut self) -> Result<&[u8]>
    where
        T: Basic,
    {
        self.prep_deserialize_basic::<T>()?;

        self.next_slice(T::alignment(self.ctxt.format()))
    }

    fn abs_pos(&self) -> usize {
        self.ctxt.position() + self.pos
    }
}

impl<'de, 'd, 'sig, 'f, B> de::Deserializer<'de> for &'d mut Deserializer<'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.sig_parser.next_char() {
            u8::SIGNATURE_CHAR => self.deserialize_u8(visitor),
            bool::SIGNATURE_CHAR => self.deserialize_bool(visitor),
            i16::SIGNATURE_CHAR => self.deserialize_i16(visitor),
            u16::SIGNATURE_CHAR => self.deserialize_u16(visitor),
            i32::SIGNATURE_CHAR | Fd::SIGNATURE_CHAR => self.deserialize_i32(visitor),
            u32::SIGNATURE_CHAR => self.deserialize_u32(visitor),
            i64::SIGNATURE_CHAR => self.deserialize_i64(visitor),
            u64::SIGNATURE_CHAR => self.deserialize_u64(visitor),
            f64::SIGNATURE_CHAR => self.deserialize_f64(visitor),
            <&str>::SIGNATURE_CHAR | ObjectPath::SIGNATURE_CHAR | Signature::SIGNATURE_CHAR => {
                self.deserialize_str(visitor)
            }
            VARIANT_SIGNATURE_CHAR => self.deserialize_seq(visitor),
            ARRAY_SIGNATURE_CHAR => self.deserialize_seq(visitor),
            STRUCT_SIG_START_CHAR => self.deserialize_seq(visitor),
            c => Err(de::Error::invalid_value(
                de::Unexpected::Char(c),
                &"a valid signature character",
            )),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = B::read_u32(self.next_const_size_slice::<bool>()?);
        let b = match v {
            1 => true,
            0 => false,
            // As per D-Bus spec, only 0 and 1 values are allowed
            _ => {
                return Err(de::Error::invalid_value(
                    de::Unexpected::Unsigned(v as u64),
                    &"0 or 1",
                ))
            }
        };

        visitor.visit_bool(b)
    }

    // Use macros to avoid code duplication here
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i16(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = B::read_i16(self.next_const_size_slice::<i16>()?);

        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = match self.sig_parser.next_char() {
            Fd::SIGNATURE_CHAR => {
                self.sig_parser.skip_char()?;
                self.parse_padding(u32::alignment(self.ctxt.format()))?;
                let idx = B::read_u32(self.next_slice(u32::alignment(self.ctxt.format()))?);
                self.get_fd(idx)?
            }
            _ => B::read_i32(self.next_const_size_slice::<i32>()?),
        };

        visitor.visit_i32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = B::read_i64(self.next_const_size_slice::<i64>()?);

        visitor.visit_i64(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Endianness is irrelevant for single bytes.
        visitor.visit_u8(self.next_const_size_slice::<u8>().map(|bytes| bytes[0])?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = B::read_u16(self.next_const_size_slice::<u16>()?);

        visitor.visit_u16(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = B::read_u32(self.next_const_size_slice::<u32>()?);

        visitor.visit_u32(v)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = B::read_u64(self.next_const_size_slice::<u64>()?);

        visitor.visit_u64(v)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = B::read_f64(self.next_const_size_slice::<f64>()?);

        visitor.visit_f32(f64_to_f32(v))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = B::read_f64(self.next_const_size_slice::<f64>()?);

        visitor.visit_f64(v)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let s = match self.ctxt.format() {
            EncodingFormat::DBus => {
                let len = match self.sig_parser.next_char() {
                    Signature::SIGNATURE_CHAR | VARIANT_SIGNATURE_CHAR => {
                        let len_slice = self.next_slice(1)?;

                        len_slice[0] as usize
                    }
                    <&str>::SIGNATURE_CHAR | ObjectPath::SIGNATURE_CHAR => {
                        let alignment = u32::alignment(self.ctxt.format());
                        self.parse_padding(alignment)?;
                        let len_slice = self.next_slice(alignment)?;

                        B::read_u32(len_slice) as usize
                    }
                    c => {
                        let expected = format!(
                            "`{}`, `{}`, `{}` or `{}`",
                            <&str>::SIGNATURE_STR,
                            Signature::SIGNATURE_STR,
                            ObjectPath::SIGNATURE_STR,
                            VARIANT_SIGNATURE_CHAR,
                        );
                        return Err(de::Error::invalid_type(
                            de::Unexpected::Char(c),
                            &expected.as_str(),
                        ));
                    }
                };
                let slice = self.next_slice(len)?;
                self.pos += 1; // skip trailing null byte
                str::from_utf8(slice).map_err(Error::Utf8)?
            }
            EncodingFormat::GVariant => {
                if self.sig_parser.next_char() == VARIANT_SIGNATURE_CHAR {
                    // GVariant decided to skip the trailing nul at the end of signature string
                    str::from_utf8(&self.bytes[self.pos..]).map_err(Error::Utf8)?
                } else {
                    let cstr =
                        CStr::from_bytes_with_nul(&self.bytes[self.pos..]).map_err(|_| {
                            let c = self.bytes[self.bytes.len() - 1] as char;
                            de::Error::invalid_value(
                                de::Unexpected::Char(c),
                                &"nul byte expected at the end of strings",
                            )
                        })?;
                    let s = cstr.to_str().map_err(Error::Utf8)?;
                    self.pos += s.len() + 1; // string and trailing null byte

                    s
                }
            }
        };
        self.sig_parser.skip_char()?;

        visitor.visit_borrowed_str(s)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // FIXME: Corresponds to GVariant's `Maybe` type.
        todo!();
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.ctxt.format() {
            EncodingFormat::GVariant => {
                let byte = self.bytes[self.pos];
                if byte != 0 {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Bytes(&self.bytes[self.pos..self.pos + 1]),
                        &"0 byte expected for empty tuples (unit type)",
                    ));
                }

                self.pos += 1;
            }
            EncodingFormat::DBus => (),
        }

        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.sig_parser.next_char() {
            VARIANT_SIGNATURE_CHAR => {
                self.sig_parser.skip_char()?;
                let alignment = match self.ctxt.format() {
                    EncodingFormat::DBus => VARIANT_ALIGNMENT_DBUS,
                    EncodingFormat::GVariant => VARIANT_ALIGNMENT_GVARIANT,
                };
                self.parse_padding(alignment)?;
                let value_de = ValueDeserializer::new(self)?;

                visitor.visit_seq(value_de)
            }
            ARRAY_SIGNATURE_CHAR => {
                self.sig_parser.skip_char()?;
                let next_signature_char = self.sig_parser.next_char();
                let array_de = ArrayDeserializer::new(self)?;

                if next_signature_char == DICT_ENTRY_SIG_START_CHAR {
                    visitor.visit_map(array_de)
                } else {
                    visitor.visit_seq(array_de)
                }
            }
            STRUCT_SIG_START_CHAR => {
                self.sig_parser.skip_char()?;
                self.parse_padding(STRUCT_ALIGNMENT_DBUS)?;

                visitor
                    .visit_seq(StructureDeserializer { de: self })
                    .and_then(|v| {
                        self.sig_parser.skip_char()?;

                        Ok(v)
                    })
            }
            c => Err(de::Error::invalid_type(
                de::Unexpected::Char(c),
                &format!(
                    "`{}`, `{}` or `{}`",
                    VARIANT_SIGNATURE_CHAR, ARRAY_SIGNATURE_CHAR, STRUCT_SIG_START_CHAR,
                )
                .as_str(),
            )),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_struct("", &[], visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_struct(name, &[], visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(Enum::<B> { de: self, name })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Not using serialize_u32 cause identifier isn't part of the signature
        self.parse_padding(u32::alignment(self.ctxt.format()))?;
        let variant_index = from_slice_fds::<B, _>(&self.bytes[self.pos..], self.fds, self.ctxt)?;
        self.pos += u32::alignment(self.ctxt.format());

        visitor.visit_u32(variant_index)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct ArrayDeserializer<'d, 'de, 'sig, 'f, B> {
    de: &'d mut Deserializer<'de, 'sig, 'f, B>,
    len: usize,
    start: usize,
    // alignement of element
    element_alignment: usize,
    // where value signature starts
    element_signature_len: usize,
    // All offsets (GVariant-specific)
    offsets: Option<FramingOffsets>,
    // Length of all the offsets after the arrray
    offsets_len: usize,
    // size of the framing offset of last dict-entry key read (GVariant-specific)
    key_offset_size: Option<FramingOffsetSize>,
}

impl<'d, 'de, 'sig, 'f, B> ArrayDeserializer<'d, 'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, B>) -> Result<Self> {
        let mut len = match de.ctxt.format() {
            EncodingFormat::DBus => {
                de.parse_padding(ARRAY_ALIGNMENT_DBUS)?;

                B::read_u32(de.next_slice(4)?) as usize
            }
            EncodingFormat::GVariant => de.bytes.len() - de.pos,
        };

        let element_signature_pos = de.sig_parser.pos();
        let rest_of_signature =
            Signature::from_str_unchecked(&de.sig_parser.signature()[element_signature_pos..]);
        let element_signature = slice_signature(&rest_of_signature)?;
        let element_alignment = alignment_for_signature(&element_signature, de.ctxt.format());
        let element_signature_len = element_signature.len();
        let fixed_sized_child = crate::utils::is_fixed_sized_signature(&element_signature)?;
        let fixed_sized_key = if de.sig_parser.next_char() == DICT_ENTRY_SIG_START_CHAR {
            // Key signature can only be 1 char
            let key_signature = Signature::from_str_unchecked(&element_signature[1..2]);

            crate::utils::is_fixed_sized_signature(&key_signature)?
        } else {
            false
        };

        // D-Bus requires padding for the first element even when there is no first element
        // (i-e empty array) so we parse padding already. In case of GVariant this is just
        // the padding of the array itself since array starts with first element.
        let padding = de.parse_padding(element_alignment)?;
        let (offsets, offsets_len, key_offset_size) = match de.ctxt.format() {
            EncodingFormat::GVariant => {
                len -= padding;

                if !fixed_sized_child {
                    let (array_offsets, offsets_len) =
                        FramingOffsets::from_encoded_array(&de.bytes[de.pos..]);
                    len -= offsets_len;
                    let key_offset_size = if !fixed_sized_key {
                        // The actual offset for keys is calculated per key later, this is just to
                        // put Some value to indicate at key is not fixed sized and thus uses
                        // offsets.
                        Some(FramingOffsetSize::U8)
                    } else {
                        None
                    };

                    (Some(array_offsets), offsets_len, key_offset_size)
                } else {
                    (None, 0, None)
                }
            }
            EncodingFormat::DBus => (None, 0, None),
        };
        let start = de.pos;

        if de.sig_parser.next_char() == DICT_ENTRY_SIG_START_CHAR {
            de.sig_parser.skip_char()?;
        }

        Ok(Self {
            de,
            len,
            start,
            element_alignment,
            element_signature_len,
            offsets,
            offsets_len,
            key_offset_size,
        })
    }

    fn element_end(&mut self, pop: bool) -> Result<usize> {
        match self.offsets.as_mut() {
            Some(offsets) => {
                assert_eq!(self.de.ctxt.format(), EncodingFormat::GVariant);

                let offset = if pop { offsets.pop() } else { offsets.peek() };
                match offset {
                    Some(offset) => Ok(self.start + offset),
                    None => Err(Error::MissingFramingOffset),
                }
            }
            None => Ok(self.start + self.len),
        }
    }

    fn done(&self) -> bool {
        match self.offsets.as_ref() {
            // If all offsets have been popped/used, we're already at the end
            Some(offsets) => offsets.is_empty(),
            None => self.de.pos == self.start + self.len,
        }
    }
}

impl<'d, 'de, 'sig, 'f, B> SeqAccess<'de> for ArrayDeserializer<'d, 'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.done() {
            self.de.sig_parser.skip_chars(self.element_signature_len)?;
            self.de.pos += self.offsets_len;

            return Ok(None);
        }

        let ctxt =
            EncodingContext::new(self.de.ctxt.format(), self.de.ctxt.position() + self.de.pos);
        let end = self.element_end(true)?;

        let mut de = Deserializer::<B> {
            ctxt,
            sig_parser: self.de.sig_parser.clone(),
            bytes: &self.de.bytes[self.de.pos..end],
            fds: self.de.fds,
            pos: 0,
            b: PhantomData,
        };

        let v = seed.deserialize(&mut de).map(Some);
        self.de.pos += de.pos;

        if self.de.pos > self.start + self.len {
            return Err(serde::de::Error::invalid_length(
                self.len,
                &format!(">= {}", self.de.pos - self.start).as_str(),
            ));
        }

        v
    }
}

impl<'d, 'de, 'sig, 'f, B> MapAccess<'de> for ArrayDeserializer<'d, 'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.done() {
            // Starting bracket was already skipped
            self.de
                .sig_parser
                .skip_chars(self.element_signature_len - 1)?;
            self.de.pos += self.offsets_len;

            return Ok(None);
        }

        self.de.parse_padding(self.element_alignment)?;

        let ctxt =
            EncodingContext::new(self.de.ctxt.format(), self.de.ctxt.position() + self.de.pos);
        let element_end = self.element_end(false)?;

        let key_end = match self.key_offset_size {
            Some(_) => {
                let offset_size =
                    FramingOffsetSize::for_encoded_container(element_end - self.de.pos);
                self.key_offset_size.replace(offset_size);

                self.de.pos
                    + offset_size
                        .read_last_offset_from_buffer(&self.de.bytes[self.de.pos..element_end])
            }
            None => element_end,
        };

        let mut de = Deserializer::<B> {
            ctxt,
            sig_parser: self.de.sig_parser.clone(),
            bytes: &self.de.bytes[self.de.pos..key_end],
            fds: self.de.fds,
            pos: 0,
            b: PhantomData,
        };
        let v = seed.deserialize(&mut de).map(Some);
        self.de.pos += de.pos;

        if self.de.pos > self.start + self.len {
            return Err(serde::de::Error::invalid_length(
                self.len,
                &format!(">= {}", self.de.pos - self.start).as_str(),
            ));
        }

        v
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let ctxt =
            EncodingContext::new(self.de.ctxt.format(), self.de.ctxt.position() + self.de.pos);
        let element_end = self.element_end(true)?;
        let value_end = match self.key_offset_size {
            Some(key_offset_size) => element_end - key_offset_size as usize,
            None => element_end,
        };
        let mut sig_parser = self.de.sig_parser.clone();
        // Skip key signature (always 1 char)
        sig_parser.skip_char()?;

        let mut de = Deserializer::<B> {
            ctxt,
            sig_parser,
            bytes: &self.de.bytes[self.de.pos..value_end],
            fds: self.de.fds,
            pos: 0,
            b: PhantomData,
        };
        let v = seed.deserialize(&mut de);
        self.de.pos += de.pos;

        if let Some(key_offset_size) = self.key_offset_size {
            self.de.pos += key_offset_size as usize;
        }

        if self.de.pos > self.start + self.len {
            return Err(serde::de::Error::invalid_length(
                self.len,
                &format!(">= {}", self.de.pos - self.start).as_str(),
            ));
        }

        v
    }
}

#[derive(Debug)]
struct StructureDeserializer<'d, 'de, 'sig, 'f, B> {
    de: &'d mut Deserializer<'de, 'sig, 'f, B>,
}

impl<'d, 'de, 'sig, 'f, B> SeqAccess<'de> for StructureDeserializer<'d, 'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de).map(Some)
    }
}

#[derive(Debug)]
enum ValueParseStage {
    Signature,
    Value,
    Done,
}

#[derive(Debug)]
struct ValueDeserializer<'d, 'de, 'sig, 'f, B> {
    de: &'d mut Deserializer<'de, 'sig, 'f, B>,
    stage: ValueParseStage,
    sig_start: usize,
    sig_end: usize,
    value_start: usize,
    value_end: usize,
}

impl<'d, 'de, 'sig, 'f, B> ValueDeserializer<'d, 'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    fn new(de: &'d mut Deserializer<'de, 'sig, 'f, B>) -> Result<Self> {
        // GVariant format has signature at the end
        let (sig_start, sig_end, value_start, value_end) = match de.ctxt.format() {
            EncodingFormat::DBus => {
                let sig_len = de.bytes[de.pos] as usize;
                let sig_end = de.pos + sig_len + 2;
                // value_end not necessarily correct but in this case this doesn't matter.
                (de.pos, sig_end, sig_end, de.bytes.len())
            }
            EncodingFormat::GVariant => {
                let mut seperator_pos = None;

                // Search for the nul byte seperator
                for i in (de.pos..de.bytes.len() - 1).rev() {
                    if de.bytes[i] == b'\0' {
                        seperator_pos = Some(i);

                        break;
                    }
                }

                match seperator_pos {
                    None => {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Bytes(&de.bytes[de.pos..]),
                            &"nul byte seperator between Variant's value & signature",
                        ));
                    }
                    Some(seperator_pos) => {
                        (seperator_pos + 1, de.bytes.len(), de.pos, seperator_pos)
                    }
                }
            }
        };

        Ok(ValueDeserializer::<B> {
            de,
            stage: ValueParseStage::Signature,
            sig_start,
            sig_end,
            value_start,
            value_end,
        })
    }
}

impl<'d, 'de, 'sig, 'f, B> SeqAccess<'de> for ValueDeserializer<'d, 'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.stage {
            ValueParseStage::Signature => {
                self.stage = ValueParseStage::Value;

                let signature = Signature::from_str_unchecked(VARIANT_SIGNATURE_STR);
                let sig_parser = SignatureParser::new(signature);

                let mut de = Deserializer::<B> {
                    // No padding in signatures so just pass the same context
                    ctxt: self.de.ctxt,
                    sig_parser,
                    bytes: &self.de.bytes[self.sig_start..self.sig_end],
                    fds: self.de.fds,
                    pos: 0,
                    b: PhantomData,
                };

                let s = seed.deserialize(&mut de).map(Some);

                self.de.pos = match self.de.ctxt.format() {
                    EncodingFormat::DBus => self.de.pos + de.pos,
                    // No incremement needed in this case; we'll set pos after value parsing below.
                    EncodingFormat::GVariant => 0,
                };

                s
            }
            ValueParseStage::Value => {
                self.stage = ValueParseStage::Done;

                let (sig_start, sig_end) = match self.de.ctxt.format() {
                    // skip length byte & // trim trailing nul byte
                    EncodingFormat::DBus => (self.sig_start + 1, self.sig_end - 1),
                    EncodingFormat::GVariant => (self.sig_start, self.sig_end),
                };
                let slice = &self.de.bytes[sig_start..sig_end];
                // FIXME: Can we just use `Signature::from_bytes_unchecked`?
                let signature = Signature::try_from(slice)?;
                let sig_parser = SignatureParser::new(signature);

                let ctxt = EncodingContext::new(
                    self.de.ctxt.format(),
                    self.de.ctxt.position() + self.value_start,
                );
                let mut de = Deserializer::<B> {
                    ctxt,
                    sig_parser,
                    bytes: &self.de.bytes[self.value_start..self.value_end],
                    fds: self.de.fds,
                    pos: 0,
                    b: PhantomData,
                };

                let v = seed.deserialize(&mut de).map(Some);

                self.de.pos = match self.de.ctxt.format() {
                    EncodingFormat::DBus => self.de.pos + de.pos,
                    EncodingFormat::GVariant => self.sig_end,
                };

                v
            }
            ValueParseStage::Done => Ok(None),
        }
    }
}

struct Enum<'d, 'de: 'd, 'sig, 'f, B> {
    de: &'d mut Deserializer<'de, 'sig, 'f, B>,
    name: &'static str,
}

impl<'de, 'd, 'sig, 'f, B> EnumAccess<'de> for Enum<'d, 'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de).map(|v| (v, self))
    }
}

impl<'de, 'd, 'sig, 'f, B> VariantAccess<'de> for Enum<'d, 'de, 'sig, 'f, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        self.de.sig_parser.skip_char()?;

        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, self.name, &[], visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.de, self.name, fields, visitor)
    }
}
