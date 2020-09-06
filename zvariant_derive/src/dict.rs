use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::Meta::Path;
use syn::NestedMeta::Meta;
use syn::{Data, DeriveInput, Type, TypePath};

use crate::utils::*;

pub fn expand_type_derive(input: DeriveInput) -> TokenStream {
    let name = match input.data {
        Data::Struct(_) => input.ident,
        _ => panic!("Only works with structure"),
    };

    let zv = get_zvariant_crate_ident();
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::#zv::Type for #name #ty_generics
        #where_clause
        {
            fn signature() -> ::#zv::Signature<'static> {
                ::#zv::Signature::from_str_unchecked("a{sv}")
            }
        }
    };

    expanded.into()
}

pub fn expand_serialize_derive(input: DeriveInput) -> TokenStream {
    let (name, data) = match input.data {
        Data::Struct(data) => (input.ident, data),
        _ => panic!("Only works with structure"),
    };

    let zv = get_zvariant_crate_ident();
    let mut entries = quote! {};

    for f in &data.fields {
        let attrs = parse_item_attributes(&f.attrs).unwrap();
        let name = &f.ident;
        let dict_name = attrs
            .iter()
            .find_map(|x| match x {
                ItemAttribute::Rename(n) => Some(n.to_string()),
            })
            .unwrap_or_else(|| f.ident.as_ref().unwrap().to_string());

        let is_option = match &f.ty {
            Type::Path(TypePath {
                path: syn::Path { segments, .. },
                ..
            }) => segments.last().unwrap().ident == "Option",
            _ => false,
        };

        let e = if is_option {
            quote! {
                if self.#name.is_some() {
                    map.serialize_entry(#dict_name, &#zv::SerializeValue(self.#name.as_ref().unwrap()))?;
                }
            }
        } else {
            quote! {
                map.serialize_entry(#dict_name, &#zv::SerializeValue(&self.#name))?;
            }
        };

        entries.extend(e);
    }

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::#zv::export::serde::ser::Serialize for #name #ty_generics
        #where_clause
        {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: ::#zv::export::serde::ser::Serializer,
            {
                use ::#zv::export::serde::ser::SerializeMap;

                // zbus doesn't care about number of entries (it would need bytes instead)
                let mut map = serializer.serialize_map(None)?;
                #entries
                map.end()
            }
        }

    };

    expanded.into()
}

pub fn expand_deserialize_derive(input: DeriveInput) -> TokenStream {
    let (name, data) = match input.data {
        Data::Struct(data) => (input.ident, data),
        _ => panic!("Only works with structure"),
    };

    let mut deny_unknown_fields = false;
    for meta_item in input.attrs.iter().flat_map(get_meta_items).flatten() {
        match &meta_item {
            Meta(Path(p)) if p.is_ident("deny_unknown_fields") => {
                deny_unknown_fields = true;
            }
            _ => panic!("unsupported attribute"),
        }
    }

    let visitor = format_ident!("{}Visitor", name);
    let zv = get_zvariant_crate_ident();
    let mut fields = Vec::new();
    let mut req_fields = Vec::new();
    let mut dict_names = Vec::new();
    let mut entries = Vec::new();

    for f in &data.fields {
        let attrs = parse_item_attributes(&f.attrs).unwrap();
        let name = &f.ident;
        let dict_name = attrs
            .iter()
            .find_map(|x| match x {
                ItemAttribute::Rename(n) => Some(n.to_string()),
            })
            .unwrap_or_else(|| f.ident.as_ref().unwrap().to_string());

        let is_option = match &f.ty {
            Type::Path(TypePath {
                path: syn::Path { segments, .. },
                ..
            }) => segments.last().unwrap().ident == "Option",
            _ => false,
        };

        entries.push(quote! {
            #dict_name => {
                if let Ok(val) = value.try_into() {
                    #name = Some(val);
                }
            }
        });

        dict_names.push(dict_name);
        fields.push(name);

        if !is_option {
            req_fields.push(name);
        }
    }

    let fallback = if deny_unknown_fields {
        quote! {
            field => {
                return Err(<M::Error as ::#zv::export::serde::de::Error>::unknown_field(
                    field,
                    &[#(#dict_names),*],
                ));
            }
        }
    } else {
        quote! {
            _ => {
                continue;
            }
        }
    };
    entries.push(fallback);

    let (_, ty_generics, _) = input.generics.split_for_impl();
    let mut generics = input.generics.clone();
    let def = syn::LifetimeDef {
        attrs: Vec::new(),
        lifetime: syn::Lifetime::new("'de", Span::call_site()),
        colon_token: None,
        bounds: Punctuated::new(),
    };
    generics.params = Some(syn::GenericParam::Lifetime(def))
        .into_iter()
        .chain(generics.params)
        .collect();

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::#zv::export::serde::de::Deserialize<'de> for #name #ty_generics
        #where_clause
        {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: ::#zv::export::serde::de::Deserializer<'de>,
            {
                struct #visitor #ty_generics(std::marker::PhantomData<#name #ty_generics>);

                impl #impl_generics ::#zv::export::serde::de::Visitor<'de> for #visitor #ty_generics {
                    type Value = #name #ty_generics;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a dictionary")
                    }

                    fn visit_map<M>(
                        self,
                        mut access: M,
                    ) -> std::result::Result<Self::Value, M::Error>
                    where
                        M: ::#zv::export::serde::de::MapAccess<'de>,
                    {
                        use std::convert::TryInto;

                        #( let mut #fields = Default::default(); )*

                        // does not check duplicated fields, since those shouldn't exist in stream
                        while let Some((key, value)) = access.next_entry::<&str, ::#zv::Value>()? {
                            match key {
                                #(#entries)*
                            }
                        }

                        #(let #req_fields = if let Some(val) = #req_fields {
                            val
                        } else {
                            return Err(<M::Error as ::#zv::export::serde::de::Error>::missing_field(
                                stringify!(#req_fields),
                            ));
                        };)*

                        Ok(#name { #(#fields),* })
                    }
                }


                deserializer.deserialize_map(#visitor(std::marker::PhantomData))
            }
        }
    };

    expanded.into()
}
