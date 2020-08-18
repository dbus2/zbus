use crate::Value;
use crate::{Array, Dict};
use crate::{Fd, ObjectPath, Signature, Structure};

//
// Conversions from encodable types to `Value`

macro_rules! into_value {
    ($from:ty, $kind:ident) => {
        impl<'a> From<$from> for Value<'a> {
            fn from(v: $from) -> Self {
                Value::$kind(v.into())
            }
        }

        impl<'a> From<&'a $from> for Value<'a> {
            fn from(v: &'a $from) -> Self {
                Value::from(v.clone())
            }
        }
    };
}

into_value!(u8, U8);
into_value!(i8, I16);
into_value!(bool, Bool);
into_value!(u16, U16);
into_value!(i16, I16);
into_value!(u32, U32);
into_value!(i32, I32);
into_value!(u64, U64);
into_value!(i64, I64);
into_value!(f32, F64);
into_value!(f64, F64);
into_value!(Fd, Fd);

into_value!(&'a str, Str);
into_value!(Signature<'a>, Signature);
into_value!(ObjectPath<'a>, ObjectPath);
into_value!(Array<'a>, Array);
into_value!(Dict<'a, 'a>, Dict);

impl<'v, 's: 'v, T> From<T> for Value<'v>
where
    T: Into<Structure<'s>>,
{
    fn from(v: T) -> Value<'v> {
        Value::Structure(v.into())
    }
}

impl<'v, V> From<&'v [V]> for Value<'v>
where
    &'v [V]: Into<Array<'v>>,
{
    fn from(v: &'v [V]) -> Value<'v> {
        Value::Array(v.into())
    }
}

impl<'v, V> From<Vec<V>> for Value<'v>
where
    Vec<V>: Into<Array<'v>>,
{
    fn from(v: Vec<V>) -> Value<'v> {
        Value::Array(v.into())
    }
}

impl<'v, V> From<&'v Vec<V>> for Value<'v>
where
    &'v Vec<V>: Into<Array<'v>>,
{
    fn from(v: &'v Vec<V>) -> Value<'v> {
        Value::Array(v.into())
    }
}

impl<'v> From<&'v String> for Value<'v> {
    fn from(v: &'v String) -> Value<'v> {
        Value::Str(v.into())
    }
}
