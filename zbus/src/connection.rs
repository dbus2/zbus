use std::{
    os::unix::{
        io::{AsRawFd, RawFd},
        net::UnixStream,
    },
    sync::{Arc, Mutex, RwLock},
};

use nix::poll::PollFlags;
use once_cell::sync::OnceCell;

use crate::{
    fdo,
    handshake::{ClientHandshake, Handshake, ServerHandshake},
    raw::{Connection as RawConnection, Socket},
    utils::wait_on,
    Error, Guid, Message, MessageType, Result,
};

pub(crate) const DEFAULT_MAX_QUEUED: usize = 32;
const LOCK_FAIL_MSG: &str = "Failed to lock a mutex or read-write lock";

#[derive(Debug)]
struct ConnectionInner<S> {
    server_guid: Guid,
    cap_unix_fd: bool,
    bus_conn: bool,
    unique_name: OnceCell<String>,

    raw_in_conn: Mutex<RawConnection<S>>,
    raw_out_conn: Mutex<RawConnection<S>>,
    // Serial number for next outgoing message
    serial: Mutex<u32>,

    // Queue of incoming messages
    incoming_queue: Mutex<Vec<Message>>,

    // Max number of messages to queue
    max_queued: RwLock<usize>,
}

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
/// The default size is 32. All messages that are received after the queue is full, are dropped.
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
/// [file an issue]: https://gitlab.freedesktop.org/zeenix/zbus/-/issues/new
/// [`receive_message`]: struct.Connection.html#method.receive_message
/// [`set_max_queued`]: struct.Connection.html#method.set_max_queued
#[derive(Debug, Clone)]
pub struct Connection(Arc<ConnectionInner<Box<dyn Socket>>>);

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        (**self.0.raw_in_conn.lock().expect(LOCK_FAIL_MSG).socket()).as_raw_fd()
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
        Self::new(
            ClientHandshake::new(Box::new(stream) as Box<dyn Socket>),
            bus_connection,
        )
    }

    /// Create a `Connection` to the session/user message bus.
    pub fn new_session() -> Result<Self> {
        Self::new(ClientHandshake::new_session()?, true)
    }

    /// Create a `Connection` to the system-wide message bus.
    pub fn new_system() -> Result<Self> {
        Self::new(ClientHandshake::new_system()?, true)
    }

    /// Create a `Connection` for the given [D-Bus address].
    ///
    /// [D-Bus address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub fn new_for_address(address: &str, bus_connection: bool) -> Result<Self> {
        Self::new(ClientHandshake::new_for_address(address)?, bus_connection)
    }

    /// Create a server `Connection` for the given `UnixStream` and the server `guid`.
    ///
    /// The connection will wait for incoming client authentication handshake & negotiation messages,
    /// for peer-to-peer communications.
    ///
    /// Upon successful return, the connection is fully established and negotiated: D-Bus messages
    /// can be sent and received.
    pub fn new_unix_server(stream: UnixStream, guid: &Guid) -> Result<Self> {
        use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

        let creds = getsockopt(stream.as_raw_fd(), PeerCredentials)
            .map_err(|e| Error::Handshake(format!("Failed to get peer credentials: {}", e)))?;

        Self::new(
            ServerHandshake::new(
                Box::new(stream) as Box<dyn Socket>,
                guid.clone(),
                creds.uid(),
            ),
            false,
        )
    }

    /// Max number of messages to queue.
    pub fn max_queued(&self) -> usize {
        *self.0.max_queued.read().expect(LOCK_FAIL_MSG)
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
        *self.0.max_queued.write().expect(LOCK_FAIL_MSG) = max;

        self
    }

    /// The server's GUID.
    pub fn server_guid(&self) -> &str {
        self.0.server_guid.as_str()
    }

    /// The unique name as assigned by the message bus or `None` if not a message bus connection.
    pub fn unique_name(&self) -> Option<&str> {
        self.0.unique_name.get().map(|s| s.as_str())
    }

    /// Fetch the next message from the connection.
    ///
    /// Read from the connection until a message is received or an error is reached. Return the
    /// message on success. If the connection is in non-blocking mode, this will return a
    /// `WouldBlock` error instead of blocking. If there are pending messages in the queue, the
    /// first one from the queue is returned instead of attempting to read the connection.
    ///
    /// # Warning
    ///
    /// If you use this method in combination with [`Self::receive_specific`] or
    /// [`Proxy`](crate::Proxy) API on the same connection from multiple threads, you can end up
    /// with situation where this method takes away the message the other API is awaiting for and
    /// end up in a deadlock situation. It is therefore highly recommended not to use such a
    /// combination.
    pub fn receive_message(&self) -> Result<Message> {
        loop {
            let mut queue = self.0.incoming_queue.lock().expect(LOCK_FAIL_MSG);
            if let Some(msg) = queue.pop() {
                return Ok(msg);
            }

            if let Some(msg) = self.receive_message_raw()? {
                return Ok(msg);
            }
        }
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
        loop {
            let mut queue = self.0.incoming_queue.lock().expect(LOCK_FAIL_MSG);
            for (i, msg) in queue.iter().enumerate() {
                if predicate(msg)? {
                    return Ok(queue.remove(i));
                }
            }

            let msg = match self.receive_message_raw()? {
                Some(msg) => msg,
                None => continue,
            };

            if predicate(&msg)? {
                return Ok(msg);
            } else if queue.len() < *self.0.max_queued.read().expect(LOCK_FAIL_MSG) {
                queue.push(msg);
            }
        }
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
    pub fn send_message(&self, mut msg: Message) -> Result<u32> {
        if !msg.fds().is_empty() && !self.0.cap_unix_fd {
            return Err(Error::Unsupported);
        }

        let serial = self.next_serial();
        msg.modify_primary_header(|primary| {
            primary.set_serial_num(serial);

            Ok(())
        })?;

        let mut conn = self.0.raw_out_conn.lock().expect(LOCK_FAIL_MSG);
        conn.enqueue_message(msg);
        // Swallow a potential WouldBLock error, but propagate the others
        if let Err(e) = conn.try_flush() {
            if e.kind() != std::io::ErrorKind::WouldBlock {
                return Err(e.into());
            }
        }

        Ok(serial)
    }

    /// Flush pending outgoing messages to the server
    ///
    /// This method is only useful if the connection is in non-blocking mode. It will
    /// write as many pending outgoing messages as possible to the socket.
    ///
    /// It will return `Ok(())` if all messages could be sent, and error otherwise. A
    /// `WouldBlock` error means that the internal buffer of the connection transport is
    /// full, and you need to wait for write-readiness before calling this method again.
    ///
    /// If the connection is in blocking mode, this will return `Ok(())` and do nothing.
    pub fn flush(&self) -> Result<()> {
        self.0
            .raw_out_conn
            .lock()
            .expect(LOCK_FAIL_MSG)
            .try_flush()?;
        Ok(())
    }

    /// Send a method call.
    ///
    /// Create a method-call message, send it over the connection, then wait for the reply. Incoming
    /// messages are received through [`receive_message`] until the matching method reply (error or
    /// return) is received.
    ///
    /// On succesful reply, an `Ok(Message)` is returned. On error, an `Err` is returned. D-Bus
    /// error replies are returned as [`MethodError`].
    ///
    /// *Note:* This method will block until the response is received even if the connection is
    /// in non-blocking mode. If you don't want to block like this, use [`Connection::send_message`].
    ///
    /// [`receive_message`]: struct.Connection.html#method.receive_message
    /// [`MethodError`]: enum.Error.html#variant.MethodError
    /// [`sent_message`]: struct.Connection.html#method.send_message
    pub fn call_method<B>(
        &self,
        destination: Option<&str>,
        path: &str,
        iface: Option<&str>,
        method_name: &str,
        body: &B,
    ) -> Result<Message>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::method(
            self.unique_name(),
            destination,
            path,
            iface,
            method_name,
            body,
        )?;

        let serial = self.send_message(m)?;
        // loop & sleep until the message is completely sent
        loop {
            match self.flush() {
                Ok(()) => break,
                Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    wait_on(self.as_raw_fd(), PollFlags::POLLOUT)?;
                }
                Err(e) => return Err(e),
            }
        }

        loop {
            match self.receive_specific(|m| {
                let h = m.header()?;

                Ok(h.reply_serial()? == Some(serial))
            }) {
                Ok(m) => match m.header()?.message_type()? {
                    MessageType::Error => return Err(m.into()),
                    MessageType::MethodReturn => return Ok(m),
                    _ => (),
                },
                Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    wait_on(self.as_raw_fd(), PollFlags::POLLIN)?;
                }
                Err(e) => return Err(e),
            };
        }
    }

    /// Emit a signal.
    ///
    /// Create a signal message, and send it over the connection.
    pub fn emit_signal<B>(
        &self,
        destination: Option<&str>,
        path: &str,
        iface: &str,
        signal_name: &str,
        body: &B,
    ) -> Result<()>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::signal(
            self.unique_name(),
            destination,
            path,
            iface,
            signal_name,
            body,
        )?;

        self.send_message(m)?;

        Ok(())
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
        let m = Message::method_reply(self.unique_name(), call, body)?;
        self.send_message(m)
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
        let m = Message::method_error(self.unique_name(), call, error_name, body)?;
        self.send_message(m)
    }

    /// Checks if `self` is a connection to a message bus.
    ///
    /// This will return `false` for p2p connections.
    pub fn is_bus(&self) -> bool {
        self.0.bus_conn
    }

    fn hello_bus(self) -> Result<Self> {
        let name = fdo::DBusProxy::new(&self)?
            .hello()
            .map_err(|e| Error::Handshake(format!("Hello failed: {}", e)))?;
        self.0
            .unique_name
            .set(name)
            // programmer (probably our) error if this fails.
            .expect("Attempted to set unique_name twice");

        Ok(self)
    }

    fn new<H: Handshake<Box<dyn Socket>>>(handshake: H, bus_conn: bool) -> Result<Self> {
        let auth = handshake.blocking_finish()?;
        let out_socket = auth
            .conn
            .socket()
            .try_clone()
            .expect("Failed to clone socket");

        let conn = Self(Arc::new(ConnectionInner {
            raw_in_conn: Mutex::new(auth.conn),
            raw_out_conn: Mutex::new(RawConnection::wrap(out_socket)),
            server_guid: auth.server_guid,
            cap_unix_fd: auth.cap_unix_fd,
            bus_conn,
            serial: Mutex::new(1),
            unique_name: OnceCell::new(),
            incoming_queue: Mutex::new(vec![]),
            max_queued: RwLock::new(DEFAULT_MAX_QUEUED),
        }));

        if !bus_conn {
            return Ok(conn);
        }
        // Now that the server has approved us, we must send the bus Hello, as per specs
        conn.hello_bus()
    }

    fn next_serial(&self) -> u32 {
        let mut serial = self.0.serial.lock().expect(LOCK_FAIL_MSG);
        let current = *serial;
        *serial = current + 1;

        current
    }

    // Get the message directly from the socket (ignoring the queue).
    fn receive_message_raw(&self) -> Result<Option<Message>> {
        let incoming = self
            .0
            .raw_in_conn
            .lock()
            .expect(LOCK_FAIL_MSG)
            .try_receive_message()?;
        Ok(Some(incoming))
    }
}

#[cfg(test)]
mod tests {
    use std::{os::unix::net::UnixStream, thread};

    use crate::{Connection, Guid};

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

        let val = server_thread.join().expect("failed to join server thread");
        assert_eq!(val, "yay");
    }

    #[test]
    fn serial_monotonically_increases() {
        let c = Connection::new_session().unwrap();
        let serial = c.next_serial() + 1;

        for next in serial..serial + 10 {
            assert_eq!(next, c.next_serial());
        }
    }
}
