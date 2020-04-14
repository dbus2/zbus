extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, Attribute, Data, DataEnum, DeriveInput, Fields, Generics, Ident};

// TODO: Note about enums requiring repr attr. and serde-repr crate in the docs.

#[proc_macro_derive(VariantValue)]
pub fn variant_value_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Struct(ds) => match ds.fields {
            Fields::Named(_) => impl_struct(ast.ident, ast.generics, ds.fields),
            Fields::Unnamed(_) => impl_struct(ast.ident, ast.generics, ds.fields),
            Fields::Unit => impl_unit_struct(ast.ident, ast.generics),
        },
        Data::Enum(data) => impl_enum(ast.ident, ast.generics, ast.attrs, data),
        _ => panic!("Only structures supported at the moment"),
    }
}

fn impl_struct(name: Ident, generics: Generics, fields: Fields) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let field_types = fields.iter().map(|field| field.ty.to_token_stream());
    let signature = if field_types.len() == 1 {
        quote! {
            #(
                <#field_types as zvariant::VariantValue>::signature()
             )*
        }
    } else {
        quote! {
                let mut s = String::from("(");
                #(
                    s.push_str(<#field_types as zvariant::VariantValue>::signature().as_str());
                )*
                s.push_str(")");

                zvariant::Signature::from(s)
        }
    };
    let expended = quote! {
        impl #impl_generics zvariant::VariantValue for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                #signature
            }
        }
    };

    TokenStream::from(expended)
}

fn impl_unit_struct(name: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expended = quote! {
        impl #impl_generics zvariant::VariantValue for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                zvariant::Signature::from("")
            }
        }
    };

    TokenStream::from(expended)
}

fn impl_enum(
    name: Ident,
    generics: Generics,
    attrs: Vec<Attribute>,
    data: DataEnum,
) -> TokenStream {
    let repr: proc_macro2::TokenStream = match attrs.iter().find(|attr| attr.path.is_ident("repr"))
    {
        Some(repr_attr) => repr_attr
            .parse_args()
            .expect("Failed to parse `#[repr(...)]` attribute"),
        None => quote! { u32 },
    };

    for variant in data.variants {
        // Ensure all variants of the enum are unit type
        match variant.fields {
            Fields::Unit => (),
            _ => panic!("`{}` must be a unit variant", variant.ident.to_string()),
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let expended = quote! {
        impl #impl_generics zvariant::VariantValue for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                <#repr as zvariant::VariantValue>::signature()
            }
        }
    };

    TokenStream::from(expended)
}
