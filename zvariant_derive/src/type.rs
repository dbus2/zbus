use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    spanned::Spanned, Attribute, Data, DataEnum, DeriveInput, Error, Fields, Generics, Ident,
};
use zvariant_utils::signature::Signature;

use crate::utils::*;

pub fn expand_derive(ast: DeriveInput) -> Result<TokenStream, Error> {
    let StructAttributes { signature, .. } = StructAttributes::parse(&ast.attrs)?;

    let zv = zvariant_path();
    if let Some(signature_str) = signature {
        // Signature already provided, easy then!

        let signature = match signature_str.as_str() {
            "dict" => Signature::dict(Signature::Str, Signature::Variant),
            s => Signature::from_str(s).map_err(|e| Error::new(ast.span(), e))?,
        };
        let signature_tokens = signature_to_tokens(&signature, &zv);

        let name = ast.ident;
        let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
        return Ok(quote! {
            impl #impl_generics #zv::Type for #name #ty_generics #where_clause {
                const SIGNATURE: &'static #zv::Signature = &#signature_tokens;
            }
        });
    }

    match ast.data {
        Data::Struct(ds) => match ds.fields {
            Fields::Named(_) if ds.fields.is_empty() => {
                impl_empty_struct(ast.ident, ast.generics, &zv)
            }
            Fields::Named(_) | Fields::Unnamed(_) => {
                impl_struct(ast.ident, ast.generics, ds.fields, &zv)
            }
            Fields::Unit => impl_unit_struct(ast.ident, ast.generics, &zv),
        },
        Data::Enum(data) => impl_enum(ast.ident, ast.generics, ast.attrs, data, &zv),
        _ => Err(Error::new(
            ast.span(),
            "only structs and enums supported at the moment",
        )),
    }
    .map(|implementation| {
        quote! {
            #[allow(deprecated)]
            #implementation
        }
    })
}

fn impl_struct(
    name: Ident,
    generics: Generics,
    fields: Fields,
    zv: &TokenStream,
) -> Result<TokenStream, Error> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let signature = signature_for_struct(&fields, zv, false);

    Ok(quote! {
        impl #impl_generics #zv::Type for #name #ty_generics #where_clause {
            const SIGNATURE: &'static #zv::Signature = #signature;
        }
    })
}

fn signature_for_struct(
    fields: &Fields,
    zv: &TokenStream,
    insert_enum_variant: bool,
) -> TokenStream {
    let field_types = fields.iter().map(|field| field.ty.to_token_stream());
    let new_type = match fields {
        Fields::Named(_) => false,
        Fields::Unnamed(_) if field_types.len() == 1 => true,
        Fields::Unnamed(_) => false,
        Fields::Unit => panic!("signature_for_struct must not be called for unit fields"),
    };
    let field_types_clone = field_types.clone();
    let signature = if new_type {
        quote! {#(
            <#field_types_clone as #zv::Type>::SIGNATURE
        )*}
    } else {
        quote! {
            &#zv::Signature::Structure(#zv::signature::Fields::Static {
                fields: &[#(
                    <#field_types_clone as #zv::Type>::SIGNATURE
                ),*],
            })
        }
    };

    if insert_enum_variant {
        quote! {
            &#zv::Signature::Structure(#zv::signature::Fields::Static {
                fields: &[
                    <u32 as #zv::Type>::SIGNATURE,
                    #signature
                ],
            })
        }
    } else {
        signature
    }
}

fn impl_unit_struct(
    name: Ident,
    generics: Generics,
    zv: &TokenStream,
) -> Result<TokenStream, Error> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #zv::Type for #name #ty_generics #where_clause {
            const SIGNATURE: &'static #zv::Signature = &#zv::Signature::Unit;
        }
    })
}

fn impl_empty_struct(
    name: Ident,
    generics: Generics,
    zv: &TokenStream,
) -> Result<TokenStream, Error> {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #zv::Type for #name #ty_generics #where_clause {
            const SIGNATURE: &'static #zv::Signature = &#zv::Signature::U8;
        }
    })
}

fn impl_enum(
    name: Ident,
    generics: Generics,
    attrs: Vec<Attribute>,
    data: DataEnum,
    zv: &TokenStream,
) -> Result<TokenStream, Error> {
    let mut all_signatures: Vec<Result<TokenStream, Error>> = data
        .variants
        .iter()
        .map(|variant| signature_for_variant(variant, &attrs, zv))
        .collect();
    let signature = all_signatures.pop().unwrap()?;
    // Ensure all variants of the enum have the same number and type of fields.
    for sig in all_signatures {
        if sig?.to_string() != signature.to_string() {
            return Err(Error::new(
                name.span(),
                "all variants must have the same number and type of fields",
            ));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #zv::Type for #name #ty_generics #where_clause {
            const SIGNATURE: &'static #zv::Signature = #signature;
        }
    })
}

fn signature_for_variant(
    variant: &syn::Variant,
    attrs: &[Attribute],
    zv: &TokenStream,
) -> Result<TokenStream, Error> {
    let repr = attrs.iter().find(|attr| attr.path().is_ident("repr"));
    match &variant.fields {
        Fields::Unit => {
            let repr = match repr {
                Some(repr_attr) => repr_attr.parse_args()?,
                None => quote! { u32 },
            };

            Ok(quote! { <#repr as #zv::Type>::SIGNATURE })
        }
        Fields::Named(_) => Ok(signature_for_struct(&variant.fields, zv, true)),
        Fields::Unnamed(_) => Ok(signature_for_struct(&variant.fields, zv, true)),
    }
}

fn signature_to_tokens(signature: &Signature, zv: &TokenStream) -> TokenStream {
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
            let signature = signature_to_tokens(child.signature(), zv);
            quote! {
                #zv::Signature::Array(#zv::signature::Child::Static {
                    child: &#signature,
                })
            }
        }
        Signature::Dict { key, value } => {
            let key_sig = signature_to_tokens(key.signature(), zv);
            let value_sig = signature_to_tokens(value.signature(), zv);
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
            let fields = fields.iter().map(|f| signature_to_tokens(f, zv));
            quote! {
                #zv::Signature::Structure(#zv::signature::Fields::Static {
                    fields: &[#(&#fields),*],
                })
            }
        }
        #[cfg(feature = "gvariant")]
        Signature::Maybe(child) => {
            let signature = signature_to_tokens(child.signature(), zv);
            quote! {
                #zv::Signature::Maybe(#zv::signature::Child::Static {
                    child: &#signature,
                })
            }
        }
    }
}
