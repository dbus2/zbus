use async_io::Async;
use once_cell::sync::OnceCell;
use std::{
    io::{self, ErrorKind},
    os::unix::{io::AsRawFd, net::UnixStream},
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::stream;
use futures_util::{sink::SinkExt, stream::TryStreamExt};

use crate::{
    azync::Authenticated,
    raw::{Connection as RawConnection, Socket},
    Error, Guid, Message, MessageType, Result, DEFAULT_MAX_QUEUED,
};

#[derive(Debug)]
struct ConnectionInner<S> {
    server_guid: Guid,
    cap_unix_fd: bool,
    unique_name: OnceCell<String>,

    raw_in_conn: RawConnection<Async<S>>,
    raw_out_conn: RawConnection<Async<S>>,
    // Serial number for next outgoing message
    serial: u32,

    // Queue of incoming messages
    incoming_queue: Vec<Message>,

    // Max number of messages to queue
    max_queued: usize,
}

/// The asynchronous sibling of [`zbus::Connection`].
///
/// Most of the API is very similar to [`zbus::Connection`], except it's asynchronous. However,
/// there are a few differences:
///
/// ### Cloning and Mutability
///
/// Unlike [`zbus::Connection`], this type does not implement [`std::clone::Clone`]. The reason is
/// that implementation will be very difficult (and still prone to deadlocks) if connection is
/// owned by multiple tasks/threads.
///
/// Also notice that unlike [`zbus::Connection`], most methods take a `&mut self`, rather than a
/// `&self`. If they'd take `&self`, `Connection` will need to manage mutability internally, which
/// is not a very good match with the general async/await machinery and runtimes in Rust and could
/// easily lead into some hard-to-debug deadlocks. You can use [`std::cell::Cell`],
/// [`std::sync::Mutex`] or other related API combined with [`std::rc::Rc`] or [`std::sync::Arc`]
/// for sharing a mutable `Connection` instance between different parts of your code (or threads).
///
/// ### Sending Messages
///
/// For sending messages you can either use [`Connection::send_message`] method or make use of the
/// [`futures_sink::Sink`] that is returned by [`Connection::sink`] method. For latter, you might
/// find [`SinkExt`] API very useful. Keep in mind that [`Connection`] will not manage the serial
/// numbers (cookies) on the messages for you when they are sent through the [`Sink`]. You can
/// manually assign unique serial numbers to them using the [`Connection::assign_serial_num`] method
/// before sending them off, if needed. Having said that, [`Sink`] is mainly useful for sending out
/// signals, as they do not expect a reply, and serial numbers are not very useful for signals
/// either for the same reason.
///
/// ### Receiving Messages
///
/// Unlike [`zbus::Connection`], there is no direct async equivalent of
/// [`zbus::Connection::receive_message`] method provided. This is because the `futures` crate
/// already provides a nice rich API that makes use of the [`stream::Stream`] implementation that is
/// returned by [`Connection::stream`] method.
///
/// ### Examples
///
/// #### Get the session bus ID
///
/// ```
///# use zvariant::Type;
///#
///# futures_executor::block_on(async {
/// use zbus::azync::Connection;
///
/// let mut connection = Connection::new_session().await?;
///
/// let reply = connection
///     .call_method(
///         Some("org.freedesktop.DBus"),
///         "/org/freedesktop/DBus",
///         Some("org.freedesktop.DBus"),
///         "GetId",
///         &(),
///     )
///     .await?;
///
/// let id: &str = reply.body()?;
/// println!("Unique ID of the bus: {}", id);
///# Ok::<(), zbus::Error>(())
///# });
/// ```
///
/// #### Monitoring all messages
///
/// Let's eavesdrop on the session bus 😈 using the [Monitor] interface:
///
/// ```rust,no_run
///# futures_executor::block_on(async {
/// use futures_util::stream::TryStreamExt;
/// use zbus::azync::Connection;
///
/// let mut connection = Connection::new_session().await?;
///
/// connection
///     .call_method(
///         Some("org.freedesktop.DBus"),
///         "/org/freedesktop/DBus",
///         Some("org.freedesktop.DBus.Monitoring"),
///         "BecomeMonitor",
///         &(&[] as &[&str], 0u32),
///     )
///     .await?;
///
/// while let Some(msg) = connection.stream().try_next().await? {
///     println!("Got message: {}", msg);
/// }
///
///# Ok::<(), zbus::Error>(())
///# });
/// ```
///
/// This should print something like:
///
/// ```console
/// Got message: Signal NameAcquired from org.freedesktop.DBus
/// Got message: Signal NameLost from org.freedesktop.DBus
/// Got message: Method call GetConnectionUnixProcessID from :1.1324
/// Got message: Error org.freedesktop.DBus.Error.NameHasNoOwner:
///              Could not get PID of name ':1.1332': no such name from org.freedesktop.DBus
/// Got message: Method call AddMatch from :1.918
/// Got message: Method return from org.freedesktop.DBus
/// ```
///
/// [Monitor]: https://dbus.freedesktop.org/doc/dbus-specification.html#bus-messages-become-monitor
#[derive(Debug)]
pub struct Connection(ConnectionInner<Box<dyn Socket>>);

impl Connection {
    /// Create and open a D-Bus connection from a `UnixStream`.
    ///
    /// The connection may either be set up for a *bus* connection, or not (for peer-to-peer
    /// communications).
    ///
    /// Upon successful return, the connection is fully established and negotiated: D-Bus messages
    /// can be sent and received.
    pub async fn new_unix_client(stream: UnixStream, bus_connection: bool) -> Result<Self> {
        // SASL Handshake
        let auth = Authenticated::client(Async::new(Box::new(stream) as Box<dyn Socket>)?).await?;

        Self::new(auth, bus_connection).await
    }

    /// Create a server `Connection` for the given `UnixStream` and the server `guid`.
    ///
    /// The connection will wait for incoming client authentication handshake & negotiation messages,
    /// for peer-to-peer communications.
    ///
    /// Upon successful return, the connection is fully established and negotiated: D-Bus messages
    /// can be sent and received.
    pub async fn new_unix_server(stream: UnixStream, guid: &Guid) -> Result<Self> {
        use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

        // FIXME: Could and should this be async?
        let creds = getsockopt(stream.as_raw_fd(), PeerCredentials)
            .map_err(|e| Error::Handshake(format!("Failed to get peer credentials: {}", e)))?;

        let auth = Authenticated::server(
            Async::new(Box::new(stream) as Box<dyn Socket>)?,
            guid.clone(),
            creds.uid(),
        )
        .await?;

        Self::new(auth, false).await
    }

    /// Get a stream to receive incoming messages.
    pub fn stream<'s, 'c: 's>(&'c mut self) -> Stream<'s> {
        Stream {
            raw_conn: &mut self.0.raw_in_conn,
            incoming_queue: &mut self.0.incoming_queue,
        }
    }

    /// Get a sink to send out messages.
    pub fn sink<'s, 'c: 's>(&'c mut self) -> Sink<'s> {
        Sink {
            raw_conn: &mut self.0.raw_out_conn,
            cap_unix_fd: self.0.cap_unix_fd,
        }
    }

    /// Send `msg` to the peer.
    ///
    /// Unlike [`Sink`], this method sets a unique (to this connection) serial number on the message
    /// before sending it off, for you.
    ///
    /// On successfully sending off `msg`, the assigned serial number is returned.
    pub async fn send_message(&mut self, mut msg: Message) -> Result<u32> {
        let serial = self.assign_serial_num(&mut msg)?;

        self.sink().send(msg).await?;

        Ok(serial)
    }

    /// Send a method call.
    ///
    /// Create a method-call message, send it over the connection, then wait for the reply.
    ///
    /// On succesful reply, an `Ok(Message)` is returned. On error, an `Err` is returned. D-Bus
    /// error replies are returned as [`Error::MethodError`].
    pub async fn call_method<B>(
        &mut self,
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
        let serial = self.send_message(m).await?;

        let mut tmp_queue = vec![];

        while let Some(m) = self.stream().try_next().await? {
            let h = m.header()?;

            if h.reply_serial()? != Some(serial) {
                if self.0.incoming_queue.len() + tmp_queue.len() < self.max_queued() {
                    // We first push to a temporary queue as otherwise it'll create an infinite loop
                    // since subsequent `receive_message` call will pick up the message from the main
                    // queue.
                    tmp_queue.push(m);
                }

                continue;
            } else {
                self.0.incoming_queue.append(&mut tmp_queue);
            }

            match h.message_type()? {
                MessageType::Error => return Err(m.into()),
                MessageType::MethodReturn => return Ok(m),
                _ => (),
            }
        }

        // If Stream gives us None, that means the socket was closed
        Err(Error::Io(io::Error::new(
            ErrorKind::BrokenPipe,
            "socket closed",
        )))
    }

    /// Emit a signal.
    ///
    /// Create a signal message, and send it over the connection.
    pub async fn emit_signal<B>(
        &mut self,
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

        self.send_message(m).await.map(|_| ())
    }

    /// Reply to a message.
    ///
    /// Given an existing message (likely a method call), send a reply back to the caller with the
    /// given `body`.
    ///
    /// Returns the message serial number.
    pub async fn reply<B>(&mut self, call: &Message, body: &B) -> Result<u32>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::method_reply(self.unique_name(), call, body)?;
        self.send_message(m).await
    }

    /// Reply an error to a message.
    ///
    /// Given an existing message (likely a method call), send an error reply back to the caller
    /// with the given `error_name` and `body`.
    ///
    /// Returns the message serial number.
    pub async fn reply_error<B>(
        &mut self,
        call: &Message,
        error_name: &str,
        body: &B,
    ) -> Result<u32>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::method_error(self.unique_name(), call, error_name, body)?;
        self.send_message(m).await
    }

    /// Assigns a serial number to `msg` that is unique to this connection.
    ///
    /// This method can fail if `msg` is corrupt.
    pub fn assign_serial_num(&mut self, msg: &mut Message) -> Result<u32> {
        let serial = self.next_serial();
        msg.modify_primary_header(|primary| {
            primary.set_serial_num(serial);

            Ok(())
        })?;

        Ok(serial)
    }

    /// The unique name as assigned by the message bus or `None` if not a message bus connection.
    pub fn unique_name(&self) -> Option<&str> {
        self.0.unique_name.get().map(|s| s.as_str())
    }

    /// Max number of messages to queue.
    pub fn max_queued(&self) -> usize {
        self.0.max_queued
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
    ///# use zbus::azync::Connection;
    /// use futures_executor::block_on;
    ///
    /// let conn = block_on(Connection::new_session())?.set_max_queued(30);
    /// assert_eq!(conn.max_queued(), 30);
    ///
    /// // Do something usefull with `conn`..
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn set_max_queued(mut self, max: usize) -> Self {
        self.0.max_queued = max;

        self
    }

    /// The server's GUID.
    pub fn server_guid(&self) -> &str {
        self.0.server_guid.as_str()
    }

    async fn hello_bus(mut self) -> Result<Self> {
        // TODO: Use fdo module once it's async.
        let name: String = self
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "Hello",
                &(),
            )
            .await?
            .body()?;

        self.0
            .unique_name
            .set(name)
            // programmer (probably our) error if this fails.
            .expect("Attempted to set unique_name twice");

        Ok(self)
    }

    async fn new(
        auth: Authenticated<Async<Box<dyn Socket>>>,
        bus_connection: bool,
    ) -> Result<Self> {
        let auth = auth.into_inner();
        let out_socket = auth.conn.socket().get_ref().try_clone()?;

        let connection = Self(ConnectionInner {
            raw_in_conn: auth.conn,
            raw_out_conn: RawConnection::wrap(Async::new(out_socket)?),
            server_guid: auth.server_guid,
            cap_unix_fd: auth.cap_unix_fd,
            serial: 1,
            unique_name: OnceCell::new(),
            incoming_queue: vec![],
            max_queued: DEFAULT_MAX_QUEUED,
        });

        if !bus_connection {
            return Ok(connection);
        }

        // Now that the server has approved us, we must send the bus Hello, as per specs
        connection.hello_bus().await
    }

    fn next_serial(&mut self) -> u32 {
        let serial = self.0.serial;
        self.0.serial = serial + 1;

        serial
    }

    /// Create a `Connection` to the session/user message bus.
    pub async fn new_session() -> Result<Self> {
        Self::new(Authenticated::session().await?, true).await
    }

    /// Create a `Connection` to the system-wide message bus.
    pub async fn new_system() -> Result<Self> {
        Self::new(Authenticated::system().await?, true).await
    }

    /// Create a `Connection` for the given [D-Bus address].
    ///
    /// [D-Bus address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub async fn new_for_address(address: &str, bus_connection: bool) -> Result<Self> {
        Self::new(Authenticated::for_address(address).await?, bus_connection).await
    }
}

/// Our [`futures_sink::Sink`] implementation.
///
/// Use [`Connection::sink`] to create an instance of this type.
pub struct Sink<'s> {
    raw_conn: &'s mut RawConnection<Async<Box<dyn Socket>>>,
    cap_unix_fd: bool,
}

impl<'s> Sink<'s> {
    fn flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        loop {
            match self.raw_conn.try_flush() {
                Ok(()) => return Poll::Ready(Ok(())),
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        let poll = self.raw_conn.socket().poll_writable(cx);

                        match poll {
                            Poll::Pending => return Poll::Pending,
                            // Guess socket became ready already so let's try it again.
                            Poll::Ready(Ok(_)) => continue,
                            Poll::Ready(Err(e)) => return Poll::Ready(Err(e.into())),
                        }
                    } else {
                        return Poll::Ready(Err(Error::Io(e)));
                    }
                }
            }
        }
    }
}

impl<'s> futures_sink::Sink<Message> for Sink<'s> {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        // TODO: We should have a max queue length in raw::Socket for outgoing messages.
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, msg: Message) -> Result<()> {
        let sink = self.get_mut();
        if !msg.fds().is_empty() && !sink.cap_unix_fd {
            return Err(Error::Unsupported);
        }

        sink.raw_conn.enqueue_message(msg);

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.get_mut().flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let sink = self.get_mut();
        match sink.flush(cx) {
            Poll::Ready(Ok(_)) => (),
            Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
            Poll::Pending => return Poll::Pending,
        }

        Poll::Ready((sink.raw_conn).close())
    }
}

/// Our [`stream::Stream`] implementation.
///
/// Use [`Connection::stream`] to create an instance of this type.
pub struct Stream<'s> {
    raw_conn: &'s mut RawConnection<Async<Box<dyn Socket>>>,
    incoming_queue: &'s mut Vec<Message>,
}

impl<'s> stream::Stream for Stream<'s> {
    type Item = Result<Message>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let stream = self.get_mut();

        if let Some(msg) = stream.incoming_queue.pop() {
            return Poll::Ready(Some(Ok(msg)));
        }

        loop {
            match stream.raw_conn.try_receive_message() {
                Ok(m) => return Poll::Ready(Some(Ok(m))),
                Err(Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => {
                    let poll = stream.raw_conn.socket().poll_readable(cx);

                    match poll {
                        Poll::Pending => return Poll::Pending,
                        // Guess socket became ready already so let's try it again.
                        Poll::Ready(Ok(_)) => continue,
                        Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e.into()))),
                    }
                }
                Err(Error::Io(e)) if e.kind() == ErrorKind::BrokenPipe => return Poll::Ready(None),
                Err(e) => return Poll::Ready(Some(Err(e))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixStream;

    use super::*;

    #[test]
    fn unix_p2p() {
        futures_executor::block_on(test_unix_p2p()).unwrap();
    }

    async fn test_unix_p2p() -> Result<()> {
        let guid = Guid::generate();

        let (p0, p1) = UnixStream::pair().unwrap();

        let server = Connection::new_unix_server(p0, &guid);
        let client = Connection::new_unix_client(p1, false);

        let (mut client_conn, mut server_conn) = futures_util::try_join!(client, server)?;

        let server_future = async {
            let mut method: Option<Message> = None;
            while let Some(m) = server_conn.stream().try_next().await? {
                if m.to_string() == "Method call Test" {
                    method.replace(m);

                    break;
                }
            }
            let method = method.unwrap();

            // Send another message first to check the queueing function on client side.
            server_conn
                .emit_signal(None, "/", "org.zbus.p2p", "ASignalForYou", &())
                .await?;
            server_conn.reply(&method, &("yay")).await
        };

        let client_future = async {
            let reply = client_conn
                .call_method(None, "/", Some("org.zbus.p2p"), "Test", &())
                .await?;
            assert_eq!(reply.to_string(), "Method return");
            // Check we didn't miss the signal that was sent during the call.
            let m = client_conn.stream().try_next().await?.unwrap();
            assert_eq!(m.to_string(), "Signal ASignalForYou");
            reply.body::<String>().map_err(|e| e.into())
        };

        let (val, _) = futures_util::try_join!(client_future, server_future)?;
        assert_eq!(val, "yay");

        Ok(())
    }
}
