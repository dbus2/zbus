//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

use static_assertions::assert_impl_all;
use std::{borrow::Cow, collections::HashMap};
use zbus_names::InterfaceName;
use zvariant::{OwnedValue, Value};

use super::{Error, Result};
use crate::{interface, message::Header, object_server::SignalEmitter, Connection, ObjectServer};

/// Service-side implementation for the `org.freedesktop.DBus.Properties` interface.
/// This interface is implemented automatically for any object registered to the
/// [ObjectServer].
pub struct Properties;

assert_impl_all!(Properties: Send, Sync, Unpin);

#[interface(
    name = "org.freedesktop.DBus.Properties",
    introspection_docs = false,
    proxy(visibility = "pub")
)]
impl Properties {
    /// Get a property value.
    async fn get(
        &self,
        interface_name: InterfaceName<'_>,
        property_name: &str,
        #[zbus(connection)] conn: &Connection,
        #[zbus(object_server)] server: &ObjectServer,
        #[zbus(header)] header: Header<'_>,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> Result<OwnedValue> {
        let path = header.path().ok_or(crate::Error::MissingField)?;
        let root = server.root().read().await;
        let iface = root
            .get_child(path)
            .and_then(|node| node.interface_lock(interface_name.as_ref()))
            .ok_or_else(|| {
                Error::UnknownInterface(format!("Unknown interface '{interface_name}'"))
            })?;

        let res = iface
            .instance
            .read()
            .await
            .get(property_name, server, conn, Some(&header), &emitter)
            .await;
        res.unwrap_or_else(|| {
            Err(Error::UnknownProperty(format!(
                "Unknown property '{property_name}'"
            )))
        })
    }

    /// Set a property value.
    #[allow(clippy::too_many_arguments)]
    async fn set(
        &self,
        interface_name: InterfaceName<'_>,
        property_name: &str,
        value: Value<'_>,
        #[zbus(object_server)] server: &ObjectServer,
        #[zbus(connection)] connection: &Connection,
        #[zbus(header)] header: Header<'_>,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> Result<()> {
        let path = header.path().ok_or(crate::Error::MissingField)?;
        let root = server.root().read().await;
        let iface = root
            .get_child(path)
            .and_then(|node| node.interface_lock(interface_name.as_ref()))
            .ok_or_else(|| {
                Error::UnknownInterface(format!("Unknown interface '{interface_name}'"))
            })?;

        match iface.instance.read().await.set(
            property_name,
            &value,
            server,
            connection,
            Some(&header),
            &emitter,
        ) {
            zbus::object_server::DispatchResult::RequiresMut => {}
            zbus::object_server::DispatchResult::NotFound => {
                return Err(Error::UnknownProperty(format!(
                    "Unknown property '{property_name}'"
                )));
            }
            zbus::object_server::DispatchResult::Async(f) => {
                return f.await.map_err(Into::into);
            }
        }
        let res = iface
            .instance
            .write()
            .await
            .set_mut(
                property_name,
                &value,
                server,
                connection,
                Some(&header),
                &emitter,
            )
            .await;
        res.unwrap_or_else(|| {
            Err(Error::UnknownProperty(format!(
                "Unknown property '{property_name}'"
            )))
        })
    }

    /// Get all properties.
    async fn get_all(
        &self,
        interface_name: InterfaceName<'_>,
        #[zbus(object_server)] server: &ObjectServer,
        #[zbus(connection)] connection: &Connection,
        #[zbus(header)] header: Header<'_>,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> Result<HashMap<String, OwnedValue>> {
        let path = header.path().ok_or(crate::Error::MissingField)?;
        let root = server.root().read().await;
        let iface = root
            .get_child(path)
            .and_then(|node| node.interface_lock(interface_name.as_ref()))
            .ok_or_else(|| {
                Error::UnknownInterface(format!("Unknown interface '{interface_name}'"))
            })?;

        let res = iface
            .instance
            .read()
            .await
            .get_all(server, connection, Some(&header), &emitter)
            .await?;
        Ok(res)
    }

    /// Emit the `org.freedesktop.DBus.Properties.PropertiesChanged` signal.
    #[zbus(signal)]
    #[rustfmt::skip]
    pub async fn properties_changed(
        emitter: &SignalEmitter<'_>,
        interface_name: InterfaceName<'_>,
        changed_properties: HashMap<&str, Value<'_>>,
        invalidated_properties: Cow<'_, [&str]>,
    ) -> zbus::Result<()>;
}

assert_impl_all!(PropertiesProxy<'_>: Send, Sync, Unpin);
#[cfg(feature = "blocking-api")]
assert_impl_all!(PropertiesProxyBlocking<'_>: Send, Sync, Unpin);
