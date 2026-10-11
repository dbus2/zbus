use crate::{
    Error,
    wire::{
        Array, Dict, NoneValue, ObjectPath, Optional, OwnedObjectPath, Signature, Str, Structure,
        Variant,
    },
};

#[cfg(unix)]
use crate::Fd;

use std::{
    collections::{BTreeMap, HashMap},
    hash::BuildHasher,
};

macro_rules! value_try_from {
    ($kind:ident, $to:ty) => {
        impl<'a> TryFrom<Variant<'a>> for $to {
            type Error = Error;

            fn try_from(value: Variant<'a>) -> Result<Self, Self::Error> {
                if let Variant::$kind(value) = value {
                    Ok(value.into())
                } else {
                    Err(Error::IncorrectType)
                }
            }
        }
    };
}

macro_rules! value_try_from_ref {
    ($kind:ident, $to:ty) => {
        impl<'a> TryFrom<&'a Variant<'_>> for &'a $to {
            type Error = Error;

            fn try_from(value: &'a Variant<'_>) -> Result<Self, Self::Error> {
                if let Variant::$kind(value) = value {
                    Ok(value)
                } else {
                    Err(Error::IncorrectType)
                }
            }
        }
    };
}

macro_rules! value_try_from_ref_clone {
    ($kind:ident, $to:ty) => {
        impl<'a> TryFrom<&Variant<'a>> for $to {
            type Error = Error;

            fn try_from(value: &Variant<'a>) -> Result<Self, Self::Error> {
                if let Variant::$kind(value) = value {
                    Ok(value.clone().into())
                } else {
                    Err(Error::IncorrectType)
                }
            }
        }
    };
}

macro_rules! value_try_from_all {
    ($from:ident, $to:ty) => {
        value_try_from!($from, $to);
        value_try_from_ref!($from, $to);
        value_try_from_ref_clone!($from, $to);
    };
}

value_try_from_all!(U8, u8);
value_try_from_all!(Bool, bool);
value_try_from_all!(I16, i16);
value_try_from_all!(U16, u16);
value_try_from_all!(I32, i32);
value_try_from_all!(U32, u32);
value_try_from_all!(I64, i64);
value_try_from_all!(U64, u64);
value_try_from_all!(F64, f64);

value_try_from_all!(Str, Str<'a>);
value_try_from_all!(Signature, Signature);
value_try_from_all!(ObjectPath, ObjectPath<'a>);
value_try_from!(Str, String);
value_try_from_ref!(Str, str);

macro_rules! value_try_from_ref_try_clone {
    ($kind:ident, $to:ty) => {
        impl<'a> TryFrom<&Variant<'a>> for $to {
            type Error = Error;

            fn try_from(value: &Variant<'a>) -> Result<Self, Self::Error> {
                if let Variant::$kind(value) = value {
                    value.try_clone().map_err(Into::into)
                } else {
                    Err(Error::IncorrectType)
                }
            }
        }
    };
}

value_try_from!(Structure, Structure<'a>);
value_try_from_ref!(Structure, Structure<'a>);
value_try_from_ref_try_clone!(Structure, Structure<'a>);

value_try_from!(Dict, Dict<'a, 'a>);
value_try_from_ref!(Dict, Dict<'a, 'a>);
value_try_from_ref_try_clone!(Dict, Dict<'a, 'a>);

value_try_from!(Array, Array<'a>);
value_try_from_ref!(Array, Array<'a>);
value_try_from_ref_try_clone!(Array, Array<'a>);

#[cfg(unix)]
value_try_from!(Fd, Fd<'a>);
#[cfg(unix)]
value_try_from_ref!(Fd, Fd<'a>);
#[cfg(unix)]
value_try_from_ref_try_clone!(Fd, Fd<'a>);

impl TryFrom<&Variant<'_>> for String {
    type Error = Error;

    fn try_from(value: &Variant<'_>) -> Result<Self, Self::Error> {
        Ok(<&str>::try_from(value)?.into())
    }
}

impl<'a, T> TryFrom<Variant<'a>> for Vec<T>
where
    T: TryFrom<Variant<'a>>,
    T::Error: Into<crate::Error>,
{
    type Error = Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Self::Error> {
        if let Variant::Array(v) = value {
            Self::try_from(v)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

impl TryFrom<Variant<'_>> for OwnedObjectPath {
    type Error = Error;

    fn try_from(value: Variant<'_>) -> Result<Self, Self::Error> {
        ObjectPath::try_from(value).map(OwnedObjectPath::from)
    }
}

// tuple conversions in `structure` module for avoiding code-duplication.

impl<'a, K, V> TryFrom<Variant<'a>> for BTreeMap<K, V>
where
    K: crate::Basic + TryFrom<Variant<'a>> + std::cmp::Ord,
    V: TryFrom<Variant<'a>>,
    K::Error: Into<crate::wire::Error>,
    V::Error: Into<crate::wire::Error>,
{
    type Error = crate::wire::Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Self::Error> {
        if let Variant::Dict(v) = value {
            Self::try_from(v)
        } else {
            Err(crate::wire::Error::IncorrectType)
        }
    }
}

impl<'a, K, V, H> TryFrom<Variant<'a>> for HashMap<K, V, H>
where
    K: crate::Basic + TryFrom<Variant<'a>> + std::hash::Hash + std::cmp::Eq,
    V: TryFrom<Variant<'a>>,
    H: BuildHasher + Default,
    K::Error: Into<crate::Error>,
    V::Error: Into<crate::Error>,
{
    type Error = crate::Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Self::Error> {
        if let Variant::Dict(v) = value {
            Self::try_from(v)
        } else {
            Err(crate::Error::IncorrectType)
        }
    }
}

impl<'a, T> TryFrom<Variant<'a>> for Optional<T>
where
    T: TryFrom<Variant<'a>> + NoneValue,
    <T as NoneValue>::NoneType: Into<Variant<'a>>,
    T::Error: Into<crate::Error>,
{
    type Error = crate::Error;

    fn try_from(value: Variant<'a>) -> Result<Self, Self::Error> {
        // Check for the null sentinel before converting, since `T`'s conversion may
        // legitimately reject it (e.g. name types validating the empty string).
        if value == T::null_value().into() {
            Ok(Optional::from(None))
        } else {
            T::try_from(value)
                .map(|value| Optional::from(Some(value)))
                .map_err(Into::into)
        }
    }
}

// This would be great but somehow it conflicts with some blanket generic implementations from
// core:
//
// impl<'a, T> TryFrom<Variant<'a>> for Option<T>
//
// TODO: this could be useful
// impl<'a, 'b, T> TryFrom<&'a Variant<'b>> for Vec<T>
// impl<'a, 'b, K, V, H> TryFrom<&'a Variant<'v>> for HashMap<K, V, H>
// and more..
