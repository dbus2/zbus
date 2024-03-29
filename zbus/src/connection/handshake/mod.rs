mod auth_mechanism;
mod client;
mod command;
#[cfg(feature = "p2p")]
mod server;

use async_trait::async_trait;
use futures_util::StreamExt;
#[cfg(unix)]
use nix::unistd::Uid;
use std::{collections::VecDeque, fmt::Debug, path::PathBuf};
use tracing::{instrument, trace};
use zvariant::Str;

use xdg_home::home_dir;

#[cfg(windows)]
use crate::win32;
use crate::{file::FileLines, Error, OwnedGuid, Result};

use super::socket::{BoxedSplit, ReadHalf, WriteHalf};

pub use auth_mechanism::AuthMechanism;
use client::ClientHandshake;
use command::Command;
#[cfg(feature = "p2p")]
use server::ServerHandshake;

/// The result of a finalized handshake
///
/// The result of a finalized [`ClientHandshake`] or [`ServerHandshake`].
///
/// [`ClientHandshake`]: struct.ClientHandshake.html
/// [`ServerHandshake`]: struct.ServerHandshake.html
#[derive(Debug)]
pub struct Authenticated {
    pub(crate) socket_write: Box<dyn WriteHalf>,
    /// The server Guid
    pub(crate) server_guid: OwnedGuid,
    /// Whether file descriptor passing has been accepted by both sides
    #[cfg(unix)]
    pub(crate) cap_unix_fd: bool,

    pub(crate) socket_read: Option<Box<dyn ReadHalf>>,
    pub(crate) already_received_bytes: Option<Vec<u8>>,
}

impl Authenticated {
    /// Create a client-side `Authenticated` for the given `socket`.
    pub async fn client(
        socket: BoxedSplit,
        server_guid: Option<OwnedGuid>,
        mechanisms: Option<VecDeque<AuthMechanism>>,
    ) -> Result<Self> {
        ClientHandshake::new(socket, mechanisms, server_guid)
            .perform()
            .await
    }

    /// Create a server-side `Authenticated` for the given `socket`.
    ///
    /// The function takes `client_uid` on Unix only. On Windows, it takes `client_sid` instead.
    #[cfg(feature = "p2p")]
    pub async fn server(
        socket: BoxedSplit,
        guid: OwnedGuid,
        #[cfg(unix)] client_uid: Option<u32>,
        #[cfg(windows)] client_sid: Option<String>,
        auth_mechanisms: Option<VecDeque<AuthMechanism>>,
        cookie_id: Option<usize>,
        cookie_context: CookieContext<'_>,
    ) -> Result<Self> {
        ServerHandshake::new(
            socket,
            guid,
            #[cfg(unix)]
            client_uid,
            #[cfg(windows)]
            client_sid,
            auth_mechanisms,
            cookie_id,
            cookie_context,
        )?
        .perform()
        .await
    }
}

#[async_trait]
pub trait Handshake {
    /// Perform the handshake.
    ///
    /// On a successful handshake, you get an `Authenticated`. If you need to send a Bus Hello,
    /// this remains to be done.
    async fn perform(mut self) -> Result<Authenticated>;
}

fn random_ascii(len: usize) -> String {
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    use std::iter;

    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

fn sasl_auth_id() -> Result<String> {
    let id = {
        #[cfg(unix)]
        {
            Uid::effective().to_string()
        }

        #[cfg(windows)]
        {
            win32::ProcessToken::open(None)?.sid()?
        }
    };

    Ok(id)
}

#[derive(Debug)]
struct Cookie {
    id: usize,
    cookie: String,
}

impl Cookie {
    fn keyring_path() -> Result<PathBuf> {
        let mut path = home_dir()
            .ok_or_else(|| Error::Handshake("Failed to determine home directory".into()))?;
        path.push(".dbus-keyrings");
        Ok(path)
    }

    async fn read_keyring(context: &CookieContext<'_>) -> Result<Vec<Cookie>> {
        let mut path = Cookie::keyring_path()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let perms = crate::file::metadata(&path).await?.permissions().mode();
            if perms & 0o066 != 0 {
                return Err(Error::Handshake(
                    "DBus keyring has invalid permissions".into(),
                ));
            }
        }
        #[cfg(not(unix))]
        {
            // FIXME: add code to check directory permissions
        }
        path.push(&*context.0);
        trace!("Reading keyring {:?}", path);
        let mut lines = FileLines::open(&path).await?.enumerate();
        let mut cookies = vec![];
        while let Some((n, line)) = lines.next().await {
            let line = line?;
            let mut split = line.split_whitespace();
            let id = split
                .next()
                .ok_or_else(|| {
                    Error::Handshake(format!(
                        "DBus cookie `{}` missing ID at line {n}",
                        path.display(),
                    ))
                })?
                .parse()
                .map_err(|e| {
                    Error::Handshake(format!(
                        "Failed to parse cookie ID in file `{}` at line {n}: {e}",
                        path.display(),
                    ))
                })?;
            let _ = split.next().ok_or_else(|| {
                Error::Handshake(format!(
                    "DBus cookie `{}` missing creation time at line {n}",
                    path.display(),
                ))
            })?;
            let cookie = split
                .next()
                .ok_or_else(|| {
                    Error::Handshake(format!(
                        "DBus cookie `{}` missing cookie data at line {}",
                        path.to_str().unwrap(),
                        n
                    ))
                })?
                .to_string();
            cookies.push(Cookie { id, cookie })
        }
        trace!("Loaded keyring {:?}", cookies);
        Ok(cookies)
    }

    async fn lookup(context: &CookieContext<'_>, id: usize) -> Result<Cookie> {
        let keyring = Self::read_keyring(context).await?;
        keyring
            .into_iter()
            .find(|c| c.id == id)
            .ok_or_else(|| Error::Handshake(format!("DBus cookie ID {id} not found")))
    }

    #[cfg(feature = "p2p")]
    async fn first(context: &CookieContext<'_>) -> Result<Cookie> {
        let keyring = Self::read_keyring(context).await?;
        keyring
            .into_iter()
            .next()
            .ok_or_else(|| Error::Handshake("No cookies available".into()))
    }
}

#[derive(Debug)]
pub struct CookieContext<'c>(Str<'c>);

impl<'c> TryFrom<Str<'c>> for CookieContext<'c> {
    type Error = Error;

    fn try_from(value: Str<'c>) -> Result<Self> {
        if value.is_empty() {
            return Err(Error::Handshake("Empty cookie context".into()));
        } else if !value.is_ascii() || value.contains(['/', '\\', ' ', '\n', '\r', '\t', '.']) {
            return Err(Error::Handshake(
                "Invalid characters in cookie context".into(),
            ));
        }

        Ok(Self(value))
    }
}

impl Default for CookieContext<'_> {
    fn default() -> Self {
        Self(Str::from_static("org_freedesktop_general"))
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::Handshake(format!("Invalid hexcode: {e}"))
    }
}

// Common code for the client and server side of the handshake.
#[derive(Debug)]
pub struct HandshakeCommon {
    socket: BoxedSplit,
    recv_buffer: Vec<u8>,
    cap_unix_fd: bool,
    // the current AUTH mechanism is front, ordered by priority
    mechanisms: VecDeque<AuthMechanism>,
}

impl HandshakeCommon {
    /// Start a handshake on this client socket
    pub fn new(socket: BoxedSplit, mechanisms: VecDeque<AuthMechanism>) -> Self {
        Self {
            socket,
            recv_buffer: Vec::new(),
            cap_unix_fd: false,
            mechanisms,
        }
    }

    #[instrument(skip(self))]
    async fn write_command(&mut self, command: Command) -> Result<()> {
        self.write_commands(&[command]).await
    }

    #[instrument(skip(self))]
    async fn write_commands(&mut self, commands: &[Command]) -> Result<()> {
        let mut send_buffer =
            commands
                .iter()
                .map(Vec::<u8>::from)
                .fold(vec![], |mut acc, mut c| {
                    acc.append(&mut c);
                    acc.extend_from_slice(b"\r\n");
                    acc
                });
        while !send_buffer.is_empty() {
            let written = self
                .socket
                .write_mut()
                .sendmsg(
                    &send_buffer,
                    #[cfg(unix)]
                    &[],
                )
                .await?;
            send_buffer.drain(..written);
        }
        trace!("Wrote all commands");
        Ok(())
    }

    #[instrument(skip(self))]
    async fn read_command(&mut self) -> Result<Command> {
        self.read_commands(1)
            .await
            .map(|cmds| cmds.into_iter().next().unwrap())
    }

    #[instrument(skip(self))]
    async fn read_commands(&mut self, n_commands: usize) -> Result<Vec<Command>> {
        let mut commands = Vec::with_capacity(n_commands);
        let mut n_received_commands = 0;
        'outer: loop {
            while let Some(lf_index) = self.recv_buffer.iter().position(|b| *b == b'\n') {
                if self.recv_buffer[lf_index - 1] != b'\r' {
                    return Err(Error::Handshake("Invalid line ending in handshake".into()));
                }

                let line_bytes = self.recv_buffer.drain(..=lf_index);
                let line = std::str::from_utf8(line_bytes.as_slice())
                    .map_err(|e| Error::Handshake(e.to_string()))?;

                trace!("Reading {line}");
                commands.push(line.parse()?);
                n_received_commands += 1;

                if n_received_commands == n_commands {
                    break 'outer;
                }
            }

            let mut buf = vec![0; 1024];
            let res = self.socket.read_mut().recvmsg(&mut buf).await?;
            let read = {
                #[cfg(unix)]
                {
                    let (read, fds) = res;
                    if !fds.is_empty() {
                        return Err(Error::Handshake("Unexpected FDs during handshake".into()));
                    }
                    read
                }
                #[cfg(not(unix))]
                {
                    res
                }
            };
            if read == 0 {
                return Err(Error::Handshake("Unexpected EOF during handshake".into()));
            }
            self.recv_buffer.extend(&buf[..read]);
        }

        Ok(commands)
    }

    fn next_mechanism(&mut self) -> Result<AuthMechanism> {
        self.mechanisms
            .pop_front()
            .ok_or_else(|| Error::Handshake("Exhausted available AUTH mechanisms".into()))
    }
}

#[cfg(feature = "p2p")]
#[cfg(unix)]
#[cfg(test)]
mod tests {
    #[cfg(not(feature = "tokio"))]
    use async_std::io::{Write as AsyncWrite, WriteExt};
    use futures_util::future::join;
    use ntest::timeout;
    #[cfg(not(feature = "tokio"))]
    use std::os::unix::net::UnixStream;
    use test_log::test;
    #[cfg(feature = "tokio")]
    use tokio::{
        io::{AsyncWrite, AsyncWriteExt},
        net::UnixStream,
    };

    use super::*;

    use crate::{Guid, Socket};

    fn create_async_socket_pair() -> (impl AsyncWrite + Socket, impl AsyncWrite + Socket) {
        // Tokio needs us to call the sync function from async context. :shrug:
        let (p0, p1) = crate::utils::block_on(async { UnixStream::pair().unwrap() });

        // initialize both handshakes
        #[cfg(not(feature = "tokio"))]
        let (p0, p1) = {
            p0.set_nonblocking(true).unwrap();
            p1.set_nonblocking(true).unwrap();

            (
                async_io::Async::new(p0).unwrap(),
                async_io::Async::new(p1).unwrap(),
            )
        };

        (p0, p1)
    }

    #[test]
    #[timeout(15000)]
    fn handshake() {
        let (p0, p1) = create_async_socket_pair();

        let guid = OwnedGuid::from(Guid::generate());
        let client = ClientHandshake::new(p0.into(), None, Some(guid.clone()));
        let server = ServerHandshake::new(
            p1.into(),
            guid,
            Some(Uid::effective().into()),
            None,
            None,
            CookieContext::default(),
        )
        .unwrap();

        // proceed to the handshakes
        let (client, server) = crate::utils::block_on(join(
            async move { client.perform().await.unwrap() },
            async move { server.perform().await.unwrap() },
        ));

        assert_eq!(client.server_guid, server.server_guid);
        assert_eq!(client.cap_unix_fd, server.cap_unix_fd);
    }

    #[test]
    #[timeout(15000)]
    fn pipelined_handshake() {
        let (mut p0, p1) = create_async_socket_pair();
        let server = ServerHandshake::new(
            p1.into(),
            Guid::generate().into(),
            Some(Uid::effective().into()),
            None,
            None,
            CookieContext::default(),
        )
        .unwrap();

        crate::utils::block_on(
            p0.write_all(
                format!(
                    "\0AUTH EXTERNAL {}\r\nNEGOTIATE_UNIX_FD\r\nBEGIN\r\n",
                    hex::encode(sasl_auth_id().unwrap())
                )
                .as_bytes(),
            ),
        )
        .unwrap();
        let server = crate::utils::block_on(server.perform()).unwrap();

        assert!(server.cap_unix_fd);
    }

    #[test]
    #[timeout(15000)]
    fn separate_external_data() {
        let (mut p0, p1) = create_async_socket_pair();
        let server = ServerHandshake::new(
            p1.into(),
            Guid::generate().into(),
            Some(Uid::effective().into()),
            None,
            None,
            CookieContext::default(),
        )
        .unwrap();

        crate::utils::block_on(
            p0.write_all(
                format!(
                    "\0AUTH EXTERNAL\r\nDATA {}\r\nBEGIN\r\n",
                    hex::encode(sasl_auth_id().unwrap())
                )
                .as_bytes(),
            ),
        )
        .unwrap();
        crate::utils::block_on(server.perform()).unwrap();
    }

    #[test]
    #[timeout(15000)]
    fn missing_external_data() {
        let (mut p0, p1) = create_async_socket_pair();
        let server = ServerHandshake::new(
            p1.into(),
            Guid::generate().into(),
            Some(Uid::effective().into()),
            None,
            None,
            CookieContext::default(),
        )
        .unwrap();

        crate::utils::block_on(p0.write_all(b"\0AUTH EXTERNAL\r\nDATA\r\nBEGIN\r\n")).unwrap();
        crate::utils::block_on(server.perform()).unwrap();
    }

    #[test]
    #[timeout(15000)]
    fn anonymous_handshake() {
        let (mut p0, p1) = create_async_socket_pair();
        let server = ServerHandshake::new(
            p1.into(),
            Guid::generate().into(),
            Some(Uid::effective().into()),
            Some(vec![AuthMechanism::Anonymous].into()),
            None,
            CookieContext::default(),
        )
        .unwrap();

        crate::utils::block_on(p0.write_all(b"\0AUTH ANONYMOUS abcd\r\nBEGIN\r\n")).unwrap();
        crate::utils::block_on(server.perform()).unwrap();
    }

    #[test]
    #[timeout(15000)]
    fn separate_anonymous_data() {
        let (mut p0, p1) = create_async_socket_pair();
        let server = ServerHandshake::new(
            p1.into(),
            Guid::generate().into(),
            Some(Uid::effective().into()),
            Some(vec![AuthMechanism::Anonymous].into()),
            None,
            CookieContext::default(),
        )
        .unwrap();

        crate::utils::block_on(p0.write_all(b"\0AUTH ANONYMOUS\r\nDATA abcd\r\nBEGIN\r\n"))
            .unwrap();
        crate::utils::block_on(server.perform()).unwrap();
    }
}
