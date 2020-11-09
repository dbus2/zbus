use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{self, AttributeArgs, Ident, ItemTrait, NestedMeta, TraitItemMethod};

use crate::utils::*;

pub fn expand(args: AttributeArgs, input: ItemTrait) -> TokenStream {
    let mut iface_name = None;
    let mut default_path = None;
    let mut default_service = None;
    let mut has_introspect_method = false;

    let zbus = get_zbus_crate_ident();

    for arg in args {
        match arg {
            NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                if nv.path.is_ident("interface") || nv.path.is_ident("name") {
                    if let syn::Lit::Str(lit) = nv.lit {
                        iface_name = Some(lit.value());
                    } else {
                        panic!("Invalid interface argument")
                    }
                } else if nv.path.is_ident("default_path") {
                    if let syn::Lit::Str(lit) = nv.lit {
                        default_path = Some(lit.value());
                    } else {
                        panic!("Invalid path argument")
                    }
                } else if nv.path.is_ident("default_service") {
                    if let syn::Lit::Str(lit) = nv.lit {
                        default_service = Some(lit.value());
                    } else {
                        panic!("Invalid service argument")
                    }
                } else {
                    panic!("Unsupported argument");
                }
            }
            _ => panic!("Unknown attribute"),
        }
    }

    let doc = get_doc_attrs(&input.attrs);
    let proxy_name = Ident::new(&format!("{}Proxy", input.ident), Span::call_site());
    let ident = input.ident.to_string();
    let name = iface_name.unwrap_or(format!("org.freedesktop.{}", ident));
    let default_path = default_path.unwrap_or(format!("/org/freedesktop/{}", ident));
    let default_service = default_service.unwrap_or_else(|| name.clone());
    let mut methods = TokenStream::new();

    for i in input.items.iter() {
        if let syn::TraitItem::Method(m) = i {
            let method_name = m.sig.ident.to_string();
            if method_name == "introspect" {
                has_introspect_method = true;
            }

            let attrs = parse_item_attributes(&m.attrs, "dbus_proxy").unwrap();
            let is_property = attrs.iter().any(|x| x.is_property());
            let has_inputs = m.sig.inputs.len() > 1;
            let name = attrs
                .iter()
                .find_map(|x| match x {
                    ItemAttribute::Name(n) => Some(n.to_string()),
                    _ => None,
                })
                .unwrap_or_else(|| {
                    pascal_case(if is_property && has_inputs {
                        assert!(method_name.starts_with("set_"));
                        &method_name[4..]
                    } else {
                        &method_name
                    })
                });
            let m = if is_property {
                gen_proxy_property(&name, &m)
            } else {
                gen_proxy_method_call(&name, &m)
            };
            methods.extend(m);
        }
    }

    if !has_introspect_method {
        methods.extend(quote! {
            pub fn introspect(&self) -> ::#zbus::fdo::Result<String> {
                self.0.introspect()
            }
        });
    };

    quote! {
        #(#doc)*
        pub struct #proxy_name<'c>(::#zbus::Proxy<'c>);

        impl<'c> #proxy_name<'c> {
            /// Creates a new proxy with the default service & path.
            pub fn new(conn: &::#zbus::Connection) -> ::#zbus::Result<Self> {
                Ok(Self(::#zbus::Proxy::new(
                    conn,
                    #default_service,
                    #default_path,
                    #name,
                )?))
            }

            /// Creates a new proxy for the given `destination` and `path`.
            pub fn new_for(
                conn: &::#zbus::Connection,
                destination: &'c str,
                path: &'c str,
            ) -> ::#zbus::Result<Self> {
                Ok(Self(::#zbus::Proxy::new(
                    conn,
                    destination,
                    path,
                    #name,
                )?))
            }

            /// Same as `new_for` but takes ownership of the passed arguments.
            pub fn new_for_owned(
                conn: ::#zbus::Connection,
                destination: String,
                path: String,
            ) -> ::#zbus::Result<Self> {
                Ok(Self(::#zbus::Proxy::new_owned(
                    conn,
                    destination,
                    path,
                    #name.to_owned(),
                )?))
            }

            /// Consumes `self`, returning the underlying `zbus::Proxy`.
            pub fn into_inner(self) -> ::#zbus::Proxy<'c> {
                self.0
            }

            /// The reference to the underlying `zbus::Proxy`.
            pub fn inner(&self) -> &::#zbus::Proxy {
                &self.0
            }

            #methods
        }

        impl<'c> std::ops::Deref for #proxy_name<'c> {
            type Target = ::#zbus::Proxy<'c>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    }
}

fn gen_proxy_method_call(method_name: &str, m: &TraitItemMethod) -> TokenStream {
    let doc = get_doc_attrs(&m.attrs);
    let args = m.sig.inputs.iter().filter_map(|arg| arg_ident(arg));
    let sig = &m.sig;
    quote! {
        #(#doc)*
        pub #sig {
            let reply = self.0.call(#method_name, &(#(#args),*))?;
            Ok(reply)
        }
    }
}

fn gen_proxy_property(property_name: &str, m: &TraitItemMethod) -> TokenStream {
    let doc = get_doc_attrs(&m.attrs);
    let sig = &m.sig;
    if sig.inputs.len() > 1 {
        let value = arg_ident(sig.inputs.last().unwrap()).unwrap();
        quote! {
            #(#doc)*
            pub #sig {
                self.0.set_property(#property_name, #value)
            }
        }
    } else {
        quote! {
            #(#doc)*
            pub #sig {
                self.0.get_property(#property_name)
            }
        }
    }
}
