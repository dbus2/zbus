use serde::{
    ser::{self, SerializeMap, SerializeSeq},
    Serialize,
};
use static_assertions::assert_impl_all;
use std::{
    collections::VecDeque,
    io::{Seek, Write},
    str,
};

use crate::{
    container_depths::ContainerDepths,
    serialized::{Context, Format},
    utils::*,
    value::{
        parsed_signature::{ParsedSignature, SignatureEntry},
        ser::ValueSerializer,
    },
    Basic, Error, Result, Signature, Value, WriteBytes,
};

/// Our D-Bus serialization implementation.
pub(crate) struct Serializer<'ser, W> {
    common: crate::SerializerCommon<'ser, W>,
    parsed_signature: ParsedSignature,
    container_depths: ContainerDepths,
}

assert_impl_all!(Serializer<'_, i32>: Send, Sync, Unpin);

impl<'ser, W> Serializer<'ser, W>
where
    W: Write + Seek,
{
    /// Create a D-Bus Serializer struct instance.
    ///
    /// On Windows, there is no `fds` argument.
    pub fn new<'w: 'ser, 'f: 'ser, 'sig: 'ser, S>(
        signature: S,
        writer: &'w mut W,
        #[cfg(unix)] fds: &'f mut crate::ser::FdList,
        ctxt: Context,
    ) -> Result<Serializer<'ser, W>>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
    {
        assert_eq!(ctxt.format(), Format::DBus);

        let signature = signature.try_into().map_err(Into::into)?;
        let parsed_signature = ParsedSignature::new(&signature);

        Ok(Self {
            common: crate::SerializerCommon {
                ctxt,
                writer,
                #[cfg(unix)]
                fds,
                bytes_written: 0,
            },
            parsed_signature,
            container_depths: ContainerDepths::default(),
        })
    }

    fn inner_array_value<'sig: 'ser, S, V>(&mut self, signature: S, value: V) -> Result<usize>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: Serialize,
    {
        self.inner_value(signature, value, self.container_depths.inc_array()?)
    }

    fn inner_struct_value<'sig: 'ser, S, V>(&mut self, signature: S, value: V) -> Result<usize>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: Serialize,
    {
        self.inner_value(signature, value, self.container_depths.inc_structure()?)
    }

    fn inner_variant_value<'sig: 'ser, S, V>(&mut self, signature: S, value: V) -> Result<usize>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: Serialize,
    {
        self.inner_value(signature, value, self.container_depths.inc_variant()?)
    }

    fn inner_value_samedepth<'sig: 'ser, S, V>(&mut self, signature: S, value: V) -> Result<usize>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: Serialize,
    {
        self.inner_value(signature, value, self.container_depths)
    }

    fn inner_value<'sig: 'ser, S, V>(
        &mut self,
        signature: S,
        value: V,
        container_depths: ContainerDepths,
    ) -> Result<usize>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: Serialize,
    {
        let inner_context = Context::new_dbus(
            self.common.ctxt.endian(),
            self.common.ctxt.position() + self.common.bytes_written,
        );

        let mut ser = Serializer {
            common: crate::SerializerCommon {
                ctxt: inner_context,
                writer: self.common.writer,
                #[cfg(unix)]
                fds: self.common.fds,
                bytes_written: 0,
            },
            parsed_signature: ParsedSignature::new(&signature.try_into().map_err(Into::into)?),
            container_depths,
        };

        value.serialize(&mut ser)?;
        self.common.bytes_written += ser.common.bytes_written;
        Ok(ser.common.bytes_written)
    }

    pub fn bytes_written(&self) -> usize {
        self.common.bytes_written
    }
}

macro_rules! serialize_basic {
    ($method:ident($type:ty) $write_method:ident) => {
        serialize_basic!($method($type) $write_method($type));
    };
    ($method:ident($type:ty) $write_method:ident($as:ty)) => {
        fn $method(self, v: $type) -> Result<()> {
            match self.parsed_signature.next() {
                Some(signature) => {
                    if signature.matches::<$type>() {
                        let alignment = <$type>::alignment(Format::DBus);
                        self.common.add_padding(alignment)?;
                        self.common.$write_method(self.common.ctxt.endian(), v as $as).map_err(|e| Error::InputOutput(e.into()))
                    } else {
                        Err(crate::Error::SignatureMismatch(
                            signature.into(),
                            <$type as Basic>::SIGNATURE_STR.to_string(),
                        ))
                    }
                }

                None => Err(crate::Error::UnexpectedValue(v.to_string())),
            }
        }
    };
}

impl<'ser, 'b, W> ser::Serializer for &'b mut Serializer<'ser, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ArraySerializer<'ser, 'b, W>;
    type SerializeTuple = StructSeqSerializer<'ser, 'b, W>;
    type SerializeTupleStruct = StructSeqSerializer<'ser, 'b, W>;
    type SerializeTupleVariant = StructSeqSerializer<'ser, 'b, W>;
    type SerializeMap = DictSerializer<'ser, 'b, W>;
    type SerializeStruct = StructSeqSerializer<'ser, 'b, W>;
    type SerializeStructVariant = StructSeqSerializer<'ser, 'b, W>;

    serialize_basic!(serialize_bool(bool) write_u32(u32));
    // No i8 type in D-Bus/GVariant, let's pretend it's i16
    serialize_basic!(serialize_i8(i8) write_i16(i16));
    serialize_basic!(serialize_i16(i16) write_i16);
    serialize_basic!(serialize_i64(i64) write_i64);

    fn serialize_i32(self, v: i32) -> Result<()> {
        match self.parsed_signature.next() {
            Some(SignatureEntry::I32) => {
                self.common.add_padding(i32::alignment(Format::DBus))?;
                self.common
                    .write_i32(self.common.ctxt.endian(), v)
                    .map_err(|e| Error::InputOutput(e.into()))
            }
            Some(SignatureEntry::Fd) => {
                self.common.add_padding(i32::alignment(Format::DBus))?;
                let idx = self.common.add_fd(v)?;
                self.common
                    .write_u32(self.common.ctxt.endian(), idx)
                    .map_err(|e| Error::InputOutput(e.into()))
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected an i32 of fd type signature".into(),
            )),
            None => Err(Error::UnexpectedValue(v.to_string())),
        }
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        match self.parsed_signature.next() {
            Some(SignatureEntry::U8) => {
                self.common.add_padding(u8::alignment(Format::DBus))?;
                self.common
                    .write_u8(self.common.ctxt.endian(), v)
                    .map_err(|e| Error::InputOutput(e.into()))
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a u8 signature".into(),
            )),
            None => Err(Error::UnexpectedValue(v.to_string())),
        }
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
                &"D-Bus string-like type must not contain interior null bytes",
            ));
        }

        // Write the length field based on signature type
        match self.parsed_signature.next() {
            Some(SignatureEntry::ObjectPath) | Some(SignatureEntry::Str) => {
                self.common.add_padding(<&str>::alignment(Format::DBus))?;
                self.common
                    .write_u32(self.common.ctxt.endian(), usize_to_u32(v.len()))
                    .map_err(|e| Error::InputOutput(e.into()))?;
            }

            Some(SignatureEntry::Signature) => {
                self.common
                    .write_u8(self.common.ctxt.endian(), usize_to_u8(v.len()))
                    .map_err(|e| Error::InputOutput(e.into()))?;
            }

            Some(signature) => {
                return Err(Error::SignatureMismatch(
                    signature.into(),
                    "Expected a string, object path, signature or variant signature".into(),
                ));
            }

            None => {
                return Err(Error::UnexpectedValue(v.to_string()));
            }
        }

        // Write the actual string
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

    fn serialize_none(self) -> Result<()> {
        #[cfg(feature = "option-as-array")]
        {
            let seq = self.serialize_seq(Some(0))?;
            seq.end()
        }

        #[cfg(not(feature = "option-as-array"))]
        unreachable!(
            "Can only encode Option<T> in D-Bus format if `option-as-array` feature is enabled",
        );
    }

    fn serialize_some<T>(self, #[allow(unused)] value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        #[cfg(feature = "option-as-array")]
        {
            let mut seq = self.serialize_seq(Some(1))?;
            seq.serialize_element(value)?;
            seq.end()
        }

        #[cfg(not(feature = "option-as-array"))]
        unreachable!(
            "Can only encode Option<T> in D-Bus format if `option-as-array` feature is enabled",
        );
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
        variant: &'static str,
    ) -> Result<()> {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Str) => {
                let _ = self.inner_value_samedepth(SignatureEntry::Str, variant)?;
                Ok(())
            }
            Some(SignatureEntry::U32) => {
                self.common.add_padding(u32::alignment(Format::DBus))?;
                self.common
                    .write_u32(self.common.ctxt.endian(), variant_index)?;
                Ok(())
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a string or u32 signature".into(),
            )),
            None => Err(Error::UnexpectedValue(variant.to_string())),
        }
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
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
        match self.parsed_signature.next().as_mut() {
            Some(SignatureEntry::Struct(fields)) => {
                self.common.add_padding(STRUCT_ALIGNMENT_DBUS)?;

                if let Some(SignatureEntry::U32) = fields.pop_front() {
                    self.common
                        .write_u32(self.common.ctxt.endian(), variant_index)?;

                    if let Some(data_field) = fields.pop_front() {
                        let _ = self.inner_struct_value(data_field, value)?;
                        Ok(())
                    } else {
                        Err(crate::Error::Message(
                            "Newtype variant value signature required".to_string(),
                        ))
                    }
                } else {
                    Err(crate::Error::Message(
                        "Newtype variant discriminant required".to_string(),
                    ))
                }
            }
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "a valid newtype variant signature".to_string(),
            )),
            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized newtype variant".to_string(),
            )),
        }
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Array(signature)) => {
                self.common.add_padding(ARRAY_ALIGNMENT_DBUS)?;

                // Length in bytes (unfortunately not the same as len passed to us here) which we
                // initially set to 0.
                self.common
                    .write_u32(self.common.ctxt.endian(), 0_u32)
                    .map_err(|e| Error::InputOutput(e.into()))?;

                // D-Bus expects us to add padding for the first element even when there is no first
                // element (i-e empty array) so we add padding already.
                let initial_padding =
                    self.common.add_padding(signature.alignment(Format::DBus))? as i64;

                Ok(ArraySerializer {
                    ser: self,
                    len: 0,
                    initial_padding,
                    signature: *signature,
                })
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected an array signature".into(),
            )),
            None => Err(Error::UnexpectedValue(
                "No signature found for serialized sequence".to_string(),
            )),
        }
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
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Struct(mut fields)) => {
                self.common.add_padding(STRUCT_ALIGNMENT_DBUS)?;

                match fields.pop_front() {
                    Some(SignatureEntry::U32) => {
                        self.common
                            .write_u32(self.common.ctxt.endian(), variant_index)?;
                    }

                    Some(SignatureEntry::Str) => {
                        let _ = self.inner_value_samedepth(SignatureEntry::Str, variant)?;
                    }
                    Some(signature) => {
                        return Err(crate::Error::SignatureMismatch(
                            signature.into(),
                            "a valid tuple variant discriminant signature".to_string(),
                        ));
                    }
                    None => {
                        return Err(crate::Error::Message(
                            "Tuple variant discriminant required".to_string(),
                        ));
                    }
                }

                match fields.pop_front() {
                    Some(SignatureEntry::Struct(fields)) => {
                        self.common.add_padding(STRUCT_ALIGNMENT_DBUS)?;
                        StructSerializer::enum_variant(self, fields)
                            .map(StructSeqSerializer::Struct)
                    }
                    Some(signature) => Err(crate::Error::SignatureMismatch(
                        signature.into(),
                        "a valid tuple-variant inner value signature".to_string(),
                    )),
                    None => Err(crate::Error::UnexpectedValue(
                        "No signature found for serialized tuple-variant inner content".to_string(),
                    )),
                }
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a struct signature for tuple variant".into(),
            )),
            None => Err(Error::UnexpectedValue(
                "No signature found for serialized tuple variant".to_string(),
            )),
        }
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Array(signature)) => {
                match *signature {
                    SignatureEntry::DictEntry(key_signature, value_signature) => {
                        // Pad to the beginning of an array
                        self.common.add_padding(ARRAY_ALIGNMENT_DBUS)?;

                        // Write a 0 length
                        self.common.write_u32(self.common.ctxt.endian(), 0)?;

                        // Go ahead and add padding for the first element
                        let padding = self.common.add_padding(STRUCT_ALIGNMENT_DBUS)?;

                        Ok(DictSerializer {
                            ser: self,
                            key_signature: *key_signature,
                            value_signature: *value_signature,
                            len: 0,
                            initial_padding: padding as i64,
                        })
                    }
                    signature => Err(Error::SignatureMismatch(
                        signature.into(),
                        "Expected a dict entry signature".into(),
                    )),
                }
            }

            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a dictionary signature".into(),
            )),

            None => Err(Error::UnexpectedValue(
                "No signature found for serialized map".to_string(),
            )),
        }
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Struct(fields)) => {
                if fields.is_empty() {
                    StructSerializer::unit(self).map(StructSeqSerializer::Struct)
                } else {
                    StructSerializer::structure(self, fields).map(StructSeqSerializer::Struct)
                }
            }

            Some(SignatureEntry::Array(field)) => {
                self.parsed_signature = SignatureEntry::Array(field).into();
                self.serialize_seq(Some(len))
                    .map(StructSeqSerializer::Array)
            }

            Some(SignatureEntry::DictEntry(key, value)) => {
                StructSerializer::structure(self, VecDeque::from([*key, *value]))
                    .map(StructSeqSerializer::Struct)
            }

            Some(SignatureEntry::Variant) => {
                self.parsed_signature = ParsedSignature::from(SignatureEntry::Variant);
                StructSerializer::variant(self).map(StructSeqSerializer::Struct)
            }

            Some(SignatureEntry::U8) => {
                self.parsed_signature = ParsedSignature::from(SignatureEntry::U8);
                StructSerializer::unit(self).map(StructSeqSerializer::Struct)
            }

            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a struct, array or variant signature".into(),
            )),

            None => Err(Error::UnexpectedValue(
                "No signature found for serialized struct".to_string(),
            )),
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Struct(mut fields)) => {
                self.common.add_padding(STRUCT_ALIGNMENT_DBUS)?;

                match fields.pop_front() {
                    Some(SignatureEntry::U32) => {
                        self.common
                            .write_u32(self.common.ctxt.endian(), variant_index)?;
                    }

                    Some(SignatureEntry::Str) => {
                        let _ = self.inner_value_samedepth(SignatureEntry::Str, variant)?;
                    }
                    Some(signature) => {
                        return Err(crate::Error::SignatureMismatch(
                            signature.into(),
                            "a valid tuple variant discriminant signature".to_string(),
                        ));
                    }
                    None => {
                        return Err(crate::Error::Message(
                            "Tuple variant discriminant required".to_string(),
                        ));
                    }
                }

                match fields.pop_front() {
                    Some(SignatureEntry::Struct(fields)) => {
                        self.common.add_padding(STRUCT_ALIGNMENT_DBUS)?;
                        StructSerializer::enum_variant(self, fields)
                            .map(StructSeqSerializer::Struct)
                    }
                    Some(signature) => Err(crate::Error::SignatureMismatch(
                        signature.into(),
                        "a valid tuple-variant inner value signature".to_string(),
                    )),
                    None => Err(crate::Error::UnexpectedValue(
                        "No signature found for serialized tuple-variant inner content".to_string(),
                    )),
                }
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a struct signature for tuple variant".into(),
            )),
            None => Err(Error::UnexpectedValue(
                "No signature found for serialized tuple variant".to_string(),
            )),
        }
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

#[doc(hidden)]
pub struct DictSerializer<'ser, 'b, W> {
    ser: &'b mut Serializer<'ser, W>,
    key_signature: SignatureEntry,
    value_signature: SignatureEntry,
    len: i64,
    initial_padding: i64,
}

#[doc(hidden)]
pub struct ArraySerializer<'ser, 'b, W> {
    ser: &'b mut Serializer<'ser, W>,
    signature: SignatureEntry,
    len: i64,
    initial_padding: i64,
}

impl<'ser, 'b, W> SerializeSeq for ArraySerializer<'ser, 'b, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let serialized_size = self.ser.inner_array_value(self.signature.clone(), value)?;
        self.len += serialized_size as i64;
        Ok(())
    }

    fn end(self) -> Result<()> {
        // Set size of array in bytes
        self.ser
            .common
            .writer
            .seek(std::io::SeekFrom::Current(
                -(self.len + self.initial_padding + 4),
            ))
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.ser
            .common
            .writer
            .write_u32(self.ser.common.ctxt.endian(), self.len as u32)
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.ser
            .common
            .writer
            .seek(std::io::SeekFrom::Current(self.len + self.initial_padding))
            .map_err(|e| Error::InputOutput(e.into()))?;

        Ok(())
    }
}

#[doc(hidden)]
pub struct StructSerializer<'ser, 'b, W> {
    ser: &'b mut Serializer<'ser, W>,
    fields: VecDeque<SignatureEntry>,
    variant_signature: Option<Signature<'static>>,
}

impl<'ser, 'b, W> StructSerializer<'ser, 'b, W>
where
    W: Write + Seek,
{
    fn variant(ser: &'b mut Serializer<'ser, W>) -> Result<Self> {
        ser.common.add_padding(VARIANT_ALIGNMENT_DBUS)?;

        Ok(Self {
            ser,
            fields: VecDeque::from(vec![]),
            variant_signature: None,
        })
    }

    fn structure(
        ser: &'b mut Serializer<'ser, W>,
        fields: VecDeque<SignatureEntry>,
    ) -> Result<Self> {
        ser.common.add_padding(STRUCT_ALIGNMENT_DBUS)?;
        Ok(Self {
            ser,
            fields,
            variant_signature: None,
        })
    }

    fn unit(ser: &'b mut Serializer<'ser, W>) -> Result<Self> {
        // serialize as a `0u8`
        serde::Serializer::serialize_u8(&mut *ser, 0)?;

        Ok(Self {
            ser,
            fields: VecDeque::new(),
            variant_signature: None,
        })
    }

    fn enum_variant(
        ser: &'b mut Serializer<'ser, W>,
        fields: VecDeque<SignatureEntry>,
    ) -> Result<Self> {
        Self::structure(ser, fields)
    }

    fn serialize_struct_element<T>(&mut self, name: Option<&'static str>, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match name {
            Some("zvariant::Value::Signature") => {
                let value_serializer = ValueSerializer::new(SignatureEntry::Signature.into());
                let parsed_value: Value<'_> = value.serialize(value_serializer)?;

                if let Value::Signature(signature) = parsed_value {
                    self.variant_signature = Some(signature.clone());
                    let _ = self
                        .ser
                        .inner_value_samedepth(SignatureEntry::Signature, signature.as_str())?;
                    Ok(())
                } else {
                    Err(Error::Message("Incorrect Signature encoding".to_string()))
                }
            }

            Some("zvariant::Value::Value") => {
                let signature = self
                    .variant_signature
                    .take()
                    .expect("Value should serialize signature before value");

                let _ = self.ser.inner_variant_value(signature, value)?;
                Ok(())
            }

            _ => match self.fields.pop_front() {
                Some(signature) => {
                    let _ = self.ser.inner_struct_value(signature, value)?;
                    Ok(())
                }
                None => Err(Error::Message(
                    "No signature found for serialized struct field".to_string(),
                )),
            },
        }
    }
}

#[doc(hidden)]
/// Allows us to serialize a struct as an ARRAY.
pub enum StructSeqSerializer<'ser, 'b, W> {
    Struct(StructSerializer<'ser, 'b, W>),
    Array(ArraySerializer<'ser, 'b, W>),
}

macro_rules! serialize_struct_anon_fields {
    ($trait:ident $method:ident) => {
        impl<'ser, 'sig, 'b, W> ser::$trait for StructSerializer<'ser, 'b, W>
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
                Ok(())
            }
        }

        impl<'ser, 'sig, 'b, W> ser::$trait for StructSeqSerializer<'ser, 'b, W>
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
                    StructSeqSerializer::Array(ser) => ser.serialize_element(value),
                }
            }

            fn end(self) -> Result<()> {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.end(),
                    StructSeqSerializer::Array(ser) => ser.end(),
                }
            }
        }
    };
}
serialize_struct_anon_fields!(SerializeTuple serialize_element);
serialize_struct_anon_fields!(SerializeTupleStruct serialize_field);
serialize_struct_anon_fields!(SerializeTupleVariant serialize_field);

impl<'ser, 'b, W> SerializeMap for DictSerializer<'ser, 'b, W>
where
    W: Write + Seek,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Dict entries must be padded out the same way as a struct
        self.len += self.ser.common.add_padding(STRUCT_ALIGNMENT_DBUS)? as i64;
        let serialized_len = self
            .ser
            .inner_struct_value(self.key_signature.clone(), key)?;
        self.len += serialized_len as i64;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let serialized_len = self
            .ser
            .inner_struct_value(self.value_signature.clone(), value)?;
        self.len += serialized_len as i64;
        Ok(())
    }

    fn end(self) -> Result<()> {
        // Set size of array in bytes
        self.ser
            .common
            .writer
            .seek(std::io::SeekFrom::Current(
                -(self.initial_padding + self.len + 4),
            ))
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.ser
            .common
            .writer
            .write_u32(self.ser.common.ctxt.endian(), self.len as u32)
            .map_err(|e| Error::InputOutput(e.into()))?;
        self.ser
            .common
            .writer
            .seek(std::io::SeekFrom::Current(self.initial_padding + self.len))
            .map_err(|e| Error::InputOutput(e.into()))?;

        Ok(())
    }
}

macro_rules! serialize_struct_named_fields {
    ($trait:ident) => {
        impl<'ser, 'b, W> ser::$trait for StructSerializer<'ser, 'b, W>
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
                Ok(())
            }
        }

        impl<'ser, 'b, W> ser::$trait for StructSeqSerializer<'ser, 'b, W>
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
                    StructSeqSerializer::Array(ser) => ser.serialize_element(value),
                }
            }

            fn end(self) -> Result<()> {
                match self {
                    StructSeqSerializer::Struct(ser) => ser.end(),
                    StructSeqSerializer::Array(ser) => ser.end(),
                }
            }
        }
    };
}
serialize_struct_named_fields!(SerializeStruct);
serialize_struct_named_fields!(SerializeStructVariant);
