use std::{
    collections::VecDeque,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use nix::{poll::PollFlags, unistd::Uid};

use crate::{
    guid::Guid,
    raw::{Connection, Socket},
    utils::wait_on,
    Error, Result,
};

/*
 * Client-side handshake logic
 */

#[derive(Clone, Copy, Debug, PartialEq)]
enum ClientHandshakeStep {
    Init,
    MechanismInit,
    WaitingForData,
    WaitingForOK,
    WaitingForAgreeUnixFD,
    Done,
}

pub enum IoOperation {
    None,
    Read,
    Write,
}

// See <https://dbus.freedesktop.org/doc/dbus-specification.html#auth-mechanisms>
#[derive(Clone, Copy, Debug)]
enum Mechanism {
    External,
    Cookie,
    Anonymous,
}

// The plain-text SASL profile authentication protocol described here:
// <https://dbus.freedesktop.org/doc/dbus-specification.html#auth-protocol>
//
// These are all the known commands, which can be parsed from or serialized to text.
#[derive(Debug)]
enum Command {
    Auth(Option<Mechanism>, Option<String>),
    Cancel,
    Begin,
    Data(Vec<u8>),
    Error(String),
    NegotiateUnixFD,
    Rejected(Vec<Mechanism>),
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
/// If handling the handshake asynchronously is not necessary, the [`blocking_finish`] method is provided
/// which blocks until the handshake is completed or an error occurs.
///
/// [`advance_handshake`]: struct.ClientHandshake.html#method.advance_handshake
/// [`try_finish`]: struct.ClientHandshake.html#method.try_finish
/// [`Authenticated`]: struct.AUthenticated.html
/// [`Connection::new_authenticated`]: ../struct.Connection.html#method.new_authenticated
/// [`blocking_finish`]: struct.ClientHandshake.html#method.blocking_finish
#[derive(Debug)]
pub struct ClientHandshake<S> {
    socket: S,
    recv_buffer: Vec<u8>,
    send_buffer: Vec<u8>,
    step: ClientHandshakeStep,
    server_guid: Option<Guid>,
    cap_unix_fd: bool,
    // the current AUTH mechanism is front, ordered by priority
    mechanisms: VecDeque<Mechanism>,
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
    pub(crate) cap_unix_fd: bool,
}

pub trait Handshake<S> {
    /// Block and automatically drive the handshake for this server
    ///
    /// This method will block until the handshake is finalized, even if the
    /// socket is in non-blocking mode.
    fn blocking_finish(self) -> Result<Authenticated<S>>;

    /// The next I/O operation needed for advancing the handshake.
    ///
    /// If [`Handshake::advance_handshake`] returns a `std::io::ErrorKind::WouldBlock` error, you
    /// can use this to figure out which operation to poll for, before calling `advance_handshake`
    /// again.
    fn next_io_operation(&self) -> IoOperation;

    /// Attempt to advance the handshake
    ///
    /// In non-blocking mode, you need to invoke this method repeatedly
    /// until it returns `Ok(())`. Once it does, the handshake is finished
    /// and you can invoke the [`Handshake::try_finish`] method.
    ///
    /// Note that only the intial handshake is done. If you need to send a
    /// Bus Hello, this remains to be done.
    fn advance_handshake(&mut self) -> Result<()>;

    /// Attempt to finalize this handshake into an initialized client.
    ///
    /// This method should only be called once `advance_handshake()` has
    /// returned `Ok(())`. Otherwise it'll error and return you the object.
    fn try_finish(self) -> std::result::Result<Authenticated<S>, Self>
    where
        Self: Sized;

    /// Access the socket backing this handshake
    ///
    /// Would typically be used to register it for readiness.
    fn socket(&self) -> &S;
}

impl<S: Socket> ClientHandshake<S> {
    /// Start a handsake on this client socket
    pub fn new(socket: S) -> ClientHandshake<S> {
        let mut mechanisms = VecDeque::new();
        mechanisms.push_back(Mechanism::External);
        mechanisms.push_back(Mechanism::Cookie);
        ClientHandshake {
            socket,
            recv_buffer: Vec::new(),
            send_buffer: Vec::new(),
            step: ClientHandshakeStep::Init,
            server_guid: None,
            cap_unix_fd: false,
            mechanisms,
        }
    }

    fn flush_buffer(&mut self) -> Result<()> {
        while !self.send_buffer.is_empty() {
            let written = self.socket.sendmsg(&self.send_buffer, &[])?;
            self.send_buffer.drain(..written);
        }
        Ok(())
    }

    fn read_command(&mut self) -> Result<Command> {
        self.recv_buffer.clear(); // maybe until \r\n instead?
        while !self.recv_buffer.ends_with(b"\r\n") {
            let mut buf = [0; 40];
            let (read, fds) = self.socket.recvmsg(&mut buf)?;
            if !fds.is_empty() {
                return Err(Error::Handshake("Unexecpted FDs during handshake".into()));
            }
            self.recv_buffer.extend(&buf[..read]);
        }

        let line = String::from_utf8_lossy(&self.recv_buffer);
        line.parse()
    }

    fn mechanism(&self) -> Result<&Mechanism> {
        self.mechanisms
            .front()
            .ok_or_else(|| Error::Handshake("Exhausted available AUTH mechanisms".into()))
    }

    fn mechanism_init(&mut self) -> Result<(ClientHandshakeStep, Command)> {
        use ClientHandshakeStep::*;
        let mech = self.mechanism()?;
        match mech {
            Mechanism::External => Ok((
                WaitingForOK,
                Command::Auth(Some(*mech), Some(sasl_auth_id())),
            )),
            Mechanism::Cookie => Ok((
                WaitingForData,
                Command::Auth(Some(*mech), Some(sasl_auth_id())),
            )),
            _ => Err(Error::Handshake("Unimplemented AUTH mechanisms".into())),
        }
    }

    fn mechanism_data(&mut self, data: Vec<u8>) -> Result<(ClientHandshakeStep, Command)> {
        use ClientHandshakeStep::*;
        let mech = self.mechanism()?;
        match mech {
            Mechanism::Cookie => {
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
                let sec = format!("{}:{}:{}", server_chall, client_chall, cookie);
                let sha1 = sha1::Sha1::from(sec).hexdigest();
                let data = format!("{} {}", client_chall, sha1);
                Ok((WaitingForOK, Command::Data(data.into())))
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

fn sasl_auth_id() -> String {
    Uid::current()
        .to_string()
        .chars()
        .map(|c| format!("{:x}", c as u32))
        .collect::<String>()
}

#[derive(Debug)]
struct Cookie {
    id: String,
    creation_time: String,
    cookie: String,
}

impl Cookie {
    fn keyring_path() -> Result<PathBuf> {
        let home = std::env::var("HOME")
            .map_err(|e| Error::Handshake(format!("Failed to read $HOME: {}", e)))?;
        let mut path = PathBuf::new();
        path.push(home);
        path.push(".dbus-keyrings");
        Ok(path)
    }

    fn read_keyring(name: &str) -> Result<Vec<Cookie>> {
        use std::os::unix::fs::PermissionsExt;

        let mut path = Cookie::keyring_path()?;
        let perms = std::fs::metadata(&path)?.permissions().mode();
        if perms & 0o066 != 0 {
            return Err(Error::Handshake(
                "DBus keyring has invalid permissions".into(),
            ));
        }
        path.push(name);
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
            let creation_time = split
                .next()
                .ok_or_else(|| {
                    Error::Handshake(format!(
                        "DBus cookie `{}` missing creation time at line {}",
                        path.to_str().unwrap(),
                        n
                    ))
                })?
                .to_string();
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
            cookies.push(Cookie {
                id,
                creation_time,
                cookie,
            })
        }
        Ok(cookies)
    }

    fn lookup(name: &str, id: &str) -> Result<String> {
        let keyring = Self::read_keyring(name)?;
        let c = keyring
            .iter()
            .find(|c| c.id == id)
            .ok_or_else(|| Error::Handshake(format!("DBus cookie ID {} not found", id)))?;
        Ok(c.cookie.to_string())
    }
}

impl<S: Socket> Handshake<S> for ClientHandshake<S> {
    fn blocking_finish(mut self) -> Result<Authenticated<S>> {
        loop {
            match self.advance_handshake() {
                Ok(()) => return Ok(self.try_finish().unwrap_or_else(|_| unreachable!())),
                Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // we raised a WouldBlock error, this means this is a non-blocking socket
                    // we use poll to wait until the action we need is available
                    let flags = match self.next_io_operation() {
                        IoOperation::Write => PollFlags::POLLOUT,
                        IoOperation::Read => PollFlags::POLLIN,
                        _ => unreachable!(),
                    };
                    wait_on(self.socket.as_raw_fd(), flags)?;
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn next_io_operation(&self) -> IoOperation {
        use ClientHandshakeStep::*;
        if self.send_buffer.is_empty() {
            match self.step {
                WaitingForOK | WaitingForData | WaitingForAgreeUnixFD => IoOperation::Read,
                Init | MechanismInit | Done => IoOperation::None,
            }
        } else {
            IoOperation::Write
        }
    }

    fn advance_handshake(&mut self) -> Result<()> {
        use ClientHandshakeStep::*;
        loop {
            self.flush_buffer()?;
            let (next_step, cmd) = match self.step {
                Init | MechanismInit => self.mechanism_init()?,
                WaitingForData | WaitingForOK => {
                    let reply = self.read_command()?;
                    match (self.step, reply) {
                        (_, Command::Data(data)) => self.mechanism_data(data)?,
                        (_, Command::Rejected(_)) => {
                            self.mechanisms.pop_front();
                            self.step = MechanismInit;
                            continue;
                        }
                        (WaitingForOK, Command::Ok(guid)) => {
                            self.server_guid = Some(guid);
                            (WaitingForAgreeUnixFD, Command::NegotiateUnixFD)
                        }
                        (_, reply) => {
                            return Err(Error::Handshake(format!(
                                "Unexpected server AUTH OK reply: {}",
                                reply
                            )))
                        }
                    }
                }
                WaitingForAgreeUnixFD => {
                    let reply = self.read_command()?;
                    match reply {
                        Command::AgreeUnixFD => self.cap_unix_fd = true,
                        Command::Error(_) => self.cap_unix_fd = false,
                        _ => {
                            return Err(Error::Handshake(format!(
                                "Unexpected server UNIX_FD reply: {}",
                                reply
                            )));
                        }
                    }
                    (Done, Command::Begin)
                }
                Done => return Ok(()),
            };
            self.send_buffer = if self.step == Init {
                format!("\0{}", cmd).into()
            } else {
                cmd.into()
            };
            // The dbus daemon on these platforms currently requires sending the zero byte
            // as a separate message with SCM_CREDS
            #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
            if self.step == Init {
                use nix::sys::socket::{sendmsg, ControlMessage, MsgFlags};

                // Steal the leading null byte from the buffer.
                let zero = &[self.send_buffer.drain(0..1).next().unwrap()];
                let iov = [nix::sys::uio::IoVec::from_slice(zero)];

                if sendmsg(
                    self.socket.as_raw_fd(),
                    &iov,
                    &[ControlMessage::ScmCreds],
                    MsgFlags::empty(),
                    None,
                ) != Ok(1)
                {
                    return Err(Error::Handshake(
                        "Could not send zero byte with credentials".to_string(),
                    ));
                }
            }
            self.step = next_step;
        }
    }

    fn try_finish(self) -> std::result::Result<Authenticated<S>, Self> {
        if let ClientHandshakeStep::Done = self.step {
            Ok(Authenticated {
                conn: Connection::wrap(self.socket),
                server_guid: self.server_guid.unwrap(),
                cap_unix_fd: self.cap_unix_fd,
            })
        } else {
            Err(self)
        }
    }

    fn socket(&self) -> &S {
        &self.socket
    }
}

/*
 * Server-side handshake logic
 */

#[derive(Debug)]
enum ServerHandshakeStep {
    WaitingForNull,
    WaitingForAuth,
    SendingAuthOK,
    SendingAuthError,
    WaitingForBegin,
    SendingBeginMessage,
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
/// If handling the handshake asynchronously is not necessary, the [`blocking_finish`] method is provided
/// which blocks until the handshake is completed or an error occurs.
///
/// [`advance_handshake`]: struct.ServerHandshake.html#method.advance_handshake
/// [`try_finish`]: struct.ServerHandshake.html#method.try_finish
/// [`Authenticated`]: struct.Authenticated.html
/// [`Connection::new_authenticated`]: ../struct.Connection.html#method.new_authenticated
/// [`blocking_finish`]: struct.ServerHandshake.html#method.blocking_finish
#[derive(Debug)]
pub struct ServerHandshake<S> {
    socket: S,
    buffer: Vec<u8>,
    step: ServerHandshakeStep,
    server_guid: Guid,
    cap_unix_fd: bool,
    client_uid: u32,
}

impl<S: Socket> ServerHandshake<S> {
    pub fn new(socket: S, guid: Guid, client_uid: u32) -> ServerHandshake<S> {
        ServerHandshake {
            socket,
            buffer: Vec::new(),
            step: ServerHandshakeStep::WaitingForNull,
            server_guid: guid,
            cap_unix_fd: false,
            client_uid,
        }
    }

    fn flush_buffer(&mut self) -> Result<()> {
        while !self.buffer.is_empty() {
            let written = self.socket.sendmsg(&self.buffer, &[])?;
            self.buffer.drain(..written);
        }
        Ok(())
    }

    fn read_command(&mut self) -> Result<()> {
        while !self.buffer.ends_with(b"\r\n") {
            let mut buf = [0; 40];
            let (read, _) = self.socket.recvmsg(&mut buf)?;
            self.buffer.extend(&buf[..read]);
        }
        Ok(())
    }
}

impl<S: Socket> Handshake<S> for ServerHandshake<S> {
    fn blocking_finish(mut self) -> Result<Authenticated<S>> {
        loop {
            match self.advance_handshake() {
                Ok(()) => return Ok(self.try_finish().unwrap_or_else(|_| unreachable!())),
                Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // we raised a WouldBlock error, this means this is a non-blocking socket
                    // we use poll to wait until the action we need is available
                    let flags = match self.step {
                        ServerHandshakeStep::SendingAuthError
                        | ServerHandshakeStep::SendingAuthOK
                        | ServerHandshakeStep::SendingBeginMessage => PollFlags::POLLOUT,
                        ServerHandshakeStep::WaitingForNull
                        | ServerHandshakeStep::WaitingForBegin
                        | ServerHandshakeStep::WaitingForAuth => PollFlags::POLLIN,
                        ServerHandshakeStep::Done => unreachable!(),
                    };
                    wait_on(self.socket.as_raw_fd(), flags)?;
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn next_io_operation(&self) -> IoOperation {
        match self.step {
            ServerHandshakeStep::Done => IoOperation::None,
            ServerHandshakeStep::WaitingForNull
            | ServerHandshakeStep::WaitingForAuth
            | ServerHandshakeStep::WaitingForBegin => IoOperation::Read,
            ServerHandshakeStep::SendingAuthOK
            | ServerHandshakeStep::SendingAuthError
            | ServerHandshakeStep::SendingBeginMessage => IoOperation::Write,
        }
    }

    fn advance_handshake(&mut self) -> Result<()> {
        loop {
            match self.step {
                ServerHandshakeStep::WaitingForNull => {
                    let mut buffer = [0; 1];
                    let (read, _) = self.socket.recvmsg(&mut buffer)?;
                    // recvmsg cannot return anything else than Ok(1) or Err
                    debug_assert!(read == 1);
                    if buffer[0] != 0 {
                        return Err(Error::Handshake(
                            "First client byte is not NUL!".to_string(),
                        ));
                    }
                    self.step = ServerHandshakeStep::WaitingForAuth;
                }
                ServerHandshakeStep::WaitingForAuth => {
                    self.read_command()?;
                    let mut reply = String::new();
                    (&self.buffer[..]).read_line(&mut reply)?;
                    let mut words = reply.split_whitespace();
                    match (words.next(), words.next(), words.next(), words.next()) {
                        (Some("AUTH"), Some("EXTERNAL"), Some(uid), None) => {
                            let uid = id_from_str(uid)
                                .map_err(|e| Error::Handshake(format!("Invalid UID: {}", e)))?;
                            if uid == self.client_uid {
                                self.buffer = format!("OK {}\r\n", self.server_guid).into();
                                self.step = ServerHandshakeStep::SendingAuthOK;
                            } else {
                                self.buffer = Vec::from(&b"REJECTED EXTERNAL\r\n"[..]);
                                self.step = ServerHandshakeStep::SendingAuthError;
                            }
                        }
                        (Some("AUTH"), _, _, _) | (Some("ERROR"), _, _, _) => {
                            self.buffer = Vec::from(&b"REJECTED EXTERNAL\r\n"[..]);
                            self.step = ServerHandshakeStep::SendingAuthError;
                        }
                        (Some("BEGIN"), None, None, None) => {
                            return Err(Error::Handshake(
                                "Received BEGIN while not authenticated".to_string(),
                            ));
                        }
                        _ => {
                            self.buffer = Vec::from(&b"ERROR Unsupported command\r\n"[..]);
                            self.step = ServerHandshakeStep::SendingAuthError;
                        }
                    }
                }
                ServerHandshakeStep::SendingAuthError => {
                    self.flush_buffer()?;
                    self.step = ServerHandshakeStep::WaitingForAuth;
                }
                ServerHandshakeStep::SendingAuthOK => {
                    self.flush_buffer()?;
                    self.step = ServerHandshakeStep::WaitingForBegin;
                }
                ServerHandshakeStep::WaitingForBegin => {
                    self.read_command()?;
                    let mut reply = String::new();
                    (&self.buffer[..]).read_line(&mut reply)?;
                    let mut words = reply.split_whitespace();
                    match (words.next(), words.next()) {
                        (Some("BEGIN"), None) => {
                            self.step = ServerHandshakeStep::Done;
                        }
                        (Some("CANCEL"), None) => {
                            self.buffer = Vec::from(&b"REJECTED EXTERNAL\r\n"[..]);
                            self.step = ServerHandshakeStep::SendingAuthError;
                        }
                        (Some("ERROR"), _) => {
                            self.buffer = Vec::from(&b"REJECTED EXTERNAL\r\n"[..]);
                            self.step = ServerHandshakeStep::SendingAuthError;
                        }
                        (Some("NEGOTIATE_UNIX_FD"), None) => {
                            self.cap_unix_fd = true;
                            self.buffer = Vec::from(&b"AGREE_UNIX_FD\r\n"[..]);
                            self.step = ServerHandshakeStep::SendingBeginMessage;
                        }
                        _ => {
                            self.buffer = Vec::from(&b"ERROR Unsupported command\r\n"[..]);
                            self.step = ServerHandshakeStep::SendingBeginMessage;
                        }
                    }
                }
                ServerHandshakeStep::SendingBeginMessage => {
                    self.flush_buffer()?;
                    self.step = ServerHandshakeStep::WaitingForBegin;
                }
                ServerHandshakeStep::Done => return Ok(()),
            }
        }
    }

    fn try_finish(self) -> std::result::Result<Authenticated<S>, Self> {
        if let ServerHandshakeStep::Done = self.step {
            Ok(Authenticated {
                conn: Connection::wrap(self.socket),
                server_guid: self.server_guid,
                cap_unix_fd: self.cap_unix_fd,
            })
        } else {
            Err(self)
        }
    }

    fn socket(&self) -> &S {
        &self.socket
    }
}

fn id_from_str(s: &str) -> std::result::Result<u32, Box<dyn std::error::Error>> {
    let mut id = String::new();
    for s in s.as_bytes().chunks(2) {
        let c = char::from(u8::from_str_radix(std::str::from_utf8(s)?, 16)?);
        id.push(c);
    }
    Ok(id.parse::<u32>()?)
}

impl fmt::Display for Mechanism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mech = match self {
            Mechanism::External => "EXTERNAL",
            Mechanism::Cookie => "DBUS_COOKIE_SHA1",
            Mechanism::Anonymous => "ANONYMOUS",
        };
        write!(f, "{}", mech)
    }
}

impl FromStr for Mechanism {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "EXTERNAL" => Ok(Mechanism::External),
            "DBUS_COOKIE_SHA1" => Ok(Mechanism::Cookie),
            "ANONYMOUS" => Ok(Mechanism::Anonymous),
            _ => Err(Error::Handshake(format!("Unknown mechanism: {}", s))),
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
                (Some(mech), Some(resp)) => format!("AUTH {} {}", mech, resp),
                (Some(mech), None) => format!("AUTH {}", mech),
                _ => "AUTH".into(),
            },
            Command::Cancel => "CANCEL".into(),
            Command::Begin => "BEGIN".into(),
            Command::Data(data) => {
                format!("DATA {}", hex::encode(data))
            }
            Command::Error(expl) => {
                format!("ERROR {}", expl)
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
                format!("OK {}", guid)
            }
            Command::AgreeUnixFD => "AGREE_UNIX_FD".into(),
        };
        write!(f, "{}\r\n", cmd)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::Handshake(format!("Invalid hexcode: {}", e))
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
                let resp = words.next().map(|s| s.into());
                Command::Auth(mech, resp)
            }
            Some("CANCEL") => Command::Cancel,
            Some("BEGIN") => Command::Begin,
            Some("DATA") => {
                let data = words
                    .next()
                    .ok_or_else(|| Error::Handshake("Missing DATA data".into()))?;
                Command::Data(hex::decode(data)?)
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
            _ => return Err(Error::Handshake(format!("Unknown command: {}", s))),
        };
        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixStream;

    use super::*;

    use crate::Guid;

    #[test]
    fn handshake() {
        // a pair of non-blocking connection UnixStream
        let (p0, p1) = UnixStream::pair().unwrap();
        p0.set_nonblocking(true).unwrap();
        p1.set_nonblocking(true).unwrap();

        // initialize both handshakes
        let mut client = ClientHandshake::new(p0);
        let mut server = ServerHandshake::new(p1, Guid::generate(), Uid::current().into());

        // proceed to the handshakes
        let mut client_done = false;
        let mut server_done = false;
        while !(client_done && server_done) {
            match client.advance_handshake() {
                Ok(()) => client_done = true,
                Err(Error::Io(e)) => assert!(e.kind() == std::io::ErrorKind::WouldBlock),
                Err(e) => panic!("Unexpected error: {:?}", e),
            }

            match server.advance_handshake() {
                Ok(()) => server_done = true,
                Err(Error::Io(e)) => assert!(e.kind() == std::io::ErrorKind::WouldBlock),
                Err(e) => panic!("Unexpected error: {:?}", e),
            }
        }

        let client = client.try_finish().unwrap();
        let server = server.try_finish().unwrap();

        assert_eq!(client.server_guid, server.server_guid);
        assert_eq!(client.cap_unix_fd, server.cap_unix_fd);
    }
}
