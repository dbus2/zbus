use std::cell::{Cell, RefCell};
use std::convert::TryInto;
use std::env;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::rc::Rc;
use std::str::FromStr;

use nix::unistd::Uid;
use once_cell::unsync::OnceCell;

use crate::address::Address;
use crate::utils::{read_exact, write_all};
use crate::{fdo, Error, Guid, Message, MessageError, MessageType, Result, MIN_MESSAGE_SIZE};

type MessageHandlerFn = Box<dyn FnMut(Message) -> Option<Message>>;

const DEFAULT_MAX_QUEUED: usize = 32;

#[derive(derivative::Derivative)]
#[derivative(Debug)]
struct ConnectionInner {
    server_guid: Guid,
    cap_unix_fd: bool,
    unique_name: OnceCell<String>,

    stream: UnixStream,
    // Serial number for next outgoing message
    serial: Cell<u32>,

    // Queue of incoming messages
    incoming_queue: RefCell<Vec<Message>>,

    // Max number of messages to queue
    max_queued: Cell<usize>,

    #[derivative(Debug = "ignore")]
    default_msg_handler: RefCell<Option<MessageHandlerFn>>,
}

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
/// `Connection` implements [`Clone`] and cloning it is a very cheap operation, as the underlying
/// data is not cloned. This makes it very convenient to share the connection between different
/// parts of your code. Please note however, that sharing or sending of a connection instance
/// across threads is not supported. If you've a valid use cas for that, please [file an issue]
/// about it and we'll consider adding this feature.
///
/// Since there are times when important messages arrive between a method call message is sent and
/// its reply is received, `Connection` keeps an internal queue of incoming messages so that these
/// messages are not lost and subsequent calls to [`receive_message`] will retreive messages from
/// this queue first. The size of this queue is configurable through the [`set_max_queued`] method.
/// The default size is 32. All messages that are received after the queue is full, are dropped.
///
/// [method calls]: struct.Connection.html#method.call_method
/// [signals]: struct.Connection.html#method.emit_signal
/// [`new_system`]: struct.Connection.html#method.new_system
/// [`new_session`]: struct.Connection.html#method.new_session
/// [`Proxy`]: struct.Proxy.html
/// [`ObjectServer`]: struct.ObjectServer.html
/// [`dbus_proxy`]: attr.dbus_proxy.html
/// [`dbus_interface`]: attr.dbus_interface.html
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [file an issue]: https://gitlab.freedesktop.org/zeenix/zbus/-/issues/new
/// [`receive_message`]: struct.Connection.html#method.receive_message
/// [`set_max_queued`]: struct.Connection.html#method.set_max_queued
#[derive(Debug, Clone)]
pub struct Connection(Rc<ConnectionInner>);

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        self.0.stream.as_raw_fd()
    }
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

        let connection = Connection::new_authenticated(stream, server_guid, cap_unix_fd);

        if bus_connection {
            // Now that the server has approved us, we must send the bus Hello, as per specs
            let name = fdo::DBusProxy::new(&connection)?
                .hello()
                .map_err(|e| Error::Handshake(format!("Hello failed: {}", e)))?;
            connection
                .0
                .unique_name
                .set(name)
                // programmer (probably our) error if this fails.
                .expect("Attempted to set unique_name twice");
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
        Self::new_unix_client(Address::from_str(address)?.connect()?, bus_connection)
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

    /// Max number of messages to queue.
    pub fn max_queued(&self) -> usize {
        self.0.max_queued.get()
    }

    /// Set the max number of messages to queue.
    ///
    /// Since typically you'd want to set this at instantiation time, this method takes ownership
    /// of `self` and returns an owned `Connection` instance so you can use the builder pattern to
    /// set the value.
    ///
    /// # Example
    ///
    /// ```
    ///# use std::error::Error;
    ///#
    /// let conn = zbus::Connection::new_session()?.set_max_queued(30);
    /// assert_eq!(conn.max_queued(), 30);
    ///
    /// // Do something usefull with `conn`..
    ///# Ok::<_, Box<dyn Error + Send + Sync>>(())
    /// ```
    pub fn set_max_queued(self, max: usize) -> Self {
        self.0.max_queued.replace(max);

        self
    }

    /// The server's GUID.
    pub fn server_guid(&self) -> &str {
        self.0.server_guid.as_str()
    }

    /// The unique name as assigned by the message bus or `None` if not a message bus connection.
    pub fn unique_name(&self) -> Option<&str> {
        self.0.unique_name.get().map(|s| s.as_str())
    }

    /// Fetch the next message from the connection.
    ///
    /// Read from the connection until a message is received or an error is reached. Return the
    /// message on success. If there are pending messages in the queue, the first one from the
    /// queue is returned instead.
    ///
    /// If a default message handler has been registered on this connection through
    /// [`set_default_message_handler`], it will first get to decide the fate of the received
    /// message.
    ///
    /// [`set_default_message_handler`]: struct.Connection.html#method.set_default_message_handler
    pub fn receive_message(&self) -> Result<Message> {
        let mut queue = self.0.incoming_queue.borrow_mut();
        if let Some(msg) = queue.pop() {
            return Ok(msg);
        }

        let mut buf = [0; MIN_MESSAGE_SIZE];

        loop {
            let mut fds = read_exact(&self.0.stream, &mut buf[..])?;

            let mut incoming = Message::from_bytes(&buf)?;
            let bytes_left = incoming.bytes_to_completion()?;
            if bytes_left == 0 {
                return Err(Error::Message(MessageError::InsufficientData));
            }
            let mut buf = vec![0; bytes_left as usize];
            fds.append(&mut read_exact(&self.0.stream, &mut buf[..])?);
            incoming.add_bytes(&buf[..])?;
            incoming.set_owned_fds(fds);

            if let Some(ref mut handler) = &mut *self.0.default_msg_handler.borrow_mut() {
                // Let's see if the default handler wants the message first
                match handler(incoming) {
                    // Message was returned to us so we can return that
                    Some(m) => return Ok(m),
                    None => continue,
                }
            }

            return Ok(incoming);
        }
    }

    /// Send `msg` to the peer.
    ///
    /// The connection sets a unique serial number on the message before sending it off.
    ///
    /// On successfully sending off `msg`, the assigned serial number is returned.
    pub fn send_message(&self, mut msg: Message) -> Result<u32> {
        if !msg.fds().is_empty() && !self.0.cap_unix_fd {
            return Err(Error::Unsupported);
        }

        let serial = self.next_serial();
        msg.modify_primary_header(|primary| {
            primary.set_serial_num(serial);

            Ok(())
        })?;

        write_all(&self.0.stream, msg.as_bytes(), &msg.fds())?;
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
            self.unique_name(),
            destination,
            path,
            iface,
            method_name,
            body,
        )?;

        let serial = self.send_message(m)?;
        let mut tmp_queue = vec![];

        loop {
            let m = self.receive_message()?;
            let h = m.header()?;

            if h.reply_serial()? != Some(serial) {
                let queue = self.0.incoming_queue.borrow();
                if queue.len() + tmp_queue.len() < self.0.max_queued.get() {
                    // We first push to a temporary queue as otherwise it'll create an infinite loop
                    // since subsequent `receive_message` call will pick up the message from the main
                    // queue.
                    tmp_queue.push(m);
                }

                continue;
            } else {
                self.0.incoming_queue.borrow_mut().append(&mut tmp_queue);
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
            self.unique_name(),
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
        let m = Message::method_reply(self.unique_name(), call, body)?;
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
        let m = Message::method_error(self.unique_name(), call, error_name, body)?;
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
        self.0.default_msg_handler.borrow_mut().replace(handler);
    }

    /// Reset the default message handler.
    ///
    /// Remove the previously set message handler from `set_default_message_handler`.
    pub fn reset_default_message_handler(&mut self) {
        self.0.default_msg_handler.borrow_mut().take();
    }

    fn new_authenticated(stream: UnixStream, server_guid: Guid, cap_unix_fd: bool) -> Self {
        Self(Rc::new(ConnectionInner {
            stream,
            server_guid,
            cap_unix_fd,
            serial: Cell::new(1),
            unique_name: OnceCell::new(),
            incoming_queue: RefCell::new(vec![]),
            max_queued: Cell::new(DEFAULT_MAX_QUEUED),
            default_msg_handler: RefCell::new(None),
        }))
    }

    fn next_serial(&self) -> u32 {
        let next = self.0.serial.get() + 1;

        self.0.serial.replace(next)
    }
}

/// Get a session socket respecting the DBUS_SESSION_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// /run/user/UID/bus
fn session_socket() -> Result<UnixStream> {
    match env::var("DBUS_SESSION_BUS_ADDRESS") {
        Ok(val) => Address::from_str(&val)?.connect(),
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
        Ok(val) => Address::from_str(&val)?.connect(),
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

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixStream;
    use std::thread;

    use crate::{Connection, Guid};

    #[test]
    fn unix_p2p() {
        let guid = Guid::generate();

        let (p0, p1) = UnixStream::pair().unwrap();

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

    #[test]
    fn serial_monotonically_increases() {
        let c = Connection::new_session().unwrap();
        let serial = c.next_serial() + 1;

        for next in serial..serial + 10 {
            assert_eq!(next, c.next_serial());
        }
    }
}
