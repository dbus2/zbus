use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{Data, DeriveInput, Fields, Ident, Variant};

use crate::utils::*;

pub fn expand_type_derive(input: DeriveInput) -> TokenStream {
    let name = match input.data {
        Data::Enum(_) => input.ident,
        _ => panic!("Only works with enum"),
    };

    let zv = get_zvariant_crate_ident();
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::#zv::Type for #name #ty_generics
        #where_clause
        {
            fn signature() -> ::#zv::Signature<'static> {
                ::#zv::Signature::from_str_unchecked("(sv)")
            }
        }
    }
}

fn serialize_variant(this: &Ident, variant: &Variant) -> TokenStream {
    let zv = get_zvariant_crate_ident();
    let variant_ident = &variant.ident;
    let type_name = this.to_string();
    let variant_name = variant.ident.to_string();

    let case = match variant.fields {
        Fields::Unit => {
            quote! {
                #this::#variant_ident
            }
        }
        Fields::Unnamed(_) => {
            let field_names = (0..variant.fields.len()).map(|i| format_ident!("__field{}", i));
            quote! {
                #this::#variant_ident(#(ref #field_names),*)
            }
        }
        Fields::Named(_) => {
            let members = variant.fields.iter().map(|f| &f.ident);
            quote! {
                #this::#variant_ident { #(ref #members),* }
            }
        }
    };

    let body = match variant.fields {
        Fields::Unit => {
            quote! {
                let mut __struct = ::#zv::export::serde::ser::Serializer::serialize_struct(
                        __serializer, #type_name, 2)?;
                ::#zv::export::serde::ser::SerializeStruct::serialize_field(
                    &mut __struct, "type", #variant_name)?;
                ::#zv::export::serde::ser::SerializeStruct::serialize_field(
                    &mut __struct, "data", &::#zv::SerializeValue(&()))?;
                ::#zv::export::serde::ser::SerializeStruct::end(__struct)
            }
        }
        Fields::Unnamed(_) => {
            let field_names = (0..variant.fields.len()).map(|i| format_ident!("__field{}", i));
            quote! {
                let mut __struct = ::#zv::export::serde::ser::Serializer::serialize_struct(
                    __serializer, #type_name, 2)?;
                ::#zv::export::serde::ser::SerializeStruct::serialize_field(
                    &mut __struct, "type", #variant_name)?;
                ::#zv::export::serde::ser::SerializeStruct::serialize_field(
                    &mut __struct, "data", &::#zv::SerializeValue(&(#(#field_names),*)))?;
                ::#zv::export::serde::ser::SerializeStruct::end(__struct)
            }
        }
        Fields::Named(_) => {
            let members = variant.fields.iter().map(|f| &f.ident);
            quote! {
                let mut __struct = ::#zv::export::serde::ser::Serializer::serialize_struct(
                    __serializer, #type_name, 2)?;
                ::#zv::export::serde::ser::SerializeStruct::serialize_field(
                    &mut __struct, "type", #variant_name)?;
                ::#zv::export::serde::ser::SerializeStruct::serialize_field(
                    &mut __struct, "data", &::#zv::SerializeValue(&(#(#members),*)))?;
                ::#zv::export::serde::ser::SerializeStruct::end(__struct)
            }
        }
    };

    quote! {
        #case => { #body },
    }
}

pub fn expand_serialize_derive(input: DeriveInput) -> TokenStream {
    let (name, data) = match input.data {
        Data::Enum(data) => (input.ident, data),
        _ => panic!("Only works with structure"),
    };

    let zv = get_zvariant_crate_ident();

    let arms: Vec<_> = data
        .variants
        .iter()
        .map(|variant| serialize_variant(&name, variant))
        .collect();

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::#zv::export::serde::ser::Serialize for #name #ty_generics
        #where_clause
        {
            fn serialize<S>(&self, __serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: ::#zv::export::serde::ser::Serializer,
            {
                match *self {
                    #(#arms)*
                }
            }
        }
    }
}

fn deserialize_variant(this: &Ident, variant: &Variant) -> TokenStream {
    let zv = get_zvariant_crate_ident();
    let ident = &variant.ident;
    let variant_name = variant.ident.to_string();

    let body = match variant.fields {
        Fields::Unit => {
            quote! {
                let _ = seq.next_element::<#zv::DeserializeValue<()>>()?
                    .ok_or_else(|| #zv::export::serde::de::Error::invalid_length(1, &self))?;
                Ok(#this::#ident)
            }
        }
        Fields::Unnamed(_) => {
            let field_names: Vec<_> = (0..variant.fields.len())
                .map(|i| format_ident!("__field{}", i))
                .collect();
            quote! {
                let (#(#field_names),*) = seq.next_element::<#zv::DeserializeValue<_>>()?
                    .map(|v| v.0)
                    .ok_or_else(|| #zv::export::serde::de::Error::invalid_length(1, &self))?;
                Ok(#this::#ident(#(#field_names),*))
            }
        }
        Fields::Named(_) => {
            let members: Vec<_> = variant.fields.iter().map(|f| &f.ident).collect();
            quote! {
                let (#(#members),*) = seq.next_element::<#zv::DeserializeValue<_>>()?
                    .map(|v| v.0)
                    .ok_or_else(|| #zv::export::serde::de::Error::invalid_length(1, &self))?;
                Ok(#this::#ident { #(#members),* })
            }
        }
    };

    quote! {
        #variant_name => { #body },
    }
}

pub fn expand_deserialize_derive(input: DeriveInput) -> TokenStream {
    let (name, data) = match input.data {
        Data::Enum(data) => (input.ident, data),
        _ => panic!("Only works with enum"),
    };

    let zv = get_zvariant_crate_ident();
    let type_name = name.to_string();
    let arms: Vec<_> = data
        .variants
        .iter()
        .map(|variant| deserialize_variant(&name, variant))
        .collect();
    let variants: Vec<_> = data
        .variants
        .iter()
        .map(|variant| variant.ident.to_string())
        .collect();

    let visitor = format_ident!("{}Visitor", name);
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

    quote! {
        impl #impl_generics ::#zv::export::serde::de::Deserialize<'de> for #name #ty_generics
        #where_clause
        {
            fn deserialize<D>(__deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: ::#zv::export::serde::de::Deserializer<'de>,
            {
                struct #visitor #ty_generics(std::marker::PhantomData<#name #ty_generics>);

                impl #impl_generics ::#zv::export::serde::de::Visitor<'de> for #visitor #ty_generics {
                    type Value = #name #ty_generics;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("A tagged enum")
                    }

                    fn visit_seq<V>(self, mut seq: V) -> std::result::Result<Self::Value, V::Error>
                    where
                        V: #zv::export::serde::de::SeqAccess<'de>,
                    {
                        let type_name: &str = seq.next_element()?
                            .ok_or_else(|| #zv::export::serde::de::Error::invalid_length(0, &self))?;

                        match type_name {
                            #(#arms)*
                            unknown => {
                                Err(#zv::export::serde::de::Error::unknown_variant(unknown, &[#(#variants),*]))
                            }
                        }
                    }
                }

                const FIELDS: &[&str] = &["type", "data"];

                __deserializer.deserialize_struct(#type_name, FIELDS, #visitor(std::marker::PhantomData))
            }
        }
    }
}
