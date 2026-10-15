use std::collections::HashMap;

use zbus::{
    DeserializeDict, Dict, SerializeDict, Type, Variant,
    wire::{LE, serialized::Context, to_bytes},
};

#[test]
fn issue_1819() {
    // A peer that wraps a dict value in a variant twice (`v` -> `v` -> `a{sv}`) doesn't match a
    // typed field, and the error must name the signature we did expect.
    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}", rename_all = "kebab-case")]
    struct UpdateInfo {
        version: Option<String>,
    }

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}", rename_all = "kebab-case")]
    struct BundleInfo {
        update: Option<UpdateInfo>,
    }

    let ctxt = Context::new(LE, 0);
    let mut update: HashMap<&str, Variant<'_>> = HashMap::new();
    update.insert("version", Variant::new("1.0"));
    let mut info: HashMap<&str, Variant<'_>> = HashMap::new();
    info.insert(
        "update",
        Variant::Variant(Box::new(Variant::Dict(Dict::from(update)))),
    );
    let encoded = to_bytes(ctxt, &info).unwrap();

    let err = encoded.deserialize::<BundleInfo>().unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains(r#"string "v""#), "{msg}");
    assert!(msg.contains("a{sv}"), "{msg}");
}
