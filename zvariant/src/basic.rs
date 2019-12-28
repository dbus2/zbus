/// Marker trait for basic types.
///
/// All basic types are also [`SimpleDecode`] implementers but not necessarily the other way around
/// (e.g [`Variant`]). Also a basic type, `f64` is excluded here even though it's a basic D-Bus type.
/// The reason is that `f64` doesn't implement [`Hash`] for [good
/// reasons](https://internals.rust-lang.org/t/f32-f64-should-implement-hash/5436/33) and given
/// that we mainly need this marker train for [`DictEntry`] (i-e Hashmaps), it's more important to
/// require `Hash` from implementers of this type than to implement this for `f64`.
///
/// [`SimpleDecode`]: trait.SimpleDecode.html
/// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
/// [`DictEntry`]: struct.DictEntry.html
/// [`Variant`]: enum.Variant.html
pub trait Basic: std::hash::Hash + std::cmp::Eq {}

impl Basic for u8 {}
impl Basic for bool {}
impl Basic for i16 {}
impl Basic for u16 {}
impl Basic for i32 {}
impl Basic for u32 {}
impl Basic for i64 {}
impl Basic for u64 {}
