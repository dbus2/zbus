//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

use async_io::block_on;
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use static_assertions::assert_impl_all;
use std::collections::HashMap;
use std::sync::Arc;
use zbus_names::{
    BusName, InterfaceName, OwnedBusName, OwnedInterfaceName, OwnedUniqueName, UniqueName,
    WellKnownName,
};
use zvariant::{derive::Type, ObjectPath, Optional, OwnedObjectPath, OwnedValue, Value};

use crate::{
    dbus_interface, dbus_proxy, DBusError, DispatchResult, MessageHeader, ObjectServer,
    SignalContext,
};

/// Proxy for the `org.freedesktop.DBus.Introspectable` interface.
#[dbus_proxy(interface = "org.freedesktop.DBus.Introspectable", default_path = "/")]
trait Introspectable {
    /// Returns an XML description of the object, including its interfaces (with signals and
    /// methods), objects below it in the object path tree, and its properties.
    fn introspect(&self) -> Result<String>;
}

assert_impl_all!(AsyncIntrospectableProxy<'_>: Send, Sync, Unpin);
assert_impl_all!(IntrospectableProxy<'_>: Send, Sync, Unpin);

/// Server-side implementation for the `org.freedesktop.DBus.Introspectable` interface.
/// This interface is implemented automatically for any object registered to the
/// [ObjectServer](crate::ObjectServer).
pub(crate) struct Introspectable;

#[dbus_interface(name = "org.freedesktop.DBus.Introspectable")]
impl Introspectable {
    async fn introspect(
        &self,
        #[zbus(object_server)] server: &ObjectServer,
        #[zbus(header)] header: MessageHeader<'_>,
    ) -> Result<String> {
        let path = header.path()?.ok_or(crate::Error::MissingField)?;
        let node = server
            .get_node(path)
            .ok_or_else(|| Error::UnknownObject(format!("Unknown object '{}'", path)))?;

        Ok(node.introspect().await)
    }
}

/// Proxy for the `org.freedesktop.DBus.Properties` interface.
#[dbus_proxy(interface = "org.freedesktop.DBus.Properties")]
trait Properties {
    /// Get a property value.
    async fn get(
        &self,
        interface_name: InterfaceName<'_>,
        property_name: &str,
    ) -> Result<OwnedValue>;

    /// Set a property value.
    async fn set(
        &self,
        interface_name: InterfaceName<'_>,
        property_name: &str,
        value: &Value<'_>,
    ) -> Result<()>;

    /// Get all properties.
    async fn get_all(
        &self,
        interface_name: InterfaceName<'_>,
    ) -> Result<HashMap<String, OwnedValue>>;

    #[dbus_proxy(signal)]
    async fn properties_changed(
        &self,
        interface_name: InterfaceName<'_>,
        changed_properties: HashMap<&str, Value<'_>>,
        invalidated_properties: Vec<&str>,
    ) -> Result<()>;
}

assert_impl_all!(AsyncPropertiesProxy<'_>: Send, Sync, Unpin);
assert_impl_all!(PropertiesProxy<'_>: Send, Sync, Unpin);

/// Server-side implementation for the `org.freedesktop.DBus.Properties` interface.
/// This interface is implemented automatically for any object registered to the
/// [ObjectServer](crate::ObjectServer).
pub struct Properties;

assert_impl_all!(Properties: Send, Sync, Unpin);

#[dbus_interface(name = "org.freedesktop.DBus.Properties")]
impl Properties {
    #[dbus_interface(raw_return("v"))]
    fn get<'c>(
        &self,
        interface_name: InterfaceName<'_>,
        property_name: String,
        #[zbus(object_server)] server: &'c ObjectServer,
        #[zbus(connection)] conn: &'c zbus::Connection,
        #[zbus(message)] msg: &'c Arc<zbus::Message>,
        #[zbus(allow_blocking)] allow_blocking: bool,
    ) -> DispatchResult<'c> {
        (move || -> Result<DispatchResult<'c>> {
            let header = msg.header()?;
            let path = header.path()?.ok_or(crate::Error::MissingField)?;
            let iface = server
                .get_node(path)
                .and_then(|node| node.interface_lock(interface_name.clone()))
                .ok_or_else(|| {
                    Error::UnknownInterface(format!("Unknown interface '{}'", interface_name))
                })?;

            if allow_blocking {
                Ok(DispatchResult::Blocking(Box::new(
                    move || match block_on(iface.read()).get(
                        server,
                        conn,
                        msg,
                        &property_name,
                        allow_blocking,
                    ) {
                        DispatchResult::Async(f) => block_on(f),
                        DispatchResult::Blocking(f) => f(),
                        _ => block_on(conn.reply_dbus_error(
                            &header,
                            Error::UnknownProperty(format!("Unknown property '{}'", property_name)),
                        )),
                    },
                )))
            } else {
                Ok(DispatchResult::Async(Box::pin(async move {
                    match iface
                        .read()
                        .await
                        .get(server, conn, msg, &property_name, allow_blocking)
                    {
                        DispatchResult::Async(f) => f.await,
                        _ => {
                            conn.reply_dbus_error(
                                &header,
                                Error::UnknownProperty(format!(
                                    "Unknown property '{}'",
                                    property_name
                                )),
                            )
                            .await
                        }
                    }
                })))
            }
        })()
        .unwrap_or_else(move |e| {
            DispatchResult::Async(Box::pin(async move {
                let header = msg.header().unwrap();
                conn.reply_dbus_error(&header, e).await
            }))
        })
    }

    #[dbus_interface(raw_return)]
    #[allow(clippy::too_many_arguments)]
    fn set<'c>(
        &self,
        interface_name: InterfaceName<'_>,
        property_name: String,
        value: OwnedValue,
        #[zbus(object_server)] server: &'c ObjectServer,
        #[zbus(connection)] conn: &'c zbus::Connection,
        #[zbus(message)] msg: &'c Arc<zbus::Message>,
        #[zbus(allow_blocking)] allow_blocking: bool,
        #[zbus(header)] header: MessageHeader<'c>,
        #[zbus(signal_context)] ctxt: SignalContext<'c>,
    ) -> DispatchResult<'c> {
        (move || -> Result<DispatchResult<'c>> {
            let path = header.path()?.ok_or(crate::Error::MissingField)?;
            let iface = server
                .get_node(path)
                .and_then(|node| node.interface_lock(interface_name.clone()))
                .ok_or_else(|| {
                    Error::UnknownInterface(format!("Unknown interface '{}'", interface_name))
                })?;

            if allow_blocking {
                Ok(DispatchResult::Blocking(Box::new(
                    move || match block_on(iface.write()).set(
                        server,
                        conn,
                        msg,
                        &property_name,
                        &value,
                        &ctxt,
                        allow_blocking,
                    ) {
                        DispatchResult::Async(f) => block_on(f),
                        DispatchResult::Blocking(f) => f(),
                        _ => block_on(conn.reply_dbus_error(
                            &header,
                            Error::UnknownProperty(format!("Unknown property '{}'", property_name)),
                        )),
                    },
                )))
            } else {
                Ok(DispatchResult::Async(Box::pin(async move {
                    match iface.write().await.set(
                        server,
                        conn,
                        msg,
                        &property_name,
                        &value,
                        &ctxt,
                        allow_blocking,
                    ) {
                        DispatchResult::Async(f) => f.await,
                        _ => {
                            conn.reply_dbus_error(
                                &header,
                                Error::UnknownProperty(format!(
                                    "Unknown property '{}'",
                                    property_name
                                )),
                            )
                            .await
                        }
                    }
                })))
            }
        })()
        .unwrap_or_else(move |e| {
            DispatchResult::Async(Box::pin(async move {
                let header = msg.header().unwrap();
                conn.reply_dbus_error(&header, e).await
            }))
        })
    }

    #[dbus_interface(raw_return("a{sv}"))]
    fn get_all<'c>(
        &self,
        interface_name: InterfaceName<'_>,
        #[zbus(object_server)] server: &'c ObjectServer,
        #[zbus(connection)] conn: &'c zbus::Connection,
        #[zbus(message)] msg: &'c Arc<zbus::Message>,
        #[zbus(allow_blocking)] allow_blocking: bool,
        #[zbus(header)] header: MessageHeader<'c>,
    ) -> DispatchResult<'c> {
        (move || -> Result<DispatchResult<'c>> {
            let path = header.path()?.ok_or(crate::Error::MissingField)?;
            let iface = server
                .get_node(path)
                .and_then(|node| node.interface_lock(interface_name.clone()))
                .ok_or_else(|| {
                    Error::UnknownInterface(format!("Unknown interface '{}'", interface_name))
                })?;

            if allow_blocking {
                Ok(DispatchResult::Blocking(Box::new(
                    move || match block_on(iface.read()).get_all(server, conn, msg, allow_blocking)
                    {
                        DispatchResult::Async(f) => block_on(f),
                        DispatchResult::Blocking(f) => f(),
                        _ => Err(zbus::Error::Unsupported),
                    },
                )))
            } else {
                Ok(DispatchResult::Async(Box::pin(async move {
                    match iface
                        .read()
                        .await
                        .get_all(server, conn, msg, allow_blocking)
                    {
                        DispatchResult::Async(f) => f.await,
                        _ => Err(zbus::Error::Unsupported),
                    }
                })))
            }
        })()
        .unwrap_or_else(move |e| {
            DispatchResult::Async(Box::pin(async move {
                let header = msg.header().unwrap();
                conn.reply_dbus_error(&header, e).await
            }))
        })
    }

    /// Emits the `org.freedesktop.DBus.Properties.PropertiesChanged` signal.
    #[dbus_interface(signal)]
    #[rustfmt::skip]
    pub async fn properties_changed(
        ctxt: &SignalContext<'_>,
        interface_name: InterfaceName<'_>,
        changed_properties: &HashMap<&str, &Value<'_>>,
        invalidated_properties: &[&str],
    ) -> zbus::Result<()>;
}

/// The type returned by the [`ObjectManagerProxy::get_managed_objects`] and
/// [`AsyncObjectManagerProxy::get_managed_objects`] methods.
pub type ManagedObjects =
    HashMap<OwnedObjectPath, HashMap<OwnedInterfaceName, HashMap<String, OwnedValue>>>;

/// Proxy for the `org.freedesktop.DBus.ObjectManager` interface.
///
/// **NB:** Changes to properties on existing interfaces are not reported using this interface.
/// Please use [`PropertiesProxy::connect_properties_changed`] to monitor changes to properties on
/// objects.
#[dbus_proxy(interface = "org.freedesktop.DBus.ObjectManager")]
trait ObjectManager {
    /// The return value of this method is a dict whose keys are object paths. All returned object
    /// paths are children of the object path implementing this interface, i.e. their object paths
    /// start with the ObjectManager's object path plus '/'.
    ///
    /// Each value is a dict whose keys are interfaces names. Each value in this inner dict is the
    /// same dict that would be returned by the org.freedesktop.DBus.Properties.GetAll() method for
    /// that combination of object path and interface. If an interface has no properties, the empty
    /// dict is returned.
    fn get_managed_objects(&self) -> Result<ManagedObjects>;

    /// This signal is emitted when either a new object is added or when an existing object gains
    /// one or more interfaces. The `interfaces_and_properties` argument contains a map with the
    /// interfaces and properties (if any) that have been added to the given object path.
    #[dbus_proxy(signal)]
    fn interfaces_added(
        &self,
        object_path: ObjectPath<'_>,
        interfaces_and_properties: HashMap<&str, HashMap<&str, Value<'_>>>,
    ) -> Result<()>;

    /// This signal is emitted whenever an object is removed or it loses one or more interfaces.
    /// The `interfaces` parameters contains a list of the interfaces that were removed.
    #[dbus_proxy(signal)]
    fn interfaces_removed(&self, object_path: ObjectPath<'_>, interfaces: Vec<&str>) -> Result<()>;
}

assert_impl_all!(AsyncObjectManagerProxy<'_>: Send, Sync, Unpin);
assert_impl_all!(ObjectManagerProxy<'_>: Send, Sync, Unpin);

/// Proxy for the `org.freedesktop.DBus.Peer` interface.
#[dbus_proxy(interface = "org.freedesktop.DBus.Peer")]
trait Peer {
    /// On receipt, an application should do nothing other than reply as usual. It does not matter
    /// which object path a ping is sent to.
    fn ping(&self) -> Result<()>;

    /// An application should reply the containing a hex-encoded UUID representing the identity of
    /// the machine the process is running on. This UUID must be the same for all processes on a
    /// single system at least until that system next reboots. It should be the same across reboots
    /// if possible, but this is not always possible to implement and is not guaranteed. It does not
    /// matter which object path a GetMachineId is sent to.
    fn get_machine_id(&self) -> Result<String>;
}

assert_impl_all!(AsyncPeerProxy<'_>: Send, Sync, Unpin);
assert_impl_all!(PeerProxy<'_>: Send, Sync, Unpin);

pub(crate) struct Peer;

/// Server-side implementation for the `org.freedesktop.DBus.Peer` interface.
/// This interface is implemented automatically for any object registered to the
/// [ObjectServer](crate::ObjectServer).
#[dbus_interface(name = "org.freedesktop.DBus.Peer")]
impl Peer {
    fn ping(&self) {}

    fn get_machine_id(&self) -> Result<String> {
        let mut id = match std::fs::read_to_string("/var/lib/dbus/machine-id") {
            Ok(id) => id,
            Err(e) => {
                if let Ok(id) = std::fs::read_to_string("/etc/machine-id") {
                    id
                } else {
                    return Err(Error::IOError(format!(
                        "Failed to read from /var/lib/dbus/machine-id or /etc/machine-id: {}",
                        e
                    )));
                }
            }
        };

        let len = id.trim_end().len();
        id.truncate(len);
        Ok(id)
    }
}

/// Proxy for the `org.freedesktop.DBus.Monitoring` interface.
#[dbus_proxy(interface = "org.freedesktop.DBus.Monitoring")]
trait Monitoring {
    /// Converts the connection into a monitor connection which can be used as a
    /// debugging/monitoring tool.
    fn become_monitor(&self, n1: &[&str], n2: u32) -> Result<()>;
}

assert_impl_all!(AsyncMonitoringProxy<'_>: Send, Sync, Unpin);
assert_impl_all!(MonitoringProxy<'_>: Send, Sync, Unpin);

/// Proxy for the `org.freedesktop.DBus.Stats` interface.
#[dbus_proxy(interface = "org.freedesktop.DBus.Debug.Stats")]
trait Stats {
    /// GetStats (undocumented)
    fn get_stats(&self) -> Result<Vec<HashMap<String, OwnedValue>>>;

    /// GetConnectionStats (undocumented)
    fn get_connection_stats(&self, n1: &str) -> Result<Vec<HashMap<String, OwnedValue>>>;

    /// GetAllMatchRules (undocumented)
    fn get_all_match_rules(&self) -> Result<Vec<HashMap<String, Vec<String>>>>;
}

assert_impl_all!(AsyncStatsProxy<'_>: Send, Sync, Unpin);
assert_impl_all!(StatsProxy<'_>: Send, Sync, Unpin);

/// The flags used by the bus [`request_name`] method.
///
/// [`request_name`]: struct.DBusProxy.html#method.request_name
#[repr(u32)]
#[derive(Type, BitFlags, Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum RequestNameFlags {
    /// If an application A specifies this flag and succeeds in becoming the owner of the name, and
    /// another application B later calls [`request_name`] with the [`ReplaceExisting`] flag, then
    /// application A will lose ownership and receive a `org.freedesktop.DBus.NameLost` signal, and
    /// application B will become the new owner. If [`AllowReplacement`] is not specified by
    /// application A, or [`ReplaceExisting`] is not specified by application B, then application B
    /// will not replace application A as the owner.
    ///
    /// [`ReplaceExisting`]: enum.RequestNameFlags.html#variant.ReplaceExisting
    /// [`AllowReplacement`]: enum.RequestNameFlags.html#variant.AllowReplacement
    /// [`request_name`]: struct.DBusProxy.html#method.request_name
    AllowReplacement = 0x01,
    /// Try to replace the current owner if there is one. If this flag is not set the application
    /// will only become the owner of the name if there is no current owner. If this flag is set,
    /// the application will replace the current owner if the current owner specified
    /// [`AllowReplacement`].
    ///
    /// [`AllowReplacement`]: enum.RequestNameFlags.html#variant.AllowReplacement
    ReplaceExisting = 0x02,
    ///  Without this flag, if an application requests a name that is already owned, the application
    ///  will be placed in a queue to own the name when the current owner gives it up. If this flag
    ///  is given, the application will not be placed in the queue, the request for the name will
    ///  simply fail. This flag also affects behavior when an application is replaced as name owner;
    ///  by default the application moves back into the waiting queue, unless this flag was provided
    ///  when the application became the name owner.
    DoNotQueue = 0x04,
}

assert_impl_all!(RequestNameFlags: Send, Sync, Unpin);

/// The return code of the [`request_name`] method.
///
/// [`request_name`]: struct.DBusProxy.html#method.request_name
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
pub enum RequestNameReply {
    /// The caller is now the primary owner of the name, replacing any previous owner. Either the
    /// name had no owner before, or the caller specified [`ReplaceExisting`] and the current owner
    /// specified [`AllowReplacement`].
    ///
    /// [`ReplaceExisting`]: enum.RequestNameFlags.html#variant.ReplaceExisting
    /// [`AllowReplacement`]: enum.RequestNameFlags.html#variant.AllowReplacement
    PrimaryOwner = 0x01,
    /// The name already had an owner, [`DoNotQueue`] was not specified, and either the current
    /// owner did not specify [`AllowReplacement`] or the requesting application did not specify
    /// [`ReplaceExisting`].
    ///
    /// [`DoNotQueue`]: enum.RequestNameFlags.html#variant.DoNotQueue
    /// [`ReplaceExisting`]: enum.RequestNameFlags.html#variant.ReplaceExisting
    /// [`AllowReplacement`]: enum.RequestNameFlags.html#variant.AllowReplacement
    InQueue = 0x02,
    /// The name already has an owner, [`DoNotQueue`] was specified, and either [`AllowReplacement`]
    /// was not specified by the current owner, or [`ReplaceExisting`] was not specified by the
    /// requesting application.
    ///
    /// [`DoNotQueue`]: enum.RequestNameFlags.html#variant.DoNotQueue
    /// [`ReplaceExisting`]: enum.RequestNameFlags.html#variant.ReplaceExisting
    /// [`AllowReplacement`]: enum.RequestNameFlags.html#variant.AllowReplacement
    Exists = 0x03,
    /// The application trying to request ownership of a name is already the owner of it.
    AlreadyOwner = 0x04,
}

assert_impl_all!(RequestNameReply: Send, Sync, Unpin);

/// The return code of the [`release_name`] method.
///
/// [`release_name`]: struct.DBusProxy.html#method.release_name
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
pub enum ReleaseNameReply {
    /// The caller has released their claim on the given name. Either the caller was the primary
    /// owner of the name, and the name is now unused or taken by somebody waiting in the queue for
    /// the name, or the caller was waiting in the queue for the name and has now been removed from
    /// the queue.
    Released = 0x01,
    /// The given name does not exist on this bus.
    NonExistent = 0x02,
    /// The caller was not the primary owner of this name, and was also not waiting in the queue to
    /// own this name.
    NotOwner = 0x03,
}

assert_impl_all!(ReleaseNameReply: Send, Sync, Unpin);

/// Proxy for the `org.freedesktop.DBus` interface.
#[dbus_proxy(interface = "org.freedesktop.DBus")]
trait DBus {
    /// Adds a match rule to match messages going through the message bus
    fn add_match(&self, rule: &str) -> Result<()>;

    /// Returns auditing data used by Solaris ADT, in an unspecified binary format.
    fn get_adt_audit_session_data(&self, bus_name: BusName<'_>) -> Result<Vec<u8>>;

    /// Returns as many credentials as possible for the process connected to the server.
    fn get_connection_credentials(
        &self,
        bus_name: BusName<'_>,
    ) -> Result<HashMap<String, OwnedValue>>;

    /// Returns the security context used by SELinux, in an unspecified format.
    #[dbus_proxy(name = "GetConnectionSELinuxSecurityContext")]
    fn get_connection_selinux_security_context(&self, bus_name: BusName<'_>) -> Result<Vec<u8>>;

    /// Returns the Unix process ID of the process connected to the server.
    #[dbus_proxy(name = "GetConnectionUnixProcessID")]
    fn get_connection_unix_process_id(&self, bus_name: BusName<'_>) -> Result<u32>;

    /// Returns the Unix user ID of the process connected to the server.
    fn get_connection_unix_user(&self, bus_name: BusName<'_>) -> Result<u32>;

    /// Gets the unique ID of the bus.
    fn get_id(&self) -> Result<String>;

    /// Returns the unique connection name of the primary owner of the name given.
    fn get_name_owner(&self, name: BusName<'_>) -> Result<OwnedUniqueName>;

    /// Returns the unique name assigned to the connection.
    fn hello(&self) -> Result<OwnedUniqueName>;

    /// Returns a list of all names that can be activated on the bus.
    fn list_activatable_names(&self) -> Result<Vec<OwnedBusName>>;

    /// Returns a list of all currently-owned names on the bus.
    fn list_names(&self) -> Result<Vec<OwnedBusName>>;

    /// List the connections currently queued for a bus name.
    fn list_queued_owners(&self, name: WellKnownName<'_>) -> Result<Vec<OwnedUniqueName>>;

    /// Checks if the specified name exists (currently has an owner).
    fn name_has_owner(&self, name: BusName<'_>) -> Result<bool>;

    /// Ask the message bus to release the method caller's claim to the given name.
    fn release_name(&self, name: WellKnownName<'_>) -> Result<ReleaseNameReply>;

    /// Reload server configuration.
    fn reload_config(&self) -> Result<()>;

    /// Removes the first rule that matches.
    fn remove_match(&self, rule: &str) -> Result<()>;

    /// Ask the message bus to assign the given name to the method caller.
    fn request_name(
        &self,
        name: WellKnownName<'_>,
        flags: BitFlags<RequestNameFlags>,
    ) -> Result<RequestNameReply>;

    /// Tries to launch the executable associated with a name (service
    /// activation), as an explicit request.
    fn start_service_by_name(&self, name: WellKnownName<'_>, flags: u32) -> Result<u32>;

    /// This method adds to or modifies that environment when activating services.
    fn update_activation_environment(&self, environment: HashMap<&str, &str>) -> Result<()>;

    /// This signal indicates that the owner of a name has
    /// changed. It's also the signal to use to detect the appearance
    /// of new names on the bus.
    #[dbus_proxy(signal)]
    fn name_owner_changed(
        &self,
        name: BusName<'_>,
        old_owner: Optional<UniqueName<'_>>,
        new_owner: Optional<UniqueName<'_>>,
    );

    /// This signal is sent to a specific application when it loses ownership of a name.
    #[dbus_proxy(signal)]
    fn name_lost(&self, name: BusName<'_>);

    /// This signal is sent to a specific application when it gains ownership of a name.
    #[dbus_proxy(signal)]
    fn name_acquired(&self, name: BusName<'_>);

    /// This property lists abstract “features” provided by the message bus, and can be used by
    /// clients to detect the capabilities of the message bus with which they are communicating.
    #[dbus_proxy(property)]
    fn features(&self) -> Result<Vec<String>>;

    /// This property lists interfaces provided by the `/org/freedesktop/DBus` object, and can be
    /// used by clients to detect the capabilities of the message bus with which they are
    /// communicating. Unlike the standard Introspectable interface, querying this property does not
    /// require parsing XML. This property was added in version 1.11.x of the reference
    /// implementation of the message bus.
    ///
    /// The standard `org.freedesktop.DBus` and `org.freedesktop.DBus.Properties` interfaces are not
    /// included in the value of this property, because their presence can be inferred from the fact
    /// that a method call on `org.freedesktop.DBus.Properties` asking for properties of
    /// `org.freedesktop.DBus` was successful. The standard `org.freedesktop.DBus.Peer` and
    /// `org.freedesktop.DBus.Introspectable` interfaces are not included in the value of this
    /// property either, because they do not indicate features of the message bus implementation.
    #[dbus_proxy(property)]
    fn interfaces(&self) -> Result<Vec<OwnedInterfaceName>>;
}

assert_impl_all!(AsyncDBusProxy<'_>: Send, Sync, Unpin);
assert_impl_all!(DBusProxy<'_>: Send, Sync, Unpin);

/// Errors from <https://gitlab.freedesktop.org/dbus/dbus/-/blob/master/dbus/dbus-protocol.h>
#[derive(Debug, DBusError, PartialEq)]
#[dbus_error(prefix = "org.freedesktop.DBus.Error")]
#[allow(clippy::upper_case_acronyms)]
pub enum Error {
    /// Unknown or fall-through ZBus error.
    ZBus(zbus::Error),

    /// A generic error; "something went wrong" - see the error message for more.
    Failed(String),

    /// There was not enough memory to complete an operation.
    NoMemory(String),

    /// The bus doesn't know how to launch a service to supply the bus name you wanted.
    ServiceUnknown(String),

    /// The bus name you referenced doesn't exist (i.e. no application owns it).
    NameHasNoOwner(String),

    /// No reply to a message expecting one, usually means a timeout occurred.
    NoReply(String),

    /// Something went wrong reading or writing to a socket, for example.
    IOError(String),

    /// A D-Bus bus address was malformed.
    BadAddress(String),

    /// Requested operation isn't supported (like ENOSYS on UNIX).
    NotSupported(String),

    /// Some limited resource is exhausted.
    LimitsExceeded(String),

    /// Security restrictions don't allow doing what you're trying to do.
    AccessDenied(String),

    /// Authentication didn't work.
    AuthFailed(String),

    /// Unable to connect to server (probably caused by ECONNREFUSED on a socket).
    NoServer(String),

    /// Certain timeout errors, possibly ETIMEDOUT on a socket.
    /// Note that `TimedOut` is used for message reply timeouts.
    Timeout(String),

    /// No network access (probably ENETUNREACH on a socket).
    NoNetwork(String),

    /// Can't bind a socket since its address is in use (i.e. EADDRINUSE).
    AddressInUse(String),

    /// The connection is disconnected and you're trying to use it.
    Disconnected(String),

    /// Invalid arguments passed to a method call.
    InvalidArgs(String),

    /// Missing file.
    FileNotFound(String),

    /// Existing file and the operation you're using does not silently overwrite.
    FileExists(String),

    /// Method name you invoked isn't known by the object you invoked it on.
    UnknownMethod(String),

    /// Object you invoked a method on isn't known.
    UnknownObject(String),

    /// Interface you invoked a method on isn't known by the object.
    UnknownInterface(String),

    /// Property you tried to access isn't known by the object.
    UnknownProperty(String),

    /// Property you tried to set is read-only.
    PropertyReadOnly(String),

    /// Certain timeout errors, e.g. while starting a service.
    TimedOut(String),

    /// Tried to remove or modify a match rule that didn't exist.
    MatchRuleNotFound(String),

    /// The match rule isn't syntactically valid.
    MatchRuleInvalid(String),

    /// While starting a new process, the exec() call failed.
    #[dbus_error(name = "Spawn.ExecFailed")]
    SpawnExecFailed(String),

    /// While starting a new process, the fork() call failed.
    #[dbus_error(name = "Spawn.ForkFailed")]
    SpawnForkFailed(String),

    /// While starting a new process, the child exited with a status code.
    #[dbus_error(name = "Spawn.ChildExited")]
    SpawnChildExited(String),

    /// While starting a new process, the child exited on a signal.
    #[dbus_error(name = "Spawn.ChildSignaled")]
    SpawnChildSignaled(String),

    /// While starting a new process, something went wrong.
    #[dbus_error(name = "Spawn.Failed")]
    SpawnFailed(String),

    /// We failed to setup the environment correctly.
    #[dbus_error(name = "Spawn.FailedToSetup")]
    SpawnFailedToSetup(String),

    /// We failed to setup the config parser correctly.
    #[dbus_error(name = "Spawn.ConfigInvalid")]
    SpawnConfigInvalid(String),

    /// Bus name was not valid.
    #[dbus_error(name = "Spawn.ServiceNotValid")]
    SpawnServiceNotValid(String),

    /// Service file not found in system-services directory.
    #[dbus_error(name = "Spawn.ServiceNotFound")]
    SpawnServiceNotFound(String),

    /// Permissions are incorrect on the setuid helper.
    #[dbus_error(name = "Spawn.PermissionsInvalid")]
    SpawnPermissionsInvalid(String),

    /// Service file invalid (Name, User or Exec missing).
    #[dbus_error(name = "Spawn.FileInvalid")]
    SpawnFileInvalid(String),

    /// There was not enough memory to complete the operation.
    #[dbus_error(name = "Spawn.NoMemory")]
    SpawnNoMemory(String),

    /// Tried to get a UNIX process ID and it wasn't available.
    UnixProcessIdUnknown(String),

    /// A type signature is not valid.
    InvalidSignature(String),

    /// A file contains invalid syntax or is otherwise broken.
    InvalidFileContent(String),

    /// Asked for SELinux security context and it wasn't available.
    SELinuxSecurityContextUnknown(String),

    /// Asked for ADT audit data and it wasn't available.
    AdtAuditDataUnknown(String),

    /// There's already an object with the requested object path.
    ObjectPathInUse(String),

    /// The message meta data does not match the payload. e.g. expected number of file descriptors
    /// were not sent over the socket this message was received on.
    InconsistentMessage(String),

    /// The message is not allowed without performing interactive authorization, but could have
    /// succeeded if an interactive authorization step was allowed.
    InteractiveAuthorizationRequired(String),

    /// The connection is not from a container, or the specified container instance does not exist.
    NotContainer(String),
}

assert_impl_all!(Error: Send, Sync, Unpin);

/// Alias for a `Result` with the error type [`zbus::fdo::Error`].
///
/// [`zbus::fdo::Error`]: enum.Error.html
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use crate::{fdo, Error, Message};
    use event_listener::Event;
    use futures_util::StreamExt;
    use ntest::timeout;
    use std::{convert::TryInto, future::ready, sync::Arc};
    use test_env_log::test;
    use tokio::runtime;
    use zbus_names::WellKnownName;

    #[test]
    fn error_from_zerror() {
        let m = Message::method(Some(":1.2"), None::<()>, "/", None::<()>, "foo", &()).unwrap();
        let m = Message::method_error(
            None::<()>,
            &m,
            "org.freedesktop.DBus.Error.TimedOut",
            &("so long"),
        )
        .unwrap();
        let e: Error = m.into();
        let e: fdo::Error = e.try_into().unwrap();
        assert_eq!(e, fdo::Error::TimedOut("so long".to_string()));
    }

    #[test]
    #[timeout(15000)]
    fn signal_connect() {
        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let conn = crate::blocking::Connection::session().unwrap();

        let owner_change_signaled = Arc::new(Event::new());
        let owner_change_listener = owner_change_signaled.listen();
        let name_acquired_signaled = Arc::new(Event::new());
        let name_acquired_listener = name_acquired_signaled.listen();

        let proxy = fdo::DBusProxy::new(&conn).unwrap();

        let well_known = "org.freedesktop.zbus.FdoSignalConnectTest";
        let mut unique_name = Some(conn.unique_name().unwrap().clone());
        proxy
            .connect_name_owner_changed(move |name, _, new_owner| {
                if name != well_known {
                    // Meant for the other testcase then
                    return;
                }
                if let Some(unique) = unique_name.take() {
                    assert_eq!(*new_owner.as_ref().unwrap(), *unique);
                }
                owner_change_signaled.notify(1);
            })
            .unwrap();
        // `NameAcquired` is emitted twice, first when the unique name is assigned on
        // connection and secondly after we ask for a specific name.
        proxy
            .connect_name_acquired(move |name| {
                if name == well_known {
                    name_acquired_signaled.notify(1);
                }
            })
            .unwrap();

        let well_known: WellKnownName<'static> = well_known.try_into().unwrap();
        proxy
            .request_name(
                well_known.clone(),
                fdo::RequestNameFlags::ReplaceExisting.into(),
            )
            .unwrap();

        owner_change_listener.wait();
        name_acquired_listener.wait();

        let result = proxy.release_name(well_known.clone()).unwrap();
        assert_eq!(result, fdo::ReleaseNameReply::Released);

        let result = proxy.release_name(well_known).unwrap();
        assert_eq!(result, fdo::ReleaseNameReply::NonExistent);
    }

    #[test]
    #[timeout(15000)]
    fn signal_stream() {
        // Multi-threaded scheduler.
        runtime::Runtime::new()
            .unwrap()
            .block_on(test_signal_stream());

        // single-threaded scheduler.
        runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(test_signal_stream());
    }

    async fn test_signal_stream() {
        let conn = crate::Connection::session().await.unwrap();

        {
            let conn = conn.clone();
            tokio::task::spawn(async move {
                loop {
                    conn.executor().tick().await;
                }
            });
        }

        let proxy = fdo::AsyncDBusProxy::new(&conn).await.unwrap();

        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let well_known = "org.freedesktop.zbus.FdoSignalStreamTest";
        let unique_name = conn.unique_name().unwrap();
        let owner_change_stream =
            proxy
                .receive_name_owner_changed()
                .await
                .unwrap()
                .filter(|signal| {
                    let args = signal.args().unwrap();

                    if args.name() != well_known {
                        // Meant for the other testcase then
                        return ready(false);
                    }
                    assert_eq!(*args.new_owner().as_ref().unwrap(), *unique_name);

                    ready(true)
                });

        let name_acquired_stream = proxy
            .receive_name_acquired()
            .await
            .unwrap()
            .filter(|signal| {
                let args = signal.args().unwrap();
                // `NameAcquired` is emitted twice, first when the unique name is assigned on
                // connection and secondly after we ask for a specific name.
                ready(args.name() == well_known)
            });
        let mut stream = owner_change_stream.zip(name_acquired_stream);

        let well_known: WellKnownName<'static> = well_known.try_into().unwrap();
        proxy
            .request_name(
                well_known.clone(),
                fdo::RequestNameFlags::ReplaceExisting.into(),
            )
            .await
            .unwrap();

        let (name_owner_changed, name_acquired) = stream.next().await.unwrap();
        assert_eq!(name_owner_changed.args().unwrap().name(), &well_known);
        assert_eq!(
            *name_owner_changed
                .args()
                .unwrap()
                .new_owner()
                .as_ref()
                .unwrap(),
            *unique_name
        );
        assert_eq!(name_acquired.args().unwrap().name(), &well_known);

        let result = proxy.release_name(well_known.clone()).await.unwrap();
        assert_eq!(result, fdo::ReleaseNameReply::Released);

        let result = proxy.release_name(well_known).await.unwrap();
        assert_eq!(result, fdo::ReleaseNameReply::NonExistent);

        let _stream = proxy.receive_features_changed().await.filter(|v| {
            dbg!(v);
            ready(true)
        });
    }
}
