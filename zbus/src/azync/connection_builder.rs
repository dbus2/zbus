use async_io::Async;
use static_assertions::assert_impl_all;
use std::{
    convert::TryInto,
    os::unix::{io::AsRawFd, net::UnixStream},
};

use crate::{
    address::{self, Address},
    azync::{Authenticated, Connection},
    raw::Socket,
    Error, Guid, Result,
};

const DEFAULT_MAX_QUEUED: usize = 64;

#[derive(Debug)]
enum Target {
    UnixStream(UnixStream),
    Address(Address),
}

/// A builder for [`zbus::azync::Connection`].
#[derive(Debug)]
pub struct ConnectionBuilder<'a> {
    target: Target,
    max_queued: Option<usize>,
    guid: Option<&'a Guid>,
    p2p: bool,
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
    ///# use zbus::azync::ConnectionBuilder;
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

    /// Build the connection, consuming the builder.
    ///
    /// # Errors
    ///
    /// Until server-side bus connection is supported, attempting to build such a connection will
    /// result in [`Error::Unsupported`] error.
    pub async fn build(self) -> Result<Connection> {
        let stream = match self.target {
            Target::UnixStream(stream) => stream,
            Target::Address(address) => match address.connect().await? {
                address::Stream::Unix(stream) => stream.into_inner()?,
            },
        };
        let auth = match self.guid {
            None => {
                // SASL Handshake
                Authenticated::client(Box::new(Async::new(stream)?) as Box<dyn Socket>).await?
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

                Authenticated::server(
                    Box::new(Async::new(stream)?) as Box<dyn Socket>,
                    guid.clone(),
                    client_uid,
                )
                .await?
            }
        };

        let mut conn = Connection::new(auth, !self.p2p).await?;
        conn.set_max_queued(self.max_queued.unwrap_or(DEFAULT_MAX_QUEUED));

        Ok(conn)
    }

    fn new(target: Target) -> Self {
        Self {
            target,
            p2p: false,
            max_queued: None,
            guid: None,
        }
    }
}
