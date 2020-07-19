use std::convert::{TryFrom, TryInto};
use zvariant::{OwnedValue, Value};

use crate::{Connection, Error, Message, Result};

use crate::fdo::{self, IntrospectableProxy, PropertiesProxy};

/// A client-side interface proxy.
///
/// A `Proxy` is a helper to interact with an interface on a remote object.
///
/// # Example
///
/// ```
/// use std::result::Result;
/// use std::error::Error;
/// use zbus::{Connection, Proxy};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let connection = Connection::new_session()?;
///     let p = Proxy::new(
///         &connection,
///         "org.freedesktop.DBus",
///         "/org/freedesktop/DBus",
///         "org.freedesktop.DBus",
///     )?;
///     // owned return value
///     let _id: String = p.call("GetId", &())?;
///     // borrowed return value
///     let _id: &str = p.call_method("GetId", &())?.body()?;
///     Ok(())
/// }
/// ```
///
/// # Note
///
/// It is recommended to use the [`dbus_proxy`] macro, which provides a more convenient and
/// type-safe *façade* `Proxy` derived from a Rust trait.
///
/// ## Current limitations:
///
/// At this point, the `Proxy` is "simple". Notably, it doesn't:
/// - have any signal handling
/// - cache properties
/// - track the current name owner
/// - prevent auto-launching
///
/// [`dbus_proxy`]: attr.dbus_proxy.html
pub struct Proxy<'a> {
    conn: &'a Connection,
    destination: &'a str,
    path: &'a str,
    interface: &'a str,
}

impl<'a> Proxy<'a> {
    /// Create a new `Proxy` for the given destination/path/interface.
    pub fn new(
        conn: &'a Connection,
        destination: &'a str,
        path: &'a str,
        interface: &'a str,
    ) -> Result<Self> {
        Ok(Self {
            conn,
            destination,
            path,
            interface,
        })
    }

    /// Introspect the associated object, and return the XML description.
    ///
    /// See the [xml](xml/index.html) module for parsing the result.
    pub fn introspect(&self) -> fdo::Result<String> {
        IntrospectableProxy::new_for(self.conn, self.destination, self.path)?.introspect()
    }

    /// Get the property `property_name`.
    ///
    /// Effectively, call the `Get` method of the `org.freedesktop.DBus.Properties` interface.
    pub fn get_property<T>(&self, property_name: &str) -> fdo::Result<T>
    where
        T: TryFrom<OwnedValue>,
    {
        PropertiesProxy::new_for(self.conn, self.destination, self.path)?
            .get(self.interface, property_name)?
            .try_into()
            .map_err(|_| Error::InvalidReply.into())
    }

    /// Set the property `property_name`.
    ///
    /// Effectively, call the `Set` method of the `org.freedesktop.DBus.Properties` interface.
    pub fn set_property<'t, T: 't>(&self, property_name: &str, value: T) -> fdo::Result<()>
    where
        T: Into<Value<'t>>,
    {
        PropertiesProxy::new_for(self.conn, self.destination, self.path)?.set(
            self.interface,
            property_name,
            &value.into(),
        )
    }

    /// Call a method and return the reply.
    ///
    /// Typically, you would want to use [`call`] method instead. Use this method if you need to
    /// deserialize the reply message manually (this way, you can avoid avoid the memory
    /// allocation/copying, by deserializing the reply to an unowned type).
    ///
    /// [`call`]: struct.Proxy.html#method.call
    pub fn call_method<B>(&self, method_name: &str, body: &B) -> Result<Message>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let reply = self.conn.call_method(
            Some(self.destination),
            self.path,
            Some(self.interface),
            method_name,
            body,
        );
        match reply {
            Ok(mut reply) => {
                reply.disown_fds();

                Ok(reply)
            }
            Err(e) => Err(e),
        }
    }

    /// Call a method and return the reply body.
    ///
    /// Use [`call_method`] instead if you need to deserialize the reply manually/separately.
    ///
    /// [`call_method`]: struct.Proxy.html#method.call_method
    pub fn call<B, R>(&self, method_name: &str, body: &B) -> Result<R>
    where
        B: serde::ser::Serialize + zvariant::Type,
        R: serde::de::DeserializeOwned + zvariant::Type,
    {
        Ok(self.call_method(method_name, body)?.body()?)
    }
}
