//! D-Bus standard interfaces.
//!
//! Provides blocking versions of the proxy types in [`zbus::fdo`] module.

pub type IntrospectableProxy<'p> = crate::fdo::IntrospectableProxyBlocking<'p>;
pub type PropertiesProxy<'p> = crate::fdo::PropertiesProxyBlocking<'p>;
pub type ObjectManagerProxy<'p> = crate::fdo::ObjectManagerProxyBlocking<'p>;
pub type PeerProxy<'p> = crate::fdo::PeerProxyBlocking<'p>;
pub type MonitoringProxy<'p> = crate::fdo::MonitoringProxyBlocking<'p>;
pub type StatsProxy<'p> = crate::fdo::StatsProxyBlocking<'p>;
pub type DBusProxy<'p> = crate::fdo::DBusProxyBlocking<'p>;
