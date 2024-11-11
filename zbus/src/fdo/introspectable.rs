//! D-Bus standard interfaces.
//!
//! The D-Bus specification defines the message bus messages and some standard interfaces that may
//! be useful across various D-Bus applications. This module provides their proxy.

use static_assertions::assert_impl_all;

use super::{Error, Result};
use crate::{interface, message::Header, ObjectServer};

/// Service-side implementation for the `org.freedesktop.DBus.Introspectable` interface.
/// This interface is implemented automatically for any object registered to the
/// [ObjectServer](crate::ObjectServer).
pub(crate) struct Introspectable;

#[interface(
    name = "org.freedesktop.DBus.Introspectable",
    introspection_docs = false,
    proxy(default_path = "/", visibility = "pub")
)]
impl Introspectable {
    /// Returns an XML description of the object, including its interfaces (with signals and
    /// methods), objects below it in the object path tree, and its properties.
    async fn introspect(
        &self,
        #[zbus(object_server)] server: &ObjectServer,
        #[zbus(header)] header: Header<'_>,
    ) -> Result<String> {
        let path = header.path().ok_or(crate::Error::MissingField)?;
        let root = server.root().read().await;
        let node = root
            .get_child(path)
            .ok_or_else(|| Error::UnknownObject(format!("Unknown object '{path}'")))?;

        Ok(node.introspect().await)
    }
}

assert_impl_all!(IntrospectableProxy<'_>: Send, Sync, Unpin);
#[cfg(feature = "blocking-api")]
assert_impl_all!(IntrospectableProxyBlocking<'_>: Send, Sync, Unpin);
