use serde::{Deserialize, Serialize};
use zbus::DBusError;
use zvariant::{DeserializeDict, OwnedValue, SerializeDict, Str, Type, Value};

#[derive(Debug, Deserialize, Serialize, Type)]
pub struct ArgStructTest {
    pub foo: i32,
    pub bar: String,
}

// Mimic a NetworkManager interface property that's a dict. This tests ability to use a custom
// dict type using the `Type` And `*Dict` macros (issue #241).
#[derive(DeserializeDict, SerializeDict, Type, Debug, Value, OwnedValue, PartialEq, Eq)]
#[zvariant(signature = "dict")]
pub struct IP4Adress {
    pub prefix: u32,
    pub address: String,
}

// To test property setter for types with lifetimes.
#[derive(Serialize, Deserialize, Type, Debug, Value, OwnedValue, PartialEq, Eq)]
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
