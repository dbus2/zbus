//! Deprecated aliases for the [`wire`] module.
//!
//! Every item here is an alias for, a forwarding wrapper around, or a re-export of, its
//! [`crate::wire`] counterpart. The module exists so that most `zbus::zvariant::…` paths keep
//! compiling after the merge; notable exceptions — tuple-struct constructors called through an
//! alias, such as `DynamicTuple(…)`, patterns on the `Value::Value` variant (now
//! `Variant::Variant`), the removed `Error` variants and the removed GVariant APIs, such as
//! `Value::Maybe` and `Context::new_gvariant` — are covered by the "Upgrading to
//! zbus 6.0" chapter of the zbus book. Code written against the `zvariant` crate itself needs
//! more than this: it switches its dependency to `zbus` and uses the corresponding items from
//! the `zbus` crate root (or [`crate::wire`] for the low-level encoding and decoding API), as the
//! same chapter describes. This module is removed in zbus 7.0.
//!
//! Importing the module, or naming any of the aliases, produces a deprecation warning:
//!
//! ```compile_fail
//! #![deny(deprecated)]
//! use zbus::zvariant;
//!
//! let _ = zvariant::Value::from(42u8);
//! ```
//!
//! Paths *through* the module, and the traits, derives and submodules it re-exports, resolve
//! silently, because `#[deprecated]` on a `pub use` has no effect:
//!
//! ```
//! use zbus::zvariant::Type;
//!
//! #[derive(Type)]
//! struct Foo(u32);
//! ```
// The aliases reference each other freely; the deprecation is aimed at downstream crates, not at
// this module.
#![allow(deprecated)]

use crate::wire;

// Traits, and the derives whose name they share. Re-exporting `wire::Type` carries both the
// `Type` trait and the `Type` derive, since `wire` exports both under that name.
pub use crate::wire::{
    Basic, DynamicDeserialize, DynamicType, NoneValue, ReadBytes, Type, WriteBytes,
};
// The remaining derives and `signature!` come straight from the macro crate so that they land
// only in the macro namespace and leave the type namespace to the aliases below. The `Value` and
// `OwnedValue` derives are the `Variant` and `OwnedVariant` derives under their pre-6.0 names.
pub use zbus_macros::{
    DeserializeDict, OwnedVariant as OwnedValue, SerializeDict, Variant as Value, signature,
};
// Submodules, the two `unsafe` writer functions and the `Type` helper macros, all silent.
pub use crate::wire::{
    as_variant as as_value,
    as_variant::{Deserialize as DeserializeValue, Serialize as SerializeValue},
    dbus, export, impl_type_with_repr, static_str_type, to_writer, to_writer_for_signature,
};
// The derives generate code that names `Variant`, `OwnedVariant` and `as_variant` under the path
// of their `crate` attribute, so a derive pointed at this module needs them here under their 6.0
// names. Re-exporting the `wire` items carries the `Variant` and `OwnedVariant` derives along.
pub use crate::wire::{OwnedVariant, Variant, as_variant};

/// Deprecated alias of [`crate::Array`].
#[deprecated(since = "6.0.0", note = "use `zbus::Array` instead")]
pub type Array<'a> = wire::Array<'a>;

/// Deprecated alias of [`crate::wire::ArraySeed`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::ArraySeed` instead")]
pub type ArraySeed = wire::ArraySeed;

/// Deprecated alias of [`crate::Dict`].
#[deprecated(since = "6.0.0", note = "use `zbus::Dict` instead")]
pub type Dict<'k, 'v> = wire::Dict<'k, 'v>;

/// Deprecated alias of [`crate::DynamicTuple`].
#[deprecated(since = "6.0.0", note = "use `zbus::DynamicTuple` instead")]
pub type DynamicTuple<T> = wire::DynamicTuple<T>;

/// Deprecated alias of [`crate::wire::TupleSeed`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::TupleSeed` instead")]
pub type TupleSeed<'a, T, S> = wire::TupleSeed<'a, T, S>;

/// Deprecated alias of [`crate::wire::Endian`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::Endian` instead")]
pub type Endian = wire::Endian;

/// Deprecated alias of [`crate::Fd`].
#[cfg(unix)]
#[deprecated(since = "6.0.0", note = "use `zbus::Fd` instead")]
pub type Fd<'f> = wire::Fd<'f>;

/// Deprecated alias of [`crate::OwnedFd`].
#[cfg(unix)]
#[deprecated(since = "6.0.0", note = "use `zbus::OwnedFd` instead")]
pub type OwnedFd = wire::OwnedFd;

/// Deprecated alias of [`crate::FilePath`].
#[deprecated(since = "6.0.0", note = "use `zbus::FilePath` instead")]
pub type FilePath<'f> = wire::FilePath<'f>;

/// Deprecated alias of [`crate::ObjectPath`].
#[deprecated(since = "6.0.0", note = "use `zbus::ObjectPath` instead")]
pub type ObjectPath<'a> = wire::ObjectPath<'a>;

/// Deprecated alias of [`crate::OwnedObjectPath`].
#[deprecated(since = "6.0.0", note = "use `zbus::OwnedObjectPath` instead")]
pub type OwnedObjectPath = wire::OwnedObjectPath;

/// Deprecated alias of [`crate::Optional`].
#[deprecated(since = "6.0.0", note = "use `zbus::Optional` instead")]
pub type Optional<T> = wire::Optional<T>;

/// Deprecated alias of [`crate::Signature`].
#[deprecated(since = "6.0.0", note = "use `zbus::Signature` instead")]
pub type Signature = wire::Signature;

/// Deprecated alias of [`crate::Str`].
#[deprecated(since = "6.0.0", note = "use `zbus::Str` instead")]
pub type Str<'a> = wire::Str<'a>;

/// Deprecated alias of [`crate::Structure`].
#[deprecated(since = "6.0.0", note = "use `zbus::Structure` instead")]
pub type Structure<'a> = wire::Structure<'a>;

/// Deprecated alias of [`crate::wire::StructureBuilder`].
#[deprecated(since = "6.0.0", note = "use `zbus::Structure::builder()` instead")]
pub type StructureBuilder<'a> = wire::StructureBuilder<'a>;

/// Deprecated alias of [`crate::wire::StructureSeed`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::StructureSeed` instead")]
pub type StructureSeed<'a> = wire::StructureSeed;

/// Deprecated alias of [`crate::OwnedStructure`].
#[deprecated(since = "6.0.0", note = "use `zbus::OwnedStructure` instead")]
pub type OwnedStructure = wire::OwnedStructure;

/// Deprecated alias of [`crate::wire::OwnedStructureSeed`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::OwnedStructureSeed` instead")]
pub type OwnedStructureSeed = wire::OwnedStructureSeed;

/// Deprecated alias of [`enum@crate::Variant`].
#[deprecated(since = "6.0.0", note = "use `zbus::Variant` instead")]
pub type Value<'a> = wire::Variant<'a>;

/// Deprecated alias of [`struct@crate::OwnedVariant`].
#[deprecated(since = "6.0.0", note = "use `zbus::OwnedVariant` instead")]
pub type OwnedValue = wire::OwnedVariant;

/// Deprecated alias of [`crate::Error`].
#[deprecated(since = "6.0.0", note = "use `zbus::Error` instead")]
pub type Error = crate::Error;

/// Deprecated alias of [`crate::Result`].
#[deprecated(since = "6.0.0", note = "use `zbus::Result` instead")]
pub type Result<T> = crate::Result<T>;

/// Deprecated alias of [`crate::MaxDepthExceeded`].
#[deprecated(since = "6.0.0", note = "use `zbus::MaxDepthExceeded` instead")]
pub type MaxDepthExceeded = crate::MaxDepthExceeded;

/// Deprecated wrapper for [`crate::wire::serialized_size`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::serialized_size` instead")]
pub fn serialized_size<T>(
    ctxt: wire::serialized::Context,
    value: &T,
) -> crate::Result<wire::serialized::Size>
where
    T: ?Sized + serde::Serialize + wire::DynamicType,
{
    wire::serialized_size(ctxt, value)
}

/// Deprecated wrapper for [`crate::wire::to_bytes`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::to_bytes` instead")]
pub fn to_bytes<T>(
    ctxt: wire::serialized::Context,
    value: &T,
) -> crate::Result<wire::serialized::Data<'static, 'static>>
where
    T: ?Sized + serde::Serialize + wire::DynamicType,
{
    wire::to_bytes(ctxt, value)
}

/// Deprecated wrapper for [`crate::wire::to_bytes_for_signature`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::to_bytes_for_signature` instead"
)]
pub fn to_bytes_for_signature<S, T>(
    ctxt: wire::serialized::Context,
    signature: S,
    value: &T,
) -> crate::Result<wire::serialized::Data<'static, 'static>>
where
    S: TryInto<wire::Signature>,
    S::Error: Into<crate::Error>,
    T: ?Sized + serde::Serialize,
{
    wire::to_bytes_for_signature(ctxt, signature, value)
}

/// Deprecated wrapper for `crate::wire::padding_for_n_bytes`.
#[doc(hidden)]
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::padding_for_n_bytes` instead"
)]
pub fn padding_for_n_bytes(value: usize, align: usize) -> usize {
    wire::padding_for_n_bytes(value, align)
}

/// Deprecated alias of [`crate::wire::ARRAY_SIGNATURE_CHAR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::ARRAY_SIGNATURE_CHAR` instead"
)]
pub const ARRAY_SIGNATURE_CHAR: char = wire::ARRAY_SIGNATURE_CHAR;

/// Deprecated alias of [`crate::wire::ARRAY_SIGNATURE_STR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::ARRAY_SIGNATURE_STR` instead"
)]
pub const ARRAY_SIGNATURE_STR: &str = wire::ARRAY_SIGNATURE_STR;

/// Deprecated alias of [`crate::wire::STRUCT_SIG_START_CHAR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::STRUCT_SIG_START_CHAR` instead"
)]
pub const STRUCT_SIG_START_CHAR: char = wire::STRUCT_SIG_START_CHAR;

/// Deprecated alias of [`crate::wire::STRUCT_SIG_END_CHAR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::STRUCT_SIG_END_CHAR` instead"
)]
pub const STRUCT_SIG_END_CHAR: char = wire::STRUCT_SIG_END_CHAR;

/// Deprecated alias of [`crate::wire::STRUCT_SIG_START_STR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::STRUCT_SIG_START_STR` instead"
)]
pub const STRUCT_SIG_START_STR: &str = wire::STRUCT_SIG_START_STR;

/// Deprecated alias of [`crate::wire::STRUCT_SIG_END_STR`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::STRUCT_SIG_END_STR` instead")]
pub const STRUCT_SIG_END_STR: &str = wire::STRUCT_SIG_END_STR;

/// Deprecated alias of [`crate::wire::DICT_ENTRY_SIG_START_CHAR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::DICT_ENTRY_SIG_START_CHAR` instead"
)]
pub const DICT_ENTRY_SIG_START_CHAR: char = wire::DICT_ENTRY_SIG_START_CHAR;

/// Deprecated alias of [`crate::wire::DICT_ENTRY_SIG_END_CHAR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::DICT_ENTRY_SIG_END_CHAR` instead"
)]
pub const DICT_ENTRY_SIG_END_CHAR: char = wire::DICT_ENTRY_SIG_END_CHAR;

/// Deprecated alias of [`crate::wire::DICT_ENTRY_SIG_START_STR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::DICT_ENTRY_SIG_START_STR` instead"
)]
pub const DICT_ENTRY_SIG_START_STR: &str = wire::DICT_ENTRY_SIG_START_STR;

/// Deprecated alias of [`crate::wire::DICT_ENTRY_SIG_END_STR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::DICT_ENTRY_SIG_END_STR` instead"
)]
pub const DICT_ENTRY_SIG_END_STR: &str = wire::DICT_ENTRY_SIG_END_STR;

/// Deprecated alias of [`crate::wire::VARIANT_SIGNATURE_CHAR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::VARIANT_SIGNATURE_CHAR` instead"
)]
pub const VARIANT_SIGNATURE_CHAR: char = wire::VARIANT_SIGNATURE_CHAR;

/// Deprecated alias of [`crate::wire::VARIANT_SIGNATURE_STR`].
#[deprecated(
    since = "6.0.0",
    note = "use `zbus::wire::VARIANT_SIGNATURE_STR` instead"
)]
pub const VARIANT_SIGNATURE_STR: &str = wire::VARIANT_SIGNATURE_STR;

/// Deprecated alias of [`crate::wire::LE`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::LE` instead")]
pub const LE: wire::Endian = wire::LE;

/// Deprecated alias of [`crate::wire::BE`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::BE` instead")]
pub const BE: wire::Endian = wire::BE;

/// Deprecated alias of [`crate::wire::NATIVE_ENDIAN`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::NATIVE_ENDIAN` instead")]
pub const NATIVE_ENDIAN: wire::Endian = wire::NATIVE_ENDIAN;

/// Deprecated alias of [`crate::wire::NETWORK_ENDIAN`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::NETWORK_ENDIAN` instead")]
pub const NETWORK_ENDIAN: wire::Endian = wire::NETWORK_ENDIAN;

/// Deprecated aliases for [`crate::wire::serialized`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::serialized` instead")]
pub mod serialized {
    use crate::wire;

    /// Deprecated alias of [`crate::wire::serialized::Context`].
    #[deprecated(
        since = "6.0.0",
        note = "use `zbus::wire::serialized::Context` instead"
    )]
    pub type Context = wire::serialized::Context;

    /// Deprecated alias of [`crate::wire::serialized::Data`].
    #[deprecated(since = "6.0.0", note = "use `zbus::wire::serialized::Data` instead")]
    pub type Data<'bytes, 'fds> = wire::serialized::Data<'bytes, 'fds>;

    /// Deprecated alias of [`crate::wire::serialized::Size`].
    #[deprecated(since = "6.0.0", note = "use `zbus::wire::serialized::Size` instead")]
    pub type Size = wire::serialized::Size;

    /// Deprecated alias of [`crate::wire::serialized::Written`].
    #[deprecated(
        since = "6.0.0",
        note = "use `zbus::wire::serialized::Written` instead"
    )]
    pub type Written = wire::serialized::Written;
}

/// Deprecated aliases for [`mod@crate::wire::signature`].
#[deprecated(since = "6.0.0", note = "use `zbus::wire::signature` instead")]
pub mod signature {
    use crate::wire;

    /// Deprecated alias of [`crate::wire::signature::Signature`].
    #[deprecated(
        since = "6.0.0",
        note = "use `zbus::wire::signature::Signature` instead"
    )]
    pub type Signature = wire::signature::Signature;

    /// Deprecated alias of [`crate::wire::signature::Child`].
    #[deprecated(since = "6.0.0", note = "use `zbus::wire::signature::Child` instead")]
    pub type Child = wire::signature::Child;

    /// Deprecated alias of [`crate::wire::signature::Fields`].
    #[deprecated(since = "6.0.0", note = "use `zbus::wire::signature::Fields` instead")]
    pub type Fields = wire::signature::Fields;

    /// Deprecated alias of [`crate::wire::signature::Error`].
    #[deprecated(since = "6.0.0", note = "use `zbus::wire::signature::Error` instead")]
    pub type Error = wire::signature::Error;

    /// Deprecated wrapper for [`crate::wire::signature::validate`].
    #[deprecated(
        since = "6.0.0",
        note = "use `zbus::wire::signature::validate` instead"
    )]
    pub fn validate(bytes: &[u8]) -> Result<(), Error> {
        wire::signature::validate(bytes)
    }
}
