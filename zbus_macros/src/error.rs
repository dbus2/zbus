use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Fields, Lit,
    Meta::{List, NameValue},
    NestedMeta,
    NestedMeta::Meta,
    Variant,
};

use crate::utils::*;

pub fn get_dbus_error_meta_items(attr: &Attribute) -> Result<Vec<NestedMeta>, ()> {
    if !attr.path.is_ident("dbus_error") {
        return Ok(Vec::new());
    }

    match attr.parse_meta() {
        Ok(List(meta)) => Ok(meta.nested.into_iter().collect()),
        _ => panic!("unsupported attribute"),
    }
}

pub fn expand_derive(input: DeriveInput) -> TokenStream {
    let mut prefix = "org.freedesktop.DBus".to_string();
    for meta_item in input
        .attrs
        .iter()
        .flat_map(get_dbus_error_meta_items)
        .flatten()
    {
        match &meta_item {
            // Parse `#[dbus_error(prefix = "foo")]`
            Meta(NameValue(m)) if m.path.is_ident("prefix") => {
                if let Lit::Str(s) = &m.lit {
                    prefix = s.value();
                }
            }
            _ => panic!("unsupported attribute"),
        }
    }
    let (vis, name, generics, data) = match input.data {
        Data::Enum(data) => (input.vis, input.ident, input.generics, data),
        _ => panic!("Only works with DBus error enums"),
    };

    let zbus = zbus_path();
    let mut replies = quote! {};
    let mut error_names = quote! {};
    let mut error_descriptions = quote! {};
    let mut error_converts = quote! {};

    for variant in data.variants {
        let attrs = error_parse_item_attributes(&variant.attrs).unwrap();
        let ident = &variant.ident;
        let name = attrs
            .iter()
            .find_map(|x| match x {
                ItemAttribute::Name(n) => Some(n.to_string()),
                _ => None,
            })
            .unwrap_or_else(|| ident.to_string());
        if name == "ZBus" {
            continue;
        }
        let fqn = format!("{}.{}", prefix, name);

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
                let f = &n.named.first().unwrap().ident;
                quote! {
                    Self::#ident { #f, } => #f,
                }
            }
        };
        error_descriptions.extend(e);

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

        let r = gen_reply_for_variant(&variant);
        replies.extend(r);
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #vis fn name(&self) -> &str {
                match self {
                    #error_names
                    Self::ZBus(_) => "Unknown",
                }
            }

            #vis fn description(&self) -> &str {
                match self {
                    #error_descriptions
                    Self::ZBus(_) => "Unknown",
                }
            }

            #vis async fn reply(
                &self,
                c: &#zbus::Connection,
                call: &#zbus::Message,
            ) -> ::std::result::Result<u32, #zbus::Error> {
                c.send_message(#zbus::DBusError::reply_to(self, &call.header()?)?).await
            }
        }

        impl #zbus::DBusError for #name {
            fn reply_to(&self, call: &#zbus::MessageHeader) -> #zbus::Result<#zbus::Message> {
                let name = self.name();
                match self {
                    #replies
                    Self::ZBus(e) => {
                        let err = #zbus::fdo::Error::Failed(e.to_string());
                        err.reply_to(call)
                    }
                }
            }
        }

        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                ::std::write!(f, "{}: {}", self.name(), self.description())
            }
        }

        impl ::std::error::Error for #name {}

        impl ::std::convert::From<#zbus::Error> for #name {
            fn from(value: #zbus::Error) -> #name {
                if let #zbus::Error::MethodError(name, desc, _) = &value {
                    let desc = ::std::clone::Clone::clone(desc)
                        .unwrap_or_else(::std::string::String::new);
                    match name.as_str() {
                        #error_converts
                        _ => Self::ZBus(value),
                    }
                } else {
                    Self::ZBus(value)
                }
            }
        }
    }
}

fn gen_reply_for_variant(variant: &Variant) -> TokenStream {
    let zbus = zbus_path();
    let ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => {
            quote! {
                Self::#ident => #zbus::MessageBuilder::error(call, name)?.build(&()),
            }
        }
        Fields::Unnamed(f) => {
            let fields = (0..f.unnamed.len())
                .map(|n| format!("f{}", n))
                .map(|v| syn::Ident::new(&v, ident.span()))
                .collect::<Vec<_>>();
            quote! {
                Self::#ident(#(#fields),*) => #zbus::MessageBuilder::error(call, name)?.build(&(#(#fields),*)),
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
