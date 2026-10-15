use serde::{Deserialize, Serialize};
use zbus::{DBusError, DeserializeDict, SerializeDict, Str, Type};

// Tests the `crate` attribute with a path to the wire module.
#[derive(Debug, Deserialize, Serialize, Type)]
#[zbus(crate = "zbus::wire")]
pub struct ArgStructTest {
    pub foo: i32,
    pub bar: String,
}

// Mimic a NetworkManager interface property that's a dict. This tests ability to use a custom
// dict type using the `Type` And `*Dict` macros (issue #241).
// Also tests the `crate` attribute with a path to the wire module.
#[derive(DeserializeDict, SerializeDict, Type, Debug, PartialEq, Eq)]
#[zbus(signature = "dict", crate = "zbus::wire")]
pub struct IP4Adress {
    pub prefix: u32,
    pub address: String,
}

// To test property setter for types with lifetimes.
// Also tests the `crate` attribute with a path to the wire module.
#[derive(Serialize, Deserialize, Type, Debug, PartialEq, Eq)]
#[zbus(crate = "zbus::wire")]
pub struct RefType<'a> {
    #[serde(borrow)]
    pub field1: Str<'a>,
}

#[derive(Debug, Clone)]
pub enum NextAction {
    Quit,
    CreateObj(String),
    DestroyObj(String),
}

/// Custom D-Bus error type.
#[derive(Debug, DBusError, PartialEq)]
#[zbus(prefix = "org.freedesktop.MyIface.Error")]
pub enum MyIfaceError {
    SomethingWentWrong(String),
    #[zbus(error)]
    ZBus(zbus::Error),
}
