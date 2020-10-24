use byteorder::WriteBytesExt;
use serde::{ser, ser::SerializeSeq, Serialize};
use std::io::{Seek, Write};
use std::os::unix::io::RawFd;
use std::{marker::PhantomData, str};

use crate::framing_offset_size::FramingOffsetSize;
use crate::framing_offsets::FramingOffsets;
use crate::signature_parser::SignatureParser;
use crate::utils::*;
use crate::Type;
use crate::{Basic, EncodingContext, EncodingFormat};
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

/// Calculate the serialized size of `T`.
///
/// # Panics
///
/// This function will panic if the value to serialize contains file descriptors. Use
/// [`serialized_size_fds`] if `T` (potentially) contains FDs.
///
/// # Examples
///
/// ```
/// use zvariant::{EncodingContext, serialized_size};
///
/// let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
/// let len = serialized_size(ctxt, "hello world").unwrap();
/// assert_eq!(len, 16);
///
/// let len = serialized_size(ctxt, &("hello world!", 42_u64)).unwrap();
/// assert_eq!(len, 32);
/// ```
///
/// [`serialized_size_fds`]: fn.serialized_size_fds.html
pub fn serialized_size<B, T: ?Sized>(ctxt: EncodingContext<B>, value: &T) -> Result<usize>
where
    B: byteorder::ByteOrder,
    T: Serialize + Type,
{
    let mut null = NullWriteSeek;

    to_writer(&mut null, ctxt, value)
}

/// Calculate the serialized size of `T` that (potentially) contains FDs.
///
/// Returns the serialized size of `T` and the number of FDs.
pub fn serialized_size_fds<B, T: ?Sized>(
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<(usize, usize)>
where
    B: byteorder::ByteOrder,
    T: Serialize + Type,
{
    let mut null = NullWriteSeek;

    let (len, fds) = to_writer_fds(&mut null, ctxt, value)?;
    Ok((len, fds.len()))
}

/// Serialize `T` to the given `writer`.
///
/// This function returns the number of bytes written to the given `writer`.
///
/// # Panics
///
/// This function will panic if the value to serialize contains file descriptors. Use
/// [`to_writer_fds`] if you'd want to potentially pass FDs.
///
/// # Examples
///
/// ```
/// use zvariant::{EncodingContext, from_slice, to_writer};
///
/// let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
/// let mut cursor = std::io::Cursor::new(vec![]);
/// to_writer(&mut cursor, ctxt, &42u32).unwrap();
/// let value: u32 = from_slice(cursor.get_ref(), ctxt).unwrap();
/// assert_eq!(value, 42);
/// ```
///
/// [`to_writer_fds`]: fn.to_writer_fds.html
pub fn to_writer<B, W, T: ?Sized>(
    writer: &mut W,
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<usize>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize + Type,
{
    let signature = T::signature();

    to_writer_for_signature(writer, ctxt, &signature, value)
}

/// Serialize `T` that (potentially) contains FDs, to the given `writer`.
///
/// This function returns the number of bytes written to the given `writer` and the file descriptor
/// vector, which needs to be transferred via an out-of-band platform specific mechanism.
pub fn to_writer_fds<B, W, T: ?Sized>(
    writer: &mut W,
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<(usize, Vec<RawFd>)>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize + Type,
{
    let signature = T::signature();

    to_writer_fds_for_signature(writer, ctxt, &signature, value)
}

/// Serialize `T` as a byte vector.
///
/// See [`from_slice`] documentation for an example of how to use this function.
///
/// # Panics
///
/// This function will panic if the value to serialize contains file descriptors. Use
/// [`to_bytes_fds`] if you'd want to potentially pass FDs.
///
/// [`to_bytes_fds`]: fn.to_bytes_fds.html
/// [`from_slice`]: fn.from_slice.html#examples
pub fn to_bytes<B, T: ?Sized>(ctxt: EncodingContext<B>, value: &T) -> Result<Vec<u8>>
where
    B: byteorder::ByteOrder,
    T: Serialize + Type,
{
    let (bytes, fds) = to_bytes_fds(ctxt, value)?;
    if !fds.is_empty() {
        panic!("can't serialize with FDs")
    }

    Ok(bytes)
}

/// Serialize `T` that (potentially) contains FDs, as a byte vector.
///
/// The returned file descriptor needs to be transferred via an out-of-band platform specific
/// mechanism.
pub fn to_bytes_fds<B, T: ?Sized>(
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<(Vec<u8>, Vec<RawFd>)>
where
    B: byteorder::ByteOrder,
    T: Serialize + Type,
{
    let mut cursor = std::io::Cursor::new(vec![]);
    let (_, fds) = to_writer_fds(&mut cursor, ctxt, value)?;
    Ok((cursor.into_inner(), fds))
}

/// Serialize `T` that has the given signature, to the given `writer`.
///
/// Use this function instead of [`to_writer`] if the value being serialized does not implement
/// [`Type`].
///
/// This function returns the number of bytes written to the given `writer`.
///
/// [`to_writer`]: fn.to_writer.html
/// [`Type`]: trait.Type.html
pub fn to_writer_for_signature<B, W, T: ?Sized>(
    writer: &mut W,
    ctxt: EncodingContext<B>,
    signature: &Signature,
    value: &T,
) -> Result<usize>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize,
{
    let (len, fds) = to_writer_fds_for_signature(writer, ctxt, signature, value)?;
    if !fds.is_empty() {
        panic!("can't serialize with FDs")
    }

    Ok(len)
}

/// Serialize `T` that (potentially) contains FDs and has the given signature, to the given `writer`.
///
/// Use this function instead of [`to_writer_fds`] if the value being serialized does not implement
/// [`Type`].
///
/// This function returns the number of bytes written to the given `writer` and the file descriptor
/// vector, which needs to be transferred via an out-of-band platform specific mechanism.
///
/// [`to_writer_fds`]: fn.to_writer_fds.html
/// [`Type`]: trait.Type.html
pub fn to_writer_fds_for_signature<B, W, T: ?Sized>(
    writer: &mut W,
    ctxt: EncodingContext<B>,
    signature: &Signature,
    value: &T,
) -> Result<(usize, Vec<RawFd>)>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize,
{
    let mut fds = vec![];
    let mut ser = Serializer::<B, W>::new(signature, writer, &mut fds, ctxt);
    value.serialize(&mut ser)?;
    Ok((ser.bytes_written, fds))
}

/// Serialize `T` that has the given signature, to a new byte vector.
///
/// Use this function instead of [`to_bytes`] if the value being serialized does not implement
/// [`Type`]. See [`from_slice_for_signature`] documentation for an example of how to use this
/// function.
///
/// # Panics
///
/// This function will panic if the value to serialize contains file descriptors. Use
/// [`to_bytes_fds_for_signature`] if you'd want to potentially pass FDs.
///
/// [`to_bytes`]: fn.to_bytes.html
/// [`Type`]: trait.Type.html
/// [`from_slice_for_signature`]: fn.from_slice_for_signature.html#examples
pub fn to_bytes_for_signature<B, T: ?Sized>(
    ctxt: EncodingContext<B>,
    signature: &Signature,
    value: &T,
) -> Result<Vec<u8>>
where
    B: byteorder::ByteOrder,
    T: Serialize,
{
    let (bytes, fds) = to_bytes_fds_for_signature(ctxt, signature, value)?;
    if !fds.is_empty() {
        panic!("can't serialize with FDs")
    }

    Ok(bytes)
}

/// Serialize `T` that (potentially) contains FDs and has the given signature, to a new byte vector.
///
/// Use this function instead of [`to_bytes_fds`] if the value being serialized does not implement
/// [`Type`].
///
/// Please note that the serialized bytes only contain the indices of the file descriptors from the
/// returned file descriptor vector, which needs to be transferred via an out-of-band platform
/// specific mechanism.
///
/// [`to_bytes_fds`]: fn.to_bytes_fds.html
/// [`Type`]: trait.Type.html
pub fn to_bytes_fds_for_signature<B, T: ?Sized>(
    ctxt: EncodingContext<B>,
    signature: &Signature,
    value: &T,
) -> Result<(Vec<u8>, Vec<RawFd>)>
where
    B: byteorder::ByteOrder,
    T: Serialize,
{
    let mut cursor = std::io::Cursor::new(vec![]);
    let (_, fds) = to_writer_fds_for_signature(&mut cursor, ctxt, signature, value)?;
    Ok((cursor.into_inner(), fds))
}

/// Our serialization implementation.
pub struct Serializer<'ser, 'sig, B, W> {
    pub(self) ctxt: EncodingContext<B>,
    pub(self) writer: &'ser mut W,
    pub(self) bytes_written: usize,
    pub(self) fds: &'ser mut Vec<RawFd>,

    pub(self) sig_parser: SignatureParser<'sig>,

    pub(self) value_sign: Option<Signature<'static>>,

    b: PhantomData<B>,
}

impl<'ser, 'sig, B, W> Serializer<'ser, 'sig, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    /// Create a Serializer struct instance.
    pub fn new<'w: 'ser, 'f: 'ser>(
        signature: &Signature<'sig>,
        writer: &'w mut W,
        fds: &'f mut Vec<RawFd>,
        ctxt: EncodingContext<B>,
    ) -> Self {
        let sig_parser = SignatureParser::new(signature.clone());

        Self {
            ctxt,
            sig_parser,
            writer,
            fds,
            bytes_written: 0,
            value_sign: None,
            b: PhantomData,
        }
    }

    /// Unwrap the `Writer` reference from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> &'ser mut W {
        self.writer
    }

    fn add_fd(&mut self, fd: RawFd) -> u32 {
        if let Some(idx) = self.fds.iter().position(|&x| x == fd) {
            return idx as u32;
        }
        let idx = self.fds.len();
        self.fds.push(fd);

        idx as u32
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
        self.sig_parser.skip_char()?;
        self.add_padding(T::alignment(self.ctxt.format()))?;

        Ok(())
    }

    fn prep_serialize_enum_variant(&mut self, variant_index: u32) -> Result<()> {
        // Encode enum variants as a struct with first field as variant index
        self.add_padding(u32::alignment(self.ctxt.format()))?;
        self.write_u32::<B>(variant_index).map_err(Error::Io)?;

        Ok(())
    }

    fn serialize_maybe<T>(&mut self, value: Option<&T>) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let signature = self.sig_parser.next_signature()?;
        let alignment = alignment_for_signature(&signature, self.ctxt.format());
        let child_sig_parser = self.sig_parser.slice(1..);
        let child_signature = child_sig_parser.next_signature()?;
        let child_sig_len = child_signature.len();
        let fixed_sized_child = crate::utils::is_fixed_sized_signature(&child_signature)?;

        match self.ctxt.format() {
            EncodingFormat::GVariant => {
                self.sig_parser.skip_char()?;

                self.add_padding(alignment)?;

                match value {
                    Some(value) => {
                        value.serialize(&mut *self)?;

                        if !fixed_sized_child {
                            self.write_all(&b"\0"[..]).map_err(Error::Io)?;
                        }
                    }
                    None => {
                        self.sig_parser.skip_chars(child_sig_len)?;
                    }
                }

                Ok(())
            }
            EncodingFormat::DBus => Err(Error::IncompatibleFormat(
                signature.to_owned(),
                self.ctxt.format(),
            )),
        }
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
        self.writer.write(buf).map(|n| {
            self.bytes_written += n;

            n
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
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
        self.serialize_i16(v as i16)
    }

    // TODO: Use macro to avoid code-duplication here
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.prep_serialize_basic::<i16>()?;
        self.write_i16::<B>(v).map_err(Error::Io)
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        match self.sig_parser.next_char() {
            'h' => {
                self.sig_parser.skip_char()?;
                self.add_padding(u32::alignment(self.ctxt.format()))?;
                let v = self.add_fd(v);
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
        let c = self.sig_parser.next_char();
        if c == VARIANT_SIGNATURE_CHAR {
            self.value_sign = Some(signature_string!(v));

            if self.ctxt.format() == EncodingFormat::GVariant {
                // signature is serialized after the value in GVariant
                return Ok(());
            }
        }

        // Strings in GVariant format require no alignment or
        if self.ctxt.format() == EncodingFormat::DBus {
            match c {
                ObjectPath::SIGNATURE_CHAR | <&str>::SIGNATURE_CHAR => {
                    self.add_padding(<&str>::alignment(self.ctxt.format()))?;
                    self.write_u32::<B>(usize_to_u32(v.len()))
                        .map_err(Error::Io)?;
                }
                Signature::SIGNATURE_CHAR | VARIANT_SIGNATURE_CHAR => {
                    self.write_u8(usize_to_u8(v.len())).map_err(Error::Io)?;
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
        }
        self.sig_parser.skip_char()?;
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
        self.serialize_maybe::<()>(None)
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_maybe(Some(value))
    }

    fn serialize_unit(self) -> Result<()> {
        match self.ctxt.format() {
            EncodingFormat::GVariant => self.write_all(&b"\0"[..]).map_err(Error::Io),
            EncodingFormat::DBus => Ok(()),
        }
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
        self.sig_parser.skip_char()?;
        if let EncodingFormat::DBus = self.ctxt.format() {
            self.add_padding(ARRAY_ALIGNMENT_DBUS)?;
            // Length in bytes (unfortunately not the same as len passed to us here) which we
            // initially set to 0.
            self.write_u32::<B>(0_u32).map_err(Error::Io)?;
        }

        let element_signature = self.sig_parser.next_signature()?;
        let element_signature_len = element_signature.len();
        let element_alignment = alignment_for_signature(&element_signature, self.ctxt.format());
        let (offsets, key_start) = match self.ctxt.format() {
            EncodingFormat::GVariant => {
                let fixed_sized_child = crate::utils::is_fixed_sized_signature(&element_signature)?;
                let offsets = if !fixed_sized_child {
                    Some(FramingOffsets::new())
                } else {
                    None
                };

                let key_start = if self.sig_parser.next_char() == DICT_ENTRY_SIG_START_CHAR {
                    let key_signature = Signature::from_str_unchecked(&element_signature[1..2]);
                    if !crate::utils::is_fixed_sized_signature(&key_signature)? {
                        Some(0)
                    } else {
                        None
                    }
                } else {
                    None
                };

                (offsets, key_start)
            }
            _ => (None, None),
        };
        // D-Bus expects us to add padding for the first element even when there is no first
        // element (i-e empty array) so we add padding already. In case of GVariant this is just
        // the padding of the array itself since array starts with first element.
        let first_padding = self.add_padding(element_alignment)?;
        let start = self.bytes_written;

        Ok(SeqSerializer {
            ser: self,
            start,
            element_alignment,
            element_signature_len,
            first_padding,
            offsets,
            key_start,
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
        let c = self.sig_parser.next_char();
        let end_parens;
        if c == VARIANT_SIGNATURE_CHAR {
            let alignment = match self.ctxt.format() {
                EncodingFormat::DBus => VARIANT_ALIGNMENT_DBUS,
                EncodingFormat::GVariant => VARIANT_ALIGNMENT_GVARIANT,
            };
            self.add_padding(alignment)?;
            end_parens = false;
        } else {
            let signature = self.sig_parser.next_signature()?;
            let alignment = alignment_for_signature(&signature, self.ctxt.format());
            self.add_padding(alignment)?;

            self.sig_parser.skip_char()?;

            if c == STRUCT_SIG_START_CHAR || c == DICT_ENTRY_SIG_START_CHAR {
                end_parens = true;
            } else {
                let expected = format!(
                    "`{}` or `{}`",
                    STRUCT_SIG_START_STR, DICT_ENTRY_SIG_START_STR,
                );
                return Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Char(c),
                    &expected.as_str(),
                ));
            }
        }

        let offsets =
            if c == STRUCT_SIG_START_CHAR && self.ctxt.format() == EncodingFormat::GVariant {
                Some(FramingOffsets::new())
            } else {
                None
            };
        let start = self.bytes_written;

        Ok(StructSerializer {
            ser: self,
            start,
            end_parens,
            offsets,
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
    ser: &'b mut Serializer<'ser, 'sig, B, W>,
    start: usize,
    // alignement of element
    element_alignment: usize,
    // size of element signature
    element_signature_len: usize,
    // First element's padding
    first_padding: usize,
    // FIXME: Best to create a separate struct for all of these GVariant-specific fields.
    // All offsets (GVariant-specific)
    offsets: Option<FramingOffsets>,
    // start of last dict-entry key written (GVariant-specific)
    key_start: Option<usize>,
}

impl<'ser, 'sig, 'b, B, W> SeqSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    pub(self) fn end_seq(self) -> Result<()> {
        self.ser.sig_parser.skip_chars(self.element_signature_len)?;

        match self.ser.ctxt.format() {
            EncodingFormat::DBus => self.end_dbus_seq(),
            EncodingFormat::GVariant => self.end_gvariant_seq(),
        }
    }

    fn end_dbus_seq(self) -> Result<()> {
        // Set size of array in bytes
        let array_len = self.ser.bytes_written - self.start;
        let len = usize_to_u32(array_len);
        let total_array_len = (array_len + self.first_padding + 4) as i64;
        self.ser
            .writer
            .seek(std::io::SeekFrom::Current(-total_array_len))
            .map_err(Error::Io)?;
        self.ser.writer.write_u32::<B>(len).map_err(Error::Io)?;
        self.ser
            .writer
            .seek(std::io::SeekFrom::Current(total_array_len - 4))
            .map_err(Error::Io)?;

        Ok(())
    }

    fn end_gvariant_seq(self) -> Result<()> {
        let offsets = match self.offsets {
            Some(offsets) => offsets,
            None => return Ok(()),
        };
        let array_len = self.ser.bytes_written - self.start;
        if array_len == 0 {
            // Empty sequence
            assert!(offsets.is_empty());

            return Ok(());
        }

        offsets.write_all(self.ser, array_len)?;

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
        // We want to keep parsing the same signature repeatedly for each element so we use a
        // disposable clone.
        let sig_parser = self.ser.sig_parser.clone();
        self.ser.sig_parser = sig_parser.clone();

        value.serialize(&mut *self.ser)?;
        self.ser.sig_parser = sig_parser;

        if let Some(ref mut offsets) = self.offsets {
            assert_eq!(self.ser.ctxt.format(), EncodingFormat::GVariant);
            let offset = self.ser.bytes_written - self.start;

            offsets.push(offset);
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

#[doc(hidden)]
pub struct StructSerializer<'ser, 'sig, 'b, B, W> {
    ser: &'b mut Serializer<'ser, 'sig, B, W>,
    start: usize,
    end_parens: bool,
    // FIXME: Best to create a separate struct for all of these GVariant-specific fields.
    // All offsets (GVariant-specific)
    offsets: Option<FramingOffsets>,
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
                    .ser
                    .value_sign
                    .take()
                    .expect("Incorrect Value encoding");

                let sig_parser = SignatureParser::new(signature.clone());
                let mut ser = Serializer::<B, W> {
                    ctxt: self.ser.ctxt,
                    sig_parser,
                    writer: &mut self.ser.writer,
                    fds: self.ser.fds,
                    bytes_written: self.ser.bytes_written,
                    value_sign: None,
                    b: PhantomData,
                };
                value.serialize(&mut ser)?;
                self.ser.bytes_written = ser.bytes_written;

                if self.ser.ctxt.format() == EncodingFormat::GVariant {
                    self.ser.write_all(&b"\0"[..]).map_err(Error::Io)?;
                    self.ser
                        .write_all(&signature.as_bytes())
                        .map_err(Error::Io)?;
                }

                Ok(())
            }
            _ => {
                let fixed_sized_element = if self.ser.ctxt.format() == EncodingFormat::GVariant {
                    let element_signature = self.ser.sig_parser.next_signature()?;

                    crate::utils::is_fixed_sized_signature(&element_signature)?
                } else {
                    true
                };

                value.serialize(&mut *self.ser)?;

                if let Some(ref mut offsets) = self.offsets {
                    assert_eq!(self.ser.ctxt.format(), EncodingFormat::GVariant);
                    if !fixed_sized_element {
                        offsets.push_front(self.ser.bytes_written - self.start);
                    }
                }

                Ok(())
            }
        }
    }

    fn end_struct(self) -> Result<()> {
        if self.end_parens {
            self.ser.sig_parser.skip_char()?;
        }
        let mut offsets = match self.offsets {
            Some(offsets) => offsets,
            None => return Ok(()),
        };
        let struct_len = self.ser.bytes_written - self.start;
        if struct_len == 0 {
            // Empty sequence
            assert!(offsets.is_empty());

            return Ok(());
        }
        if offsets.peek() == Some(struct_len) {
            // For structs, we don't want offset of last element
            offsets.pop();
        }

        offsets.write_all(self.ser, struct_len)?;

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
        self.ser.add_padding(self.element_alignment)?;

        if self.key_start.is_some() {
            self.key_start.replace(self.ser.bytes_written);
        }

        // We want to keep parsing the same signature repeatedly for each key so we use a
        // disposable clone.
        let sig_parser = self.ser.sig_parser.clone();
        self.ser.sig_parser = sig_parser.clone();

        // skip `{`
        self.ser.sig_parser.skip_char()?;

        key.serialize(&mut *self.ser)?;
        self.ser.sig_parser = sig_parser;

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key_offset = match self.ser.ctxt.format() {
            EncodingFormat::DBus => None,
            EncodingFormat::GVariant => {
                // For non-fixed-sized keys, we must add the key offset after the value
                self.key_start.map(|start| self.ser.bytes_written - start)
            }
        };

        // We want to keep parsing the same signature repeatedly for each key so we use a
        // disposable clone.
        let sig_parser = self.ser.sig_parser.clone();
        self.ser.sig_parser = sig_parser.clone();

        // skip `{` and key char
        self.ser.sig_parser.skip_chars(2)?;

        value.serialize(&mut *self.ser)?;
        // Restore the original parser
        self.ser.sig_parser = sig_parser;

        if let Some(key_offset) = key_offset {
            let entry_size = self.ser.bytes_written - self.key_start.unwrap_or(0);
            let offset_size = FramingOffsetSize::for_encoded_container(entry_size);
            offset_size.write_offset(&mut *self.ser, key_offset)?;
        }

        // And now the offset of the array element end (which is encoded later)
        if let Some(ref mut offsets) = self.offsets {
            let offset = self.ser.bytes_written - self.start;

            offsets.push(offset);
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
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
