use crate::{Array, Dict};
use crate::{ObjectPath, Signature, Structure};
use crate::{Type, Value};

//
// Conversions from encodable types to `Value`
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
