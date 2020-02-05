use byteorder::ByteOrder;
use serde::{ser, ser::SerializeSeq, Serialize};

use crate::utils::*;
use crate::{Basic, EncodingFormat};
use crate::{Error, Result};
use crate::{ObjectPath, Signature};

pub struct Serializer<'a> {
    pub(self) format: EncodingFormat,
    // FIXME: Endianness needs to be configurable too
    pub(self) output: &'a mut Vec<u8>,

    pub(self) last_padding: usize,
    // Used when serialising `Signature` and `ObjectPath`.
    custom_str_signature: Option<char>,
}

impl<'a> Serializer<'a> {
    fn add_padding(&mut self, alignment: usize) -> usize {
        let padding = padding_for_n_bytes(self.output.len(), alignment);
        if padding > 0 {
            self.output.resize(self.output.len() + padding, 0);
        }
        self.last_padding = padding;

        return padding;
    }

    fn prep_serialize_basic<T>(&mut self) -> Result<()>
    where
        T: Basic,
    {
        self.add_padding(T::ALIGNMENT);

        Ok(())
    }
}

// FIXME: to_write() would be better, then to_bytes() can be a think wrapper over it
pub fn to_bytes<T: ?Sized>(value: &T, format: EncodingFormat) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut output = vec![];
    let mut serializer = Serializer {
        format,
        output: &mut output,
        last_padding: 0,
        custom_str_signature: None,
    };
    value.serialize(&mut serializer)?;
    Ok(output)
}

impl<'a, 'b> ser::Serializer for &'b mut Serializer<'a> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, 'b>;
    type SerializeTuple = StructSerializer<'a, 'b>;
    type SerializeTupleStruct = StructSerializer<'a, 'b>;
    type SerializeTupleVariant = StructSerializer<'a, 'b>;
    type SerializeMap = SeqSerializer<'a, 'b>;
    type SerializeStruct = StructSerializer<'a, 'b>;
    type SerializeStructVariant = StructSerializer<'a, 'b>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.prep_serialize_basic::<bool>()?;
        self.output.extend(&(v as u32).to_ne_bytes());

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        // No i8 type in D-Bus/GVariant, let's pretend it's i16
        self.serialize_i16(v as i16)
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.prep_serialize_basic::<i16>()?;
        self.output.extend(&v.to_ne_bytes());

        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.prep_serialize_basic::<i32>()?;
        self.output.extend(&v.to_ne_bytes());

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.prep_serialize_basic::<i64>()?;
        self.output.extend(&v.to_ne_bytes());

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.prep_serialize_basic::<u8>()?;
        self.output.extend(&v.to_ne_bytes());

        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.prep_serialize_basic::<u16>()?;
        self.output.extend(&v.to_ne_bytes());

        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.prep_serialize_basic::<u32>()?;
        self.output.extend(&v.to_ne_bytes());

        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.prep_serialize_basic::<u64>()?;
        self.output.extend(&v.to_ne_bytes());

        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        // No f32 type in D-Bus/GVariant, let's pretend it's f64
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.prep_serialize_basic::<f64>()?;
        let mut buf = [0; 8];
        byteorder::NativeEndian::write_f64(&mut buf, v);
        self.output.extend_from_slice(&buf);

        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        // No char type in D-Bus/GVariant, let's pretend it's a string
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        match self.custom_str_signature.take() {
            Some(Signature::SIGNATURE_CHAR) => {
                self.output.extend(&usize_to_u8(v.len()).to_ne_bytes());
            }
            Some(ObjectPath::SIGNATURE_CHAR) => {
                self.add_padding(<&str>::ALIGNMENT);
                self.output.extend(&usize_to_u32(v.len()).to_ne_bytes());
            }
            _ => {
                self.add_padding(<&str>::ALIGNMENT);
                self.output.extend(&usize_to_u32(v.len()).to_ne_bytes());
            }
        }

        self.output.extend(v.as_bytes());
        self.output.push(b'\0');

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

    // FIXME: What am i supposed to do with this strange type?
    fn serialize_unit(self) -> Result<()> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        // Not sure what else can we do with this?
        self.serialize_str(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.serialize_u32(variant_index)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if name == "zvariant::Signature" {
            self.custom_str_signature = Some(Signature::SIGNATURE_CHAR);
        } else if name == "zvariant::ObjectPath" {
            self.custom_str_signature = Some(ObjectPath::SIGNATURE_CHAR);
        }
        value.serialize(self)?;

        Ok(())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)?;

        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.add_padding(ARRAY_ALIGNMENT);
        // Length in bytes (unfortunately not the same as len passed to us here) which we initially
        // set to 0.
        self.output.extend(&0u32.to_ne_bytes());

        let start = self.output.len();
        let padding = self.last_padding;
        Ok(SeqSerializer {
            serializer: self,
            start,
            padding,
            first_padding: 0,
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
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_struct(name, len)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        self.serialize_seq(len)
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        if name != "zvariant::Variant" {
            self.add_padding(STRUCT_ALIGNMENT);
        }

        Ok(StructSerializer { serializer: self })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_struct(name, len)
    }
}

// TODO: Put this in a separate file
pub struct SeqSerializer<'a, 'b> {
    serializer: &'b mut Serializer<'a>,
    start: usize,
    // Own padding
    padding: usize,
    // First element's padding
    first_padding: usize,
}

impl<'a, 'b> SeqSerializer<'a, 'b> {
    pub(self) fn end_seq(self) -> Result<()> {
        // Set size of array in bytes
        let output = &mut (&mut *self.serializer).output;
        let len = usize_to_u32(output.len() - self.start - self.first_padding);
        let len_pos = self.start - 4;
        byteorder::NativeEndian::write_u32(&mut output[len_pos..self.start], len);

        self.serializer.last_padding = self.padding;

        Ok(())
    }
}

impl<'a, 'b> ser::SerializeSeq for SeqSerializer<'a, 'b> {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.start == self.serializer.output.len() {
            // First element
            value.serialize(&mut *self.serializer)?;
            self.first_padding = self.serializer.last_padding;

            Ok(())
        } else {
            value.serialize(&mut *self.serializer)
        }
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

// TODO: Put this in a separate file
pub struct StructSerializer<'a, 'b> {
    serializer: &'b mut Serializer<'a>,
}

impl<'a, 'b> ser::SerializeTuple for StructSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTupleStruct for StructSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeTupleVariant for StructSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeMap for SeqSerializer<'a, 'b> {
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
        if self.start == self.serializer.output.len() {
            // First element
            key.serialize(&mut *self.serializer)?;
            self.first_padding = self.serializer.last_padding;

            Ok(())
        } else {
            key.serialize(&mut *self.serializer)
        }
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<()> {
        self.end_seq()
    }
}

impl<'a, 'b> ser::SerializeStruct for StructSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'b> ser::SerializeStructVariant for StructSerializer<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
