#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::{
    OwnedVariant, Signature, Type, Variant,
    as_variant::{self, optional},
    wire::{LE, serialized::Context},
};

#[test]
fn derive_unit_struct() {
    #[derive(Type)]
    struct FooF(f64);

    assert_eq!(FooF::SIGNATURE, "d")
}

#[test]
fn derive_struct() {
    #[derive(Type)]
    struct TestStruct {
        name: String,
        age: u8,
        blob: Vec<u8>,
    }

    assert_eq!(TestStruct::SIGNATURE, "(syay)")
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

    assert_eq!(RequestNameFlags::SIGNATURE, "u")
}

#[test]
fn derive_dict() {
    #[derive(Serialize, Deserialize, Type, Default)]
    #[zbus(signature = "a{sv}")]
    #[serde(deny_unknown_fields, rename_all = "camelCase", default)]
    struct Test {
        #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
        field_a: Option<u32>,
        #[serde(with = "as_variant", rename = "field-b")]
        field_b: String,
        #[serde(with = "as_variant")]
        field_c: Vec<u8>,
    }

    let test = Test {
        field_a: Some(1),
        field_b: "foo".to_string(),
        field_c: vec![1, 2, 3],
    };

    let ctxt = Context::new(LE, 0);
    let serialized = zbus::wire::to_bytes(ctxt, &test).unwrap();
    let deserialized: HashMap<String, OwnedVariant> = serialized.deserialize().unwrap().0;

    assert_eq!(
        deserialized["fieldA"],
        Variant::from(1u32).try_into().unwrap()
    );
    assert_eq!(
        deserialized["field-b"],
        Variant::from("foo").try_into().unwrap()
    );
    assert_eq!(
        deserialized["fieldC"],
        Variant::from(&[1u8, 2, 3][..]).try_into().unwrap()
    );

    let serialized = zbus::wire::to_bytes(ctxt, &deserialized).unwrap();
    let deserialized: Test = serialized.deserialize().unwrap().0;

    assert_eq!(deserialized.field_a, Some(1u32));
    assert_eq!(deserialized.field_b.as_str(), "foo");
    assert_eq!(deserialized.field_c.as_slice(), &[1u8, 2, 3][..]);

    assert_eq!(Test::SIGNATURE, "a{sv}")
}

#[test]
#[ignore]
fn issues_311() {
    // Issue 311: Variant macro not able to handle Option in Dict.
    //
    // org.freedesktop.ModemManager1.Modem.Signal props are a dict with optional values depending on
    // the property you read.
    #[derive(Debug, Type, Deserialize, OwnedVariant, Variant, Default)]
    #[zbus(signature = "dict")]
    #[serde(deny_unknown_fields, default)]
    pub struct SignalInfo {
        #[serde(with = "optional")]
        pub rssi: Option<i32>,
        #[serde(with = "optional")]
        pub ecio: Option<i32>,
        #[serde(with = "optional")]
        pub io: Option<i32>,
        #[serde(with = "optional")]
        pub sinr: Option<i32>,
    }
}

#[test]
#[ignore]
fn issues_1252() {
    // Issue 1252: Naming a field `key` in a dict struct causes a conflict with variables created by
    // `DeserializeDict` macro, ending up with a strange error.
    #[derive(Type, Deserialize)]
    #[zbus(signature = "a{sv}")]
    pub struct OwnedProperties {
        #[serde(with = "as_variant")]
        key: String,
        #[serde(with = "as_variant")]
        val: OwnedVariant,
    }
}

#[test]
fn derive_with_crate_attr() {
    // Test that the `crate` attribute works for custom crate paths.
    // This is useful when the wire module is re-exported or renamed in Cargo.toml.
    #[derive(Type)]
    #[zbus(crate = "zbus::wire")]
    struct TestCrateAttr {
        name: String,
        value: u32,
    }

    assert_eq!(TestCrateAttr::SIGNATURE, "(su)");

    // Also test on enums
    #[repr(u8)]
    #[derive(Type)]
    #[zbus(crate = "zbus::wire")]
    enum TestCrateAttrEnum {
        A = 1,
        B = 2,
    }

    assert_eq!(TestCrateAttrEnum::SIGNATURE, "y");
}

#[test]
fn signature_macro_uses_detected_crate_path() {
    use zbus::signature;

    const DICT: Signature = signature!("a{sv}");

    assert_eq!(DICT.to_string(), "a{sv}");
    assert_eq!(signature!("(su)").to_string(), "(su)");
}
