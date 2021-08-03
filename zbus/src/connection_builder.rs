use async_io::block_on;
use static_assertions::assert_impl_all;
use std::{convert::TryInto, os::unix::net::UnixStream};

use crate::{address::Address, azync, Connection, Error, Guid, Result};

/// A builder for [`zbus::Connection`].
#[derive(Debug)]
pub struct ConnectionBuilder<'a>(azync::ConnectionBuilder<'a>);

assert_impl_all!(ConnectionBuilder<'_>: Send, Sync, Unpin);

impl<'a> ConnectionBuilder<'a> {
    /// Create a builder for the session/user message bus connection.
    pub fn session() -> Result<Self> {
        azync::ConnectionBuilder::session().map(Self)
    }

    /// Create a builder for the system-wide message bus connection.
    pub fn system() -> Result<Self> {
        azync::ConnectionBuilder::system().map(Self)
    }

    /// Create a builder for connection that will use the given [D-Bus bus address].
    ///
    /// [D-Bus bus address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub fn address<A>(address: A) -> Result<Self>
    where
        A: TryInto<Address>,
        A::Error: Into<Error>,
    {
        azync::ConnectionBuilder::address(address).map(Self)
    }

    /// Create a builder for connection that will use the given unix stream.
    pub fn unix_stream(stream: UnixStream) -> Self {
        Self(azync::ConnectionBuilder::unix_stream(stream))
    }

    /// The to-be-created connection will be a peer-to-peer connection.
    pub fn p2p(self) -> Self {
        Self(self.0.p2p())
    }

    /// The to-be-created connection will a be server using the given GUID.
    ///
    /// The to-be-created connection will wait for incoming client authentication handshake and
    /// negotiation messages, for peer-to-peer communications after successful creation.
    pub fn server(self, guid: &'a Guid) -> Self {
        Self(self.0.server(guid))
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
    ///#
    /// let conn = ConnectionBuilder::session()?
    ///     .max_queued(30)
    ///     .build()?;
    /// assert_eq!(conn.max_queued(), 30);
    ///
    /// // Do something useful with `conn`..
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn max_queued(self, max: usize) -> Self {
        Self(self.0.max_queued(max))
    }

    /// Build the connection, consuming the builder.
    ///
    /// # Errors
    ///
    /// Until server-side bus connection is supported, attempting to build such a connection will
    /// result in [`Error::Unsupported`] error.
    pub fn build(self) -> Result<Connection> {
        block_on(self.0.build()).map(Into::into)
    }
}
