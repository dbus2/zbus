use futures_util::future::FutureExt;
use std::{
    convert::TryInto,
    future::ready,
    os::unix::{
        io::{AsRawFd, RawFd},
        net::UnixStream,
    },
};
use zvariant::ObjectPath;

use async_io::block_on;

use crate::{azync, Guid, Message, MessageError, Result};

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
/// Typically, a connection is made to the session bus with [`new_session`], or to the system bus
/// with [`new_system`]. Then the connection is shared with the [`Proxy`] and [`ObjectServer`]
/// instances.
///
/// `Connection` implements [`Clone`] and cloning it is a very cheap operation, as the underlying
/// data is not cloned. This makes it very convenient to share the connection between different
/// parts of your code. `Connection` also implements [`std::marker::Sync`] and[`std::marker::Send`]
/// so you can send and share a connection instance across threads as well.
///
/// Since there are times when important messages arrive between a method call message is sent and
/// its reply is received, `Connection` keeps an internal queue of incoming messages so that these
/// messages are not lost and subsequent calls to [`receive_message`] will retreive messages from
/// this queue first. The size of this queue is configurable through the [`set_max_queued`] method.
/// The default size is 64. When the queue is full, messages are dropped to create room, starting
/// from the oldest one.
///
/// [method calls]: struct.Connection.html#method.call_method
/// [signals]: struct.Connection.html#method.emit_signal
/// [`new_system`]: struct.Connection.html#method.new_system
/// [`new_session`]: struct.Connection.html#method.new_session
/// [`Proxy`]: struct.Proxy.html
/// [`ObjectServer`]: struct.ObjectServer.html
/// [`dbus_proxy`]: attr.dbus_proxy.html
/// [`dbus_interface`]: attr.dbus_interface.html
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [file an issue]: https://gitlab.freedesktop.org/dbus/zbus/-/issues/new
/// [`receive_message`]: struct.Connection.html#method.receive_message
/// [`set_max_queued`]: struct.Connection.html#method.set_max_queued
#[derive(Debug, Clone)]
pub struct Connection(azync::Connection);

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        block_on(self.0.as_raw_fd())
    }
}

impl Connection {
    /// Create and open a D-Bus connection from a `UnixStream`.
    ///
    /// The connection may either be set up for a *bus* connection, or not (for peer-to-peer
    /// communications).
    ///
    /// Upon successful return, the connection is fully established and negotiated: D-Bus messages
    /// can be sent and received.
    pub fn new_unix_client(stream: UnixStream, bus_connection: bool) -> Result<Self> {
        block_on(azync::Connection::new_unix_client(stream, bus_connection)).map(Self)
    }

    /// Create a `Connection` to the session/user message bus.
    pub fn new_session() -> Result<Self> {
        block_on(azync::Connection::new_session()).map(Self)
    }

    /// Create a `Connection` to the system-wide message bus.
    pub fn new_system() -> Result<Self> {
        block_on(azync::Connection::new_system()).map(Self)
    }

    /// Create a `Connection` for the given [D-Bus address].
    ///
    /// [D-Bus address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub fn new_for_address(address: &str, bus_connection: bool) -> Result<Self> {
        block_on(azync::Connection::new_for_address(address, bus_connection)).map(Self)
    }

    /// Create a server `Connection` for the given `UnixStream` and the server `guid`.
    ///
    /// The connection will wait for incoming client authentication handshake & negotiation messages,
    /// for peer-to-peer communications.
    ///
    /// Upon successful return, the connection is fully established and negotiated: D-Bus messages
    /// can be sent and received.
    pub fn new_unix_server(stream: UnixStream, guid: &Guid) -> Result<Self> {
        block_on(azync::Connection::new_unix_server(stream, guid)).map(Self)
    }

    /// Max number of messages to queue.
    pub fn max_queued(&self) -> usize {
        self.0.max_queued()
    }

    /// Set the max number of messages to queue.
    ///
    /// Since typically you'd want to set this at instantiation time, this method takes ownership
    /// of `self` and returns an owned `Connection` instance so you can use the builder pattern to
    /// set the value.
    ///
    /// # Example
    ///
    /// ```
    ///# use std::error::Error;
    ///#
    /// let conn = zbus::Connection::new_session()?.set_max_queued(30);
    /// assert_eq!(conn.max_queued(), 30);
    ///
    /// // Do something usefull with `conn`..
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn set_max_queued(self, max: usize) -> Self {
        Self(self.0.set_max_queued(max))
    }

    /// The server's GUID.
    pub fn server_guid(&self) -> &str {
        self.0.server_guid()
    }

    /// The unique name as assigned by the message bus or `None` if not a message bus connection.
    pub fn unique_name(&self) -> Option<&str> {
        self.0.unique_name()
    }

    /// Fetch the next message from the connection.
    ///
    /// Read from the connection until a message is received or an error is reached. Return the
    /// message on success. If there are pending messages in the queue, the first one from the queue
    /// is returned instead of attempting to read the connection.
    ///
    /// # Warning
    ///
    /// If you use this method in combination with [`Self::receive_specific`] or
    /// [`Proxy`](crate::Proxy) API on the same connection from multiple threads, you can end up
    /// with situation where this method takes away the message the other API is awaiting for and
    /// end up in a deadlock situation. It is therefore highly recommended not to use such a
    /// combination.
    pub fn receive_message(&self) -> Result<Message> {
        block_on(self.0.receive_specific(|_| ready(Ok(true)).boxed()))
    }

    /// Receive a specific message.
    ///
    /// This is the same as [`Self::receive_message`], except that it takes a predicate function that
    /// decides if the message received should be returned by this method or not. Message received
    /// during this call that are not returned by it, are pushed to the queue to be picked by the
    /// susubsequent call to `receive_message`] or this method.
    pub fn receive_specific<P>(&self, predicate: P) -> Result<Message>
    where
        P: Fn(&Message) -> Result<bool>,
    {
        block_on(self.0.receive_specific(|msg| ready(predicate(msg)).boxed()))
    }

    /// Send `msg` to the peer.
    ///
    /// The connection sets a unique serial number on the message before sending it off.
    ///
    /// On successfully sending off `msg`, the assigned serial number is returned.
    ///
    /// **Note:** if this connection is in non-blocking mode, the message may not actually
    /// have been sent when this method returns, and you need to call the [`flush`] method
    /// so that pending messages are written to the socket.
    ///
    /// [`flush`]: struct.Connection.html#method.flush
    pub fn send_message(&self, msg: Message) -> Result<u32> {
        block_on(self.0.send_message(msg))
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
    /// *Note:* This method will block until the response is received even if the connection is
    /// in non-blocking mode. If you don't want to block like this, use [`Connection::send_message`].
    ///
    /// [`receive_message`]: struct.Connection.html#method.receive_message
    /// [`MethodError`]: enum.Error.html#variant.MethodError
    /// [`sent_message`]: struct.Connection.html#method.send_message
    pub fn call_method<'p, B, E>(
        &self,
        destination: Option<&str>,
        path: impl TryInto<ObjectPath<'p>, Error = E>,
        iface: Option<&str>,
        method_name: &str,
        body: &B,
    ) -> Result<Message>
    where
        B: serde::ser::Serialize + zvariant::Type,
        MessageError: From<E>,
    {
        block_on(
            self.0
                .call_method(destination, path, iface, method_name, body),
        )
    }

    /// Emit a signal.
    ///
    /// Create a signal message, and send it over the connection.
    pub fn emit_signal<'p, B, E>(
        &self,
        destination: Option<&str>,
        path: impl TryInto<ObjectPath<'p>, Error = E>,
        iface: &str,
        signal_name: &str,
        body: &B,
    ) -> Result<()>
    where
        B: serde::ser::Serialize + zvariant::Type,
        MessageError: From<E>,
    {
        block_on(
            self.0
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
        B: serde::ser::Serialize + zvariant::Type,
    {
        block_on(self.0.reply(call, body))
    }

    /// Reply an error to a message.
    ///
    /// Given an existing message (likely a method call), send an error reply back to the caller
    /// with the given `error_name` and `body`.
    ///
    /// Returns the message serial number.
    pub fn reply_error<B>(&self, call: &Message, error_name: &str, body: &B) -> Result<u32>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        block_on(self.0.reply_error(call, error_name, body))
    }

    /// Checks if `self` is a connection to a message bus.
    ///
    /// This will return `false` for p2p connections.
    pub fn is_bus(&self) -> bool {
        self.0.is_bus()
    }

    /// Get a reference to the underlying async Connection.
    pub fn inner(&self) -> &azync::Connection {
        &self.0
    }

    /// Get the underlying async Connection, consuming `self`.
    pub fn into_inner(self) -> azync::Connection {
        self.0
    }
}

impl From<azync::Connection> for Connection {
    fn from(conn: azync::Connection) -> Self {
        Self(conn)
    }
}

#[cfg(test)]
mod tests {
    use std::{os::unix::net::UnixStream, thread};
    use test_env_log::test;

    use crate::{Connection, Error, Guid};
    #[test]
    fn unix_p2p() {
        let guid = Guid::generate();

        let (p0, p1) = UnixStream::pair().unwrap();

        let server_thread = thread::spawn(move || {
            let c = Connection::new_unix_server(p0, &guid).unwrap();
            let reply = c
                .call_method(None, "/", Some("org.zbus.p2p"), "Test", &())
                .unwrap();
            assert_eq!(reply.to_string(), "Method return");
            let val: String = reply.body().unwrap();
            val
        });

        let c = Connection::new_unix_client(p1, false).unwrap();
        let m = c.receive_message().unwrap();
        assert_eq!(m.to_string(), "Method call Test");
        c.reply(&m, &("yay")).unwrap();

        assert!(matches!(c.receive_message().unwrap_err(), Error::Io(_)));

        let val = server_thread.join().expect("failed to join server thread");
        assert_eq!(val, "yay");
    }
}
