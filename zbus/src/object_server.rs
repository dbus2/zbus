use async_lock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{
    collections::{hash_map::Entry, HashMap},
    convert::TryInto,
    fmt::Write,
    future::Future,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use static_assertions::assert_impl_all;
use zbus_names::InterfaceName;
use zvariant::{ObjectPath, OwnedObjectPath};

use crate::{
    fdo,
    fdo::{Introspectable, Peer, Properties},
    Connection, DispatchResult, Error, Interface, Message, MessageHeader, MessageType, Result,
    SignalContext, WeakConnection,
};

/// Opaque structure that derefs to an `Interface` type.
pub struct InterfaceDeref<'d, I> {
    iface: RwLockReadGuard<'d, dyn Interface>,
    phantom: PhantomData<I>,
}

impl<I> Deref for InterfaceDeref<'_, I>
where
    I: Interface,
{
    type Target = I;

    fn deref(&self) -> &I {
        self.iface.downcast_ref::<I>().unwrap()
    }
}

/// Opaque structure that mutably derefs to an `Interface` type.
pub struct InterfaceDerefMut<'d, I> {
    iface: RwLockWriteGuard<'d, dyn Interface>,
    phantom: PhantomData<I>,
}

impl<I> Deref for InterfaceDerefMut<'_, I>
where
    I: Interface,
{
    type Target = I;

    fn deref(&self) -> &I {
        self.iface.downcast_ref::<I>().unwrap()
    }
}

impl<I> DerefMut for InterfaceDerefMut<'_, I>
where
    I: Interface,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.iface.downcast_mut::<I>().unwrap()
    }
}

#[derive(Default, derivative::Derivative)]
#[derivative(Debug)]
pub(crate) struct Node {
    path: OwnedObjectPath,
    children: HashMap<String, Node>,
    #[derivative(Debug = "ignore")]
    interfaces: HashMap<InterfaceName<'static>, Arc<RwLock<dyn Interface>>>,
}

impl Node {
    pub(crate) fn new(path: OwnedObjectPath) -> Self {
        let mut node = Self {
            path,
            ..Default::default()
        };
        node.at(Peer::name(), Peer);
        node.at(Introspectable::name(), Introspectable);
        node.at(Properties::name(), Properties);

        node
    }

    pub(crate) fn interface_lock(
        &self,
        interface_name: InterfaceName<'_>,
    ) -> Option<Arc<RwLock<dyn Interface>>> {
        self.interfaces.get(&interface_name).cloned()
    }

    fn remove_interface(&mut self, interface_name: InterfaceName<'static>) -> bool {
        self.interfaces.remove(&interface_name).is_some()
    }

    fn is_empty(&self) -> bool {
        !self
            .interfaces
            .keys()
            .any(|k| *k != Peer::name() && *k != Introspectable::name() && *k != Properties::name())
    }

    fn remove_node(&mut self, node: &str) -> bool {
        self.children.remove(node).is_some()
    }

    fn at<I>(&mut self, name: InterfaceName<'static>, iface: I) -> bool
    where
        I: Interface,
    {
        match self.interfaces.entry(name) {
            Entry::Vacant(e) => e.insert(Arc::new(RwLock::new(iface))),
            Entry::Occupied(_) => return false,
        };

        true
    }

    async fn with_iface_func<'node, 's, F, Fut, I>(
        &'node self,
        func: F,
        signal_ctxt: SignalContext<'s>,
    ) -> Result<()>
    where
        F: FnOnce(InterfaceDeref<'node, I>, SignalContext<'s>) -> Fut,
        Fut: Future<Output = Result<()>>,
        I: Interface,
    {
        let iface = self.get_interface::<I>().await?;

        func(iface, signal_ctxt).await
    }

    async fn with_iface_func_mut<'node, 's, F, Fut, I>(
        &'node self,
        func: F,
        signal_ctxt: SignalContext<'s>,
    ) -> Result<()>
    where
        F: FnOnce(InterfaceDerefMut<'node, I>, SignalContext<'s>) -> Fut,
        Fut: Future<Output = Result<()>>,
        I: Interface,
    {
        let iface = self.get_interface_mut::<I>().await?;

        func(iface, signal_ctxt).await
    }

    async fn get_interface<I>(&self) -> Result<InterfaceDeref<'_, I>>
    where
        I: Interface,
    {
        let iface = self
            .interfaces
            .get(&I::name())
            .ok_or(Error::InterfaceNotFound)?
            .read()
            .await;
        // Ensure what we return can later be dowcasted safely.
        iface.downcast_ref::<I>().ok_or(Error::InterfaceNotFound)?;

        Ok(InterfaceDeref {
            iface,
            phantom: PhantomData,
        })
    }

    async fn get_interface_mut<I>(&self) -> Result<InterfaceDerefMut<'_, I>>
    where
        I: Interface,
    {
        let mut iface = self
            .interfaces
            .get(&I::name())
            .ok_or(Error::InterfaceNotFound)?
            .write()
            .await;
        // Ensure what we return can later be dowcasted safely.
        iface.downcast_ref::<I>().ok_or(Error::InterfaceNotFound)?;
        iface.downcast_mut::<I>().ok_or(Error::InterfaceNotFound)?;

        Ok(InterfaceDerefMut {
            iface,
            phantom: PhantomData,
        })
    }

    #[async_recursion::async_recursion]
    async fn introspect_to_writer<W: Write + Send>(&self, writer: &mut W, level: usize) {
        if level == 0 {
            writeln!(
                writer,
                r#"
<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN"
 "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node>"#
            )
            .unwrap();
        }

        for iface in self.interfaces.values() {
            iface.read().await.introspect_to_writer(writer, level + 2);
        }

        for (path, node) in &self.children {
            let level = level + 2;
            writeln!(
                writer,
                "{:indent$}<node name=\"{}\">",
                "",
                path,
                indent = level
            )
            .unwrap();
            node.introspect_to_writer(writer, level).await;
            writeln!(writer, "{:indent$}</node>", "", indent = level).unwrap();
        }

        if level == 0 {
            writeln!(writer, "</node>").unwrap();
        }
    }

    pub(crate) async fn introspect(&self) -> String {
        let mut xml = String::with_capacity(1024);

        self.introspect_to_writer(&mut xml, 0).await;

        xml
    }
}

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
/// use zbus::{Connection, ObjectServer, dbus_interface};
/// use std::sync::{Arc, Mutex};
/// use event_listener::Event;
///# use async_io::block_on;
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
///     async fn quit(&mut self) {
///         self.quit_event.notify(1);
///     }
///
///     // See `dbus_interface` documentation to learn
///     // how to expose properties & signals as well.
/// }
///
///# block_on(async {
/// let connection = Connection::session().await?;
///
/// let quit_event = Event::new();
/// let quit_listener = quit_event.listen();
/// let interface = Example::new(quit_event);
/// connection
///     .object_server_mut()
///     .await
///     .at("/org/zbus/path", interface)?;
///
/// quit_listener.await;
///# Ok::<_, Box<dyn Error + Send + Sync>>(())
///# });
///# Ok::<_, Box<dyn Error + Send + Sync>>(())
/// ```
#[derive(Debug)]
pub struct ObjectServer {
    conn: WeakConnection,
    root: Node,
}

assert_impl_all!(ObjectServer: Send, Sync, Unpin);

impl ObjectServer {
    /// Creates a new D-Bus `ObjectServer`.
    pub(crate) fn new(conn: &Connection) -> Self {
        Self {
            conn: conn.into(),
            root: Node::new("/".try_into().expect("zvariant bug")),
        }
    }

    // Get the Node at path.
    pub(crate) fn get_node(&self, path: &ObjectPath<'_>) -> Option<&Node> {
        let mut node = &self.root;

        for i in path.split('/').skip(1) {
            if i.is_empty() {
                continue;
            }
            match node.children.get(i) {
                Some(n) => node = n,
                None => return None,
            }
        }

        Some(node)
    }

    // Get the Node at path. Optionally create one if it doesn't exist.
    fn get_node_mut(&mut self, path: &ObjectPath<'_>, create: bool) -> Option<&mut Node> {
        let mut node = &mut self.root;
        let mut node_path = String::new();

        for i in path.split('/').skip(1) {
            if i.is_empty() {
                continue;
            }
            write!(&mut node_path, "/{}", i).unwrap();
            match node.children.entry(i.into()) {
                Entry::Vacant(e) => {
                    if create {
                        let path = node_path.as_str().try_into().expect("Invalid Object Path");
                        node = e.insert(Node::new(path));
                    } else {
                        return None;
                    }
                }
                Entry::Occupied(e) => node = e.into_mut(),
            }
        }

        Some(node)
    }

    /// Register a D-Bus [`Interface`] at a given path. (see the example above)
    ///
    /// If the interface already exists at this path, returns false.
    pub fn at<'p, P, I>(&mut self, path: P, iface: I) -> Result<bool>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        Ok(self.get_node_mut(&path, true).unwrap().at(I::name(), iface))
    }

    /// Unregister a D-Bus [`Interface`] at a given path.
    ///
    /// If there are no more interfaces left at that path, destroys the object as well.
    /// Returns whether the object was destroyed.
    pub fn remove<'p, I, P>(&mut self, path: P) -> Result<bool>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let node = self
            .get_node_mut(&path, false)
            .ok_or(Error::InterfaceNotFound)?;
        if !node.remove_interface(I::name()) {
            return Err(Error::InterfaceNotFound);
        }
        if node.is_empty() {
            let mut path_parts = path.rsplit('/').filter(|i| !i.is_empty());
            let last_part = path_parts.next().unwrap();
            let ppath = ObjectPath::from_string_unchecked(
                path_parts.fold(String::new(), |a, p| format!("/{}{}", p, a)),
            );
            self.get_node_mut(&ppath, false)
                .unwrap()
                .remove_node(last_part);
            return Ok(true);
        }
        Ok(false)
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
    ///# use zbus::{Connection, InterfaceDeref, ObjectServer, SignalContext, dbus_interface};
    ///# use async_io::block_on;
    ///#
    /// struct MyIface;
    /// #[dbus_interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///     #[dbus_interface(signal)]
    ///     async fn emit_signal(ctxt: &SignalContext<'_>) -> zbus::Result<()>;
    /// }
    ///
    ///# block_on(async {
    ///# let connection = Connection::session().await?;
    ///#
    ///# let path = "/org/zbus/path";
    ///# connection.object_server_mut().await.at(path, MyIface)?;
    /// connection
    ///     .object_server()
    ///     .await
    ///     .with(path, |_iface: InterfaceDeref<'_, MyIface>, signal_ctxt| async move {
    ///         MyIface::emit_signal(&signal_ctxt).await
    ///     })
    ///     .await?;
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    ///# })?;
    ///#
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub async fn with<'server, 'p, P, F, Fut, I>(&'server self, path: P, func: F) -> Result<()>
    where
        F: FnOnce(InterfaceDeref<'server, I>, SignalContext<'p>) -> Fut,
        Fut: Future<Output = Result<()>>,
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let node = self.get_node(&path).ok_or(Error::InterfaceNotFound)?;
        let conn = self.connection();
        // SAFETY: We know that there is a valid path on the node as we already converted w/o error.
        let ctxt = SignalContext::new(&conn, path).unwrap();

        node.with_iface_func(func, ctxt).await
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
    ///# use zbus::{Connection, InterfaceDerefMut, ObjectServer, SignalContext, dbus_interface};
    ///# use async_io::block_on;
    ///#
    /// struct MyIface(u32);
    ///
    /// #[dbus_interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///      #[dbus_interface(property)]
    ///      async fn count(&self) -> u32 {
    ///          self.0
    ///      }
    /// }
    ///
    ///# block_on(async {
    ///# let connection = Connection::session().await?;
    ///#
    ///# let path = "/org/zbus/path";
    ///# connection.object_server_mut().await.at(path, MyIface(0))?;
    /// connection
    ///     .object_server()
    ///     .await
    ///     .with_mut(path, |mut iface: InterfaceDerefMut<'_, MyIface>, signal_ctxt| async move {
    ///         iface.0 = 42;
    ///         iface.count_changed(&signal_ctxt).await
    ///     })
    ///     .await?;
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    ///# })?;
    ///#
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub async fn with_mut<'server, 'p, P, F, Fut, I>(&'server self, path: P, func: F) -> Result<()>
    where
        F: FnOnce(InterfaceDerefMut<'server, I>, SignalContext<'p>) -> Fut,
        Fut: Future<Output = Result<()>>,
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let node = self.get_node(&path).ok_or(Error::InterfaceNotFound)?;
        let conn = self.connection();
        // SAFETY: We know that there is a valid path on the node as we already converted w/o error.
        let ctxt = SignalContext::new(&conn, path).unwrap();

        node.with_iface_func_mut(func, ctxt).await
    }

    /// Get a reference to the interface at the given path.
    pub async fn get_interface<'p, P, I>(&self, path: P) -> Result<InterfaceDeref<'_, I>>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let node = self.get_node(&path).ok_or(Error::InterfaceNotFound)?;

        node.get_interface().await
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
    ///# use zbus::{Connection, ObjectServer, SignalContext, dbus_interface};
    ///
    /// struct MyIface(u32);
    ///
    /// #[dbus_interface(name = "org.myiface.MyIface")]
    /// impl MyIface {
    ///    #[dbus_interface(property)]
    ///    async fn count(&self) -> u32 {
    ///        self.0
    ///    }
    /// }
    ///
    ///# block_on(async {
    /// // Setup connection and object_server etc here and then in another part of the code:
    ///# let connection = Connection::session().await?;
    ///#
    ///# let path = "/org/zbus/path";
    ///# connection.object_server_mut().await.at(path, MyIface(22))?;
    /// let mut object_server = connection.object_server().await;
    /// let mut iface = object_server.get_interface_mut::<_, MyIface>(path).await?;
    /// // Note: This will not be needed when using `ObjectServer::with_mut`
    /// let ctxt = SignalContext::new(&connection, path)?;
    /// iface.0 = 42;
    /// iface.count_changed(&ctxt).await?;
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    ///# })?;
    ///#
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub async fn get_interface_mut<'p, P, I>(&self, path: P) -> Result<InterfaceDerefMut<'_, I>>
    where
        I: Interface,
        P: TryInto<ObjectPath<'p>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let node = self.get_node(&path).ok_or(Error::InterfaceNotFound)?;

        node.get_interface_mut().await
    }

    async fn dispatch_method_call_try(
        &self,
        connection: &Connection,
        msg_header: &MessageHeader<'_>,
        msg: &Message,
    ) -> fdo::Result<Result<u32>> {
        let path = msg_header
            .path()
            .ok()
            .flatten()
            .ok_or_else(|| fdo::Error::Failed("Missing object path".into()))?;
        let iface = msg_header
            .interface()
            .ok()
            .flatten()
            // TODO: In the absence of an INTERFACE field, if two or more interfaces on the same object
            // have a method with the same name, it is undefined which of those methods will be
            // invoked. Implementations may choose to either return an error, or deliver the message
            // as though it had an arbitrary one of those interfaces.
            .ok_or_else(|| fdo::Error::Failed("Missing interface".into()))?;
        let member = msg_header
            .member()
            .ok()
            .flatten()
            .ok_or_else(|| fdo::Error::Failed("Missing member".into()))?;

        let node = self
            .get_node(path)
            .ok_or_else(|| fdo::Error::UnknownObject(format!("Unknown object '{}'", path)))?;
        let iface = node.interface_lock(iface.clone()).ok_or_else(|| {
            fdo::Error::UnknownInterface(format!("Unknown interface '{}'", iface))
        })?;

        let read_lock = iface.read().await;
        match read_lock.call(self, connection, msg, member.clone()) {
            DispatchResult::MethodNotFound => {
                return Err(fdo::Error::UnknownMethod(format!(
                    "Unknown method '{}'",
                    member
                )));
            }
            DispatchResult::Async(f) => {
                return Ok(f.await);
            }
            DispatchResult::RequiresMut => {}
        }
        drop(read_lock);
        let mut write_lock = iface.write().await;
        match write_lock.call_mut(self, connection, msg, member.clone()) {
            DispatchResult::MethodNotFound => {}
            DispatchResult::RequiresMut => {}
            DispatchResult::Async(f) => {
                return Ok(f.await);
            }
        }
        drop(write_lock);
        Err(fdo::Error::UnknownMethod(format!(
            "Unknown method '{}'",
            member
        )))
    }

    async fn dispatch_method_call(
        &self,
        connection: &Connection,
        msg_header: &MessageHeader<'_>,
        msg: &Message,
    ) -> Result<u32> {
        match self
            .dispatch_method_call_try(connection, msg_header, msg)
            .await
        {
            Err(e) => e.reply(connection, msg).await,
            Ok(r) => r,
        }
    }

    /// Dispatch an incoming message to a registered interface.
    ///
    /// The object server will handle the message by:
    ///
    /// - looking up the called object path & interface,
    ///
    /// - calling the associated method if one exists,
    ///
    /// - returning a message (responding to the caller with either a return or error message) to
    ///   the caller through the associated server connection.
    ///
    /// Returns an error if the message is malformed, true if it's handled, false otherwise.
    pub(crate) async fn dispatch_message(&self, msg: &Message) -> Result<bool> {
        let msg_header = msg.header()?;

        match msg_header.message_type()? {
            MessageType::MethodCall => {
                let conn = self.connection();
                self.dispatch_method_call(&conn, &msg_header, msg).await?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    pub(crate) fn connection(&self) -> Connection {
        self.conn
            .upgrade()
            .expect("ObjectServer can't exist w/o an associated Connection")
    }
}

impl From<crate::blocking::ObjectServer> for ObjectServer {
    fn from(server: crate::blocking::ObjectServer) -> Self {
        server.into_inner()
    }
}

#[cfg(test)]
#[allow(clippy::blacklisted_name)]
mod tests {
    use std::{
        collections::HashMap,
        convert::TryInto,
        error::Error,
        sync::{
            mpsc::{channel, sync_channel, Sender, SyncSender},
            Arc,
        },
        thread,
    };

    use async_io::block_on;
    use event_listener::Event;
    use ntest::timeout;
    use serde::{Deserialize, Serialize};
    use test_env_log::test;
    use zbus::DBusError;
    use zvariant::derive::Type;

    use crate::{
        blocking, dbus_interface, dbus_proxy, Connection, InterfaceDeref, MessageHeader,
        MessageType, SignalContext,
    };

    #[derive(Deserialize, Serialize, Type)]
    pub struct ArgStructTest {
        foo: i32,
        bar: String,
    }

    #[dbus_proxy]
    trait MyIface {
        fn ping(&self) -> zbus::Result<u32>;

        fn quit(&self) -> zbus::Result<()>;

        fn test_header(&self) -> zbus::Result<()>;

        fn test_error(&self) -> zbus::Result<()>;

        fn test_single_struct_arg(&self, arg: ArgStructTest) -> zbus::Result<()>;

        fn test_single_struct_ret(&self) -> zbus::Result<ArgStructTest>;

        fn test_multi_ret(&self) -> zbus::Result<(i32, String)>;

        fn test_hashmap_return(&self) -> zbus::Result<HashMap<String, String>>;

        fn create_obj(&self, key: &str) -> zbus::Result<()>;

        fn destroy_obj(&self, key: &str) -> zbus::Result<()>;

        #[dbus_proxy(property)]
        fn count(&self) -> zbus::Result<u32>;

        #[dbus_proxy(property)]
        fn set_count(&self, count: u32) -> zbus::Result<()>;

        #[dbus_proxy(property)]
        fn hash_map(&self) -> zbus::Result<HashMap<String, String>>;
    }

    #[derive(Debug, Clone)]
    enum NextAction {
        Quit,
        CreateObj(String),
        DestroyObj(String),
    }

    struct MyIfaceImpl {
        next_tx: SyncSender<NextAction>,
        count: u32,
    }

    impl MyIfaceImpl {
        fn new(next_tx: SyncSender<NextAction>) -> Self {
            Self { next_tx, count: 0 }
        }
    }

    /// Custom D-Bus error type.
    #[derive(Debug, DBusError)]
    #[dbus_error(prefix = "org.freedesktop.MyIface.Error")]
    enum MyIfaceError {
        SomethingWentWrong(String),
        ZBus(zbus::Error),
    }

    #[dbus_interface(interface = "org.freedesktop.MyIface")]
    impl MyIfaceImpl {
        async fn ping(&mut self, #[zbus(signal_context)] ctxt: SignalContext<'_>) -> u32 {
            self.count += 1;
            if self.count % 3 == 0 {
                MyIfaceImpl::alert_count(&ctxt, self.count)
                    .await
                    .expect("Failed to emit signal");
            }
            self.count
        }

        fn quit(&self) {
            self.next_tx.send(NextAction::Quit).unwrap();
        }

        fn test_header(&self, #[zbus(header)] header: MessageHeader<'_>) {
            assert_eq!(header.message_type().unwrap(), MessageType::MethodCall);
            assert_eq!(header.member().unwrap().unwrap(), "TestHeader");
        }

        fn test_error(&self) -> zbus::fdo::Result<()> {
            Err(zbus::fdo::Error::Failed("error raised".to_string()))
        }

        fn test_custom_error(&self) -> Result<(), MyIfaceError> {
            Err(MyIfaceError::SomethingWentWrong("oops".to_string()))
        }

        fn test_single_struct_arg(
            &self,
            arg: ArgStructTest,
            #[zbus(header)] header: MessageHeader<'_>,
        ) -> zbus::fdo::Result<()> {
            assert_eq!(header.signature()?.unwrap(), "(is)");
            assert_eq!(arg.foo, 1);
            assert_eq!(arg.bar, "TestString");

            Ok(())
        }

        // This attribute is a noop but add to ensure user specifying it doesn't break anything.
        #[dbus_interface(struct_return)]
        fn test_single_struct_ret(&self) -> zbus::fdo::Result<ArgStructTest> {
            Ok(ArgStructTest {
                foo: 42,
                bar: String::from("Meaning of life"),
            })
        }

        #[dbus_interface(out_args("foo", "bar"))]
        fn test_multi_ret(&self) -> zbus::fdo::Result<(i32, String)> {
            Ok((42, String::from("Meaning of life")))
        }

        async fn test_hashmap_return(&self) -> zbus::fdo::Result<HashMap<String, String>> {
            let mut map = HashMap::new();
            map.insert("hi".into(), "hello".into());
            map.insert("bye".into(), "now".into());

            Ok(map)
        }

        fn create_obj(&self, key: String) {
            self.next_tx.send(NextAction::CreateObj(key)).unwrap();
        }

        fn destroy_obj(&self, key: String) {
            self.next_tx.send(NextAction::DestroyObj(key)).unwrap();
        }

        #[dbus_interface(property)]
        fn set_count(&mut self, val: u32) -> zbus::fdo::Result<()> {
            if val == 42 {
                return Err(zbus::fdo::Error::InvalidArgs("Tsss tsss!".to_string()));
            }
            self.count = val;
            Ok(())
        }

        #[dbus_interface(property)]
        fn count(&self) -> u32 {
            self.count
        }

        #[dbus_interface(property)]
        async fn hash_map(&self) -> HashMap<String, String> {
            self.test_hashmap_return().await.unwrap()
        }

        #[dbus_interface(signal)]
        async fn alert_count(ctxt: &SignalContext<'_>, val: u32) -> zbus::Result<()>;
    }

    fn check_hash_map(map: HashMap<String, String>) {
        assert_eq!(map["hi"], "hello");
        assert_eq!(map["bye"], "now");
    }

    fn my_iface_test(tx: Sender<()>) -> std::result::Result<u32, Box<dyn Error>> {
        let conn = blocking::Connection::session()?;
        let proxy = MyIfaceProxy::builder(&conn)
            .destination("org.freedesktop.MyService")?
            .path("/org/freedesktop/MyService")?
            // the server isn't yet running
            .cache_properties(false)
            .build()?;
        let props_proxy = zbus::fdo::PropertiesProxy::builder(&conn)
            .destination("org.freedesktop.MyService")?
            .path("/org/freedesktop/MyService")?
            .build()?;

        let prop_changed = Arc::new(Event::new());
        let prop_changed_listener = prop_changed.listen();
        props_proxy
            .connect_properties_changed(move |_, changed, _| {
                let (name, _) = changed.iter().next().unwrap();
                assert_eq!(*name, "Count");
                prop_changed.notify(1);
            })
            .unwrap();
        tx.send(()).unwrap();

        prop_changed_listener.wait();

        proxy.ping()?;
        assert_eq!(proxy.count()?, 1);
        proxy.test_header()?;
        proxy.test_single_struct_arg(ArgStructTest {
            foo: 1,
            bar: "TestString".into(),
        })?;
        check_hash_map(proxy.test_hashmap_return()?);
        check_hash_map(proxy.hash_map()?);
        #[cfg(feature = "xml")]
        {
            let xml = proxy.introspect()?;
            let node = crate::xml::Node::from_reader(xml.as_bytes())?;
            let ifaces = node.interfaces();
            let iface = ifaces
                .iter()
                .find(|i| i.name() == "org.freedesktop.MyIface")
                .unwrap();
            let methods = iface.methods();
            for method in methods {
                if method.name() != "TestSingleStructRet" && method.name() != "TestMultiRet" {
                    continue;
                }
                let args = method.args();
                let mut out_args = args.iter().filter(|a| a.direction().unwrap() == "out");

                if method.name() == "TestSingleStructRet" {
                    assert_eq!(args.len(), 1);
                    assert_eq!(out_args.next().unwrap().ty(), "(is)");
                    assert!(out_args.next().is_none());
                } else {
                    assert_eq!(args.len(), 2);
                    let foo = out_args.find(|a| a.name() == Some("foo")).unwrap();
                    assert_eq!(foo.ty(), "i");
                    let bar = out_args.find(|a| a.name() == Some("bar")).unwrap();
                    assert_eq!(bar.ty(), "s");
                }
            }
        }
        // build-time check to see if macro is doing the right thing.
        let _ = proxy.test_single_struct_ret()?.foo;
        let _ = proxy.test_multi_ret()?.1;

        let val = proxy.ping()?;

        proxy.create_obj("MyObj")?;
        let my_obj_proxy = MyIfaceProxy::builder(&conn)
            .destination("org.freedesktop.MyService")?
            .path("/zbus/test/MyObj")?
            .build()?;
        my_obj_proxy.ping()?;
        proxy.destroy_obj("MyObj")?;
        assert!(my_obj_proxy.introspect().is_err());
        assert!(my_obj_proxy.ping().is_err());

        proxy.quit()?;
        Ok(val)
    }

    #[test]
    #[timeout(15000)]
    fn basic_iface() {
        block_on(basic_iface_());
    }

    async fn basic_iface_() {
        let (tx, rx) = channel::<()>();

        let conn = Connection::session()
            .await
            .unwrap()
            // primary name
            .request_name("org.freedesktop.MyService")
            .await
            .unwrap()
            .request_name("org.freedesktop.MyService.foo")
            .await
            .unwrap()
            .request_name("org.freedesktop.MyService.bar")
            .await
            .unwrap();

        let child = thread::spawn(move || my_iface_test(tx).expect("child failed"));
        // Wait for the listener to be ready
        rx.recv().unwrap();

        let (next_tx, next_rx) = sync_channel(64);
        let iface = MyIfaceImpl::new(next_tx.clone());
        {
            let mut server = conn.object_server_mut().await;
            server.at("/org/freedesktop/MyService", iface).unwrap();

            server
                .with(
                    "/org/freedesktop/MyService",
                    |iface: InterfaceDeref<'_, MyIfaceImpl>, ctxt| async move {
                        iface.count_changed(&ctxt).await
                    },
                )
                .await
                .unwrap();
        }

        loop {
            conn.object_server_mut()
                .await
                .with(
                    "/org/freedesktop/MyService",
                    |_iface: InterfaceDeref<'_, MyIfaceImpl>, ctxt| async move {
                        MyIfaceImpl::alert_count(&ctxt, 51).await
                    },
                )
                .await
                .unwrap();

            match next_rx.recv().unwrap() {
                NextAction::Quit => break,
                NextAction::CreateObj(key) => {
                    let path = format!("/zbus/test/{}", key);
                    conn.object_server_mut()
                        .await
                        .at(path, MyIfaceImpl::new(next_tx.clone()))
                        .unwrap();
                }
                NextAction::DestroyObj(key) => {
                    let path = format!("/zbus/test/{}", key);
                    conn.object_server_mut()
                        .await
                        .remove::<MyIfaceImpl, _>(path)
                        .unwrap();
                }
            }
        }

        // FIXME: This should ideally be done in asnyc/await way.
        let val = child.join().expect("failed to join");
        assert_eq!(val, 2);

        // Release primary name explicitly and let others be released implicitly.
        assert_eq!(
            conn.release_name("org.freedesktop.MyService").await,
            Ok(true)
        );
        assert_eq!(
            conn.release_name("org.freedesktop.MyService.foo").await,
            Ok(true)
        );
        assert_eq!(
            conn.release_name("org.freedesktop.MyService.bar").await,
            Ok(true)
        );

        // Let's ensure all names were released.
        let proxy = zbus::fdo::AsyncDBusProxy::new(&conn).await.unwrap();
        assert_eq!(
            proxy
                .name_has_owner("org.freedesktop.MyService".try_into().unwrap())
                .await,
            Ok(false)
        );
        assert_eq!(
            proxy
                .name_has_owner("org.freedesktop.MyService.foo".try_into().unwrap())
                .await,
            Ok(false)
        );
        assert_eq!(
            proxy
                .name_has_owner("org.freedesktop.MyService.bar".try_into().unwrap())
                .await,
            Ok(false)
        );
    }
}
