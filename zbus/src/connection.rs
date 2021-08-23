use futures_util::StreamExt;
use static_assertions::assert_impl_all;
use std::{
    convert::TryInto,
    io::{self, ErrorKind},
    os::unix::io::{AsRawFd, RawFd},
    sync::{Arc, Mutex},
};
use zbus_names::{BusName, ErrorName, InterfaceName, MemberName, OwnedUniqueName};
use zvariant::ObjectPath;

use async_io::block_on;

use crate::{
    azync::{self, MessageStream},
    Error, Message, Result,
};

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
/// Typically, a connection is made to the session bus with [`session`], or to the system bus
/// with [`system`]. Then the connection is shared with the [`Proxy`] and [`ObjectServer`]
/// instances.
///
/// `Connection` implements [`Clone`] and cloning it is a very cheap operation, as the underlying
/// data is not cloned. This makes it very convenient to share the connection between different
/// parts of your code. `Connection` also implements [`std::marker::Sync`] and[`std::marker::Send`]
/// so you can send and share a connection instance across threads as well.
///
/// `Connection` keeps an internal ringbuffer of incoming message. The maximum capacity of this
/// ringbuffer is configurable through the [`set_max_queued`] method. The default size is 64. When
/// the buffer is full, messages are dropped to create room, starting from the oldest one.
///
/// [method calls]: struct.Connection.html#method.call_method
/// [signals]: struct.Connection.html#method.emit_signal
/// [`system`]: struct.Connection.html#method.new_system
/// [`session`]: struct.Connection.html#method.new_session
/// [`Proxy`]: struct.Proxy.html
/// [`ObjectServer`]: struct.ObjectServer.html
/// [`dbus_proxy`]: attr.dbus_proxy.html
/// [`dbus_interface`]: attr.dbus_interface.html
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [file an issue]: https://gitlab.freedesktop.org/dbus/zbus/-/issues/new
/// [`set_max_queued`]: struct.Connection.html#method.set_max_queued
#[derive(derivative::Derivative, Clone)]
#[derivative(Debug)]
pub struct Connection {
    inner: azync::Connection,
    #[derivative(Debug = "ignore")]
    stream: Arc<Mutex<azync::MessageStream>>,
}

assert_impl_all!(Connection: Send, Sync, Unpin);

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        block_on(self.inner.as_raw_fd())
    }
}

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

    /// Fetch the next message from the connection.
    ///
    /// Read from the connection until a message is received or an error is reached. Return the
    /// message on success.
    pub fn receive_message(&self) -> Result<Arc<Message>> {
        let mut stream = self.stream.lock().expect("lock poisoned");
        block_on(stream.next())
            .ok_or_else(|| Error::Io(io::Error::new(ErrorKind::BrokenPipe, "socket closed")))?
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

    /// Checks if `self` is a connection to a message bus.
    ///
    /// This will return `false` for p2p connections.
    pub fn is_bus(&self) -> bool {
        self.inner.is_bus()
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
        let stream = Arc::new(Mutex::new(MessageStream::from(&conn)));

        Self {
            inner: conn,
            stream,
        }
    }
}

#[cfg(test)]
mod tests {
    use ntest::timeout;
    use std::{os::unix::net::UnixStream, thread};
    use test_env_log::test;

    use crate::{ConnectionBuilder, Guid, MessageStream};
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
