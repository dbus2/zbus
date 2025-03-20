use crate::{impl_type_with_repr, Signature, Type};
use std::{
    cell::{Cell, RefCell},
    cmp::Reverse,
    marker::PhantomData,
    num::{Saturating, Wrapping},
    ops::{Range, RangeFrom, RangeInclusive, RangeTo},
    rc::{Rc, Weak as RcWeak},
    sync::{
        atomic::{
            AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16,
            AtomicU32, AtomicU64, AtomicU8, AtomicUsize,
        },
        Arc, Mutex, RwLock, Weak as ArcWeak,
    },
    time::Duration,
};

impl<T> Type for PhantomData<T>
where
    T: Type + ?Sized,
{
    const SIGNATURE: &'static Signature = T::SIGNATURE;
}

macro_rules! array_type {
    ($arr:ty) => {
        impl<T> Type for $arr
        where
            T: Type,
        {
            const SIGNATURE: &'static Signature = &Signature::static_array(T::SIGNATURE);
        }
    };
}

array_type!([T]);
array_type!(Vec<T>);
array_type!(std::collections::VecDeque<T>);
array_type!(std::collections::LinkedList<T>);

impl<T, S> Type for std::collections::HashSet<T, S>
where
    T: Type + Eq + Hash,
    S: BuildHasher,
{
    const SIGNATURE: &'static Signature = <[T]>::SIGNATURE;
}

impl<T> Type for std::collections::BTreeSet<T>
where
    T: Type + Ord,
{
    const SIGNATURE: &'static Signature = <[T]>::SIGNATURE;
}

impl<T> Type for std::collections::BinaryHeap<T>
where
    T: Type + Ord,
{
    const SIGNATURE: &'static Signature = <[T]>::SIGNATURE;
}

#[cfg(feature = "smartstring")]
impl<T> Type for smartstring::SmartString<T>
where
    T: smartstring::SmartStringMode,
{
    const SIGNATURE: &'static Signature = String::SIGNATURE;
}

#[cfg(feature = "arrayvec")]
impl<T, const CAP: usize> Type for arrayvec::ArrayVec<T, CAP>
where
    T: Type,
{
    const SIGNATURE: &'static Signature = <[T]>::SIGNATURE;
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Type for arrayvec::ArrayString<CAP> {
    const SIGNATURE: &'static Signature = &Signature::Str;
}

#[cfg(feature = "heapless")]
impl<T, const CAP: usize> Type for heapless::Vec<T, CAP>
where
    T: Type,
{
    const SIGNATURE: &'static Signature = <[T]>::SIGNATURE;
}

#[cfg(feature = "heapless")]
impl<const CAP: usize> Type for heapless::String<CAP> {
    const SIGNATURE: &'static Signature = &Signature::Str;
}

// Empty type deserves empty signature
impl Type for () {
    const SIGNATURE: &'static Signature = &Signature::Unit;
}

macro_rules! deref_impl {
    (
        $type:ty,
        <$($desc:tt)+
    ) => {
        impl <$($desc)+ {
            const SIGNATURE: &'static Signature = <$type>::SIGNATURE;
        }
    };
}

deref_impl!(T, <T: ?Sized + Type> Type for &T);
deref_impl!(T, <T: ?Sized + Type> Type for &mut T);
deref_impl!(T, <T: ?Sized + Type + ToOwned> Type for Cow<'_, T>);
deref_impl!(T, <T: ?Sized + Type> Type for Arc<T>);
deref_impl!(T, <T: ?Sized + Type> Type for ArcWeak<T>);
deref_impl!(T, <T: ?Sized + Type> Type for Mutex<T>);
deref_impl!(T, <T: ?Sized + Type> Type for RwLock<T>);
deref_impl!(T, <T: ?Sized + Type> Type for Box<T>);
deref_impl!(T, <T: ?Sized + Type> Type for Rc<T>);
deref_impl!(T, <T: ?Sized + Type> Type for RcWeak<T>);
deref_impl!(T, <T: ?Sized + Type> Type for Cell<T>);
deref_impl!(T, <T: ?Sized + Type> Type for RefCell<T>);

#[cfg(all(feature = "gvariant", not(feature = "option-as-array")))]
impl<T> Type for Option<T>
where
    T: Type,
{
    const SIGNATURE: &'static Signature = &Signature::static_maybe(T::SIGNATURE);
}

#[cfg(feature = "option-as-array")]
impl<T> Type for Option<T>
where
    T: Type,
{
    const SIGNATURE: &'static Signature = &Signature::static_array(T::SIGNATURE);
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> Type for ($($name,)+)
            where
                $($name: Type,)+
            {
                const SIGNATURE: &'static Signature =
                    &Signature::static_structure(&[
                        $(
                            $name::SIGNATURE,
                        )+
                    ]);
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

// Arrays are serialized as tuples/structs by Serde so we treat them as such too even though
// it's very strange. Slices and arrayvec::ArrayVec can be used anyway so I guess it's no big
// deal.
impl<T, const N: usize> Type for [T; N]
where
    T: Type,
{
    const SIGNATURE: &'static Signature = &Signature::static_structure(&[T::SIGNATURE; N]);
}

////////////////////////////////////////////////////////////////////////////////

use std::{
    borrow::Cow,
    collections::{BTreeMap, HashMap},
    hash::{BuildHasher, Hash},
};

macro_rules! map_impl {
    ($ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(, $typaram:ident : $bound:ident)* >) => {
        impl<K, V $(, $typaram)*> Type for $ty<K, V $(, $typaram)*>
        where
            K: Type $(+ $kbound1 $(+ $kbound2)*)*,
            V: Type,
            $($typaram: $bound,)*
        {
            const SIGNATURE: &'static Signature =
                &Signature::static_dict(K::SIGNATURE, V::SIGNATURE);
        }
    }
}

map_impl!(BTreeMap<K: Ord, V>);
map_impl!(HashMap<K: Eq + Hash, V, H: BuildHasher>);

////////////////////////////////////////////////////////////////////////////////

impl_type_with_repr! {
    // usize is serialized as u64:
    // https://github.com/serde-rs/serde/blob/9b868ef831c95f50dd4bde51a7eb52e3b9ee265a/serde/src/ser/impls.rs#L28
    usize => u64 {
        usize {
            samples = [usize::MAX, usize::MIN],
            repr(n) = n as u64,
        }
    }
}

impl_type_with_repr! {
    // isize is serialized as i64:
    // https://github.com/serde-rs/serde/blob/9b868ef831c95f50dd4bde51a7eb52e3b9ee265a/serde/src/ser/impls.rs#L22
    isize => i64 {
        isize {
            samples = [isize::MAX, isize::MIN],
            repr(n) = n as i64,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

impl_type_with_repr! {
    Duration => (u64, u32) {
        duration {
            samples = [Duration::ZERO, Duration::MAX],
            repr(d) = (d.as_secs(), d.subsec_nanos()),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! impl_type_for_wrapper {
    ($($wrapper:ident<$T:ident>),+) => {
        $(
            impl<$T: Type> Type for $wrapper<$T> {
                const SIGNATURE: &'static Signature = <$T>::SIGNATURE;
            }
        )+
    };
}

impl_type_for_wrapper!(Wrapping<T>, Saturating<T>, Reverse<T>);

////////////////////////////////////////////////////////////////////////////////

macro_rules! atomic_impl {
    ($($ty:ident $size:expr => $primitive:ident)*) => {
        $(
            #[cfg(target_has_atomic = $size)]
            impl Type for $ty {
                const SIGNATURE: &'static Signature = <$primitive as Type>::SIGNATURE;
            }
        )*
    }
}

atomic_impl! {
    AtomicBool "8" => bool
    AtomicI8 "8" => i8
    AtomicI16 "16" => i16
    AtomicI32 "32" => i32
    AtomicIsize "ptr" => isize
    AtomicI64 "64" => i64
    AtomicU8 "8" => u8
    AtomicU16 "16" => u16
    AtomicU32 "32" => u32
    AtomicU64 "64" => u64
    AtomicUsize "ptr" => usize
}

////////////////////////////////////////////////////////////////////////////////

impl_type_with_repr! {
    Range<Idx: Type> => (Idx, Idx) {
        range <Idx = u32> {
            samples = [0..42, 17..100],
            repr(range) = (range.start, range.end),
        }
    }
}

impl_type_with_repr! {
    RangeFrom<Idx: Type> => (Idx,) {
        range_from <Idx = u32> {
            samples = [0.., 17..],
            repr(range) = (range.start,),
        }
    }
}

impl_type_with_repr! {
    RangeInclusive<Idx: Type> => (Idx, Idx) {
        range_inclusive <Idx = u32> {
            samples = [0..=42, 17..=100],
            repr(range) = (*range.start(), *range.end()),
        }
    }
}

impl_type_with_repr! {
    RangeTo<Idx: Type> => (Idx,) {
        range_to <Idx = u32> {
            samples = [..42, ..100],
            repr(range) = (range.end,),
        }
    }
}

// serde::Serialize is not implemented for `RangeToInclusive` and `RangeFull`:
// https://github.com/serde-rs/serde/issues/2685

// TODO: Blanket implementation for more types: https://github.com/serde-rs/serde/blob/master/serde/src/ser/impls.rs
