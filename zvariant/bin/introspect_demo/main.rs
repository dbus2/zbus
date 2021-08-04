extern crate zvariant;
extern crate zvariant_derive;

use zvariant::introspect::*;
use zvariant_derive::Introspectable;

#[derive(Introspectable)]
struct Foo {
    foo: Option<Bar>,
    bar: Option<i32>,
    baz: i32,
}

#[derive(Introspectable)]
struct Bar {
    foo: f32,
    bar: f64,
    baz: i32,
    baz2: Baz,
}

#[derive(Introspectable)]
struct Baz;

pub fn print_introspection_info(info: IntrospectionHandle, indent_level: u32) -> () {
    let indent: String = (0..indent_level).map(|_| ' ').collect();
    println!("{}{:?} {}", indent, info.primary_type(), info.name().unwrap_or(""));
    match info.primary_type() {
        PrimaryType::Enum | PrimaryType::Struct | PrimaryType::Option | PrimaryType::Array => {
            for (name, child_info) in info {
                println!("{}  {}", indent, name);
                print_introspection_info(child_info, indent_level + 4);
            }
        },
        _ => ()
    }
}

fn main() -> () {
    print_introspection_info(Foo::introspection_info(), 0);
}
