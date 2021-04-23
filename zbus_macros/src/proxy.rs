use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    self, parse_quote, spanned::Spanned, AttributeArgs, FnArg, Ident, ItemTrait, NestedMeta,
    ReturnType, TraitItemMethod, Type,
};

use crate::utils::*;

struct AsyncOpts {
    azync: bool,
    usage: TokenStream,
    wait: TokenStream,
}

impl AsyncOpts {
    fn new(azync: bool) -> Self {
        let (usage, wait) = if azync {
            (quote! { async }, quote! { .await })
        } else {
            (quote! {}, quote! {})
        };
        Self { azync, usage, wait }
    }
}

pub fn expand(args: AttributeArgs, input: ItemTrait) -> TokenStream {
    let sync_proxy = create_proxy(&args, &input, false);
    let async_proxy = create_proxy(&args, &input, true);

    quote! {
        #sync_proxy

        #async_proxy
    }
}

pub fn create_proxy(args: &[NestedMeta], input: &ItemTrait, azync: bool) -> TokenStream {
    let mut iface_name = None;
    let mut default_path = None;
    let mut default_service = None;

    let zbus = get_zbus_crate_ident();

    for arg in args {
        match arg {
            NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                if nv.path.is_ident("interface") || nv.path.is_ident("name") {
                    if let syn::Lit::Str(lit) = &nv.lit {
                        iface_name = Some(lit.value());
                    } else {
                        panic!("Invalid interface argument")
                    }
                } else if nv.path.is_ident("default_path") {
                    if let syn::Lit::Str(lit) = &nv.lit {
                        default_path = Some(lit.value());
                    } else {
                        panic!("Invalid path argument")
                    }
                } else if nv.path.is_ident("default_service") {
                    if let syn::Lit::Str(lit) = &nv.lit {
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
    let proxy_name = if azync {
        format!("Async{}Proxy", input.ident)
    } else {
        format!("{}Proxy", input.ident)
    };
    let builder_name = Ident::new(&format!("{}Builder", proxy_name), Span::call_site());
    let proxy_name = Ident::new(&proxy_name, Span::call_site());
    let ident = input.ident.to_string();
    let name = iface_name.unwrap_or(format!("org.freedesktop.{}", ident));
    let default_path = default_path.unwrap_or(format!("/org/freedesktop/{}", ident));
    let default_service = default_service.unwrap_or_else(|| name.clone());
    let mut methods = TokenStream::new();
    let async_opts = AsyncOpts::new(azync);

    for i in input.items.iter() {
        if let syn::TraitItem::Method(m) = i {
            let method_name = m.sig.ident.to_string();
            let attrs = parse_item_attributes(&m.attrs, "dbus_proxy").unwrap();
            let is_property = attrs.iter().any(|x| x.is_property());
            let is_signal = attrs.iter().any(|x| x.is_signal());
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
                gen_proxy_property(&name, &m, &async_opts)
            } else if is_signal {
                gen_proxy_signal(&name, &method_name, &m, &async_opts)
            } else {
                gen_proxy_method_call(&name, &method_name, &m, &async_opts)
            };
            methods.extend(m);
        }
    }

    let (proxy_doc, proxy_struct, connection) = if azync {
        let sync_proxy = Ident::new(&format!("{}Proxy", input.ident), Span::call_site());
        let doc = format!("Asynchronous sibling of [`{}`].", sync_proxy);
        let connection = quote! { ::#zbus::azync::Connection };
        let proxy = quote! { ::#zbus::azync::Proxy };

        (doc, proxy, connection)
    } else {
        let doc = String::from("");
        let connection = quote! { ::#zbus::Connection };
        let proxy = quote! { ::#zbus::Proxy };

        (doc, proxy, connection)
    };

    let builder_doc = format!("A builder for [`{}`].", proxy_name);
    let builder_new_doc = format!(
        "Create a new [`{}`] for the given connection.",
        builder_name
    );
    let builder_build_doc = format!("Build a [`{}`] from the builder.", proxy_name);

    quote! {
        #[doc = #builder_doc]
        #[derive(Debug)]
        pub struct #builder_name<'a>(::#zbus::azync::ProxyBuilder<'a>);

        impl<'a> #builder_name<'a> {
            #[doc = #builder_new_doc]
            pub fn new(conn: &#connection) -> ::#zbus::Result<Self> {
                let conn = conn.clone().into();

                Ok(Self(
                    ::#zbus::azync::ProxyBuilder::new(&conn)
                        .destination(#default_service)
                        .path(#default_path)?
                    .interface(#name)
                ))
            }

            /// Set the proxy destination address.
            pub fn destination<D>(mut self, destination: D) -> Self
            where
                D: std::convert::Into<std::borrow::Cow<'a, str>>
            {
                Self(self.0.destination(destination))
            }

            /// Set the proxy path.
            pub fn path<E, P>(mut self, path: P) -> ::#zbus::Result<Self>
            where
                P: std::convert::TryInto<::#zbus::export::zvariant::ObjectPath<'a>, Error = E>,
                ::#zbus::Error: From<E>,
            {
                Ok(Self(self.0.path(path)?))
            }

            /// Set the proxy interface.
            pub fn interface<I>(mut self, interface: I) -> Self
            where
                I: std::convert::Into<std::borrow::Cow<'a, str>>
            {
                Self(self.0.interface(interface))
            }

            #[doc = #builder_build_doc]
            ///
            /// An error is returned when the builder is lacking the necessary details.
            pub fn build(self) -> ::#zbus::Result<#proxy_name<'a>> {
                let inner = self.0.build()?;
                Ok(#proxy_name(inner.into()))
            }
        }

        #[doc = #proxy_doc]
        #(#doc)*
        #[derive(Debug)]
        pub struct #proxy_name<'c>(#proxy_struct<'c>);

        impl<'c> #proxy_name<'c> {
            /// Creates a new proxy with the default service & path.
            pub fn new(conn: &#connection) -> ::#zbus::Result<Self> {
                #builder_name::new(conn)?.build()
            }

            /// Consumes `self`, returning the underlying `zbus::Proxy`.
            pub fn into_inner(self) -> #proxy_struct<'c> {
                self.0
            }

            /// The reference to the underlying `zbus::Proxy`.
            pub fn inner(&self) -> &#proxy_struct {
                &self.0
            }

            #methods
        }

        impl<'c> std::ops::Deref for #proxy_name<'c> {
            type Target = #proxy_struct<'c>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'c> std::ops::DerefMut for #proxy_name<'c> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<'c> std::convert::AsRef<#proxy_struct<'c>> for #proxy_name<'c> {
            fn as_ref(&self) -> &#proxy_struct<'c> {
                &*self
            }
        }

        impl<'c> std::convert::AsMut<#proxy_struct<'c>> for #proxy_name<'c> {
            fn as_mut(&mut self) -> &mut #proxy_struct<'c> {
                &mut *self
            }
        }

        impl<'c> #zbus::export::zvariant::Type for #proxy_name<'c> {
            fn signature() -> #zbus::export::zvariant::Signature<'static> {
                #zbus::export::zvariant::OwnedObjectPath::signature()
            }
        }

        impl<'c> #zbus::export::serde::ser::Serialize for #proxy_name<'c> {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: #zbus::export::serde::ser::Serializer,
            {
                String::serialize(&self.inner().path().to_string(), serializer)
            }
        }
    }
}

fn gen_proxy_method_call(
    method_name: &str,
    snake_case_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts { usage, wait, azync } = async_opts;
    let zbus = get_zbus_crate_ident();
    let doc = get_doc_attrs(&m.attrs);
    let args: Vec<_> = m
        .sig
        .inputs
        .iter()
        .filter_map(|arg| arg_ident(arg))
        .collect();
    let attrs = parse_item_attributes(&m.attrs, "dbus_proxy").unwrap();
    let proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::Object(o) => {
            if *azync {
                Some(format!("Async{}Proxy", o))
            } else {
                Some(format!("{}Proxy", o))
            }
        }
        _ => None,
    });
    if let Some(proxy_name) = proxy_object {
        let method = Ident::new(snake_case_name, Span::call_site());
        let proxy = Ident::new(&proxy_name, Span::call_site());
        let proxy_builder = Ident::new(&format!("{}Builder", proxy_name), Span::call_site());
        let inputs = &m.sig.inputs;
        let (_, ty_generics, where_clause) = m.sig.generics.split_for_impl();
        let signature = if where_clause.is_some() {
            quote! {
                fn #method#ty_generics(#inputs) -> ::#zbus::Result<#proxy<'_>>
                #where_clause,
            }
        } else {
            quote! {
                fn #method(#inputs) -> ::#zbus::Result<#proxy<'_>>
            }
        };

        quote! {
            #(#doc)*
            pub #usage #signature {
                let object_path: ::#zbus::export::zvariant::OwnedObjectPath =
                    self.0.call(
                        #method_name,
                        &(#(#args),*),
                    )
                    #wait?;
                let proxy = #proxy_builder::new(self.0.connection())?
                    .path(object_path)?
                    .build()?;
                Ok(proxy)
            }
        }
    } else {
        let signature = &m.sig;
        let body = if args.len() == 1 {
            // Wrap single arg in a tuple so if it's a struct/tuple itself, zbus will only remove
            // the '()' from the signature that we add and not the actual intended ones.
            let arg = &args[0];
            quote! {
                &(#arg,)
            }
        } else {
            quote! {
                &(#(#args),*)
            }
        };

        quote! {
            #(#doc)*
            pub #usage #signature {
                let reply = self.0.call(#method_name, #body)#wait?;
                Ok(reply)
            }
        }
    }
}

fn gen_proxy_property(
    property_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts { usage, wait, .. } = async_opts;
    let doc = get_doc_attrs(&m.attrs);
    let signature = &m.sig;
    if signature.inputs.len() > 1 {
        let value = arg_ident(signature.inputs.last().unwrap()).unwrap();
        quote! {
            #(#doc)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                Ok(self.0.set_property(#property_name, #value)#wait?)
            }
        }
    } else {
        // This should fail to compile only if the return type is wrong,
        // so use that as the span.
        let body_span = if let ReturnType::Type(_, ty) = &signature.output {
            ty.span()
        } else {
            signature.span()
        };
        let body = quote_spanned! {body_span =>
            Ok(self.0.get_property(#property_name)#wait?)
        };
        quote! {
            #(#doc)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                #body
            }
        }
    }
}

fn gen_proxy_signal(
    signal_name: &str,
    snake_case_name: &str,
    m: &TraitItemMethod,
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts { usage, wait, azync } = async_opts;
    let zbus = get_zbus_crate_ident();
    let doc = get_doc_attrs(&m.attrs);
    let method = format_ident!("connect_{}", snake_case_name);
    let input_types: Vec<Box<Type>> = m
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(p) => Some(p.ty.clone()),
            _ => None,
        })
        .collect();
    let args: Vec<Ident> = m
        .sig
        .inputs
        .iter()
        .filter_map(|arg| arg_ident(arg).cloned())
        .collect();
    let handler = if *azync {
        quote! { FnMut(#(#input_types),*) -> ::#zbus::export::futures_core::future::BoxFuture<'static, ::#zbus::Result<()>> }
    } else {
        quote! { FnMut(#(#input_types),*) -> ::#zbus::Result<()> }
    };

    let (proxy_method, link) = if *azync {
        (
            "zbus::azync::Proxy::connect_signal",
            "https://docs.rs/zbus/latest/zbus/azync/struct.Proxy.html#method.connect_signal",
        )
    } else {
        (
            "zbus::Proxy::connect_signal",
            "https://docs.rs/zbus/latest/zbus/struct.Proxy.html#method.connect_signal",
        )
    };
    let gen_doc = format!(
        " Connect the handler for the `{}` signal. This is a convenient wrapper around [`{}`]({}).",
        signal_name, proxy_method, link,
    );

    let mut generics = m.sig.generics.clone();
    generics.params.push(parse_quote!(__H));
    {
        let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
        where_clause
            .predicates
            .push(parse_quote!(__H: #handler + Send + 'static));
    }

    let (_, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        #[doc = #gen_doc]
        #(#doc)*
        pub #usage fn #method#ty_generics(
            &self,
            mut handler: __H,
        ) -> ::#zbus::fdo::Result<::#zbus::SignalHandlerId>
        #where_clause,
        {
            self.0.connect_signal(#signal_name, move |m| {
                let (#(#args),*) = m.body().expect("Incorrect signal signature");

                handler(#(#args),*)
            })#wait
        }
    }
}
