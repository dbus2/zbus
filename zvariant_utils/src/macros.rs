//! Contains utilities useful for development of procedural macros, in particular parsing the
//! attributes.

use proc_macro2::{Ident, Span};
use syn::{
    spanned::Spanned, Attribute, Lit, LitStr, Meta, MetaList, NestedMeta, Result, Type, TypePath,
};

// find the #[@attr_name] attribute in @attrs
fn find_attribute_meta(attrs: &[Attribute], attr_name: &str) -> Result<Option<MetaList>> {
    let meta = match attrs.iter().find(|a| a.path.is_ident(attr_name)) {
        Some(a) => a.parse_meta(),
        _ => return Ok(None),
    }?;
    match meta {
        Meta::List(n) => Ok(Some(n)),
        _ => Err(syn::Error::new(
            meta.span(),
            format!("{attr_name} meta must specify a meta list"),
        )),
    }
}

/// Parses a [`NestedMeta`] into an attribute identifier with an optional string literal value.
pub fn parse_attribute(meta: &NestedMeta) -> Result<(&Ident, Option<&LitStr>)> {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => {
            return Err(syn::Error::new(
                meta.span(),
                "expected meta, found a literal",
            ))
        }
    };
    let meta = match meta {
        Meta::Path(p) => return Ok((p.get_ident().unwrap(), None)),
        Meta::NameValue(n) => n,
        Meta::List(_) => {
            return Err(syn::Error::new(
                meta.span(),
                "expected either a path or a name-value meta, found a list",
            ))
        }
    };
    let value = match &meta.lit {
        Lit::Str(s) => s,
        _ => {
            return Err(syn::Error::new(
                meta.lit.span(),
                "the value must be a string literal",
            ))
        }
    };

    let ident = match meta.path.get_ident() {
        None => panic!("missing ident"),
        Some(ident) => ident,
    };

    Ok((ident, Some(value)))
}

/// Compares `ident` and `attr` and in case they match ensures `value` is `Some`. Returns `true` in
/// case `ident` and `attr` match, otherwise false.
///
/// # Errors
///
/// Returns an error in case `ident` and `attr` match but the value is not `Some`.
pub fn match_attribute_with_value<'a>(
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

/// Compares `ident` and `attr` and in case they match ensures `value` is `None`. Returns `true` in
/// case `ident` and `attr` match, otherwise false.
///
/// # Errors
///
/// Returns an error in case `ident` and `attr` match but the value is not `None`.
pub fn match_attribute_without_value(
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

/// Returns an iterator over the contents of all [`MetaList`]s with the specified identifier in an
/// array of [`Attribute`]s.
pub fn iter_meta_lists(
    attrs: &[Attribute],
    list_name: &str,
) -> Result<impl Iterator<Item = NestedMeta>> {
    let meta = find_attribute_meta(attrs, list_name)?;

    Ok(meta.into_iter().flat_map(|meta| meta.nested.into_iter()))
}

/// Generates one or more structures used for parsing attributes in proc macros.
///
/// Generated structures have one static method called parse that accepts a slice of [`Attribute`]s.
/// The method finds attributes that contain meta lists (look like `#[your_custom_ident(...)]`) and
/// fills a newly allocated structure with values of the attributes if any.
///
/// The expected input looks as follows:
///
/// ```ignore
/// def_attrs! {
///     crate zvariant;
///
///     /// A comment.
///     pub StructAttributes("struct") { foo with, bar with, baz without };
///     #[derive(Hash)]
///     FieldAttributes("field") { field_attr with };
/// }
/// ```
///
/// Here we see multiple entries: an entry for an attributes group called `StructAttributes` and
/// another one for `FieldAttributes`. The former has three defined attributes: `foo`, `bar` and
/// `baz`. The generated structures will look like this in that case:
///
/// ```ignore
/// /// A comment.
/// #[derive(Default, Clone, Debug)]
/// pub struct StructAttributes {
///     foo: Option<String>,
///     bar: Option<String>,
///     baz: bool,
/// }
///
/// #[derive(Hash)]
/// #[derive(Default, Clone, Debug)]
/// struct FieldAttributes {
///     field_attr: Option<String>,
/// }
/// ```
///
/// `foo` and `bar` attributes got translated to fields with `Option<String>` type which contain the
/// value of the attribute when one is specified. They are marked with `with` keyword which stands
/// for _with value_. The `baz` attribute, on the other hand, has `bool` type because it's an
/// attribute _without value_.
///
/// The strings between braces are embedded into error messages produced when an attribute defined
/// for one attribute group is used on another group where it is not defined. For example, if the
/// `field_attr` attribute was encountered by the generated `StructAttributes::parse` method, the
/// error message would say that it "is not allowed on structs".
///
/// # Errors
///
/// The generated parse method checks for some error conditions:
///
/// 1. Unknown attributes. When multiple attribute groups are defined in the same macro invocation,
/// one gets a different error message when providing an attribute from a different attribute group.
/// 2. Duplicate attributes.
/// 3. Missing attribute value or present attribute value when none is expected.
#[macro_export]
macro_rules! def_attrs {
    (@attr_ty str) => {::std::option::Option<::std::string::String>};
    (@attr_ty none) => {bool};
    (@match_attr str $attr_name:ident, $ident:ident, $value:expr, $span:expr, $self:ident) => {
        if let ::std::option::Option::Some(value) = $crate::macros::match_attribute_with_value(
            ::std::stringify!($attr_name),
            $ident,
            $value,
            $span
        )? {
            if $self.$attr_name.is_none() {
                $self.$attr_name = ::std::option::Option::Some(value.value());
                continue;
            } else {
                return ::std::result::Result::Err(::syn::Error::new(
                    $span,
                    concat!("duplicate `", ::std::stringify!($attr_name), "` attribute")
                ));
            }
        }
    };
    (@match_attr none $attr_name:ident, $ident:ident, $value:expr, $span:expr, $self:ident) => {
        if $crate::macros::match_attribute_without_value(
            ::std::stringify!($attr_name),
            $ident,
            $value,
            $span
        )? {
            if !$self.$attr_name {
                $self.$attr_name = true;
                continue;
            } else {
                return ::std::result::Result::Err(::syn::Error::new(
                    $span,
                    concat!("duplicate `", stringify!($attr_name), "` attribute")
                ));
            }
        }
    };
    (
        crate $list_name:ident;
        $(
            $(#[$m:meta])*
            $vis:vis $name:ident($what:literal) {
                $($attr_name:ident $kind:tt),+
            }
        );+;
    ) => {
        static ALLOWED_ATTRS: &[&'static str] = &[
            $($(::std::stringify!($attr_name),)+)+
        ];

        $(
            $(#[$m])*
            #[derive(Default, Clone, Debug)]
            $vis struct $name {
                $(pub $attr_name: $crate::def_attrs!(@attr_ty $kind)),+
            }

            impl $name {
                pub fn parse(attrs: &[::syn::Attribute]) -> ::syn::Result<Self> {
                    use ::syn::spanned::Spanned;

                    let mut parsed = $name::default();

                    for nested_meta in $crate::macros::iter_meta_lists(
                        attrs,
                        ::std::stringify!($list_name)
                    )? {
                        let (ident, value) = $crate::macros::parse_attribute(&nested_meta)?;
                        let span = nested_meta.span();

                        // This creates subsequent if blocks for simplicity. Any block that is taken
                        // either returns an error or sets the attribute field and continues.
                        $(
                            $crate::def_attrs!(@match_attr $kind $attr_name, ident, value, span, parsed);
                        )+

                        // None of the if blocks have been taken, return the appropriate error.
                        let is_valid_attr = ALLOWED_ATTRS.iter().any(|attr| ident == attr);
                        return ::std::result::Result::Err(::syn::Error::new(span, if is_valid_attr {
                            ::std::format!(
                                ::std::concat!("attribute `{}` is not allowed on ", $what),
                                ident
                            )
                        } else {
                            ::std::format!("unknown attribute `{ident}`")
                        }))
                    }

                    ::std::result::Result::Ok(parsed)
                }
            }
        )+
    }
}

/// Checks if a [`Type`]'s identifier is "Option".
pub fn ty_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => segments.last().unwrap().ident == "Option",
        _ => false,
    }
}
