use async_io::Async;
use async_lock::RwLock;
use static_assertions::assert_impl_all;
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    os::unix::net::UnixStream,
    sync::Arc,
};
use zvariant::ObjectPath;

use crate::{
    address::{self, Address},
    names::{InterfaceName, WellKnownName},
    raw::Socket,
    Authenticated, Connection, Error, Guid, Interface, Result,
};

const DEFAULT_MAX_QUEUED: usize = 64;

#[derive(Debug)]
enum Target {
    UnixStream(UnixStream),
    Address(Address),
    Socket(Box<dyn Socket>),
}

type Interfaces<'a> =
    HashMap<ObjectPath<'a>, HashMap<InterfaceName<'static>, Arc<RwLock<dyn Interface>>>>;

/// A builder for [`zbus::Connection`].
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct ConnectionBuilder<'a> {
    target: Target,
    max_queued: Option<usize>,
    guid: Option<&'a Guid>,
    p2p: bool,
    internal_executor: bool,
    #[derivative(Debug = "ignore")]
    interfaces: Interfaces<'a>,
    names: HashSet<WellKnownName<'a>>,
}

assert_impl_all!(ConnectionBuilder<'_>: Send, Sync, Unpin);

impl<'a> ConnectionBuilder<'a> {
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
    pub fn unix_stream(stream: UnixStream) -> Self {
        Self::new(Target::UnixStream(stream))
    }

    /// Create a builder for connection that will use the given socket.
    pub fn socket<S: Socket + 'static>(socket: S) -> Self {
        Self::new(Target::Socket(Box::new(socket)))
    }

    /// The to-be-created connection will be a peer-to-peer connection.
    pub fn p2p(mut self) -> Self {
        self.p2p = true;

        self
    }

    /// The to-be-created connection will a be server using the given GUID.
    ///
    /// The to-be-created connection will wait for incoming client authentication handshake and
    /// negotiation messages, for peer-to-peer communications after successful creation.
    pub fn server(mut self, guid: &'a Guid) -> Self {
        self.guid = Some(guid);

        self
    }

    /// Set the max number of messages to queue.
    ///
    /// Since typically you'd want to set this at instantiation time, you can set it through the builder.
    ///
    /// # Example
    ///
    /// ```
    ///# use std::error::Error;
    ///# use zbus::ConnectionBuilder;
    ///# use async_io::block_on;
    ///#
    ///# block_on(async {
    /// let conn = ConnectionBuilder::session()?
    ///     .max_queued(30)
    ///     .build()
    ///     .await?;
    /// assert_eq!(conn.max_queued(), 30);
    ///
    ///#     Ok::<(), zbus::Error>(())
    ///# });
    ///#
    /// // Do something useful with `conn`..
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
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
    /// This is similar to [`zbus::Connection::register_name`], except the name is requested as part
    /// of the connection setup ([`ConnectionBuilder::build`]), immediately after interfaces
    /// registered (through [`ConnectionBuilder::serve_at`]) are advertised. Typically this is
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

    /// Build the connection, consuming the builder.
    ///
    /// # Errors
    ///
    /// Until server-side bus connection is supported, attempting to build such a connection will
    /// result in [`Error::Unsupported`] error.
    pub async fn build(self) -> Result<Connection> {
        let stream = match self.target {
            Target::UnixStream(stream) => Box::new(Async::new(stream)?),
            Target::Address(address) => match address.connect().await? {
                address::Stream::Unix(stream) => Box::new(Async::new(stream.into_inner()?)?),
            },
            Target::Socket(stream) => stream,
        };
        let auth = match self.guid {
            None => {
                // SASL Handshake
                Authenticated::client(stream).await?
            }
            Some(guid) => {
                if !self.p2p {
                    return Err(Error::Unsupported);
                }

                #[cfg(any(target_os = "android", target_os = "linux"))]
                let client_uid = {
                    use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

                    let creds = getsockopt(stream.as_raw_fd(), PeerCredentials).map_err(|e| {
                        Error::Handshake(format!("Failed to get peer credentials: {}", e))
                    })?;

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
                    .map_err(|e| {
                        Error::Handshake(format!("Failed to get peer credentials: {}", e))
                    })?
                    .0
                    .into();

                Authenticated::server(stream, guid.clone(), client_uid).await?
            }
        };

        let mut conn = Connection::new(auth, !self.p2p, self.internal_executor).await?;
        conn.set_max_queued(self.max_queued.unwrap_or(DEFAULT_MAX_QUEUED));

        if !self.interfaces.is_empty() {
            let mut object_server = conn.sync_object_server_mut(false).await;
            for (path, interfaces) in self.interfaces {
                for (name, iface) in interfaces {
                    // FIXME: Log warning message on `at` returning `false`.
                    let added = object_server.at_ready(path.clone(), name, iface)?;
                    // Duplicates shouldn't happen.
                    assert!(added);
                }
            }

            conn.start_object_server();
        }

        for name in self.names {
            conn.request_name(name).await?;
        }

        Ok(conn)
    }

    fn new(target: Target) -> Self {
        Self {
            target,
            p2p: false,
            max_queued: None,
            guid: None,
            internal_executor: true,
            interfaces: HashMap::new(),
            names: HashSet::new(),
        }
    }
}

#[cfg(feature = "tokio")]
#[test]
fn tokio_socket() {
    use std::error::Error;
    use tokio::net::UnixStream;
    use zbus::Address;
    async fn run() -> std::result::Result<(), Box<dyn Error>> {
        let stream = match Address::session()? {
            Address::Unix(s) => UnixStream::connect(s).await?,
        };
        let conn = ConnectionBuilder::socket(stream)
            .internal_executor(false)
            .build()
            .await?;
        let executor_conn = conn.clone();
        tokio::task::spawn(async move {
            loop {
                executor_conn.executor().tick().await;
            }
        });
        let proxy = zbus::fdo::DBusProxy::new(&conn).await?;
        proxy.features().await?;
        Ok(())
    }
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    tokio::task::LocalSet::new().block_on(&rt, run()).unwrap();
}
