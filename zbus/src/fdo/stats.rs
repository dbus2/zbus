//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

use std::collections::HashMap;
use zbus_names::{BusName, OwnedUniqueName};
use zvariant::OwnedValue;

use super::Result;
use crate::proxy;

/// Proxy for the [`org.freedesktop.DBus.Debug.Stats`][link] interface.
///
/// [link]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-bus-debug-stats-interface
#[proxy(
    interface = "org.freedesktop.DBus.Debug.Stats",
    default_service = "org.freedesktop.DBus",
    default_path = "/org/freedesktop/DBus"
)]
pub trait Stats {
    /// Get statistics about the message bus itself.
    fn get_stats(&self) -> Result<HashMap<String, OwnedValue>>;

    /// Get statistics about a connection, identified by its unique connection name or by any
    /// well-known bus name for which it is the primary owner. This method is not meaningful for
    /// the message bus `org.freedesktop.DBus` itself.
    fn get_connection_stats(&self, name: BusName<'_>) -> Result<HashMap<String, OwnedValue>>;

    /// List all of the match rules that are active on this message bus. The keys in the result
    /// dictionary are unique connection names. The values are lists of match rules registered by
    /// that connection, in an unspecified order. If a connection has registered the same match rule
    /// more than once, it is unspecified whether duplicate entries appear in the list.
    fn get_all_match_rules(&self) -> Result<HashMap<OwnedUniqueName, Vec<crate::OwnedMatchRule>>>;
}
