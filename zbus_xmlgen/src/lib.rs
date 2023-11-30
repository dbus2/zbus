use snakecase::ascii::to_snakecase;
use std::fmt::{Display, Formatter};

use zbus::names::BusName;
use zbus_xml::{Arg, ArgDirection, Interface};
use zvariant::{
    Basic, CompleteType, ObjectPath, Signature, ARRAY_SIGNATURE_CHAR, DICT_ENTRY_SIG_END_CHAR,
    DICT_ENTRY_SIG_START_CHAR, STRUCT_SIG_END_CHAR, STRUCT_SIG_START_CHAR, VARIANT_SIGNATURE_CHAR,
};

pub struct GenTrait<'i> {
    pub interface: &'i Interface<'i>,
    pub service: Option<&'i BusName<'i>>,
    pub path: Option<&'i ObjectPath<'i>>,
}

impl<'i> Display for GenTrait<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let iface = self.interface;
        let idx = iface.name().rfind('.').unwrap() + 1;
        let name = &iface.name()[idx..];

        write!(f, "#[dbus_proxy(interface = \"{}\"", iface.name())?;
        if let Some(service) = self.service {
            write!(f, ", default_service = \"{service}\"")?;
        }
        if let Some(path) = self.path {
            write!(f, ", default_path = \"{path}\"")?;
        }
        if self.path.is_none() || self.service.is_none() {
            write!(f, ", assume_defaults = true")?;
        }
        writeln!(f, ")]")?;
        writeln!(f, "trait {name} {{")?;

        let mut methods = iface.methods().to_vec();
        methods.sort_by(|a, b| a.name().partial_cmp(&b.name()).unwrap());
        for m in &methods {
            let (inputs, output) = inputs_output_from_args(m.args());
            let name = to_identifier(&to_snakecase(m.name().as_str()));
            writeln!(f)?;
            writeln!(f, "    /// {} method", m.name())?;
            if pascal_case(&name) != m.name().as_str() {
                writeln!(f, "    #[dbus_proxy(name = \"{}\")]", m.name())?;
            }
            hide_clippy_lints(f, m)?;
            writeln!(f, "    fn {name}({inputs}){output};")?;
        }

        let mut signals = iface.signals().to_vec();
        signals.sort_by(|a, b| a.name().partial_cmp(&b.name()).unwrap());
        for signal in &signals {
            let args = parse_signal_args(signal.args());
            let name = to_identifier(&to_snakecase(signal.name().as_str()));
            writeln!(f)?;
            writeln!(f, "    /// {} signal", signal.name())?;
            if pascal_case(&name) != signal.name().as_str() {
                writeln!(f, "    #[dbus_proxy(signal, name = \"{}\")]", signal.name())?;
            } else {
                writeln!(f, "    #[dbus_proxy(signal)]")?;
            }
            writeln!(f, "    fn {name}({args}) -> zbus::Result<()>;",)?;
        }

        let mut props = iface.properties().to_vec();
        props.sort_by(|a, b| a.name().partial_cmp(&b.name()).unwrap());
        for p in props {
            let name = to_identifier(&to_snakecase(p.name().as_str()));
            let fn_attribute = if pascal_case(&name) != p.name().as_str() {
                format!("    #[dbus_proxy(property, name = \"{}\")]", p.name())
            } else {
                "    #[dbus_proxy(property)]".to_string()
            };

            writeln!(f)?;
            writeln!(f, "    /// {} property", p.name())?;
            if p.access().read() {
                writeln!(f, "{}", fn_attribute)?;
                let output = to_rust_type(p.ty(), false, false);
                hide_clippy_type_complexity_lint(f, p.ty().signature())?;
                writeln!(f, "    fn {name}(&self) -> zbus::Result<{output}>;",)?;
            }

            if p.access().write() {
                writeln!(f, "{}", fn_attribute)?;
                let input = to_rust_type(p.ty(), true, true);
                writeln!(
                    f,
                    "    fn set_{name}(&self, value: {input}) -> zbus::Result<()>;",
                )?;
            }
        }
        writeln!(f, "}}")
    }
}

fn hide_clippy_lints(fmt: &mut Formatter<'_>, method: &zbus_xml::Method<'_>) -> std::fmt::Result {
    // check for <https://rust-lang.github.io/rust-clippy/master/index.html#/too_many_arguments>
    // triggers when a functions has at least 7 paramters
    if method.args().len() >= 7 {
        writeln!(fmt, "    #[allow(clippy::too_many_arguments)]")?;
    }

    // check for <https://rust-lang.github.io/rust-clippy/master/index.html#/type_complexity>
    for arg in method.args() {
        let signature = arg.ty().signature();
        hide_clippy_type_complexity_lint(fmt, signature)?;
    }

    Ok(())
}

fn hide_clippy_type_complexity_lint(
    fmt: &mut Formatter<'_>,
    signature: &zvariant::Signature,
) -> std::fmt::Result {
    let mut it = signature.as_bytes().iter().peekable();
    let complexity = estimate_type_complexity(&mut it);
    if complexity >= 1700 {
        writeln!(fmt, "    #[allow(clippy::type_complexity)]")?;
    }
    Ok(())
}

fn inputs_output_from_args(args: &[Arg]) -> (String, String) {
    let mut inputs = vec!["&self".to_string()];
    let mut output = vec![];
    let mut n = 0;
    let mut gen_name = || {
        n += 1;
        format!("arg_{n}")
    };

    for a in args {
        match a.direction() {
            None | Some(ArgDirection::In) => {
                let ty = to_rust_type(a.ty(), true, true);
                let arg = if let Some(name) = a.name() {
                    to_identifier(name)
                } else {
                    gen_name()
                };
                inputs.push(format!("{arg}: {ty}"));
            }
            Some(ArgDirection::Out) => {
                let ty = to_rust_type(a.ty(), false, false);
                output.push(ty);
            }
        }
    }

    let output = match output.len() {
        0 => "()".to_string(),
        1 => output[0].to_string(),
        _ => format!("({})", output.join(", ")),
    };

    (inputs.join(", "), format!(" -> zbus::Result<{output}>"))
}

fn parse_signal_args(args: &[Arg]) -> String {
    let mut inputs = vec!["&self".to_string()];
    let mut n = 0;
    let mut gen_name = || {
        n += 1;
        format!("arg_{n}")
    };

    for a in args {
        let ty = to_rust_type(a.ty(), true, false);
        let arg = if let Some(name) = a.name() {
            to_identifier(name)
        } else {
            gen_name()
        };
        inputs.push(format!("{arg}: {ty}"));
    }

    inputs.join(", ")
}

fn to_rust_type(ty: &CompleteType, input: bool, as_ref: bool) -> String {
    // can't haz recursive closure, yet
    fn iter_to_rust_type(
        it: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        input: bool,
        as_ref: bool,
    ) -> String {
        let c = it.next().unwrap();
        match *c as char {
            u8::SIGNATURE_CHAR => "u8".into(),
            bool::SIGNATURE_CHAR => "bool".into(),
            i16::SIGNATURE_CHAR => "i16".into(),
            u16::SIGNATURE_CHAR => "u16".into(),
            i32::SIGNATURE_CHAR => "i32".into(),
            u32::SIGNATURE_CHAR => "u32".into(),
            i64::SIGNATURE_CHAR => "i64".into(),
            u64::SIGNATURE_CHAR => "u64".into(),
            f64::SIGNATURE_CHAR => "f64".into(),
            // xmlgen accepts 'h' on Windows, only for code generation
            'h' => (if input {
                "zbus::zvariant::Fd<'_>"
            } else {
                "zbus::zvariant::OwnedFd"
            })
            .into(),
            <&str>::SIGNATURE_CHAR => (if input || as_ref { "&str" } else { "String" }).into(),
            ObjectPath::SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::ObjectPath<'_>"
                } else {
                    "zbus::zvariant::ObjectPath<'_>"
                }
            } else {
                "zbus::zvariant::OwnedObjectPath"
            })
            .into(),
            Signature::SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::Signature<'_>"
                } else {
                    "zbus::zvariant::Signature<'_>"
                }
            } else {
                "zbus::zvariant::OwnedSignature"
            })
            .into(),
            VARIANT_SIGNATURE_CHAR => (if input {
                if as_ref {
                    "&zbus::zvariant::Value<'_>"
                } else {
                    "zbus::zvariant::Value<'_>"
                }
            } else {
                "zbus::zvariant::OwnedValue"
            })
            .into(),
            ARRAY_SIGNATURE_CHAR => {
                let c = it.peek().unwrap();
                match **c as char {
                    '{' => format!(
                        "std::collections::HashMap<{}>",
                        iter_to_rust_type(it, input, false)
                    ),
                    _ => {
                        let ty = iter_to_rust_type(it, input, false);
                        if input {
                            format!("&[{ty}]")
                        } else {
                            format!("{}Vec<{}>", if as_ref { "&" } else { "" }, ty)
                        }
                    }
                }
            }
            c @ STRUCT_SIG_START_CHAR | c @ DICT_ENTRY_SIG_START_CHAR => {
                let dict = c == '{';
                let mut vec = vec![];
                loop {
                    let c = it.peek().unwrap();
                    match **c as char {
                        STRUCT_SIG_END_CHAR | DICT_ENTRY_SIG_END_CHAR => {
                            // consume the closing character
                            it.next().unwrap();
                            break;
                        }
                        _ => vec.push(iter_to_rust_type(it, input, false)),
                    }
                }
                if dict {
                    vec.join(", ")
                } else if vec.len() > 1 {
                    format!("{}({})", if as_ref { "&" } else { "" }, vec.join(", "))
                } else {
                    format!("{}({},)", if as_ref { "&" } else { "" }, vec[0])
                }
            }
            _ => unimplemented!(),
        }
    }

    let mut it = ty.signature().as_bytes().iter().peekable();
    iter_to_rust_type(&mut it, input, as_ref)
}

static KWORDS: &[&str] = &[
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "union", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

fn to_identifier(id: &str) -> String {
    if KWORDS.contains(&id) {
        format!("{id}_")
    } else {
        id.replace('-', "_")
    }
}

// This function is the same as zbus_macros::utils::pascal_case
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

fn estimate_type_complexity(it: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>) -> u32 {
    let mut score = 0;
    let c = it.next().unwrap();
    match *c as char {
        u8::SIGNATURE_CHAR
        | bool::SIGNATURE_CHAR
        | i16::SIGNATURE_CHAR
        | u16::SIGNATURE_CHAR
        | i32::SIGNATURE_CHAR
        | u32::SIGNATURE_CHAR
        | i64::SIGNATURE_CHAR
        | u64::SIGNATURE_CHAR
        | f64::SIGNATURE_CHAR
        | <&str>::SIGNATURE_CHAR => {
            score += 1;
        }
        'h' => score += 10,
        Signature::SIGNATURE_CHAR | VARIANT_SIGNATURE_CHAR | ObjectPath::SIGNATURE_CHAR => {
            score *= 10
        }
        ARRAY_SIGNATURE_CHAR => {
            let c = it.peek().unwrap();
            match **c as char {
                '{' => {
                    score *= 10;
                    score += estimate_type_complexity(it);
                }
                _ => {
                    score += 5 * estimate_type_complexity(it);
                }
            }
        }
        STRUCT_SIG_START_CHAR | DICT_ENTRY_SIG_START_CHAR => {
            score += 50;
            loop {
                let c = it.peek().unwrap();
                match **c as char {
                    STRUCT_SIG_END_CHAR | DICT_ENTRY_SIG_END_CHAR => {
                        // consume the closing character
                        it.next().unwrap();
                        break;
                    }
                    _ => score += 5 * estimate_type_complexity(it),
                }
            }
        }
        _ => {}
    };
    score
}
