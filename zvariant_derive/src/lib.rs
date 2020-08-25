#![doc(
    html_logo_url = "https://storage.googleapis.com/fdo-gitlab-uploads/project/avatar/3213/zbus-logomark.png"
)]

//! This crate provides derive macros helpers for zvariant.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{self, DeriveInput};

mod dict;
mod r#type;
mod utils;

/// Derive macro to add [`Type`] implementation to structs and enums.
///
/// # Examples
///
/// For structs it works just like serde's [`Serialize`] and [`Deserialize`] macros:
///
/// ```
/// use zvariant::{EncodingContext, from_slice, to_bytes};
/// use zvariant::Type;
/// use zvariant_derive::Type;
/// use serde::{Deserialize, Serialize};
/// use byteorder::LE;
///
/// #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
/// struct Struct<'s> {
///     field1: u16,
///     field2: i64,
///     field3: &'s str,
/// }
///
/// assert_eq!(Struct::signature(), "(qxs)");
/// let s = Struct {
///     field1: 42,
///     field2: i64::max_value(),
///     field3: "hello",
/// };
/// let ctxt = EncodingContext::<LE>::new_dbus(0);
/// let encoded = to_bytes(ctxt, &s).unwrap();
/// let decoded: Struct = from_slice(&encoded, ctxt).unwrap();
/// assert_eq!(decoded, s);
/// ```
///
/// Same with enum, except that only enums with unit variants are supported. If you want the
/// encoding size of the enum to be dictated by `repr` attribute (like in the example below),
/// you'll also need [serde_repr] crate.
///
/// ```
/// use zvariant::{EncodingContext, from_slice, to_bytes};
/// use zvariant::Type;
/// use zvariant_derive::Type;
/// use serde::{Deserialize, Serialize};
/// use serde_repr::{Deserialize_repr, Serialize_repr};
/// use byteorder::LE;
///
/// #[repr(u8)]
/// #[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
/// enum Enum {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(Enum::signature(), u8::signature());
/// let ctxt = EncodingContext::<LE>::new_dbus(0);
/// let encoded = to_bytes(ctxt, &Enum::Variant2).unwrap();
/// let decoded: Enum = from_slice(&encoded, ctxt).unwrap();
/// assert_eq!(decoded, Enum::Variant2);
///
/// #[repr(i64)]
/// #[derive(Deserialize_repr, Serialize_repr, Type)]
/// enum Enum2 {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(Enum2::signature(), i64::signature());
///
/// // w/o repr attribute, u32 representation is chosen
/// #[derive(Deserialize, Serialize, Type)]
/// enum NoReprEnum {
///     Variant1,
///     Variant2,
/// }
/// assert_eq!(NoReprEnum::signature(), u32::signature());
/// ```
///
/// [`Type`]: https://docs.rs/zvariant/2.0.0/zvariant/trait.Type.html
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [serde_repr]: https://crates.io/crates/serde_repr
#[proc_macro_derive(Type)]
pub fn type_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    r#type::expand_derive(ast)
}

/// Derive macro to add [`Type`] implementation to structs serialized as `a{sv}` type.
///
/// # Examples
///
/// ```
/// use zvariant::{Signature, Type};
/// use zvariant_derive::TypeDict;
///
/// #[derive(TypeDict)]
/// struct Struct {
///     field: u32,
/// }
///
/// assert_eq!(Struct::signature(), Signature::from_str_unchecked("a{sv}"));
/// ```
///
/// [`Type`]: ../zvariant/trait.Type.html
#[proc_macro_derive(TypeDict)]
pub fn type_dict_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    dict::expand_type_derive(ast)
}

/// Adds [`Serialize`] implementation to structs to be serialized as `a{sv}` type.
///
/// This macro serializes the deriving struct as a D-Bus dictionary type, where keys are strings and
/// values are generic values. Such dictionary types are very commonly used with
/// [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-properties)
/// and GVariant.
///
/// # Examples
///
/// For structs it works just like serde's [`Serialize`] macros:
///
/// ```
/// use zvariant::{EncodingContext, to_bytes};
/// use zvariant_derive::{SerializeDict, TypeDict};
///
/// #[derive(SerializeDict, TypeDict)]
/// struct Struct {
///     field1: u16,
///     #[zvariant(rename = "another-name")]
///     field2: i64,
///     optional_field: Option<String>,
/// }
/// ```
///
/// The serialized D-Bus version of `Struct {42, 77, None}`
/// will be `{"field1": Value::U16(42), "another-name": Value::I64(77)}`.
///
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
#[proc_macro_derive(SerializeDict, attributes(zvariant))]
pub fn serialize_dict_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    dict::expand_serialize_derive(input)
}

/// Adds [`Deserialize`] implementation to structs to be deserialized from `a{sv}` type.
///
/// This macro deserializes a D-Bus dictionary type as a struct, where keys are strings and values
/// are generic values. Such dictionary types are very commonly used with
/// [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-properties)
/// and GVariant.
///
/// # Examples
///
/// For structs it works just like serde's [`Deserialize`] macros:
///
/// ```
/// use zvariant::{EncodingContext, to_bytes};
/// use zvariant_derive::{DeserializeDict, TypeDict};
///
/// #[derive(DeserializeDict, TypeDict)]
/// struct Struct {
///     field1: u16,
///     #[zvariant(rename = "another-name")]
///     field2: i64,
///     optional_field: Option<String>,
/// }
/// ```
///
/// The deserialized D-Bus dictionary `{"field1": Value::U16(42), "another-name": Value::I64(77)}`
/// will be `Struct {42, 77, None}`.
///
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
#[proc_macro_derive(DeserializeDict, attributes(zvariant))]
pub fn deserialize_dict_macro_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    dict::expand_deserialize_derive(input)
}
