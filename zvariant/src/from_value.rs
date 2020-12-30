#[cfg(feature = "gvariant")]
use crate::Maybe;
use crate::{Array, Dict, Error, Fd, ObjectPath, Signature, Str, Structure, Value};
use std::convert::TryFrom;

macro_rules! value_try_from {
    ($kind:ident, $to:ty) => {
        impl<'a> TryFrom<Value<'a>> for $to {
            type Error = Error;

            fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
                if let Value::$kind(value) = value {
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
        impl<'a> TryFrom<&'a Value<'a>> for &'a $to {
            type Error = Error;

            fn try_from(value: &'a Value<'a>) -> Result<Self, Self::Error> {
                if let Value::$kind(value) = value {
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
        impl<'a> TryFrom<&'a Value<'a>> for $to {
            type Error = Error;

            fn try_from(value: &'a Value<'_>) -> Result<Self, Self::Error> {
                if let Value::$kind(value) = value {
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
value_try_from_all!(Fd, Fd);

value_try_from_all!(Str, Str<'a>);
value_try_from_all!(Signature, Signature<'a>);
value_try_from_all!(ObjectPath, ObjectPath<'a>);
value_try_from_all!(Structure, Structure<'a>);
value_try_from_all!(Dict, Dict<'a, 'a>);
value_try_from_all!(Array, Array<'a>);
#[cfg(feature = "gvariant")]
value_try_from_all!(Maybe, Maybe<'a>);

value_try_from!(Str, String);
value_try_from_ref!(Str, str);
value_try_from_ref_clone!(Str, String);

impl<'a, T> TryFrom<Value<'a>> for Vec<T>
where
    T: TryFrom<Value<'a>, Error = Error> + 'a,
{
    type Error = Error;

    fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
        if let Value::Array(v) = value {
            Self::try_from(v)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// This would be great but somehow it conflicts with some blanket generic implementations from
// core:
//
// impl<'a, T> TryFrom<Value<'a>> for Option<T>
//
// TODO: this could be useful
// impl<'a, 'b, T> TryFrom<&'a Value<'b>> for Vec<T>
// impl<'a, K, V, H> TryFrom<Value<'a> for HashMap<K, V, H>
// impl<'a, 'b, K, V, H> TryFrom<&'a Value<'v>> for HashMap<K, V, H>
// and more..
