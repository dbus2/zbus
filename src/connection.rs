use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::{error, fmt, io};

use nix::unistd::Uid;

use crate::message;
use crate::variant;

pub struct Connection {
    socket: UnixStream,
    server_guid: String,
    // Serial number for next outgoing message
    serial: u32,
}

#[derive(Debug)]
pub enum ConnectionError {
    IO(io::Error),
    Message(message::MessageError),
    Variant(variant::VariantError),
    Handshake,
    InvalidReply,
}

impl error::Error for ConnectionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ConnectionError::IO(e) => Some(e),
            ConnectionError::Handshake => None,
            ConnectionError::Message(e) => Some(e),
            ConnectionError::Variant(e) => Some(e),
            ConnectionError::InvalidReply => None,
        }
    }
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionError::IO(e) => write!(f, "I/O error: {}", e),
            ConnectionError::Handshake => write!(f, "D-Bus handshake failed"),
            ConnectionError::Message(e) => write!(f, "Message creation error: {}", e),
            ConnectionError::Variant(e) => write!(f, "{}", e),
            ConnectionError::InvalidReply => write!(f, "Invalid D-Bus method reply"),
        }
    }
}

impl Connection {
    pub fn new_session() -> Result<Self, ConnectionError> {
        // FIXME: Currently just assume a path
        let uid = Uid::current();
        let path = format!("/run/user/{}/bus", uid);
        let mut socket = UnixStream::connect(path).map_err(|e| ConnectionError::IO(e))?;

        // SASL Handshake
        let uid_str = uid
            .to_string()
            .chars()
            .map(|c| format!("{:x}", c as u32))
            .collect::<String>();
        socket
            .write(format!("\0AUTH EXTERNAL {}\r\n", uid_str).as_bytes())
            .map_err(|e| ConnectionError::IO(e))?;
        let mut buf_reader = BufReader::new(&socket);
        let mut buf = String::new();
        let bytes_read = buf_reader
            .read_line(&mut buf)
            .map_err(|e| ConnectionError::IO(e))?;
        let mut components = buf.split_whitespace();
        if bytes_read < 3 || components.next() != Some("OK") {
            return Err(ConnectionError::Handshake);
        }

        let server_guid = String::from(components.next().ok_or(ConnectionError::Handshake)?);

        socket
            .write(b"BEGIN\r\n")
            .map_err(|e| ConnectionError::IO(e))?;

        let mut connection = Self {
            socket,
            server_guid,
            serial: 0,
        };

        // Now that daemon has approved us, we must send a hello as per specs
        let serial = connection.next_serial();
        let m = message::Message::method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "Hello",
            "",
            &[],
        )
        .map_err(|e| ConnectionError::Message(e))?
        .set_serial(serial);

        // FIXME: Separate out message sending & reply receiving/parsing into separate method

        connection
            .socket
            .write(m.as_bytes())
            .map_err(|e| ConnectionError::IO(e))?;

        loop {
            // FIXME: We need to read incoming messages in a separate thread and maintain a queue

            let mut buf = [0; message::PRIMARY_HEADER_SIZE];
            let bytes_read = connection
                .socket
                .read(&mut buf[..])
                .map_err(|e| ConnectionError::IO(e))?;

            let mut incoming =
                message::Message::from_bytes(&buf).map_err(|e| ConnectionError::Message(e))?;
            let bytes_left = incoming.bytes_to_completion();
            if bytes_left == 0 {
                return Err(ConnectionError::Handshake);
            }
            let mut buf = vec![0; bytes_left as usize];
            let bytes_read = connection
                .socket
                .read(&mut buf[..])
                .map_err(|e| ConnectionError::IO(e))?;
            incoming
                .add_bytes(&buf[0..bytes_read])
                .map_err(|e| ConnectionError::Message(e))?;

            if incoming.message_type() == message::MessageType::MethodReturn {
                let all_fields = incoming
                    .get_fields()
                    .map_err(|e| ConnectionError::Message(e))?;

                if let Some(_) = all_fields.iter().find(|element| {
                    let (f, v) = element;

                    *f == message::MessageField::ReplySerial
                        && v.get_u32().unwrap_or(std::u32::MAX) == serial
                }) {
                    // TODO: Get string from reply body
                    if let Some((field, value)) = all_fields.iter().find(|element| {
                        let (f, v) = element;

                        *f == message::MessageField::Signature
                            && v.get_string().unwrap_or(String::from("")) == "s"
                    }) {
                        let bus_name = variant::Variant::from_data(
                            &incoming
                                .get_body()
                                .map_err(|e| ConnectionError::Message(e))?,
                            "s",
                        )
                        .map_err(|e| ConnectionError::Variant(e))?
                        .get_string()
                        .map_err(|e| ConnectionError::Variant(e))?;

                        println!("bus name: {}", bus_name);
                    } else {
                        return Err(ConnectionError::InvalidReply);
                    }

                    break;
                }
            }
        }

        Ok(connection)
    }

    fn next_serial(&mut self) -> u32 {
        self.serial += 1;

        self.serial
    }
}
