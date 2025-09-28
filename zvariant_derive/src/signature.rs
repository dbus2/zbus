use std::str::FromStr;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{parse::Parse, Error};
use zvariant_utils::signature::Signature;

/// Input type for the signature macro.
struct SignatureInput {
    literal: Literal,
}

impl Parse for SignatureInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(SignatureInput {
            literal: input.parse()?,
        })
    }
}

/// Expand the `signature!` macro implementation.
///
/// Takes a string literal signature and converts it to compile-time tokens
/// representing a const `Signature`.
pub fn expand_signature_macro(input: TokenStream) -> Result<TokenStream, Error> {
    let SignatureInput {
        literal: signature_str,
    } = syn::parse2(input)?;

    let signature_string = signature_str.to_string();
    let signature_string = signature_string.trim_matches('"');

    let signature = match signature_string {
        "dict" => Signature::dict(Signature::Str, Signature::Variant),
        s => Signature::from_str(s).map_err(|e| Error::new(signature_str.span(), e))?,
    };

    let signature_tokens = signature_to_tokens(&signature);

    Ok(signature_tokens)
}

/// Converts a parsed `Signature` to compile-time token representation.
///
/// This function generates the Rust tokens that will construct the signature
/// at compile time. Used by both the signature! macro and the Type derive macro.
pub fn signature_to_tokens(signature: &Signature) -> TokenStream {
    let zv = quote! { ::zvariant };

    match signature {
        Signature::Unit => quote! { #zv::Signature::Unit },
        Signature::Bool => quote! { #zv::Signature::Bool },
        Signature::U8 => quote! { #zv::Signature::U8 },
        Signature::I16 => quote! { #zv::Signature::I16 },
        Signature::U16 => quote! { #zv::Signature::U16 },
        Signature::I32 => quote! { #zv::Signature::I32 },
        Signature::U32 => quote! { #zv::Signature::U32 },
        Signature::I64 => quote! { #zv::Signature::I64 },
        Signature::U64 => quote! { #zv::Signature::U64 },
        Signature::F64 => quote! { #zv::Signature::F64 },
        Signature::Str => quote! { #zv::Signature::Str },
        Signature::Signature => quote! { #zv::Signature::Signature },
        Signature::ObjectPath => quote! { #zv::Signature::ObjectPath },
        Signature::Variant => quote! { #zv::Signature::Variant },
        #[cfg(unix)]
        Signature::Fd => quote! { #zv::Signature::Fd },
        Signature::Array(child) => {
            let signature = signature_to_tokens(child.signature());
            quote! {
                #zv::Signature::Array(#zv::signature::Child::Static {
                    child: &#signature,
                })
            }
        }
        Signature::Dict { key, value } => {
            let key_sig = signature_to_tokens(key.signature());
            let value_sig = signature_to_tokens(value.signature());
            quote! {
                #zv::Signature::Dict {
                    key: #zv::signature::Child::Static {
                        child: &#key_sig,
                    },
                    value: #zv::signature::Child::Static {
                        child: &#value_sig,
                    },
                }
            }
        }
        Signature::Structure(fields) => {
            let fields = fields.iter().map(signature_to_tokens);
            quote! {
                #zv::Signature::Structure(#zv::signature::Fields::Static {
                    fields: &[#(&#fields),*],
                })
            }
        }
        #[cfg(feature = "gvariant")]
        Signature::Maybe(child) => {
            let signature = signature_to_tokens(child.signature());
            quote! {
                #zv::Signature::Maybe(#zv::signature::Child::Static {
                    child: &#signature,
                })
            }
        }
    }
}
