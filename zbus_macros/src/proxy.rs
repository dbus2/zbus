use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use regex::Regex;
use syn::{
    self, fold::Fold, parse_quote, spanned::Spanned, AttributeArgs, FnArg, Ident, ItemTrait,
    NestedMeta, ReturnType, TraitItemMethod, Type,
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

    let zbus = zbus_path();

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
    let proxy_name = Ident::new(&proxy_name, Span::call_site());
    let ident = input.ident.to_string();
    let name = iface_name.unwrap_or(format!("org.freedesktop.{}", ident));
    let default_path = default_path.unwrap_or(format!("/org/freedesktop/{}", ident));
    let default_service = default_service.unwrap_or_else(|| name.clone());
    let mut methods = TokenStream::new();
    let mut stream_types = TokenStream::new();
    let mut has_properties = false;
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
                has_properties = true;
                gen_proxy_property(&name, m, &async_opts)
            } else if is_signal {
                let (method, types) =
                    gen_proxy_signal(&proxy_name, &name, &method_name, m, &async_opts);
                stream_types.extend(types);

                method
            } else {
                gen_proxy_method_call(&name, &method_name, m, &async_opts)
            };
            methods.extend(m);
        }
    }

    let AsyncOpts { usage, wait, .. } = async_opts;
    let (proxy_doc, proxy_struct, connection, build) = if azync {
        let sync_proxy = Ident::new(&format!("{}Proxy", input.ident), Span::call_site());
        let doc = format!("Asynchronous sibling of [`{}`].", sync_proxy);
        let connection = quote! { #zbus::azync::Connection };
        let proxy = quote! { #zbus::azync::Proxy };
        let build = Ident::new("build_async", Span::call_site());

        (doc, proxy, connection, build)
    } else {
        let doc = String::from("");
        let connection = quote! { #zbus::Connection };
        let proxy = quote! { #zbus::Proxy };
        let build = Ident::new("build", Span::call_site());

        (doc, proxy, connection, build)
    };

    quote! {
        impl<'a> #zbus::ProxyDefault for #proxy_name<'a> {
            const INTERFACE: &'static str = #name;
            const DESTINATION: &'static str = #default_service;
            const PATH: &'static str = #default_path;
        }

        #[doc = #proxy_doc]
        #(#doc)*
        #[derive(Clone, Debug)]
        pub struct #proxy_name<'c>(#proxy_struct<'c>);

        impl<'c> #proxy_name<'c> {
            /// Creates a new proxy with the default service & path.
            pub #usage fn new(conn: &#connection) -> #zbus::Result<#proxy_name<'c>> {
                Self::builder(conn).#build()#wait
            }

            /// Returns a customizable builder for this proxy.
            pub fn builder(conn: &#connection) -> #zbus::ProxyBuilder<'c, Self> {
                #zbus::ProxyBuilder::new(conn)
                    .cache_properties(#has_properties)
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

        impl<'c> ::std::convert::From<#zbus::azync::Proxy<'c>> for #proxy_name<'c> {
            fn from(proxy: #zbus::azync::Proxy<'c>) -> Self {
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

        impl<'c> #zbus::export::zvariant::Type for #proxy_name<'c> {
            fn signature() -> #zbus::export::zvariant::Signature<'static> {
                #zbus::export::zvariant::OwnedObjectPath::signature()
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
    async_opts: &AsyncOpts,
) -> TokenStream {
    let AsyncOpts { usage, wait, azync } = async_opts;
    let zbus = zbus_path();
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
    let method = Ident::new(snake_case_name, Span::call_site());
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
            #param: #serde_bound + #zbus::export::zvariant::Type
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
                let object_path: #zbus::export::zvariant::OwnedObjectPath =
                    self.0.call(
                        #method_name,
                        &(#(#args),*),
                    )
                    #wait?;
                #proxy::builder(&self.0.connection())
                    .path(object_path)?
                    .build()
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
        quote! {
            #(#doc)*
            pub #usage #signature {
                let reply = self.0.call(#method_name, #body)#wait?;
                ::std::result::Result::Ok(reply)
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
        quote! {
            #(#doc)*
            #[allow(clippy::needless_question_mark)]
            pub #usage #signature {
                #body
            }
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
    async_opts: &AsyncOpts,
) -> (TokenStream, TokenStream) {
    let AsyncOpts { usage, wait, azync } = async_opts;
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

    let (receive_signal, stream_types) = if async_opts.azync {
        let mut generics = m.sig.generics.clone();
        let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
        for param in generics
            .params
            .iter()
            .filter(|a| matches!(a, syn::GenericParam::Type(_)))
        {
            where_clause
                .predicates
                .push(parse_quote!(#param: #zbus::export::serde::de::Deserialize<'s> + #zbus::export::zvariant::Type + ::std::fmt::Debug));
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
            "https://docs.rs/zbus/latest/zbus/azync/struct.Proxy.html#method.receive_signal";
        let receive_gen_doc = format!(
            "Create a stream that receives `{}` signals.\n\
            \n\
            This a convenient wrapper around [`zbus::azync::Proxy::receive_signal`]({}).",
            signal_name, receive_signal_link,
        );
        let receive_signal = quote! {
            #[doc = #receive_gen_doc]
            #(#doc)*
            pub async fn #receiver_name(&self) -> #zbus::Result<#stream_name<'c>>
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
            pub struct #stream_name<'s>(#zbus::azync::SignalStream<'s>);

            #zbus::export::static_assertions::assert_impl_all!(
                #stream_name<'_>: ::std::marker::Send, ::std::marker::Unpin
            );

            impl #zbus::export::futures_core::stream::Stream for #stream_name<'_> {
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

            impl<'s> #stream_name<'s> {
                /// Consumes `self`, returning the underlying `zbus::azync::SignalStream`.
                pub fn into_inner(self) -> #zbus::azync::SignalStream<'s> {
                    self.0
                }

                /// The reference to the underlying `zbus::azync::SignalStream`.
                pub fn inner(&self) -> & #zbus::azync::SignalStream<'s> {
                    &self.0
                }
            }

            impl<'s> std::ops::Deref for #stream_name<'s> {
                type Target = #zbus::azync::SignalStream<'s>;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl ::std::ops::DerefMut for #stream_name<'_> {
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

    let handler = if *azync {
        quote! {
            ::std::ops::FnMut(
                #(#input_types),*
            ) -> #zbus::export::futures_core::future::BoxFuture<'static, #zbus::Result<()>>
        }
    } else {
        quote! { ::std::ops::FnMut(#(#input_types),*) -> #zbus::Result<()> }
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
    {
        let where_clause = generics.where_clause.get_or_insert(parse_quote!(where));
        for param in generics
            .params
            .iter()
            .filter(|a| matches!(a, syn::GenericParam::Type(_)))
        {
            where_clause
                .predicates
                .push(parse_quote!(#param: #zbus::export::serde::de::DeserializeOwned + #zbus::export::zvariant::Type + ::std::fmt::Debug));
        }
        where_clause
            .predicates
            .push(parse_quote!(__H: #handler + ::std::marker::Send + 'static));
    }
    generics.params.push(parse_quote!(__H));

    let (_, ty_generics, where_clause) = generics.split_for_impl();
    let methods = quote! {
        #[doc = #gen_doc]
        #(#doc)*
        pub #usage fn #method#ty_generics(
            &self,
            mut handler: __H,
        ) -> #zbus::fdo::Result<#zbus::SignalHandlerId>
        #where_clause,
        {
            self.0.connect_signal(#signal_name, move |m| {
                let (#(#args),*) = m.body().expect("Incorrect signal signature");

                handler(#(#args),*)
            })#wait
        }

        #receive_signal
    };

    (methods, stream_types)
}
