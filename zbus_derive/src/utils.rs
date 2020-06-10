use proc_macro2::Span;
use proc_macro_crate::crate_name;
use syn::{
    Attribute, FnArg, Ident, Lit, Meta, MetaList, NestedMeta, Pat, PatIdent, PatType, Result,
};

pub fn get_crate_ident(name: &str) -> Ident {
    Ident::new(
        &match crate_name(name) {
            Ok(x) => x,
            Err(_) => name.into(),
        },
        Span::call_site(),
    )
}

pub fn arg_ident(arg: &FnArg) -> Option<&Ident> {
    match arg {
        FnArg::Typed(PatType { pat, .. }) => {
            if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                return Some(ident);
            }
            None
        }
        _ => None,
    }
}

pub fn get_doc_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
    attrs.iter().filter(|x| x.path.is_ident("doc")).collect()
}

pub fn pascal_case(s: &str) -> String {
    let mut pascal = String::new();
    let mut capitalize = true;
    for ch in s.chars() {
        if ch == '_' {
            capitalize = true;
        } else if capitalize {
            pascal.push(ch.to_ascii_uppercase());
            capitalize = false;
        } else {
            pascal.push(ch);
        }
    }
    pascal
}

#[derive(Debug, PartialEq)]
pub enum ItemAttribute {
    Property,
    Name(String),
}

impl ItemAttribute {
    pub fn is_property(&self) -> bool {
        match self {
            Self::Property => true,
            _ => false,
        }
    }
}

// find the #[@attr_name] attribute in @attrs
pub fn find_attribute_meta(attrs: &[Attribute], attr_name: &str) -> Result<Option<MetaList>> {
    let meta = match attrs.iter().find(|a| a.path.is_ident(attr_name)) {
        Some(a) => a.parse_meta(),
        _ => return Ok(None),
    };
    match meta? {
        Meta::List(n) => Ok(Some(n)),
        _ => panic!("wrong meta type"),
    }
}

// parse a single meta like: ident = "value"
fn parse_attribute(meta: &NestedMeta) -> Result<(String, String)> {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => panic!("wrong meta type"),
    };
    let meta = match meta {
        Meta::Path(p) => return Ok((p.get_ident().unwrap().to_string(), "".to_string())),
        Meta::NameValue(n) => n,
        _ => panic!("wrong meta type"),
    };
    let value = match &meta.lit {
        Lit::Str(s) => s.value(),
        _ => panic!("wrong meta type"),
    };

    let ident = match meta.path.get_ident() {
        None => panic!("missing ident"),
        Some(ident) => ident,
    };

    Ok((ident.to_string(), value))
}

fn proxy_parse_item_attribute(meta: &NestedMeta) -> Result<ItemAttribute> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "name" => Ok(ItemAttribute::Name(v)),
        "property" => Ok(ItemAttribute::Property),
        s => panic!("Unknown item meta {}", s),
    }
}

// Parse optional item attributes such as:
// #[dbus_proxy(name = "MyName", property)]
pub fn proxy_parse_item_attributes(attrs: &[Attribute]) -> Result<Vec<ItemAttribute>> {
    let meta = find_attribute_meta(attrs, "dbus_proxy")?;

    let v = match meta {
        Some(meta) => meta
            .nested
            .iter()
            .map(|m| proxy_parse_item_attribute(&m).unwrap())
            .collect(),
        None => Vec::new(),
    };

    Ok(v)
}

fn error_parse_item_attribute(meta: &NestedMeta) -> Result<ItemAttribute> {
    let (ident, v) = parse_attribute(meta)?;

    match ident.as_ref() {
        "name" => Ok(ItemAttribute::Name(v)),
        s => panic!("Unknown item meta {}", s),
    }
}

// Parse optional item attributes such as:
// #[dbus_error(name = "MyName")]
pub fn error_parse_item_attributes(attrs: &[Attribute]) -> Result<Vec<ItemAttribute>> {
    let meta = find_attribute_meta(attrs, "dbus_error")?;

    let v = match meta {
        Some(meta) => meta
            .nested
            .iter()
            .map(|m| error_parse_item_attribute(&m).unwrap())
            .collect(),
        None => Vec::new(),
    };

    Ok(v)
}
