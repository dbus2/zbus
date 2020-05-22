//! This crate provides a derive macro to add [`Type`] implementation to structs and enums.
//!
//! # Examples
//!
//! For structs it works just like serde's [`Serialize`] and [`Deserialize`] macros:
//!
//! ```
//! use zvariant::{EncodingContext, from_slice, to_bytes};
//! use zvariant::Type;
//! use zvariant_derive::Type;
//! use serde::{Deserialize, Serialize};
//! use byteorder::LE;
//!
//! #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
//! struct Struct<'s> {
//!     field1: u16,
//!     field2: i64,
//!     field3: &'s str,
//! }
//!
//! assert_eq!(Struct::signature(), "(qxs)");
//! let s = Struct {
//!     field1: 42,
//!     field2: i64::max_value(),
//!     field3: "hello",
//! };
//! let ctxt = EncodingContext::<LE>::new_dbus(0);
//! let encoded = to_bytes(ctxt, &s).unwrap();
//! let decoded: Struct = from_slice(&encoded, ctxt).unwrap();
//! assert_eq!(decoded, s);
//! ```
//!
//! Same with enum, except that only enums with unit variants are supported. If you want the
//! encoding size of the enum to be dictated by `repr` attribute (like in the example below),
//! you'll also need [serde_repr] crate.
//!
//! ```
//! use zvariant::{EncodingContext, from_slice, to_bytes};
//! use zvariant::Type;
//! use zvariant_derive::Type;
//! use serde::{Deserialize, Serialize};
//! use serde_repr::{Deserialize_repr, Serialize_repr};
//! use byteorder::LE;
//!
//! #[repr(u8)]
//! #[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
//! enum Enum {
//!     Variant1,
//!     Variant2,
//! }
//! assert_eq!(Enum::signature(), u8::signature());
//! let ctxt = EncodingContext::<LE>::new_dbus(0);
//! let encoded = to_bytes(ctxt, &Enum::Variant2).unwrap();
//! let decoded: Enum = from_slice(&encoded, ctxt).unwrap();
//! assert_eq!(decoded, Enum::Variant2);
//!
//! #[repr(i64)]
//! #[derive(Deserialize_repr, Serialize_repr, Type)]
//! enum Enum2 {
//!     Variant1,
//!     Variant2,
//! }
//! assert_eq!(Enum2::signature(), i64::signature());
//!
//! // w/o repr attribute, u32 representation is chosen
//! #[derive(Deserialize, Serialize, Type)]
//! enum NoReprEnum {
//!     Variant1,
//!     Variant2,
//! }
//! assert_eq!(NoReprEnum::signature(), u32::signature());
//! ```
//!
//! [`Type`]: https://docs.rs/zvariant/2.0.0/zvariant/trait.Type.html
//! [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
//! [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
//! [serde_repr]: https://crates.io/crates/serde_repr

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, Attribute, Data, DataEnum, DeriveInput, Fields, Generics, Ident};

/// Derive macro to add [`Type`] implementation to structs and enums.
///
/// See [crate-level documentation] for more information and example code.
///
/// [`Type`]: https://docs.rs/zvariant/2.0.0/zvariant/trait.Type.html
/// [crate-level documentation]: index.html
#[proc_macro_derive(Type)]
pub fn type_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Struct(ds) => match ds.fields {
            Fields::Named(_) | Fields::Unnamed(_) => {
                impl_struct(ast.ident, ast.generics, ds.fields)
            }
            Fields::Unit => impl_unit_struct(ast.ident, ast.generics),
        },
        Data::Enum(data) => impl_enum(ast.ident, ast.generics, ast.attrs, data),
        _ => panic!("Only structures supported at the moment"),
    }
}

fn impl_struct(name: Ident, generics: Generics, fields: Fields) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let signature = signature_for_struct(fields);
    let expended = quote! {
        impl #impl_generics zvariant::Type for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                #signature
            }
        }
    };

    TokenStream::from(expended)
}

fn signature_for_struct(fields: Fields) -> proc_macro2::TokenStream {
    let field_types = fields.iter().map(|field| field.ty.to_token_stream());
    let named = match fields {
        Fields::Named(_) => true,
        Fields::Unnamed(_) => false,
        Fields::Unit => panic!("signature_for_struct must not be called for unit fields"),
    };
    if !named && field_types.len() == 1 {
        quote! {
            #(
                <#field_types as zvariant::Type>::signature()
             )*
        }
    } else {
        quote! {
                let mut s = String::from("(");
                #(
                    s.push_str(<#field_types as zvariant::Type>::signature().as_str());
                )*
                s.push_str(")");

                zvariant::Signature::from_string_unchecked(s)
        }
    }
}

fn impl_unit_struct(name: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expended = quote! {
        impl #impl_generics zvariant::Type for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                zvariant::Signature::from_str_unchecked("")
            }
        }
    };

    TokenStream::from(expended)
}

fn impl_enum(
    name: Ident,
    generics: Generics,
    attrs: Vec<Attribute>,
    data: DataEnum,
) -> TokenStream {
    let repr: proc_macro2::TokenStream = match attrs.iter().find(|attr| attr.path.is_ident("repr"))
    {
        Some(repr_attr) => repr_attr
            .parse_args()
            .expect("Failed to parse `#[repr(...)]` attribute"),
        None => quote! { u32 },
    };

    for variant in data.variants {
        // Ensure all variants of the enum are unit type
        match variant.fields {
            Fields::Unit => (),
            _ => panic!("`{}` must be a unit variant", variant.ident.to_string()),
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expended = quote! {
        impl #impl_generics zvariant::Type for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                <#repr as zvariant::Type>::signature()
            }
        }
    };

    TokenStream::from(expended)
}
