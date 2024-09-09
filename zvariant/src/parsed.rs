use std::str::FromStr;

use crate::{Basic, Type};

pub use zvariant_utils::parsed::*;

impl From<Error> for crate::Error {
    fn from(e: Error) -> Self {
        crate::Error::SignatureParse(e)
    }
}

impl From<crate::Signature<'_>> for Signature {
    fn from(value: crate::Signature<'_>) -> Self {
        Self::from_str(value.as_str()).expect("valid signature")
    }
}

impl PartialEq<crate::Signature<'_>> for Signature {
    fn eq(&self, other: &crate::Signature<'_>) -> bool {
        self.eq(other.as_str())
    }
}

impl PartialEq<crate::OwnedSignature> for Signature {
    fn eq(&self, other: &crate::OwnedSignature) -> bool {
        self.eq(other.as_str())
    }
}

impl From<&Signature> for crate::Signature<'static> {
    fn from(value: &Signature) -> Self {
        match value {
            Signature::Unit => crate::Signature::from_static_str_unchecked(""),
            Signature::U8 => crate::Signature::from_static_str_unchecked("y"),
            Signature::Bool => crate::Signature::from_static_str_unchecked("b"),
            Signature::I16 => crate::Signature::from_static_str_unchecked("n"),
            Signature::U16 => crate::Signature::from_static_str_unchecked("q"),
            Signature::I32 => crate::Signature::from_static_str_unchecked("i"),
            Signature::U32 => crate::Signature::from_static_str_unchecked("u"),
            Signature::I64 => crate::Signature::from_static_str_unchecked("x"),
            Signature::U64 => crate::Signature::from_static_str_unchecked("t"),
            Signature::F64 => crate::Signature::from_static_str_unchecked("d"),
            Signature::Str => crate::Signature::from_static_str_unchecked("s"),
            Signature::Signature => crate::Signature::from_static_str_unchecked("g"),
            Signature::ObjectPath => crate::Signature::from_static_str_unchecked("o"),
            Signature::Variant => crate::Signature::from_static_str_unchecked("v"),
            #[cfg(unix)]
            Signature::Fd => crate::Signature::from_static_str_unchecked("h"),
            container_signature => {
                crate::Signature::from_string_unchecked(container_signature.to_string())
            }
        }
    }
}

impl From<Signature> for crate::Signature<'static> {
    fn from(value: Signature) -> Self {
        Self::from(&value)
    }
}

impl Type for Signature {
    const SIGNATURE: &'static Signature = &Signature::Signature;
}

impl Basic for Signature {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
}

impl From<Signature> for crate::Value<'static> {
    fn from(value: Signature) -> Self {
        crate::Value::Signature(value.into())
    }
}
