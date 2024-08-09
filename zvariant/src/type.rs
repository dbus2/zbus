use crate::{utils::*, Signature};
use serde::de::{Deserialize, DeserializeSeed};
use std::{
    cmp::Reverse,
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    num::{Saturating, Wrapping},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex, RwLock},
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
    /// Get the signature for the implementing type.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use zvariant::Type;
    ///
    /// assert_eq!(u32::signature(), "u");
    /// assert_eq!(String::signature(), "s");
    /// assert_eq!(<(u32, &str, u64)>::signature(), "(ust)");
    /// assert_eq!(<(u32, &str, &[u64])>::signature(), "(usat)");
    /// assert_eq!(<HashMap<u8, &str>>::signature(), "a{ys}");
    /// ```
    fn signature() -> Signature<'static>;
}

/// Types with dynamic signatures.
///
/// Prefer implementing [Type] if possible, but if the actual signature of your type cannot be
/// determined until runtime, you can implement this type to support serialization.  You should
/// also implement [DynamicDeserialize] for deserialization.
pub trait DynamicType {
    /// Get the signature for the implementing type.
    ///
    /// See [Type::signature] for details.
    fn dynamic_signature(&self) -> Signature<'_>;
}

/// Types that deserialize based on dynamic signatures.
///
/// Prefer implementing [Type] and [Deserialize] if possible, but if the actual signature of your
/// type cannot be determined until runtime, you should implement this type to support
/// deserialization given a signature.
pub trait DynamicDeserialize<'de>: DynamicType {
    /// A [DeserializeSeed] implementation for this type.
    type Deserializer: DeserializeSeed<'de, Value = Self> + DynamicType;

    /// Get a deserializer compatible with this signature.
    fn deserializer_for_signature<S>(signature: S) -> zvariant::Result<Self::Deserializer>
    where
        S: TryInto<Signature<'de>>,
        S::Error: Into<zvariant::Error>;
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
            #[inline]
            fn signature() -> crate::Signature<'static> {
                <$repr>::signature()
            }
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
                    assert_eq!(<Ty as Type>::signature(), $signature);
                }
            )?
        }
    };
}

impl<T> DynamicType for T
where
    T: Type + ?Sized,
{
    fn dynamic_signature(&self) -> Signature<'_> {
        <T as Type>::signature()
    }
}

impl<T> Type for PhantomData<T>
where
    T: Type + ?Sized,
{
    fn signature() -> Signature<'static> {
        T::signature()
    }
}

impl<'de, T> DynamicDeserialize<'de> for T
where
    T: Type + Deserialize<'de>,
{
    type Deserializer = PhantomData<T>;

    fn deserializer_for_signature<S>(signature: S) -> zvariant::Result<Self::Deserializer>
    where
        S: TryInto<Signature<'de>>,
        S::Error: Into<zvariant::Error>,
    {
        let mut expected = <T as Type>::signature();
        let original = signature.try_into().map_err(Into::into)?;

        if original == expected {
            return Ok(PhantomData);
        }

        let mut signature = original.as_ref();
        while expected.len() < signature.len()
            && signature.starts_with(STRUCT_SIG_START_CHAR)
            && signature.ends_with(STRUCT_SIG_END_CHAR)
        {
            signature = signature.slice(1..signature.len() - 1);
        }

        while signature.len() < expected.len()
            && expected.starts_with(STRUCT_SIG_START_CHAR)
            && expected.ends_with(STRUCT_SIG_END_CHAR)
        {
            expected = expected.slice(1..expected.len() - 1);
        }

        if signature == expected {
            Ok(PhantomData)
        } else {
            let expected = <T as Type>::signature();
            Err(zvariant::Error::SignatureMismatch(
                original.to_owned(),
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
            #[inline]
            fn signature() -> Signature<'static> {
                Signature::from_string_unchecked(format!("a{}", T::signature()))
            }
        }
    };
}

array_type!([T]);
array_type!(Vec<T>);

impl<T, S> Type for std::collections::HashSet<T, S>
where
    T: Type + Eq + Hash,
    S: BuildHasher,
{
    #[inline]
    fn signature() -> Signature<'static> {
        <[T]>::signature()
    }
}

#[cfg(feature = "arrayvec")]
impl<T, const CAP: usize> Type for arrayvec::ArrayVec<T, CAP>
where
    T: Type,
{
    #[inline]
    fn signature() -> Signature<'static> {
        <[T]>::signature()
    }
}

#[cfg(feature = "arrayvec")]
impl<const CAP: usize> Type for arrayvec::ArrayString<CAP> {
    #[inline]
    fn signature() -> Signature<'static> {
        <&str>::signature()
    }
}

#[cfg(feature = "heapless")]
impl<T, const CAP: usize> Type for heapless::Vec<T, CAP>
where
    T: Type,
{
    #[inline]
    fn signature() -> Signature<'static> {
        <[T]>::signature()
    }
}

#[cfg(feature = "heapless")]
impl<const CAP: usize> Type for heapless::String<CAP> {
    #[inline]
    fn signature() -> Signature<'static> {
        <&str>::signature()
    }
}

// Empty type deserves empty signature
impl Type for () {
    #[inline]
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked("")
    }
}

macro_rules! deref_impl {
    (
        $type:ty,
        <$($desc:tt)+
    ) => {
        impl <$($desc)+ {
            #[inline]
            fn signature() -> Signature<'static> {
                <$type>::signature()
            }
        }
    };
}

deref_impl!(T, <T: ?Sized + Type> Type for &T);
deref_impl!(T, <T: ?Sized + Type> Type for &mut T);
deref_impl!(T, <T: ?Sized + Type + ToOwned> Type for Cow<'_, T>);
deref_impl!(T, <T: ?Sized + Type> Type for Arc<T>);
deref_impl!(T, <T: ?Sized + Type> Type for Mutex<T>);
deref_impl!(T, <T: ?Sized + Type> Type for RwLock<T>);
deref_impl!(T, <T: ?Sized + Type> Type for Box<T>);
deref_impl!(T, <T: ?Sized + Type> Type for Rc<T>);

#[cfg(all(feature = "gvariant", not(feature = "option-as-array")))]
impl<T> Type for Option<T>
where
    T: Type,
{
    #[inline]
    fn signature() -> Signature<'static> {
        Signature::from_string_unchecked(format!("m{}", T::signature()))
    }
}

#[cfg(feature = "option-as-array")]
impl<T> Type for Option<T>
where
    T: Type,
{
    #[inline]
    fn signature() -> Signature<'static> {
        Signature::from_string_unchecked(format!("a{}", T::signature()))
    }
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> Type for ($($name,)+)
            where
                $($name: Type,)+
            {
                fn signature() -> Signature<'static> {
                    let mut sig = String::with_capacity(255);
                    sig.push(STRUCT_SIG_START_CHAR);
                    $(
                        sig.push_str($name::signature().as_str());
                    )+
                    sig.push(STRUCT_SIG_END_CHAR);

                    Signature::from_string_unchecked(sig)
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

// Arrays are serialized as tuples/structs by Serde so we treat them as such too even though
// it's very strange. Slices and arrayvec::ArrayVec can be used anyway so I guess it's no big
// deal.
impl<T, const N: usize> Type for [T; N]
where
    T: Type,
{
    #[allow(clippy::reversed_empty_ranges)]
    fn signature() -> Signature<'static> {
        let mut sig = String::with_capacity(255);
        sig.push(STRUCT_SIG_START_CHAR);
        for _ in 0..N {
            sig.push_str(T::signature().as_str());
        }
        sig.push(STRUCT_SIG_END_CHAR);

        Signature::from_string_unchecked(sig)
    }
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
            #[inline]
            fn signature() -> Signature<'static> {
                Signature::from_string_unchecked(format!("a{{{}{}}}", K::signature(), V::signature()))
            }
        }
    }
}

map_impl!(BTreeMap<K: Ord, V>);
map_impl!(HashMap<K: Eq + Hash, V, H: BuildHasher>);

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

// BitFlags
#[cfg(feature = "enumflags2")]
impl<F> Type for enumflags2::BitFlags<F>
where
    F: Type + enumflags2::BitFlag,
{
    #[inline]
    fn signature() -> Signature<'static> {
        F::signature()
    }
}

#[cfg(feature = "serde_bytes")]
impl Type for serde_bytes::Bytes {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked("ay")
    }
}

#[cfg(feature = "serde_bytes")]
impl Type for serde_bytes::ByteBuf {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked("ay")
    }
}

#[allow(unused)]
macro_rules! static_str_type {
    ($ty:ty) => {
        impl Type for $ty {
            fn signature() -> Signature<'static> {
                <&str>::signature()
            }
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

// FIXME: Ignoring the `serde-human-readable` feature of `time` crate in these impls:
// https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L110
#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Date => (i32, u16) {
        time_date {
            samples = [time::Date::MIN, time::Date::MAX],
            // https://github.com/time-rs/time/blob/f9398b9598757508ca3815694f23203843e0011b/src/serde/mod.rs#L92
            repr(d) = (d.year(), d.ordinal()),
        }
    }
}

#[cfg(feature = "time")]
impl_type_with_repr! {
    time::Duration => (i64, i32) {
        time_duration {
            samples = [time::Duration::MIN, time::Duration::MAX],
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
            samples = [time::Time::MIDNIGHT, time::Time::from_hms_nano(15, 32, 43, 2_000).unwrap()],
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
                #[inline]
                fn signature() -> Signature<'static> {
                    <$T>::signature()
                }
            }
        )+
    };
}

impl_type_for_wrapper!(Wrapping<T>, Saturating<T>, Reverse<T>);

// TODO: Blanket implementation for more types: https://github.com/serde-rs/serde/blob/master/serde/src/ser/impls.rs
