//! Serialization of Rust types into `Value` variants.
//!
//! This module provides a `Serializer` implementation that may be
//! used to serialize Rust types into `Value` variants. These types
//! should be serializable into a specific signature provided at
//! construction time.
//!
//! Normally this module is not used directly, but is instead
//! invoked via the `zvariant::to_value` methods provided by
//! this crate.

use std::{collections::VecDeque, marker::PhantomData};

#[cfg(feature = "gvariant")]
use crate::Maybe;
use crate::{
    basic::Basic, value::parsed_signature::{ParsedSignature, SignatureEntry}, Array, Dict, ObjectPath, Signature, Str, StructureBuilder, Value
};
use serde::{
    de::Error,
    ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    },
    Serializer,
};

/// Serialize a basic type into a `Value` variant. This macro can
/// be used to serialize any basic type that requires no or minimal
/// conversion (using `as`) to encode it as a `Value` variant.
macro_rules! serialize_basic_type {
    ($func:ident($type:ty), $enum:expr) => {
        #[inline]
        fn $func(mut self, value: $type) -> Result<Self::Ok, crate::Error> {
            match self.signature.next() {
                Some(signature) => {
                    if signature.matches::<$type>() {
                        return Ok($enum(value));
                    } else {
                        Err(crate::Error::SignatureMismatch(
                            signature.into(),
                            <$type as Basic>::SIGNATURE_STR.to_string(),
                        ))
                    }
                }
                None => Err(crate::Error::UnexpectedValue(value.to_string()))
            }
        }
    };

    ($func:ident($type:ty), $enum:expr, $as:ty) => {
        #[inline]
        fn $func(mut self, value: $type) -> Result<Self::Ok, crate::Error> {
            match self.signature.next() {
                Some(signature) => {
                    if signature.matches::<$type>() {
                        return Ok($enum(value as $as));
                    } else {
                        Err(crate::Error::SignatureMismatch(
                            signature.into(),
                            <$type as Basic>::SIGNATURE_STR.to_string(),
                        ))
                    }
                }
                
                None => Err(crate::Error::UnexpectedValue(value.to_string()))
            }
        }
    };
}

/// A serializer implementation which produces a `Value` variant
/// from the given input. It serializes according to a signature
/// provided at construction time. If the given input does not
/// compatible the given signature errors will be produced.
pub struct ValueSerializer<'a> {
    signature: ParsedSignature,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ValueSerializer<'a> {
    pub fn new(signature: Signature<'_>) -> Self {
        Self {
            signature: ParsedSignature::parse_bytes(signature.as_bytes()).unwrap(),
            phantom: PhantomData,
        }
    }
}

impl<'a> Serializer for ValueSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;
    type SerializeSeq = ValueSeqSerializer<'a>;
    type SerializeTuple = ValueTupleSerializer<'a>;
    type SerializeTupleStruct = ValueTupleStructSerializer<'a>;
    type SerializeTupleVariant = ValueTupleVariantSerializer<'a>;
    type SerializeMap = ValueMapSerializer<'a>;
    type SerializeStruct = ValueStructSerializer<'a>;
    type SerializeStructVariant = ValueStructVariantSerializer<'a>;

    serialize_basic_type!(serialize_bool(bool), Value::Bool);
    serialize_basic_type!(serialize_i8(i8), Value::I16, i16);
    serialize_basic_type!(serialize_i16(i16), Value::I16);
    serialize_basic_type!(serialize_i32(i32), Value::I32);
    serialize_basic_type!(serialize_i64(i64), Value::I64);
    serialize_basic_type!(serialize_u8(u8), Value::U8);
    serialize_basic_type!(serialize_u16(u16), Value::U16);
    serialize_basic_type!(serialize_u32(u32), Value::U32);
    serialize_basic_type!(serialize_u64(u64), Value::U64);
    serialize_basic_type!(serialize_f32(f32), Value::F64, f64);
    serialize_basic_type!(serialize_f64(f64), Value::F64);

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::ObjectPath) => {
                Ok(Value::ObjectPath(ObjectPath::try_from(v.to_string())?))
            }
            Some(SignatureEntry::Signature) => {
                Ok(Value::Signature(Signature::try_from(v.to_string())?))
            }
            Some(SignatureEntry::Str) => Ok(Value::Str(Str::from(v.to_string()))),
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                String::SIGNATURE_STR.to_string(),
            )),
            None => Err(crate::Error::UnexpectedValue(v.to_string()))
        }
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::Array(elem_signature)) => match *elem_signature {
                SignatureEntry::U8 => Ok(Value::Array(Array::from(v.to_vec()))),
                signature => Err(crate::Error::SignatureMismatch(
                    signature.into(),
                    "ay".to_string(),
                )),
            },
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "ay".to_string(),
            )),
            None => Err(crate::Error::UnexpectedValue("No signature provided for serialized bytes".to_string()))
        }
    }

    fn serialize_none(mut self) -> Result<Self::Ok, Self::Error> {
        match self.signature.next() {
            #[cfg(feature = "gvariant")]
            Some(SignatureEntry::Maybe(signature)) => {
                let signature = *signature;
                Ok(Value::Maybe(Maybe::nothing_full_signature(
                    signature.into(),
                )))
            }

            #[cfg(feature = "option-as-array")]
            Some(SignatureEntry::Array(signature)) => {
                let signature = *signature;
                Ok(Value::Array(Array::new(signature.into())))
            }

            _ => unreachable!(
                "Can only encode Option<T> if `option-as-array` or `gvariant` feature is enabled",
            ),
        }
    }

    fn serialize_some<T>(mut self, #[allow(unused)] value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        match self.signature.next() {
            #[cfg(feature = "gvariant")]
            Some(SignatureEntry::Maybe(signature)) => {
                let signature = *signature;
                let value: Value<'a> =
                    value.serialize(ValueSerializer::new(signature.clone().into()))?;
                Ok(Value::Maybe(Maybe::just_full_signature(
                    value,
                    signature.into(),
                )))
            }

            #[cfg(feature = "option-as-array")]
            Some(SignatureEntry::Array(signature)) => {
                let signature = *signature;
                let mut array = Array::new(signature.clone().into());
                let value: Value<'a> = value.serialize(ValueSerializer::new(signature.into()))?;
                array.append(value)?;
                Ok(Value::Array(array))
            }

            _ => unreachable!("Can only encode Option<T> if `option-as-array` feature is enabled"),
        }
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("serializing unit")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let structure = StructureBuilder::new().add_field(variant).build();

        Ok(Value::Structure(structure))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        match name {
            "zvariant::ObjectPath" => match value.serialize(self)? {
                Value::Str(s) => Ok(Value::ObjectPath(ObjectPath::try_from(s.to_string())?)),
                Value::ObjectPath(p) => Ok(Value::ObjectPath(p)),
                value => Err(crate::Error::invalid_type(
                    value.unexpected(),
                    &"an object path",
                )),
            },
            "zvariant::Signature" => match value.serialize(self)? {
                Value::Str(s) => Ok(Value::Signature(Signature::try_from(s.to_string())?)),
                Value::Signature(s) => Ok(Value::Signature(s)),
                value => Err(crate::Error::invalid_type(
                    value.unexpected(),
                    &"a signature",
                )),
            },
            _ => {
                let structure = StructureBuilder::new()
                    .append_field(value.serialize(self)?)
                    .build();

                Ok(Value::Structure(structure))
            }
        }
    }

    fn serialize_newtype_variant<T>(
        mut self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        match self.signature.next().as_mut() {
            Some(SignatureEntry::Struct(fields)) => {
                let discriminator = match fields.pop_front() {
                    Some(SignatureEntry::Str) => Value::Str(Str::from(variant)),
                    Some(SignatureEntry::U32) => Value::U32(variant_index),
                    Some(signature) => {
                        return Err(crate::Error::SignatureMismatch(
                            signature.into(),
                            "a valid newtype variant discriminant signature".to_string(),
                        ));
                    }
                    None => {
                        return Err(crate::Error::Message(
                            "Newtype variant discriminant required".to_string(),
                        ));
                    }
                };

                if let Some(data_field) = fields.pop_front() {
                    let structure = StructureBuilder::new()
                        .append_field(discriminator)
                        .append_field(value.serialize(ValueSerializer::new(data_field.into()))?)
                        .build();

                    Ok(Value::Structure(structure))
                } else {
                    Err(crate::Error::Message("Struct field required".to_string()))
                }
            }
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "a valid newtype variant signature".to_string(),
            )),
            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized newtype variant".to_string()
            ))
        }
    }

    fn serialize_seq(mut self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::Array(signature)) => Ok(ValueSeqSerializer::new(*signature)),
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "an array".to_string(),
            )),
            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized sequence".to_string()
            ))
        }
    }

    fn serialize_tuple(mut self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::Struct(signature)) => Ok(ValueTupleSerializer::new(signature)),
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "a valid tuple signature".to_string(),
            )),
            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized tuple".to_string()
            ))
        }
    }

    fn serialize_tuple_struct(
        mut self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::Struct(signature)) => {
                Ok(ValueTupleStructSerializer::new(name.to_string(), signature))
            }
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "a valid tuple-struct signature".to_string(),
            )),
            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized tuple-struct".to_string()
            ))
        }
    }

    fn serialize_tuple_variant(
        mut self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::Struct(mut signature)) => {
                let discriminant = match signature.pop_front() {
                    Some(SignatureEntry::U32) => Value::U32(variant_index),
                    Some(SignatureEntry::Str) => Value::Str(Str::from(variant)),
                    Some(signature) => {
                        return Err(crate::Error::SignatureMismatch(
                            signature.into(),
                            "a valid tuple-variant discriminant signature".to_string(),
                        ));
                    }
                    None => {
                        return Err(crate::Error::UnexpectedValue(
                            "Tuple variant discriminant required".to_string(),
                        ));
                    }
                };

                match signature.pop_front() {
                    Some(SignatureEntry::Struct(signature)) => {
                        Ok(ValueTupleVariantSerializer::new(discriminant, signature))
                    }
                    Some(signature) => Err(crate::Error::SignatureMismatch(
                        signature.into(),
                        "a valid tuple-variant inner value signature".to_string(),
                    )),
                    None => Err(crate::Error::UnexpectedValue(
                        "No signature found for serialized tuple-variant inner content".to_string()
                    ))
                }
            }
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "a valid tuple-struct signature".to_string(),
            )),

            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized tuple-variant".to_string()
            ))
        }
    }

    fn serialize_map(mut self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::Array(inner_sig)) => match *inner_sig {
                SignatureEntry::DictEntry(key, value) => Ok(ValueMapSerializer::new(*key, *value)),
                _ => Err(crate::Error::SignatureMismatch(
                    (*inner_sig).into(),
                    "a DictEntry".to_string(),
                )),
            },
            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "a Dictionary of the form a{kv}".to_string(),
            )),
            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized map".to_string()
            ))
        }
    }

    fn serialize_struct(
        mut self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::Struct(signature)) => Ok(ValueStructSerializer::new(signature)),

            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "a valid struct signature".to_string(),
            )),

            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized struct".to_string()
            ))
        }
    }

    fn serialize_struct_variant(
        mut self,
        _name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        match self.signature.next() {
            Some(SignatureEntry::Struct(mut signature)) => {
                let discriminant = match signature.pop_front() {
                    Some(SignatureEntry::U32) => Value::U32(variant_index),
                    Some(SignatureEntry::Str) => Value::Str(Str::from(variant)),
                    Some(signature) => {
                        return Err(crate::Error::SignatureMismatch(
                            signature.into(),
                            "a valid struct-variant discriminant signature".to_string(),
                        ));
                    }
                    None => {
                        return Err(crate::Error::UnexpectedValue(
                            "Struct variant discriminant required".to_string(),
                        ));
                    }
                };

                match signature.pop_front() {
                    Some(SignatureEntry::Struct(signature)) => {
                        Ok(ValueStructVariantSerializer::new(discriminant, signature))
                    }

                    Some(signature) => Err(crate::Error::SignatureMismatch(
                        signature.into(),
                        "a valid struct-variant signature".to_string(),
                    )),

                    None => Err(crate::Error::UnexpectedValue(
                        "No signature found for serialized struct-variant inner content".to_string()
                    ))
                }
            }

            Some(signature) => Err(crate::Error::SignatureMismatch(
                signature.into(),
                "a valid struct-variant signature".to_string(),
            )),

            None => Err(crate::Error::UnexpectedValue(
                "No signature found for serialized struct-variant".to_string()
            ))
        }
    }
}

/// Serialize a sequence of items matching the given signature
/// into an `Array`.
pub struct ValueSeqSerializer<'a> {
    signature: SignatureEntry,
    fields: Vec<Value<'a>>,
}

impl<'a> ValueSeqSerializer<'a> {
    pub fn new(signature: SignatureEntry) -> Self {
        Self {
            signature,
            fields: Vec::new(),
        }
    }
}

impl<'a> SerializeSeq for ValueSeqSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.fields
            .push(value.serialize(ValueSerializer::new(self.signature.clone().into()))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(Array::from(self.fields)))
    }
}

/// Serialize a tuple of items matching the given signature
/// sequence into a `Structure`.
pub struct ValueTupleSerializer<'a> {
    signature_entries: VecDeque<SignatureEntry>,
    fields: Vec<Value<'a>>,
}

impl<'a> ValueTupleSerializer<'a> {
    pub fn new(signature_entries: VecDeque<SignatureEntry>) -> Self {
        Self {
            signature_entries,
            fields: Vec::new(),
        }
    }
}

impl<'a> SerializeTuple for ValueTupleSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if let Some(signature) = self.signature_entries.pop_front() {
            let serializer = ValueSerializer::new(signature.into());
            self.fields.push(value.serialize(serializer)?);
            Ok(())
        } else {
            Err(crate::Error::Message(
                "Too many elements for the tuple".to_string(),
            ))
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut builder = StructureBuilder::new();

        for field in self.fields.drain(..) {
            builder.push_value(field);
        }

        Ok(Value::Structure(builder.build()))
    }
}

/// Serialize a "Tuple Struct" (of the form `struct Name(u32, String)`)
/// into a `Structure`.
pub struct ValueTupleStructSerializer<'a> {
    signature_entries: VecDeque<SignatureEntry>,
    fields: Vec<Value<'a>>,
}

impl<'a> ValueTupleStructSerializer<'a> {
    pub fn new(_name: String, signature_entries: VecDeque<SignatureEntry>) -> Self {
        Self {
            signature_entries,
            fields: vec![],
        }
    }
}

impl<'a> SerializeTupleStruct for ValueTupleStructSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if let Some(signature) = self.signature_entries.pop_front() {
            let serializer = ValueSerializer::new(signature.into());
            self.fields.push(value.serialize(serializer)?);
            Ok(())
        } else {
            Err(crate::Error::Message(
                "Too many elements for the tuple".to_string(),
            ))
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut builder = StructureBuilder::new();

        for field in self.fields.drain(..) {
            builder.push_value(field);
        }

        Ok(Value::Structure(builder.build()))
    }
}

/// Serialize a tuple variant (of the form `enum MyEnum { A(u32, String) }`)
/// into a `Structure` with the discriminant as the first field.
pub struct ValueTupleVariantSerializer<'a> {
    signature_entries: VecDeque<SignatureEntry>,
    fields: Vec<Value<'a>>,
}

impl<'a> ValueTupleVariantSerializer<'a> {
    pub fn new(discriminant: Value<'a>, signature_entries: VecDeque<SignatureEntry>) -> Self {
        Self {
            signature_entries,
            fields: vec![discriminant],
        }
    }
}

impl<'a> SerializeTupleVariant for ValueTupleVariantSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if let Some(signature) = self.signature_entries.pop_front() {
            let serializer = ValueSerializer::new(signature.into());
            self.fields.push(value.serialize(serializer)?);
            Ok(())
        } else {
            Err(crate::Error::Message(
                "Too many elements for the tuple".to_string(),
            ))
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut builder = StructureBuilder::new();

        for field in self.fields.drain(..) {
            builder.push_value(field);
        }

        Ok(Value::Structure(builder.build()))
    }
}

/// Serialize a map of items matching the given key and
/// value signatures into a `Dict`.
pub struct ValueMapSerializer<'a> {
    key_signature: SignatureEntry,
    value_signature: SignatureEntry,
    keys: Vec<Value<'a>>,
    values: Vec<Value<'a>>,
}

impl<'a> ValueMapSerializer<'a> {
    pub fn new(key_signature: SignatureEntry, value_signature: SignatureEntry) -> Self {
        Self {
            key_signature,
            value_signature,
            keys: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl<'a> SerializeMap for ValueMapSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.keys
            .push(key.serialize(ValueSerializer::new(self.key_signature.clone().into()))?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.values
            .push(value.serialize(ValueSerializer::new(self.value_signature.clone().into()))?);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut dict = Dict::new(self.key_signature.into(), self.value_signature.into());

        for (k, v) in self.keys.drain(..).zip(self.values.drain(..)) {
            dict.append(k, v)?;
        }

        Ok(Value::Dict(dict))
    }
}

/// Serialize a struct (of the form `struct MyStruct { a: u32, b: String }`)
/// into a `Structure`.
pub struct ValueStructSerializer<'a> {
    signature_entries: VecDeque<SignatureEntry>,
    values: Vec<Value<'a>>,
}

impl<'a> ValueStructSerializer<'a> {
    pub fn new(signature_entries: VecDeque<SignatureEntry>) -> Self {
        Self {
            signature_entries,
            values: Vec::new(),
        }
    }
}

impl<'a> SerializeStruct for ValueStructSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if let Some(signature) = self.signature_entries.pop_front() {
            let serializer = ValueSerializer::new(signature.into());
            self.values.push(value.serialize(serializer)?);
            Ok(())
        } else {
            Err(crate::Error::Message(
                "Too many elements for the struct".to_string(),
            ))
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut builder = StructureBuilder::new();

        for field in self.values.drain(..) {
            builder.push_value(field);
        }

        Ok(Value::Structure(builder.build()))
    }
}

/// Serialize a struct variant (of the form `enum MyEnum { A { a: u32, b: String } }`)
/// into a `Structure` with the discriminant as the first field.
pub struct ValueStructVariantSerializer<'a> {
    fields: Vec<Value<'a>>,
    signature_entries: VecDeque<SignatureEntry>,
}

impl<'a> ValueStructVariantSerializer<'a> {
    pub fn new(discriminant: Value<'a>, signature_entries: VecDeque<SignatureEntry>) -> Self {
        Self {
            fields: vec![discriminant],
            signature_entries,
        }
    }
}

impl<'a> SerializeStructVariant for ValueStructVariantSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        if let Some(signature) = self.signature_entries.pop_front() {
            let serializer = ValueSerializer::new(signature.into());
            self.fields.push(value.serialize(serializer)?);
            Ok(())
        } else {
            Err(crate::Error::Message(
                "Too many elements for the struct".to_string(),
            ))
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut builder = StructureBuilder::new();

        for field in self.fields.drain(..) {
            builder.push_value(field);
        }

        Ok(Value::Structure(builder.build()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ObjectPath, OwnedObjectPath, OwnedSignature, Type};
    use serde::Serialize;

    #[test]
    fn serialize_u8() {
        let input = 42u8;
        let output = input
            .serialize(ValueSerializer::new(u8::signature()))
            .unwrap();
        assert_eq!(output, Value::U8(42));
    }

    #[test]
    fn serialize_u8_bad_signature() {
        let input = 42u8;
        let err = input
            .serialize(ValueSerializer::new(u16::signature()))
            .expect_err("Should fail due to signature mismatch");
        assert_eq!(err.to_string(), "Signature mismatch: got `q`, expected y");
    }

    #[test]
    fn serialize_u16() {
        let input = 42u16;
        let output = input
            .serialize(ValueSerializer::new(u16::signature()))
            .unwrap();
        assert_eq!(output, Value::U16(42));
    }

    #[test]
    fn serialize_u32() {
        let input = 42u32;
        let output = input
            .serialize(ValueSerializer::new(u32::signature()))
            .unwrap();
        assert_eq!(output, Value::U32(42));
    }

    #[test]
    fn serialize_u64() {
        let input = 42u64;
        let output = input
            .serialize(ValueSerializer::new(u64::signature()))
            .unwrap();
        assert_eq!(output, Value::U64(42));
    }

    #[test]
    fn serialize_i8() {
        let input = 42i8;
        let output = input
            .serialize(ValueSerializer::new(i16::signature()))
            .unwrap();
        assert_eq!(output, Value::I16(42));
    }

    #[test]
    fn serialize_i16() {
        let input = 42i16;
        let output = input
            .serialize(ValueSerializer::new(i16::signature()))
            .unwrap();
        assert_eq!(output, Value::I16(42));
    }

    #[test]
    fn serialize_i32() {
        let input = 42i32;
        let output = input
            .serialize(ValueSerializer::new(signature_string!("i")))
            .unwrap();
        assert_eq!(output, Value::I32(42));
    }

    #[test]
    fn serialize_i64() {
        let input = 42i64;
        let output = input
            .serialize(ValueSerializer::new(signature_string!("x")))
            .unwrap();
        assert_eq!(output, Value::I64(42));
    }

    #[test]
    fn serialize_f32() {
        let input = 42.0f32;
        let output = input
            .serialize(ValueSerializer::new(signature_string!("d")))
            .unwrap();
        assert_eq!(output, Value::F64(42.0));
    }

    #[test]
    fn serialize_f64() {
        let input = 42.0f64;
        let output = input
            .serialize(ValueSerializer::new(signature_string!("d")))
            .unwrap();
        assert_eq!(output, Value::F64(42.0));
    }

    #[test]
    fn serialize_bool() {
        let input = true;
        let output = input
            .serialize(ValueSerializer::new(signature_string!("b")))
            .unwrap();
        assert_eq!(output, Value::Bool(true));
    }

    #[test]
    fn serialize_char() {
        let input: char = 'a';
        let output = input
            .serialize(ValueSerializer::new(char::signature()))
            .unwrap();
        assert_eq!(output, Value::Str(Str::from("a")));
    }

    #[test]
    fn serialize_string() {
        let input: String = "Hello World".to_string();
        let output = input
            .serialize(ValueSerializer::new(signature_string!("s")))
            .unwrap();
        assert_eq!(output, Value::Str(Str::from("Hello World")));
    }

    #[test]
    fn serialize_string_as_path() {
        let input: String = "/hello".to_string();
        let output = input
            .serialize(ValueSerializer::new(signature_string!("o")))
            .unwrap();
        assert_eq!(
            output,
            Value::ObjectPath(ObjectPath::try_from("/hello").unwrap())
        );
    }

    #[test]
    fn serialize_string_as_signature() {
        let input: String = "a{sv}".to_string();
        let output = input
            .serialize(ValueSerializer::new(Signature::signature()))
            .unwrap();
        assert_eq!(output, Value::Signature(signature_string!("a{sv}")));
    }

    #[test]
    fn serialize_newtype_struct() {
        #[derive(Serialize, Type)]
        struct NewTypeStruct(i32);

        let input: NewTypeStruct = NewTypeStruct(42);
        let output = input
            .serialize(ValueSerializer::new(NewTypeStruct::signature()))
            .unwrap();

        let expected = StructureBuilder::new().add_field(42i32).build();
        assert_eq!(output, Value::Structure(expected));
    }

    #[test]
    fn serialize_object_path() {
        let input = ObjectPath::try_from("/hello").unwrap();
        let output = input
            .serialize(ValueSerializer::new(ObjectPath::signature()))
            .unwrap();
        assert_eq!(
            output,
            Value::ObjectPath(ObjectPath::try_from("/hello").unwrap())
        );
    }

    #[test]
    fn serialize_signature() {
        let input = signature_string!("a(sv)");
        let output = input
            .serialize(ValueSerializer::new(Signature::signature()))
            .unwrap();
        assert_eq!(output, Value::Signature(signature_string!("a(sv)")));
    }

    #[test]
    fn serialize_struct() {
        #[derive(Serialize, Type)]
        struct MyStruct {
            a: OwnedObjectPath,
            b: OwnedSignature,
        }

        let input = MyStruct {
            a: ObjectPath::from_static_str_unchecked("/hello").into(),
            b: Signature::try_from("s").unwrap().into(),
        };

        let output = input
            .serialize(ValueSerializer::new(MyStruct::signature()))
            .unwrap();

        let expected = StructureBuilder::new()
            .add_field(ObjectPath::from_static_str_unchecked("/hello"))
            .add_field(signature_string!("s"))
            .build();

        assert_eq!(output, Value::Structure(expected));
    }

    #[test]
    fn serialize_unit_variant() {
        #[derive(Serialize)]
        enum SomeStuff {
            A,
            B,
        }

        {
            let input = SomeStuff::A;
            let output = input
                .serialize(ValueSerializer::new(signature_string!("(s)")))
                .unwrap();

            let expected = StructureBuilder::new().add_field("A").build();

            assert_eq!(output, Value::Structure(expected));
        }

        {
            let input = SomeStuff::B;
            let output = input
                .serialize(ValueSerializer::new(signature_string!("(s)")))
                .unwrap();

            let expected = StructureBuilder::new().add_field("B").build();

            assert_eq!(output, Value::Structure(expected));
        }
    }

    #[test]
    fn serialize_newtype_variant() {
        #[derive(Serialize, Type)]
        enum MyEnum {
            A(i32),
            B(i32),
        }

        {
            let input = MyEnum::A(1);
            let output = input
                .serialize(ValueSerializer::new(MyEnum::signature()))
                .unwrap();

            let expected = StructureBuilder::new()
                .add_field(0u32)
                .add_field(1i32)
                .build();

            assert_eq!(output, Value::Structure(expected));
        }

        {
            let input = MyEnum::B(2);
            let output = input
                .serialize(ValueSerializer::new(MyEnum::signature()))
                .unwrap();

            let expected = StructureBuilder::new()
                .add_field(1u32)
                .add_field(2i32)
                .build();

            assert_eq!(output, Value::Structure(expected));
        }
    }

    #[test]
    fn serialize_struct_variant() {
        #[derive(Serialize, Type)]
        enum MyEnum {
            A { a: i32, b: String },
            B { a: i32, b: String },
        }

        assert_eq!(MyEnum::signature().as_str(), "(u(is))");

        {
            let input = MyEnum::A {
                a: 1,
                b: "hello".into(),
            };
            let output = input
                .serialize(ValueSerializer::new(MyEnum::signature()))
                .unwrap();

            let expected = StructureBuilder::new()
                .add_field(0u32)
                .add_field(1i32)
                .add_field("hello")
                .build();

            assert_eq!(output, Value::Structure(expected));
        }

        {
            let input = MyEnum::B {
                a: 2,
                b: "goodbye".into(),
            };
            let output = input
                .serialize(ValueSerializer::new(MyEnum::signature()))
                .unwrap();

            let expected = StructureBuilder::new()
                .add_field(1u32)
                .add_field(2i32)
                .add_field("goodbye")
                .build();

            assert_eq!(output, Value::Structure(expected));
        }
    }

    #[test]
    fn serialize_tuple_variant() {
        #[derive(Serialize, Type)]
        enum MyEnum {
            A(i32, String),
            B(i32, String),
        }

        assert_eq!(MyEnum::signature().as_str(), "(u(is))");

        {
            let input = MyEnum::A(1, "hello".into());
            let output = input
                .serialize(ValueSerializer::new(MyEnum::signature()))
                .unwrap();

            let expected = StructureBuilder::new()
                .add_field(0u32)
                .add_field(1i32)
                .add_field("hello")
                .build();

            assert_eq!(output, Value::Structure(expected));
        }

        {
            let input = MyEnum::B(2, "goodbye".into());
            let output = input
                .serialize(ValueSerializer::new(MyEnum::signature()))
                .unwrap();

            let expected = StructureBuilder::new()
                .add_field(1u32)
                .add_field(2i32)
                .add_field("goodbye")
                .build();

            assert_eq!(output, Value::Structure(expected));
        }
    }
}
