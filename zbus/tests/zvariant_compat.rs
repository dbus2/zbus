//! The pre-6.0 paths still compile through the `zbus::zvariant` compatibility module.
//!
//! Everything here is deliberately spelled the way code written against zvariant 5 spells it.
#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::{
    names::{self, InterfaceName},
    zvariant::{self, LE, ObjectPath, Signature, Type, Value, serialized::Context, to_bytes},
};

#[derive(Type, Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

// `Value` and `OwnedValue` name both a type-namespace alias (`zvariant::Value` the enum) and a
// macro-namespace derive (`zvariant::Value` the derive macro); spelling the derive path fully
// exercises that the two coexist under the same module.
#[derive(Serialize, Deserialize, Type, zvariant::Value, zvariant::OwnedValue, PartialEq, Debug)]
struct Pair {
    x: i32,
    y: String,
}

#[test]
fn value_alias_and_variants() {
    let value = zvariant::Value::from(42u8);
    assert!(matches!(value, Value::U8(42)));

    let owned = zvariant::OwnedValue::try_from(zvariant::Value::from(7u32)).unwrap();
    assert_eq!(u32::try_from(owned).unwrap(), 7);

    // The variant holding a variant is `Variant` now, but it still constructs under its old name.
    let nested = Value::Value(Box::new(Value::from(42u8)));
    assert!(matches!(nested, Value::Variant(inner) if *inner == Value::U8(42)));
}

#[test]
fn value_and_ownedvalue_derives_reached_through_the_module() {
    let pair = Pair {
        x: 42,
        y: "hello".to_string(),
    };

    let value = zvariant::Value::from(pair);
    let owned = zvariant::OwnedValue::try_from(value).unwrap();
    let back = Pair::try_from(owned).unwrap();

    assert_eq!(
        back,
        Pair {
            x: 42,
            y: "hello".to_string()
        }
    );
}

#[test]
fn as_value_module() {
    #[derive(Serialize, Deserialize, Type, PartialEq, Debug)]
    #[zvariant(signature = "dict")]
    struct Props {
        #[serde(with = "zvariant::as_value")]
        count: u32,
        #[serde(
            with = "zvariant::as_value::optional",
            skip_serializing_if = "Option::is_none",
            default
        )]
        name: Option<String>,
    }

    let props = Props {
        count: 7,
        name: Some("seven".to_string()),
    };
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &props).unwrap();
    let decoded: Props = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, props);

    let map: HashMap<String, zvariant::OwnedValue> = encoded.deserialize().unwrap().0;
    assert_eq!(u32::try_from(&map["count"]).unwrap(), 7);
}

// Code that depends on zbus alone points the derives at the compatibility module. The code they
// generate names the items under their 6.0 names, so the module has to carry those too.
#[derive(Serialize, Deserialize, Type, zvariant::Value, zvariant::OwnedValue, PartialEq, Debug)]
#[zvariant(crate = "zbus::zvariant")]
struct PairThroughTheModule {
    x: i32,
    y: String,
}

#[derive(zvariant::SerializeDict, zvariant::DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(crate = "zbus::zvariant", signature = "dict")]
struct DictThroughTheModule {
    count: u32,
    name: Option<String>,
}

#[test]
fn derives_pointed_at_the_module() {
    let pair = PairThroughTheModule {
        x: 42,
        y: "hello".to_string(),
    };
    let owned = zvariant::OwnedValue::try_from(zvariant::Value::from(pair)).unwrap();
    let back = PairThroughTheModule::try_from(owned).unwrap();
    assert_eq!(
        back,
        PairThroughTheModule {
            x: 42,
            y: "hello".to_string()
        }
    );

    let dict = DictThroughTheModule {
        count: 7,
        name: Some("seven".to_string()),
    };
    let encoded = to_bytes(Context::new_dbus(LE, 0), &dict).unwrap();
    let decoded: DictThroughTheModule = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, dict);
}

#[test]
fn serialization_functions_and_constants() {
    let ctxt = Context::new_dbus(LE, 0);
    let encoded = to_bytes(ctxt, &"hello").unwrap();
    // 4 length bytes, 5 characters and the terminating nul.
    assert_eq!(encoded.len(), 10);
    assert_eq!(*zvariant::serialized_size(ctxt, "hello").unwrap(), 10);
    assert_eq!(zvariant::ARRAY_SIGNATURE_CHAR, 'a');
}

#[test]
fn string_like_aliases() {
    let path = ObjectPath::try_from("/org/zbus/Test").unwrap();
    assert_eq!(path.as_str(), "/org/zbus/Test");

    let signature = Signature::try_from("a{sv}").unwrap();
    assert_eq!(signature.to_string(), "a{sv}");
}

#[test]
fn derives_resolve_through_the_module() {
    assert_eq!(<Point as Type>::SIGNATURE.to_string(), "(ii)");
}

#[test]
fn names_error_alias_is_the_root_error() {
    let error: names::Error = InterfaceName::try_from("not a name").unwrap_err();
    assert!(matches!(error, zbus::Error::InvalidName(_)));

    let ok: names::Result<()> = Ok(());
    assert!(ok.is_ok());
}

#[test]
fn signature_submodule() {
    zvariant::signature::validate(b"a{sv}").unwrap();

    let sig: zvariant::signature::Signature = zvariant::Signature::try_from("a{sv}").unwrap();
    assert_eq!(sig.to_string(), "a{sv}");
}

#[test]
fn root_type_aliases() {
    let ok: zvariant::Result<()> = Ok(());
    assert!(ok.is_ok());

    let err: zvariant::Result<()> = Err(zvariant::Error::IncorrectType);
    assert_eq!(err, Err(zbus::Error::IncorrectType));

    let max: zvariant::MaxDepthExceeded = zvariant::MaxDepthExceeded::Structure;
    assert_eq!(max, zbus::MaxDepthExceeded::Structure);
}
