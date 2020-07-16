use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use zvariant::{OwnedValue, Value};
use zvariant_derive::Type;

use crate as zbus;
use crate::{dbus_proxy, DBusError};

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
    fn request_name(&self, name: &str, flags: BitFlags<RequestNameFlags>) -> zbus::Result<u32>;

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

/// Errors from https://gitlab.freedesktop.org/dbus/dbus/-/blob/master/dbus/dbus-protocol.h
#[derive(Debug, DBusError, PartialEq)]
#[dbus_error(prefix = "org.freedesktop.DBus.Error")]
pub enum Error {
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

pub type Result<T> = std::result::Result<T, Error>;

impl From<zbus::MessageError> for Error {
    fn from(val: zbus::MessageError) -> Self {
        match val {
            zbus::MessageError::StrTooLarge => Self::LimitsExceeded("string too large".to_string()),
            zbus::MessageError::InsufficientData => {
                Self::InconsistentMessage("insufficient data".to_string())
            }
            zbus::MessageError::ExcessData => Self::InconsistentMessage("excess data".to_string()),
            zbus::MessageError::IncorrectEndian => {
                Self::InconsistentMessage("incorrect endian".to_string())
            }
            zbus::MessageError::Io(e) => Self::IOError(e.to_string()),
            zbus::MessageError::NoBodySignature => {
                Self::InvalidSignature("missing body signature".to_string())
            }
            zbus::MessageError::MissingSender => {
                Self::InconsistentMessage("missing sender".to_string())
            }
            zbus::MessageError::InvalidField => {
                Self::InconsistentMessage("invalid message field".to_string())
            }
            zbus::MessageError::Variant(e) => Self::InconsistentMessage(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fdo;
    use crate::{Error, Message};
    use std::convert::TryInto;

    #[test]
    fn error_from_zerror() {
        let m = Message::method(Some(":1.2"), None, "/", None, "foo", &()).unwrap();
        let m = Message::method_error(
            None,
            &m,
            "org.freedesktop.DBus.Error.TimedOut",
            &("so long"),
        )
        .unwrap();
        let e: Error = m.into();
        let e: fdo::Error = e.try_into().unwrap();
        assert_eq!(e, fdo::Error::TimedOut("so long".to_string()));
    }
}
