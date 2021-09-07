use async_io::block_on;
use static_assertions::assert_impl_all;
use std::{
    convert::{TryFrom, TryInto},
    future::ready,
    sync::Arc,
};
use zbus_names::{BusName, InterfaceName, MemberName};
use zvariant::{ObjectPath, OwnedValue, Value};

use crate::{
    azync::{self, PropertyChangedHandlerId, SignalHandlerId},
    Connection, Error, Message, Result,
};

use crate::fdo;

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
///     let connection = Connection::session()?;
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
/// At the moment, `Proxy` doesn't:
///
/// * cache properties
/// * track the current name owner
/// * prevent auto-launching
///
/// [`dbus_proxy`]: attr.dbus_proxy.html
#[derive(derivative::Derivative)]
#[derivative(Clone, Debug)]
pub struct Proxy<'a> {
    #[derivative(Debug = "ignore")]
    conn: Connection,
    azync: azync::Proxy<'a>,
}

assert_impl_all!(Proxy<'_>: Send, Sync, Unpin);

impl<'a> Proxy<'a> {
    /// Create a new `Proxy` for the given destination/path/interface.
    pub fn new<D, P, I>(
        conn: &Connection,
        destination: D,
        path: P,
        interface: I,
    ) -> Result<Proxy<'a>>
    where
        D: TryInto<BusName<'a>>,
        P: TryInto<ObjectPath<'a>>,
        I: TryInto<InterfaceName<'a>>,
        D::Error: Into<Error>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
    {
        let proxy = block_on(azync::Proxy::new(
            conn.inner(),
            destination,
            path,
            interface,
        ))?;

        Ok(Self {
            conn: conn.clone(),
            azync: proxy,
        })
    }

    /// Create a new `Proxy` for the given destination/path/interface, taking ownership of all
    /// passed arguments.
    pub fn new_owned<D, P, I>(
        conn: Connection,
        destination: D,
        path: P,
        interface: I,
    ) -> Result<Proxy<'a>>
    where
        D: TryInto<BusName<'static>>,
        P: TryInto<ObjectPath<'static>>,
        I: TryInto<InterfaceName<'static>>,
        D::Error: Into<Error>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
    {
        let proxy = block_on(azync::Proxy::new_owned(
            conn.clone().into_inner(),
            destination,
            path,
            interface,
        ))?;

        Ok(Self { conn, azync: proxy })
    }

    /// Get a reference to the associated connection.
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Get a reference to the destination service name.
    pub fn destination(&self) -> &BusName<'_> {
        self.azync.destination()
    }

    /// Get a reference to the object path.
    pub fn path(&self) -> &ObjectPath<'_> {
        self.azync.path()
    }

    /// Get a reference to the interface.
    pub fn interface(&self) -> &InterfaceName<'_> {
        self.azync.interface()
    }

    /// Introspect the associated object, and return the XML description.
    ///
    /// See the [xml](xml/index.html) module for parsing the result.
    pub fn introspect(&self) -> fdo::Result<String> {
        block_on(self.azync.introspect())
    }

    /// Get the cached value of the property `property_name`.
    ///
    /// This returns `None` if the property is not in the cache.  This could be because the cache
    /// was invalidated by an update, because caching was disabled for this property or proxy, or
    /// because the cache has not yet been populated.  Use `get_property` to fetch the value from
    /// the peer.
    pub fn get_cached_property<T>(&self, property_name: &str) -> fdo::Result<Option<T>>
    where
        T: TryFrom<OwnedValue>,
    {
        self.azync.get_cached_property(property_name)
    }

    /// Get the property `property_name`.
    ///
    /// Get the property value from the cache or call the `Get` method of the
    /// `org.freedesktop.DBus.Properties` interface.
    pub fn get_property<T>(&self, property_name: &str) -> fdo::Result<T>
    where
        T: TryFrom<OwnedValue>,
    {
        block_on(self.azync.get_property(property_name))
    }

    /// Set the property `property_name`.
    ///
    /// Effectively, call the `Set` method of the `org.freedesktop.DBus.Properties` interface.
    pub fn set_property<'t, T: 't>(&self, property_name: &str, value: T) -> fdo::Result<()>
    where
        T: Into<Value<'t>>,
    {
        block_on(self.azync.set_property(property_name, value))
    }

    /// Call a method and return the reply.
    ///
    /// Typically, you would want to use [`call`] method instead. Use this method if you need to
    /// deserialize the reply message manually (this way, you can avoid the memory
    /// allocation/copying, by deserializing the reply to an unowned type).
    ///
    /// [`call`]: struct.Proxy.html#method.call
    pub fn call_method<'m, M, B>(&self, method_name: M, body: &B) -> Result<Arc<Message>>
    where
        M: TryInto<MemberName<'m>>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + zvariant::DynamicType,
    {
        block_on(self.azync.call_method(method_name, body))
    }

    /// Call a method and return the reply body.
    ///
    /// Use [`call_method`] instead if you need to deserialize the reply manually/separately.
    ///
    /// [`call_method`]: struct.Proxy.html#method.call_method
    pub fn call<'m, M, B, R>(&self, method_name: M, body: &B) -> Result<R>
    where
        M: TryInto<MemberName<'m>>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + zvariant::DynamicType,
        R: serde::de::DeserializeOwned + zvariant::Type,
    {
        block_on(self.azync.call(method_name, body))
    }

    /// Register a handler for signal named `signal_name`.
    ///
    /// A unique ID for the handler is returned, which can be used to deregister this handler using
    /// [`Self::disconnect_signal`] method.
    ///
    /// *Note:* The signal handler will be called by the executor thread of the [`Connection`].
    /// See the [`azync::Connection::executor`] documentation for an example of how you can run the
    /// executor (and in turn all the signal handlers called) in your own thread.
    ///
    /// ### Errors
    ///
    /// This method can fail if addition of the relevant match rule on the bus fails. You can
    /// safely `unwrap` the `Result` if you're certain that associated connection is not a bus
    /// connection.
    pub fn connect_signal<M, H>(
        &self,
        signal_name: M,
        mut handler: H,
    ) -> fdo::Result<SignalHandlerId>
    where
        M: TryInto<MemberName<'static>>,
        M::Error: Into<Error>,
        H: FnMut(&Message) + Send + 'static,
    {
        block_on(self.azync.connect_signal(signal_name, move |msg| {
            Box::pin({
                handler(msg);

                ready(())
            })
        }))
    }

    /// Deregister the signal handler with the ID `handler_id`.
    ///
    /// This method returns `Ok(true)` if a handler with the id `handler_id` is found and removed;
    /// `Ok(false)` otherwise.
    ///
    /// ### Errors
    ///
    /// This method can fail if removal of the relevant match rule on the bus fails. You can
    /// safely `unwrap` the `Result` if you're certain that associated connection is not a bus
    /// connection.
    pub fn disconnect_signal(&self, handler_id: SignalHandlerId) -> fdo::Result<bool> {
        block_on(self.azync.disconnect_signal(handler_id))
    }

    /// Register a changed handler for the property named `property_name`.
    ///
    /// A unique ID for the handler is returned, which can be used to deregister this handler using
    /// [`Self::disconnect_signal`] method.
    ///
    /// *Note:* The signal handler will be called by the executor thread of the [`Connection`].
    /// See the [`azync::Connection::executor`] documentation for an example of how you can run the
    /// executor (and in turn all the signal handlers called) in your own thread.
    ///
    /// # Errors
    ///
    /// The current implementation requires cached properties. It returns an [`Error::Unsupported`]
    /// if the proxy isn't setup with cache.
    pub fn connect_property_changed<H>(
        &self,
        property_name: &'static str,
        mut handler: H,
    ) -> Result<PropertyChangedHandlerId>
    where
        H: FnMut(Option<&Value<'_>>) + Send + 'static,
    {
        block_on(
            self.azync
                .connect_property_changed(property_name, move |v| {
                    Box::pin({
                        handler(v);
                        ready(())
                    })
                }),
        )
    }

    /// Deregister the property handler with the ID `handler_id`.
    ///
    /// This method returns `Ok(true)` if a handler with the id `handler_id` is found and removed;
    /// `Ok(false)` otherwise.
    pub fn disconnect_property_changed(
        &self,
        handler_id: PropertyChangedHandlerId,
    ) -> Result<bool> {
        block_on(self.azync.disconnect_property_changed(handler_id))
    }

    /// Get a reference to the underlying async Proxy.
    pub fn inner(&self) -> &azync::Proxy<'a> {
        &self.azync
    }

    /// Get the underlying async Proxy, consuming `self`.
    pub fn into_inner(self) -> azync::Proxy<'a> {
        self.azync
    }
}

impl<'a> std::convert::AsRef<Proxy<'a>> for Proxy<'a> {
    fn as_ref(&self) -> &Proxy<'a> {
        self
    }
}

impl<'a> From<azync::Proxy<'a>> for Proxy<'a> {
    fn from(proxy: azync::Proxy<'a>) -> Self {
        Self {
            conn: proxy.connection().clone().into(),
            azync: proxy,
        }
    }
}

#[cfg(test)]
mod tests {
    use event_listener::Event;
    use zbus_names::{BusName, UniqueName};

    use super::*;
    use ntest::timeout;
    use test_env_log::test;
    use zvariant::Optional;

    #[test]
    #[timeout(15000)]
    fn signal() {
        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let conn = Connection::session().unwrap();

        let owner_change_signaled = Arc::new(Event::new());
        let owner_change_listener = owner_change_signaled.listen();
        let name_acquired_signaled = Arc::new(Event::new());
        let name_acquired_listener = name_acquired_signaled.listen();

        let proxy = fdo::DBusProxy::new(&conn).unwrap();
        let well_known = "org.freedesktop.zbus.ProxySignalTest";
        let unique_name = conn.unique_name().unwrap().to_string();
        {
            let signaled = owner_change_signaled.clone();
            proxy
                .connect_signal("NameOwnerChanged", move |m| {
                    let (name, _, new_owner) = m
                        .body::<(
                            BusName<'_>,
                            Optional<UniqueName<'_>>,
                            Optional<UniqueName<'_>>,
                        )>()
                        .unwrap();
                    if name != well_known {
                        // Meant for the other testcase then
                        return;
                    }
                    assert_eq!(*new_owner.as_ref().unwrap(), *unique_name);
                    signaled.notify(1);
                })
                .unwrap();
        }
        {
            let signaled = name_acquired_signaled.clone();
            // `NameAcquired` is emitted twice, first when the unique name is assigned on
            // connection and secondly after we ask for a specific name.
            proxy
                .connect_signal("NameAcquired", move |m| {
                    if m.body::<&str>().unwrap() == well_known {
                        signaled.notify(1);
                    }
                })
                .unwrap();
        }

        fdo::DBusProxy::new(&conn)
            .unwrap()
            .request_name(
                well_known.try_into().unwrap(),
                fdo::RequestNameFlags::ReplaceExisting.into(),
            )
            .unwrap();

        let h = proxy
            .connect_features_changed(|val| {
                dbg!(val);
            })
            .unwrap();

        owner_change_listener.wait();
        name_acquired_listener.wait();

        proxy.disconnect_property_changed(h).unwrap();
    }
}
