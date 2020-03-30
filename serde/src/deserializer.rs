use serde::de::{self, DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::Deserialize;

use std::{marker::PhantomData, str};

use crate::signature_parser::SignatureParser;
use crate::utils::*;
use crate::VariantValue;
use crate::{Basic, EncodingFormat};
use crate::{Error, Result};
use crate::{ObjectPath, Signature};

pub fn from_slice<'d, 'r: 'd, B, T: ?Sized>(bytes: &'r [u8], format: EncodingFormat) -> Result<T>
where
    B: byteorder::ByteOrder,
    T: Deserialize<'d> + VariantValue,
{
    let signature = T::signature();

    from_slice_for_signature::<B, _, _>(bytes, format, signature)
}

pub fn from_slice_for_signature<'d, 'r: 'd, 's: 'd, B, S, T: ?Sized>(
    bytes: &'r [u8],
    format: EncodingFormat,
    signature: S,
) -> Result<T>
where
    B: byteorder::ByteOrder,
    S: Into<Signature<'s>>,
    T: Deserialize<'d>,
{
    let mut de = Deserializer::<B>::new(bytes, signature, format);

    T::deserialize(&mut de)
}

pub struct Deserializer<'de, B> {
    pub(self) format: EncodingFormat,
    pub(self) bytes: &'de [u8],
    pub(self) pos: usize,

    pub(self) sign_parser: SignatureParser<'de>,

    b: PhantomData<B>,
}

impl<'de, B> Deserializer<'de, B>
where
    B: byteorder::ByteOrder,
{
    pub fn new<'s: 'de, 'r: 'de>(
        bytes: &'r [u8],
        signature: impl Into<Signature<'s>>,
        format: EncodingFormat,
    ) -> Self {
        let sign_parser = SignatureParser::new(signature.into());

        Self {
            format,
            sign_parser,
            bytes,
            pos: 0,
            b: PhantomData,
        }
    }

    fn parse_padding(&mut self, alignment: usize) -> Result<usize> {
        let padding = padding_for_n_bytes(self.pos, alignment);
        if padding > 0 {
            for i in 0..padding {
                if self.bytes[self.pos + i] != 0 {
                    return Err(Error::PaddingNot0);
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
        self.sign_parser.parse_char(Some(T::SIGNATURE_CHAR))?;
        self.parse_padding(T::ALIGNMENT)?;

        Ok(())
    }

    fn next_slice(&mut self, len: usize) -> Result<&'de [u8]> {
        if self.pos + len > self.bytes.len() {
            return Err(Error::InsufficientData);
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

        self.next_slice(T::ALIGNMENT)
    }
}

impl<'de, 'd, B> de::Deserializer<'de> for &'d mut Deserializer<'de, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.sign_parser.next_char()? {
            u8::SIGNATURE_CHAR => self.deserialize_u8(visitor),
            bool::SIGNATURE_CHAR => self.deserialize_bool(visitor),
            i16::SIGNATURE_CHAR => self.deserialize_i16(visitor),
            u16::SIGNATURE_CHAR => self.deserialize_u16(visitor),
            i32::SIGNATURE_CHAR => self.deserialize_i32(visitor),
            u32::SIGNATURE_CHAR => self.deserialize_u32(visitor),
            i64::SIGNATURE_CHAR => self.deserialize_i64(visitor),
            u64::SIGNATURE_CHAR => self.deserialize_u64(visitor),
            f64::SIGNATURE_CHAR => self.deserialize_f64(visitor),
            <&str>::SIGNATURE_CHAR | ObjectPath::SIGNATURE_CHAR | Signature::SIGNATURE_CHAR => {
                self.deserialize_str(visitor)
            }
            VARIANT_SIGNATURE_CHAR => self.deserialize_seq(visitor),
            ARRAY_SIGNATURE_CHAR => {
                // FIXME: Should be different for dict
                self.deserialize_seq(visitor)
            }
            STRUCT_SIG_START_CHAR => self.deserialize_seq(visitor),
            _ => Err(Error::UnsupportedType(String::from(
                self.sign_parser.signature(),
            ))),
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
            _ => return Err(Error::IncorrectValue),
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
        let v = B::read_i32(self.next_const_size_slice::<i32>()?);

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
        let len = match self.sign_parser.next_char()? {
            Signature::SIGNATURE_CHAR | VARIANT_SIGNATURE_CHAR => {
                let len_slice = self.next_slice(1)?;

                len_slice[0] as usize
            }
            <&str>::SIGNATURE_CHAR | ObjectPath::SIGNATURE_CHAR => {
                self.parse_padding(u32::ALIGNMENT)?;
                let len_slice = self.next_slice(u32::ALIGNMENT)?;

                B::read_u32(len_slice) as usize
            }
            _ => {
                // TODO: Better error here with more info
                return Err(Error::IncorrectType);
            }
        };
        self.sign_parser.parse_char(None)?;
        let slice = self.next_slice(len)?;
        let s = str::from_utf8(slice).map_err(|_| Error::InvalidUtf8)?;
        self.pos += 1; // skip trailing null byte

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

    // FIXME: What am i supposed to do with this strange type?
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_option(visitor)
    }

    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Not sure what else can we do with this?
        visitor.visit_borrowed_str(name)
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
        match self.sign_parser.next_char()? {
            VARIANT_SIGNATURE_CHAR => {
                let start = self.pos + 1; // skip length byte
                let variant_de = VariantDeserializer::<B> {
                    de: self,
                    stage: VariantParseStage::Signature,
                    start,
                };

                visitor.visit_seq(variant_de)
            }
            ARRAY_SIGNATURE_CHAR => {
                self.sign_parser.parse_char(Some(ARRAY_SIGNATURE_CHAR))?;
                self.parse_padding(ARRAY_ALIGNMENT)?;
                let len = B::read_u32(self.next_slice(4)?) as usize;

                let next_signature_char = self.sign_parser.next_char()?;
                let alignment = alignment_for_signature_char(next_signature_char, self.format);
                // D-Bus requires padding for the first element even when there is no first element
                // (i-e empty array) so we parse padding already.
                self.parse_padding(alignment)?;
                let start = self.pos;

                let element_signature_pos = self.sign_parser.pos();
                if next_signature_char == DICT_ENTRY_SIG_START_CHAR {
                    self.sign_parser
                        .parse_char(Some(DICT_ENTRY_SIG_START_CHAR))?;
                }
                let rest_of_signature =
                    Signature::from(&self.sign_parser.signature()[element_signature_pos..]);
                let element_signature = slice_signature(&rest_of_signature)?;

                if next_signature_char == DICT_ENTRY_SIG_START_CHAR {
                    // Everything except the starting and ending brackets
                    let element_signature_len = element_signature.len() - 2;
                    visitor
                        .visit_map(ArrayDeserializer {
                            de: self,
                            len,
                            start,
                            element_signature_len,
                        })
                        .and_then(|v| {
                            self.sign_parser.parse_char(Some(DICT_ENTRY_SIG_END_CHAR))?;

                            Ok(v)
                        })
                } else {
                    let element_signature_len = element_signature.len();
                    visitor.visit_seq(ArrayDeserializer {
                        de: self,
                        len,
                        start,
                        element_signature_len,
                    })
                }
            }
            STRUCT_SIG_START_CHAR => {
                self.sign_parser.parse_char(Some(STRUCT_SIG_START_CHAR))?;
                self.parse_padding(STRUCT_ALIGNMENT)?;

                visitor
                    .visit_seq(StructureDeserializer { de: self })
                    .and_then(|v| {
                        self.sign_parser.parse_char(Some(STRUCT_SIG_END_CHAR))?;

                        Ok(v)
                    })
            }
            _ => Err(Error::IncorrectType),
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
        self.parse_padding(u32::ALIGNMENT)?;
        let variant_index = from_slice::<B, _>(&self.bytes[self.pos..], self.format)?;
        self.pos += u32::ALIGNMENT;

        visitor.visit_u32(variant_index)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

pub struct ArrayDeserializer<'d, 'de, B> {
    de: &'d mut Deserializer<'de, B>,
    len: usize,
    start: usize,
    // where value signature starts
    element_signature_len: usize,
}

impl<'d, 'de, B> SeqAccess<'de> for ArrayDeserializer<'d, 'de, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.de.pos == self.start + self.len {
            if self.len == 0 {
                // Empty sequence so we need to parse the element signature.
                self.de.sign_parser.skip_chars(self.element_signature_len)?;
            }

            return Ok(None);
        }

        if self.start != self.de.pos {
            // The signature needs to be rewinded before encoding each element.
            self.de.sign_parser.rewind_chars(self.element_signature_len);
        }

        let v = seed.deserialize(&mut *self.de).map(Some);
        if self.de.pos > self.start + self.len {
            return Err(Error::InsufficientData);
        }

        v
    }
}

impl<'d, 'de, B> MapAccess<'de> for ArrayDeserializer<'d, 'de, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.de.pos == self.start + self.len {
            if self.len == 0 {
                // Empty sequence so we need to parse the element signature.
                self.de.sign_parser.skip_chars(self.element_signature_len)?;
            }

            return Ok(None);
        }

        if self.start != self.de.pos {
            // The signature needs to be rewinded before encoding each element.
            self.de.sign_parser.rewind_chars(self.element_signature_len);
            self.de.parse_padding(DICT_ENTRY_ALIGNMENT)?;
        }

        let v = seed.deserialize(&mut *self.de).map(Some);
        if self.de.pos > self.start + self.len {
            return Err(Error::InsufficientData);
        }

        v
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        // TODO: Ensure we can handle empty dict
        let v = seed.deserialize(&mut *self.de);
        if self.de.pos > self.start + self.len {
            return Err(Error::InsufficientData);
        }

        v
    }
}

pub struct StructureDeserializer<'d, 'de, B> {
    de: &'d mut Deserializer<'de, B>,
}

impl<'d, 'de, B> SeqAccess<'de> for StructureDeserializer<'d, 'de, B>
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

enum VariantParseStage {
    Signature,
    Value,
    Done,
}

pub struct VariantDeserializer<'d, 'de, B> {
    de: &'d mut Deserializer<'de, B>,
    stage: VariantParseStage,
    start: usize,
}

impl<'d, 'de, B> SeqAccess<'de> for VariantDeserializer<'d, 'de, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.stage {
            VariantParseStage::Signature => {
                self.stage = VariantParseStage::Value;

                seed.deserialize(&mut *self.de).map(Some)
            }
            VariantParseStage::Value => {
                self.stage = VariantParseStage::Done;

                let slice = &self.de.bytes[self.start..=self.de.pos];
                let signature = str::from_utf8(slice)
                    .map(Signature::from)
                    .map_err(|_| Error::InvalidUtf8)?;
                // FIXME: Remove this
                println!("decoding variant value: {}", signature.as_str());
                let sign_parser = SignatureParser::new(signature);

                let mut de = Deserializer::<B> {
                    format: self.de.format,
                    sign_parser,
                    bytes: self.de.bytes,
                    pos: self.de.pos,
                    b: PhantomData,
                };

                let v = seed.deserialize(&mut de).map(Some);
                self.de.pos = de.pos;

                v
            }
            VariantParseStage::Done => Ok(None),
        }
    }
}

struct Enum<'d, 'de: 'd, B> {
    de: &'d mut Deserializer<'de, B>,
    name: &'static str,
}

impl<'de, 'd, B> EnumAccess<'de> for Enum<'d, 'de, B>
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

impl<'de, 'd, B> VariantAccess<'de> for Enum<'d, 'de, B>
where
    B: byteorder::ByteOrder,
{
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
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
