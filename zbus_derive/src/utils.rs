use syn::Attribute;

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
