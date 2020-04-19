use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::{env, error, fmt, io};

use nix::unistd::Uid;

use zvariant::{Error as VariantError, FromValue};

use crate::address::{self, Address, AddressError};
use crate::message_field::{self, MessageFieldCode};
use crate::{Message, MessageError, MessageType, MIN_MESSAGE_SIZE};

pub struct Connection {
    pub server_guid: String,
    pub unique_name: String,

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
    MethodError(String, Option<String>),
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
            ConnectionError::MethodError(_, _) => None,
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
            ConnectionError::MethodError(name, detail) => write!(
                f,
                "{}: {}",
                name,
                detail.as_ref().map(|s| s.as_str()).unwrap_or("no details")
            ),
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

        // First, get the error name
        let name = match header.fields().get_field(MessageFieldCode::ErrorName) {
            Some(f) => match <&str>::from_value_ref(f.value()) {
                Ok(s) => String::from(*s),
                Err(e) => return ConnectionError::Variant(e),
            },
            None => return ConnectionError::InvalidReply,
        };

        // Then, try to get the optional description string
        match message.body::<&str>() {
            Ok(detail) => ConnectionError::MethodError(name, Some(String::from(detail))),
            Err(e) => ConnectionError::Message(e),
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
        let mut buf_reader = BufReader::new(&socket);
        let mut buf = String::new();
        let bytes_read = buf_reader.read_line(&mut buf)?;
        let mut components = buf.split_whitespace();
        if bytes_read < 3 || components.next() != Some("OK") {
            return Err(ConnectionError::Handshake);
        }

        let server_guid = String::from(components.next().ok_or(ConnectionError::Handshake)?);

        socket.write_all(b"BEGIN\r\n")?;

        let mut connection = Self {
            socket,
            server_guid,
            serial: 0,
            unique_name: String::from(""),
        };

        // Now that daemon has approved us, we must send a hello as per specs
        let reply = connection.call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "Hello",
            &(),
        )?;

        connection.unique_name = reply.body::<&str>().map(String::from)?;
        println!("bus name: {}", connection.unique_name);

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
        println!("Starting: {}", method_name);
        let serial = self.next_serial();
        let mut m = Message::method(destination, path, iface, method_name, body)?;
        m.modify_primary_header(|primary| {
            primary.set_serial_num(serial);

            Ok(())
        })?;

        self.socket.write_all(m.as_bytes())?;

        loop {
            // FIXME: We need to read incoming messages in a separate thread and maintain a queue

            let mut buf = [0; MIN_MESSAGE_SIZE];
            self.socket.read_exact(&mut buf[..])?;

            let mut incoming = Message::from_bytes(&buf)?;
            let bytes_left = incoming.bytes_to_completion()?;
            if bytes_left == 0 {
                return Err(ConnectionError::Handshake);
            }
            let mut buf = vec![0; bytes_left as usize];
            self.socket.read_exact(&mut buf[..])?;
            incoming.add_bytes(&buf[..])?;

            let header = incoming.header()?;
            let msg_type = header.primary().msg_type();
            if msg_type == MessageType::MethodReturn || msg_type == MessageType::Error {
                let serial_field = header.fields().get_field(MessageFieldCode::ReplySerial);

                if let Some(serial_field) = serial_field {
                    if *serial_field.value() != zvariant::Value::U32(serial) {
                        continue;
                    }
                } else {
                    // FIXME: Debug log about ignoring the message
                    continue;
                }

                match msg_type {
                    MessageType::Error => return Err(incoming.into()),
                    MessageType::MethodReturn => return Ok(incoming),
                    _ => (),
                }
            }
        }
    }

    fn next_serial(&mut self) -> u32 {
        self.serial += 1;

        self.serial
    }
}
