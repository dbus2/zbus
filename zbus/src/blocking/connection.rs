use static_assertions::assert_impl_all;
use std::{
    convert::TryInto,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use zbus_names::{BusName, ErrorName, InterfaceName, MemberName, OwnedUniqueName, WellKnownName};
use zvariant::ObjectPath;

use async_io::block_on;

use crate::{azync, blocking::ObjectServer, Error, Message, Result};

/// A D-Bus connection.
///
/// A connection to a D-Bus bus, or a direct peer.
///
/// Once created, the connection is authenticated and negotiated and messages can be sent or
/// received, such as [method calls] or [signals].
///
/// For higher-level message handling (typed functions, introspection, documentation reasons etc),
/// it is recommended to wrap the low-level D-Bus messages into Rust functions with the
/// [`dbus_proxy`] and [`dbus_interface`] macros instead of doing it directly on a `Connection`.
///
/// Typically, a connection is made to the session bus with [`Connection::session`], or to the
/// system bus with [`Connection::system`]. Then the connection is shared with the
/// [`crate::blocking::Proxy`] and [`ObjectServer`] instances.
///
/// `Connection` implements [`Clone`] and cloning it is a very cheap operation, as the underlying
/// data is not cloned. This makes it very convenient to share the connection between different
/// parts of your code. `Connection` also implements [`std::marker::Sync`] and[`std::marker::Send`]
/// so you can send and share a connection instance across threads as well.
///
/// `Connection` keeps an internal queue of incoming message. The maximum capacity of this queue
/// is configurable through the [`set_max_queued`] method. The default size is 64. When the queue is
/// full, no more messages can be received until there is room is created for more. This is why it's
/// important to ensure that all [`crate::blocking::MessageStream`] and
/// [`crate::azync::MessageStream`] instances are continuously iterated on and polled, respectively.
///
/// [method calls]: blocking/struct.Connection.html#method.call_method
/// [signals]: blocking/struct.Connection.html#method.emit_signal
/// [`dbus_proxy`]: attr.dbus_proxy.html
/// [`dbus_interface`]: attr.dbus_interface.html
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [file an issue]: https://gitlab.freedesktop.org/dbus/zbus/-/issues/new
/// [`set_max_queued`]: struct.Connection.html#method.set_max_queued
#[derive(derivative::Derivative, Clone)]
#[derivative(Debug)]
pub struct Connection {
    inner: azync::Connection,
}

assert_impl_all!(Connection: Send, Sync, Unpin);

impl Connection {
    /// Create a `Connection` to the session/user message bus.
    pub fn session() -> Result<Self> {
        block_on(azync::Connection::session()).map(Self::from)
    }

    /// Create a `Connection` to the system-wide message bus.
    pub fn system() -> Result<Self> {
        block_on(azync::Connection::system()).map(Self::from)
    }

    /// Max number of messages to queue.
    pub fn max_queued(&self) -> usize {
        self.inner.max_queued()
    }

    /// Set the max number of messages to queue.
    pub fn set_max_queued(mut self, max: usize) {
        self.inner.set_max_queued(max)
    }

    /// The server's GUID.
    pub fn server_guid(&self) -> &str {
        self.inner.server_guid()
    }

    /// The unique name as assigned by the message bus or `None` if not a message bus connection.
    pub fn unique_name(&self) -> Option<&OwnedUniqueName> {
        self.inner.unique_name()
    }

    /// Send `msg` to the peer.
    ///
    /// The connection sets a unique serial number on the message before sending it off.
    ///
    /// On successfully sending off `msg`, the assigned serial number is returned.
    pub fn send_message(&self, msg: Message) -> Result<u32> {
        block_on(self.inner.send_message(msg))
    }

    /// Send a method call.
    ///
    /// Create a method-call message, send it over the connection, then wait for the reply. Incoming
    /// messages are received through [`receive_message`] until the matching method reply (error or
    /// return) is received.
    ///
    /// On successful reply, an `Ok(Message)` is returned. On error, an `Err` is returned. D-Bus
    /// error replies are returned as [`MethodError`].
    ///
    /// [`receive_message`]: struct.Connection.html#method.receive_message
    /// [`MethodError`]: enum.Error.html#variant.MethodError
    pub fn call_method<'d, 'p, 'i, 'm, D, P, I, M, B>(
        &self,
        destination: Option<D>,
        path: P,
        iface: Option<I>,
        method_name: M,
        body: &B,
    ) -> Result<Arc<Message>>
    where
        D: TryInto<BusName<'d>>,
        P: TryInto<ObjectPath<'p>>,
        I: TryInto<InterfaceName<'i>>,
        M: TryInto<MemberName<'m>>,
        D::Error: Into<Error>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + zvariant::DynamicType,
    {
        block_on(
            self.inner
                .call_method(destination, path, iface, method_name, body),
        )
    }

    /// Emit a signal.
    ///
    /// Create a signal message, and send it over the connection.
    pub fn emit_signal<'d, 'p, 'i, 'm, D, P, I, M, B>(
        &self,
        destination: Option<D>,
        path: P,
        iface: I,
        signal_name: M,
        body: &B,
    ) -> Result<()>
    where
        D: TryInto<BusName<'d>>,
        P: TryInto<ObjectPath<'p>>,
        I: TryInto<InterfaceName<'i>>,
        M: TryInto<MemberName<'m>>,
        D::Error: Into<Error>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + zvariant::DynamicType,
    {
        block_on(
            self.inner
                .emit_signal(destination, path, iface, signal_name, body),
        )
    }

    /// Reply to a message.
    ///
    /// Given an existing message (likely a method call), send a reply back to the caller with the
    /// given `body`.
    ///
    /// Returns the message serial number.
    pub fn reply<B>(&self, call: &Message, body: &B) -> Result<u32>
    where
        B: serde::ser::Serialize + zvariant::DynamicType,
    {
        block_on(self.inner.reply(call, body))
    }

    /// Reply an error to a message.
    ///
    /// Given an existing message (likely a method call), send an error reply back to the caller
    /// with the given `error_name` and `body`.
    ///
    /// Returns the message serial number.
    pub fn reply_error<'e, E, B>(&self, call: &Message, error_name: E, body: &B) -> Result<u32>
    where
        B: serde::ser::Serialize + zvariant::DynamicType,
        E: TryInto<ErrorName<'e>>,
        E::Error: Into<Error>,
    {
        block_on(self.inner.reply_error(call, error_name, body))
    }

    /// Register a well-known name for this service on the bus.
    ///
    /// You can request multiple names for the same `ObjectServer`. Use [`Connection::release_name`]
    /// for deregistering names registered through this method.
    ///
    /// Note that exclusive ownership without queueing is requested (using
    /// [`crate::fdo::RequestNameFlags::ReplaceExisting`] and
    /// [`crate::fdo::RequestNameFlags::DoNotQueue`] flags) since that is the most typical case. If
    /// that is not what you want, you should use [`crate::fdo::DBusProxy::request_name`] instead
    /// (but make sure then that name is requested **after** you've setup your service
    /// implementation with the `ObjectServer`).
    pub fn request_name<'w, W>(self, well_known_name: W) -> Result<Self>
    where
        W: TryInto<WellKnownName<'w>>,
        W::Error: Into<Error>,
    {
        block_on(self.inner.request_name(well_known_name)).map(Into::into)
    }

    /// Deregister a previously registered well-known name for this service on the bus.
    ///
    /// Use this method to deregister a well-known name, registered through
    /// [`Connection::request_name`].
    ///
    /// Unless an error is encountered, returns `Ok(true)` if name was previously registered with
    /// the bus through `self` and it has now been successfully deregistered, `Ok(fasle)` if name
    /// was not previously registered or already deregistered.
    pub fn release_name<'w, W>(&self, well_known_name: W) -> Result<bool>
    where
        W: TryInto<WellKnownName<'w>>,
        W::Error: Into<Error>,
    {
        block_on(self.inner.release_name(well_known_name))
    }

    /// Checks if `self` is a connection to a message bus.
    ///
    /// This will return `false` for p2p connections.
    pub fn is_bus(&self) -> bool {
        self.inner.is_bus()
    }

    /// Get a reference to the associated [`ObjectServer`].
    ///
    /// The `ObjectServer` is created on-demand.
    pub fn object_server(&self) -> impl Deref<Target = ObjectServer> + '_ {
        block_on(self.inner.sync_object_server())
    }

    /// Get a mutable reference to the associated [`ObjectServer`].
    ///
    /// The `ObjectServer` is created on-demand.
    ///
    /// # Caveats
    ///
    /// The return value of this method should not be kept around for longer than needed. The method
    /// dispatch machinery of the [`ObjectServer`] will be paused as long as the return value is alive.
    pub fn object_server_mut(&self) -> impl DerefMut<Target = ObjectServer> + '_ {
        block_on(self.inner.sync_object_server_mut())
    }

    /// Get a reference to the underlying async Connection.
    pub fn inner(&self) -> &azync::Connection {
        &self.inner
    }

    /// Get the underlying async Connection, consuming `self`.
    pub fn into_inner(self) -> azync::Connection {
        self.inner
    }
}

impl From<azync::Connection> for Connection {
    fn from(conn: azync::Connection) -> Self {
        Self { inner: conn }
    }
}

#[cfg(test)]
mod tests {
    use ntest::timeout;
    use std::{os::unix::net::UnixStream, thread};
    use test_env_log::test;

    use crate::{
        blocking::{ConnectionBuilder, MessageStream},
        Guid,
    };
    #[test]
    #[timeout(15000)]
    fn unix_p2p() {
        let guid = Guid::generate();

        let (p0, p1) = UnixStream::pair().unwrap();

        let server_thread = thread::spawn(move || {
            let c = ConnectionBuilder::unix_stream(p0)
                .server(&guid)
                .p2p()
                .build()
                .unwrap();
            let reply = c
                .call_method(None::<()>, "/", Some("org.zbus.p2p"), "Test", &())
                .unwrap();
            assert_eq!(reply.to_string(), "Method return");
            let val: String = reply.body().unwrap();
            val
        });

        let c = ConnectionBuilder::unix_stream(p1).p2p().build().unwrap();
        let mut s = MessageStream::from(&c);
        let m = s.next().unwrap().unwrap();
        assert_eq!(m.to_string(), "Method call Test");
        c.reply(&m, &("yay")).unwrap();

        for _ in s {}

        let val = server_thread.join().expect("failed to join server thread");
        assert_eq!(val, "yay");
    }
}
