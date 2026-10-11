//! Compile-only fixture: a downstream crate that depends only on `zbus` and uses the wire
//! derives through it, as the book examples do.
//!
//! The derives default to `::zbus::wire` internally, so no `crate` attribute is needed.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use zbus::{DeserializeDict, OwnedVariant, SerializeDict, Type, Variant};

#[derive(DeserializeDict, SerializeDict, Type)]
#[zbus(signature = "dict")]
struct Dictionary {
    field1: u16,
    #[zbus(rename = "another-name")]
    field2: i64,
    optional_field: Option<String>,
}

#[derive(Deserialize, Serialize, Type, Variant, OwnedVariant)]
struct Data {
    field1: u32,
    field2: String,
}

#[derive(Deserialize, Serialize, Type)]
#[zbus(signature = "s")]
enum StrEnum {
    Variant1,
    Variant2,
}
