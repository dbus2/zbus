use async_broadcast::{broadcast, InactiveReceiver, Sender as Broadcaster};
use async_channel::{bounded, Receiver, Sender};
use async_io::{block_on, Async};
use async_lock::{Mutex, MutexGuard};
use once_cell::sync::OnceCell;
use static_assertions::assert_impl_all;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    convert::TryInto,
    future::ready,
    hash::{Hash, Hasher},
    io::{self, ErrorKind},
    os::unix::{
        io::{AsRawFd, RawFd},
        net::UnixStream,
    },
    pin::Pin,
    sync::{
        self,
        atomic::{AtomicBool, AtomicU32, Ordering::SeqCst},
        Arc,
    },
    task::{Context, Poll},
};
use zvariant::ObjectPath;

use event_listener::Event;
use futures_core::{stream, Future};
use futures_util::{
    future::{select, Either},
    sink::SinkExt,
    stream::{select as stream_select, StreamExt},
};

use crate::{
    azync::Authenticated,
    fdo,
    raw::{Connection as RawConnection, Socket},
    Error, Guid, Message, MessageError, MessageType, Result,
};

const DEFAULT_MAX_QUEUED: usize = 64;

const FDO_DBUS_SERVICE: &str = "org.freedesktop.DBus";
const FDO_DBUS_INTERFACE: &str = "org.freedesktop.DBus";
const FDO_DBUS_PATH: &str = "/org/freedesktop/DBus";
const FDO_DBUS_MATCH_RULE_EXCEMPT_SIGNALS: [&str; 2] = ["NameAcquired", "NameLost"];

#[derive(Debug, Hash, Eq, PartialEq)]
struct SignalInfo<'s> {
    sender: &'s str,
    path: ObjectPath<'s>,
    interface: &'s str,
    signal_name: &'s str,
}

impl<'s> SignalInfo<'s> {
    fn new<E>(
        sender: &'s str,
        path: impl TryInto<ObjectPath<'s>, Error = E>,
        interface: &'s str,
        signal_name: &'s str,
    ) -> Result<Self>
    where
        Error: From<E>,
    {
        Ok(Self {
            sender,
            path: path.try_into()?,
            interface,
            signal_name,
        })
    }

    fn create_match_rule(&self) -> Option<String> {
        if self.match_rule_excempt() {
            return None;
        }

        // FIXME: Use the API to create this once we've it (issue#69).
        Some(format!(
            "type='signal',sender='{}',path_namespace='{}',interface='{}',member='{}'",
            self.sender, self.path, self.interface, self.signal_name,
        ))
    }

    fn match_rule_excempt(&self) -> bool {
        self.sender == FDO_DBUS_SERVICE
            && self.interface == FDO_DBUS_INTERFACE
            && self.path.as_str() == FDO_DBUS_PATH
            && FDO_DBUS_MATCH_RULE_EXCEMPT_SIGNALS.contains(&self.signal_name)
    }

    fn calc_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        hasher.finish()
    }
}

#[derive(Debug)]
struct SignalSubscription {
    num_subscribers: usize,
    match_rule: Option<String>,
}

#[derive(Debug)]
struct ConnectionInner<S> {
    server_guid: Guid,
    cap_unix_fd: bool,
    bus_conn: bool,
    unique_name: OnceCell<String>,

    raw_out_conn: Mutex<RawConnection<Async<S>>>,
    // Serial number for next outgoing message
    serial: AtomicU32,

    // Message receiver thread
    msg_receiver_thread: Arc<MessageReceiverThread<S>>,

    // We're using sync Mutex here as we don't intend to keep it locked while awaiting.
    msg_receiver: sync::RwLock<InactiveReceiver<Arc<Message>>>,

    // Receiver side of the error channel
    error_receiver: Receiver<Error>,

    signal_subscriptions: Mutex<HashMap<u64, SignalSubscription>>,
}

impl<S> Drop for ConnectionInner<S> {
    fn drop(&mut self) {
        self.msg_receiver_thread.run.store(false, SeqCst);
        self.msg_receiver_thread.run_event.notify(1);
    }
}

#[derive(Debug)]
struct MessageReceiverThread<S> {
    raw_in_conn: Mutex<RawConnection<Async<S>>>,

    // Message broadcaster.
    msg_sender: Broadcaster<Arc<Message>>,

    // Sender side of the error channel
    error_sender: Sender<Error>,

    // To notify msg receeiver thread that it doesn't need to run anymore.
    run_event: Event,
    run: AtomicBool,
}

impl MessageReceiverThread<Box<dyn Socket>> {
    fn new(
        raw_in_conn: RawConnection<Async<Box<dyn Socket>>>,
        msg_sender: Broadcaster<Arc<Message>>,
        error_sender: Sender<Error>,
    ) -> Arc<Self> {
        Arc::new(Self {
            raw_in_conn: Mutex::new(raw_in_conn),
            msg_sender,
            error_sender,
            run_event: Event::new(),
            run: AtomicBool::new(true),
        })
    }

    fn launch(self: Arc<Self>) -> Result<()> {
        // FIXME: Perhaps this should be a task but something needs to drive it then then. Maybe
        // the message stream should run the task but then we'll probably be back to the issue of
        // stream not being reliable in all situations, which is precisely why we split the message
        // receiving into a separate thread.
        std::thread::Builder::new()
            .name("zbus::azync::Connection::receive_msg".into())
            .spawn(move || block_on(self.receive_msg()))?;

        Ok(())
    }

    // Keep receiving messages and put them on the queue.
    async fn receive_msg(self: Arc<Self>) {
        while self.run.load(SeqCst) {
            let mut raw_conn = self.raw_in_conn.lock().await;

            // Ignore errors from sending to msg or error channels. The only reason these calls
            // fail is when the channel is closed and that will only happen when `Connection` is
            // being dropped.
            // TODO: We should still log in case of error when we've logging.

            let msg = match select(
                Box::pin(ReceiveMessage {
                    raw_conn: &mut raw_conn,
                }),
                self.run_event.listen(),
            )
            .await
            {
                Either::Left((msg, _)) => {
                    match msg {
                        Ok(msg) => msg,
                        Err(e) => {
                            // Ignoring errors. See comment above.
                            let _ = self.error_sender.send(e).await;

                            continue;
                        }
                    }
                }
                Either::Right((_, _)) => continue,
            };

            let msg = Arc::new(msg);
            // Ignoring errors. See comment above.
            let _ = self.msg_sender.broadcast(msg.clone()).await;
        }
    }
}

/// The asynchronous sibling of [`zbus::Connection`].
///
/// Most of the API is very similar to [`zbus::Connection`], except it's asynchronous. However,
/// there are a few differences:
///
/// ### Sending Messages
///
/// For sending messages you can either use [`Connection::send_message`] method or make use of the
/// [`futures_sink::Sink`] implementation that is returned by [`Connection::sink`] method. For
/// latter, you might find [`SinkExt`] API very useful. Keep in mind that [`Connection`] will not
/// manage the serial numbers (cookies) on the messages for you when they are sent through the
/// [`MessageSink`]. You can manually assign unique serial numbers to them using the
/// [`Connection::assign_serial_num`] method before sending them off, if needed. Having said that,
/// [`MessageSink`] is mainly useful for sending out signals, as they do not expect a reply, and
/// serial numbers are not very useful for signals either for the same reason.
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
///# async_io::block_on(async {
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
///# async_io::block_on(async {
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
/// while let Some(msg) = connection.stream().await.try_next().await? {
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
#[derive(Clone, Debug)]
pub struct Connection(Arc<ConnectionInner<Box<dyn Socket>>>);

assert_impl_all!(Connection: Send, Sync, Unpin);

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
        #[cfg(any(target_os = "android", target_os = "linux"))]
        let client_uid = {
            use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

            let creds = getsockopt(stream.as_raw_fd(), PeerCredentials)
                .map_err(|e| Error::Handshake(format!("Failed to get peer credentials: {}", e)))?;

            creds.uid()
        };
        #[cfg(any(
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        let client_uid = nix::unistd::getpeereid(stream.as_raw_fd())
            .map_err(|e| Error::Handshake(format!("Failed to get peer credentials: {}", e)))?
            .0
            .into();

        let auth = Authenticated::server(
            Async::new(Box::new(stream) as Box<dyn Socket>)?,
            guid.clone(),
            client_uid,
        )
        .await?;

        Self::new(auth, false).await
    }

    /// Get a stream to receive incoming messages.
    pub async fn stream(&self) -> MessageStream {
        let msg_receiver = self
            .0
            .msg_receiver
            .read()
            // SAFETY: Not much we can do about a poisoned mutex.
            .expect("poisoned lock")
            .activate_cloned()
            .map(Ok);
        let error_stream = self.0.error_receiver.clone().map(Err);
        let stream = stream_select(error_stream, msg_receiver).boxed();

        MessageStream { stream }
    }

    /// Get a sink to send out messages.
    ///
    /// **Note:** At the moment, a sink requires locking all other outgoing messages on the
    /// connection. Therefore once you have created a sink for a connection, all sending
    /// operations will not yield any results until the sink is dropped. However, this is not as
    /// big an issue since all operations in this API are asynchronous (i-e non-blocking). Moreover,
    /// this limitation will hopefully be removed in the near future.
    pub async fn sink(&self) -> MessageSink<'_> {
        MessageSink {
            raw_conn: self.0.raw_out_conn.lock().await,
            cap_unix_fd: self.0.cap_unix_fd,
        }
    }

    /// Send `msg` to the peer.
    ///
    /// Unlike [`MessageSink`], this method sets a unique (to this connection) serial number on the
    /// message before sending it off, for you.
    ///
    /// On successfully sending off `msg`, the assigned serial number is returned.
    pub async fn send_message(&self, mut msg: Message) -> Result<u32> {
        let serial = self.assign_serial_num(&mut msg)?;

        self.sink().await.send(msg).await?;

        Ok(serial)
    }

    /// Send a method call.
    ///
    /// Create a method-call message, send it over the connection, then wait for the reply.
    ///
    /// On successful reply, an `Ok(Message)` is returned. On error, an `Err` is returned. D-Bus
    /// error replies are returned as [`Error::MethodError`].
    pub async fn call_method<B, E>(
        &self,
        destination: Option<&str>,
        path: impl TryInto<ObjectPath<'_>, Error = E>,
        interface: Option<&str>,
        method_name: &str,
        body: &B,
    ) -> Result<Arc<Message>>
    where
        B: serde::ser::Serialize + zvariant::Type,
        MessageError: From<E>,
    {
        let stream = self.stream().await;
        let m = Message::method(
            self.unique_name(),
            destination,
            path,
            interface,
            method_name,
            body,
        )?;
        let serial = self.send_message(m).await?;
        match stream
            .filter(move |m| {
                ready(
                    m.as_ref()
                        .map(|m| {
                            matches!(
                                m.primary_header().msg_type(),
                                MessageType::Error | MessageType::MethodReturn
                            ) && m.header().and_then(|h| h.reply_serial()) == Ok(Some(serial))
                        })
                        .unwrap_or(false),
                )
            })
            .next()
            .await
        {
            Some(msg) => match msg {
                Ok(m) => {
                    match m.header()?.message_type()? {
                        MessageType::Error => Err(m.into()),
                        MessageType::MethodReturn => Ok(m),
                        // We already established the msg type in `filter` above.
                        _ => unreachable!(),
                    }
                }
                Err(e) => Err(e),
            },
            None => {
                // If SocketStream gives us None, that means the socket was closed
                Err(crate::Error::Io(io::Error::new(
                    ErrorKind::BrokenPipe,
                    "socket closed",
                )))
            }
        }
    }

    /// Emit a signal.
    ///
    /// Create a signal message, and send it over the connection.
    pub async fn emit_signal<B, E>(
        &self,
        destination: Option<&str>,
        path: impl TryInto<ObjectPath<'_>, Error = E>,
        interface: &str,
        signal_name: &str,
        body: &B,
    ) -> Result<()>
    where
        B: serde::ser::Serialize + zvariant::Type,
        MessageError: From<E>,
    {
        let m = Message::signal(
            self.unique_name(),
            destination,
            path,
            interface,
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
    pub async fn reply<B>(&self, call: &Message, body: &B) -> Result<u32>
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
    pub async fn reply_error<B>(&self, call: &Message, error_name: &str, body: &B) -> Result<u32>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::method_error(self.unique_name(), call, error_name, body)?;
        self.send_message(m).await
    }

    /// Checks if `self` is a connection to a message bus.
    ///
    /// This will return `false` for p2p connections.
    pub fn is_bus(&self) -> bool {
        self.0.bus_conn
    }

    /// Assigns a serial number to `msg` that is unique to this connection.
    ///
    /// This method can fail if `msg` is corrupt.
    pub fn assign_serial_num(&self, msg: &mut Message) -> Result<u32> {
        let mut serial = 0;
        msg.modify_primary_header(|primary| {
            serial = *primary.serial_num_or_init(|| self.next_serial());
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
        self.0
            .msg_receiver
            .read()
            .expect("poisoned lock")
            .capacity()
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
    ///# use async_io::block_on;
    ///#
    ///# block_on(async {
    /// let conn = Connection::new_session()
    ///     .await?
    ///     .set_max_queued(30);
    /// assert_eq!(conn.max_queued(), 30);
    ///
    ///#     Ok::<(), zbus::Error>(())
    ///# });
    ///#
    /// // Do something usefull with `conn`..
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn set_max_queued(self, max: usize) -> Self {
        self.0
            .msg_receiver
            .write()
            .expect("poisoned lock")
            .set_capacity(max);

        self
    }

    /// The server's GUID.
    pub fn server_guid(&self) -> &str {
        self.0.server_guid.as_str()
    }

    /// Get the raw file descriptor of this connection.
    pub async fn as_raw_fd(&self) -> RawFd {
        (self.0.msg_receiver_thread.raw_in_conn.lock().await.socket()).as_raw_fd()
    }

    pub(crate) async fn subscribe_signal<'s, E>(
        &self,
        sender: &'s str,
        path: impl TryInto<ObjectPath<'s>, Error = E>,
        interface: &'s str,
        signal_name: &'s str,
    ) -> Result<u64>
    where
        Error: From<E>,
    {
        let signal = SignalInfo::new(sender, path, interface, signal_name)?;
        let hash = signal.calc_hash();
        let mut subscriptions = self.0.signal_subscriptions.lock().await;
        match subscriptions.get_mut(&hash) {
            Some(subscription) => subscription.num_subscribers += 1,
            None => {
                let match_rule = signal.create_match_rule();
                if let Some(match_rule) = &match_rule {
                    fdo::AsyncDBusProxy::new(&self)?
                        .add_match(&match_rule)
                        .await?;
                }

                subscriptions.insert(
                    hash,
                    SignalSubscription {
                        num_subscribers: 1,
                        match_rule,
                    },
                );
            }
        }

        Ok(hash)
    }

    pub(crate) async fn unsubscribe_signal<'s, E>(
        &self,
        sender: &'s str,
        path: impl TryInto<ObjectPath<'s>, Error = E>,
        interface: &'s str,
        signal_name: &'s str,
    ) -> Result<bool>
    where
        Error: From<E>,
    {
        let signal = SignalInfo::new(sender, path, interface, signal_name)?;
        let hash = signal.calc_hash();

        self.unsubscribe_signal_by_id(hash).await
    }

    pub(crate) async fn unsubscribe_signal_by_id(&self, subscription_id: u64) -> Result<bool> {
        let mut subscriptions = self.0.signal_subscriptions.lock().await;
        match subscriptions.get_mut(&subscription_id) {
            Some(subscription) => {
                subscription.num_subscribers -= 1;

                if subscription.num_subscribers == 0 {
                    if let Some(match_rule) = &subscription.match_rule {
                        fdo::AsyncDBusProxy::new(&self)?
                            .remove_match(match_rule.as_str())
                            .await?;
                    }

                    subscriptions.remove(&subscription_id);
                }

                Ok(true)
            }
            None => Ok(false),
        }
    }

    async fn hello_bus(self) -> Result<Self> {
        let name = fdo::AsyncDBusProxy::new(&self)?.hello().await?;

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
        let out_conn = RawConnection::wrap(Async::new(out_socket)?);
        let (mut msg_sender, msg_receiver) = broadcast(DEFAULT_MAX_QUEUED);
        msg_sender.set_overflow(true);
        let msg_receiver = msg_receiver.deactivate();
        let (error_sender, error_receiver) = bounded(1);
        let msg_receiver_thread = MessageReceiverThread::new(auth.conn, msg_sender, error_sender);

        let connection = Self(Arc::new(ConnectionInner {
            raw_out_conn: Mutex::new(out_conn),
            error_receiver,
            server_guid: auth.server_guid,
            cap_unix_fd: auth.cap_unix_fd,
            bus_conn: bus_connection,
            serial: AtomicU32::new(1),
            unique_name: OnceCell::new(),
            signal_subscriptions: Mutex::new(HashMap::new()),
            msg_receiver_thread: msg_receiver_thread.clone(),
            msg_receiver: sync::RwLock::new(msg_receiver),
        }));

        // Start the message receiver thread.
        msg_receiver_thread.launch()?;

        if !bus_connection {
            return Ok(connection);
        }

        // Now that the server has approved us, we must send the bus Hello, as per specs
        connection.hello_bus().await
    }

    fn next_serial(&self) -> u32 {
        self.0.serial.fetch_add(1, SeqCst)
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

/// A [`futures_sink::Sink`] implementation that consumes [`Message`] instances.
///
/// Use [`Connection::sink`] to create an instance of this type.
pub struct MessageSink<'s> {
    raw_conn: MutexGuard<'s, RawConnection<Async<Box<dyn Socket>>>>,
    cap_unix_fd: bool,
}

assert_impl_all!(MessageSink<'_>: Send, Sync, Unpin);

impl MessageSink<'_> {
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

impl futures_sink::Sink<Message> for MessageSink<'_> {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        // TODO: We should have a max queue length in raw::Socket for outgoing messages.
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, msg: Message) -> Result<()> {
        if !msg.fds().is_empty() && !self.cap_unix_fd {
            return Err(Error::Unsupported);
        }

        self.get_mut().raw_conn.enqueue_message(msg);

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

/// A [`stream::Stream`] implementation that yields [`Message`] items.
///
/// Use [`Connection::stream`] to create an instance of this type.
pub struct MessageStream {
    stream: stream::BoxStream<'static, Result<Arc<Message>>>,
}

assert_impl_all!(MessageStream: Send, Unpin);

impl stream::Stream for MessageStream {
    type Item = Result<Arc<Message>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        stream::Stream::poll_next(self.get_mut().stream.as_mut(), cx)
    }
}

struct ReceiveMessage<'r, 's> {
    raw_conn: &'r mut MutexGuard<'s, RawConnection<Async<Box<dyn Socket>>>>,
}

impl<'r, 's> Future for ReceiveMessage<'r, 's> {
    type Output = Result<Message>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let stream = self.get_mut();

        loop {
            match stream.raw_conn.try_receive_message() {
                Err(Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => {
                    let poll = stream.raw_conn.socket().poll_readable(cx);

                    match poll {
                        Poll::Pending => return Poll::Pending,
                        // Guess socket became ready already so let's try it again.
                        Poll::Ready(Ok(_)) => continue,
                        Poll::Ready(Err(e)) => return Poll::Ready(Err(e.into())),
                    }
                }
                m => return Poll::Ready(m),
            }
        }
    }
}

impl From<crate::Connection> for Connection {
    fn from(conn: crate::Connection) -> Self {
        conn.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use futures_util::stream::TryStreamExt;
    use ntest::timeout;
    use std::os::unix::net::UnixStream;
    use test_env_log::test;

    use super::*;

    #[test]
    #[timeout(1000)]
    fn unix_p2p() {
        async_io::block_on(test_unix_p2p()).unwrap();
    }

    async fn test_unix_p2p() -> Result<()> {
        let guid = Guid::generate();

        let (p0, p1) = UnixStream::pair().unwrap();

        let server = Connection::new_unix_server(p0, &guid);
        let client = Connection::new_unix_client(p1, false);

        let (client_conn, server_conn) = futures_util::try_join!(client, server)?;
        let mut client_stream = client_conn.stream().await;
        let mut server_stream = server_conn.stream().await;

        let server_future = async {
            let mut method: Option<Arc<Message>> = None;
            while let Some(m) = server_stream.try_next().await? {
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
            let m = client_stream.try_next().await?.unwrap();
            assert_eq!(m.to_string(), "Signal ASignalForYou");
            reply.body::<String>().map_err(|e| e.into())
        };

        let (val, _) = futures_util::try_join!(client_future, server_future)?;
        assert_eq!(val, "yay");

        Ok(())
    }

    #[test]
    #[timeout(1000)]
    fn serial_monotonically_increases() {
        async_io::block_on(test_serial_monotonically_increases());
    }

    async fn test_serial_monotonically_increases() {
        let c = Connection::new_session().await.unwrap();
        let serial = c.next_serial() + 1;

        for next in serial..serial + 10 {
            assert_eq!(next, c.next_serial());
        }
    }
}
