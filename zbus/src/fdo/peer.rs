//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

use super::{Error, Result};

pub(crate) struct Peer;

/// Service-side implementation for the `org.freedesktop.DBus.Peer` interface.
/// This interface is implemented automatically for any object registered to the
/// [ObjectServer](crate::ObjectServer).
#[crate::interface(
    name = "org.freedesktop.DBus.Peer",
    introspection_docs = false,
    proxy(visibility = "pub")
)]
impl Peer {
    /// On receipt, an application should do nothing other than reply as usual. It does not matter
    /// which object path a ping is sent to.
    fn ping(&self) {}

    /// An application should reply the containing a hex-encoded UUID representing the identity of
    /// the machine the process is running on. This UUID must be the same for all processes on a
    /// single system at least until that system next reboots. It should be the same across reboots
    /// if possible, but this is not always possible to implement and is not guaranteed. It does not
    /// matter which object path a GetMachineId is sent to.
    fn get_machine_id(&self) -> Result<String> {
        let mut id = match std::fs::read_to_string("/var/lib/dbus/machine-id") {
            Ok(id) => id,
            Err(e) => {
                if let Ok(id) = std::fs::read_to_string("/etc/machine-id") {
                    id
                } else {
                    return Err(Error::IOError(format!(
                        "Failed to read from /var/lib/dbus/machine-id or /etc/machine-id: {e}"
                    )));
                }
            }
        };

        let len = id.trim_end().len();
        id.truncate(len);
        Ok(id)
    }
}
