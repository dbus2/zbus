use std::collections::HashMap;
use zvariant::{OwnedValue, Value};

use crate::{Connection, Proxy, Result};

// These implementations will be replaced by macro-generated code.

pub struct IntrospectableProxy<'a>(Proxy<'a>);

impl<'a> IntrospectableProxy<'a> {
    pub fn new_for(conn: &'a Connection, destination: &'a str, path: &'a str) -> Result<Self> {
        Ok(Self(Proxy::new(
            conn,
            destination,
            path,
            "org.freedesktop.DBus.Introspectable",
        )?))
    }

    pub fn introspect(&self) -> Result<String> {
        let reply = self.0.call("Introspect", &())?;
        Ok(reply)
    }
}

pub struct PropertiesProxy<'a>(Proxy<'a>);

impl<'a> PropertiesProxy<'a> {
    pub fn new_for(conn: &'a Connection, destination: &'a str, path: &'a str) -> Result<Self> {
        Ok(Self(Proxy::new(
            conn,
            destination,
            path,
            "org.freedesktop.DBus.Properties",
        )?))
    }

    pub fn get(&self, interface_name: &str, property_name: &str) -> Result<OwnedValue> {
        let reply = self.0.call("Get", &(interface_name, property_name))?;
        Ok(reply)
    }

    pub fn set(&self, interface_name: &str, property_name: &str, value: &Value) -> Result<()> {
        let reply = self
            .0
            .call("Set", &(interface_name, property_name, value))?;
        Ok(reply)
    }

    pub fn get_all(&self, interface_name: &str) -> Result<HashMap<String, OwnedValue>> {
        let reply = self.0.call("GetAll", &(interface_name))?;
        Ok(reply)
    }
}
