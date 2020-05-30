use std::collections::HashMap;
use zbus_derive::dbus_proxy;
use zvariant::{OwnedValue, Value};

use crate as zbus;

#[dbus_proxy(interface = "org.freedesktop.DBus.Introspectable", default_path = "/")]
trait Introspectable {
    /// Returns an XML description of the object, including its interfaces (with signals and
    /// methods), objects below it in the object path tree, and its properties.
    fn introspect(&self) -> zbus::Result<String>;
}

#[dbus_proxy(interface = "org.freedesktop.DBus.Properties")]
trait Properties {
    /// Get a property value.
    fn get(&self, interface_name: &str, property_name: &str) -> zbus::Result<OwnedValue>;

    /// Set a property value.
    fn set(&self, interface_name: &str, property_name: &str, value: &Value) -> zbus::Result<()>;

    /// Get all properties.
    fn get_all(&self, interface_name: &str) -> zbus::Result<HashMap<String, OwnedValue>>;
}
