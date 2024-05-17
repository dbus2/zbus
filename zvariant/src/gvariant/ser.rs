use endi::WriteBytes;
use serde::{ser, ser::SerializeSeq, Serialize};
use static_assertions::assert_impl_all;
use std::{
    io::{Seek, Write},
    str,
};

use crate::{
    container_depths::ContainerDepths,
    framing_offset_size::FramingOffsetSize,
    framing_offsets::FramingOffsets,
    serialized::{Context, Format},
    signature_parser::SignatureParser,
    utils::*,
    Basic, Error, Fd, Result, Signature,
};

/// Our serialization implementation.
pub(crate) struct Serializer<'ser, 'sig, W> {
    sig_parser: SignatureParser<'sig>,
    value_sign: Option<Signature<'sig>>,
    container_depths: ContainerDepths,
    pub(crate) common: crate::SerializerCommon<'ser, W>,
}

assert_impl_all!(Serializer<'_, '_, i32>: Send, Sync, Unpin);

impl<'ser, 'sig, W> Serializer<'ser, 'sig, W>
where
    W: Write + Seek,
{
    /// Create a GVariant Serializer struct instance.
    ///
    /// On Windows, the method doesn't have `fds` argument.
    pub fn new<'w: 'ser, 'f: 'ser, S>(
        signature: S,
        writer: &'w mut W,
        #[cfg(unix)] fds: &'f mut crate::ser::FdList,
        ctxt: Context,
    ) -> Result<Self>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
    {
        assert_eq!(ctxt.format(), Format::GVariant);

        let signature = signature.try_into().map_err(Into::into)?;
        let sig_parser = SignatureParser::new(signature);
        Ok(Self {
            sig_parser,
            value_sign: None,
            container_depths: Default::default(),
            common: crate::SerializerCommon {
                ctxt,
                writer,
                #[cfg(unix)]
                fds,
                bytes_written: 0,
            },
        })
    }

    #[cfg(not(feature = "option-as-array"))]
    fn serialize_maybe<T>(&mut self, value: Option<&T>) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let signature = self.sig_parser.next_signature()?;
        let alignment = alignment_for_signature(&signature, self.common.ctxt.format())?;
        let child_sig_parser = self.sig_parser.slice(1..);
        let child_signature = child_sig_parser.next_signature()?;
        let child_sig_len = child_signature.len();
        let fixed_sized_child = crate::utils::is_fixed_sized_signature(&child_signature)?;

        self.sig_parser.skip_char()?;

        self.common.add_padding(alignment)?;

        match value {
            Some(value) => {
                self.container_depths = self.container_depths.inc_maybe()?;
                value.serialize(&mut *self)?;
                self.container_depths = self.container_depths.dec_maybe();

                if !fixed_sized_child {
                    self.common
                        .write_all(&b"\0"[..])
                        .map_err(|e| Error::InputOutput(e.into()))?;
                }
            }
            None => {
                self.sig_parser.skip_chars(child_sig_len)?;
            }
        }

        Ok(())
    }

    fn prep_serialize_basic<T>(&mut self) -> Result<()>
    where
        T: Basic,
    {
        self.sig_parser.skip_char()?;
        self.common
            .add_padding(T::alignment(self.common.ctxt.format()))?;

        Ok(())
    }

    /// This starts the enum serialization.
    ///
    /// It's up to the caller to do the rest: serialize the variant payload and skip the `).
    fn prep_serialize_enum_variant(&mut self, variant_index: u32) -> Result<()> {
        // Encode enum variants as a struct with first field as variant index
        let signature = self.sig_parser.next_signature()?;
        if self.sig_parser.next_char()? != STRUCT_SIG_START_CHAR {
            return Err(Error::SignatureMismatch(
                signature.to_owned(),
                format!("expected `{STRUCT_SIG_START_CHAR}`"),
            ));
        }

        let alignment = alignment_for_signature(&signature, self.common.ctxt.format())?;
        self.common.add_padding(alignment)?;

        // Now serialize the veriant index.
        self.common
            .write_u32(self.common.ctxt.endian(), variant_index)
            .map_err(|e| Error::InputOutput(e.into()))?;

        // Skip the `(`, `u`.
        self.sig_parser.skip_chars(2)?;

        Ok(())
    }
}

macro_rules! serialize_basic {
    ($method:ident($type:ty) $write_method:ident) => {
        serialize_basic!($method($type) $write_method($type));
    };
    ($method:ident($type:ty) $write_method:ident($as:ty)) => {
        fn $method(self, v: $type) -> Result<()> {
            self.prep_serialize_basic::<$type>()?;
            self.common.$write_method(self.common.ctxt.endian(), v as $as).map_err(|e| Error::InputOutput(e.into()))
        }
    };
}

impl<'ser, 'sig, 'b, W> ser::Serializer for &'b mut Serializer<'ser, 'sig, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'ser, 'sig, 'b, W>;
    type SerializeTuple = StructSeqSerializer<'ser, 'sig, 'b, W>;
    type SerializeTupleStruct = StructSeqSerializer<'ser, 'sig, 'b, W>;
    type SerializeTupleVariant = StructSeqSerializer<'ser, 'sig, 'b, W>;
    type SerializeMap = SeqSerializer<'ser, 'sig, 'b, W>;
    type SerializeStruct = StructSeqSerializer<'ser, 'sig, 'b, W>;
    type SerializeStructVariant = StructSeqSerializer<'ser, 'sig, 'b, W>;

    serialize_basic!(serialize_bool(bool) write_u32(u32));
    // No i8 type in D-Bus/GVariant, let's pretend it's i16
    serialize_basic!(serialize_i8(i8) write_i16(i16));
    serialize_basic!(serialize_i16(i16) write_i16);
    serialize_basic!(serialize_i64(i64) write_i64);

    fn serialize_i32(self, v: i32) -> Result<()> {
        match self.sig_parser.next_char()? {
            #[cfg(unix)]
            Fd::SIGNATURE_CHAR => {
                self.sig_parser.skip_char()?;
                self.common.add_padding(u32::alignment(Format::DBus))?;
                let idx = self.common.add_fd(v)?;
                self.common
                    .write_u32(self.common.ctxt.endian(), idx)
                    .map_err(|e| Error::InputOutput(e.into()))
            }
            _ => {
                self.prep_serialize_basic::<i32>()?;
                self.common
                    .write_i32(self.common.ctxt.endian(), v)
                    .map_err(|e| Error::InputOutput(e.into()))
            }
        }
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.prep_serialize_basic::<u8>()?;
        // Endianness is irrelevant for single bytes.
        self.common
            .write_u8(self.common.ctxt.endian(), v)
            .map_err(|e| Error::InputOutput(e.into()))
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
                &"GVariant string type must not contain interior null bytes",
            ));
        }

        let c = self.sig_parser.next_char()?;
        if c == VARIANT_SIGNATURE_CHAR {
            self.value_sign = Some(signature_string!(v));

            // signature is serialized after the value in GVariant
            return Ok(());
        }

        // Strings in GVariant format require no alignment.

        self.sig_parser.skip_char()?;
        self.common
            .write_all(v.as_bytes())
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.common
            .write_all(&b"\0"[..])
            .map_err(|e| Error::InputOutput(e.into()))?;

        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        let seq = self.serialize_seq(Some(v.len()))?;
        seq.ser
            .common
            .write(v)
            .map_err(|e| Error::InputOutput(e.into()))?;
        seq.end()
    }

    #[cfg(not(feature = "option-as-array"))]
    fn serialize_none(self) -> Result<()> {
        self.serialize_maybe::<()>(None)
    }

    #[cfg(feature = "option-as-array")]
    fn serialize_none(self) -> Result<()> {
        panic!("`option-as-array` and `gvariant` features are incompatible. Don't enable both.");
    }

    #[cfg(not(feature = "option-as-array"))]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_maybe(Some(value))
    }

    #[cfg(feature = "option-as-array")]
    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        panic!("`option-as-array` and `gvariant` features are incompatible. Don't enable both.");
    }

    fn serialize_unit(self) -> Result<()> {
        self.common
            .write_all(&b"\0"[..])
            .map_err(|e| Error::InputOutput(e.into()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        if self.sig_parser.next_char()? == <&str>::SIGNATURE_CHAR {
            variant.serialize(self)
        } else {
            variant_index.serialize(self)
        }
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
        let element_signature = self.sig_parser.next_signature()?;
        let element_signature_len = element_signature.len();
        let element_alignment =
            alignment_for_signature(&element_signature, self.common.ctxt.format())?;

        let fixed_sized_child = crate::utils::is_fixed_sized_signature(&element_signature)?;
        let offsets = (!fixed_sized_child).then(FramingOffsets::new);

        let key_start = if self.sig_parser.next_char()? == DICT_ENTRY_SIG_START_CHAR {
            let key_signature = Signature::from_str_unchecked(&element_signature[1..2]);
            (!crate::utils::is_fixed_sized_signature(&key_signature)?).then_some(0)
        } else {
            None
        };
        self.common.add_padding(element_alignment)?;
        self.container_depths = self.container_depths.inc_array()?;

        let start = self.common.bytes_written;

        Ok(SeqSerializer {
            ser: self,
            start,
            element_alignment,
            element_signature_len,
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
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.prep_serialize_enum_variant(variant_index)?;

        StructSerializer::enum_variant(self).map(StructSeqSerializer::Struct)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.serialize_seq(len)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        if len == 0 {
            return StructSerializer::unit(self).map(StructSeqSerializer::Struct);
        }

        match self.sig_parser.next_char()? {
            VARIANT_SIGNATURE_CHAR => {
                StructSerializer::variant(self).map(StructSeqSerializer::Struct)
            }
            ARRAY_SIGNATURE_CHAR => self.serialize_seq(Some(len)).map(StructSeqSerializer::Seq),
            _ => StructSerializer::structure(self).map(StructSeqSerializer::Struct),
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.prep_serialize_enum_variant(variant_index)?;

        StructSerializer::enum_variant(self).map(StructSeqSerializer::Struct)
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

#[doc(hidden)]
pub struct SeqSerializer<'ser, 'sig, 'b, W> {
    ser: &'b mut Serializer<'ser, 'sig, W>,
    start: usize,
    // alignment of element
    element_alignment: usize,
    // size of element signature
    element_signature_len: usize,
    // All offsets
    offsets: Option<FramingOffsets>,
    // start of last dict-entry key written
    key_start: Option<usize>,
}

impl<'ser, 'sig, 'b, W> SeqSerializer<'ser, 'sig, 'b, W>
where
    W: Write + Seek,
{
    pub(self) fn end_seq(self) -> Result<()> {
        self.ser.sig_parser.skip_chars(self.element_signature_len)?;
        self.ser.container_depths = self.ser.container_depths.dec_array();

        let offsets = match self.offsets {
            Some(offsets) => offsets,
            None => return Ok(()),
        };
        let array_len = self.ser.common.bytes_written - self.start;
        if array_len == 0 {
            // Empty sequence
            return Ok(());
        }

        offsets.write_all(&mut self.ser.common, array_len)?;

        Ok(())
    }
}

impl<'ser, 'sig, 'b, W> ser::SerializeSeq for SeqSerializer<'ser, 'sig, 'b, W>
where
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
            let offset = self.ser.common.bytes_written - self.start;

            offsets.push(offset);
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

#[doc(hidden)]
pub struct StructSerializer<'ser, 'sig, 'b, W> {
    ser: &'b mut Serializer<'ser, 'sig, W>,
    start: usize,
    // The number of `)` in the signature to skip at the end.
    end_parens: u8,
    // All offsets
    offsets: Option<FramingOffsets>,
    // The original container depths. We restore to that at the end.
    container_depths: ContainerDepths,
}

impl<'ser, 'sig, 'b, W> StructSerializer<'ser, 'sig, 'b, W>
where
    W: Write + Seek,
{
    fn variant(ser: &'b mut Serializer<'ser, 'sig, W>) -> Result<Self> {
        ser.common.add_padding(VARIANT_ALIGNMENT_GVARIANT)?;
        let offsets = if ser.sig_parser.next_char()? == STRUCT_SIG_START_CHAR {
            Some(FramingOffsets::new())
        } else {
            None
        };
        let start = ser.common.bytes_written;
        let container_depths = ser.container_depths;
        ser.container_depths = ser.container_depths.inc_variant()?;

        Ok(Self {
            ser,
            end_parens: 0,
            offsets,
            start,
            container_depths,
        })
    }

    fn structure(ser: &'b mut Serializer<'ser, 'sig, W>) -> Result<Self> {
        let c = ser.sig_parser.next_char()?;
        if c != STRUCT_SIG_START_CHAR && c != DICT_ENTRY_SIG_START_CHAR {
            let expected = format!("`{STRUCT_SIG_START_STR}` or `{DICT_ENTRY_SIG_START_STR}`",);

            return Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Char(c),
                &expected.as_str(),
            ));
        }

        let signature = ser.sig_parser.next_signature()?;
        let alignment = alignment_for_signature(&signature, Format::GVariant)?;
        ser.common.add_padding(alignment)?;

        ser.sig_parser.skip_char()?;

        let offsets = if c == STRUCT_SIG_START_CHAR {
            Some(FramingOffsets::new())
        } else {
            None
        };
        let start = ser.common.bytes_written;
        let container_depths = ser.container_depths;
        ser.container_depths = ser.container_depths.inc_structure()?;

        Ok(Self {
            ser,
            end_parens: 1,
            offsets,
            start,
            container_depths,
        })
    }

    fn unit(ser: &'b mut Serializer<'ser, 'sig, W>) -> Result<Self> {
        // serialize as a `0u8`
        serde::Serializer::serialize_u8(&mut *ser, 0)?;

        let start = ser.common.bytes_written;
        let container_depths = ser.container_depths;
        Ok(Self {
            ser,
            end_parens: 0,
            offsets: None,
            start,
            container_depths,
        })
    }

    fn enum_variant(ser: &'b mut Serializer<'ser, 'sig, W>) -> Result<Self> {
        let mut ser = Self::structure(ser)?;
        ser.end_parens += 1;

        Ok(ser)
    }

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
                let bytes_written = self.ser.common.bytes_written;
                let mut ser = Serializer {
                    sig_parser,
                    value_sign: None,
                    container_depths: self.ser.container_depths,
                    common: crate::SerializerCommon::<W> {
                        ctxt: self.ser.common.ctxt,
                        writer: self.ser.common.writer,
                        #[cfg(unix)]
                        fds: self.ser.common.fds,
                        bytes_written,
                    },
                };
                value.serialize(&mut ser)?;
                self.ser.common.bytes_written = ser.common.bytes_written;

                self.ser
                    .common
                    .write_all(&b"\0"[..])
                    .map_err(|e| Error::InputOutput(e.into()))?;
                self.ser
                    .common
                    .write_all(signature.as_bytes())
                    .map_err(|e| Error::InputOutput(e.into()))?;

                Ok(())
            }
            _ => {
                let element_signature = self.ser.sig_parser.next_signature()?;
                let fixed_sized_element =
                    crate::utils::is_fixed_sized_signature(&element_signature)?;

                value.serialize(&mut *self.ser)?;

                if let Some(ref mut offsets) = self.offsets {
                    if !fixed_sized_element {
                        offsets.push_front(self.ser.common.bytes_written - self.start);
                    }
                }

                Ok(())
            }
        }
    }

    fn end_struct(self) -> Result<()> {
        if self.end_parens > 0 {
            self.ser.sig_parser.skip_chars(self.end_parens as usize)?;
        }
        // Restore the original container depths.
        self.ser.container_depths = self.container_depths;

        let mut offsets = match self.offsets {
            Some(offsets) => offsets,
            None => return Ok(()),
        };
        let struct_len = self.ser.common.bytes_written - self.start;
        if struct_len == 0 {
            // Empty sequence
            return Ok(());
        }
        if offsets.peek() == Some(struct_len) {
            // For structs, we don't want offset of last element
            offsets.pop();
        }

        offsets.write_all(&mut self.ser.common, struct_len)?;

        Ok(())
    }
}

#[doc(hidden)]
/// Allows us to serialize a struct as an ARRAY.
pub enum StructSeqSerializer<'ser, 'sig, 'b, W> {
    Struct(StructSerializer<'ser, 'sig, 'b, W>),
    Seq(SeqSerializer<'ser, 'sig, 'b, W>),
}

macro_rules! serialize_struct_anon_fields {
    ($trait:ident $method:ident) => {
        impl<'ser, 'sig, 'b, W> ser::$trait for StructSerializer<'ser, 'sig, 'b, W>
        where
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

        impl<'ser, 'sig, 'b, W> ser::$trait for StructSeqSerializer<'ser, 'sig, 'b, W>
        where
            W: Write + Seek,
        {
            type Ok = ();
            type Error = Error;

            fn $method<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.$method(value),
                    StructSeqSerializer::Seq(ser) => ser.serialize_element(value),
                }
            }

            fn end(self) -> Result<()> {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.end_struct(),
                    StructSeqSerializer::Seq(ser) => ser.end_seq(),
                }
            }
        }
    };
}
serialize_struct_anon_fields!(SerializeTuple serialize_element);
serialize_struct_anon_fields!(SerializeTupleStruct serialize_field);
serialize_struct_anon_fields!(SerializeTupleVariant serialize_field);

impl<'ser, 'sig, 'b, W> ser::SerializeMap for SeqSerializer<'ser, 'sig, 'b, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.common.add_padding(self.element_alignment)?;

        if self.key_start.is_some() {
            self.key_start.replace(self.ser.common.bytes_written);
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
        // For non-fixed-sized keys, we must add the key offset after the value
        let key_offset = self
            .key_start
            .map(|start| self.ser.common.bytes_written - start);

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
            let entry_size = self.ser.common.bytes_written - self.key_start.unwrap_or(0);
            let offset_size = FramingOffsetSize::for_encoded_container(entry_size);
            offset_size.write_offset(&mut self.ser.common, key_offset)?;
        }

        // And now the offset of the array element end (which is encoded later)
        if let Some(ref mut offsets) = self.offsets {
            let offset = self.ser.common.bytes_written - self.start;

            offsets.push(offset);
        }

        Ok(())
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

macro_rules! serialize_struct_named_fields {
    ($trait:ident) => {
        impl<'ser, 'sig, 'b, W> ser::$trait for StructSerializer<'ser, 'sig, 'b, W>
        where
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

        impl<'ser, 'sig, 'b, W> ser::$trait for StructSeqSerializer<'ser, 'sig, 'b, W>
        where
            W: Write + Seek,
        {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.serialize_field(key, value),
                    StructSeqSerializer::Seq(ser) => ser.serialize_element(value),
                }
            }

            fn end(self) -> Result<()> {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.end_struct(),
                    StructSeqSerializer::Seq(ser) => ser.end_seq(),
                }
            }
        }
    };
}
serialize_struct_named_fields!(SerializeStruct);
serialize_struct_named_fields!(SerializeStructVariant);
