use std::{collections::VecDeque, fmt::Debug, marker::PhantomData, ops::Deref};

use crate::{
    guid::Guid,
    raw::{self, Handshake as SyncHandshake, Socket},
    Result,
};

/// Authentication mechanisms
///
/// See <https://dbus.freedesktop.org/doc/dbus-specification.html#auth-mechanisms>
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthMechanism {
    /// This is the recommended authentication mechanism on platforms where credentials can be
    /// transferred out-of-band, in particular Unix platforms that can perform credentials-passing
    /// over the `unix:` transport.
    External,

    /// This mechanism is designed to establish that a client has the ability to read a private file
    /// owned by the user being authenticated.
    Cookie,

    /// Does not perform any authentication at all, and should not be accepted by message buses.
    /// However, it might sometimes be useful for non-message-bus uses of D-Bus.
    Anonymous,
}

/// The asynchronous authentication implementation based on non-blocking [`raw::Handshake`].
///
/// The underlying socket is in nonblocking mode. Enabling blocking mode on it, will lead to
/// undefined behaviour.
pub(crate) struct Authenticated<S>(raw::Authenticated<S>);

impl<S> Authenticated<S> {
    /// Unwraps the inner [`raw::Authenticated`].
    pub fn into_inner(self) -> raw::Authenticated<S> {
        self.0
    }
}

impl<S> Deref for Authenticated<S> {
    type Target = raw::Authenticated<S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> Authenticated<S>
where
    S: Socket + Unpin,
{
    /// Create a client-side `Authenticated` for the given `socket`.
    pub async fn client(socket: S, mechanisms: Option<VecDeque<AuthMechanism>>) -> Result<Self> {
        Handshake {
            handshake: raw::ClientHandshake::new(socket, mechanisms),
            phantom: PhantomData,
        }
        .perform()
        .await
    }

    /// Create a server-side `Authenticated` for the given `socket`.
    ///
    /// The function takes `client_uid` on Unix only. On Windows, it takes `client_sid` instead.
    pub async fn server(
        socket: S,
        guid: Guid,
        #[cfg(unix)] client_uid: Option<u32>,
        #[cfg(windows)] client_sid: Option<String>,
        auth_mechanisms: Option<VecDeque<AuthMechanism>>,
    ) -> Result<Self> {
        Handshake {
            handshake: raw::ServerHandshake::new(
                socket,
                guid,
                #[cfg(unix)]
                client_uid,
                #[cfg(windows)]
                client_sid,
                auth_mechanisms,
            )?,
            phantom: PhantomData,
        }
        .perform()
        .await
    }
}

struct Handshake<H, S> {
    handshake: H,
    phantom: PhantomData<S>,
}

impl<H, S> Handshake<H, S>
where
    H: SyncHandshake<S> + Unpin + Debug,
    S: Unpin,
{
    async fn perform(mut self) -> Result<Authenticated<S>> {
        self.handshake.advance_handshake().await?;

        let authenticated = self
            .handshake
            .try_finish()
            .expect("Failed to finish a successful handshake");

        Ok(Authenticated(authenticated))
    }
}
