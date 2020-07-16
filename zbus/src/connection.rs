use std::cell::RefCell;
use std::convert::TryInto;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::sync::atomic::{AtomicU32, Ordering};
use std::{env, io};

use nix::unistd::Uid;

use crate::address::{self, Address};
use crate::utils::{read_exact, write_all};
use crate::{fdo, Error, Guid, Message, MessageError, MessageType, Result, MIN_MESSAGE_SIZE};

type MessageHandlerFn = Box<dyn FnMut(Message) -> Option<Message>>;

/// A D-Bus connection.
///
/// A connection to a D-Bus bus, or a direct peer.
///
/// Once created, the connection is authenticated and negotiated and messages can be sent or
/// received, such as [method calls] or [signals].
///
/// For higher-level message handling (typed functions, introspection, documentation reasons etc),
/// it is recommended to wrap the low-level D-Bus messages into Rust functions with the
/// [`dbus_proxy`] and [`dbus_interface`] macros instead of doing it directly on a `Connection`.
///
/// Typically, a connection is made to the session bus with [`new_session`], or to the system bus
/// with [`new_system`]. Then the connection is shared with the [`Proxy`] and [`ObjectServer`]
/// instances.
///
/// [method calls]: struct.Connection.html#method.call_method
/// [signals]: struct.Connection.html#method.emit_signal
/// [`new_system`]: struct.Connection.html#method.new_system
/// [`new_session`]: struct.Connection.html#method.new_session
/// [`Proxy`]: struct.Proxy.html
/// [`ObjectServer`]: struct.ObjectServer.html
/// [`dbus_proxy`]: attr.dbus_proxy.html
/// [`dbus_interface`]: attr.dbus_interface.html
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Connection {
    server_guid: Guid,
    cap_unix_fd: bool,
    unique_name: Option<String>,

    stream: UnixStream,
    // Serial number for next outgoing message
    serial: AtomicU32,

    #[derivative(Debug = "ignore")]
    default_msg_handler: Option<RefCell<MessageHandlerFn>>,
}

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        self.stream.as_raw_fd()
    }
}

fn connect(addr: &Address) -> Result<UnixStream> {
    match addr {
        Address::Path(p) => Ok(UnixStream::connect(p)?),
        Address::Abstract(_) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "abstract sockets are not currently supported",
        )
        .into()),
    }
}

/// Get a session socket respecting the DBUS_SESSION_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// /run/user/UID/bus
fn session_socket() -> Result<UnixStream> {
    match env::var("DBUS_SESSION_BUS_ADDRESS") {
        Ok(val) => connect(&address::parse_dbus_address(&val)?),
        _ => {
            let uid = Uid::current();
            let path = format!("/run/user/{}/bus", uid);
            Ok(UnixStream::connect(path)?)
        }
    }
}

/// Get a system socket respecting the DBUS_SYSTEM_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// /var/run/dbus/system_bus_socket
fn system_socket() -> Result<UnixStream> {
    match env::var("DBUS_SYSTEM_BUS_ADDRESS") {
        Ok(val) => connect(&address::parse_dbus_address(&val)?),
        _ => Ok(UnixStream::connect("/var/run/dbus/system_bus_socket")?),
    }
}

fn read_command<R: Read>(inner: R) -> Result<Vec<String>> {
    // Note: the stateful auth DBus protocol should guarantee that the server will not pipeline
    // answers, and thus bufreader shouldn't lose any data, hopefully.
    let mut buf_reader = BufReader::new(inner);
    let mut buf = String::new();

    buf_reader.read_line(&mut buf)?;
    let components = buf.split_whitespace();

    Ok(components.map(String::from).collect())
}

fn id_from_str(s: &str) -> std::result::Result<u32, Box<dyn std::error::Error>> {
    let mut id = String::new();
    for s in s.as_bytes().chunks(2) {
        let c = char::from(u8::from_str_radix(std::str::from_utf8(s)?, 16)?);
        id.push(c);
    }
    Ok(id.parse::<u32>()?)
}

enum ServerState {
    WaitingForAuth,
    // WaitingForData,
    WaitingForBegin,
}

impl Connection {
    /// Create and open a D-Bus connection from a `UnixStream`.
    ///
    /// The connection may either be set up for a *bus* connection, or not (for peer-to-peer
    /// communications).
    ///
    /// Upon successful return, the connection is fully established and negotiated: D-Bus messages
    /// can be sent and received.
    pub fn new_unix_client(mut stream: UnixStream, bus_connection: bool) -> Result<Self> {
        let uid = Uid::current();

        // SASL Handshake
        let uid_str = uid
            .to_string()
            .chars()
            .map(|c| format!("{:x}", c as u32))
            .collect::<String>();
        stream.write_all(format!("\0AUTH EXTERNAL {}\r\n", uid_str).as_bytes())?;
        let server_guid = match read_command(&stream)?.as_slice() {
            [ok, guid] if ok == "OK" => guid.as_str().try_into()?,
            _ => return Err(Error::Handshake("Unexpected server AUTH reply".to_string())),
        };

        stream.write_all(b"NEGOTIATE_UNIX_FD\r\n")?;
        let cap_unix_fd = match read_command(&stream)?.as_slice() {
            [agree] if agree == "AGREE_UNIX_FD" => true,
            [error] if error == "ERROR" => false,
            _ => {
                return Err(Error::Handshake(
                    "Unexpected server UNIX_FD reply".to_string(),
                ))
            }
        };

        stream.write_all(b"BEGIN\r\n")?;

        let mut connection = Connection::new_authenticated(stream, server_guid, cap_unix_fd);

        if bus_connection {
            // Now that the server has approved us, we must send the bus Hello, as per specs
            connection.unique_name = Some(fdo::DBusProxy::new(&connection)?.hello()?);
        }

        Ok(connection)
    }

    /// Create a `Connection` to the session/user message bus.
    pub fn new_session() -> Result<Self> {
        Self::new_unix_client(session_socket()?, true)
    }

    /// Create a `Connection` to the system-wide message bus.
    pub fn new_system() -> Result<Self> {
        Self::new_unix_client(system_socket()?, true)
    }

    /// Create a `Connection` for the given [D-Bus address].
    ///
    /// [D-Bus address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub fn new_for_address(address: &str, bus_connection: bool) -> Result<Self> {
        Self::new_unix_client(
            connect(&address::parse_dbus_address(address)?)?,
            bus_connection,
        )
    }

    /// Create a server `Connection` for the given `UnixStream` and the server `guid`.
    ///
    /// The connection will wait for incoming client authentication handshake & negotiation messages,
    /// for peer-to-peer communications.
    ///
    /// Upon successful return, the connection is fully established and negotiated: D-Bus messages
    /// can be sent and received.
    pub fn new_unix_server(mut stream: UnixStream, guid: &Guid) -> Result<Self> {
        use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

        let creds = getsockopt(stream.as_raw_fd(), PeerCredentials)
            .map_err(|e| Error::Handshake(format!("Failed to get peer credentials: {}", e)))?;

        // TODO: read byte with other credentials?
        let mut nul = [0; 1];
        stream.read_exact(&mut nul)?;
        if nul[0] != 0 {
            return Err(Error::Handshake(
                "First client byte is not NUL!".to_string(),
            ));
        }

        let mut state = ServerState::WaitingForAuth;
        let mut cap_unix_fd = false;
        loop {
            match read_command(&stream) {
                Ok(cmd) => match state {
                    ServerState::WaitingForAuth => match cmd.as_slice() {
                        [auth, ..] if auth == "AUTH" => {
                            if cmd.len() != 3 || cmd[1] != "EXTERNAL" {
                                stream.write_all(b"REJECTED EXTERNAL\r\n")?;
                            } else {
                                let uid = &cmd[2];
                                let uid = id_from_str(uid)
                                    .map_err(|e| Error::Handshake(format!("Invalid UID: {}", e)))?;
                                if uid != creds.uid() {
                                    stream.write_all(b"REJECTED EXTERNAL\r\n")?;
                                } else {
                                    stream.write_all(format!("OK {}\r\n", guid).as_bytes())?;
                                    state = ServerState::WaitingForBegin;
                                }
                            }
                        }
                        [begin] if begin == "BEGIN" => {
                            return Err(Error::Handshake(
                                "Received BEGIN while not authenticated".to_string(),
                            ));
                        }
                        [error, ..] if error == "ERROR" => {
                            stream.write_all(b"REJECTED EXTERNAL\r\n")?;
                        }
                        _ => {
                            stream.write_all(b"ERROR Unsupported command\r\n")?;
                        }
                    },
                    ServerState::WaitingForBegin => match cmd.as_slice() {
                        [begin] if begin == "BEGIN" => {
                            break;
                        }
                        [neg] if neg == "NEGOTIATE_UNIX_FD" => {
                            cap_unix_fd = true;
                            stream.write_all(b"AGREE_UNIX_FD\r\n")?;
                        }
                        [cancel] if cancel == "CANCEL" => {
                            stream.write_all(b"REJECTED EXTERNAL\r\n")?;
                            state = ServerState::WaitingForAuth;
                        }
                        [error, ..] if error == "ERROR" => {
                            stream.write_all(b"REJECTED EXTERNAL\r\n")?;
                            state = ServerState::WaitingForAuth;
                        }
                        _ => {
                            stream.write_all(b"ERROR Unsupported command\r\n")?;
                        }
                    },
                },
                Err(err) => return Err(Error::Handshake(format!("Read command failed: {}", err))),
            }
        }

        Ok(Self::new_authenticated(stream, guid.clone(), cap_unix_fd))
    }

    /// The server's GUID.
    pub fn server_guid(&self) -> &str {
        self.server_guid.as_str()
    }

    /// The unique name as assigned by the message bus or `None` if not a message bus connection.
    pub fn unique_name(&self) -> Option<&str> {
        self.unique_name.as_deref()
    }

    /// Fetch the next message from the connection.
    ///
    /// Read from the connection until a message is received or an error is reached. Return the
    /// message on success.
    ///
    /// If a default message handler has been registered on this connection through
    /// [`set_default_message_handler`], it will first get to decide the fate of the received
    /// message.
    ///
    /// [`set_default_message_handler`]: struct.Connection.html#method.set_default_message_handler
    pub fn receive_message(&self) -> Result<Message> {
        let mut buf = [0; MIN_MESSAGE_SIZE];

        loop {
            let mut fds = read_exact(&self.stream, &mut buf[..])?;

            let mut incoming = Message::from_bytes(&buf)?;
            let bytes_left = incoming.bytes_to_completion()?;
            if bytes_left == 0 {
                return Err(Error::Message(MessageError::InsufficientData));
            }
            let mut buf = vec![0; bytes_left as usize];
            fds.append(&mut read_exact(&self.stream, &mut buf[..])?);
            incoming.add_bytes(&buf[..])?;
            incoming.set_owned_fds(fds);

            if let Some(ref handler) = self.default_msg_handler {
                // Let's see if the default handler wants the message first
                match (&mut *handler.borrow_mut())(incoming) {
                    // Message was returned to us so we can return that
                    Some(m) => return Ok(m),
                    None => continue,
                }
            }

            return Ok(incoming);
        }
    }

    fn send_message(&self, mut msg: Message) -> Result<u32> {
        if !msg.fds().is_empty() && !self.cap_unix_fd {
            return Err(Error::Unsupported);
        }

        let serial = self.next_serial();
        msg.modify_primary_header(|primary| {
            primary.set_serial_num(serial);

            Ok(())
        })?;

        write_all(&self.stream, msg.as_bytes(), &msg.fds())?;
        Ok(serial)
    }

    /// Send a method call.
    ///
    /// Create a method-call message, send it over the connection, then wait for the reply. Incoming
    /// messages are received through [`receive_message`] (and by the default message handler)
    /// until the matching method reply (error or return) is received.
    ///
    /// On succesful reply, an `Ok(Message)` is returned. On error, an `Err` is returned. D-Bus
    /// error replies are returned as [`MethodError`].
    ///
    /// [`receive_message`]: struct.Connection.html#method.receive_message
    /// [`MethodError`]: enum.Error.html#variant.MethodError
    pub fn call_method<B>(
        &self,
        destination: Option<&str>,
        path: &str,
        iface: Option<&str>,
        method_name: &str,
        body: &B,
    ) -> Result<Message>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::method(
            self.unique_name.as_deref(),
            destination,
            path,
            iface,
            method_name,
            body,
        )?;

        let serial = self.send_message(m)?;

        loop {
            let m = self.receive_message()?;
            let h = m.header()?;

            if h.reply_serial()? != Some(serial) {
                continue;
            }

            match h.message_type()? {
                MessageType::Error => return Err(m.into()),
                MessageType::MethodReturn => return Ok(m),
                _ => (),
            }
        }
    }

    /// Emit a signal.
    ///
    /// Create a signal message, and send it over the connection.
    pub fn emit_signal<B>(
        &self,
        destination: Option<&str>,
        path: &str,
        iface: &str,
        signal_name: &str,
        body: &B,
    ) -> Result<()>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::signal(
            self.unique_name.as_deref(),
            destination,
            path,
            iface,
            signal_name,
            body,
        )?;

        self.send_message(m)?;

        Ok(())
    }

    /// Reply to a message.
    ///
    /// Given an existing message (likely a method call), send a reply back to the caller with the
    /// given `body`.
    ///
    /// Returns the message serial number.
    pub fn reply<B>(&self, call: &Message, body: &B) -> Result<u32>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::method_reply(self.unique_name.as_deref(), call, body)?;
        self.send_message(m)
    }

    /// Reply an error to a message.
    ///
    /// Given an existing message (likely a method call), send an error reply back to the caller
    /// with the given `error_name` and `body`.
    ///
    /// Returns the message serial number.
    pub fn reply_error<B>(&self, call: &Message, error_name: &str, body: &B) -> Result<u32>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let m = Message::method_error(self.unique_name.as_deref(), call, error_name, body)?;
        self.send_message(m)
    }

    /// Set a default handler for incoming messages on this connection.
    ///
    /// This is the handler that will be called on all messages received during [`receive_message`]
    /// call. If the handler callback returns a message (which could be a different message than it
    /// was given), `receive_message` will return it to its caller.
    ///
    /// [`receive_message`]: struct.Connection.html#method.receive_message
    pub fn set_default_message_handler(&mut self, handler: MessageHandlerFn) {
        self.default_msg_handler = Some(RefCell::new(handler));
    }

    /// Reset the default message handler.
    ///
    /// Remove the previously set message handler from `set_default_message_handler`.
    pub fn reset_default_message_handler(&mut self) {
        self.default_msg_handler = None;
    }

    fn new_authenticated(stream: UnixStream, server_guid: Guid, cap_unix_fd: bool) -> Self {
        Self {
            stream,
            server_guid,
            cap_unix_fd,
            serial: AtomicU32::new(1),
            unique_name: None,
            default_msg_handler: None,
        }
    }

    fn next_serial(&self) -> u32 {
        self.serial.fetch_add(1, Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
    use std::os::unix::io::FromRawFd;
    use std::os::unix::net::UnixStream;
    use std::thread;

    use crate::{Connection, Guid};

    #[test]
    fn unix_p2p() {
        let guid = Guid::generate();

        let sp = socketpair(
            AddressFamily::Unix,
            SockType::Stream,
            None,
            SockFlag::empty(),
        )
        .unwrap();
        let p0 = unsafe { UnixStream::from_raw_fd(sp.0) };
        let p1 = unsafe { UnixStream::from_raw_fd(sp.1) };

        let server_thread = thread::spawn(move || {
            let c = Connection::new_unix_server(p0, &guid).unwrap();
            let reply = c
                .call_method(None, "/", Some("org.zbus.p2p"), "Test", &())
                .unwrap();
            assert_eq!(reply.to_string(), "Method return");
            let val: String = reply.body().unwrap();
            val
        });

        let c = Connection::new_unix_client(p1, false).unwrap();
        let m = c.receive_message().unwrap();
        assert_eq!(m.to_string(), "Method call Test");
        c.reply(&m, &("yay")).unwrap();

        let val = server_thread.join().expect("failed to join server thread");
        assert_eq!(val, "yay");
    }
}
