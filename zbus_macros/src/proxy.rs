use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use regex::Regex;
use syn::{
    self, fold::Fold, parse_quote, spanned::Spanned, AttributeArgs, FnArg, Ident, ItemTrait,
    NestedMeta, ReturnType, TraitItemMethod, Type,
};

use crate::utils::*;

#[derive(Clone)]
struct ProxyOpts {
    blocking: bool,
    gen_connect: bool,
    gen_dispatch: bool,
    usage: TokenStream,
    wait: TokenStream,
}

impl ProxyOpts {
    fn new() -> Self {
        Self {
            blocking: false,
            gen_connect: true,
            gen_dispatch: true,
            usage: quote! { async },
            wait: quote! { .await },
        }
    }

    fn blocking(&self) -> Self {
        let mut rv = self.clone();
        rv.blocking = true;
        rv.usage = quote! {};
        rv.wait = quote! {};
        rv
    }
}

pub fn expand(args: AttributeArgs, input: ItemTrait) -> TokenStream {
    let (mut gen_async, mut gen_blocking) = (true, true);
    let (mut async_name, mut blocking_name) = (None, None);
    let mut iface_name = None;
    let mut default_path = None;
    let mut default_service = None;
    let mut proxy_opts = ProxyOpts::new();
    for arg in &args {
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
                } else if nv.path.is_ident("async_name") {
                    if let syn::Lit::Str(lit) = &nv.lit {
                        async_name = Some(lit.value());
                    } else {
                        panic!("Invalid service argument")
                    }
                } else if nv.path.is_ident("blocking_name") {
                    if let syn::Lit::Str(lit) = &nv.lit {
                        blocking_name = Some(lit.value());
                    } else {
                        panic!("Invalid service argument")
                    }
                } else if nv.path.is_ident("gen_async") {
                    if let syn::Lit::Bool(lit) = &nv.lit {
                        gen_async = lit.value();
                    } else {
                        panic!("Invalid gen_async argument")
                    }
                } else if nv.path.is_ident("gen_blocking") {
                    if let syn::Lit::Bool(lit) = &nv.lit {
                        gen_blocking = lit.value();
                    } else {
                        panic!("Invalid gen_blocking argument")
                    }
                } else if nv.path.is_ident("gen_connect") {
                    if let syn::Lit::Bool(lit) = &nv.lit {
                        proxy_opts.gen_connect = lit.value();
                    } else {
                        panic!("Invalid gen_connect argument")
                    }
                } else if nv.path.is_ident("gen_dispatch") {
                    if let syn::Lit::Bool(lit) = &nv.lit {
                        proxy_opts.gen_dispatch = lit.value();
                    } else {
                        panic!("Invalid gen_dispatch argument")
                    }
                } else {
                    panic!("Unsupported argument");
                }
            }
            _ => panic!("Unknown attribute"),
        }
    }

    // Some sanity checks
    if !gen_blocking && !gen_async {
        panic!("Can't disable both asynchronous and blocking proxy. 😸");
    }
    if !gen_blocking && blocking_name.is_some() {
        panic!("Can't set blocking proxy's name if you disabled it. 😸");
    }
    if !gen_async && async_name.is_some() {
        panic!("Can't set asynchronous proxy's name if you disabled it. 😸");
    }

    let blocking_proxy = if gen_blocking {
        let proxy_name = blocking_name.unwrap_or_else(|| {
            if gen_async {
                format!("{}ProxyBlocking", input.ident)
            } else {
                // When only generating blocking proxy, there is no need for a suffix.
                format!("{}Proxy", input.ident)
            }
        });
        create_proxy(
            &input,
            iface_name.as_deref(),
            default_path.as_deref(),
            default_service.as_deref(),
            &proxy_name,
            proxy_opts.blocking(),
        )
    } else {
        quote! {}
    };
    let async_proxy = if gen_async {
        let proxy_name = async_name.unwrap_or_else(|| format!("{}Proxy", input.ident));
        create_proxy(
            &input,
            iface_name.as_deref(),
            default_path.as_deref(),
            default_service.as_deref(),
            &proxy_name,
            proxy_opts,
        )
    } else {
        quote! {}
    };

    quote! {
        #blocking_proxy

        #async_proxy
    }
}

fn create_proxy(
    input: &ItemTrait,
    iface_name: Option<&str>,
    default_path: Option<&str>,
    default_service: Option<&str>,
    proxy_name: &str,
    proxy_opts: ProxyOpts,
) -> TokenStream {
    let zbus = zbus_path();

    let doc = get_doc_attrs(&input.attrs);
    let proxy_name = Ident::new(proxy_name, Span::call_site());
    let ident = input.ident.to_string();
    let name = iface_name
        .map(ToString::to_string)
        .unwrap_or(format!("org.freedesktop.{}", ident));
    let default_path = default_path
        .map(ToString::to_string)
        .unwrap_or(format!("/org/freedesktop/{}", ident));
    let default_service = default_service
        .map(ToString::to_string)
        .unwrap_or_else(|| name.clone());
    let mut methods = TokenStream::new();
    let mut stream_types = TokenStream::new();
    let mut has_properties = false;
    let blocking = proxy_opts.blocking;

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
                has_properties = true;
                gen_proxy_property(&name, &method_name, m, &proxy_opts)
            } else if is_signal {
                let (method, types) =
                    gen_proxy_signal(&proxy_name, &name, &method_name, m, &proxy_opts);
                stream_types.extend(types);

                method
            } else {
                gen_proxy_method_call(&name, &method_name, m, &proxy_opts)
            };
            methods.extend(m);
        }
    }

    let ProxyOpts { usage, wait, .. } = proxy_opts;
    let (proxy_struct, connection, builder) = if blocking {
        let connection = quote! { #zbus::blocking::Connection };
        let proxy = quote! { #zbus::blocking::Proxy };
        let builder = quote! { #zbus::blocking::ProxyBuilder };

        (proxy, connection, builder)
    } else {
        let connection = quote! { #zbus::Connection };
        let proxy = quote! { #zbus::Proxy };
        let builder = quote! { #zbus::ProxyBuilder };

        (proxy, connection, builder)
    };

    quote! {
        impl<'a> #zbus::ProxyDefault for #proxy_name<'a> {
            const INTERFACE: &'static str = #name;
            const DESTINATION: &'static str = #default_service;
            const PATH: &'static str = #default_path;
        }

        #(#doc)*
        #[derive(Clone, Debug)]
        pub struct #proxy_name<'c>(#proxy_struct<'c>);

        impl<'c> #proxy_name<'c> {
            /// Creates a new proxy with the default service & path.
            pub #usage fn new(conn: &#connection) -> #zbus::Result<#proxy_name<'c>> {
                Self::builder(conn).build()#wait
            }

            /// Returns a customizable builder for this proxy.
            pub fn builder(conn: &#connection) -> #builder<'c, Self> {
                #builder::new(conn).cache_properties(#has_properties)
            }

            /// Consumes `self`, returning the underlying `zbus::Proxy`.
            pub fn into_inner(self) -> #proxy_struct<'c> {
                self.0
            }

            /// The reference to the underlying `zbus::Proxy`.
            pub fn inner(&self) -> &#proxy_struct<'c> {
                &self.0
            }

            #methods
        }

        impl<'c> ::std::convert::From<#zbus::Proxy<'c>> for #proxy_name<'c> {
            fn from(proxy: #zbus::Proxy<'c>) -> Self {
                #proxy_name(::std::convert::Into::into(proxy))
            }
        }

        impl<'c> ::std::ops::Deref for #proxy_name<'c> {
            type Target = #proxy_struct<'c>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'c> ::std::ops::DerefMut for #proxy_name<'c> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<'c> ::std::convert::AsRef<#proxy_struct<'c>> for #proxy_name<'c> {
            fn as_ref(&self) -> &#proxy_struct<'c> {
                &*self
            }
        }

        impl<'c> ::std::convert::AsMut<#proxy_struct<'c>> for #proxy_name<'c> {
            fn as_mut(&mut self) -> &mut #proxy_struct<'c> {
                &mut *self
            }
        }

        impl<'c> #zbus::zvariant::Type for #proxy_name<'c> {
            fn signature() -> #zbus::zvariant::Signature<'static> {
                #zbus::zvariant::OwnedObjectPath::signature()
            }
        }

        impl<'c> #zbus::export::serde::ser::Serialize for #proxy_name<'c> {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: #zbus::export::serde::ser::Serializer,
            {
                ::std::string::String::serialize(
                    &::std::string::ToString::to_string(self.inner().path()),
                    serializer,
                )
            }
        }

        #stream_types
    }
}

fn gen_proxy_method_call(
    method_name: &str,
    snake_case_name: &str,
    m: &TraitItemMethod,
    proxy_opts: &ProxyOpts,
) -> TokenStream {
    let ProxyOpts {
        usage,
        wait,
        blocking,
        gen_dispatch,
        ..
    } = proxy_opts;
    let zbus = zbus_path();
    let doc = get_doc_attrs(&m.attrs);
    let args: Vec<_> = m
        .sig
        .inputs
        .iter()
        .filter_map(|arg| arg_ident(arg))
        .collect();
    let attrs = parse_item_attributes(&m.attrs, "dbus_proxy").unwrap();
    let async_proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::AsyncObject(o) => Some(o.clone()),
        _ => None,
    });
    let blocking_proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::BlockingObject(o) => Some(o.clone()),
        _ => None,
    });
    let proxy_object = attrs.iter().find_map(|x| match x {
        ItemAttribute::Object(o) => {
            if *blocking {
                // FIXME: for some reason Rust doesn't let us move `blocking_proxy_object` so we've to clone.
                blocking_proxy_object
                    .as_ref()
                    .cloned()
                    .or_else(|| Some(format!("{}ProxyBlocking", o)))
            } else {
                async_proxy_object
                    .as_ref()
                    .cloned()
                    .or_else(|| Some(format!("{}Proxy", o)))
            }
        }
        _ => None,
    });
    let no_reply = attrs.iter().any(|x| matches!(x, ItemAttribute::NoReply));
    let dispatch_only = attrs.iter().any(|x| matches!(x, ItemAttribute::Dispatch));

    let method = Ident::new(snake_case_name, Span::call_site());
    let dispatch_method = if dispatch_only {
        method.clone()
    } else {
        Ident::new(&format!("dispatch_{}", snake_case_name), Span::call_site())
    };
    let inputs = &m.sig.inputs;
    let mut generics = m.sig.generics.clone();
    let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
    for param in generics
        .params
        .iter()
        .filter(|a| matches!(a, syn::GenericParam::Type(_)))
    {
        let is_input_type = inputs.iter().any(|arg| {
            // FIXME: We want to only require `Serialize` from input types and `DeserializeOwned`
            // from output types but since we don't have type introspection, we employ this
            // workaround of regex matching on string reprepresention of the the types to figure out
            // which generic types are input types.
            if let FnArg::Typed(pat) = arg {
                let pattern = format!("& *{}", param.to_token_stream());
                let regex = Regex::new(&pattern).unwrap();
                regex.is_match(&pat.ty.to_token_stream().to_string())
            } else {
                false
            }
        });
        let serde_bound: TokenStream = if is_input_type {
            parse_quote!(#zbus::export::serde::ser::Serialize)
        } else {
            parse_quote!(#zbus::export::serde::de::DeserializeOwned)
        };
        where_clause.predicates.push(parse_quote!(
            #param: #serde_bound + #zbus::zvariant::Type
        ));
    }
    let (_, ty_generics, where_clause) = generics.split_for_impl();

    if let Some(proxy_name) = proxy_object {
        let proxy = Ident::new(&proxy_name, Span::call_site());
        let signature = quote! {
            fn #method#ty_generics(#inputs) -> #zbus::Result<#proxy<'c>>
            #where_clause
        };

        quote! {
            #(#doc)*
            pub #usage #signature {
                let object_path: #zbus::zvariant::OwnedObjectPath =
                    self.0.call(
                        #method_name,
                        &(#(#args),*),
                    )
                    #wait?;
                #proxy::builder(&self.0.connection())
                    .path(object_path)?
                    .build()
                    #wait
            }
        }
    } else {
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

        let output = &m.sig.output;
        let signature = quote! {
            fn #method#ty_generics(#inputs) #output
            #where_clause
        };

        if no_reply {
            return quote! {
                #(#doc)*
                pub #usage #signature {
                    self.0.call_noreply(#method_name, #body)#wait?;
                    ::std::result::Result::Ok(())
                }
            };
        }

        let method_impl = quote! {
            #(#doc)*
            pub #usage #signature {
                let reply = self.0.call(#method_name, #body)#wait?;
                ::std::result::Result::Ok(reply)
            }
        };

        if !gen_dispatch {
            return method_impl;
        }

        generics.params.push(parse_quote!(__H));
        let cbargs = match output {
            syn::ReturnType::Default => quote!(()),
            syn::ReturnType::Type(_, t) => quote!((#t)),
        };
        let handler = if *blocking {
            parse_quote! { __H: FnOnce #cbargs + Send + 'static }
        } else {
            parse_quote! {
                __H: FnOnce #cbargs ->
                    #zbus::export::futures_core::future::BoxFuture<'static, ()> + Send + 'static
            }
        };
        {
            let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
            where_clause.predicates.push(handler);
        }
        let (_, ty_generics, where_clause) = generics.split_for_impl();
        let mut dispatch_inputs: Vec<_> = inputs.iter().map(|i| i.to_token_stream()).collect();
        dispatch_inputs.push(quote! {
            __handler: __H
        });

        let dispatch_signature = quote! {
            fn #dispatch_method#ty_generics(#(#dispatch_inputs),*) -> #zbus::Result<()>
            #where_clause
        };

        let dispatch_impl = quote! {
            pub #usage #dispatch_signature {
                self.0.dispatch_call(#method_name, #body, move |msg| {
                    if msg.message_type() == #zbus::MessageType::MethodReturn {
                        __handler(msg.body().map_err(Into::into))
                    } else {
                        let err: #zbus::Error = msg.clone().into();
                        __handler(Err(err.into()))
                    }
                })#wait
            }
        };

        if dispatch_only {
            quote! {
                #(#doc)*
                #dispatch_impl
            }
        } else {
            let see_doc = format!(
                "Dispatch a [`Self::{}`] call with a reply handled in the callback scope.",
                method
            );
            quote! {
                #method_impl

                #[doc=#see_doc]
                #[doc=""]
                #[doc="See the documentation for that method and [`zbus::Connection::dispatch_call`] for details."]
                #dispatch_impl
            }
        }
    }
}

fn gen_proxy_property(
    property_name: &str,
    method_name: &str,
    m: &TraitItemMethod,
    proxy_opts: &ProxyOpts,
) -> TokenStream {
    let ProxyOpts {
        usage,
        wait,
        blocking,
        gen_connect,
        ..
    } = proxy_opts;
    let zbus = zbus_path();
    let doc = get_doc_attrs(&m.attrs);
    let signature = &m.sig;
    if signature.inputs.len() > 1 {
        let value = arg_ident(signature.inputs.last().unwrap()).unwrap();
        quote! {
            #(#doc)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                ::std::result::Result::Ok(self.0.set_property(#property_name, #value)#wait?)
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
            ::std::result::Result::Ok(self.0.get_property(#property_name)#wait?)
        };
        let ret_type = if let ReturnType::Type(_, ty) = &signature.output {
            Some(&*ty)
        } else {
            None
        };

        let receive = if *blocking {
            quote! {}
        } else {
            let (_, ty_generics, where_clause) = m.sig.generics.split_for_impl();
            let receive = format_ident!("receive_{}_changed", method_name);
            let gen_doc = format!("Create a stream for the `{}` property changes. \
                                   This is a convenient wrapper around [`zbus::Proxy::receive_property_stream`].",
                                  property_name);
            quote! {
                #[doc = #gen_doc]
                pub async fn #receive#ty_generics(
                    &self
                ) -> #zbus::PropertyStream<'static, <#ret_type as #zbus::ResultAdapter>::Ok>
                #where_clause
                {
                    self.0.receive_property_stream(#property_name).await
                }
            }
        };

        let cached_getter = format_ident!("cached_{}", method_name);
        let cached_doc = format!(
            " Get the cached value of the `{}` property, or `None` if the property is not cached.",
            property_name,
        );

        let connect = format_ident!("connect_{}_changed", method_name);
        let handler = if *blocking {
            parse_quote! { __H: FnMut(Option<&#zbus::zvariant::Value<'_>>) + Send + 'static }
        } else {
            parse_quote! {
                for<'v> __H: FnMut(Option<&'v #zbus::zvariant::Value<'_>>) ->
                    #zbus::export::futures_core::future::BoxFuture<'v, ()> + Send + 'static
            }
        };
        let (proxy_method, link) = if *blocking {
            (
                "zbus::Proxy::connect_property_changed",
                "https://docs.rs/zbus/latest/zbus/blocking/struct.Proxy.html#method.connect_property_changed",
            )
        } else {
            (
                "zbus::Proxy::connect_property_changed",
                "https://docs.rs/zbus/latest/zbus/struct.Proxy.html#method.connect_property_changed",
            )
        };
        let gen_doc = format!(
            " Connect the handler for the `{}` property. This is a convenient wrapper around [`{}`]({}).",
            property_name, proxy_method, link,
        );
        let mut generics = m.sig.generics.clone();
        generics.params.push(parse_quote!(__H));
        {
            let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
            where_clause.predicates.push(handler);
        }

        let (_, ty_generics, where_clause) = generics.split_for_impl();

        let connect_impl = if *gen_connect {
            quote! {
                #[doc = #gen_doc]
                pub #usage fn #connect#ty_generics(
                    &self,
                    mut handler: __H,
                ) -> #zbus::Result<#zbus::PropertyChangedHandlerId>
                #where_clause,
                {
                    self.0.connect_property_changed(#property_name, handler)#wait
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #(#doc)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                #body
            }

            #[doc = #cached_doc]
            pub fn #cached_getter(&self) -> ::std::result::Result<
                ::std::option::Option<<#ret_type as #zbus::ResultAdapter>::Ok>,
                <#ret_type as #zbus::ResultAdapter>::Err>
            {
                self.0.cached_property(#property_name).map_err(::std::convert::Into::into)
            }

            #connect_impl
            #receive
        }
    }
}

struct SetLifetimeS;

impl Fold for SetLifetimeS {
    fn fold_type_reference(&mut self, node: syn::TypeReference) -> syn::TypeReference {
        let mut t = syn::fold::fold_type_reference(self, node);
        t.lifetime = Some(syn::Lifetime::new("'s", Span::call_site()));
        t
    }

    fn fold_lifetime(&mut self, _node: syn::Lifetime) -> syn::Lifetime {
        syn::Lifetime::new("'s", Span::call_site())
    }
}

fn gen_proxy_signal(
    proxy_name: &Ident,
    signal_name: &str,
    snake_case_name: &str,
    m: &TraitItemMethod,
    proxy_opts: &ProxyOpts,
) -> (TokenStream, TokenStream) {
    let ProxyOpts {
        usage,
        wait,
        blocking,
        gen_connect,
        ..
    } = proxy_opts;
    let zbus = zbus_path();
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
    let input_types_s: Vec<_> = SetLifetimeS
        .fold_signature(m.sig.clone())
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
    let args_nth: Vec<Literal> = args
        .iter()
        .enumerate()
        .map(|(i, _)| Literal::usize_unsuffixed(i))
        .collect();

    let (receive_signal, stream_types) = if !proxy_opts.blocking {
        let mut generics = m.sig.generics.clone();
        let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
        for param in generics
            .params
            .iter()
            .filter(|a| matches!(a, syn::GenericParam::Type(_)))
        {
            where_clause
                .predicates
                .push(parse_quote!(#param: #zbus::export::serde::de::Deserialize<'s> + #zbus::zvariant::Type + ::std::fmt::Debug));
        }
        generics.params.push(parse_quote!('s));
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let (receiver_name, stream_name, signal_args, signal_name_ident) = (
            format_ident!("receive_{}", snake_case_name),
            format_ident!("{}Stream", signal_name),
            format_ident!("{}Args", signal_name),
            format_ident!("{}", signal_name),
        );

        let receive_signal_link =
            "https://docs.rs/zbus/latest/zbus/struct.Proxy.html#method.receive_signal";
        let receive_gen_doc = format!(
            "Create a stream that receives `{}` signals.\n\
            \n\
            This a convenient wrapper around [`zbus::Proxy::receive_signal`]({}).",
            signal_name, receive_signal_link,
        );
        let receive_signal = quote! {
            #[doc = #receive_gen_doc]
            #(#doc)*
            pub async fn #receiver_name(&self) -> #zbus::Result<#stream_name>
            {
                self.receive_signal(#signal_name).await.map(#stream_name)
            }
        };

        let stream_gen_doc = format!(
            "A [`stream::Stream`] implementation that yields [`{}`] signals.\n\
            \n\
            Use [`{}::receive_{}`] to create an instance of this type.\n\
            \n\
            [`stream::Stream`]: https://docs.rs/futures/0.3.15/futures/stream/trait.Stream.html",
            signal_name, proxy_name, snake_case_name,
        );
        let signal_args_gen_doc = format!("`{}` signal arguments.", signal_name);
        let args_struct_gen_doc = format!("A `{}` signal.", signal_name);
        let args_impl = if args.is_empty() {
            quote!()
        } else {
            let arg_fields_init = if args.len() == 1 {
                quote! { #(#args)*: args }
            } else {
                quote! { #(#args: args.#args_nth),* }
            };
            quote! {
                impl #signal_name_ident {
                    /// Retrieve the signal arguments.
                    pub fn args#ty_generics(&'s self) -> #zbus::Result<#signal_args #ty_generics>
                        #where_clause
                    {
                        self.0.body::<(#(#input_types),*)>()
                            .map_err(::std::convert::Into::into)
                            .map(|args| {
                                #signal_args {
                                    phantom: ::std::marker::PhantomData,
                                    #arg_fields_init
                                }
                            })
                    }
                }

                #[doc = #signal_args_gen_doc]
                pub struct #signal_args #ty_generics {
                    phantom: std::marker::PhantomData<&'s ()>,
                    #(
                        pub #args: #input_types_s
                     ),*
                }

                impl #impl_generics #signal_args #ty_generics
                    #where_clause
                {
                    #(
                        pub fn #args(&self) -> &#input_types_s {
                            &self.#args
                        }
                     )*
                }

                impl #impl_generics std::fmt::Debug for #signal_args #ty_generics
                    #where_clause
                {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct(#signal_name)
                        #(
                         .field(stringify!(#args), &self.#args)
                        )*
                         .finish()
                    }
                }
            }
        };
        let stream_types = quote! {
            #[doc = #stream_gen_doc]
            pub struct #stream_name(#zbus::SignalStream);

            #zbus::export::static_assertions::assert_impl_all!(
                #stream_name: ::std::marker::Send, ::std::marker::Unpin
            );

            impl #zbus::export::futures_core::stream::Stream for #stream_name {
                type Item = #signal_name_ident;

                fn poll_next(
                    self: ::std::pin::Pin<&mut Self>,
                    cx: &mut ::std::task::Context<'_>,
                    ) -> ::std::task::Poll<::std::option::Option<Self::Item>> {
                    #zbus::export::futures_core::stream::Stream::poll_next(
                        ::std::pin::Pin::new(&mut self.get_mut().0),
                        cx,
                    )
                    .map(|msg| msg.map(#signal_name_ident))
                }
            }

            impl #stream_name {
                /// Consumes `self`, returning the underlying `zbus::SignalStream`.
                pub fn into_inner(self) -> #zbus::SignalStream {
                    self.0
                }

                /// The reference to the underlying `zbus::SignalStream`.
                pub fn inner(&self) -> & #zbus::SignalStream {
                    &self.0
                }
            }

            impl std::ops::Deref for #stream_name {
                type Target = #zbus::SignalStream;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::std::ops::DerefMut for #stream_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            #[doc = #args_struct_gen_doc]
            pub struct #signal_name_ident(::std::sync::Arc<#zbus::Message>);

            #args_impl
        };

        (receive_signal, stream_types)
    } else {
        (quote! {}, quote! {})
    };

    let input_types_s: Vec<_> = SetLifetimeS
        .fold_signature(m.sig.clone())
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(p) => Some(p.ty.clone()),
            _ => None,
        })
        .collect();

    let handler = if *blocking {
        quote! { ::std::ops::FnMut(#(#input_types),*) }
    } else if input_types == input_types_s {
        quote! {
            ::std::ops::FnMut(
                #(#input_types),*
            ) -> #zbus::export::futures_core::future::BoxFuture<'static, ()>
        }
    } else {
        quote! {
            for<'s>
            ::std::ops::FnMut(
                #(#input_types_s),*
            ) -> #zbus::export::futures_core::future::BoxFuture<'s, ()>
        }
    };

    let (proxy_method, link) = if *blocking {
        (
            "zbus::Proxy::connect_signal",
            "https://docs.rs/zbus/latest/zbus/blocking/struct.Proxy.html#method.connect_signal",
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
    {
        let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
        for param in generics
            .params
            .iter()
            .filter(|a| matches!(a, syn::GenericParam::Type(_)))
        {
            where_clause
                .predicates
                .push(parse_quote!(#param: #zbus::export::serde::de::DeserializeOwned + #zbus::zvariant::Type + ::std::fmt::Debug));
        }
        where_clause
            .predicates
            .push(parse_quote!(__H: #handler + ::std::marker::Send + 'static));
    }
    generics.params.push(parse_quote!(__H));

    let do_nothing = if *blocking {
        quote!(())
    } else {
        quote!(Box::pin(async {}))
    };

    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let connect_signal = if *gen_connect {
        quote! {
            #[doc = #gen_doc]
            #(#doc)*
            pub #usage fn #method#ty_generics(
                &self,
                mut handler: __H,
            ) -> #zbus::fdo::Result<#zbus::SignalHandlerId>
            #where_clause,
            {
                self.0.connect_signal(#signal_name, move |m| {
                    match m.body() {
                        Ok((#(#args),*)) => handler(#(#args),*),
                        // TODO log errors, or allow a fallback?
                        Err(_) => #do_nothing,
                    }
                })#wait
            }
        }
    } else {
        quote! {}
    };

    let methods = quote! {
        #connect_signal
        #receive_signal
    };

    (methods, stream_types)
}
