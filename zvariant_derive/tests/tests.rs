#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zvariant::{
    serialized::{Context, Format},
    DeserializeDict, OwnedValue, SerializeDict, Type, Value, LE,
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

fn derive_dict_struct() {
    #[derive(Deserialize, Serialize, Type)]
    #[serde(rename_all = "PascalCase")]
    #[zvariant(signature = "a{sv}")]
    struct HostAddress {
        host_name: String,
        port: u16,
    }

    let address = HostAddress {
        host_name: "example.org".into(),
        port: 0x2342,
    };

    let ctxt = Context::new(Format::DBus, LE, 0);
    let serialized = zvariant::to_bytes(ctxt, &address).unwrap();
    let deserialized: HashMap<String, OwnedValue> = serialized.deserialize().unwrap().0;

    assert_eq!(
        b"\x2e\0\0\0\0\0\0\0\x08\0\0\0HostName\0\x01s\0\x0b\0\0\0example.org\0\x04\0\0\0Port\0\x01q\0\x42\x23",
        serialized,
    );
    assert_eq!(
        deserialized["HostName"],
        Value::from("foobar").try_into().unwrap()
    );
    assert_eq!(
        deserialized["Port"],
        Value::from(0x2342u16).try_into().unwrap()
    );
}

#[test]
fn derive_dict() {
    #[derive(SerializeDict, DeserializeDict, Type)]
    #[zvariant(deny_unknown_fields, signature = "a{sv}", rename_all = "camelCase")]
    struct Test {
        field_a: Option<u32>,
        #[zvariant(rename = "field-b")]
        field_b: String,
        field_c: Vec<u8>,
    }

    let test = Test {
        field_a: Some(1),
        field_b: "foo".to_string(),
        field_c: vec![1, 2, 3],
    };

    let ctxt = Context::new(Format::DBus, LE, 0);
    let serialized = zvariant::to_bytes(ctxt, &test).unwrap();
    let deserialized: HashMap<String, OwnedValue> = serialized.deserialize().unwrap().0;

    assert_eq!(
        deserialized["fieldA"],
        Value::from(1u32).try_into().unwrap()
    );
    assert_eq!(
        deserialized["field-b"],
        Value::from("foo").try_into().unwrap()
    );
    assert_eq!(
        deserialized["fieldC"],
        Value::from(&[1u8, 2, 3][..]).try_into().unwrap()
    );

    let serialized = zvariant::to_bytes(ctxt, &deserialized).unwrap();
    let deserialized: Test = serialized.deserialize().unwrap().0;

    assert_eq!(deserialized.field_a, Some(1u32));
    assert_eq!(deserialized.field_b.as_str(), "foo");
    assert_eq!(deserialized.field_c.as_slice(), &[1u8, 2, 3][..]);

    assert_eq!(Test::signature(), "a{sv}")
}
