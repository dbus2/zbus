use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::{env, error, fmt, io};

use nix::unistd::Uid;

use crate::address::{self, Address};
use crate::message;
use crate::message_field;
use crate::Decode;
use crate::{VariantError, VariantTypeConstants};

pub struct Connection {
    pub server_guid: String,

    socket: UnixStream,
    // Serial number for next outgoing message
    serial: u32,
}

#[derive(Debug)]
pub enum ConnectionError {
    Address(address::AddressError),
    IO(io::Error),
    Message(message::MessageError),
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

impl From<address::AddressError> for ConnectionError {
    fn from(val: address::AddressError) -> Self {
        ConnectionError::Address(val)
    }
}

impl From<io::Error> for ConnectionError {
    fn from(val: io::Error) -> Self {
        ConnectionError::IO(val)
    }
}

impl From<message::MessageError> for ConnectionError {
    fn from(val: message::MessageError) -> Self {
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
impl From<message::Message> for ConnectionError {
    fn from(message: message::Message) -> ConnectionError {
        // FIXME: Instead of checking this, we should have Method as trait and specific types for
        // each message type.
        if message.message_type() != message::MessageType::Error {
            return ConnectionError::InvalidReply;
        }

        match message.fields() {
            Ok(all_fields) => {
                // First, get the error name
                let name = match all_fields.iter().find(|f| {
                    f.code()
                        .map(|c| c == message_field::MessageFieldCode::ErrorName)
                        .unwrap_or(false)
                }) {
                    Some(f) => match f.value() {
                        Ok(v) => match String::from_variant(v) {
                            Ok(s) => String::from(s),
                            Err(e) => return ConnectionError::Variant(e),
                        },
                        Err(e) => return ConnectionError::MessageField(e),
                    },
                    None => return ConnectionError::InvalidReply,
                };

                // Then, try to get the optional description string
                if message.body_len() > 0 {
                    match message.body(Some(<String>::SIGNATURE_STR.into())) {
                        Ok(body) => match String::from_variant(&body.fields()[0]) {
                            Ok(detail) => {
                                ConnectionError::MethodError(name, Some(String::from(detail)))
                            }
                            Err(e) => ConnectionError::Variant(e),
                        },
                        Err(e) => ConnectionError::Message(e),
                    }
                } else {
                    return ConnectionError::MethodError(name, None);
                }
            }
            Err(e) => ConnectionError::Message(e),
        }
    }
}

/// Get a session socket respecting the DBUS_SESSION_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// /run/user/UID/bus
fn session_socket() -> Result<UnixStream, ConnectionError> {
    match env::var("DBUS_SESSION_BUS_ADDRESS") {
        Ok(val) => match address::parse_dbus_address(&val)? {
            Address::Path(p) => Ok(UnixStream::connect(p)?),
            Address::Abstract(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "abstract sockets are not currently supported",
            )
            .into()),
        },
        _ => {
            let uid = Uid::current();
            let path = format!("/run/user/{}/bus", uid);
            Ok(UnixStream::connect(path)?)
        }
    }
}

impl Connection {
    pub fn new_session() -> Result<Self, ConnectionError> {
        let mut socket = session_socket()?;
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
        };

        // Now that daemon has approved us, we must send a hello as per specs
        let reply = connection.call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "Hello",
            None,
        )?;

        let body = reply.body(Some(<String>::SIGNATURE_STR.into()))?;
        let bus_name = String::from_variant(&body.fields()[0])?;

        println!("bus name: {}", bus_name);

        Ok(connection)
    }

    pub fn call_method(
        &mut self,
        destination: Option<&str>,
        path: &str,
        iface: Option<&str>,
        method_name: &str,
        body: Option<crate::Structure>,
    ) -> Result<message::Message, ConnectionError> {
        println!("Starting: {}", method_name);
        let serial = self.next_serial();
        let m = message::Message::method(destination, path, iface, method_name, body)?
            .set_serial(serial);

        self.socket.write_all(m.as_bytes())?;

        loop {
            // FIXME: We need to read incoming messages in a separate thread and maintain a queue

            let mut buf = [0; message::PRIMARY_HEADER_SIZE];
            self.socket.read_exact(&mut buf[..])?;

            let mut incoming = message::Message::from_bytes(&buf)?;
            let bytes_left = incoming.bytes_to_completion();
            if bytes_left == 0 {
                return Err(ConnectionError::Handshake);
            }
            let mut buf = vec![0; bytes_left as usize];
            self.socket.read_exact(&mut buf[..])?;
            incoming.add_bytes(&buf[..])?;

            if incoming.message_type() == message::MessageType::MethodReturn
                || incoming.message_type() == message::MessageType::Error
            {
                let all_fields = incoming.fields()?;

                if all_fields
                    .iter()
                    .find(|f| {
                        f.code()
                            .map(|c| c == message_field::MessageFieldCode::ReplySerial)
                            .unwrap_or(false)
                            && f.value()
                                .map(|v| {
                                    u32::from_variant(v).map(|u| *u == serial).unwrap_or(false)
                                })
                                .unwrap()
                    })
                    .is_some()
                {
                    match incoming.message_type() {
                        message::MessageType::Error => return Err(incoming.into()),
                        message::MessageType::MethodReturn => return Ok(incoming),
                        _ => (),
                    }
                }
            }
        }
    }

    fn next_serial(&mut self) -> u32 {
        self.serial += 1;

        self.serial
    }
}
