use async_broadcast::{broadcast, Receiver as BroadcastReceiver, Sender as Broadcaster};
use async_channel::{bounded, Receiver, Sender};
use async_executor::Executor;
#[cfg(feature = "internal-executor")]
use async_io::block_on;
use async_io::Async;
use async_lock::{Mutex, MutexGuard};
use async_task::Task;
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
        atomic::{AtomicU32, Ordering::SeqCst},
        Arc,
    },
    task::{Context, Poll},
};
use zvariant::ObjectPath;

use futures_core::{ready, stream, Future};
use futures_sink::Sink;
use futures_util::{
    future::{select, Either},
    sink::SinkExt,
    stream::StreamExt,
};

use crate::{
    azync::Authenticated,
    fdo,
    raw::{Connection as RawConnection, Socket},
    Error, Guid, Message, MessageType, Result,
};

const DEFAULT_MAX_QUEUED: usize = 64;

const FDO_DBUS_SERVICE: &str = "org.freedesktop.DBus";
const FDO_DBUS_INTERFACE: &str = "org.freedesktop.DBus";
const FDO_DBUS_PATH: &str = "/org/freedesktop/DBus";
const FDO_DBUS_MATCH_RULE_EXCEMPT_SIGNALS: [&str; 2] = ["NameAcquired", "NameLost"];

#[derive(Debug, Hash, Eq, PartialEq)]
struct SignalInfo<'s, 'p, 'i, 'sig> {
    sender: &'s str,
    path: ObjectPath<'p>,
    interface: &'i str,
    signal_name: &'sig str,
}

impl<'s, 'p, 'i, 'sig> SignalInfo<'s, 'p, 'i, 'sig> {
    fn new<E>(
        sender: &'s str,
        path: impl TryInto<ObjectPath<'p>, Error = E>,
        interface: &'i str,
        signal_name: &'sig str,
    ) -> Result<Self>
    where
        E: Into<Error>,
    {
        Ok(Self {
            sender,
            path: path.try_into().map_err(Into::into)?,
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

    raw_in_conn: Arc<Mutex<RawConnection<Async<S>>>>,
    // FIXME: We really should be using async_lock::Mutex here but `Sink::start_send is not very
    // async friendly. :(
    raw_out_conn: Arc<sync::Mutex<DynSocketConnection>>,
    // Serial number for next outgoing message
    serial: AtomicU32,

    // Our executor
    executor: Arc<Executor<'static>>,

    // Message receiver task
    msg_receiver_task: sync::Mutex<Option<Task<()>>>,

    signal_subscriptions: Mutex<HashMap<u64, SignalSubscription>>,
}

// FIXME: Should really use [`AsyncDrop`] for `ConnectionInner` when we've something like that to
//        cancel `msg_receiver_task` manually to ensure task is gone before the connection is.
//
// [`AsyncDrop`]: https://github.com/rust-lang/wg-async-foundations/issues/65

#[derive(Debug)]
struct MessageReceiverTask<S> {
    raw_in_conn: Arc<Mutex<RawConnection<Async<S>>>>,

    // Message broadcaster.
    msg_sender: Broadcaster<Arc<Message>>,

    // Sender side of the error channel
    error_sender: Sender<Error>,
}

type DynSocketConnection = RawConnection<Async<Box<dyn Socket>>>;

impl MessageReceiverTask<Box<dyn Socket>> {
    fn new(
        raw_in_conn: Arc<Mutex<DynSocketConnection>>,
        msg_sender: Broadcaster<Arc<Message>>,
        error_sender: Sender<Error>,
    ) -> Arc<Self> {
        Arc::new(Self {
            raw_in_conn,
            msg_sender,
            error_sender,
        })
    }

    fn spawn(self: Arc<Self>, executor: &Executor<'_>) -> Task<()> {
        executor.spawn(async move {
            self.receive_msg().await;
        })
    }

    // Keep receiving messages and put them on the queue.
    async fn receive_msg(self: Arc<Self>) {
        loop {
            let mut raw_conn = self.raw_in_conn.lock().await;

            // Ignore errors from sending to msg or error channels. The only reason these calls
            // fail is when the channel is closed and that will only happen when `Connection` is
            // being dropped.
            // TODO: We should still log in case of error when we've logging.

            let receive_msg = ReceiveMessage {
                raw_conn: &mut raw_conn,
            };
            let msg = match receive_msg.await {
                Ok(msg) => msg,
                Err(e) => {
                    // Ignoring errors. See comment above.
                    let _ = self.error_sender.send(e).await;

                    continue;
                }
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
/// [`Sink`] implementation. For latter, you might find [`SinkExt`] API very useful. Keep in mind
/// that [`Connection`] will not manage the serial numbers (cookies) on the messages for you when
/// they are sent through the [`Sink`] implementation. You can manually assign unique serial numbers
/// to them using the [`Connection::assign_serial_num`] method before sending them off, if needed.
/// Having said that, the [`Sink`] is mainly useful for sending out signals, as they do not expect
/// a reply, and serial numbers are not very useful for signals either for the same reason.
///
/// Since you do not need exclusive access to a `zbus::Connection` to send messages on the bus,
/// [`Sink`] is also implemented on `&Connection`.
///
/// ### Receiving Messages
///
/// Unlike [`zbus::Connection`], there is no direct async equivalent of
/// [`zbus::Connection::receive_message`] method provided. This is because the `futures` crate
/// already provides a nice rich API that makes use of the [`stream::Stream`] implementation.
///
/// Each `Connection` instance maintains its own queue of incoming messages (storing the last
/// `max_queued()` messages), so you can filter the stream for messages you care about and discard
/// the rest.  To avoid having multiple receivers filtering the same queue, `Stream` is only
/// available with an exclusive (mutable) reference to a `Connection`; clone the `Connection` to
/// get a new queue to use the `Stream`.
///
/// # Caveats
///
/// At the moment, a simultaneous [flush request] from multiple tasks/threads could
/// potentially create a busy loop, thus wasting CPU time. This limitation may be removed in the
/// future.
///
/// [flush request]: https://docs.rs/futures/0.3.15/futures/sink/trait.SinkExt.html#method.flush
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
/// while let Some(msg) = connection.try_next().await? {
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
pub struct Connection {
    inner: Arc<ConnectionInner<Box<dyn Socket>>>,

    msg_receiver: BroadcastReceiver<Arc<Message>>,

    // Receiver side of the error channel
    error_receiver: Receiver<Error>,
}

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

    /// Send `msg` to the peer.
    ///
    /// Unlike our [`Sink`] implementation, this method sets a unique (to this connection) serial
    /// number on the message before sending it off, for you.
    ///
    /// On successfully sending off `msg`, the assigned serial number is returned.
    pub async fn send_message(&self, mut msg: Message) -> Result<u32> {
        let serial = self.assign_serial_num(&mut msg)?;

        (&*self).send(msg).await?;

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
        E: Into<Error>,
    {
        let stream = self.clone();
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
        E: Into<Error>,
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
        self.inner.bus_conn
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
        self.inner.unique_name.get().map(|s| s.as_str())
    }

    /// Max number of messages to queue.
    pub fn max_queued(&self) -> usize {
        self.msg_receiver.capacity()
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
    /// // Do something useful with `conn`..
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn set_max_queued(mut self, max: usize) -> Self {
        self.msg_receiver.set_capacity(max);

        self
    }

    /// The server's GUID.
    pub fn server_guid(&self) -> &str {
        self.inner.server_guid.as_str()
    }

    /// The underlying executor.
    ///
    /// When `internal-executor` feature is disabled, zbus will not spawn thread internally to run
    /// the executor. You're responsible to continuously [tick the executor][tte]. Failure to do so
    /// will result in hangs.
    ///
    /// # Examples
    ///
    /// Here is how one would typically run the zbus executor through tokio's single-threaded
    /// scheduler:
    ///
    /// ```
    /// use zbus::azync::Connection;
    /// use tokio::runtime;
    ///
    ///# #[cfg(not(feature = "internal-executor"))]
    /// runtime::Builder::new_current_thread()
    ///        .build()
    ///        .unwrap()
    ///        .block_on(async {
    ///     let conn = Connection::new_session().await.unwrap();
    ///     {
    ///        let conn = conn.clone();
    ///        tokio::task::spawn(async move {
    ///            loop {
    ///                conn.executor().tick().await;
    ///            }
    ///        });
    ///     }
    ///
    ///     // All your other async code goes here.
    /// });
    /// ```
    ///
    /// [tte]: https://docs.rs/async-executor/1.4.1/async_executor/struct.Executor.html#method.tick
    pub fn executor(&self) -> &Executor<'static> {
        &self.inner.executor
    }

    /// Get the raw file descriptor of this connection.
    pub async fn as_raw_fd(&self) -> RawFd {
        (self.inner.raw_in_conn.lock().await.socket()).as_raw_fd()
    }

    pub(crate) async fn subscribe_signal<E>(
        &self,
        sender: &str,
        path: impl TryInto<ObjectPath<'_>, Error = E>,
        interface: &str,
        signal_name: &str,
    ) -> Result<u64>
    where
        E: Into<Error>,
    {
        let signal = SignalInfo::new(sender, path, interface, signal_name)?;
        let hash = signal.calc_hash();
        let mut subscriptions = self.inner.signal_subscriptions.lock().await;
        match subscriptions.get_mut(&hash) {
            Some(subscription) => subscription.num_subscribers += 1,
            None => {
                let match_rule = signal.create_match_rule();
                if let Some(match_rule) = &match_rule {
                    fdo::AsyncDBusProxy::builder(self)
                        .cache_properties(false)
                        .build_async()
                        .await?
                        .add_match(match_rule)
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

    pub(crate) async fn unsubscribe_signal<E>(
        &self,
        sender: &str,
        path: impl TryInto<ObjectPath<'_>, Error = E>,
        interface: &str,
        signal_name: &str,
    ) -> Result<bool>
    where
        Error: From<E>,
    {
        let signal = SignalInfo::new(sender, path, interface, signal_name)?;
        let hash = signal.calc_hash();

        self.unsubscribe_signal_by_id(hash).await
    }

    pub(crate) async fn unsubscribe_signal_by_id(&self, subscription_id: u64) -> Result<bool> {
        let mut subscriptions = self.inner.signal_subscriptions.lock().await;
        match subscriptions.get_mut(&subscription_id) {
            Some(subscription) => {
                subscription.num_subscribers -= 1;

                if subscription.num_subscribers == 0 {
                    if let Some(match_rule) = &subscription.match_rule {
                        fdo::AsyncDBusProxy::builder(self)
                            .cache_properties(false)
                            .build_async()
                            .await?
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

    pub(crate) fn queue_unsubscribe_signal(&self, subscription_id: u64) {
        let conn = self.clone();
        self.inner
            .executor
            .spawn(async move {
                // FIXME: Ignoring the errors here. We should at least log a message when we've
                //        logging.
                let _ = conn.unsubscribe_signal_by_id(subscription_id).await;
            })
            .detach()
    }

    async fn hello_bus(&self) -> Result<()> {
        let dbus_proxy = fdo::AsyncDBusProxy::builder(self)
            .cache_properties(false)
            .build_async()
            .await?;
        let future = dbus_proxy.hello();

        #[cfg(feature = "internal-executor")]
        let name = future.await?;

        // With external executor, our executor is only run after the connection construction is
        // completed and this method is (and must) run before that so we need to tick the executor
        // ourselves in parallel to making the method call.
        #[cfg(not(feature = "internal-executor"))]
        let name = {
            let executor = self.inner.executor.clone();
            let ticking_future = async move {
                // Keep running as long as this task/future is not cancelled.
                loop {
                    executor.tick().await;
                }
            };

            futures_util::pin_mut!(future);
            futures_util::pin_mut!(ticking_future);

            match select(future, ticking_future).await {
                Either::Left((res, _)) => res?,
                Either::Right((_, _)) => unreachable!("ticking task future shouldn't finish"),
            }
        };

        self.inner
            .unique_name
            .set(name)
            // programmer (probably our) error if this fails.
            .expect("Attempted to set unique_name twice");

        Ok(())
    }

    async fn new(
        auth: Authenticated<Async<Box<dyn Socket>>>,
        bus_connection: bool,
    ) -> Result<Self> {
        let auth = auth.into_inner();
        let out_socket = auth.conn.socket().get_ref().try_clone()?;
        let out_conn = RawConnection::wrap(Async::new(out_socket)?);
        let cap_unix_fd = auth.cap_unix_fd;
        let raw_out_conn = Arc::new(sync::Mutex::new(out_conn));

        let (mut msg_sender, msg_receiver) = broadcast(DEFAULT_MAX_QUEUED);
        msg_sender.set_overflow(true);
        let (error_sender, error_receiver) = bounded(1);
        let executor = Arc::new(Executor::new());
        let raw_in_conn = Arc::new(Mutex::new(auth.conn));

        // Start the message receiver task.
        let msg_receiver_task =
            MessageReceiverTask::new(raw_in_conn.clone(), msg_sender, error_sender)
                .spawn(&executor);

        let connection = Self {
            error_receiver,
            msg_receiver,
            inner: Arc::new(ConnectionInner {
                raw_in_conn,
                raw_out_conn,
                server_guid: auth.server_guid,
                cap_unix_fd,
                bus_conn: bus_connection,
                serial: AtomicU32::new(1),
                unique_name: OnceCell::new(),
                signal_subscriptions: Mutex::new(HashMap::new()),
                executor: executor.clone(),
                msg_receiver_task: sync::Mutex::new(Some(msg_receiver_task)),
            }),
        };

        #[cfg(feature = "internal-executor")]
        std::thread::Builder::new()
            .name("zbus::azync::Connection::receive_msg".into())
            .spawn(move || {
                block_on(async move {
                    // Run as long as there is a task to run.
                    while !executor.is_empty() {
                        executor.tick().await;
                    }
                })
            })?;

        if !bus_connection {
            return Ok(connection);
        }

        // Now that the server has approved us, we must send the bus Hello, as per specs
        connection.hello_bus().await?;

        Ok(connection)
    }

    fn next_serial(&self) -> u32 {
        self.inner.serial.fetch_add(1, SeqCst)
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

impl Sink<Message> for Connection {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut &*self).poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, msg: Message) -> Result<()> {
        Pin::new(&mut &*self).start_send(msg)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut &*self).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        Pin::new(&mut &*self).poll_close(cx)
    }
}

impl<'a> Sink<Message> for &'a Connection {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        // TODO: We should have a max queue length in raw::Socket for outgoing messages.
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, msg: Message) -> Result<()> {
        if !msg.fds().is_empty() && !self.inner.cap_unix_fd {
            return Err(Error::Unsupported);
        }

        self.inner
            .raw_out_conn
            .lock()
            .expect("poisened lock")
            .enqueue_message(msg);

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.inner
            .raw_out_conn
            .lock()
            .expect("poisened lock")
            .flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let mut raw_out_conn = self.inner.raw_out_conn.lock().expect("poisened lock");
        match ready!(raw_out_conn.flush(cx)) {
            Ok(_) => (),
            Err(e) => return Poll::Ready(Err(e)),
        }

        Poll::Ready(raw_out_conn.close())
    }
}

impl stream::Stream for Connection {
    type Item = Result<Arc<Message>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let stream = self.get_mut();
        let msg_fut = stream.msg_receiver.next();
        let err_fut = stream.error_receiver.next();
        let mut select_fut = select(msg_fut, err_fut);

        match ready!(Pin::new(&mut select_fut).poll(cx)) {
            Either::Left((msg, _)) => Poll::Ready(msg.map(Ok)),
            Either::Right((error, _)) => Poll::Ready(error.map(Err)),
        }
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
                    match ready!(stream.raw_conn.socket().poll_readable(cx)) {
                        // Guess socket became ready already so let's try it again.
                        Ok(_) => continue,
                        Err(e) => return Poll::Ready(Err(e.into())),
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
    #[timeout(15000)]
    fn unix_p2p() {
        async_io::block_on(test_unix_p2p()).unwrap();
    }

    async fn test_unix_p2p() -> Result<()> {
        let guid = Guid::generate();

        let (p0, p1) = UnixStream::pair().unwrap();

        let server = Connection::new_unix_server(p0, &guid);
        let client = Connection::new_unix_client(p1, false);

        let (mut client_conn, mut server_conn) = futures_util::try_join!(client, server)?;

        let server_future = async {
            let mut method: Option<Arc<Message>> = None;
            while let Some(m) = server_conn.try_next().await? {
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
            let m = client_conn.try_next().await?.unwrap();
            assert_eq!(m.to_string(), "Signal ASignalForYou");
            reply.body::<String>().map_err(|e| e.into())
        };

        let (val, _) = futures_util::try_join!(client_future, server_future)?;
        assert_eq!(val, "yay");

        Ok(())
    }

    #[test]
    #[timeout(15000)]
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
