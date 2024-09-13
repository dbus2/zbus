use crate::Signature;
use serde::de::{Deserialize, DeserializeSeed};
use std::{
    cell::{Cell, RefCell},
    cmp::Reverse,
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6},
    num::{Saturating, Wrapping},
    ops::{Range, RangeFrom, RangeInclusive, RangeTo},
    path::{Path, PathBuf},
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

/// Trait implemented by all serializable types.
///
/// This very simple trait provides the signature for the implementing type. Since the [D-Bus type
/// system] relies on these signatures, our [serialization and deserialization] API requires this
/// trait in addition to [`Serialize`] and [`Deserialize`], respectively.
///
/// Implementation is provided for all the [basic types] and blanket implementations for common
/// container types, such as, arrays, slices, tuples, [`Vec`] and [`HashMap`]. For easy
/// implementation for custom types, use `Type` derive macro from [zvariant_derive] crate.
///
/// If your type's signature cannot be determined statically, you should implement the
/// [DynamicType] trait instead, which is otherwise automatically implemented if you implement this
/// trait.
///
/// [D-Bus type system]: https://dbus.freedesktop.org/doc/dbus-specification.html#type-system
/// [serialization and deserialization]: index.html#functions
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [basic types]: trait.Basic.html
/// [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
/// [zvariant_derive]: https://docs.rs/zvariant_derive/latest/zvariant_derive/
pub trait Type {
    /// The signature for the implementing type, in parsed format.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use zvariant::{Type, {ChildSignature, Signature}};
    ///
    /// assert_eq!(u32::SIGNATURE, &Signature::U32);
    /// assert_eq!(String::SIGNATURE, &Signature::Str);
    /// assert_eq!(
    ///     <(u32, &str, u64)>::SIGNATURE,
    ///     &Signature::static_structure(&[&Signature::U32, &Signature::Str, &Signature::U64]),
    /// );
    /// assert_eq!(
    ///     <(u32, &str, &[u64])>::SIGNATURE,
    ///     &Signature::static_structure(&[
    ///         &Signature::U32,
    ///         &Signature::Str,
    ///         &Signature::Array(ChildSignature::Static { child: &Signature::U64 }),
    ///     ]),
    /// );
    /// assert_eq!(
    ///     <HashMap<u8, &str>>::SIGNATURE,
    ///     &Signature::static_dict(&Signature::U8, &Signature::Str),
    /// );
    /// ```
    const SIGNATURE: &'static Signature;
}

/// Types with dynamic signatures.
///
/// Prefer implementing [Type] if possible, but if the actual signature of your type cannot be
/// determined until runtime, you can implement this type to support serialization.  You should
/// also implement [DynamicDeserialize] for deserialization.
pub trait DynamicType {
    /// The type signature for `self`.
    ///
    /// See [`Type::SIGNATURE`] for details.
    fn dynamic_signature(&self) -> Signature;
}

/// Types that deserialize based on dynamic signatures.
///
/// Prefer implementing [Type] and [Deserialize] if possible, but if the actual signature of your
/// type cannot be determined until runtime, you should implement this type to support
/// deserialization given a signature.
pub trait DynamicDeserialize<'de>: DynamicType {
    /// A [DeserializeSeed] implementation for this type.
    type Deserializer: DeserializeSeed<'de, Value = Self> + DynamicType;

    /// Get a deserializer compatible with this parsed signature.
    fn deserializer_for_signature(signature: &Signature) -> zvariant::Result<Self::Deserializer>;
}

/// Implements the [`Type`] trait by delegating the signature to a simpler type (usually a tuple).
/// Tests that ensure that the two types are serialize-compatible are auto-generated.
///
/// Example:
/// ```no_compile
/// impl_type_with_repr! {
///    // Duration is serialized as a (u64, u32) pair.
///    Duration => (u64, u32) {
///        // The macro auto-generates tests for us,
///        // so we need to provide a test name.
///        duration {
///            // Sample values used to test serialize compatibility.
///            samples = [Duration::ZERO, Duration::MAX],
///            // Converts our type into the simpler "repr" type.
///            repr(d) = (d.as_secs(), d.subsec_nanos()),
///        }
///    }
/// }
/// ```
macro_rules! impl_type_with_repr {
    ($($ty:ident)::+ $(<$typaram:ident $(: $($tbound:ident)::+)?>)? => $repr:ty {
        $test_mod:ident $(<$($typaram_sample:ident = $typaram_sample_value:ty),*>)? {
            $(signature = $signature:literal,)?
            samples = $samples:expr,
            repr($sample_ident:ident) = $into_repr:expr,
        }
    }) => {
        impl $(<$typaram $(: $($tbound)::+)?>)? crate::Type for $($ty)::+ $(<$typaram>)? {
            const SIGNATURE: &'static Signature = <$repr>::SIGNATURE;
        }

        #[cfg(test)]
        #[allow(unused_imports)]
        mod $test_mod {
            use super::*;
            use crate::{serialized::Context, to_bytes, LE};

            $($(type $typaram_sample = $typaram_sample_value;)*)?
            type Ty = $($ty)::+$(<$typaram>)?;

            const _: fn() = || {
                fn assert_impl_all<'de, T: ?Sized + serde::Serialize + serde::Deserialize<'de>>() {}
                assert_impl_all::<Ty>();
            };

            #[test]
            fn type_can_be_deserialized_from_encoded_type() {
                let ctx = Context::new_dbus(LE, 0);
                let samples = $samples;
                let _: &[Ty] = &samples;

                for $sample_ident in samples {
                    let encoded = to_bytes(ctx, &$sample_ident).unwrap();
                    let (decoded, _): (Ty, _) = encoded.deserialize().unwrap();
                    assert_eq!($sample_ident, decoded);
                }
            }

            #[test]
            fn repr_can_be_deserialized_from_encoded_type() {
                let ctx = Context::new_dbus(LE, 0);
                let samples = $samples;
                let _: &[Ty] = &samples;

                for $sample_ident in samples {
                    let repr: $repr = $into_repr;
                    let encoded = to_bytes(ctx, &$sample_ident).unwrap();
                    let (decoded, _): ($repr, _) = encoded.deserialize().unwrap();
                    assert_eq!(repr, decoded);
                }
            }

            #[test]
            fn type_can_be_deserialized_from_encoded_repr() {
                let ctx = Context::new_dbus(LE, 0);
                let samples = $samples;
                let _: &[Ty] = &samples;

                for $sample_ident in samples {
                    let repr: $repr = $into_repr;
                    let encoded = to_bytes(ctx, &repr).unwrap();
                    let (decoded, _): (Ty, _) = encoded.deserialize().unwrap();
                    assert_eq!($sample_ident, decoded);
                }
            }

            #[test]
            fn encoding_of_type_and_repr_match() {
                let ctx = Context::new_dbus(LE, 0);
                let samples = $samples;
                let _: &[Ty] = &samples;

                for $sample_ident in samples {
                    let repr: $repr = $into_repr;
                    let encoded = to_bytes(ctx, &$sample_ident).unwrap();
                    let encoded_repr = to_bytes(ctx, &repr).unwrap();
                    assert_eq!(encoded.bytes(), encoded_repr.bytes());
                }
            }

            $(
                #[test]
                fn signature_equals() {
                    assert_eq!(<Ty as Type>::SIGNATURE, $signature);
                }
            )?
        }
    };
}

impl<T> DynamicType for T
where
    T: Type + ?Sized,
{
    fn dynamic_signature(&self) -> Signature {
        <T as Type>::SIGNATURE.clone()
    }
}

impl<T> Type for PhantomData<T>
where
    T: Type + ?Sized,
{
    const SIGNATURE: &'static Signature = T::SIGNATURE;
}

impl<'de, T> DynamicDeserialize<'de> for T
where
    T: Type + Deserialize<'de>,
{
    type Deserializer = PhantomData<T>;

    fn deserializer_for_signature(signature: &Signature) -> zvariant::Result<Self::Deserializer> {
        let expected = <T as Type>::SIGNATURE;

        if expected == signature {
            Ok(PhantomData)
        } else {
            let expected = <T as Type>::SIGNATURE;
            Err(zvariant::Error::SignatureMismatch(
                signature.clone(),
                format!("`{expected}`"),
            ))
        }
    }
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
    time::SystemTime,
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

impl_type_with_repr! {
    SystemTime => (u64, u32) {
        system_time {
            samples = [SystemTime::now()],
            repr(t) = {
                let since_epoch = t.duration_since(SystemTime::UNIX_EPOCH).unwrap();
                (since_epoch.as_secs(), since_epoch.subsec_nanos())
            },
        }
    }
}

impl_type_with_repr! {
    Ipv4Addr => [u8; 4] {
        ipv4_addr {
            samples = [Ipv4Addr::LOCALHOST],
            repr(addr) = addr.octets(),
        }
    }
}

impl_type_with_repr! {
    Ipv6Addr => [u8; 16] {
        ipv6_addr {
            samples = [Ipv6Addr::LOCALHOST],
            repr(addr) = addr.octets(),
        }
    }
}

impl_type_with_repr! {
    IpAddr => (u32, &[u8]) {
        ip_addr {
            samples = [IpAddr::V4(Ipv4Addr::LOCALHOST), IpAddr::V6(Ipv6Addr::LOCALHOST)],
            repr(addr) = match addr {
                IpAddr::V4(v4) => (0, &v4.octets()),
                IpAddr::V6(v6) => (1, &v6.octets()),
            },
        }
    }
}

impl_type_with_repr! {
    SocketAddrV4 => (Ipv4Addr, u16) {
        socket_addr_v4 {
            samples = [SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080)],
            repr(addr) = (*addr.ip(), addr.port()),
        }
    }
}

impl_type_with_repr! {
    SocketAddrV6 => (Ipv6Addr, u16) {
        socket_addr_v6 {
            samples = [SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8080, 0, 0)],
            // https://github.com/serde-rs/serde/blob/9b868ef831c95f50dd4bde51a7eb52e3b9ee265a/serde/src/ser/impls.rs#L966
            repr(addr) = (*addr.ip(), addr.port()),
        }
    }
}

// TODO(bash): Implement DynamicType for SocketAddr

// BitFlags
#[cfg(feature = "enumflags2")]
impl<F> Type for enumflags2::BitFlags<F>
where
    F: Type + enumflags2::BitFlag,
{
    const SIGNATURE: &'static Signature = F::SIGNATURE;
}

#[cfg(feature = "serde_bytes")]
impl Type for serde_bytes::Bytes {
    const SIGNATURE: &'static Signature = &Signature::static_array(&Signature::U8);
}

#[cfg(feature = "serde_bytes")]
impl Type for serde_bytes::ByteBuf {
    const SIGNATURE: &'static Signature = &Signature::static_array(&Signature::U8);
}

#[allow(unused)]
macro_rules! static_str_type {
    ($ty:ty) => {
        impl Type for $ty {
            const SIGNATURE: &'static Signature = &Signature::Str;
        }
    };
}

static_str_type!(Path);
static_str_type!(PathBuf);

#[cfg(feature = "uuid")]
impl_type_with_repr! {
    uuid::Uuid => &[u8] {
        uuid_ {
            signature = "ay",
            samples = [uuid::Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8").unwrap()],
            repr(u) = u.as_bytes(),
        }
    }
}

#[cfg(feature = "url")]
impl_type_with_repr! {
    url::Url => &str {
        url_ {
            samples = [url::Url::parse("https://example.com").unwrap()],
            repr(url) = &url.to_string(),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Date => (i32, u16) {
        time_date {
            samples = [time::Date::MIN, time::Date::MAX, time::Date::from_calendar_date(2011, time::Month::June, 21).unwrap()],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L92
            repr(d) = (d.year(), d.ordinal()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Duration => (i64, i32) {
        time_duration {
            samples = [time::Duration::MIN, time::Duration::MAX, time::Duration::new(42, 123456789)],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L119
            repr(d) = (d.whole_seconds(), d.subsec_nanoseconds()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::OffsetDateTime => (i32, u16, u8, u8, u8, u32, i8, i8, i8) {
        time_offset_date_time {
            samples = [
                time::OffsetDateTime::now_utc(),
                time::OffsetDateTime::new_in_offset(
                    time::Date::from_calendar_date(2024, time::Month::May, 4).unwrap(),
                    time::Time::from_hms_nano(15, 32, 43, 2_000).unwrap(),
                    time::UtcOffset::from_hms(1, 2, 3).unwrap())
            ],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L155
            repr(d) = (
                d.year(),
                d.ordinal(),
                d.hour(),
                d.minute(),
                d.second(),
                d.nanosecond(),
                d.offset().whole_hours(),
                d.offset().minutes_past_hour(),
                d.offset().seconds_past_minute()
            ),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::PrimitiveDateTime => (i32, u16, u8, u8, u8, u32) {
        time_primitive_date_time {
            samples = [
                time::PrimitiveDateTime::MIN,
                time::PrimitiveDateTime::MAX,
                time::PrimitiveDateTime::new(
                    time::Date::from_calendar_date(2024, time::Month::May, 4).unwrap(),
                    time::Time::from_hms_nano(15, 32, 43, 2_000).unwrap())
            ],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L200
            repr(d) = (
                d.year(),
                d.ordinal(),
                d.hour(),
                d.minute(),
                d.second(),
                d.nanosecond()
            ),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Time => (u8, u8, u8, u32) {
        time_time {
            samples = [time::Time::MIDNIGHT, time::Time::from_hms(23, 42, 59).unwrap(), time::Time::from_hms_nano(15, 32, 43, 2_000).unwrap()],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L246
            repr(t) = (t.hour(), t.minute(), t.second(), t.nanosecond()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::UtcOffset => (i8, i8, i8) {
        time_utc_offset {
            samples = [time::UtcOffset::UTC, time::UtcOffset::from_hms(1, 2, 3).unwrap()],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L282
            repr(offset) = (offset.whole_hours(), offset.minutes_past_hour(), offset.seconds_past_minute()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Weekday => u8 {
        time_weekday {
            samples = [time::Weekday::Monday, time::Weekday::Wednesday, time::Weekday::Friday],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L312
            repr(weekday) = weekday.number_from_monday(),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Month => u8 {
        time_month {
            samples = [time::Month::January, time::Month::July, time::Month::December],
            // Serialized as month number:
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L337
            repr(month) = month as u8,
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::DateTime<Tz: chrono::TimeZone> => &str {
        chrono_date_time <Tz = chrono::offset::Utc> {
            samples = [chrono::DateTime::<Tz>::MIN_UTC, chrono::DateTime::<Tz>::MAX_UTC],
            repr(date) = &date.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string(),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::Month => &str {
        chrono_month {
            samples = [chrono::Month::January, chrono::Month::December],
            repr(month) = month.name(),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::NaiveDate => &str {
        chrono_naive_date {
            samples = [chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap()],
            repr(d) = &format!("{d:?}"),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::NaiveDateTime => &str {
        chrono_naive_date_time {
            samples = [chrono::NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap()],
            repr(dt) = &format!("{dt:?}"),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::NaiveTime => &str {
        chrono_naive_time {
            samples = [chrono::NaiveTime::from_hms_opt(9, 10, 11).unwrap()],
            repr(t) = &format!("{t:?}"),
        }
    }
}

#[cfg(feature = "chrono")]
impl_type_with_repr! {
    chrono::Weekday => &str {
        chrono_weekday {
            samples = [chrono::Weekday::Mon, chrono::Weekday::Fri],
            // Serialized as the weekday's name.
            repr(weekday) = &weekday.to_string(),
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
            static_assertions::assert_impl_all!($ty: From<$primitive>);

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
