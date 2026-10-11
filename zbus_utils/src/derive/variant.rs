use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Attribute, Data, DataEnum, DeriveInput, Error, Fields, GenericParam, Generics, Ident, Lifetime,
    LifetimeParam, Variant, spanned::Spanned,
};

use crate::macros;

use super::{Config, attrs::*};

/// Which of the `Variant`-family traits `expand_variant_derive` should implement.
pub enum VariantType {
    Variant,
    OwnedVariant,
}

/// Implements the `Variant` or `OwnedVariant` conversions, per `variant_type`, for structs and
/// enums.
pub fn expand_variant_derive(
    ast: DeriveInput,
    variant_type: VariantType,
    config: &Config,
) -> Result<TokenStream, Error> {
    let StructAttributes {
        signature,
        rename_all,
        crate_path: crate_attr,
        ..
    } = StructAttributes::parse_with_lists(&ast.attrs, config.attr_lists)?;
    let zv = config.resolve_path(crate_attr.as_deref())?;

    let signature = signature.map(|signature| match signature.as_str() {
        "dict" => "a{sv}".to_string(),
        _ => signature,
    });

    let ctx = Ctx {
        zv: &zv,
        attr_lists: config.attr_lists,
    };
    match &ast.data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(_) | Fields::Unnamed(_) => impl_struct(
                variant_type,
                ast.ident,
                ast.generics,
                &ds.fields,
                signature,
                rename_all,
                &ctx,
            ),
            Fields::Unit => Err(Error::new(ast.span(), "Unit structures not supported")),
        },
        Data::Enum(data) => impl_enum(variant_type, ast.ident, ast.generics, ast.attrs, data, &ctx),
        _ => Err(Error::new(
            ast.span(),
            "only structs and enums are supported",
        )),
    }
}

/// Codegen context threaded through the per-field/per-variant attribute parsing helpers.
#[derive(Clone, Copy)]
struct Ctx<'a> {
    zv: &'a TokenStream,
    attr_lists: &'static [&'static str],
}

fn impl_struct(
    variant_type: VariantType,
    name: Ident,
    generics: Generics,
    fields: &Fields,
    signature: Option<String>,
    rename_all: Option<String>,
    ctx: &Ctx<'_>,
) -> Result<TokenStream, Error> {
    let Ctx { zv, attr_lists } = *ctx;
    let statc_lifetime = LifetimeParam::new(Lifetime::new("'static", Span::call_site()));
    let newtype = matches!(fields, Fields::Unnamed(_)) && fields.len() == 1;
    // A newtype that has no lifetime of its own converts from and to a `Variant` of any lifetime,
    // as far as the type it wraps does, so that it can be taken out of a `Variant` borrowed from a
    // message, for instance. The impls get a lifetime parameter of their own for that.
    let mut value_generics = generics.clone();
    let mut fresh_lifetime = None;
    let (
        value_type,
        value_lifetime,
        into_value_trait,
        into_value_method,
        into_value_error_decl,
        into_value_ret,
        into_value_error_transform,
    ) = match variant_type {
        VariantType::Variant => {
            let mut lifetimes = generics.lifetimes();
            let value_lifetime = match lifetimes.next() {
                Some(lifetime) => lifetime.clone(),
                None if newtype => {
                    let lifetime = LifetimeParam::new(Lifetime::new("'__value", Span::call_site()));
                    value_generics
                        .params
                        .insert(0, GenericParam::Lifetime(lifetime.clone()));
                    fresh_lifetime = Some(lifetime.clone());

                    lifetime
                }
                None => statc_lifetime.clone(),
            };
            if lifetimes.next().is_some() {
                return Err(Error::new(
                    name.span(),
                    "Type with more than 1 lifetime not supported",
                ));
            }

            (
                quote! { #zv::Variant<#value_lifetime> },
                value_lifetime,
                quote! { From },
                quote! { from },
                quote! {},
                quote! { Self },
                quote! {},
            )
        }
        VariantType::OwnedVariant => (
            quote! { #zv::OwnedVariant },
            statc_lifetime,
            quote! { TryFrom },
            quote! { try_from },
            quote! { type Error = #zv::Error; },
            quote! { #zv::Result<Self> },
            quote! { .map_err(::std::convert::Into::into) },
        ),
    };

    let type_params = generics.type_params().cloned().collect::<Vec<_>>();
    let mut from_value_bounds = type_params
        .iter()
        .map(|type_param| {
            quote! {
                #type_param: ::std::convert::TryFrom<#zv::Variant<#value_lifetime>> + #zv::Type,
                <#type_param as ::std::convert::TryFrom<#zv::Variant<#value_lifetime>>>::Error:
                    ::std::convert::Into<#zv::Error>
            }
        })
        .collect::<Vec<_>>();
    let mut into_value_bounds = type_params
        .iter()
        .map(|type_param| {
            quote! {
                #type_param: ::std::convert::Into<#zv::Variant<#value_lifetime>> + #zv::Type
            }
        })
        .collect::<Vec<_>>();
    if let Some(lifetime) = &fresh_lifetime {
        // The conversions of the wrapped type have to work for any lifetime of the `Variant`, which
        // they do for the types that hold no borrowed data.
        let field_ty = &fields.iter().next().unwrap().ty;
        from_value_bounds.push(quote! {
            #field_ty: ::std::convert::TryFrom<#zv::Variant<#lifetime>, Error = #zv::Error>
        });
        into_value_bounds.push(quote! {
            #zv::Variant<#lifetime>: ::std::convert::From<#field_ty>
        });
    }
    let where_clause =
        |bounds: &[TokenStream]| (!bounds.is_empty()).then(|| quote! { where #(#bounds),* });
    let from_value_where_clause = where_clause(&from_value_bounds);
    let into_value_where_clause = where_clause(&into_value_bounds);
    let (impl_generics, _, _) = value_generics.split_for_impl();
    let (_, ty_generics, _) = generics.split_for_impl();
    match fields {
        Fields::Named(_) => {
            let field_names: Vec<_> = fields
                .iter()
                .map(|field| field.ident.to_token_stream())
                .collect();
            let (from_value_impl, into_value_impl) = match signature {
                Some(signature) if signature == "a{sv}" => {
                    // User wants the type to be encoded as a dict.
                    // FIXME: Not the most efficient implementation.
                    let (fields_init, entries_init): (TokenStream, TokenStream) = fields
                        .iter()
                        .map(|field| {
                            let FieldAttributes { rename, .. } =
                                FieldAttributes::parse_with_lists(&field.attrs, attr_lists)?;
                            let field_name = field.ident.to_token_stream();
                            let key_name = rename_identifier(
                                field.ident.as_ref().unwrap().to_string(),
                                field.span(),
                                rename,
                                rename_all.as_deref(),
                            )
                            .unwrap_or(field_name.to_string());
                            let convert = if macros::ty_is_option(&field.ty) {
                                quote! {
                                    .map(#zv::Variant::downcast)
                                    .transpose()?
                                }
                            } else {
                                quote! {
                                    .ok_or_else(|| #zv::Error::IncorrectType)?
                                    .downcast()?
                                }
                            };

                            let fields_init = quote! {
                                #field_name: fields
                                    .remove(#key_name)
                                    #convert,
                            };
                            let entries_init = if macros::ty_is_option(&field.ty) {
                                quote! {
                                    if let Some(v) = s.#field_name {
                                        fields.insert(
                                            #key_name,
                                            #zv::Variant::from(v),
                                        );
                                    }
                                }
                            } else {
                                quote! {
                                    fields.insert(
                                        #key_name,
                                        #zv::Variant::from(s.#field_name),
                                    );
                                }
                            };

                            Ok((fields_init, entries_init))
                        })
                        .collect::<Result<Vec<_>, Error>>()?
                        .into_iter()
                        .unzip();

                    (
                        quote! {
                            let mut fields = <::std::collections::HashMap::<
                                ::std::string::String,
                                #zv::Variant,
                            >>::try_from(value)?;

                            ::std::result::Result::Ok(Self { #fields_init })
                        },
                        quote! {
                            let mut fields = ::std::collections::HashMap::new();
                            #entries_init

                            <#value_type>::#into_value_method(#zv::Variant::from(fields))
                                #into_value_error_transform
                        },
                    )
                }
                Some(_) | None => (
                    quote! {
                        let mut fields = #zv::Structure::try_from(value)?.into_fields();

                        ::std::result::Result::Ok(Self {
                            #(
                                #field_names: fields.remove(0).downcast()?
                            ),*
                        })
                    },
                    quote! {
                        <#value_type>::#into_value_method(#zv::StructureBuilder::new()
                        #(
                            .add_field(s.#field_names)
                        )*
                        .build().unwrap())
                        #into_value_error_transform
                    },
                ),
            };
            Ok(quote! {
                impl #impl_generics ::std::convert::TryFrom<#value_type> for #name #ty_generics
                    #from_value_where_clause
                {
                    type Error = #zv::Error;

                    #[inline]
                    fn try_from(value: #value_type) -> #zv::Result<Self> {
                        #from_value_impl
                    }
                }

                impl #impl_generics #into_value_trait<#name #ty_generics> for #value_type
                    #into_value_where_clause
                {
                    #into_value_error_decl

                    #[inline]
                    fn #into_value_method(s: #name #ty_generics) -> #into_value_ret {
                        #into_value_impl
                    }
                }
            })
        }
        Fields::Unnamed(_) if fields.iter().next().is_some() => {
            // Newtype struct.
            Ok(quote! {
                impl #impl_generics ::std::convert::TryFrom<#value_type> for #name #ty_generics
                    #from_value_where_clause
                {
                    type Error = #zv::Error;

                    #[inline]
                    fn try_from(value: #value_type) -> #zv::Result<Self> {
                        ::std::convert::TryInto::try_into(value).map(Self)
                    }
                }

                impl #impl_generics #into_value_trait<#name #ty_generics> for #value_type
                    #into_value_where_clause
                {
                    #into_value_error_decl

                    #[inline]
                    fn #into_value_method(s: #name #ty_generics) -> #into_value_ret {
                        <#value_type>::#into_value_method(s.0) #into_value_error_transform
                    }
                }
            })
        }
        Fields::Unnamed(_) => panic!("impl_struct must not be called for tuples"),
        Fields::Unit => panic!("impl_struct must not be called for unit structures"),
    }
}

fn impl_enum(
    variant_type: VariantType,
    name: Ident,
    _generics: Generics,
    attrs: Vec<Attribute>,
    data: &DataEnum,
    ctx: &Ctx<'_>,
) -> Result<TokenStream, Error> {
    let Ctx { zv, attr_lists } = *ctx;
    let repr: TokenStream = match attrs.iter().find(|attr| attr.path().is_ident("repr")) {
        Some(repr_attr) => repr_attr.parse_args()?,
        None => quote! { u32 },
    };
    let enum_attrs = EnumAttributes::parse_with_lists(&attrs, attr_lists)?;
    let str_enum = enum_attrs
        .signature
        .map(|sig| sig == "s")
        .unwrap_or_default();

    let mut variant_names = vec![];
    let mut str_values = vec![];
    for variant in &data.variants {
        let variant_attrs = VariantAttributes::parse_with_lists(&variant.attrs, attr_lists)?;
        // Ensure all variants of the enum are unit type
        match variant.fields {
            Fields::Unit => {
                variant_names.push(&variant.ident);
                if str_enum {
                    let str_value = enum_name_for_variant(
                        variant,
                        variant_attrs.rename,
                        enum_attrs.rename_all.as_ref().map(AsRef::as_ref),
                    )?;
                    str_values.push(str_value);
                }
            }
            _ => return Err(Error::new(variant.span(), "must be a unit variant")),
        }
    }

    let into_val = if str_enum {
        quote! {
            match e {
                #(
                    #name::#variant_names => #str_values,
                )*
            }
        }
    } else {
        quote! { e as #repr }
    };

    let (value_type, into_value) = match variant_type {
        VariantType::Variant => (
            quote! { #zv::Variant<'_> },
            quote! {
                impl ::std::convert::From<#name> for #zv::Variant<'_> {
                    #[inline]
                    fn from(e: #name) -> Self {
                        <#zv::Variant as ::std::convert::From<_>>::from(#into_val)
                    }
                }
            },
        ),
        VariantType::OwnedVariant => (
            quote! { #zv::OwnedVariant },
            quote! {
                impl ::std::convert::TryFrom<#name> for #zv::OwnedVariant {
                    type Error = #zv::Error;

                    #[inline]
                    fn try_from(e: #name) -> #zv::Result<Self> {
                        <#zv::OwnedVariant as ::std::convert::TryFrom<_>>::try_from(
                            <#zv::Variant as ::std::convert::From<_>>::from(#into_val)
                        )
                    }
                }
            },
        ),
    };

    let from_val = if str_enum {
        quote! {
            let v: #zv::Str = ::std::convert::TryInto::try_into(value)?;

            ::std::result::Result::Ok(match v.as_str() {
                #(
                    #str_values => #name::#variant_names,
                )*
                _ => return ::std::result::Result::Err(#zv::Error::IncorrectType),
            })
        }
    } else {
        quote! {
            let v: #repr = ::std::convert::TryInto::try_into(value)?;

            ::std::result::Result::Ok(match v {
                #(
                    x if x == #name::#variant_names as #repr => #name::#variant_names
                 ),*,
                _ => return ::std::result::Result::Err(#zv::Error::IncorrectType),
            })
        }
    };

    Ok(quote! {
        impl ::std::convert::TryFrom<#value_type> for #name {
            type Error = #zv::Error;

            #[inline]
            fn try_from(value: #value_type) -> #zv::Result<Self> {
                #from_val
            }
        }

        #into_value
    })
}

fn enum_name_for_variant(
    v: &Variant,
    rename_attr: Option<String>,
    rename_all_attr: Option<&str>,
) -> Result<String, Error> {
    let ident = v.ident.to_string();

    rename_identifier(ident, v.span(), rename_attr, rename_all_attr)
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::{DeriveInput, parse_quote};

    use super::*;

    fn config() -> Config {
        Config {
            attr_lists: &["zbus", "zvariant"],
            default_path: quote! { ::zvariant },
        }
    }

    #[test]
    fn dict_signature_rejects_cross_namespace_duplicate_rename() {
        let ast: DeriveInput = parse_quote! {
            #[zvariant(signature = "dict")]
            struct Foo {
                #[zvariant(rename = "first")]
                #[zbus(rename = "second")]
                field: String,
            }
        };

        let err = expand_variant_derive(ast, VariantType::Variant, &config()).unwrap_err();

        assert!(
            err.to_string().contains("duplicate"),
            "unexpected error: {err}",
        );
    }

    #[test]
    fn dict_signature_expands_with_single_rename() {
        let ast: DeriveInput = parse_quote! {
            #[zvariant(signature = "dict")]
            struct Foo {
                #[zvariant(rename = "first")]
                field: String,
            }
        };

        assert!(expand_variant_derive(ast, VariantType::Variant, &config()).is_ok());
    }
}
