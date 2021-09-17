use std::{
    convert::TryInto,
    ops::{Deref, DerefMut},
};

use async_io::block_on;
use static_assertions::assert_impl_all;
use zvariant::ObjectPath;

use crate::{Error, Interface, InterfaceDeref, InterfaceDerefMut, Result, SignalContext};

/// An object server, holding server-side D-Bus objects & interfaces.
///
/// Object servers hold interfaces on various object paths, and expose them over D-Bus.
///
/// All object paths will have the standard interfaces implemented on your behalf, such as
/// `org.freedesktop.DBus.Introspectable` or `org.freedesktop.DBus.Properties`.
///
/// # Example
///
/// This example exposes the `org.myiface.Example.Quit` method on the `/org/zbus/path`
/// path.
///
/// ```no_run
///# use std::error::Error;
/// use zbus::{blocking::{Connection, ObjectServer}, dbus_interface};
/// use std::sync::{Arc, Mutex};
/// use event_listener::Event;
///
/// struct Example {
///     // Interfaces are owned by the ObjectServer. They can have
///     // `&mut self` methods.
///     quit_event: Event,
/// }
///
/// impl Example {
///     fn new(quit_event: Event) -> Self {
///         Self { quit_event }
///     }
/// }
///
/// #[dbus_interface(name = "org.myiface.Example")]
/// impl Example {
///     // This will be the "Quit" D-Bus method.
///     fn quit(&mut self) {
///         self.quit_event.notify(1);
///     }
///
///     // See `dbus_interface` documentation to learn
///     // how to expose properties & signals as well.
/// }
///
/// let connection = Connection::session()?;
///
/// let quit_event = Event::new();
/// let quit_listener = quit_event.listen();
/// let interface = Example::new(quit_event);
/// connection
///     .object_server_mut()
///     .at("/org/zbus/path", interface)?;
///
/// quit_listener.wait();
///# Ok::<_, Box<dyn Error + Send + Sync>>(())
/// ```
#[derive(Debug)]
pub struct ObjectServer {
    azync: crate::ObjectServer,
}

assert_impl_all!(ObjectServer: Send, Sync, Unpin);

impl ObjectServer {
    /// Creates a new D-Bus `ObjectServer`.
    pub(crate) fn new(conn: &crate::Connection) -> Self {
        Self {
            azync: crate::ObjectServer::new(conn),
        }
    }

    /// Register a D-Bus [`Interface`] at a given path. (see the example above)
    ///
    /// If the interface already exists at this path, returns false.
    ///
    /// [`Interface`]: trait.Interface.html
    pub fn at<'p, P, I>(&mut self, path: P, iface: I) -> Result<bool>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        self.azync.at(path, iface)
    }

    /// Unregister a D-Bus [`Interface`] at a given path.
    ///
    /// If there are no more interfaces left at that path, destroys the object as well.
    /// Returns whether the object was destroyed.
    ///
    /// [`Interface`]: trait.Interface.html
    pub fn remove<'p, I, P>(&mut self, path: P) -> Result<bool>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        self.azync.remove::<I, P>(path)
    }

    /// Run `func` with the given path & interface.
    ///
    /// # Errors
    ///
    /// If the interface is not registered at the given path, `Error::InterfaceNotFound` error is
    /// returned.
    ///
    /// # Examples
    ///
    /// The typical use of this is to emit signals outside of a dispatched handler:
    ///
    /// ```no_run
    ///# use std::error::Error;
    ///# use async_io::block_on;
    ///# use zbus::{
    ///#    InterfaceDeref, SignalContext,
    ///#    blocking::{Connection, ObjectServer},
    ///#    dbus_interface,
    ///# };
    ///#
    /// struct MyIface;
    /// #[dbus_interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///     #[dbus_interface(signal)]
    ///     async fn emit_signal(ctxt: &SignalContext<'_>) -> zbus::Result<()>;
    /// }
    ///
    ///# let connection = Connection::session()?;
    ///#
    ///# let path = "/org/zbus/path";
    ///# connection.object_server_mut().at(path, MyIface)?;
    /// connection
    ///     .object_server()
    ///     .with(path, |_iface: InterfaceDeref<'_, MyIface>, signal_ctxt| {
    ///         block_on(MyIface::emit_signal(&signal_ctxt))
    ///     })?;
    ///#
    ///#
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn with<'server, 'p, P, F, I>(&'server self, path: P, func: F) -> Result<()>
    where
        F: FnOnce(InterfaceDeref<'server, I>, SignalContext<'p>) -> Result<()>,
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let interface = self.get_interface::<_, I>(path.clone())?;
        let conn = self.azync.connection();
        let ctxt = SignalContext::new(&conn, path)?;

        func(interface, ctxt)
    }

    /// Run `func` with the given path & interface.
    ///
    /// Same as [`ObjectServer::with`], except `func` gets a mutable reference.
    ///
    /// # Examples
    ///
    /// The typical use of this is property changes outside of a dispatched handler:
    ///
    /// ```no_run
    ///# use std::error::Error;
    ///# use async_io::block_on;
    ///# use zbus::{
    ///#    InterfaceDerefMut, SignalContext,
    ///#    blocking::{Connection, ObjectServer},
    ///#    dbus_interface,
    ///# };
    ///#
    /// struct MyIface(u32);
    ///
    /// #[dbus_interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///      #[dbus_interface(property)]
    ///      fn count(&self) -> u32 {
    ///          self.0
    ///      }
    /// }
    ///
    ///# let connection = Connection::session()?;
    ///#
    ///# let path = "/org/zbus/path";
    ///# connection.object_server_mut().at(path, MyIface(0))?;
    /// connection
    ///     .object_server()
    ///     .with_mut(path, |mut iface: InterfaceDerefMut<'_, MyIface>, signal_ctxt| {
    ///         iface.0 = 42;
    ///         block_on(iface.count_changed(&signal_ctxt))
    ///     })?;
    ///#
    ///#
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn with_mut<'server, 'p, P, F, I>(&'server self, path: P, func: F) -> Result<()>
    where
        F: FnOnce(InterfaceDerefMut<'server, I>, SignalContext<'p>) -> Result<()>,
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let interface = self.get_interface_mut::<_, I>(path.clone())?;
        let conn = self.azync.connection();
        let ctxt = SignalContext::new(&conn, path)?;

        func(interface, ctxt)
    }

    /// Get a reference to the interface at the given path.
    pub fn get_interface<'p, P, I>(&self, path: P) -> Result<InterfaceDeref<'_, I>>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        block_on(self.azync.get_interface(path))
    }

    /// Get a reference to the interface at the given path.
    ///
    /// **WARNINGS:** Since `self` will not be able to access the interface in question until the
    /// return value of this method is dropped, it is highly recommended to prefer
    /// [`ObjectServer::with`] or [`ObjectServer::with_mut`] over this method. They are also more
    /// convenient to use for emitting signals and changing properties.
    ///
    /// # Errors
    ///
    /// If the interface is not registered at the given path, `Error::InterfaceNotFound` error is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    ///# use std::error::Error;
    ///# use async_io::block_on;
    ///# use zbus::{SignalContext, blocking::{Connection, ObjectServer}, dbus_interface};
    ///
    /// struct MyIface(u32);
    ///
    /// #[dbus_interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///    #[dbus_interface(property)]
    ///    fn count(&self) -> u32 {
    ///        self.0
    ///    }
    /// }
    /// // Setup connection and object_server etc here and then in another part of the code:
    ///#
    ///# let connection = Connection::session()?;
    ///#
    ///# let path = "/org/zbus/path";
    ///# connection.object_server_mut().at(path, MyIface(22))?;
    /// let mut object_server = connection.object_server();
    /// let mut iface = object_server.get_interface_mut::<_, MyIface>(path)?;
    /// // Note: This will not be needed when using `ObjectServer::with_mut`
    /// let ctxt = SignalContext::new(connection.inner(), path)?;
    /// iface.0 = 42;
    /// block_on(iface.count_changed(&ctxt))?;
    ///#
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn get_interface_mut<'p, P, I>(&self, path: P) -> Result<InterfaceDerefMut<'_, I>>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        block_on(self.azync.get_interface_mut(path))
    }

    /// Get a reference to the underlying async ObjectServer.
    pub fn inner(&self) -> &crate::ObjectServer {
        &self.azync
    }

    /// Get a mutable reference to the underlying async ObjectServer.
    pub fn inner_mut(&mut self) -> &mut crate::ObjectServer {
        &mut self.azync
    }

    /// Get the underlying async ObjectServer, consuming `self`.
    pub fn into_inner(self) -> crate::ObjectServer {
        self.azync
    }
}

impl Deref for ObjectServer {
    type Target = crate::ObjectServer;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl DerefMut for ObjectServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}

impl From<crate::ObjectServer> for ObjectServer {
    fn from(azync: crate::ObjectServer) -> Self {
        Self { azync }
    }
}
