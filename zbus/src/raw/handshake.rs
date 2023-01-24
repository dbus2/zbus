#[cfg(unix)]
use nix::unistd::Uid;
use std::{
    collections::VecDeque,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
    task::{Context, Poll},
};
use tracing::{instrument, trace};

#[cfg(windows)]
use crate::win32;
use crate::{
    guid::Guid,
    raw::{Connection, Socket},
    AuthMechanism, Error, Result,
};

use futures_core::ready;

use sha1::{Digest, Sha1};

/*
 * Client-side handshake logic
 */

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
enum ClientHandshakeStep {
    Init,
    MechanismInit,
    WaitingForData,
    WaitingForOK,
    WaitingForAgreeUnixFD,
    Done,
}

// The plain-text SASL profile authentication protocol described here:
// <https://dbus.freedesktop.org/doc/dbus-specification.html#auth-protocol>
//
// These are all the known commands, which can be parsed from or serialized to text.
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum Command {
    Auth(Option<AuthMechanism>, Option<Vec<u8>>),
    Cancel,
    Begin,
    Data(Option<Vec<u8>>),
    Error(String),
    NegotiateUnixFD,
    Rejected(Vec<AuthMechanism>),
    Ok(Guid),
    AgreeUnixFD,
}

/// A representation of an in-progress handshake, client-side
///
/// This struct is an async-compatible representation of the initial handshake that must be performed before
/// a D-Bus connection can be used. To use it, you should call the [`advance_handshake`] method whenever the
/// underlying socket becomes ready (tracking the readiness itself is not managed by this abstraction) until
/// it returns `Ok(())`, at which point you can invoke the [`try_finish`] method to get an [`Authenticated`],
/// which can be given to [`Connection::new_authenticated`].
///
/// [`advance_handshake`]: struct.ClientHandshake.html#method.advance_handshake
/// [`try_finish`]: struct.ClientHandshake.html#method.try_finish
/// [`Authenticated`]: struct.AUthenticated.html
/// [`Connection::new_authenticated`]: ../struct.Connection.html#method.new_authenticated
#[derive(Debug)]
pub struct ClientHandshake<S> {
    common: HandshakeCommon<S>,
    step: ClientHandshakeStep,
}

/// The result of a finalized handshake
///
/// The result of a finalized [`ClientHandshake`] or [`ServerHandshake`]. It can be passed to
/// [`Connection::new_authenticated`] to initialize a connection.
///
/// [`ClientHandshake`]: struct.ClientHandshake.html
/// [`ServerHandshake`]: struct.ServerHandshake.html
/// [`Connection::new_authenticated`]: ../struct.Connection.html#method.new_authenticated
#[derive(Debug)]
pub struct Authenticated<S> {
    pub(crate) conn: Connection<S>,
    /// The server Guid
    pub(crate) server_guid: Guid,
    /// Whether file descriptor passing has been accepted by both sides
    #[cfg(unix)]
    pub(crate) cap_unix_fd: bool,
}

pub trait Handshake<S> {
    /// Attempt to advance the handshake
    ///
    /// In non-blocking mode, you need to invoke this method repeatedly
    /// until it returns `Ok(())`. Once it does, the handshake is finished
    /// and you can invoke the [`Handshake::try_finish`] method.
    ///
    /// Note that only the initial handshake is done. If you need to send a
    /// Bus Hello, this remains to be done.
    fn advance_handshake(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>>;

    /// Attempt to finalize this handshake into an initialized client.
    ///
    /// This method should only be called once `advance_handshake()` has
    /// returned `Ok(())`. Otherwise it'll error and return you the object.
    fn try_finish(self) -> std::result::Result<Authenticated<S>, Self>
    where
        Self: Sized;
}

impl<S: Socket> ClientHandshake<S> {
    /// Start a handshake on this client socket
    pub fn new(socket: S, mechanisms: Option<VecDeque<AuthMechanism>>) -> ClientHandshake<S> {
        let mechanisms = mechanisms.unwrap_or_else(|| {
            let mut mechanisms = VecDeque::new();
            mechanisms.push_back(AuthMechanism::External);
            mechanisms.push_back(AuthMechanism::Cookie);
            mechanisms.push_back(AuthMechanism::Anonymous);
            mechanisms
        });

        ClientHandshake {
            common: HandshakeCommon::new(socket, mechanisms, None),
            step: ClientHandshakeStep::Init,
        }
    }

    fn mechanism_init(&mut self) -> Result<(ClientHandshakeStep, Command)> {
        use ClientHandshakeStep::*;
        let mech = self.common.mechanism()?;
        match mech {
            AuthMechanism::Anonymous => Ok((
                WaitingForOK,
                Command::Auth(Some(*mech), Some("zbus".into())),
            )),
            AuthMechanism::External => Ok((
                WaitingForOK,
                Command::Auth(Some(*mech), Some(sasl_auth_id()?.into_bytes())),
            )),
            AuthMechanism::Cookie => Ok((
                WaitingForData,
                Command::Auth(Some(*mech), Some(sasl_auth_id()?.into_bytes())),
            )),
        }
    }

    fn mechanism_data(&mut self, data: Vec<u8>) -> Result<(ClientHandshakeStep, Command)> {
        use ClientHandshakeStep::*;
        let mech = self.common.mechanism()?;
        match mech {
            AuthMechanism::Cookie => {
                let context = String::from_utf8_lossy(&data);
                let mut split = context.split_ascii_whitespace();
                let name = split
                    .next()
                    .ok_or_else(|| Error::Handshake("Missing cookie context name".into()))?;
                let id = split
                    .next()
                    .ok_or_else(|| Error::Handshake("Missing cookie ID".into()))?;
                let server_chall = split
                    .next()
                    .ok_or_else(|| Error::Handshake("Missing cookie challenge".into()))?;

                let cookie = Cookie::lookup(name, id)?;
                let client_chall = random_ascii(16);
                let sec = format!("{server_chall}:{client_chall}:{cookie}");
                let sha1 = hex::encode(Sha1::digest(sec));
                let data = format!("{client_chall} {sha1}");
                Ok((WaitingForOK, Command::Data(Some(data.into()))))
            }
            _ => Err(Error::Handshake("Unexpected mechanism DATA".into())),
        }
    }
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
    id: String,
    cookie: String,
}

impl Cookie {
    fn keyring_path() -> Result<PathBuf> {
        let mut path =
            home_dir().ok_or_else(|| Error::Handshake("Failed to get home directory".into()))?;
        path.push(".dbus-keyrings");
        Ok(path)
    }

    fn read_keyring(name: &str) -> Result<Vec<Cookie>> {
        let mut path = Cookie::keyring_path()?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let perms = std::fs::metadata(&path)?.permissions().mode();
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
        path.push(name);
        trace!("Reading keyring {:?}", path);
        let file = File::open(&path)?;
        let mut cookies = vec![];
        for (n, line) in BufReader::new(file).lines().enumerate() {
            let line = line?;
            let mut split = line.split_whitespace();
            let id = split
                .next()
                .ok_or_else(|| {
                    Error::Handshake(format!(
                        "DBus cookie `{}` missing ID at line {}",
                        path.to_str().unwrap(),
                        n
                    ))
                })?
                .to_string();
            let _ = split.next().ok_or_else(|| {
                Error::Handshake(format!(
                    "DBus cookie `{}` missing creation time at line {}",
                    path.to_str().unwrap(),
                    n
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

    fn lookup(name: &str, id: &str) -> Result<String> {
        let keyring = Self::read_keyring(name)?;
        let c = keyring
            .iter()
            .find(|c| c.id == id)
            .ok_or_else(|| Error::Handshake(format!("DBus cookie ID {id} not found")))?;
        Ok(c.cookie.to_string())
    }
}

// See https://github.com/dirs-dev/dirs-rs/issues/45
fn home_dir() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        Some(home.into())
    } else {
        dirs::home_dir()
    }
}

impl<S: Socket> Handshake<S> for ClientHandshake<S> {
    #[instrument(skip(self, cx))]
    fn advance_handshake(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        use ClientHandshakeStep::*;
        loop {
            ready!(self.common.flush_buffer(cx))?;
            let (next_step, cmd) = match self.step {
                Init => {
                    trace!("Initializing");
                    #[allow(clippy::let_and_return)]
                    let ret = self.mechanism_init()?;
                    // The dbus daemon on some platforms requires sending the zero byte as a separate message with SCM_CREDS.
                    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
                    self.common
                        .socket
                        .send_zero_byte()
                        .map_err(|e| {
                            Error::Handshake(format!(
                                "Could not send zero byte with credentials: {}",
                                e
                            ))
                        })
                        .and_then(|n| match n {
                            Some(n) if n != 1 => Err(Error::Handshake(
                                "Could not send zero byte with credentials".to_string(),
                            )),
                            _ => Ok(()),
                        })?;

                    ret
                }
                MechanismInit => {
                    trace!("Initializing auth mechanisms");
                    self.mechanism_init()?
                }
                WaitingForData | WaitingForOK => {
                    trace!("Waiting for DATA or OK from server");
                    let reply = ready!(self.common.read_command(cx))?;
                    match (self.step, reply) {
                        (_, Command::Data(data)) => {
                            trace!("Received DATA from server");
                            let data = data.ok_or_else(|| {
                                Error::Handshake("Received DATA with no data from server".into())
                            })?;
                            self.mechanism_data(data)?
                        }
                        (_, Command::Rejected(_)) => {
                            trace!("Received REJECT from server. Will try next auth mechanism..");
                            self.common.mechanisms.pop_front();
                            self.step = MechanismInit;
                            continue;
                        }
                        (WaitingForOK, Command::Ok(guid)) => {
                            trace!("Received OK from server");
                            self.common.server_guid = Some(guid);
                            if self.common.socket.can_pass_unix_fd() {
                                (WaitingForAgreeUnixFD, Command::NegotiateUnixFD)
                            } else {
                                (Done, Command::Begin)
                            }
                        }
                        (_, reply) => {
                            return Poll::Ready(Err(Error::Handshake(format!(
                                "Unexpected server AUTH OK reply: {reply}"
                            ))));
                        }
                    }
                }
                WaitingForAgreeUnixFD => {
                    trace!("Waiting for Unix FD passing agreement from server");
                    let reply = ready!(self.common.read_command(cx))?;
                    match reply {
                        Command::AgreeUnixFD => {
                            trace!("Unix FD passing agreed by server");
                            self.common.cap_unix_fd = true
                        }
                        Command::Error(_) => {
                            trace!("Unix FD passing rejected by server");
                            self.common.cap_unix_fd = false
                        }
                        _ => {
                            return Poll::Ready(Err(Error::Handshake(format!(
                                "Unexpected server UNIX_FD reply: {reply}"
                            ))));
                        }
                    }
                    (Done, Command::Begin)
                }
                Done => {
                    trace!("Handshake done");
                    return Poll::Ready(Ok(()));
                }
            };
            self.common.send_buffer = if self.step == Init
                // leading 0 is sent separately already for `freebsd` and `dragonfly` above.
                && !cfg!(any(target_os = "freebsd", target_os = "dragonfly"))
            {
                format!("\0{cmd}").into()
            } else {
                cmd.into()
            };
            self.step = next_step;
        }
    }

    fn try_finish(self) -> std::result::Result<Authenticated<S>, Self> {
        if let ClientHandshakeStep::Done = self.step {
            Ok(Authenticated {
                conn: Connection::new(self.common.socket, self.common.recv_buffer),
                server_guid: self.common.server_guid.unwrap(),
                #[cfg(unix)]
                cap_unix_fd: self.common.cap_unix_fd,
            })
        } else {
            Err(self)
        }
    }
}

/*
 * Server-side handshake logic
 */

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
enum ServerHandshakeStep {
    WaitingForNull,
    WaitingForAuth,
    SendingAuthData(AuthMechanism),
    WaitingForData(AuthMechanism),
    SendingAuthOK,
    SendingAuthError,
    WaitingForBegin,
    #[cfg(unix)]
    SendingAgreeUnixFD,
    Done,
}

/// A representation of an in-progress handshake, server-side
///
/// This would typically be used to implement a D-Bus broker, or in the context of a P2P connection.
///
/// This struct is an async-compatible representation of the initial handshake that must be performed before
/// a D-Bus connection can be used. To use it, you should call the [`advance_handshake`] method whenever the
/// underlying socket becomes ready (tracking the readiness itself is not managed by this abstraction) until
/// it returns `Ok(())`, at which point you can invoke the [`try_finish`] method to get an [`Authenticated`],
/// which can be given to [`Connection::new_authenticated`].
///
/// [`advance_handshake`]: struct.ServerHandshake.html#method.advance_handshake
/// [`try_finish`]: struct.ServerHandshake.html#method.try_finish
/// [`Authenticated`]: struct.Authenticated.html
/// [`Connection::new_authenticated`]: ../struct.Connection.html#method.new_authenticated
#[derive(Debug)]
pub struct ServerHandshake<S> {
    common: HandshakeCommon<S>,
    step: ServerHandshakeStep,
    #[cfg(unix)]
    client_uid: Option<u32>,
    #[cfg(windows)]
    client_sid: Option<String>,
}

impl<S: Socket> ServerHandshake<S> {
    pub fn new(
        socket: S,
        guid: Guid,
        #[cfg(unix)] client_uid: Option<u32>,
        #[cfg(windows)] client_sid: Option<String>,
        mechanisms: Option<VecDeque<AuthMechanism>>,
    ) -> Result<ServerHandshake<S>> {
        let mechanisms = match mechanisms {
            Some(mechanisms) => mechanisms,
            None => {
                let mut mechanisms = VecDeque::new();
                mechanisms.push_back(AuthMechanism::External);

                mechanisms
            }
        };
        if mechanisms.contains(&AuthMechanism::Cookie) {
            return Err(Error::Unsupported);
        }

        Ok(ServerHandshake {
            common: HandshakeCommon::new(socket, mechanisms, Some(guid)),
            step: ServerHandshakeStep::WaitingForNull,
            #[cfg(unix)]
            client_uid,
            #[cfg(windows)]
            client_sid,
        })
    }

    fn auth_ok(&mut self) {
        self.common.send_buffer = Command::Ok(self.guid().clone()).into();
        self.step = ServerHandshakeStep::SendingAuthOK;
    }

    fn check_external_auth(&mut self, sasl_id: &[u8]) -> Result<()> {
        let auth_ok = {
            let id = std::str::from_utf8(sasl_id)
                .map_err(|e| Error::Handshake(format!("Invalid ID: {e}")))?;
            #[cfg(unix)]
            {
                let uid = id
                    .parse::<u32>()
                    .map_err(|e| Error::Handshake(format!("Invalid UID: {e}")))?;
                self.client_uid.map(|u| u == uid).unwrap_or(false)
            }
            #[cfg(windows)]
            {
                self.client_sid.as_ref().map(|u| u == id).unwrap_or(false)
            }
        };

        if auth_ok {
            self.auth_ok();
        } else {
            self.rejected_error();
        }

        Ok(())
    }

    fn unsupported_command_error(&mut self) {
        self.common.send_buffer = Command::Error("Unsupported command".to_string()).into();
        self.step = ServerHandshakeStep::SendingAuthError;
    }

    fn rejected_error(&mut self) {
        let mechanisms = self.common.mechanisms.iter().cloned().collect();
        self.common.send_buffer = Command::Rejected(mechanisms).into();
        self.step = ServerHandshakeStep::SendingAuthError;
    }

    fn guid(&self) -> &Guid {
        // SAFETY: We know that the server GUID is set because we set it in the constructor.
        self.common
            .server_guid
            .as_ref()
            .expect("Server GUID not set")
    }
}

impl<S: Socket> Handshake<S> for ServerHandshake<S> {
    #[instrument(skip(self, cx))]
    fn advance_handshake(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        loop {
            match self.step {
                ServerHandshakeStep::WaitingForNull => {
                    trace!("Waiting for NULL");
                    let mut buffer = [0; 1];
                    let read = ready!(self.common.socket.poll_recvmsg(cx, &mut buffer))?;
                    #[cfg(unix)]
                    let read = read.0;
                    // recvmsg cannot return anything else than Ok(1) or Err
                    debug_assert!(read == 1);
                    if buffer[0] != 0 {
                        return Poll::Ready(Err(Error::Handshake(
                            "First client byte is not NUL!".to_string(),
                        )));
                    }
                    trace!("Received NULL from client");
                    self.step = ServerHandshakeStep::WaitingForAuth;
                }
                ServerHandshakeStep::WaitingForAuth => {
                    trace!("Waiting for authentication");
                    let reply = ready!(self.common.read_command(cx))?;
                    match reply {
                        Command::Auth(mech, resp) => {
                            let mech = mech.filter(|m| self.common.mechanisms.contains(m));

                            match (mech, &resp) {
                                (Some(mech), None) => {
                                    self.common.send_buffer = Command::Data(None).into();
                                    self.step = ServerHandshakeStep::SendingAuthData(mech);
                                }
                                (Some(AuthMechanism::Anonymous), Some(_)) => {
                                    self.auth_ok();
                                }
                                (Some(AuthMechanism::External), Some(sasl_id)) => {
                                    self.check_external_auth(sasl_id)?;
                                }
                                _ => self.rejected_error(),
                            }
                        }
                        Command::Error(_) => self.rejected_error(),
                        Command::Begin => {
                            return Poll::Ready(Err(Error::Handshake(
                                "Received BEGIN while not authenticated".to_string(),
                            )));
                        }
                        _ => self.unsupported_command_error(),
                    }
                }
                ServerHandshakeStep::SendingAuthData(mech) => {
                    trace!("Sending data request");
                    ready!(self.common.flush_buffer(cx))?;
                    self.step = ServerHandshakeStep::WaitingForData(mech);
                }
                ServerHandshakeStep::WaitingForData(mech) => {
                    trace!("Waiting for authentication");
                    let reply = ready!(self.common.read_command(cx))?;
                    match (mech, reply) {
                        (AuthMechanism::External, Command::Data(data)) => match data {
                            None => self.auth_ok(),
                            Some(data) => self.check_external_auth(&data)?,
                        },
                        (AuthMechanism::Anonymous, Command::Data(_)) => self.auth_ok(),
                        (_, Command::Data(_)) => self.rejected_error(),
                        (_, _) => self.unsupported_command_error(),
                    }
                }
                ServerHandshakeStep::SendingAuthError => {
                    trace!("Sending authentication error");
                    ready!(self.common.flush_buffer(cx))?;
                    self.step = ServerHandshakeStep::WaitingForAuth;
                }
                ServerHandshakeStep::SendingAuthOK => {
                    trace!("Sending authentication OK");
                    ready!(self.common.flush_buffer(cx))?;
                    self.step = ServerHandshakeStep::WaitingForBegin;
                }
                ServerHandshakeStep::WaitingForBegin => {
                    trace!("Waiting for Begin command from the client");
                    let reply = ready!(self.common.read_command(cx))?;
                    match reply {
                        Command::Begin => {
                            trace!("Received Begin command from the client");
                            self.step = ServerHandshakeStep::Done;
                        }
                        Command::Cancel | Command::Error(_) => {
                            trace!("Received CANCEL or ERROR command from the client");
                            self.rejected_error()
                        }
                        #[cfg(unix)]
                        Command::NegotiateUnixFD => {
                            trace!("Received NEGOTIATE_UNIX_FD command from the client");
                            self.common.cap_unix_fd = true;
                            self.common.send_buffer = Command::AgreeUnixFD.into();
                            self.step = ServerHandshakeStep::SendingAgreeUnixFD;
                        }
                        _ => self.unsupported_command_error(),
                    }
                }
                #[cfg(unix)]
                ServerHandshakeStep::SendingAgreeUnixFD => {
                    trace!("Sending AGREE_UNIX_FD to the client");
                    ready!(self.common.flush_buffer(cx))?;
                    self.step = ServerHandshakeStep::WaitingForBegin;
                }
                ServerHandshakeStep::Done => {
                    trace!("Handshake done");
                    return Poll::Ready(Ok(()));
                }
            }
        }
    }

    fn try_finish(self) -> std::result::Result<Authenticated<S>, Self> {
        if let ServerHandshakeStep::Done = self.step {
            Ok(Authenticated {
                conn: Connection::new(self.common.socket, self.common.recv_buffer),
                // SAFETY: We know that the server GUID is set because we set it in the constructor.
                server_guid: self.common.server_guid.expect("Server GUID not set"),
                #[cfg(unix)]
                cap_unix_fd: self.common.cap_unix_fd,
            })
        } else {
            Err(self)
        }
    }
}

impl fmt::Display for AuthMechanism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mech = match self {
            AuthMechanism::External => "EXTERNAL",
            AuthMechanism::Cookie => "DBUS_COOKIE_SHA1",
            AuthMechanism::Anonymous => "ANONYMOUS",
        };
        write!(f, "{mech}")
    }
}

impl FromStr for AuthMechanism {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "EXTERNAL" => Ok(AuthMechanism::External),
            "DBUS_COOKIE_SHA1" => Ok(AuthMechanism::Cookie),
            "ANONYMOUS" => Ok(AuthMechanism::Anonymous),
            _ => Err(Error::Handshake(format!("Unknown mechanism: {s}"))),
        }
    }
}

impl From<Command> for Vec<u8> {
    fn from(c: Command) -> Self {
        c.to_string().into()
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmd = match self {
            Command::Auth(mech, resp) => match (mech, resp) {
                (Some(mech), Some(resp)) => format!("AUTH {mech} {}", hex::encode(resp)),
                (Some(mech), None) => format!("AUTH {mech}"),
                _ => "AUTH".into(),
            },
            Command::Cancel => "CANCEL".into(),
            Command::Begin => "BEGIN".into(),
            Command::Data(data) => match data {
                None => "DATA".to_string(),
                Some(data) => format!("DATA {}", hex::encode(data)),
            },
            Command::Error(expl) => {
                format!("ERROR {expl}")
            }
            Command::NegotiateUnixFD => "NEGOTIATE_UNIX_FD".into(),
            Command::Rejected(mechs) => {
                format!(
                    "REJECTED {}",
                    mechs
                        .iter()
                        .map(|m| m.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Command::Ok(guid) => {
                format!("OK {guid}")
            }
            Command::AgreeUnixFD => "AGREE_UNIX_FD".into(),
        };
        write!(f, "{cmd}\r\n")
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::Handshake(format!("Invalid hexcode: {e}"))
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut words = s.split_ascii_whitespace();
        let cmd = match words.next() {
            Some("AUTH") => {
                let mech = if let Some(m) = words.next() {
                    Some(m.parse()?)
                } else {
                    None
                };
                let resp = match words.next() {
                    Some(resp) => Some(hex::decode(resp)?),
                    None => None,
                };
                Command::Auth(mech, resp)
            }
            Some("CANCEL") => Command::Cancel,
            Some("BEGIN") => Command::Begin,
            Some("DATA") => {
                let data = match words.next() {
                    Some(data) => Some(hex::decode(data)?),
                    None => None,
                };

                Command::Data(data)
            }
            Some("ERROR") => Command::Error(s.into()),
            Some("NEGOTIATE_UNIX_FD") => Command::NegotiateUnixFD,
            Some("REJECTED") => {
                let mechs = words.map(|m| m.parse()).collect::<Result<_>>()?;
                Command::Rejected(mechs)
            }
            Some("OK") => {
                let guid = words
                    .next()
                    .ok_or_else(|| Error::Handshake("Missing OK server GUID!".into()))?;
                Command::Ok(guid.parse()?)
            }
            Some("AGREE_UNIX_FD") => Command::AgreeUnixFD,
            _ => return Err(Error::Handshake(format!("Unknown command: {s}"))),
        };
        Ok(cmd)
    }
}

// Common code for the client and server side of the handshake.
#[derive(Debug)]
pub struct HandshakeCommon<S> {
    socket: S,
    recv_buffer: Vec<u8>,
    send_buffer: Vec<u8>,
    server_guid: Option<Guid>,
    cap_unix_fd: bool,
    // the current AUTH mechanism is front, ordered by priority
    mechanisms: VecDeque<AuthMechanism>,
}

impl<S: Socket> HandshakeCommon<S> {
    /// Start a handshake on this client socket
    pub fn new(socket: S, mechanisms: VecDeque<AuthMechanism>, server_guid: Option<Guid>) -> Self {
        Self {
            socket,
            recv_buffer: Vec::new(),
            send_buffer: Vec::new(),
            server_guid,
            cap_unix_fd: false,
            mechanisms,
        }
    }

    #[instrument(skip(self, cx))]
    fn flush_buffer(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        while !self.send_buffer.is_empty() {
            let written = ready!(self.socket.poll_sendmsg(
                cx,
                &self.send_buffer,
                #[cfg(unix)]
                &[]
            ))?;
            self.send_buffer.drain(..written);
        }
        Ok(()).into()
    }

    #[instrument(skip(self, cx))]
    fn read_command(&mut self, cx: &mut Context<'_>) -> Poll<Result<Command>> {
        let mut cmd_end = 0;
        loop {
            if let Some(i) = self.recv_buffer[cmd_end..].iter().position(|b| *b == b'\n') {
                if cmd_end + i == 0 || self.recv_buffer.get(cmd_end + i - 1) != Some(&b'\r') {
                    return Poll::Ready(Err(Error::Handshake(
                        "Invalid line ending in handshake".into(),
                    )));
                }
                cmd_end += i + 1;

                break;
            } else {
                cmd_end = self.recv_buffer.len();
            }

            let mut buf = [0; 64];
            let res = ready!(self.socket.poll_recvmsg(cx, &mut buf))?;
            let read = {
                #[cfg(unix)]
                {
                    let (read, fds) = res;
                    if !fds.is_empty() {
                        return Poll::Ready(Err(Error::Handshake(
                            "Unexpected FDs during handshake".into(),
                        )));
                    }
                    read
                }
                #[cfg(not(unix))]
                {
                    res
                }
            };
            if read == 0 {
                return Poll::Ready(Err(Error::Handshake(
                    "Unexpected EOF during handshake".into(),
                )));
            }
            self.recv_buffer.extend(&buf[..read]);
        }

        let line_bytes = self.recv_buffer.drain(..cmd_end);
        let line = std::str::from_utf8(line_bytes.as_slice())
            .map_err(|e| Error::Handshake(e.to_string()))?;
        let cmd = line.parse();

        Poll::Ready(cmd)
    }

    fn mechanism(&self) -> Result<&AuthMechanism> {
        self.mechanisms
            .front()
            .ok_or_else(|| Error::Handshake("Exhausted available AUTH mechanisms".into()))
    }
}

#[cfg(unix)]
#[cfg(test)]
mod tests {
    #[cfg(not(feature = "tokio"))]
    use async_std::io::{Write as AsyncWrite, WriteExt};
    use futures_util::future::poll_fn;
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

    use crate::Guid;

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
    fn handshake() {
        let (p0, p1) = create_async_socket_pair();

        let mut client = ClientHandshake::new(p0, None);
        let mut server =
            ServerHandshake::new(p1, Guid::generate(), Some(Uid::effective().into()), None)
                .unwrap();

        // proceed to the handshakes
        let mut client_done = false;
        let mut server_done = false;
        crate::utils::block_on(poll_fn(|cx| {
            match client.advance_handshake(cx) {
                Poll::Ready(Ok(())) => client_done = true,
                Poll::Ready(Err(e)) => panic!("Unexpected error: {:?}", e),
                Poll::Pending => {}
            }

            match server.advance_handshake(cx) {
                Poll::Ready(Ok(())) => server_done = true,
                Poll::Ready(Err(e)) => panic!("Unexpected error: {:?}", e),
                Poll::Pending => {}
            }
            if client_done && server_done {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }));

        let client = client.try_finish().unwrap();
        let server = server.try_finish().unwrap();

        assert_eq!(client.server_guid, server.server_guid);
        assert_eq!(client.cap_unix_fd, server.cap_unix_fd);
    }

    #[test]
    #[timeout(15000)]
    fn pipelined_handshake() {
        let (mut p0, p1) = create_async_socket_pair();
        let mut server =
            ServerHandshake::new(p1, Guid::generate(), Some(Uid::effective().into()), None)
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
        crate::utils::block_on(poll_fn(|cx| server.advance_handshake(cx))).unwrap();

        let server = server.try_finish().unwrap();

        assert!(server.cap_unix_fd);
    }

    #[test]
    #[timeout(15000)]
    fn separate_external_data() {
        let (mut p0, p1) = create_async_socket_pair();
        let mut server =
            ServerHandshake::new(p1, Guid::generate(), Some(Uid::effective().into()), None)
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
        crate::utils::block_on(poll_fn(|cx| server.advance_handshake(cx))).unwrap();

        server.try_finish().unwrap();
    }

    #[test]
    #[timeout(15000)]
    fn missing_external_data() {
        let (mut p0, p1) = create_async_socket_pair();
        let mut server =
            ServerHandshake::new(p1, Guid::generate(), Some(Uid::effective().into()), None)
                .unwrap();

        crate::utils::block_on(p0.write_all(b"\0AUTH EXTERNAL\r\nDATA\r\nBEGIN\r\n")).unwrap();
        crate::utils::block_on(poll_fn(|cx| server.advance_handshake(cx))).unwrap();

        server.try_finish().unwrap();
    }

    #[test]
    #[timeout(15000)]
    fn anonymous_handshake() {
        let (mut p0, p1) = create_async_socket_pair();
        let mut server = ServerHandshake::new(
            p1,
            Guid::generate(),
            Some(Uid::effective().into()),
            Some(vec![AuthMechanism::Anonymous].into()),
        )
        .unwrap();

        crate::utils::block_on(p0.write_all(b"\0AUTH ANONYMOUS abcd\r\nBEGIN\r\n")).unwrap();
        crate::utils::block_on(poll_fn(|cx| server.advance_handshake(cx))).unwrap();

        server.try_finish().unwrap();
    }

    #[test]
    #[timeout(15000)]
    fn separate_anonymous_data() {
        let (mut p0, p1) = create_async_socket_pair();
        let mut server = ServerHandshake::new(
            p1,
            Guid::generate(),
            Some(Uid::effective().into()),
            Some(vec![AuthMechanism::Anonymous].into()),
        )
        .unwrap();

        crate::utils::block_on(p0.write_all(b"\0AUTH ANONYMOUS\r\nDATA abcd\r\nBEGIN\r\n"))
            .unwrap();
        crate::utils::block_on(poll_fn(|cx| server.advance_handshake(cx))).unwrap();

        server.try_finish().unwrap();
    }
}
