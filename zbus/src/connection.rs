use std::cell::RefCell;
use std::convert::TryInto;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::sync::atomic::{AtomicU32, Ordering};
use std::{env, io};

use nix::unistd::Uid;

use crate::address::{self, Address};
use crate::utils::{read_exact, write_all};
use crate::{Error, Guid, Message, MessageType, Result, MIN_MESSAGE_SIZE};

type MessageHandlerFn = Box<dyn FnMut(Message) -> Option<Message>>;

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Connection {
    server_guid: Guid,
    cap_unix_fd: bool,
    unique_name: Option<String>,

    socket: UnixStream,
    // Serial number for next outgoing message
    serial: AtomicU32,

    #[derivative(Debug = "ignore")]
    default_msg_handler: Option<RefCell<MessageHandlerFn>>,
}

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        self.socket.as_raw_fd()
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

fn read_reply(socket: &UnixStream) -> Result<Vec<String>> {
    let mut buf_reader = BufReader::new(socket);
    let mut buf = String::new();

    buf_reader.read_line(&mut buf)?;
    let components = buf.split_whitespace();

    Ok(components.map(String::from).collect())
}

impl Connection {
    fn new(mut socket: UnixStream) -> Result<Self> {
        let uid = Uid::current();

        // SASL Handshake
        let uid_str = uid
            .to_string()
            .chars()
            .map(|c| format!("{:x}", c as u32))
            .collect::<String>();
        socket.write_all(format!("\0AUTH EXTERNAL {}\r\n", uid_str).as_bytes())?;
        let server_guid = match read_reply(&socket)?.as_slice() {
            [ok, guid] if ok == "OK" => guid.as_str().try_into()?,
            _ => return Err(Error::Handshake),
        };

        socket.write_all(b"NEGOTIATE_UNIX_FD\r\n")?;
        let cap_unix_fd = match read_reply(&socket)?.as_slice() {
            [agree] if agree == "AGREE_UNIX_FD" => true,
            [error] if error == "ERROR" => false,
            _ => return Err(Error::Handshake),
        };

        socket.write_all(b"BEGIN\r\n")?;

        let mut connection = Self {
            socket,
            server_guid,
            cap_unix_fd,
            serial: AtomicU32::new(1),
            unique_name: None,
            default_msg_handler: None,
        };

        // Now that daemon has approved us, we must send a hello as per specs
        let reply = connection.call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "Hello",
            &(),
        )?;

        connection.unique_name = Some(reply.body::<&str>().map(String::from)?);

        Ok(connection)
    }

    pub fn new_session() -> Result<Self> {
        Self::new(session_socket()?)
    }

    pub fn new_system() -> Result<Self> {
        Self::new(system_socket()?)
    }

    /// Create a `Connection` for the given [D-Bus address].
    ///
    /// [D-Bus address]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses
    pub fn new_for_address(address: &str) -> Result<Self> {
        Self::new(connect(&address::parse_dbus_address(address)?)?)
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
            let mut fds = read_exact(&self.socket, &mut buf[..])?;

            let mut incoming = Message::from_bytes(&buf)?;
            let bytes_left = incoming.bytes_to_completion()?;
            if bytes_left == 0 {
                return Err(Error::Handshake);
            }
            let mut buf = vec![0; bytes_left as usize];
            fds.append(&mut read_exact(&self.socket, &mut buf[..])?);
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

        write_all(&self.socket, msg.as_bytes(), &msg.fds())?;
        Ok(serial)
    }

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

    pub fn reset_default_message_handler(&mut self) {
        self.default_msg_handler = None;
    }

    fn next_serial(&self) -> u32 {
        self.serial.fetch_add(1, Ordering::SeqCst)
    }
}
