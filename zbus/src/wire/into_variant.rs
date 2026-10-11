use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    hash::BuildHasher,
    sync::Arc,
};

use crate::wire::{
    Array, Basic, Dict, NoneValue, ObjectPath, Optional, Str, Structure, Type, Variant,
};

#[cfg(unix)]
use crate::Fd;

//
// Conversions from encodable types to `Variant`

macro_rules! into_value {
    ($from:ty, $kind:ident) => {
        impl<'a> From<$from> for Variant<'a> {
            fn from(v: $from) -> Self {
                Variant::$kind(v.into())
            }
        }
    };
}

macro_rules! into_value_from_ref {
    ($from:ty, $kind:ident) => {
        impl<'a> From<&'a $from> for Variant<'a> {
            fn from(v: &'a $from) -> Self {
                Variant::$kind(v.clone().into())
            }
        }
    };
}

macro_rules! into_value_from_both {
    ($from:ty, $kind:ident) => {
        into_value!($from, $kind);
        into_value_from_ref!($from, $kind);
    };
}

into_value_from_both!(u8, U8);
into_value_from_both!(i8, I16);
into_value_from_both!(bool, Bool);
into_value_from_both!(u16, U16);
into_value_from_both!(i16, I16);
into_value_from_both!(u32, U32);
into_value_from_both!(i32, I32);
into_value_from_both!(u64, U64);
into_value_from_both!(i64, I64);
into_value_from_both!(f32, F64);
into_value_from_both!(f64, F64);

into_value!(Arc<str>, Str);
into_value!(Cow<'a, str>, Str);
into_value_from_both!(String, Str);

into_value_from_both!(&'a str, Str);
into_value_from_both!(Str<'a>, Str);
into_value_from_both!(ObjectPath<'a>, ObjectPath);

macro_rules! try_into_value_from_ref {
    ($from:ty, $kind:ident) => {
        impl<'a> TryFrom<&'a $from> for Variant<'a> {
            type Error = crate::Error;

            fn try_from(v: &'a $from) -> crate::Result<Self> {
                v.try_clone().map(Variant::$kind)
            }
        }
    };
}

into_value!(Array<'a>, Array);
try_into_value_from_ref!(Array<'a>, Array);
into_value!(Dict<'a, 'a>, Dict);
try_into_value_from_ref!(Dict<'a, 'a>, Dict);
#[cfg(unix)]
into_value!(Fd<'a>, Fd);
#[cfg(unix)]
try_into_value_from_ref!(Fd<'a>, Fd);

impl<'v, 's: 'v, T> From<T> for Variant<'v>
where
    T: Into<Structure<'s>>,
{
    fn from(v: T) -> Variant<'v> {
        Variant::Structure(v.into())
    }
}

impl<'b, 'v, V> From<&'b [V]> for Variant<'v>
where
    &'b [V]: Into<Array<'v>>,
{
    fn from(v: &'b [V]) -> Variant<'v> {
        Variant::Array(v.into())
    }
}

impl<'v, V> From<Vec<V>> for Variant<'v>
where
    Vec<V>: Into<Array<'v>>,
{
    fn from(v: Vec<V>) -> Variant<'v> {
        Variant::Array(v.into())
    }
}

impl<'b, 'v, V> From<&'b Vec<V>> for Variant<'v>
where
    &'b Vec<V>: Into<Array<'v>>,
{
    fn from(v: &'b Vec<V>) -> Variant<'v> {
        Variant::Array(v.into())
    }
}

impl<'a, 'k, 'v, K, V> From<BTreeMap<K, V>> for Variant<'a>
where
    'k: 'a,
    'v: 'a,
    K: Basic + Into<Variant<'k>> + std::cmp::Ord,
    V: Type + Into<Variant<'v>>,
{
    fn from(value: BTreeMap<K, V>) -> Self {
        Self::Dict(value.into())
    }
}

impl<'a, 'k, 'v, K, V, H> From<HashMap<K, V, H>> for Variant<'a>
where
    'k: 'a,
    'v: 'a,
    K: Basic + Into<Variant<'k>> + std::hash::Hash + std::cmp::Eq,
    V: Type + Into<Variant<'v>>,
    H: BuildHasher + Default,
{
    fn from(value: HashMap<K, V, H>) -> Self {
        Self::Dict(value.into())
    }
}

impl<'v, V> From<Optional<V>> for Variant<'v>
where
    V: Into<Variant<'v>> + NoneValue<NoneType = V>,
{
    fn from(v: Optional<V>) -> Variant<'v> {
        Option::<V>::from(v)
            .unwrap_or_else(|| V::null_value())
            .into()
    }
}

#[cfg(feature = "option-as-array")]
impl<'v, V> From<Option<V>> for Variant<'v>
where
    V: Into<Variant<'v>> + Type,
{
    fn from(v: Option<V>) -> Variant<'v> {
        let mut array = Array::new(V::SIGNATURE);
        if let Some(v) = v {
            // We got the signature from the `Type` impl, so this should never panic.
            array.append(v.into()).expect("signature mismatch");
        }

        array.into()
    }
}
