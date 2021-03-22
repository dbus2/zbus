use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{self, Attribute, Data, DataEnum, DeriveInput, Fields, Generics, Ident};

pub fn expand_derive(ast: DeriveInput) -> TokenStream {
    match ast.data {
        Data::Struct(ds) => match ds.fields {
            Fields::Named(_) | Fields::Unnamed(_) => {
                impl_struct(ast.ident, ast.generics, ds.fields)
            }
            Fields::Unit => impl_unit_struct(ast.ident, ast.generics),
        },
        Data::Enum(data) => impl_enum(ast.ident, ast.generics, ast.attrs, data),
        _ => panic!("Only structures and enums supported at the moment"),
    }
}

fn impl_struct(name: Ident, generics: Generics, fields: Fields) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let signature = signature_for_struct(fields);

    quote! {
        impl #impl_generics zvariant::Type for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                #signature
            }
        }
    }
}

fn signature_for_struct(fields: Fields) -> TokenStream {
    let field_types = fields.iter().map(|field| field.ty.to_token_stream());
    let new_type = match fields {
        Fields::Named(_) => false,
        Fields::Unnamed(_) if field_types.len() == 1 => true,
        Fields::Unnamed(_) => false,
        Fields::Unit => panic!("signature_for_struct must not be called for unit fields"),
    };
    if new_type {
        quote! {
            #(
                <#field_types as zvariant::Type>::signature()
             )*
        }
    } else {
        quote! {
            let mut s = String::from("(");
            #(
                s.push_str(<#field_types as zvariant::Type>::signature().as_str());
            )*
            s.push_str(")");

            zvariant::Signature::from_string_unchecked(s)
        }
    }
}

fn impl_unit_struct(name: Ident, generics: Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics zvariant::Type for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                zvariant::Signature::from_str_unchecked("")
            }
        }
    }
}

fn impl_enum(
    name: Ident,
    generics: Generics,
    attrs: Vec<Attribute>,
    data: DataEnum,
) -> TokenStream {
    let repr: TokenStream = match attrs.iter().find(|attr| attr.path.is_ident("repr")) {
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

    quote! {
        impl #impl_generics zvariant::Type for #name #ty_generics #where_clause {
            #[inline]
            fn signature() -> zvariant::Signature<'static> {
                <#repr as zvariant::Type>::signature()
            }
        }
    }
}
