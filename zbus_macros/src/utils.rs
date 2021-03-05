use proc_macro2::Span;
use proc_macro_crate::crate_name;
use syn::{
    Attribute, FnArg, Ident, Lit, Meta, MetaList, NestedMeta, Pat, PatIdent, PatType, Result,
};

pub fn get_zbus_crate_ident() -> Ident {
    Ident::new(
        crate_name("zbus")
            .as_ref()
            .map(String::as_str)
            .unwrap_or("zbus"),
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

// Convert to pascal case, assuming snake case.
// If `s` is already in pascal case, should yield the same result.
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

// Convert to snake case, assuming pascal case.
// If `s` is already in snake case, should yield the same result.
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

#[derive(Debug, PartialEq)]
pub enum ItemAttribute {
    Property,
    Signal,
    StructReturn,
    OutArgs(Vec<String>),
    Name(String),
    Object(String),
}

impl ItemAttribute {
    pub fn is_property(&self) -> bool {
        self == &Self::Property
    }

    pub fn is_signal(&self) -> bool {
        self == &Self::Signal
    }

    pub fn is_out_args(&self) -> bool {
        matches!(self, Self::OutArgs(_))
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

// parse a single meta like: ident = "value". meta can have multiple values too.
fn parse_attribute(meta: &NestedMeta) -> (String, Vec<String>) {
    let meta = match &meta {
        NestedMeta::Meta(m) => m,
        _ => panic!("wrong meta type"),
    };

    let (ident, values) = match meta {
        Meta::Path(p) => (p.get_ident().unwrap(), vec!["".to_string()]),
        Meta::NameValue(n) => {
            let value = match &n.lit {
                Lit::Str(s) => s.value(),
                _ => panic!("wrong meta type"),
            };

            let ident = match n.path.get_ident() {
                None => panic!("missing ident"),
                Some(ident) => ident,
            };

            (ident, vec![value])
        }
        Meta::List(l) => {
            let mut values = vec![];
            for nested in l.nested.iter() {
                match nested {
                    NestedMeta::Lit(lit) => match lit {
                        Lit::Str(s) => values.push(s.value()),
                        _ => panic!("wrong meta type"),
                    },
                    NestedMeta::Meta(_) => panic!("wrong meta type"),
                }
            }

            let ident = match l.path.get_ident() {
                None => panic!("missing ident"),
                Some(ident) => ident,
            };

            (ident, values)
        }
    };

    (ident.to_string(), values)
}

fn proxy_parse_item_attribute(meta: &NestedMeta) -> Result<ItemAttribute> {
    let (ident, mut values) = parse_attribute(meta);

    match ident.as_ref() {
        "name" => Ok(ItemAttribute::Name(values.remove(0))),
        "property" => Ok(ItemAttribute::Property),
        "signal" => Ok(ItemAttribute::Signal),
        "struct_return" => Ok(ItemAttribute::StructReturn),
        "out_args" => Ok(ItemAttribute::OutArgs(values)),
        "object" => Ok(ItemAttribute::Object(values.remove(0))),
        s => panic!("Unknown item meta {}", s),
    }
}

// Parse optional item attributes such as:
// #[dbus_proxy(name = "MyName", property)]
pub fn parse_item_attributes(attrs: &[Attribute], attr_name: &str) -> Result<Vec<ItemAttribute>> {
    let meta = find_attribute_meta(attrs, attr_name)?;

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
    let (ident, mut values) = parse_attribute(meta);

    match ident.as_ref() {
        "name" => Ok(ItemAttribute::Name(values.remove(0))),
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

pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

#[cfg(test)]
mod tests {
    use super::{pascal_case, snake_case};

    #[test]
    fn test_snake_to_pascal_case() {
        assert_eq!("MeaningOfLife", &pascal_case("meaning_of_life"));
    }

    #[test]
    fn test_pascal_case_on_pascal_cased_str() {
        assert_eq!("MeaningOfLife", &pascal_case("MeaningOfLife"));
    }

    #[test]
    fn test_pascal_case_to_snake_case() {
        assert_eq!("meaning_of_life", &snake_case("MeaningOfLife"));
    }

    #[test]
    fn test_snake_case_on_snake_cased_str() {
        assert_eq!("meaning_of_life", &snake_case("meaning_of_life"));
    }
}
