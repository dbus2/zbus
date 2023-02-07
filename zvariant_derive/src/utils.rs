use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{
    spanned::Spanned, Attribute, Lit, LitStr, Meta, MetaList, NestedMeta, Result, Type, TypePath,
};

pub fn zvariant_path() -> TokenStream {
    if let Ok(FoundCrate::Name(name)) = crate_name("zvariant") {
        let ident = format_ident!("{}", name);
        quote! { ::#ident }
    } else if let Ok(FoundCrate::Name(name)) = crate_name("zbus") {
        let ident = format_ident!("{}", name);
        quote! { ::#ident::zvariant }
    } else {
        quote! { ::zvariant }
    }
}

// find the #[@attr_name] attribute in @attrs
fn find_attribute_meta(attrs: &[Attribute], attr_name: &str) -> Result<Option<MetaList>> {
    let meta = match attrs.iter().find(|a| a.path.is_ident(attr_name)) {
        Some(a) => a.parse_meta(),
        _ => return Ok(None),
    };
    match meta? {
        Meta::List(n) => Ok(Some(n)),
        _ => panic!("wrong meta type"),
    }
}

fn parse_attribute(meta: &NestedMeta) -> (&Ident, Option<&LitStr>) {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => panic!("wrong meta type"),
    };
    let meta = match meta {
        Meta::Path(p) => return (p.get_ident().unwrap(), None),
        Meta::NameValue(n) => n,
        _ => panic!("wrong meta type"),
    };
    let value = match &meta.lit {
        Lit::Str(s) => s,
        _ => panic!("wrong meta type"),
    };

    let ident = match meta.path.get_ident() {
        None => panic!("missing ident"),
        Some(ident) => ident,
    };

    (ident, Some(value))
}

fn match_attribute_with_value<'a>(
    attr: &str,
    ident: &Ident,
    value: Option<&'a LitStr>,
    span: Span,
) -> Result<Option<&'a LitStr>> {
    if ident == attr {
        if let Some(value) = value {
            Ok(Some(value))
        } else {
            Err(syn::Error::new(
                span,
                format!("attribute `{attr}` must have a value"),
            ))
        }
    } else {
        Ok(None)
    }
}

fn match_attribute_without_value(
    attr: &str,
    ident: &Ident,
    value: Option<&LitStr>,
    span: Span,
) -> Result<bool> {
    if ident == attr {
        if value.is_some() {
            Err(syn::Error::new(
                span,
                format!("attribute `{attr}` must not have a value"),
            ))
        } else {
            Ok(true)
        }
    } else {
        Ok(false)
    }
}

fn iter_item_attributes(attrs: &[Attribute]) -> Result<impl Iterator<Item = NestedMeta>> {
    let meta = find_attribute_meta(attrs, "zvariant")?;

    Ok(meta.into_iter().flat_map(|meta| meta.nested.into_iter()))
}

static ALLOWED_ATTRS: &[&str] = &["signature", "rename_all", "deny_unknown_fields", "rename"];

fn is_valid_attr(ident: &Ident) -> bool {
    ALLOWED_ATTRS.iter().any(|attr| ident == attr)
}

macro_rules! def_attrs {
    (@attr_ty with) => {Option<String>};
    (@attr_ty without) => {bool};
    (@match_attr with $attr_name:ident, $ident:ident, $value:expr, $span:expr, $self:ident) => {
        if let Some(value) = match_attribute_with_value(stringify!($attr_name), $ident, $value, $span)? {
            if $self.$attr_name.is_none() {
                $self.$attr_name = Some(value.value());
                continue;
            } else {
                return Err(crate::syn::Error::new(
                    $span,
                    concat!("duplicate `", stringify!($attr_name), "` attribute")
                ));
            }
        }
    };
    (@match_attr without $attr_name:ident, $ident:ident, $value:expr, $span:expr, $self:ident) => {
        if match_attribute_without_value(stringify!($attr_name), $ident, $value, $span)? {
            if !$self.$attr_name {
                $self.$attr_name = true;
                continue;
            } else {
                return Err(crate::syn::Error::new(
                    $span,
                    concat!("duplicate `", stringify!($attr_name), "` attribute")
                ));
            }
        }
    };
    ($name:ident, $what:literal, $($attr_name:ident $kind:tt),+) => {
        #[derive(Default, Clone, Debug)]
        pub struct $name {
            $(pub $attr_name: def_attrs!(@attr_ty $kind)),+
        }

        impl $name {
            pub fn parse(attrs: &[Attribute]) -> Result<Self> {
                let mut parsed = $name::default();

                for nested_meta in iter_item_attributes(attrs)? {
                    let (ident, value) = parse_attribute(&nested_meta);
                    let span = nested_meta.span();

                    // This creates subsequent if blocks for simplicity. Any block that is taken
                    // either returns an error or sets the attribute field and continues.
                    $(
                        def_attrs!(@match_attr $kind $attr_name, ident, value, span, parsed);
                    )+

                    // None of the if blocks have been taken, return the appropriate error.
                    return Err(syn::Error::new(span, if is_valid_attr(ident) {
                        format!(concat!("attribute `{}` is not allowed on ", $what), ident)
                    } else {
                        format!("unknown attribute `{ident}`")
                    }))
                }

                Ok(parsed)
            }
        }
    };
}

def_attrs!(StructAttributes, "struct", signature with, rename_all with, deny_unknown_fields without);
def_attrs!(FieldAttributes, "field", rename with);

/// Convert to pascal or camel case, assuming snake case.
///
/// If `s` is already in pascal or camel case, should yield the same result.
pub fn pascal_or_camel_case(s: &str, is_pascal_case: bool) -> String {
    let mut result = String::new();
    let mut capitalize = is_pascal_case;
    let mut first = true;
    for ch in s.chars() {
        if ch == '_' {
            capitalize = true;
        } else if capitalize {
            result.push(ch.to_ascii_uppercase());
            capitalize = false;
        } else if first && !is_pascal_case {
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }

        if first {
            first = false;
        }
    }
    result
}

/// Convert to snake case, assuming pascal case.
///
/// If `s` is already in snake case, should yield the same result.
pub fn snake_case(s: &str) -> String {
    let mut snake = String::new();
    for ch in s.chars() {
        if ch.is_ascii_uppercase() && !snake.is_empty() {
            snake.push('_');
        }
        snake.push(ch.to_ascii_lowercase());
    }
    snake
}

pub fn ty_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => segments.last().unwrap().ident == "Option",
        _ => false,
    }
}
