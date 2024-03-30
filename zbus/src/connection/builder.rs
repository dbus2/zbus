#[cfg(not(feature = "tokio"))]
use async_io::Async;
use event_listener::Event;
use static_assertions::assert_impl_all;
#[cfg(not(feature = "tokio"))]
use std::net::TcpStream;
#[cfg(all(unix, not(feature = "tokio")))]
use std::os::unix::net::UnixStream;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::Arc,
};
#[cfg(feature = "tokio")]
use tokio::net::TcpStream;
#[cfg(all(unix, feature = "tokio"))]
use tokio::net::UnixStream;
#[cfg(feature = "tokio-vsock")]
use tokio_vsock::VsockStream;
#[cfg(all(windows, not(feature = "tokio")))]
use uds_windows::UnixStream;
#[cfg(all(feature = "vsock", not(feature = "tokio")))]
use vsock::VsockStream;

use zvariant::{ObjectPath, Str};

#[cfg(feature = "p2p")]
use crate::Guid;
use crate::{
    address::{self, Address},
    async_lock::RwLock,
    names::{InterfaceName, WellKnownName},
    object_server::Interface,
    Connection, Error, Executor, OwnedGuid, Result,
};

use super::{
    handshake::{AuthMechanism, Authenticated},
    socket::{BoxedSplit, ReadHalf, Split, WriteHalf},
};

const DEFAULT_MAX_QUEUED: usize = 64;

#[derive(Debug)]
enum Target {
    #[cfg(any(unix, not(feature = "tokio")))]
    UnixStream(UnixStream),
    TcpStream(TcpStream),
    #[cfg(any(
        all(feature = "vsock", not(feature = "tokio")),
        feature = "tokio-vsock"
    ))]
    VsockStream(VsockStream),
    Address(Address),
    Socket(Split<Box<dyn ReadHalf>, Box<dyn WriteHalf>>),
}

type Interfaces<'a> =
    HashMap<ObjectPath<'a>, HashMap<InterfaceName<'static>, Arc<RwLock<dyn Interface>>>>;

/// A builder for [`zbus::Connection`].
#[derive(derivative::Derivative)]
#[derivative(Debug)]
#[must_use]
pub struct Builder<'a> {
    target: Option<Target>,
    max_queued: Option<usize>,
    // This is only set for p2p server case.
    #[cfg(feature = "p2p")]
    guid: Option<Guid<'a>>,
    #[cfg(feature = "p2p")]
    p2p: bool,
    internal_executor: bool,
    #[derivative(Debug = "ignore")]
    interfaces: Interfaces<'a>,
    names: HashSet<WellKnownName<'a>>,
    auth_mechanisms: Option<VecDeque<AuthMechanism>>,
    #[cfg(feature = "bus-impl")]
    unique_name: Option<crate::names::UniqueName<'a>>,
    cookie_context: Option<super::handshake::CookieContext<'a>>,
    cookie_id: Option<usize>,
}

assert_impl_all!(Builder<'_>: Send, Sync, Unpin);

impl<'a> Builder<'a> {
    /// Create a builder for the session/user message bus connection.
    pub fn session() -> Result<Self> {
        Ok(Self::new(Target::Address(Address::session()?)))
    }

    /// Create a builder for the system-wide message bus connection.
    pub fn system() -> Result<Self> {
        Ok(Self::new(Target::Address(Address::system()?)))
    }

    /// Create a builder for connection that will use the given [D-Bus bus address].
    ///
    /// # Example
    ///
    /// Here is an example of connecting to an IBus service:
    ///
    /// ```no_run
    /// # use std::error::Error;
    /// # use zbus::connection::Builder;
    /// # use zbus::block_on;
    /// #
    /// # block_on(async {
    /// let addr = "unix:\
    ///     path=/home/zeenix/.cache/ibus/dbus-ET0Xzrk9,\
    ///     guid=fdd08e811a6c7ebe1fef0d9e647230da";
    /// let conn = Builder::address(addr)?
    ///     .build()
    ///     .await?;
    ///
    /// // Do something useful with `conn`..
    /// #     drop(conn);
    /// #     Ok::<(), zbus::Error>(())
    /// # }).unwrap();
    /// #
    /// # Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    ///
    /// **Note:** The IBus address is different for each session. You can find the address for your
    /// current session using `ibus address` command.
    ///
    /// [D-Bus bus address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub fn address<A>(address: A) -> Result<Self>
    where
        A: TryInto<Address>,
        A::Error: Into<Error>,
    {
        Ok(Self::new(Target::Address(
            address.try_into().map_err(Into::into)?,
        )))
    }

    /// Create a builder for connection that will use the given unix stream.
    ///
    /// If the default `async-io` feature is disabled, this method will expect
    /// [`tokio::net::UnixStream`](https://docs.rs/tokio/latest/tokio/net/struct.UnixStream.html)
    /// argument.
    ///
    /// Since tokio currently [does not support Unix domain sockets][tuds] on Windows, this method
    /// is not available when the `tokio` feature is enabled and building for Windows target.
    ///
    /// [tuds]: https://github.com/tokio-rs/tokio/issues/2201
    #[cfg(any(unix, not(feature = "tokio")))]
    pub fn unix_stream(stream: UnixStream) -> Self {
        Self::new(Target::UnixStream(stream))
    }

    /// Create a builder for connection that will use the given TCP stream.
    ///
    /// If the default `async-io` feature is disabled, this method will expect
    /// [`tokio::net::TcpStream`](https://docs.rs/tokio/latest/tokio/net/struct.TcpStream.html)
    /// argument.
    pub fn tcp_stream(stream: TcpStream) -> Self {
        Self::new(Target::TcpStream(stream))
    }

    /// Create a builder for connection that will use the given VSOCK stream.
    ///
    /// This method is only available when either `vsock` or `tokio-vsock` feature is enabled. The
    /// type of `stream` is `vsock::VsockStream` with `vsock` feature and `tokio_vsock::VsockStream`
    /// with `tokio-vsock` feature.
    #[cfg(any(
        all(feature = "vsock", not(feature = "tokio")),
        feature = "tokio-vsock"
    ))]
    pub fn vsock_stream(stream: VsockStream) -> Self {
        Self::new(Target::VsockStream(stream))
    }

    /// Create a builder for connection that will use the given socket.
    pub fn socket<S: Into<BoxedSplit>>(socket: S) -> Self {
        Self::new(Target::Socket(socket.into()))
    }

    /// Specify the mechanisms to use during authentication.
    pub fn auth_mechanisms(mut self, auth_mechanisms: &[AuthMechanism]) -> Self {
        self.auth_mechanisms = Some(VecDeque::from(auth_mechanisms.to_vec()));

        self
    }

    /// The cookie context to use during authentication.
    ///
    /// This is only used when the `cookie` authentication mechanism is enabled and only valid for
    /// server connection.
    ///
    /// If not specified, the default cookie context of `org_freedesktop_general` will be used.
    ///
    /// # Errors
    ///
    /// If the given string is not a valid cookie context.
    pub fn cookie_context<C>(mut self, context: C) -> Result<Self>
    where
        C: Into<Str<'a>>,
    {
        self.cookie_context = Some(context.into().try_into()?);

        Ok(self)
    }

    /// The ID of the cookie to use during authentication.
    ///
    /// This is only used when the `cookie` authentication mechanism is enabled and only valid for
    /// server connection.
    ///
    /// If not specified, the first cookie found in the cookie context file will be used.
    pub fn cookie_id(mut self, id: usize) -> Self {
        self.cookie_id = Some(id);

        self
    }

    /// The to-be-created connection will be a peer-to-peer connection.
    ///
    /// This method is only available when the `p2p` feature is enabled.
    #[cfg(feature = "p2p")]
    pub fn p2p(mut self) -> Self {
        self.p2p = true;

        self
    }

    /// The to-be-created connection will be a server using the given GUID.
    ///
    /// The to-be-created connection will wait for incoming client authentication handshake and
    /// negotiation messages, for peer-to-peer communications after successful creation.
    ///
    /// This method is only available when the `p2p` feature is enabled.
    #[cfg(feature = "p2p")]
    pub fn server<G>(mut self, guid: G) -> Result<Self>
    where
        G: TryInto<Guid<'a>>,
        G::Error: Into<Error>,
    {
        self.guid = Some(guid.try_into().map_err(Into::into)?);

        Ok(self)
    }

    /// Set the capacity of the main (unfiltered) queue.
    ///
    /// Since typically you'd want to set this at instantiation time, you can set it through the
    /// builder.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::error::Error;
    /// # use zbus::connection::Builder;
    /// # use zbus::block_on;
    /// #
    /// # block_on(async {
    /// let conn = Builder::session()?
    ///     .max_queued(30)
    ///     .build()
    ///     .await?;
    /// assert_eq!(conn.max_queued(), 30);
    ///
    /// #     Ok::<(), zbus::Error>(())
    /// # }).unwrap();
    /// #
    /// // Do something useful with `conn`..
    /// # Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn max_queued(mut self, max: usize) -> Self {
        self.max_queued = Some(max);

        self
    }

    /// Enable or disable the internal executor thread.
    ///
    /// The thread is enabled by default.
    ///
    /// See [Connection::executor] for more details.
    pub fn internal_executor(mut self, enabled: bool) -> Self {
        self.internal_executor = enabled;

        self
    }

    /// Register a D-Bus [`Interface`] to be served at a given path.
    ///
    /// This is similar to [`zbus::ObjectServer::at`], except that it allows you to have your
    /// interfaces available immediately after the connection is established. Typically, this is
    /// exactly what you'd want. Also in contrast to [`zbus::ObjectServer::at`], this method will
    /// replace any previously added interface with the same name at the same path.
    pub fn serve_at<P, I>(mut self, path: P, iface: I) -> Result<Self>
    where
        I: Interface,
        P: TryInto<ObjectPath<'a>>,
        P::Error: Into<Error>,
    {
        let path = path.try_into().map_err(Into::into)?;
        let entry = self.interfaces.entry(path).or_default();
        entry.insert(I::name(), Arc::new(RwLock::new(iface)));

        Ok(self)
    }

    /// Register a well-known name for this connection on the bus.
    ///
    /// This is similar to [`zbus::Connection::request_name`], except the name is requested as part
    /// of the connection setup ([`Builder::build`]), immediately after interfaces
    /// registered (through [`Builder::serve_at`]) are advertised. Typically this is
    /// exactly what you want.
    pub fn name<W>(mut self, well_known_name: W) -> Result<Self>
    where
        W: TryInto<WellKnownName<'a>>,
        W::Error: Into<Error>,
    {
        let well_known_name = well_known_name.try_into().map_err(Into::into)?;
        self.names.insert(well_known_name);

        Ok(self)
    }

    /// Sets the unique name of the connection.
    ///
    /// This is mainly provided for bus implementations. All other users should not need to use this
    /// method. Hence why this method is only available when the `bus-impl` feature is enabled.
    ///
    /// # Panics
    ///
    /// It will panic if the connection is to a message bus as it's the bus that assigns
    /// peers their unique names.
    #[cfg(feature = "bus-impl")]
    pub fn unique_name<U>(mut self, unique_name: U) -> Result<Self>
    where
        U: TryInto<crate::names::UniqueName<'a>>,
        U::Error: Into<Error>,
    {
        if !self.p2p {
            panic!("unique name can only be set for peer-to-peer connections");
        }
        let name = unique_name.try_into().map_err(Into::into)?;
        self.unique_name = Some(name);

        Ok(self)
    }

    /// Build the connection, consuming the builder.
    ///
    /// # Errors
    ///
    /// Until server-side bus connection is supported, attempting to build such a connection will
    /// result in [`Error::Unsupported`] error.
    pub async fn build(self) -> Result<Connection> {
        let executor = Executor::new();
        #[cfg(not(feature = "tokio"))]
        let internal_executor = self.internal_executor;
        // Box the future as it's large and can cause stack overflow.
        let conn = Box::pin(executor.run(self.build_(executor.clone()))).await?;

        #[cfg(not(feature = "tokio"))]
        start_internal_executor(&executor, internal_executor)?;

        Ok(conn)
    }

    async fn build_(mut self, executor: Executor<'static>) -> Result<Connection> {
        #[allow(unused_mut)]
        let (mut stream, server_guid) = self.target_connect().await?;
        #[cfg(feature = "p2p")]
        let mut auth = match self.guid {
            None => {
                // SASL Handshake
                Authenticated::client(stream, server_guid, self.auth_mechanisms).await?
            }
            Some(guid) => {
                if !self.p2p {
                    return Err(Error::Unsupported);
                }

                let creds = stream.read_mut().peer_credentials().await?;
                #[cfg(unix)]
                let client_uid = creds.unix_user_id();
                #[cfg(windows)]
                let client_sid = creds.into_windows_sid();

                Authenticated::server(
                    stream,
                    guid.to_owned().into(),
                    #[cfg(unix)]
                    client_uid,
                    #[cfg(windows)]
                    client_sid,
                    self.auth_mechanisms,
                    self.cookie_id,
                    self.cookie_context.unwrap_or_default(),
                )
                .await?
            }
        };

        #[cfg(not(feature = "p2p"))]
        let mut auth = Authenticated::client(stream, server_guid, self.auth_mechanisms).await?;

        // SAFETY: `Authenticated` is always built with these fields set to `Some`.
        let socket_read = auth.socket_read.take().unwrap();
        let already_received_bytes = auth.already_received_bytes.take().unwrap();

        #[cfg(feature = "p2p")]
        let is_bus_conn = !self.p2p;
        #[cfg(not(feature = "p2p"))]
        let is_bus_conn = true;
        let mut conn = Connection::new(auth, is_bus_conn, executor).await?;
        conn.set_max_queued(self.max_queued.unwrap_or(DEFAULT_MAX_QUEUED));
        #[cfg(feature = "bus-impl")]
        if let Some(unique_name) = self.unique_name {
            conn.set_unique_name(unique_name)?;
        }

        if !self.interfaces.is_empty() {
            let object_server = conn.sync_object_server(false, None);
            for (path, interfaces) in &self.interfaces {
                for (name, iface) in interfaces {
                    let iface = iface.clone();
                    let future =
                        object_server
                            .inner()
                            .at_ready(path.to_owned(), name.clone(), || iface);
                    let added = future.await?;
                    // Duplicates shouldn't happen.
                    assert!(added);
                }
            }

            let started_event = Event::new();
            let listener = started_event.listen();
            conn.start_object_server(Some(started_event));

            listener.await;
        }

        // Start the socket reader task.
        conn.init_socket_reader(socket_read, already_received_bytes);

        if is_bus_conn {
            // Now that the server has approved us, we must send the bus Hello, as per specs
            conn.hello_bus().await?;
        }

        for name in self.names {
            conn.request_name(name).await?;
        }

        // Now that `Hello` is done, we can emit the ObjectManager signals.
        if !self.interfaces.is_empty() {
            let object_server = conn.sync_object_server(false, None);
            for (path, interfaces) in self.interfaces {
                for name in interfaces.into_keys() {
                    let future = object_server
                        .inner()
                        .emit_object_manager_signals(path.to_owned(), name);
                    future.await?;
                }
            }
        }

        Ok(conn)
    }

    fn new(target: Target) -> Self {
        Self {
            target: Some(target),
            #[cfg(feature = "p2p")]
            p2p: false,
            max_queued: None,
            #[cfg(feature = "p2p")]
            guid: None,
            internal_executor: true,
            interfaces: HashMap::new(),
            names: HashSet::new(),
            auth_mechanisms: None,
            #[cfg(feature = "bus-impl")]
            unique_name: None,
            cookie_id: None,
            cookie_context: None,
        }
    }

    async fn target_connect(&mut self) -> Result<(BoxedSplit, Option<OwnedGuid>)> {
        // SAFETY: `self.target` is always `Some` from the beginning and this method is only called
        // once.
        let split = match self.target.take().unwrap() {
            #[cfg(not(feature = "tokio"))]
            Target::UnixStream(stream) => Async::new(stream)?.into(),
            #[cfg(all(unix, feature = "tokio"))]
            Target::UnixStream(stream) => stream.into(),
            #[cfg(not(feature = "tokio"))]
            Target::TcpStream(stream) => Async::new(stream)?.into(),
            #[cfg(feature = "tokio")]
            Target::TcpStream(stream) => stream.into(),
            #[cfg(all(feature = "vsock", not(feature = "tokio")))]
            Target::VsockStream(stream) => Async::new(stream)?.into(),
            #[cfg(feature = "tokio-vsock")]
            Target::VsockStream(stream) => stream.into(),
            Target::Address(address) => {
                let guid = address.guid().map(|g| g.to_owned().into());
                let split = match address.connect().await? {
                    #[cfg(any(unix, not(feature = "tokio")))]
                    address::transport::Stream::Unix(stream) => stream.into(),
                    address::transport::Stream::Tcp(stream) => stream.into(),
                    #[cfg(any(
                        all(feature = "vsock", not(feature = "tokio")),
                        feature = "tokio-vsock"
                    ))]
                    address::transport::Stream::Vsock(stream) => stream.into(),
                };
                return Ok((split, guid));
            }
            Target::Socket(stream) => stream,
        };

        Ok((split, None))
    }
}

/// Start the internal executor thread.
///
/// Returns a dummy task that keep the executor ticking thread from exiting due to absence of any
/// tasks until socket reader task kicks in.
#[cfg(not(feature = "tokio"))]
fn start_internal_executor(executor: &Executor<'static>, internal_executor: bool) -> Result<()> {
    use core::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    };

    /// A future that ends once the executor is empty.
    struct Empty<'a>(&'a Executor<'static>);

    impl Future for Empty<'_> {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.0.is_empty() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    if internal_executor {
        let executor = executor.clone();
        std::thread::Builder::new()
            .name("zbus::Connection executor".into())
            .spawn(move || {
                crate::utils::block_on(async move {
                    // Run as long as there is a task to run.
                    executor.run(Empty(&executor)).await
                })
            })?;
    }

    Ok(())
}
