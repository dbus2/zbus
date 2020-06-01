use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use zbus_derive::dbus_proxy;
use zvariant::{OwnedValue, Value};
use zvariant_derive::Type;

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

type ManagedObjects = HashMap<String, HashMap<String, HashMap<String, OwnedValue>>>;

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
    fn get_managed_objects(&self) -> zbus::Result<ManagedObjects>;
}

#[dbus_proxy(interface = "org.freedesktop.DBus.Peer")]
trait Peer {
    /// On receipt, an application should do nothing other than reply as usual. It does not matter
    /// which object path a ping is sent to.
    fn ping(&self) -> zbus::Result<()>;

    /// An application should reply the containing a hex-encoded UUID representing the identity of
    /// the machine the process is running on. This UUID must be the same for all processes on a
    /// single system at least until that system next reboots. It should be the same across reboots
    /// if possible, but this is not always possible to implement and is not guaranteed. It does not
    /// matter which object path a GetMachineId is sent to.
    fn get_machine_id(&self) -> zbus::Result<String>;
}

#[dbus_proxy(interface = "org.freedesktop.DBus.Monitoring")]
trait Monitoring {
    /// Converts the connection into a monitor connection which can be used as a
    /// debugging/monitoring tool.
    fn become_monitor(&self, n1: &[&str], n2: u32) -> zbus::Result<()>;
}

#[dbus_proxy(interface = "org.freedesktop.DBus.Debug.Stats")]
trait Stats {
    /// GetStats
    fn get_stats(&self) -> zbus::Result<Vec<HashMap<String, OwnedValue>>>;

    /// GetConnectionStats
    fn get_connection_stats(&self, n1: &str) -> zbus::Result<Vec<HashMap<String, OwnedValue>>>;

    /// GetAllMatchRules
    fn get_all_match_rules(&self) -> zbus::Result<Vec<HashMap<String, Vec<String>>>>;
}

#[repr(u32)]
#[derive(Type, BitFlags, Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum RequestNameFlags {
    AllowReplacement = 0x01,
    ReplaceExisting = 0x02,
    DoNotQueue = 0x04,
}

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
pub enum RequestNameReply {
    PrimaryOwner = 0x01,
    InQueue = 0x02,
    Exists = 0x03,
    AlreadyOwner = 0x04,
}

#[dbus_proxy]
trait DBus {
    /// Adds a match rule to match messages going through the message bus
    fn add_match(&self, rule: &str) -> zbus::Result<()>;

    /// Returns auditing data used by Solaris ADT, in an unspecified binary format.
    fn get_adt_audit_session_data(&self, bus_name: &str) -> zbus::Result<Vec<u8>>;

    /// Returns as many credentials as possible for the process connected to the server.
    fn get_connection_credentials(
        &self,
        bus_name: &str,
    ) -> zbus::Result<HashMap<String, OwnedValue>>;

    /// Returns the security context used by SELinux, in an unspecified format.
    #[dbus_proxy(name = "GetConnectionSELinuxSecurityContext")]
    fn get_connection_selinux_security_context(&self, bus_name: &str) -> zbus::Result<Vec<u8>>;

    /// Returns the Unix process ID of the process connected to the server.
    fn get_connection_unix_process_id(&self, bus_name: &str) -> zbus::Result<u32>;

    /// Returns the Unix user ID of the process connected to the server.
    fn get_connection_unix_user(&self, bus_name: &str) -> zbus::Result<u32>;

    /// Gets the unique ID of the bus.
    fn get_id(&self) -> zbus::Result<String>;

    /// Returns the unique connection name of the primary owner of the name given.
    fn get_name_owner(&self, name: &str) -> zbus::Result<String>;

    /// Returns the unique name assigned to the connection.
    fn hello(&self) -> zbus::Result<String>;

    /// Returns a list of all names that can be activated on the bus.
    fn list_activatable_names(&self) -> zbus::Result<Vec<String>>;

    /// Returns a list of all currently-owned names on the bus.
    fn list_names(&self) -> zbus::Result<Vec<String>>;

    /// List the connections currently queued for a bus name.
    fn list_queued_owners(&self, name: &str) -> zbus::Result<Vec<String>>;

    /// Checks if the specified name exists (currently has an owner).
    fn name_has_owner(&self, name: &str) -> zbus::Result<bool>;

    /// Ask the message bus to release the method caller's claim to the given name.
    fn release_name(&self, name: &str) -> zbus::Result<()>;

    /// Reload server configuration.
    fn reload_config(&self) -> zbus::Result<()>;

    /// Removes the first rule that matches.
    fn remove_match(&self, rule: &str) -> zbus::Result<()>;

    /// Ask the message bus to assign the given name to the method caller.
    fn request_name(&self, name: &str, flags: RequestNameFlags) -> zbus::Result<u32>;

    /// Tries to launch the executable associated with a name (service
    /// activation), as an explicit request.
    fn start_service_by_name(&self, name: &str, flags: u32) -> zbus::Result<u32>;

    /// This method adds to or modifies that environment when activating services.
    fn update_activation_environment(&self, environment: HashMap<&str, &str>) -> zbus::Result<()>;

    /// This signal indicates that the owner of a name has
    /// changed. It's also the signal to use to detect the appearance
    /// of new names on the bus.
    //#[dbus_proxy(signal)]
    //fn name_owner_changed(&self, name: &str, old_owner: &str, new_owner: &str);

    /// This signal is sent to a specific application when it loses ownership of a name.
    //#[dbus_proxy(signal)]
    //fn name_lost(&self, name: &str);

    /// This signal is sent to a specific application when it gains ownership of a name.
    //#[dbus_proxy(signal)]
    //fn name_acquired(&self, name: &str);

    /// This property lists abstract “features” provided by the message bus, and can be used by
    /// clients to detect the capabilities of the message bus with which they are communicating.
    #[dbus_proxy(property)]
    fn features(&self) -> zbus::Result<Vec<String>>;

    #[dbus_proxy(property)]
    fn interfaces(&self) -> zbus::Result<Vec<String>>;
}
