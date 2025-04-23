//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

use std::collections::HashMap;
use zbus_names::BusName;
use zvariant::OwnedValue;

use super::Result;
use crate::proxy;

/// Proxy for the `org.freedesktop.DBus.Debug.Stats` interface.
#[proxy(
    interface = "org.freedesktop.DBus.Debug.Stats",
    default_service = "org.freedesktop.DBus",
    default_path = "/org/freedesktop/DBus"
)]
pub trait Stats {
    /// GetStats (undocumented - tested with dbus-broker-36-4)
    fn get_stats(&self) -> Result<HashMap<String, OwnedValue>>;

    /// GetConnectionStats (undocumented - unimplemented with dbus-broker-36-4)
    fn get_connection_stats(&self, name: BusName<'_>) -> Result<Vec<HashMap<String, OwnedValue>>>;

    /// GetAllMatchRules (undocumented - unimplemented with dbus-broker-36-4)
    fn get_all_match_rules(
        &self,
    ) -> Result<Vec<HashMap<crate::names::OwnedUniqueName, Vec<crate::OwnedMatchRule>>>>;
}
