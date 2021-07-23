use byteorder::WriteBytesExt;
use serde::{ser, ser::SerializeMap, ser::SerializeSeq, Serialize};
use static_assertions::assert_impl_all;
use std::{
    io::{Seek, Write},
    marker::PhantomData,
    mem::swap,
    os::unix::io::RawFd,
    str,
};

use crate::{
    signature_parser::SignatureParser, utils::*, Basic, EncodingContext, EncodingFormat, Error,
    ObjectPath, Result, Signature,
};

/// Our D-Bus serialization implementation.
pub struct Serializer<'ser, 'sig, B, W>(pub(crate) crate::SerializerCommon<'ser, 'sig, B, W>);

assert_impl_all!(Serializer<'_, '_, i32, i32>: Send, Sync, Unpin);

impl<'ser, 'sig, B, W> Serializer<'ser, 'sig, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    /// Create a D-Bus Serializer struct instance.
    pub fn new<'w: 'ser, 'f: 'ser>(
        signature: &Signature<'sig>,
        writer: &'w mut W,
        fds: &'f mut Vec<RawFd>,
        ctxt: EncodingContext<B>,
    ) -> Self {
        assert_eq!(ctxt.format(), EncodingFormat::DBus);

        let sig_parser = SignatureParser::new(signature.clone());
        log::trace!("Creating serializer");
        sig_parser.log_current();
        Self(crate::SerializerCommon {
            ctxt,
            sig_parser,
            writer,
            fds,
            bytes_written: 0,
            value_sign: None,
            b: PhantomData,
        })
    }

    fn serialize_variant_body<'a, T>(&mut self, signature: Signature<'a>, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let sig_parser = SignatureParser::new(signature);
        let bytes_written = self.0.bytes_written;
        let mut fds = vec![];
        let mut ser = Serializer(crate::SerializerCommon::<B, W> {
            ctxt: self.0.ctxt,
            sig_parser,
            writer: &mut self.0.writer,
            fds: &mut fds,
            bytes_written,
            value_sign: None,
            b: PhantomData,
        });
        value.serialize(&mut ser)?;
        self.0.bytes_written = ser.0.bytes_written;
        self.0.fds.extend(fds.iter());
        Ok(())
    }
}

macro_rules! serialize_basic {
    ($method:ident($type:ty) $write_method:ident) => {
        serialize_basic!($method($type) $write_method($type));
    };
    ($method:ident($type:ty) $write_method:ident($as:ty)) => {
        fn $method(self, v: $type) -> Result<()> {
            self.0.prep_serialize_basic::<$type>()?;
            self.0.$write_method::<B>(v as $as).map_err(Error::Io)
        }
    };
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

    serialize_basic!(serialize_bool(bool) write_u32(u32));
    // No i8 type in D-Bus/GVariant, let's pretend it's i16
    serialize_basic!(serialize_i8(i8) write_i16(i16));
    serialize_basic!(serialize_i16(i16) write_i16);
    serialize_basic!(serialize_i64(i64) write_i64);

    fn serialize_i32(self, v: i32) -> Result<()> {
        match self.0.sig_parser.next_char() {
            'h' => {
                self.0.sig_parser.skip_char()?;
                self.0.add_padding(u32::alignment(EncodingFormat::DBus))?;
                let v = self.0.add_fd(v);
                self.0.write_u32::<B>(v).map_err(Error::Io)
            }
            _ => {
                self.0.prep_serialize_basic::<i32>()?;
                self.0.write_i32::<B>(v).map_err(Error::Io)
            }
        }
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.0.prep_serialize_basic::<u8>()?;
        // Endianness is irrelevant for single bytes.
        self.0.write_u8(v).map_err(Error::Io)
    }

    serialize_basic!(serialize_u16(u16) write_u16);
    serialize_basic!(serialize_u32(u32) write_u32);
    serialize_basic!(serialize_u64(u64) write_u64);
    // No f32 type in D-Bus/GVariant, let's pretend it's f64
    serialize_basic!(serialize_f32(f32) write_f64(f64));
    serialize_basic!(serialize_f64(f64) write_f64);

    fn serialize_char(self, v: char) -> Result<()> {
        // No char type in D-Bus, let's pretend it's a string
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        if v.contains('\0') {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Char('\0'),
                &"D-Bus string type must not contain interior null bytes",
            ));
        }
        let c = self.0.sig_parser.next_char();
        if c == VARIANT_SIGNATURE_CHAR {
            self.0.value_sign = Some(signature_string!(v));
        }

        match c {
            ObjectPath::SIGNATURE_CHAR | <&str>::SIGNATURE_CHAR => {
                self.0
                    .add_padding(<&str>::alignment(EncodingFormat::DBus))?;
                self.0
                    .write_u32::<B>(usize_to_u32(v.len()))
                    .map_err(Error::Io)?;
            }
            Signature::SIGNATURE_CHAR | VARIANT_SIGNATURE_CHAR => {
                // XXX Replace <> with 'a{sv}' respecting nesting
                self.0.write_u8(usize_to_u8(v.len())).map_err(Error::Io)?;
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

        self.0.sig_parser.skip_char()?;
        self.0.write_all(v.as_bytes()).map_err(Error::Io)?;
        self.0.write_all(&b"\0"[..]).map_err(Error::Io)?;

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        let seq = self.serialize_seq(Some(v.len()))?;
        seq.ser.0.write(v).map_err(Error::Io)?;
        SerializeSeq::end(seq)
    }

    fn serialize_none(self) -> Result<()> {
        unreachable!("Option<T> can not be encoded in D-Bus format");
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!("Option<T> can not be encoded in D-Bus format");
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
        self.0.prep_serialize_enum_variant(variant_index)?;

        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.0.sig_parser.skip_char()?;
        self.0.add_padding(ARRAY_ALIGNMENT_DBUS)?;
        // Length in bytes (unfortunately not the same as len passed to us here) which we
        // initially set to 0.
        self.0.write_u32::<B>(0_u32).map_err(Error::Io)?;

        let element_signature = self.0.sig_parser.next_signature()?;
        let element_signature_len = element_signature.len();
        let element_alignment = alignment_for_signature(&element_signature, self.0.ctxt.format());

        // D-Bus expects us to add padding for the first element even when there is no first
        // element (i-e empty array) so we add padding already.
        let first_padding = self.0.add_padding(element_alignment)?;
        let start = self.0.bytes_written;

        Ok(SeqSerializer {
            ser: self,
            start,
            element_alignment,
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
        self.0.prep_serialize_enum_variant(variant_index)?;

        self.serialize_struct(name, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.serialize_seq(len)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        log::trace!("serialize_struct");
        self.0.sig_parser.log_current();
        let c = self.0.sig_parser.next_char();
        log::trace!("Got signature char: {}", c);
        match c {
            VARIANT_SIGNATURE_CHAR  => {
                self.0.add_padding(VARIANT_ALIGNMENT_DBUS)?;
                Ok(StructSerializer::Variant(VariantStructSerializer {
                    ser: self,
                }))
            },
            STRUCT_SIG_START_CHAR | DICT_ENTRY_SIG_START_CHAR => {
                let signature = self.0.sig_parser.next_signature()?;
                let alignment = alignment_for_signature(&signature, EncodingFormat::DBus);
                self.0.add_padding(alignment)?;

                self.0.sig_parser.skip_char()?;

                Ok(StructSerializer::Default(DefaultStructSerializer {
                    ser: self,
                }))
            },
            SERIALIZE_DICT_SIG_START_CHAR => {
                // This is going to be a map, signature 'a{sv}'
                // The signatures of the actual elements will be stored in the signature
                self.0.sig_parser.skip_char()?; // Go past the initial '<'
                self.0.add_padding(ARRAY_ALIGNMENT_DBUS)?;
                self.0.write_u32::<B>(0_u32).map_err(Error::Io)?;
                let element_signature = signature_string!("{sv}");
                let element_signature_len = element_signature.len();
                let element_alignment = alignment_for_signature(&element_signature, self.0.ctxt.format());

                // D-Bus expects us to add padding for the first element even when there is no first
                // element (i-e empty array) so we add padding already.
                let first_padding = self.0.add_padding(element_alignment)?;
                let start = self.0.bytes_written;

                Ok(StructSerializer::VariantDict(VariantDictStructSerializer {
                    seq_serializer: SeqSerializer {
                        ser: self,
                        start,
                        element_alignment,
                        element_signature_len,
                        first_padding,
                    },
                    count: 0,
                }))
            },
            _ => {
                log::error!("Do not recognize signature char: {}", c);
                let expected = format!(
                    "`{}` or `{}`",
                    STRUCT_SIG_START_STR, DICT_ENTRY_SIG_START_STR,
                );
                Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Char(c),
                    &expected.as_str(),
                ))
            },
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.0.prep_serialize_enum_variant(variant_index)?;

        self.serialize_struct(name, len)
    }
}

#[doc(hidden)]
pub struct SeqSerializer<'ser, 'sig, 'b, B, W> {
    ser: &'b mut Serializer<'ser, 'sig, B, W>,
    start: usize,
    // alignment of element
    element_alignment: usize,
    // size of element signature
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
        self.ser
            .0
            .sig_parser
            .skip_chars(self.element_signature_len)?;

        // Set size of array in bytes
        let array_len = self.ser.0.bytes_written - self.start;
        let len = usize_to_u32(array_len);
        let total_array_len = (array_len + self.first_padding + 4) as i64;
        self.ser
            .0
            .writer
            .seek(std::io::SeekFrom::Current(-total_array_len))
            .map_err(Error::Io)?;
        self.ser.0.writer.write_u32::<B>(len).map_err(Error::Io)?;
        self.ser
            .0
            .writer
            .seek(std::io::SeekFrom::Current(total_array_len - 4))
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
        // We want to keep parsing the same signature repeatedly for each element so we use a
        // disposable clone.
        let sig_parser = self.ser.0.sig_parser.clone();
        self.ser.0.sig_parser = sig_parser.clone();

        value.serialize(&mut *self.ser)?;
        self.ser.0.sig_parser = sig_parser;

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

#[doc(hidden)]
pub enum StructSerializer<'ser, 'sig, 'b, B, W> {
    Default(DefaultStructSerializer<'ser, 'sig, 'b, B, W>),
    Variant(VariantStructSerializer<'ser, 'sig, 'b, B, W>),
    VariantDict(VariantDictStructSerializer<'ser, 'sig, 'b, B, W>),
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
        match self {
            StructSerializer::Default(ref mut dss) => dss.serialize_struct_element::<T>(name, value),
            StructSerializer::Variant(ref mut vss) => vss.serialize_struct_element::<T>(name, value),
            StructSerializer::VariantDict(ref mut vdss) => vdss.serialize_struct_element::<T>(name, value),
        }
    }

    fn end_struct(self) -> Result<()> {
        match self {
            StructSerializer::Default(dss) => dss.end_struct(),
            StructSerializer::Variant(vss) => vss.end_struct(),
            StructSerializer::VariantDict(vdss) => vdss.end_struct(),
        }
    }
}

#[doc(hidden)]
pub struct DefaultStructSerializer<'ser, 'sig, 'b, B, W> {
    ser: &'b mut Serializer<'ser, 'sig, B, W>,
}

impl<'ser, 'sig, 'b, B, W> DefaultStructSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    fn serialize_struct_element<T>(&mut self, _name: Option<&'static str>, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end_struct(self) -> Result<()> {
        self.ser.0.sig_parser.skip_char()?;

        Ok(())
    }
}

#[doc(hidden)]
pub struct VariantStructSerializer<'ser, 'sig, 'b, B, W> {
    ser: &'b mut Serializer<'ser, 'sig, B, W>,
}

impl<'ser, 'sig, 'b, B, W> VariantStructSerializer<'ser, 'sig, 'b, B, W>
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
                    .0
                    .value_sign
                    .take()
                    .expect("Incorrect Value encoding");

                self.ser.serialize_variant_body(signature, value)?;

                Ok(())
            }
            _ => value.serialize(&mut *self.ser),
        }
    }

    fn end_struct(self) -> Result<()> {
        Ok(())
    }
}

#[doc(hidden)]
pub struct VariantDictStructSerializer<'ser, 'sig, 'b, B, W> {
    seq_serializer: SeqSerializer<'ser, 'sig, 'b, B, W>,
    count: usize,
}

impl<'ser, 'sig, 'b, B, W> VariantDictStructSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    // XXX Make these into methods of seq serializer
    fn serialize_struct_element<T>(&mut self, name: Option<&'static str>, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let name = name.map(|x| x.to_string()).unwrap_or_else(|| format!("{}", self.count));
        self.count += 1;

        let mut sig_parser = SignatureParser::new(signature_string!("{sv}"));
        swap(&mut self.seq_serializer.ser.0.sig_parser, &mut sig_parser);

        self.seq_serializer.serialize_key(&name)?;

        // Get value signature
        let item_signature = sig_parser.next_signature()?.clone();

        // Serialize value (T) as variant with signature item_signature

        // skip `{` and key char (at 'v')
        self.seq_serializer.ser.0.sig_parser.skip_chars(2)?;
        item_signature.serialize(&mut *self.seq_serializer.ser)?;

        self.seq_serializer.ser.serialize_variant_body(item_signature, value)?;

        // Restore original signature
        swap(&mut self.seq_serializer.ser.0.sig_parser, &mut sig_parser);

        Ok(())
    }

    fn end_struct(mut self) -> Result<()> {
        self.seq_serializer.element_signature_len = 1; // closing '>'
        self.seq_serializer.end_seq()?;
        Ok(())
    }
}

macro_rules! serialize_struct_anon_fields {
    ($trait:ident $method:ident) => {
        impl<'ser, 'sig, 'b, B, W> ser::$trait for StructSerializer<'ser, 'sig, 'b, B, W>
        where
            B: byteorder::ByteOrder,
            W: Write + Seek,
        {
            type Ok = ();
            type Error = Error;

            fn $method<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                self.serialize_struct_element(None, value)
            }

            fn end(self) -> Result<()> {
                self.end_struct()
            }
        }
    };
}
serialize_struct_anon_fields!(SerializeTuple serialize_element);
serialize_struct_anon_fields!(SerializeTupleStruct serialize_field);
serialize_struct_anon_fields!(SerializeTupleVariant serialize_field);

impl<'ser, 'sig, 'b, B, W> ser::SerializeMap for SeqSerializer<'ser, 'sig, 'b, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.0.add_padding(self.element_alignment)?;

        // We want to keep parsing the same signature repeatedly for each key so we use a
        // disposable clone.
        let sig_parser = self.ser.0.sig_parser.clone();
        self.ser.0.sig_parser = sig_parser.clone();

        // skip `{`
        self.ser.0.sig_parser.skip_char()?;

        key.serialize(&mut *self.ser)?;
        self.ser.0.sig_parser = sig_parser;

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // We want to keep parsing the same signature repeatedly for each key so we use a
        // disposable clone.
        let sig_parser = self.ser.0.sig_parser.clone();
        self.ser.0.sig_parser = sig_parser.clone();

        // skip `{` and key char
        self.ser.0.sig_parser.skip_chars(2)?;

        value.serialize(&mut *self.ser)?;
        // Restore the original parser
        self.ser.0.sig_parser = sig_parser;

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

macro_rules! serialize_struct_named_fields {
    ($trait:ident) => {
        impl<'ser, 'sig, 'b, B, W> ser::$trait for StructSerializer<'ser, 'sig, 'b, B, W>
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
    };
}
serialize_struct_named_fields!(SerializeStruct);
serialize_struct_named_fields!(SerializeStructVariant);
