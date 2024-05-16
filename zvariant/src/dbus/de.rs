use serde::{
    de::{
        self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
        Visitor,
    },
    forward_to_deserialize_any,
};
use static_assertions::assert_impl_all;

use std::{collections::VecDeque, str};

#[cfg(unix)]
use std::os::fd::AsFd;

use crate::{
    container_depths::ContainerDepths,
    de::DeserializerCommon,
    serialized::{Context, Format},
    utils::*,
    value::parsed_signature::{ParsedSignature, SignatureEntry},
    Basic, Error, ObjectPath, Result, Signature,
};

/// Our D-Bus deserialization implementation.
#[derive(Debug)]
pub(crate) struct Deserializer<'de, 'f, F> {
    parsed_signature: ParsedSignature,
    container_depths: ContainerDepths,
    pub(crate) common: DeserializerCommon<'de, 'f, F>,
}

assert_impl_all!(Deserializer<'_, '_, ()>: Send, Sync, Unpin);

impl<'de, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> Deserializer<'de, 'f, F> {
    /// Create a Deserializer struct instance.
    ///
    /// On Windows, there is no `fds` argument.
    pub fn new<'r: 'de, 'sig: 'de, S>(
        bytes: &'r [u8],
        #[cfg(unix)] fds: Option<&'f [F]>,
        signature: S,
        ctxt: Context,
    ) -> Result<Self>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
    {
        assert_eq!(ctxt.format(), Format::DBus);

        let signature = signature.try_into().map_err(Into::into)?;
        let parsed_signature = ParsedSignature::new(&signature);

        Ok(Self {
            parsed_signature,
            container_depths: ContainerDepths::default(),
            common: DeserializerCommon {
                ctxt,
                bytes,
                #[cfg(unix)]
                fds,
                #[cfg(not(unix))]
                fds: PhantomData,
                pos: 0,
            },
        })
    }

    fn inner_array<'sig: 'de, S, V>(&mut self, signature: S, seed: V) -> Result<(usize, V::Value)>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: DeserializeSeed<'de>,
    {
        self.inner_seed(signature, seed, self.container_depths.inc_array()?)
    }

    fn inner_struct<'sig: 'de, S, V>(&mut self, signature: S, seed: V) -> Result<(usize, V::Value)>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: DeserializeSeed<'de>,
    {
        self.inner_seed(signature, seed, self.container_depths.inc_structure()?)
    }

    fn inner_variant<'sig: 'de, S, V>(&mut self, signature: S, seed: V) -> Result<(usize, V::Value)>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: DeserializeSeed<'de>,
    {
        self.inner_seed(signature, seed, self.container_depths.inc_variant()?)
    }

    #[cfg(feature = "option-as-array")]
    fn inner_some_visitor<'sig: 'de, S, V>(
        &mut self,
        signature: S,
        visitor: V,
        container_depths: ContainerDepths,
    ) -> Result<V::Value>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: Visitor<'de>,
    {
        let inner_context = Context::new_dbus(
            self.common.ctxt.endian(),
            self.common.ctxt.position() + self.common.pos,
        );

        let mut de = Deserializer {
            common: DeserializerCommon {
                ctxt: inner_context,
                bytes: subslice(self.common.bytes, self.common.pos..)?,
                fds: self.common.fds,
                pos: 0,
            },
            parsed_signature: ParsedSignature::new(&signature.try_into().map_err(Into::into)?),
            container_depths,
        };

        let result = visitor.visit_some(&mut de)?;
        self.common.pos += de.common.pos;
        Ok(result)
    }

    fn inner_seed<'sig: 'de, S, V>(
        &mut self,
        signature: S,
        seed: V,
        container_depths: ContainerDepths,
    ) -> Result<(usize, V::Value)>
    where
        S: TryInto<Signature<'sig>>,
        S::Error: Into<Error>,
        V: DeserializeSeed<'de>,
    {
        let inner_context = Context::new_dbus(
            self.common.ctxt.endian(),
            self.common.ctxt.position() + self.common.pos,
        );

        let mut de = Deserializer {
            common: DeserializerCommon {
                ctxt: inner_context,
                bytes: subslice(self.common.bytes, self.common.pos..)?,
                fds: self.common.fds,
                pos: 0,
            },
            parsed_signature: ParsedSignature::new(&signature.try_into().map_err(Into::into)?),
            container_depths,
        };

        let result = seed.deserialize(&mut de)?;
        self.common.pos += de.common.pos;
        Ok((de.common.pos, result))
    }
}

macro_rules! deserialize_basic {
    ($method:ident $read_method:ident $visitor_method:ident($type:ty)) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            match self.parsed_signature.next() {
                Some(signature) => {
                    if signature.matches::<$type>() {
                        self.common
                            .parse_padding(<$type>::alignment(Format::DBus))?;

                        let v = self
                            .common
                            .ctxt
                            .endian()
                            .$read_method(self.common.next_const_size_slice::<$type>()?);

                        visitor.$visitor_method(v)
                    } else {
                        Err(Error::SignatureMismatch(
                            signature.into(),
                            format!("`{}`", <$type>::SIGNATURE_STR),
                        ))
                    }
                }
                None => Err(Error::MissingSignature),
            }
        }
    };
}

macro_rules! deserialize_as {
    ($method:ident => $as:ident) => {
        deserialize_as!($method() => $as());
    };
    ($method:ident($($in_arg:ident: $type:ty),*) => $as:ident($($as_arg:expr),*)) => {
        #[inline]
        fn $method<V>(self, $($in_arg: $type,)* visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            self.$as($($as_arg,)* visitor)
        }
    }
}

impl<'de, 'd, 'f, #[cfg(unix)] F: AsFd, #[cfg(not(unix))] F> de::Deserializer<'de>
    for &'d mut Deserializer<'de, 'f, F>
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parsed_signature.peek() {
            Some(SignatureEntry::Bool) => self.deserialize_bool(visitor),
            Some(SignatureEntry::U8) => self.deserialize_u8(visitor),
            Some(SignatureEntry::U16) => self.deserialize_u16(visitor),
            Some(SignatureEntry::U32) => self.deserialize_u32(visitor),
            Some(SignatureEntry::U64) => self.deserialize_u64(visitor),
            Some(SignatureEntry::I16) => self.deserialize_i16(visitor),
            Some(SignatureEntry::I32) => self.deserialize_i32(visitor),
            Some(SignatureEntry::I64) => self.deserialize_i64(visitor),
            Some(SignatureEntry::F64) => self.deserialize_f64(visitor),
            Some(SignatureEntry::Str) => self.deserialize_str(visitor),
            Some(SignatureEntry::ObjectPath) => self.deserialize_str(visitor),
            Some(SignatureEntry::Signature) => self.deserialize_str(visitor),
            Some(SignatureEntry::Array(inner_signature)) => match **inner_signature {
                SignatureEntry::DictEntry(_, _) => self.deserialize_map(visitor),
                _ => self.deserialize_seq(visitor),
            },
            Some(SignatureEntry::Struct(_)) => self.deserialize_seq(visitor),
            Some(SignatureEntry::Variant) => self.deserialize_seq(visitor),

            #[cfg(unix)]
            Some(SignatureEntry::Fd) => self.deserialize_i32(visitor),

            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "a signature that can be directly deserialized".into(),
            )),

            None => Err(Error::MissingSignature),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self
            .common
            .ctxt
            .endian()
            .read_u32(self.common.next_const_size_slice::<bool>()?);
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

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_i16(visitor)
    }

    deserialize_basic!(deserialize_i16 read_i16 visit_i16(i16));
    deserialize_basic!(deserialize_i64 read_i64 visit_i64(i64));
    deserialize_basic!(deserialize_u16 read_u16 visit_u16(u16));
    deserialize_basic!(deserialize_u32 read_u32 visit_u32(u32));
    deserialize_basic!(deserialize_u64 read_u64 visit_u64(u64));
    deserialize_basic!(deserialize_f64 read_f64 visit_f64(f64));
    deserialize_as!(deserialize_char => deserialize_str);
    deserialize_as!(deserialize_string => deserialize_str);
    deserialize_as!(deserialize_tuple(_l: usize) => deserialize_struct("", &[]));
    deserialize_as!(deserialize_struct(_n: &'static str, _f: &'static [&'static str]) => deserialize_seq());
    deserialize_as!(deserialize_ignored_any => deserialize_any);

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.common.parse_padding(i32::alignment(Format::DBus))?;

        match self.parsed_signature.next() {
            Some(SignatureEntry::I32) => {
                let v = self
                    .common
                    .ctxt
                    .endian()
                    .read_i32(self.common.next_const_size_slice::<i32>()?);

                visitor.visit_i32(v)
            }

            #[cfg(unix)]
            Some(SignatureEntry::Fd) => {
                let idx = self
                    .common
                    .ctxt
                    .endian()
                    .read_u32(self.common.next_const_size_slice::<u32>()?);
                let v = self.common.get_fd(idx)?;

                visitor.visit_i32(v)
            }

            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                format!("`{}`", i32::SIGNATURE_STR),
            )),
            None => Err(Error::MissingSignature),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parsed_signature.next() {
            Some(SignatureEntry::U8) => visitor.visit_u8(
                self.common
                    .ctxt
                    .endian()
                    .read_u8(self.common.next_const_size_slice::<u8>()?),
            ),
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                format!("`{}`", u8::SIGNATURE_STR),
            )),
            None => Err(Error::MissingSignature),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parsed_signature.next() {
            Some(SignatureEntry::F64) => {
                self.common.parse_padding(f64::alignment(Format::DBus))?;

                let v = self
                    .common
                    .ctxt
                    .endian()
                    .read_f64(self.common.next_const_size_slice::<f64>()?);

                visitor.visit_f32(v as f32)
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "a valid floating point signature".into(),
            )),
            None => Err(Error::MissingSignature),
        }
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.common.parse_padding(u32::alignment(Format::DBus))?;

        let length = self.common.next_slice(4)?;
        let length = self.common.ctxt.endian().read_u32(length) as usize;
        let slice = self.common.next_slice(length)?;

        visitor.visit_borrowed_bytes(slice)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = match self.parsed_signature.next() {
            Some(SignatureEntry::Signature) => {
                let len_slice = self.common.next_slice(1)?;
                len_slice[0] as usize
            }

            Some(SignatureEntry::Str) | Some(SignatureEntry::ObjectPath) => {
                let alignment = u32::alignment(Format::DBus);
                self.common.parse_padding(alignment)?;
                let len_slice = self.common.next_slice(alignment)?;
                self.common.ctxt.endian().read_u32(len_slice) as usize
            }

            Some(signature) => {
                return Err(Error::SignatureMismatch(
                    signature.into(),
                    "A string-like signature".into(),
                ))
            }

            None => return Err(Error::MissingSignature),
        };
        let slice = self.common.next_slice(len)?;
        if slice.contains(&0) {
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Char('\0'),
                &"D-Bus string type must not contain interior null bytes",
            ));
        }
        self.common.pos += 1; // skip trailing null byte
        let s = str::from_utf8(slice).map_err(Error::Utf8)?;
        visitor.visit_borrowed_str(s)
    }

    fn deserialize_option<V>(self, #[allow(unused)] visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        #[cfg(feature = "option-as-array")]
        {
            match self.parsed_signature.next() {
                Some(SignatureEntry::Array(inner_signature)) => {
                    let inner_signature = *inner_signature;
                    let array_access = ArrayAccess::new(inner_signature.clone(), self)?;
                    if array_access.is_empty() {
                        visitor.visit_none()
                    } else {
                        self.inner_some_visitor(
                            inner_signature,
                            visitor,
                            self.container_depths.inc_array()?,
                        )
                    }
                }
                Some(signature) => Err(Error::SignatureMismatch(
                    signature.into(),
                    "Expected an array signature for Option<T>".into(),
                )),
                None => Err(Error::MissingSignature),
            }
        }

        #[cfg(not(feature = "option-as-array"))]
        Err(Error::Message(
            "Can only decode Option<T> from D-Bus format if `option-as-array` feature is enabled"
                .into(),
        ))
    }

    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match name {
            "zvariant::ObjectPath" => match self.parsed_signature.next() {
                Some(SignatureEntry::ObjectPath) => {
                    self.common
                        .parse_padding(ObjectPath::alignment(Format::DBus))?;
                    let len = self.common.next_slice(4)?;
                    let len = self.common.ctxt.endian().read_u32(len) as usize;
                    let path = self.common.next_slice(len)?;
                    let _ = self.common.next_slice(1)?[0];
                    let path = str::from_utf8(path).map_err(Error::Utf8)?;
                    visitor.visit_borrowed_str(path)
                }
                Some(signature) => Err(Error::SignatureMismatch(
                    signature.into(),
                    "Expected an object path signature".into(),
                )),
                None => Err(Error::MissingSignature),
            },
            "zvariant::Signature" => match self.parsed_signature.next() {
                Some(SignatureEntry::Signature) => {
                    let len = self.common.next_slice(1)?[0];
                    let sig = self.common.next_slice(len as usize)?;
                    let _ = self.common.next_slice(1)?[0];
                    let sig = str::from_utf8(sig).map_err(Error::Utf8)?;
                    visitor.visit_borrowed_str(sig)
                }
                Some(signature) => Err(Error::SignatureMismatch(
                    signature.into(),
                    "Expected an object path signature".into(),
                )),
                None => Err(Error::MissingSignature),
            },
            _ => visitor.visit_newtype_struct(self),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Struct(fields)) => {
                self.common.parse_padding(STRUCT_ALIGNMENT_DBUS)?;

                let fields = fields.into_iter().collect();
                let struct_access = StructureAccess::new(fields, self)?;
                visitor.visit_seq(struct_access)
            }
            Some(SignatureEntry::Array(field)) => {
                let array_access = ArrayAccess::new(*field, self)?;
                visitor.visit_seq(array_access)
            }
            Some(SignatureEntry::Variant) => {
                let value_access = ValueAccess::new(self);
                visitor.visit_map(value_access)
            }
            Some(SignatureEntry::U8) => {
                let _ = self.common.next_slice(1)?[0];
                let struct_access = StructureAccess::new(VecDeque::new(), self)?;
                visitor.visit_seq(struct_access)
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a struct, array, or variant signature for seq".into(),
            )),
            None => Err(Error::MissingSignature),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Array(inner_signature)) => match *inner_signature {
                SignatureEntry::DictEntry(key_sig, value_sig) => {
                    let dict_access = DictAccess::new(*key_sig, *value_sig, self)?;
                    visitor.visit_map(dict_access)
                }
                sig => Err(Error::SignatureMismatch(
                    sig.into(),
                    "Expected a dict entry signature".into(),
                )),
            },
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected an array signature".into(),
            )),
            None => Err(Error::MissingSignature),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Struct(fields)) => {
                self.common.parse_padding(STRUCT_ALIGNMENT_DBUS)?;

                let fields = fields.into_iter().collect();
                let enum_access = StructureEnumAccess::new(fields, self)?;
                visitor.visit_enum(enum_access)
            }
            Some(SignatureEntry::U32) => {
                let enum_access =
                    StructureEnumAccess::new(VecDeque::from([SignatureEntry::U32]), self)?;
                visitor.visit_enum(enum_access)
            }
            Some(SignatureEntry::U8) => {
                let enum_access =
                    StructureEnumAccess::new(VecDeque::from([SignatureEntry::U8]), self)?;
                visitor.visit_enum(enum_access)
            }
            Some(SignatureEntry::Str) => {
                let enum_access =
                    StructureEnumAccess::new(VecDeque::from([SignatureEntry::Str]), self)?;
                visitor.visit_enum(enum_access)
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a struct or discriminant signature for enum".into(),
            )),
            None => Err(Error::MissingSignature),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.parsed_signature.next() {
            Some(SignatureEntry::Struct(fields)) => {
                self.common.parse_padding(STRUCT_ALIGNMENT_DBUS)?;

                let fields = fields.into_iter().collect();
                let struct_access = StructureAccess::new(fields, self)?;
                visitor.visit_seq(struct_access)
            }
            Some(SignatureEntry::Array(field)) => {
                let array_access = ArrayAccess::new(*field, self)?;
                visitor.visit_seq(array_access)
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a struct signature for tuple-struct".into(),
            )),
            None => Err(Error::MissingSignature),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.parsed_signature.next() {
            Some(SignatureEntry::U8) => {
                let _ = self.common.next_slice(1)?;
                visitor.visit_unit()
            }
            Some(signature) => Err(Error::SignatureMismatch(
                signature.into(),
                "Expected a unit signature".into(),
            )),
            None => visitor.visit_unit(),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn is_human_readable(&self) -> bool {
        false
    }

    forward_to_deserialize_any! {
        identifier
    }
}

/// The internal state of a `ValueAccess`
enum ValueAccessState {
    /// The signature is being read
    ReadingSignature,

    /// The value is being read
    ReadingValue,

    /// The deserialization is done and
    /// there are no more values to read
    Done,
}

/// Deserialize a `Value` from a DBus/GVariant variant.
struct ValueAccess<'d, 'de, 'f, #[cfg(unix)] F: AsFd> {
    de: &'d mut Deserializer<'de, 'f, F>,
    state: ValueAccessState,
    sig: Option<ParsedSignature>,
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> ValueAccess<'d, 'de, 'f, F> {
    pub fn new(de: &'d mut Deserializer<'de, 'f, F>) -> Self {
        Self {
            de,
            state: ValueAccessState::ReadingSignature,
            sig: None,
        }
    }
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> MapAccess<'de> for ValueAccess<'d, 'de, 'f, F> {
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.state {
            ValueAccessState::ReadingSignature => seed
                .deserialize("zvariant::Value::Signature".into_deserializer())
                .map(Some),
            ValueAccessState::ReadingValue => seed
                .deserialize("zvariant::Value::Value".into_deserializer())
                .map(Some),
            ValueAccessState::Done => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.state {
            ValueAccessState::ReadingSignature => {
                self.state = ValueAccessState::ReadingValue;

                let len = self.de.common.next_slice(1)?[0];
                let sig = self.de.common.next_slice(len as usize)?;
                let _ = self.de.common.next_slice(1)?[0];

                let sig = ParsedSignature::parse_bytes(sig)?;
                let result = seed.deserialize(sig.to_string().into_deserializer());
                self.sig = Some(sig);
                result
            }
            ValueAccessState::ReadingValue => {
                self.state = ValueAccessState::Done;

                let sig = self.sig.take().expect("Should have a signature by now");
                let (_, result) = self.de.inner_variant(sig, seed)?;
                Ok(result)
            }
            ValueAccessState::Done => unreachable!(),
        }
    }
}

/// Deserialize a sequence of values from a given `Array`.
struct ArrayAccess<'d, 'de, 'f, #[cfg(unix)] F: AsFd> {
    item_sig: SignatureEntry,
    de: &'d mut Deserializer<'de, 'f, F>,
    count: i64,
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> ArrayAccess<'d, 'de, 'f, F> {
    pub fn new(item_sig: SignatureEntry, de: &'d mut Deserializer<'de, 'f, F>) -> Result<Self> {
        de.common.parse_padding(u32::alignment(Format::DBus))?;
        let count = de.common.next_slice(4)?;
        let count = de.common.ctxt.endian().read_u32(count) as i64;

        de.common.parse_padding(item_sig.alignment(Format::DBus))?;

        Ok(Self {
            item_sig,
            de,
            count,
        })
    }

    #[cfg(feature = "option-as-array")]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> SeqAccess<'de> for ArrayAccess<'d, 'de, 'f, F> {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.count > 0 {
            let (len, result) = self.de.inner_array(self.item_sig.clone(), seed)?;
            self.count -= len as i64;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

/// Deserialize a map of values from a given `Dict`
struct DictAccess<'d, 'de, 'f, #[cfg(unix)] F: AsFd> {
    key_sig: SignatureEntry,
    value_sig: SignatureEntry,
    de: &'d mut Deserializer<'de, 'f, F>,
    count: i64,
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> DictAccess<'d, 'de, 'f, F> {
    pub fn new(
        key_sig: SignatureEntry,
        value_sig: SignatureEntry,
        de: &'d mut Deserializer<'de, 'f, F>,
    ) -> Result<Self> {
        de.common.parse_padding(u32::alignment(Format::DBus))?;

        let count = de.common.next_slice(4)?;
        let count = de.common.ctxt.endian().read_u32(count) as i64;

        de.common.parse_padding(STRUCT_ALIGNMENT_DBUS)?;

        Ok(Self {
            key_sig,
            value_sig,
            de,
            count,
        })
    }
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> MapAccess<'de> for DictAccess<'d, 'de, 'f, F> {
    type Error = crate::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        if self.count > 0 {
            self.count -= self.de.common.parse_padding(STRUCT_ALIGNMENT_DBUS)? as i64;
            let (len, result) = self.de.inner_array(self.key_sig.clone(), seed)?;
            self.count -= len as i64;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if self.count > 0 {
            self.count -=
                self.de
                    .common
                    .parse_padding(self.value_sig.alignment(Format::DBus))? as i64;

            let (len, result) = self.de.inner_array(self.value_sig.clone(), seed)?;
            self.count -= len as i64;
            Ok(result)
        } else {
            unreachable!("next_value_seed should not be called when count is 0")
        }
    }
}

/// Deserialize a given sequence of field signatures taken
/// from a `Structure` into a rust sequence
struct StructureAccess<'d, 'de, 'f, #[cfg(unix)] F: AsFd> {
    fields: VecDeque<SignatureEntry>,
    de: &'d mut Deserializer<'de, 'f, F>,
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> StructureAccess<'d, 'de, 'f, F> {
    pub fn new(
        fields: VecDeque<SignatureEntry>,
        de: &'d mut Deserializer<'de, 'f, F>,
    ) -> Result<Self> {
        Ok(Self { fields, de })
    }
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> SeqAccess<'de> for StructureAccess<'d, 'de, 'f, F> {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if let Some(item) = self.fields.pop_front() {
            let (_, result) = self.de.inner_struct(item, seed)?;
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

/// Deserialize a given sequence of field signatures from the stream
/// and provide them to a rust enum
struct StructureEnumAccess<'d, 'de, 'f, #[cfg(unix)] F: AsFd> {
    fields: VecDeque<SignatureEntry>,
    de: &'d mut Deserializer<'de, 'f, F>,
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> StructureEnumAccess<'d, 'de, 'f, F> {
    pub fn new(
        fields: VecDeque<SignatureEntry>,
        de: &'d mut Deserializer<'de, 'f, F>,
    ) -> Result<Self> {
        Ok(Self { fields, de })
    }
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> EnumAccess<'de> for StructureEnumAccess<'d, 'de, 'f, F> {
    type Error = crate::Error;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self)>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(name_field) = self.fields.pop_front() {
            let (_, result) = self.de.inner_struct(name_field, seed)?;
            Ok((result, self))
        } else {
            Err(crate::Error::MissingSignature)
        }
    }
}

impl<'d, 'de, 'f, #[cfg(unix)] F: AsFd> VariantAccess<'de> for StructureEnumAccess<'d, 'de, 'f, F> {
    type Error = crate::Error;

    fn unit_variant(mut self) -> Result<()> {
        if let Some(field) = self.fields.pop_front() {
            Err(crate::Error::SignatureMismatch(
                field.into(),
                "Expected a unit variant".into(),
            ))
        } else {
            Ok(())
        }
    }

    fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(field) = self.fields.pop_front() {
            let (_, result) = self.de.inner_struct(field, seed)?;
            Ok(result)
        } else {
            Err(crate::Error::MissingSignature)
        }
    }

    fn tuple_variant<V>(mut self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Some(SignatureEntry::Struct(fields)) = self.fields.pop_front() {
            let fields = fields.into_iter().collect();
            self.de.common.parse_padding(STRUCT_ALIGNMENT_DBUS)?;
            self.de.container_depths = self.de.container_depths.inc_structure()?;
            let result = visitor.visit_seq(StructureAccess::new(fields, self.de)?)?;
            self.de.container_depths = self.de.container_depths.dec_structure();
            Ok(result)
        } else {
            Err(crate::Error::MissingSignature)
        }
    }

    fn struct_variant<V>(
        mut self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> std::prelude::v1::Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(SignatureEntry::Struct(fields)) = self.fields.pop_front() {
            let fields = fields.into_iter().collect();
            self.de.common.parse_padding(STRUCT_ALIGNMENT_DBUS)?;
            self.de.container_depths = self.de.container_depths.inc_structure()?;
            let result = visitor.visit_seq(StructureAccess::new(fields, self.de)?)?;
            self.de.container_depths = self.de.container_depths.dec_structure();
            Ok(result)
        } else {
            Err(crate::Error::MissingSignature)
        }
    }
}
