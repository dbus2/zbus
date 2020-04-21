use crate::utils::*;
use crate::Signature;

pub trait Type {
    /// Get the signature for the implementing type.
    ///
    /// # Example
    ///
    /// TODO
    ///
    fn signature() -> Signature<'static>;
}

impl<T> Type for [T]
where
    T: Type,
{
    #[inline]
    fn signature() -> Signature<'static> {
        Signature::from(format!("a{}", T::signature().as_str()))
    }
}

impl<T> Type for &[T]
where
    T: Type,
{
    #[inline]
    fn signature() -> Signature<'static> {
        <[T]>::signature()
    }
}

impl<T> Type for Vec<T>
where
    T: Type,
{
    #[inline]
    fn signature() -> Signature<'static> {
        <[T]>::signature()
    }
}

// Empty type deserves empty signature
impl Type for () {
    #[inline]
    fn signature() -> Signature<'static> {
        Signature::from("")
    }
}

// TODO: implement when we support GVariant support
// impl<T> Type for Option<T>

////////////////////////////////////////////////////////////////////////////////

// Arrays are serialized as tupples by Serde and that's strange. Let's just not support it at all.
// TODO: Mention this fact in the module docs.

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> Type for ($($name,)+)
            where
                $($name: Type,)+
            {
                #[inline]
                fn signature() -> Signature<'static> {
                    let mut sig = String::with_capacity(255);
                    sig.push(STRUCT_SIG_START_CHAR);
                    $(
                        sig.push_str($name::signature().as_str());
                    )+
                    sig.push(STRUCT_SIG_END_CHAR);

                    Signature::from(sig)
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

////////////////////////////////////////////////////////////////////////////////

use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};

macro_rules! map_impl {
    ($ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(, $typaram:ident : $bound:ident)* >) => {
        impl<K, V $(, $typaram)*> Type for $ty<K, V $(, $typaram)*>
        where
            K: Type $(+ $kbound1 $(+ $kbound2)*)*,
            V: Type,
            $($typaram: $bound,)*
        {
            #[inline]
            fn signature() -> Signature<'static> {
                Signature::from(format!("a{{{}{}}}", K::signature().as_str(), V::signature().as_str()))
            }
        }
    }
}

map_impl!(BTreeMap<K: Ord, V>);
map_impl!(HashMap<K: Eq + Hash, V, H: BuildHasher>);

// BitFlags
#[cfg(feature = "enumflags2")]
impl<F> Type for enumflags2::BitFlags<F>
where
    F: Type + enumflags2::RawBitFlags,
{
    #[inline]
    fn signature() -> Signature<'static> {
        F::signature()
    }
}

// TODO: Blanket implementation for more types: https://github.com/serde-rs/serde/blob/master/serde/src/ser/impls.rs
