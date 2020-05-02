use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::{env, error, fmt, io};

use nix::unistd::Uid;

use zvariant::Error as VariantError;

use crate::address::{self, Address, AddressError};
use crate::message_field;
use crate::utils::{read_exact, write_all};
use crate::{Message, MessageError, MessageType, MIN_MESSAGE_SIZE};

pub struct Connection {
    pub server_guid: String,
    pub cap_unix_fd: bool,
    pub unique_name: Option<String>,

    socket: UnixStream,
    // Serial number for next outgoing message
    serial: u32,
}

#[derive(Debug)]
pub enum ConnectionError {
    Address(AddressError),
    IO(io::Error),
    Message(MessageError),
    MessageField(message_field::MessageFieldError),
    Variant(VariantError),
    Handshake,
    InvalidReply,
    // According to the spec, there can be all kinds of details in D-Bus errors but nobody adds anything more than a
    // string description.
    MethodError(String, Option<String>, Message),
    Unsupported,
}

impl error::Error for ConnectionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ConnectionError::Address(e) => Some(e),
            ConnectionError::IO(e) => Some(e),
            ConnectionError::Handshake => None,
            ConnectionError::Message(e) => Some(e),
            ConnectionError::MessageField(e) => Some(e),
            ConnectionError::Variant(e) => Some(e),
            ConnectionError::InvalidReply => None,
            ConnectionError::MethodError(_, _, _) => None,
            ConnectionError::Unsupported => None,
        }
    }
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionError::Address(e) => write!(f, "address error: {}", e),
            ConnectionError::IO(e) => write!(f, "I/O error: {}", e),
            ConnectionError::Handshake => write!(f, "D-Bus handshake failed"),
            ConnectionError::Message(e) => write!(f, "Message creation error: {}", e),
            ConnectionError::MessageField(e) => write!(f, "Message field parsing error: {}", e),
            ConnectionError::Variant(e) => write!(f, "{}", e),
            ConnectionError::InvalidReply => write!(f, "Invalid D-Bus method reply"),
            ConnectionError::MethodError(name, detail, _reply) => write!(
                f,
                "{}: {}",
                name,
                detail.as_ref().map(|s| s.as_str()).unwrap_or("no details")
            ),
            ConnectionError::Unsupported => write!(f, "Connection support is lacking"),
        }
    }
}

impl From<AddressError> for ConnectionError {
    fn from(val: AddressError) -> Self {
        ConnectionError::Address(val)
    }
}

impl From<io::Error> for ConnectionError {
    fn from(val: io::Error) -> Self {
        ConnectionError::IO(val)
    }
}

impl From<MessageError> for ConnectionError {
    fn from(val: MessageError) -> Self {
        ConnectionError::Message(val)
    }
}

impl From<message_field::MessageFieldError> for ConnectionError {
    fn from(val: message_field::MessageFieldError) -> Self {
        ConnectionError::MessageField(val)
    }
}

impl From<VariantError> for ConnectionError {
    fn from(val: VariantError) -> Self {
        ConnectionError::Variant(val)
    }
}

// For messages that are D-Bus error returns
impl From<Message> for ConnectionError {
    fn from(message: Message) -> ConnectionError {
        // FIXME: Instead of checking this, we should have Method as trait and specific types for
        // each message type.
        let header = match message.header() {
            Ok(header) => header,
            Err(e) => {
                return ConnectionError::Message(e);
            }
        };
        if header.primary().msg_type() != MessageType::Error {
            return ConnectionError::InvalidReply;
        }

        if let Ok(Some(name)) = header.error_name() {
            match message.body::<&str>() {
                Ok(detail) => {
                    ConnectionError::MethodError(name, Some(String::from(detail)), message)
                }
                Err(_) => ConnectionError::MethodError(name, None, message),
            }
        } else {
            ConnectionError::InvalidReply
        }
    }
}

fn connect(addr: &Address) -> Result<UnixStream, ConnectionError> {
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
fn session_socket() -> Result<UnixStream, ConnectionError> {
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
fn system_socket() -> Result<UnixStream, ConnectionError> {
    match env::var("DBUS_SYSTEM_BUS_ADDRESS") {
        Ok(val) => connect(&address::parse_dbus_address(&val)?),
        _ => Ok(UnixStream::connect("/var/run/dbus/system_bus_socket")?),
    }
}

fn read_reply(socket: &UnixStream) -> Result<Vec<String>, ConnectionError> {
    let mut buf_reader = BufReader::new(socket);
    let mut buf = String::new();

    buf_reader.read_line(&mut buf)?;
    let components = buf.split_whitespace();

    Ok(components.map(String::from).collect())
}

impl Connection {
    fn new(mut socket: UnixStream) -> Result<Self, ConnectionError> {
        let uid = Uid::current();

        // SASL Handshake
        let uid_str = uid
            .to_string()
            .chars()
            .map(|c| format!("{:x}", c as u32))
            .collect::<String>();
        socket.write_all(format!("\0AUTH EXTERNAL {}\r\n", uid_str).as_bytes())?;
        let server_guid = match read_reply(&socket)?.as_slice() {
            [ok, guid] if ok == "OK" => guid.clone(),
            _ => return Err(ConnectionError::Handshake),
        };

        socket.write_all(b"NEGOTIATE_UNIX_FD\r\n")?;
        let cap_unix_fd = match read_reply(&socket)?.as_slice() {
            [agree] if agree == "AGREE_UNIX_FD" => true,
            [error] if error == "ERROR" => false,
            _ => return Err(ConnectionError::Handshake),
        };

        socket.write_all(b"BEGIN\r\n")?;

        let mut connection = Self {
            socket,
            server_guid,
            cap_unix_fd,
            serial: 0,
            unique_name: None,
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

    pub fn new_session() -> Result<Self, ConnectionError> {
        Self::new(session_socket()?)
    }

    pub fn new_system() -> Result<Self, ConnectionError> {
        Self::new(system_socket()?)
    }

    pub fn call_method<B>(
        &mut self,
        destination: Option<&str>,
        path: &str,
        iface: Option<&str>,
        method_name: &str,
        body: &B,
    ) -> Result<Message, ConnectionError>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let serial = self.next_serial();
        let mut m = Message::method(
            self.unique_name.as_deref(),
            destination,
            path,
            iface,
            method_name,
            body,
        )?;
        if !m.fds().is_empty() && !self.cap_unix_fd {
            return Err(ConnectionError::Unsupported);
        }

        m.modify_primary_header(|primary| {
            primary.set_serial_num(serial);

            Ok(())
        })?;

        write_all(&self.socket, m.as_bytes(), m.fds())?;

        loop {
            // FIXME: We need to read incoming messages in a separate thread and maintain a queue

            let mut buf = [0; MIN_MESSAGE_SIZE];
            let mut fds = read_exact(&self.socket, &mut buf[..])?;

            let mut incoming = Message::from_bytes(&buf)?;
            let bytes_left = incoming.bytes_to_completion()?;
            if bytes_left == 0 {
                return Err(ConnectionError::Handshake);
            }
            let mut buf = vec![0; bytes_left as usize];
            fds.append(&mut read_exact(&self.socket, &mut buf[..])?);
            incoming.add_bytes(&buf[..])?;

            match process_response(&incoming, serial) {
                Some(MessageType::Error) => return Err((incoming).into()),
                Some(MessageType::MethodReturn) => {
                    incoming.set_fds(fds);
                    return Ok(incoming);
                }
                _ => (),
            }
        }
    }

    fn next_serial(&mut self) -> u32 {
        self.serial += 1;

        self.serial
    }
}

fn process_response(msg: &Message, serial: u32) -> Option<MessageType> {
    let header = match msg.header() {
        Ok(header) => header,
        Err(e) => {
            println!("Error parsing a message header: {}", e);

            return None;
        }
    };

    if header.reply_serial().ok().flatten() != Some(serial) {
        return None;
    }

    match header.message_type().ok() {
        Some(t @ MessageType::MethodReturn) | Some(t @ MessageType::Error) => Some(t),
        _ => None,
    }
}
