use crate::{Array, Dict};
use crate::{ObjectPath, Signature, Structure};
use crate::{Variant, VariantValue};

//
// Conversions from encodable types to `Variant`
pub trait IntoVariant<'v> {
    fn into_variant(self) -> Variant<'v>;
}

impl<'v> IntoVariant<'v> for u8 {
    fn into_variant(self) -> Variant<'v> {
        Variant::U8(self)
    }
}

impl<'v> IntoVariant<'v> for i8 {
    fn into_variant(self) -> Variant<'v> {
        Variant::I16(self as i16)
    }
}

impl<'v> IntoVariant<'v> for bool {
    fn into_variant(self) -> Variant<'v> {
        Variant::Bool(self)
    }
}

impl<'v> IntoVariant<'v> for u16 {
    fn into_variant(self) -> Variant<'v> {
        Variant::U16(self)
    }
}

impl<'v> IntoVariant<'v> for i16 {
    fn into_variant(self) -> Variant<'v> {
        Variant::I16(self)
    }
}

impl<'v> IntoVariant<'v> for u32 {
    fn into_variant(self) -> Variant<'v> {
        Variant::U32(self)
    }
}

impl<'v> IntoVariant<'v> for i32 {
    fn into_variant(self) -> Variant<'v> {
        Variant::I32(self)
    }
}

impl<'v> IntoVariant<'v> for u64 {
    fn into_variant(self) -> Variant<'v> {
        Variant::U64(self)
    }
}

impl<'v> IntoVariant<'v> for i64 {
    fn into_variant(self) -> Variant<'v> {
        Variant::I64(self)
    }
}

impl<'v> IntoVariant<'v> for f32 {
    fn into_variant(self) -> Variant<'v> {
        Variant::F64(self as f64)
    }
}

impl<'v> IntoVariant<'v> for f64 {
    fn into_variant(self) -> Variant<'v> {
        Variant::F64(self)
    }
}

impl<'v, 's: 'v> IntoVariant<'v> for &'s str {
    fn into_variant(self) -> Variant<'v> {
        Variant::Str(self)
    }
}

impl<'v> IntoVariant<'v> for String {
    fn into_variant(self) -> Variant<'v> {
        Variant::String(self)
    }
}

impl<'v, 's: 'v> IntoVariant<'v> for Signature<'s> {
    fn into_variant(self) -> Variant<'v> {
        Variant::Signature(self)
    }
}

impl<'v, 'o: 'v> IntoVariant<'v> for ObjectPath<'o> {
    fn into_variant(self) -> Variant<'v> {
        Variant::ObjectPath(self)
    }
}

// Variant itself (deflatten)

impl<'v, 'a: 'v> IntoVariant<'v> for Variant<'a> {
    fn into_variant(self) -> Variant<'v> {
        Variant::Variant(Box::new(self))
    }
}

impl<'v, 'a: 'v> IntoVariant<'v> for Array<'a> {
    fn into_variant(self) -> Variant<'v> {
        Variant::Array(self)
    }
}

impl<'v, 'd: 'v> IntoVariant<'v> for Dict<'d, 'd> {
    fn into_variant(self) -> Variant<'v> {
        Variant::Dict(self)
    }
}

impl<'v, 's: 'v, S> IntoVariant<'v> for S
where
    S: Into<Structure<'s>>,
{
    fn into_variant(self) -> Variant<'v> {
        Variant::Structure(self.into())
    }
}

impl<'v, V> IntoVariant<'v> for &[V]
where
    V: VariantValue + IntoVariant<'v> + Clone,
{
    fn into_variant(self) -> Variant<'v> {
        Variant::Array(Array::from(self))
    }
}

impl<'v, V> IntoVariant<'v> for Vec<V>
where
    V: VariantValue + IntoVariant<'v> + Clone,
{
    fn into_variant(self) -> Variant<'v> {
        Variant::Array(Array::from(self))
    }
}
