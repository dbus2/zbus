use byteorder::WriteBytesExt;
use serde::{ser, ser::SerializeSeq, Serialize};
use std::io::{Seek, Write};
use std::os::unix::io::RawFd;
use std::{marker::PhantomData, str};

use crate::signature_parser::SignatureParser;
use crate::utils::*;
use crate::Type;
use crate::{Basic, EncodingContext};
use crate::{Error, Result};
use crate::{ObjectPath, Signature};

struct NullWriteSeek;

impl Write for NullWriteSeek {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Seek for NullWriteSeek {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        Ok(std::u64::MAX) // should never read the return value!
    }
}

pub fn serialized_size<T: ?Sized>(value: &T) -> Result<(usize, usize)>
where
    T: Serialize + Type,
{
    let signature = T::signature();
    let mut null = NullWriteSeek;

    let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
    let (len, fds) = to_write_fds_for_signature(&mut null, ctxt, &signature, value)?;
    Ok((len, fds.len()))
}

/// Serialize `T` to the given `write`.
///
/// # Panics
///
/// This function will panic if the value to serialize contains file descriptors. Use
/// [`to_write_fds`] if you'd want to potentially pass FDs.
///
/// # Examples
///
/// ```
/// use zvariant::{EncodingContext, from_slice, to_write};
/// let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
/// let mut cursor = std::io::Cursor::new(vec![]);
/// to_write(&mut cursor, ctxt, &42u32).unwrap();
/// let value: u32 = from_slice(cursor.get_ref(), ctxt).unwrap();
/// assert_eq!(value, 42);
/// ```
///
/// [`to_write_fds`]: fn.to_write_fds.html
pub fn to_write<B, W, T: ?Sized>(
    write: &mut W,
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<usize>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize + Type,
{
    let signature = T::signature();

    to_write_for_signature(write, ctxt, &signature, value)
}

pub fn to_write_fds<B, W, T: ?Sized>(
    write: &mut W,
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<(usize, Vec<RawFd>)>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize + Type,
{
    let signature = T::signature();

    to_write_fds_for_signature(write, ctxt, &signature, value)
}

pub fn to_bytes<B, T: ?Sized>(ctxt: EncodingContext<B>, value: &T) -> Result<Vec<u8>>
where
    B: byteorder::ByteOrder,
    T: Serialize + Type,
{
    Ok(to_bytes_fds(ctxt, value)?.0)
}

pub fn to_bytes_fds<B, T: ?Sized>(
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<(Vec<u8>, Vec<RawFd>)>
where
    B: byteorder::ByteOrder,
    T: Serialize + Type,
{
    let mut cursor = std::io::Cursor::new(vec![]);
    let (_, fds) = to_write_fds(&mut cursor, ctxt, value)?;
    Ok((cursor.into_inner(), fds))
}

pub fn to_write_for_signature<'s, 'sig, B, W, T: ?Sized>(
    write: &mut W,
    ctxt: EncodingContext<B>,
    signature: &'s Signature<'sig>,
    value: &T,
) -> Result<usize>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize,
{
    let (len, fds) = to_write_fds_for_signature(write, ctxt, signature, value)?;
    if !fds.is_empty() {
        panic!("can't serialize with FDs")
    }

    Ok(len)
}

pub fn to_write_fds_for_signature<'s, 'sig, B, W, T: ?Sized>(
    write: &mut W,
    ctxt: EncodingContext<B>,
    signature: &'s Signature<'sig>,
    value: &T,
) -> Result<(usize, Vec<RawFd>)>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize,
{
    let mut fds = vec![];
    let mut serializer = Serializer::<B, W>::new(signature, write, &mut fds, ctxt);
    value.serialize(&mut serializer)?;
    Ok((serializer.bytes_written, fds))
}

pub fn to_bytes_for_signature<'s, 'sig, B, T: ?Sized>(
    ctxt: EncodingContext<B>,
    signature: &'s Signature<'sig>,
    value: &T,
) -> Result<Vec<u8>>
where
    B: byteorder::ByteOrder,
    T: Serialize,
{
    let mut cursor = std::io::Cursor::new(vec![]);
    to_write_for_signature(&mut cursor, ctxt, signature, value)?;
    Ok(cursor.into_inner())
}

pub struct Serializer<'ser, 'sig, B, W> {
    pub(self) ctxt: EncodingContext<B>,
    pub(self) write: &'ser mut W,
    pub(self) bytes_written: usize,
    pub(self) fds: &'ser mut Vec<RawFd>,

    pub(self) sign_parser: SignatureParser<'sig>,

    pub(self) value_sign: Option<Signature<'static>>,

    b: PhantomData<B>,
}

impl<'ser, 'sig, B, W> Serializer<'ser, 'sig, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    pub fn new<'w: 'ser, 'f: 'ser, 's>(
        signature: &'s Signature<'sig>,
        write: &'w mut W,
        fds: &'f mut Vec<RawFd>,
        ctxt: EncodingContext<B>,
    ) -> Self {
        let sign_parser = SignatureParser::new(signature.clone());

        Self {
            ctxt,
            sign_parser,
            write,
            fds,
            bytes_written: 0,
            value_sign: None,
            b: PhantomData,
        }
    }

    fn add_fd(&mut self, fd: RawFd) -> Result<u32> {
        if let Some(idx) = self.fds.iter().position(|&x| x == fd) {
            return Ok(idx as u32);
        }
        let idx = self.fds.len();
        self.fds.push(fd);
        Ok(idx as u32)
    }

    fn add_padding(&mut self, alignment: usize) -> Result<usize> {
        let padding = padding_for_n_bytes(self.abs_pos(), alignment);
        if padding > 0 {
            let byte = [0_u8; 1];
            for _ in 0..padding {
                self.write_all(&byte).map_err(Error::Io)?;
            }
        }

        Ok(padding)
    }

    fn prep_serialize_basic<T>(&mut self) -> Result<()>
    where
        T: Basic,
    {
        self.sign_parser.parse_char(Some(T::SIGNATURE_CHAR))?;
        self.add_padding(T::ALIGNMENT)?;

        Ok(())
    }

    fn prep_serialize_enum_variant(&mut self, variant_index: u32) -> Result<()> {
        // Encode enum variants as a struct with first field as variant index
        self.add_padding(u32::ALIGNMENT)?;
        self.write_u32::<B>(variant_index).map_err(Error::Io)?;

        Ok(())
    }

    fn abs_pos(&self) -> usize {
        self.ctxt.position() + self.bytes_written
    }
}

impl<'ser, 'sig, B, W> Write for Serializer<'ser, 'sig, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    /// Write `buf` and increment internal bytes written counter.
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write.write(buf).map(|n| {
            self.bytes_written += n;

            n
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.write.flush()
    }
}

impl<'ser, 'sig, 'b, B, W> ser::Serializer for &'b mut Serializer<'ser, 'sig, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'ser, 'sig, 'b, B, W>;
    type SerializeTuple = StructSerializer<'ser, 'sig, 'b, B, W>;
    type SerializeTupleStruct = StructSerializer<'ser, 'sig, 'b, B, W>;
    type SerializeTupleVariant = StructSerializer<'ser, 'sig, 'b, B, W>;
    type SerializeMap = SeqSerializer<'ser, 'sig, 'b, B, W>;
    type SerializeStruct = StructSerializer<'ser, 'sig, 'b, B, W>;
    type SerializeStructVariant = StructSerializer<'ser, 'sig, 'b, B, W>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.prep_serialize_basic::<bool>()?;
        self.write_u32::<B>(v as u32).map_err(Error::Io)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        // No i8 type in D-Bus/GVariant, let's pretend it's i16
        self.write_i16::<B>(v as i16).map_err(Error::Io)
    }

    // TODO: Use macro to avoid code-duplication here
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.prep_serialize_basic::<i16>()?;
        self.write_i16::<B>(v).map_err(Error::Io)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        match self.sign_parser.next_char()? {
            'h' => {
                self.sign_parser.parse_char(None)?;
                self.add_padding(u32::ALIGNMENT)?;
                let v = self.add_fd(v)?;
                self.write_u32::<B>(v).map_err(Error::Io)
            }
            _ => {
                self.prep_serialize_basic::<i32>()?;
                self.write_i32::<B>(v).map_err(Error::Io)
            }
        }
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.prep_serialize_basic::<i64>()?;
        self.write_i64::<B>(v).map_err(Error::Io)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.prep_serialize_basic::<u8>()?;
        // Endianness is irrelevant for single bytes.
        self.write_u8(v).map_err(Error::Io)
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.prep_serialize_basic::<u16>()?;
        self.write_u16::<B>(v).map_err(Error::Io)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.prep_serialize_basic::<u32>()?;
        self.write_u32::<B>(v).map_err(Error::Io)
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.prep_serialize_basic::<u64>()?;
        self.write_u64::<B>(v).map_err(Error::Io)
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        // No f32 type in D-Bus/GVariant, let's pretend it's f64
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.prep_serialize_basic::<f64>()?;
        self.write_f64::<B>(v).map_err(Error::Io)
    }

    fn serialize_char(self, v: char) -> Result<()> {
        // No char type in D-Bus/GVariant, let's pretend it's a string
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let c = self.sign_parser.next_char()?;

        match c {
            ObjectPath::SIGNATURE_CHAR | <&str>::SIGNATURE_CHAR => {
                self.add_padding(<&str>::ALIGNMENT)?;
                self.write_u32::<B>(usize_to_u32(v.len()))
                    .map_err(Error::Io)?;
            }
            Signature::SIGNATURE_CHAR | VARIANT_SIGNATURE_CHAR => {
                self.write_u8(usize_to_u8(v.len())).map_err(Error::Io)?;

                if c == VARIANT_SIGNATURE_CHAR {
                    self.value_sign = Some(signature_string!(v));
                }
            }
            _ => {
                let expected = format!(
                    "`{}`, `{}`, `{}` or `{}`",
                    <&str>::SIGNATURE_STR,
                    Signature::SIGNATURE_STR,
                    ObjectPath::SIGNATURE_STR,
                    VARIANT_SIGNATURE_CHAR,
                );
                return Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Char(c),
                    &expected.as_str(),
                ));
            }
        }
        self.sign_parser.parse_char(None)?;
        self.write_all(&v.as_bytes()).map_err(Error::Io)?;
        self.write_all(&b"\0"[..]).map_err(Error::Io)?;

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }

        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        // FIXME: Corresponds to GVariant's `Maybe` type, which is empty (no bytes) for None.
        todo!();
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // FIXME: Corresponds to GVariant's `Maybe` type.
        todo!();
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        variant_index.serialize(self)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)?;

        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.prep_serialize_enum_variant(variant_index)?;

        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.sign_parser.parse_char(Some(ARRAY_SIGNATURE_CHAR))?;
        self.add_padding(ARRAY_ALIGNMENT)?;
        // Length in bytes (unfortunately not the same as len passed to us here) which we initially
        // set to 0.
        self.write_u32::<B>(0_u32).map_err(Error::Io)?;

        let next_signature_char = self.sign_parser.next_char()?;
        let alignment = alignment_for_signature_char(next_signature_char, self.ctxt.format());
        let start = self.bytes_written;
        // D-Bus expects us to add padding for the first element even when there is no first
        // element (i-e empty array) so we add padding already.
        let first_padding = self.add_padding(alignment)?;
        let element_signature_pos = self.sign_parser.pos();
        let rest_of_signature =
            Signature::from_str_unchecked(&self.sign_parser.signature()[element_signature_pos..]);
        let element_signature = slice_signature(&rest_of_signature)?;
        let element_signature_len = element_signature.len();

        Ok(SeqSerializer {
            serializer: self,
            start,
            element_signature_len,
            first_padding,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_struct("", len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.prep_serialize_enum_variant(variant_index)?;

        self.serialize_struct(name, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.serialize_seq(len)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        let c = self.sign_parser.next_char()?;
        let end_parens;
        if c == VARIANT_SIGNATURE_CHAR {
            end_parens = None;
        } else {
            self.sign_parser.parse_char(Some(c))?;
            self.add_padding(STRUCT_ALIGNMENT)?;

            if c == STRUCT_SIG_START_CHAR {
                end_parens = Some(STRUCT_SIG_END_CHAR);
            } else if c == DICT_ENTRY_SIG_START_CHAR {
                end_parens = Some(DICT_ENTRY_SIG_END_CHAR);
            } else {
                let expected = format!(
                    "`{}`, `{}`, `{}` or `{}`",
                    <&str>::SIGNATURE_STR,
                    Signature::SIGNATURE_STR,
                    ObjectPath::SIGNATURE_STR,
                    VARIANT_SIGNATURE_CHAR,
                );
                return Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Char(c),
                    &expected.as_str(),
                ));
            }
        }

        Ok(StructSerializer {
            serializer: self,
            end_parens,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.prep_serialize_enum_variant(variant_index)?;

        self.serialize_struct(name, len)
    }
}

#[doc(hidden)]
pub struct SeqSerializer<'ser, 'sig, 'b, B, W> {
    serializer: &'b mut Serializer<'ser, 'sig, B, W>,
    start: usize,
    // where value signature starts
    element_signature_len: usize,
    // First element's padding
    first_padding: usize,
}

impl<'ser, 'sig, 'b, B, W> SeqSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    pub(self) fn end_seq(self) -> Result<()> {
        if self.start + self.first_padding == self.serializer.bytes_written {
            // Empty sequence so we need to parse the element signature.
            self.serializer
                .sign_parser
                .skip_chars(self.element_signature_len)?;
        }

        // Set size of array in bytes
        let array_len = self.serializer.bytes_written - self.start;
        let len = usize_to_u32(array_len - self.first_padding);
        self.serializer
            .write
            .seek(std::io::SeekFrom::Current(-(array_len as i64) - 4))
            .map_err(Error::Io)?;
        self.serializer
            .write
            .write_u32::<B>(len)
            .map_err(Error::Io)?;
        self.serializer
            .write
            .seek(std::io::SeekFrom::Current(array_len as i64))
            .map_err(Error::Io)?;

        Ok(())
    }
}

impl<'ser, 'sig, 'b, B, W> ser::SerializeSeq for SeqSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.start + self.first_padding != self.serializer.bytes_written {
            // The signature needs to be rewinded before encoding each element.
            self.serializer
                .sign_parser
                .rewind_chars(self.element_signature_len);
        }
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

#[doc(hidden)]
pub struct StructSerializer<'ser, 'sig, 'b, B, W> {
    serializer: &'b mut Serializer<'ser, 'sig, B, W>,
    end_parens: Option<char>,
}

impl<'ser, 'sig, 'b, B, W> StructSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    fn serialize_struct_element<T>(&mut self, name: Option<&'static str>, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match name {
            Some("zvariant::Value::Value") => {
                // Serializing the value of a Value, which means signature was serialized
                // already, and also put aside for us to be picked here.
                let signature = self
                    .serializer
                    .value_sign
                    .take()
                    .expect("Incorrect Value encoding");

                let sign_parser = SignatureParser::new(signature);
                let mut serializer = Serializer::<B, W> {
                    ctxt: self.serializer.ctxt,
                    sign_parser,
                    write: &mut self.serializer.write,
                    fds: self.serializer.fds,
                    bytes_written: self.serializer.bytes_written,
                    value_sign: None,
                    b: PhantomData,
                };
                value.serialize(&mut serializer)?;
                self.serializer.bytes_written = serializer.bytes_written;

                Ok(())
            }
            _ => value.serialize(&mut *self.serializer),
        }
    }

    fn end_struct(self) -> Result<()> {
        if let Some(c) = self.end_parens {
            self.serializer.sign_parser.parse_char(Some(c))?;
        }

        Ok(())
    }
}

impl<'ser, 'sig, 'b, B, W> ser::SerializeTuple for StructSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_element(None, value)
    }

    fn end(self) -> Result<()> {
        self.end_struct()
    }
}

impl<'ser, 'sig, 'b, B, W> ser::SerializeTupleStruct for StructSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_element(None, value)
    }

    fn end(self) -> Result<()> {
        self.end_struct()
    }
}

impl<'ser, 'sig, 'b, B, W> ser::SerializeTupleVariant for StructSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_element(None, value)
    }

    fn end(self) -> Result<()> {
        self.end_struct()
    }
}

impl<'ser, 'sig, 'b, B, W> ser::SerializeMap for SeqSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    // TODO: The Serde data model allows map keys to be any serializable type. We can only support keys of
    // basic types so the implementation below will produce invalid encoding if the key serializes
    // is something other than a basic type.
    //
    // We need to validate that map keys are of basic type. We do this by using a different Serializer
    // to serialize the key (instead of `&mut **self`) and having that other serializer only implement
    // `serialize_*` for basic types and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.start + self.first_padding == self.serializer.bytes_written {
            // First key
            self.serializer
                .sign_parser
                .parse_char(Some(DICT_ENTRY_SIG_START_CHAR))?;
        } else {
            // The signature needs to be rewinded before encoding each element.
            self.serializer
                .sign_parser
                .rewind_chars(self.element_signature_len - 2);
        }
        self.serializer.add_padding(DICT_ENTRY_ALIGNMENT)?;

        key.serialize(&mut *self.serializer)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<()> {
        if self.start + self.first_padding != self.serializer.bytes_written {
            // Non-empty map, take }
            self.serializer
                .sign_parser
                .parse_char(Some(DICT_ENTRY_SIG_END_CHAR))?;
        }
        self.end_seq()
    }
}

impl<'ser, 'sig, 'b, B, W> ser::SerializeStruct for StructSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_element(Some(key), value)
    }

    fn end(self) -> Result<()> {
        self.end_struct()
    }
}

impl<'ser, 'sig, 'b, B, W> ser::SerializeStructVariant for StructSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_element(Some(key), value)
    }

    fn end(self) -> Result<()> {
        self.end_struct()
    }
}
