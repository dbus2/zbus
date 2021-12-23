use proc_macro2::TokenStream;
use quote::quote;
use std::iter;
use syn::{
    spanned::Spanned,
    Attribute, Data, DeriveInput, Error, Fields, Lit,
    Meta::{List, NameValue},
    NestedMeta,
    NestedMeta::Meta,
    Variant,
};

use crate::utils::*;

pub fn get_dbus_error_meta_items(attr: &Attribute) -> Result<Vec<NestedMeta>, Error> {
    if !attr.path.is_ident("dbus_error") {
        return Ok(Vec::new());
    }

    match attr.parse_meta()? {
        List(meta) => Ok(meta.nested.into_iter().collect()),
        _ => Err(Error::new(
            attr.path.get_ident().unwrap().span(),
            "unsupported attribute",
        )),
    }
}

pub fn expand_derive(input: DeriveInput) -> Result<TokenStream, Error> {
    let mut prefix = "org.freedesktop.DBus".to_string();
    let mut generate_display = true;
    for meta_item in input
        .attrs
        .iter()
        .flat_map(get_dbus_error_meta_items)
        .flatten()
    {
        match &meta_item {
            Meta(meta) => {
                let value = match meta {
                    NameValue(v) => v,
                    _ => {
                        return Err(Error::new(meta.span(), "unsupported attribute"));
                    }
                };
                if meta.path().is_ident("prefix") {
                    // Parse `#[dbus_error(prefix = "foo")]`
                    if let Lit::Str(s) = &value.lit {
                        prefix = s.value();
                    }
                } else if meta.path().is_ident("impl_display") {
                    // Parse `#[dbus_error(impl_display = bool)]`
                    if let Lit::Bool(b) = &value.lit {
                        generate_display = b.value;
                    } else {
                        return Err(Error::new(
                            meta.span(),
                            "`impl_display` must be `true` or `false`",
                        ));
                    }
                } else {
                    return Err(Error::new(meta.span(), "unsupported attribute"));
                }
            }
            NestedMeta::Lit(lit) => return Err(Error::new(lit.span(), "unsupported attribute")),
        }
    }
    let (_vis, name, _generics, data) = match input.data {
        Data::Enum(data) => (input.vis, input.ident, input.generics, data),
        _ => return Err(Error::new(input.span(), "only enums supported")),
    };

    let zbus = zbus_path();
    let mut replies = quote! {};
    let mut error_names = quote! {};
    let mut error_descriptions = quote! {};
    let mut error_converts = quote! {};

    let mut zbus_error_variant = None;

    for variant in data.variants {
        let attrs = error_parse_item_attributes(&variant.attrs)?;
        let ident = &variant.ident;
        let name = attrs
            .iter()
            .find_map(|x| match x {
                ItemAttribute::Name(n) => Some(n.to_string()),
                _ => None,
            })
            .unwrap_or_else(|| ident.to_string());

        let impl_from_zbus_error = attrs.iter().any(|x| x == &ItemAttribute::ZbusError);

        let fqn = if !impl_from_zbus_error {
            format!("{}.{}", prefix, name)
        } else {
            // The ZBus error variant will always be a hardcoded string.
            String::from("org.freedesktop.zbus.Error")
        };

        let e = match variant.fields {
            Fields::Unit => quote! {
                Self::#ident => #fqn,
            },
            Fields::Unnamed(_) => quote! {
                Self::#ident(..) => #fqn,
            },
            Fields::Named(_) => quote! {
                Self::#ident { .. } => #fqn,
            },
        };
        error_names.extend(e);

        if impl_from_zbus_error {
            if zbus_error_variant.is_some() {
                panic!("More than 1 `zbus_error` variant found");
            }

            zbus_error_variant = Some(quote! { #ident });
        }

        // FIXME: this will error if the first field is not a string as per the dbus spec, but we
        // may support other cases?
        let e = match &variant.fields {
            Fields::Unit => quote! {
                Self::#ident => &"",
            },
            Fields::Unnamed(_) => quote! {
                Self::#ident(desc, ..) => &desc,
            },
            Fields::Named(n) => {
                let f = &n
                    .named
                    .first()
                    .ok_or_else(|| Error::new(n.span(), "expected at least one field"))?
                    .ident;
                quote! {
                    Self::#ident { #f, } => #f,
                }
            }
        };
        error_descriptions.extend(e);

        // This is handled separately
        if !impl_from_zbus_error {
            // FIXME: deserialize msg to error field instead, to support variable args
            let e = match variant.fields {
                Fields::Unit => quote! {
                    #fqn => Self::#ident,
                },
                Fields::Unnamed(_) => quote! {
                    #fqn => Self::#ident(desc),
                },
                Fields::Named(_) => quote! {
                    #fqn => Self::#ident { desc },
                },
            };
            error_converts.extend(e);
        }

        let r = gen_reply_for_variant(&variant, impl_from_zbus_error);
        replies.extend(r);
    }

    let from_zbus_error_impl = zbus_error_variant
        .map(|ident| {
            quote! {
                impl ::std::convert::From<#zbus::Error> for #name {
                    fn from(value: #zbus::Error) -> #name {
                        if let #zbus::Error::MethodError(name, desc, _) = &value {
                            let desc = ::std::clone::Clone::clone(desc)
                                .unwrap_or_else(::std::string::String::new);
                            match name.as_str() {
                                #error_converts
                                _ => Self::#ident(desc, value),
                            }
                        } else {
                            let desc = ::std::string::ToString::to_string(&value);
                            Self::#ident(desc, value)
                        }
                    }
                }
            }
        })
        .unwrap_or_default();

    let display_impl = if generate_display {
        quote! {
            impl ::std::fmt::Display for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let name = #zbus::DBusError::name(self);
                    let description = #zbus::DBusError::description(self);
                    ::std::write!(f, "{}: {}", name, description)
                }
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        impl #zbus::DBusError for #name {
            fn name(&self) -> &str {
                match self {
                    #error_names
                }
            }

            fn description(&self) -> &str {
                match self {
                    #error_descriptions
                }
            }

            fn create_reply(&self, call: &#zbus::MessageHeader) -> #zbus::Result<#zbus::Message> {
                let name = self.name();
                match self {
                    #replies
                }
            }
        }

        #display_impl

        impl ::std::error::Error for #name {}

        #from_zbus_error_impl
    })
}

fn gen_reply_for_variant(variant: &Variant, zbus_error_variant: bool) -> TokenStream {
    let zbus = zbus_path();
    let ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => {
            quote! {
                Self::#ident => #zbus::MessageBuilder::error(call, name)?.build(&()),
            }
        }
        Fields::Unnamed(f) => {
            let (in_fields, out_fields) = if zbus_error_variant {
                // For zbus_error_variants, the last item is the contained error, and should
                // not be forwarded to the reply.
                let fields = (0..(f.unnamed.len() - 1))
                    .map(|n| format!("f{}", n))
                    .chain(iter::once(String::from("_zbus_error")))
                    .map(|v| syn::Ident::new(&v, ident.span()))
                    .collect::<Vec<_>>();
                let out_fields = Vec::from(&fields[..(fields.len() - 1)]);
                (fields, out_fields)
            } else {
                // For every other variant, just number them off with an 'f' in front.
                let fields = (0..f.unnamed.len())
                    .map(|n| format!("f{}", n))
                    .map(|v| syn::Ident::new(&v, ident.span()))
                    .collect::<Vec<_>>();
                (fields.clone(), fields)
            };

            quote! {
                Self::#ident(#(#in_fields),*) => #zbus::MessageBuilder::error(call, name)?.build(&(#(#out_fields),*)),
            }
        }
        Fields::Named(f) => {
            let fields = f.named.iter().map(|v| v.ident.as_ref()).collect::<Vec<_>>();
            quote! {
                Self::#ident { #(#fields),* } => #zbus::MessageBuilder::error(call, name)?.build(&(#(#fields),*)),
            }
        }
    }
}
