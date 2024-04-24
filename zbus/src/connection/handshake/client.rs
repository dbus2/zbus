use async_trait::async_trait;
use std::collections::VecDeque;
use tracing::{debug, instrument, trace};

use sha1::{Digest, Sha1};

use crate::Message;

use super::{
    random_ascii, sasl_auth_id, AuthMechanism, Authenticated, BoxedSplit, Command, Common, Cookie,
    Error, Handshake, OwnedGuid, Result, Str,
};

/// A representation of an in-progress handshake, client-side
///
/// This struct is an async-compatible representation of the initial handshake that must be
/// performed before a D-Bus connection can be used.
#[derive(Debug)]
pub struct Client {
    common: Common,
    server_guid: Option<OwnedGuid>,
    bus: bool,
}

impl Client {
    /// Start a handshake on this client socket
    pub fn new(
        socket: BoxedSplit,
        mechanisms: Option<VecDeque<AuthMechanism>>,
        server_guid: Option<OwnedGuid>,
        bus: bool,
    ) -> Client {
        let mechanisms = mechanisms.unwrap_or_else(|| {
            let mut mechanisms = VecDeque::new();
            mechanisms.push_back(AuthMechanism::External);
            mechanisms.push_back(AuthMechanism::Cookie);
            mechanisms.push_back(AuthMechanism::Anonymous);
            mechanisms
        });

        Client {
            common: Common::new(socket, mechanisms),
            server_guid,
            bus,
        }
    }

    /// Respond to a cookie authentication challenge from the server.
    ///
    /// Returns the next command to send to the server.
    async fn handle_cookie_challenge(&mut self, data: Vec<u8>) -> Result<Command> {
        let context = std::str::from_utf8(&data)
            .map_err(|_| Error::Handshake("Cookie context was not valid UTF-8".into()))?;
        let mut split = context.split_ascii_whitespace();
        let context = split
            .next()
            .ok_or_else(|| Error::Handshake("Missing cookie context name".into()))?;
        let context = Str::from(context).try_into()?;
        let id = split
            .next()
            .ok_or_else(|| Error::Handshake("Missing cookie ID".into()))?;
        let id = id
            .parse()
            .map_err(|e| Error::Handshake(format!("Invalid cookie ID `{id}`: {e}")))?;
        let server_challenge = split
            .next()
            .ok_or_else(|| Error::Handshake("Missing cookie challenge".into()))?;

        let cookie = Cookie::lookup(&context, id).await?;
        let cookie = cookie.cookie();
        let client_challenge = random_ascii(16);
        let sec = format!("{server_challenge}:{client_challenge}:{cookie}");
        let sha1 = hex::encode(Sha1::digest(sec));
        let data = format!("{client_challenge} {sha1}").into_bytes();

        Ok(Command::Data(Some(data)))
    }

    fn set_guid(&mut self, guid: OwnedGuid) -> Result<()> {
        match &self.server_guid {
            Some(server_guid) if *server_guid != guid => {
                return Err(Error::Handshake(format!(
                    "Server GUID mismatch: expected {server_guid}, got {guid}",
                )));
            }
            Some(_) => (),
            None => self.server_guid = Some(guid),
        }

        Ok(())
    }
}

#[async_trait]
impl Handshake for Client {
    #[instrument(skip(self))]
    async fn perform(mut self) -> Result<Authenticated> {
        trace!("Initializing");
        // The dbus daemon on some platforms requires sending the zero byte as a
        // separate message with SCM_CREDS.
        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
        let written = self
            .common
            .socket_mut()
            .write_mut()
            .send_zero_byte()
            .await
            .map_err(|e| {
                Error::Handshake(format!("Could not send zero byte with credentials: {}", e))
            })
            .and_then(|n| match n {
                None => Err(Error::Handshake(
                    "Could not send zero byte with credentials".to_string(),
                )),
                Some(n) => Ok(n),
            })?;

        // leading 0 is sent separately already for `freebsd` and `dragonfly` above.
        #[cfg(not(any(target_os = "freebsd", target_os = "dragonfly")))]
        let written = self
            .common
            .socket_mut()
            .write_mut()
            .sendmsg(
                &[b'\0'],
                #[cfg(unix)]
                &[],
            )
            .await?;

        if written != 1 {
            return Err(Error::Handshake(
                "Could not send zero byte with credentials".to_string(),
            ));
        }

        let mut commands = Vec::with_capacity(4);
        loop {
            let mechanism = self.common.next_mechanism()?;
            trace!("Trying {mechanism} mechanism");
            let auth_cmd = match mechanism {
                AuthMechanism::Anonymous => Command::Auth(Some(mechanism), Some("zbus".into())),
                AuthMechanism::External => {
                    Command::Auth(Some(mechanism), Some(sasl_auth_id()?.into_bytes()))
                }
                AuthMechanism::Cookie => Command::Auth(
                    Some(AuthMechanism::Cookie),
                    Some(sasl_auth_id()?.into_bytes()),
                ),
            };
            self.common.write_command(auth_cmd).await?;

            match self.common.read_command().await? {
                Command::Ok(guid) => {
                    trace!("Received OK from server");
                    self.set_guid(guid)?;

                    break;
                }
                Command::Data(data) if mechanism == AuthMechanism::Cookie => {
                    let data = data.ok_or_else(|| {
                        Error::Handshake("Received DATA with no data from server".into())
                    })?;
                    trace!("Received cookie challenge from server");
                    let response = self.handle_cookie_challenge(data).await?;
                    commands.push(response);

                    break;
                }
                Command::Rejected(_) => debug!("{mechanism} rejected by the server"),
                Command::Error(e) => debug!("Received error from server: {e}"),
                cmd => {
                    return Err(Error::Handshake(format!(
                        "Unexpected command from server: {cmd}"
                    )))
                }
            }
        }

        let can_pass_fd = self.common.socket_mut().read_mut().can_pass_unix_fd();
        if can_pass_fd {
            commands.push(Command::NegotiateUnixFD);
        };
        commands.push(Command::Begin);
        let hello_method = if self.bus {
            Some(
                Message::method("/org/freedesktop/DBus", "Hello")
                    .unwrap()
                    .destination("org.freedesktop.DBus")
                    .unwrap()
                    .interface("org.freedesktop.DBus")
                    .unwrap()
                    .build(&())
                    .unwrap(),
            )
        } else {
            None
        };

        // Server replies to all commands except `BEGIN`.
        let expected_n_responses = commands.len() - 1;
        self.common
            .write_commands(&commands, hello_method.as_ref().map(|m| &**m.data()))
            .await?;

        if expected_n_responses > 0 {
            for response in self.common.read_commands(expected_n_responses).await? {
                match response {
                    Command::Ok(guid) => {
                        trace!("Received OK from server");
                        self.set_guid(guid)?;
                    }
                    Command::AgreeUnixFD => self.common.set_cap_unix_fd(true),
                    // This also covers "REJECTED" and "ERROR", which would mean that the server has
                    // rejected the authentication challenge response (likely cookie) since it
                    // already agreed to the mechanism. Theoretically we should
                    // be just trying the next auth mechanism but this most
                    // likely means something is very wrong and we're already
                    // too deep into the handshake to recover.
                    cmd => {
                        return Err(Error::Handshake(format!(
                            "Unexpected command from server: {cmd}"
                        )))
                    }
                }
            }
        }

        trace!("Handshake done");
        #[allow(unused_variables)]
        let (socket, recv_buffer, cap_unix_fd, _) = self.common.into_components();
        let (mut read, write) = socket.take();

        // If we're a bus connection, we need to read the unique name from `Hello` response.
        let (unique_name, already_received_bytes) = if self.bus {
            use crate::message::Type;

            let reply = read.receive_message(0, Some(recv_buffer)).await?;
            let unique_name = match reply.message_type() {
                Type::MethodReturn => reply.body().deserialize()?,
                Type::Error => return Err(Error::from(reply)),
                m => return Err(Error::Handshake(format!("Unexpected messgage `{m:?}`"))),
            };
            (Some(unique_name), None)
        } else {
            (None, Some(recv_buffer))
        };

        Ok(Authenticated {
            socket_write: write,
            socket_read: Some(read),
            server_guid: self.server_guid.unwrap(),
            #[cfg(unix)]
            cap_unix_fd,
            already_received_bytes,
            unique_name,
        })
    }
}
