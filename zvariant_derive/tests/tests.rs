#![allow(dead_code)]

use zvariant::Type;
use zvariant_derive::{
    DeserializeDict, DeserializeTaggedEnum, SerializeDict, SerializeTaggedEnum, Type, TypeDict,
    TypeTaggedEnum,
};

#[test]
fn derive_unit_struct() {
    #[derive(Type)]
    struct FooF(f64);

    assert_eq!(FooF::signature(), "d")
}

#[test]
fn derive_struct() {
    #[derive(Type)]
    struct TestStruct {
        name: String,
        age: u8,
        blob: Vec<u8>,
    }

    assert_eq!(TestStruct::signature(), "(syay)")
}

#[test]
fn derive_enum() {
    #[repr(u32)]
    #[derive(Type)]
    enum RequestNameFlags {
        AllowReplacement = 0x01,
        ReplaceExisting = 0x02,
        DoNotQueue = 0x04,
    }

    assert_eq!(RequestNameFlags::signature(), "u")
}

#[test]
fn derive_tagged_enum() {
    #[derive(DeserializeTaggedEnum, SerializeTaggedEnum, TypeTaggedEnum)]
    enum Enum {
        None,
        First(String),
        Second(i32, i32),
        Third { has_foo: bool, has_bar: bool },
    }

    assert_eq!(Enum::signature(), "(sv)")
}

#[test]
fn derive_dict() {
    #[derive(SerializeDict, DeserializeDict, TypeDict)]
    #[zvariant(deny_unknown_fields)]
    struct Test {
        field_a: Option<u32>,
        #[zvariant(rename = "field-b")]
        field_b: String,
        field_c: Vec<u8>,
    }

    assert_eq!(Test::signature(), "a{sv}")
}
