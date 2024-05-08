use std::marker::PhantomData;
use serde::ser::SerializeMap;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use serde::ser::SerializeStructVariant;
use serde::ser::SerializeTuple;
use serde::ser::SerializeTupleStruct;
use serde::ser::SerializeTupleVariant;
use serde::Serializer;
use crate::signature_parser::SignatureParser;
use crate::Array;
use crate::Dict;
use crate::ObjectPath;
use crate::Signature;
use crate::Str;
use crate::Structure;
use crate::Value;
use crate::basic::Basic;
use crate::r#type::Type;
use crate::ARRAY_SIGNATURE_CHAR;
use crate::DICT_ENTRY_SIG_END_CHAR;
use crate::DICT_ENTRY_SIG_START_CHAR;
use crate::STRUCT_SIG_END_CHAR;
use crate::STRUCT_SIG_START_CHAR;
use crate::VARIANT_SIGNATURE_CHAR;

pub struct ValueSerializer<'a> {
    sig_parser: SignatureParser<'a>
}

impl<'a> ValueSerializer<'a> {
    pub fn new(signature: Signature<'a>) -> Self {
        Self {
            sig_parser: SignatureParser::new(signature)
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

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        let next_sig_char = self.sig_parser.next_char()?;
        self.sig_parser.skip_char()?;

        match next_sig_char {
            bool::SIGNATURE_CHAR => Ok(Value::Bool(v)),
            i16::SIGNATURE_CHAR => Ok(Value::I16(if v { 1 } else { 0 })),
            i32::SIGNATURE_CHAR => Ok(Value::I32(if v { 1 } else { 0 })),
            i64::SIGNATURE_CHAR => Ok(Value::I64(if v { 1 } else { 0 })),
            u8::SIGNATURE_CHAR => Ok(Value::U8(if v { 1 } else { 0 })),
            u16::SIGNATURE_CHAR => Ok(Value::U16(if v { 1 } else { 0 })),
            u32::SIGNATURE_CHAR => Ok(Value::U32(if v { 1 } else { 0 })),
            u64::SIGNATURE_CHAR => Ok(Value::U64(if v { 1 } else { 0 })),
            String::SIGNATURE_CHAR => Ok(Value::Str(Str::from(if v { "true" } else { "false" }))),
            _ => Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned().into_owned(), "b, n, i, x, y, q, u, t, x".to_string()))
        }
    }

    fn serialize_i8(mut self, v: i8) -> Result<Self::Ok, Self::Error> {
        let next_sig_char = self.sig_parser.next_char()?;
        self.sig_parser.skip_char()?;
         
        match next_sig_char {
            i16::SIGNATURE_CHAR => Ok(Value::I16(v as i16)),
            i32::SIGNATURE_CHAR => Ok(Value::I32(v as i32)),
            i64::SIGNATURE_CHAR => Ok(Value::I64(v as i64)),
            u16::SIGNATURE_CHAR => Ok(Value::U16(v as u16)),
            u32::SIGNATURE_CHAR => Ok(Value::U32(v as u32)),
            u64::SIGNATURE_CHAR => Ok(Value::U64(v as u64)),
            String::SIGNATURE_CHAR => Ok(Value::Str(Str::from(v.to_string()))),
            f64::SIGNATURE_CHAR => Ok(Value::F64(v as f64)),
            _ => Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned().into_owned(), "i, n, x, q, u, t, x, d".to_string()))
        }
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I16(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I32(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::I64(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U8(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U16(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U32(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U64(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F64(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::F64(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::U8(v as u8))
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        let next_sig_char = self.sig_parser.next_char()?;
        self.sig_parser.skip_char()?;
         
        match next_sig_char {
            ObjectPath::SIGNATURE_CHAR => Ok(Value::ObjectPath(ObjectPath::try_from(v.to_string())?)),
            _ => Ok(Value::Str(Str::from(v.to_string()))),
        }
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(Array::from(v)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!("serializing none")
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        unimplemented!("serializing some")
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
        Ok(Value::Str(Str::from(variant.to_string())))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {

        match name {
            "zvariant::ObjectPath" => {
                match value.serialize(self)? {
                    Value::Str(s) => Ok(Value::ObjectPath(ObjectPath::try_from(s.to_string())?)),
                    _ => Err(crate::Error::IncorrectType)
                }
            },
            "zvariant::Signature" => {
                match value.serialize(self)? {
                    Value::Str(s) => Ok(Value::Signature(Signature::try_from(s.to_string())?)),
                    _ => Err(crate::Error::IncorrectType)
                }
            },
            _ => value.serialize(self)
        }
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        Ok(Value::Structure(Structure::new(vec![
            Value::Str(Str::from(variant.to_string())),
            value.serialize(self)?
        ])))
    }

    fn serialize_seq(mut self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.sig_parser.next_char()? == ARRAY_SIGNATURE_CHAR {
            let struct_signature = self.sig_parser.parse_next_signature()?;
            Ok(ValueSeqSerializer::new(struct_signature))
        } else {
            Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned().into_owned(), "an array".to_string()))
        }
    }

    fn serialize_tuple(mut self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if self.sig_parser.next_char()? == STRUCT_SIG_START_CHAR {
            let struct_signature = self.sig_parser.parse_next_signature()?;
            Ok(ValueTupleSerializer::new(struct_signature))
        } else {
            Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned().into_owned(), "an array".to_string()))
        }
    }

    fn serialize_tuple_struct(
        mut self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        if self.sig_parser.next_char()? == STRUCT_SIG_START_CHAR {
            let struct_signature = self.sig_parser.parse_next_signature()?;
            Ok(ValueTupleStructSerializer::new(name.to_string(), struct_signature))
        } else {
            Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned().into_owned(), "an array".to_string()))
        }
    }

    fn serialize_tuple_variant(
        mut self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        if self.sig_parser.next_char()? == STRUCT_SIG_START_CHAR {
            let struct_signature = self.sig_parser.parse_next_signature()?;
            Ok(ValueTupleVariantSerializer::new(variant.to_string(), struct_signature))
        } else {
            Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned(), "an array".to_string()))
        }
    }

    fn serialize_map(mut self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if self.sig_parser.next_char()? == ARRAY_SIGNATURE_CHAR {
            self.sig_parser.skip_char()?;

            if self.sig_parser.next_char()? != DICT_ENTRY_SIG_START_CHAR {
                return Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned(), "a DictEntry".to_string()));
            }

            let dict_entry = self.sig_parser.parse_next_signature()?;
            let mut dict_parser = SignatureParser::new(dict_entry);
            let key_signature = dict_parser.parse_next_signature()?;
            let value_signature = dict_parser.parse_next_signature()?;
            Ok(ValueMapSerializer::new(key_signature, value_signature))
        } else {
            Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned(), "an array".to_string()))
        }
    }

    fn serialize_struct(
        mut self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {

        match self.sig_parser.next_char()? {
            STRUCT_SIG_START_CHAR => {
                let struct_signature = self.sig_parser.parse_next_signature()?;
                Ok(ValueStructSerializer::new(struct_signature, false))
            },
            ARRAY_SIGNATURE_CHAR => {
                self.sig_parser.skip_char()?;

                if self.sig_parser.next_char()? != DICT_ENTRY_SIG_START_CHAR {
                    return Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned(), "a DictEntry".to_string()));
                }

                self.sig_parser.skip_char()?;

                let mut start_counter = 0;
                let mut signature_chars = Vec::new();

                loop {
                    let next_char = self.sig_parser.next_char()?;
                    self.sig_parser.skip_char()?;

                    if next_char == DICT_ENTRY_SIG_START_CHAR {
                        start_counter += 1;
                    } else if next_char == DICT_ENTRY_SIG_END_CHAR {
                        if start_counter == 0 {
                            break;
                        } else {
                            start_counter -= 1;
                        }
                    }

                    signature_chars.push(next_char);
                }

                let key_signature = Signature::try_from(signature_chars[0].to_string())?;
                let value_signature = Signature::try_from(signature_chars[1..].iter().collect::<String>())?;

                if key_signature != String::signature() {
                    return Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned(), "a struct".to_string()))
                }

                Ok(ValueStructSerializer::new(value_signature, true))
            },
            _ => return Err(crate::Error::SignatureMismatch(self.sig_parser.signature().into_owned(), "a struct".to_string()))
        }
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

pub struct ValueSeqSerializer<'a> {
    signature: Signature<'a>,
    fields: Vec<Value<'a>>
}

impl<'a> ValueSeqSerializer<'a> {
    pub fn new(signature: Signature<'a>) -> Self {
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
        T: ?Sized + serde::Serialize {
        self.fields.push(value.serialize(ValueSerializer::new(self.signature.clone()))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Array(Array::from(self.fields)))
    }
}

pub struct ValueTupleSerializer<'a> {
    signature: Signature<'a>,
    fields: Vec<Value<'a>>
}

impl<'a> ValueTupleSerializer<'a> {
    pub fn new(signature: Signature<'a>) -> Self {
        Self {
            signature,
            fields: Vec::new(),
        }
    }
}

impl<'a> SerializeTuple for ValueTupleSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        self.fields.push(value.serialize(ValueSerializer::new(self.signature.clone()))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Structure(Structure::new(self.fields)))
    }
}

pub struct ValueTupleStructSerializer<'a> {
    signature: Signature<'a>,
    fields: Vec<Value<'a>>,
}

impl<'a> ValueTupleStructSerializer<'a> {
    pub fn new(name: String, signature: Signature<'a>) -> Self {
        Self {
            signature,
            fields: vec![
                Value::Str(Str::from(name)),
            ],
        }
    }

}

impl<'a> SerializeTupleStruct for ValueTupleStructSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        self.fields.push(value.serialize(ValueSerializer::new(self.signature.clone()))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Structure(Structure::new(self.fields)))
    }
}

pub struct ValueTupleVariantSerializer<'a> {
    signature: Signature<'a>,
    fields: Vec<Value<'a>>,
}

impl<'a> ValueTupleVariantSerializer<'a> {
    pub fn new(name: String, signature: Signature<'a>) -> Self {
        Self {
            signature,
            fields: vec![
                Value::Str(Str::from(name)),
            ],
        }
    }
}

impl<'a> SerializeTupleVariant for ValueTupleVariantSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        self.fields.push(value.serialize(ValueSerializer::new(self.signature.clone()))?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Structure(Structure::new(self.fields)))
    }
}

pub struct ValueMapSerializer<'a> {
    key_signature: Signature<'a>,
    value_signature: Signature<'a>,
    keys: Vec<Value<'a>>,
    values: Vec<Value<'a>>,
}

impl<'a> ValueMapSerializer<'a> {
    pub fn new(key_signature: Signature<'a>, value_signature: Signature<'a>) -> Self {
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
        T: ?Sized + serde::Serialize {
        self.keys.push(key.serialize(ValueSerializer::new(self.key_signature.clone()))?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        self.values.push(value.serialize(ValueSerializer::new(self.value_signature.clone()))?);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        let mut dict = Dict::new(self.key_signature, self.value_signature);

        for (k,v) in self.keys.drain(..).zip(self.values.drain(..)) {
            dict.append(k, v)?;
        }

        Ok(Value::Dict(dict))
    }
}

pub struct ValueStructSerializer<'a> {
    signature: Signature<'a>,
    keys: Vec<&'static str>,
    values: Vec<Value<'a>>,
    dict: bool
}

impl<'a> ValueStructSerializer<'a> {
    pub fn new(signature: Signature<'a>, dict: bool) -> Self {
        Self {
            signature,
            keys: Vec::new(),
            values: Vec::new(),
            dict
        }
    }
}

impl<'a> SerializeStruct for ValueStructSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        self.keys.push(key);
        self.values.push(value.serialize(ValueSerializer::new(self.signature.clone()))?);
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if self.dict {
            let mut dict = Dict::new(signature_string!("s"), self.signature);

            for (k,v) in self.keys.drain(..).zip(self.values.drain(..)) {
                dict.add(k, v)?;
            }

            Ok(Value::Dict(dict))
        } else {
            Ok(Value::Structure(Structure::new(self.values)))
        }
    }
}



pub struct ValueStructVariantSerializer<'a> {
    phantom: PhantomData<&'a ()>
}

impl<'a> SerializeStructVariant for ValueStructVariantSerializer<'a> {
    type Ok = Value<'a>;
    type Error = crate::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::ObjectPath;
    use crate::OwnedObjectPath;
    use crate::OwnedSignature;
    use super::*;
    use serde::Serialize;

    #[test]
    fn serialize_u8() {
        let input = 42u8;
        let output = input.serialize(ValueSerializer::new(u8::signature())).unwrap();
        assert_eq!(output, Value::U8(42));
    }

    #[test]
    fn serialize_u16() {
        let input = 42u16;
        let output = input.serialize(ValueSerializer::new(u16::signature())).unwrap();
        assert_eq!(output, Value::U16(42));
    }

    #[test]
    fn serialize_u32() {
        let input = 42u32;
        let output = input.serialize(ValueSerializer::new(u32::signature())).unwrap();
        assert_eq!(output, Value::U32(42));
    }

    #[test]
    fn serialize_u64() {
        let input = 42u64;
        let output = input.serialize(ValueSerializer::new(u64::signature())).unwrap();
        assert_eq!(output, Value::U64(42));
    }

    #[test]
    fn serialize_i8() {
        let input = 42i8;
        let output = input.serialize(ValueSerializer::new(i16::signature())).unwrap();
        assert_eq!(output, Value::I16(42));
    }

    #[test]
    fn serialize_i16() {
        let input = 42i16;
        let output = input.serialize(ValueSerializer::new(i16::signature())).unwrap();
        assert_eq!(output, Value::I16(42));
    }

    #[test]
    fn serialize_i32() {
        let input = 42i32;
        let output = input.serialize(ValueSerializer::new(signature_string!("i"))).unwrap();
        assert_eq!(output, Value::I32(42));
    }

    #[test]
    fn serialize_i64() {
        let input = 42i64;
        let output = input.serialize(ValueSerializer::new(signature_string!("x"))).unwrap();
        assert_eq!(output, Value::I64(42));
    }

    #[test]
    fn serialize_f32() {
        let input = 42.0f32;
        let output = input.serialize(ValueSerializer::new(signature_string!("d"))).unwrap();
        assert_eq!(output, Value::F64(42.0));
    }

    #[test]
    fn serialize_f64() {
        let input = 42.0f64;
        let output = input.serialize(ValueSerializer::new(signature_string!("d"))).unwrap();
        assert_eq!(output, Value::F64(42.0));
    }

    #[test]
    fn serialize_bool() {
        let input = true;
        let output = input.serialize(ValueSerializer::new(signature_string!("b"))).unwrap();
        assert_eq!(output, Value::Bool(true));
    }

    #[test]
    fn serialize_char() {
        let input: char = 'a';
        let output = input.serialize(ValueSerializer::new(signature_string!("y"))).unwrap();
        assert_eq!(output, Value::U8(97));
    }

    #[test]
    fn serialize_struct() {
        #[derive(Serialize)]
        struct MyStruct {
            a: OwnedObjectPath,
            b: OwnedSignature
        }

        let input = MyStruct {
            a: ObjectPath::from_static_str_unchecked("/hello").into(),
            b: Signature::try_from("s").unwrap().into()
        };

        let output = input.serialize(ValueSerializer::new(signature_string!("a{sv}"))).unwrap();
        let mut expected = Dict::new(signature_string!("s"), signature_string!("v"));
        expected.add("a", Value::ObjectPath(ObjectPath::from_static_str_unchecked("/hello"))).unwrap();
        expected.add("b", Value::Signature(Signature::try_from("s").unwrap())).unwrap();

        assert_eq!(output, Value::Dict(expected));
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
            let output = input.serialize(ValueSerializer::new(signature_string!("s"))).unwrap();
            assert_eq!(output, Value::Str(Str::from("A")));
        }

        {
            let input = SomeStuff::B;
            let output = input.serialize(ValueSerializer::new(signature_string!("s"))).unwrap();
            assert_eq!(output, Value::Str(Str::from("B")));
        }
    }
}

