use async_io::Async;

use std::{
    fmt::Debug,
    future::Future,
    marker::PhantomData,
    ops::Deref,
    pin::Pin,
    str::FromStr,
    task::{Context, Poll},
};

use crate::{
    address::Address,
    guid::Guid,
    handshake::{self, Handshake as SyncHandshake, IoOperation},
    raw::Socket,
    Error, Result,
};

/// The asynchronous sibling of [`handshake::Handshake`].
///
/// The underlying socket is in nonblocking mode. Enabling blocking mode on it, will lead to
/// undefined behaviour.
pub(crate) struct Authenticated<S>(handshake::Authenticated<S>);

impl<S> Authenticated<S>
where
    S: Socket,
{
    /// Unwraps the inner [`handshake::Authenticated`].
    pub fn into_inner(self) -> handshake::Authenticated<S> {
        self.0
    }
}

impl<S> Deref for Authenticated<S> {
    type Target = handshake::Authenticated<S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> Authenticated<Async<S>>
where
    S: Debug + Unpin,
    Async<S>: Socket,
{
    /// Create a client-side `Authenticated` for the given `socket`.
    pub async fn client(socket: Async<S>) -> Result<Self> {
        Handshake {
            handshake: Some(handshake::ClientHandshake::new(socket)),
            phantom: PhantomData,
        }
        .await
    }

    /// Create a server-side `Authenticated` for the given `socket`.
    pub async fn server(socket: Async<S>, guid: Guid, client_uid: u32) -> Result<Self> {
        Handshake {
            handshake: Some(handshake::ServerHandshake::new(socket, guid, client_uid)),
            phantom: PhantomData,
        }
        .await
    }
}

impl Authenticated<Async<Box<dyn Socket>>> {
    /// Create a `Authenticated` for the session/user message bus.
    pub async fn session() -> Result<Self> {
        Self::client(Address::session()?.connect_async().await?.into_boxed()?).await
    }

    /// Create a `Authenticated` for the system-wide message bus.
    pub async fn system() -> Result<Self> {
        Self::client(Address::system()?.connect_async().await?.into_boxed()?).await
    }

    /// Create a `Authenticated` for the given [D-Bus address].
    ///
    /// [D-Bus address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub async fn for_address(address: &str) -> Result<Self> {
        Self::client(
            Address::from_str(address)?
                .connect_async()
                .await?
                .into_boxed()?,
        )
        .await
    }
}

struct Handshake<H, S> {
    handshake: Option<H>,
    phantom: PhantomData<S>,
}

impl<H, S> Future for Handshake<H, S>
where
    H: SyncHandshake<Async<S>> + Unpin + Debug,
    S: Unpin,
{
    type Output = Result<Authenticated<Async<S>>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let self_mut = &mut self.get_mut();
        let handshake = self_mut
            .handshake
            .as_mut()
            .expect("ClientHandshake::poll() called unexpectedly");

        loop {
            match handshake.advance_handshake() {
                Ok(()) => {
                    let handshake = self_mut
                        .handshake
                        .take()
                        .expect("<Handshake as Future>::poll() called unexpectedly");
                    let authenticated = handshake
                        .try_finish()
                        .expect("Failed to finish a successfull handshake");

                    return Poll::Ready(Ok(Authenticated(authenticated)));
                }
                Err(Error::Io(e)) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        let poll = match handshake.next_io_operation() {
                            IoOperation::Read => handshake.socket().poll_readable(cx),
                            IoOperation::Write => handshake.socket().poll_writable(cx),
                            IoOperation::None => panic!("Invalid handshake state"),
                        };
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
                Err(e) => return Poll::Ready(Err(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use nix::unistd::Uid;
    use std::os::unix::net::UnixStream;

    use super::*;

    use crate::{Guid, Result};

    #[test]
    fn async_handshake() {
        futures_executor::block_on(handshake()).unwrap();
    }

    async fn handshake() -> Result<()> {
        // a pair of non-blocking connection UnixStream
        let (p0, p1) = UnixStream::pair()?;

        // initialize both handshakes
        let client = Authenticated::client(Async::new(p0)?);
        let server =
            Authenticated::server(Async::new(p1)?, Guid::generate(), Uid::current().into());

        // proceed to the handshakes
        let (client_auth, server_auth) = futures_util::try_join!(client, server)?;

        assert_eq!(client_auth.server_guid, server_auth.server_guid);
        assert_eq!(client_auth.cap_unix_fd, server_auth.cap_unix_fd);

        Ok(())
    }
}
