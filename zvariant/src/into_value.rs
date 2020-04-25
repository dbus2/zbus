use crate::{Array, Dict};
use crate::{ObjectPath, Signature, Structure};
use crate::{Type, Value};

//
// Conversions from encodable types to `Value`

macro_rules! into_value {
    ($from:ty, $kind:ident) => {
        impl<'a> From<$from> for Value<'a> {
            fn from(v: $from) -> Self {
                Value::$kind(v.into())
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

pub trait IntoValue<'v> {
    fn into_value(self) -> Value<'v>;
}

impl<'v> IntoValue<'v> for u8 {
    fn into_value(self) -> Value<'v> {
        Value::U8(self)
    }
}

impl<'v> IntoValue<'v> for i8 {
    fn into_value(self) -> Value<'v> {
        Value::I16(self as i16)
    }
}

impl<'v> IntoValue<'v> for bool {
    fn into_value(self) -> Value<'v> {
        Value::Bool(self)
    }
}

impl<'v> IntoValue<'v> for u16 {
    fn into_value(self) -> Value<'v> {
        Value::U16(self)
    }
}

impl<'v> IntoValue<'v> for i16 {
    fn into_value(self) -> Value<'v> {
        Value::I16(self)
    }
}

impl<'v> IntoValue<'v> for u32 {
    fn into_value(self) -> Value<'v> {
        Value::U32(self)
    }
}

impl<'v> IntoValue<'v> for i32 {
    fn into_value(self) -> Value<'v> {
        Value::I32(self)
    }
}

impl<'v> IntoValue<'v> for u64 {
    fn into_value(self) -> Value<'v> {
        Value::U64(self)
    }
}

impl<'v> IntoValue<'v> for i64 {
    fn into_value(self) -> Value<'v> {
        Value::I64(self)
    }
}

impl<'v> IntoValue<'v> for f32 {
    fn into_value(self) -> Value<'v> {
        Value::F64(self as f64)
    }
}

impl<'v> IntoValue<'v> for f64 {
    fn into_value(self) -> Value<'v> {
        Value::F64(self)
    }
}

impl<'v, 's: 'v> IntoValue<'v> for &'s str {
    fn into_value(self) -> Value<'v> {
        Value::Str(self)
    }
}

impl<'v, 's: 'v> IntoValue<'v> for Signature<'s> {
    fn into_value(self) -> Value<'v> {
        Value::Signature(self)
    }
}

impl<'v, 'o: 'v> IntoValue<'v> for ObjectPath<'o> {
    fn into_value(self) -> Value<'v> {
        Value::ObjectPath(self)
    }
}

// Value itself (deflatten)

impl<'v, 'a: 'v> IntoValue<'v> for Value<'a> {
    fn into_value(self) -> Value<'v> {
        Value::Value(Box::new(self))
    }
}

impl<'v, 'a: 'v> IntoValue<'v> for Array<'a> {
    fn into_value(self) -> Value<'v> {
        Value::Array(self)
    }
}

impl<'v, 'd: 'v> IntoValue<'v> for Dict<'d, 'd> {
    fn into_value(self) -> Value<'v> {
        Value::Dict(self)
    }
}

impl<'v, 's: 'v, S> IntoValue<'v> for S
where
    S: Into<Structure<'s>>,
{
    fn into_value(self) -> Value<'v> {
        Value::Structure(self.into())
    }
}

impl<'v, V> IntoValue<'v> for &[V]
where
    V: Type + IntoValue<'v> + Clone,
{
    fn into_value(self) -> Value<'v> {
        Value::Array(Array::from(self))
    }
}

impl<'v, V> IntoValue<'v> for Vec<V>
where
    V: Type + IntoValue<'v> + Clone,
{
    fn into_value(self) -> Value<'v> {
        Value::Array(Array::from(self))
    }
}
