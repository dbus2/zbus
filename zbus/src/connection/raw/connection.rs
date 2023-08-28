use crate::async_lock::{Mutex, MutexGuard};
use std::{future::poll_fn, ops::Deref};

use event_listener::{Event, EventListener};

use crate::{
    message::{
        header::{MAX_MESSAGE_SIZE, MIN_MESSAGE_SIZE},
        Message, PrimaryHeader,
    },
    utils::padding_for_8_bytes,
};

use super::socket::{ReadHalf, Split, WriteHalf};

/// A low-level representation of a D-Bus connection
///
/// This wrapper is agnostic on the actual transport, using the `socket::{ReadHalf, WriteHalf}`
/// traits to abstract it. It is compatible with sockets both in blocking or non-blocking
/// mode.
///
/// This wrapper abstracts away the serialization & buffering considerations of the
/// protocol, and allows interaction based on messages, rather than bytes.
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Connection<R, W> {
    activity_event: Event,

    read_socket: Mutex<R>,
    inbound: Mutex<InBound>,

    write_socket: Mutex<W>,
}

#[derive(Debug)]
pub struct InBound {
    already_received_bytes: Option<Vec<u8>>,
    prev_seq: u64,
}

impl<R: ReadHalf, W: WriteHalf> Connection<R, W> {
    pub(crate) fn new(socket: Split<R, W>, raw_in_buffer: Vec<u8>) -> Connection<R, W> {
        let (read, write) = socket.take();
        Connection {
            activity_event: Event::new(),
            read_socket: Mutex::new(read),
            inbound: Mutex::new(InBound {
                already_received_bytes: Some(raw_in_buffer),
                prev_seq: 0,
            }),
            write_socket: Mutex::new(write),
        }
    }

    /// Attempt to send a message.
    ///
    /// This method will thus only block if the socket is in blocking mode.
    pub async fn send_message(&self, msg: &Message) -> crate::Result<()> {
        self.activity_event.notify(usize::MAX);
        let mut write = self.write_socket.lock().await;
        let mut pos = 0;
        let data = msg.as_bytes();
        while pos < data.len() {
            #[cfg(unix)]
            let fds = if pos == 0 { msg.fds() } else { vec![] };
            pos += poll_fn(|cx| {
                write.poll_sendmsg(
                    cx,
                    &data[pos..],
                    #[cfg(unix)]
                    &fds,
                )
            })
            .await?;
        }
        Ok(())
    }

    /// Attempt to read a message from the socket
    ///
    /// This methods will read from the socket until either a full D-Bus message is
    /// read or an error is encountered.
    ///
    /// If the socket is in non-blocking mode, it may read a partial message. In such case it
    /// will buffer it internally and try to complete it the next time you call
    /// `try_receive_message`.
    pub async fn receive_message(&self) -> crate::Result<Message> {
        self.activity_event.notify(usize::MAX);
        let mut inbound = self.inbound.lock().await;
        let mut bytes = inbound
            .already_received_bytes
            .take()
            .unwrap_or_else(|| Vec::with_capacity(MIN_MESSAGE_SIZE));
        let mut pos = bytes.len();
        #[cfg(unix)]
        let mut fds = vec![];
        let mut read = self.read_socket.lock().await;
        if pos < MIN_MESSAGE_SIZE {
            bytes.resize(MIN_MESSAGE_SIZE, 0);
            // We don't have enough data to make a proper message header yet.
            // Some partial read may be in raw_in_buffer, so we try to complete it
            // until we have MIN_MESSAGE_SIZE bytes
            //
            // Given that MIN_MESSAGE_SIZE is 16, this codepath is actually extremely unlikely
            // to be taken more than once
            while pos < MIN_MESSAGE_SIZE {
                let res = poll_fn(|cx| read.poll_recvmsg(cx, &mut bytes[pos..])).await?;
                let len = {
                    #[cfg(unix)]
                    {
                        fds.extend(res.1);
                        res.0
                    }
                    #[cfg(not(unix))]
                    {
                        res
                    }
                };
                pos += len;
                if len == 0 {
                    return Err(crate::Error::InputOutput(
                        std::io::Error::new(
                            std::io::ErrorKind::UnexpectedEof,
                            "failed to receive message",
                        )
                        .into(),
                    ));
                }
            }
        }

        let (primary_header, fields_len) = PrimaryHeader::read(&bytes)?;
        let header_len = MIN_MESSAGE_SIZE + fields_len as usize;
        let body_padding = padding_for_8_bytes(header_len);
        let body_len = primary_header.body_len() as usize;
        let total_len = header_len + body_padding + body_len;
        if total_len > MAX_MESSAGE_SIZE {
            return Err(crate::Error::ExcessData);
        }

        // By this point we have a full primary header, so we know the exact length of the complete
        // message.
        bytes.resize(total_len, 0);

        // Now we have an incomplete message; read the rest
        while pos < total_len {
            let res = poll_fn(|cx| read.poll_recvmsg(cx, &mut bytes[pos..])).await?;
            let read = {
                #[cfg(unix)]
                {
                    fds.extend(res.1);
                    res.0
                }
                #[cfg(not(unix))]
                {
                    res
                }
            };
            pos += read;
            if read == 0 {
                return Err(crate::Error::InputOutput(
                    std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "failed to receive message",
                    )
                    .into(),
                ));
            }
        }

        // If we reach here, the message is complete; return it
        let seq = inbound.prev_seq + 1;
        inbound.prev_seq = seq;
        Message::from_raw_parts(
            bytes,
            #[cfg(unix)]
            fds,
            seq,
        )
    }

    /// Close the connection.
    ///
    /// After this call, all reading and writing operations will fail.
    pub async fn close(&self) -> crate::Result<()> {
        self.activity_event.notify(usize::MAX);
        let mut write = self.write_socket.lock().await;
        poll_fn(|cx| write.close(cx).map_err(|e| e.into())).await
    }

    /// Access the underlying read half of the socket.
    ///
    /// This method is intended to provide access to the socket in order to access certain
    /// properties (e.g peer credentials).
    ///
    /// You should not try to read from it directly, as it may corrupt the internal state of this
    /// wrapper.
    pub async fn socket_read(&self) -> impl Deref<Target = R> + '_ {
        pub struct SocketDeref<'s, R: ReadHalf> {
            socket: MutexGuard<'s, R>,
        }

        impl<R> Deref for SocketDeref<'_, R>
        where
            R: ReadHalf,
        {
            type Target = R;

            fn deref(&self) -> &Self::Target {
                &self.socket
            }
        }

        SocketDeref {
            socket: self.read_socket.lock().await,
        }
    }

    pub(crate) fn monitor_activity(&self) -> EventListener {
        self.activity_event.listen()
    }
}

#[cfg(unix)]
#[cfg(test)]
mod tests {
    use super::{super::socket::Socket, Connection};
    use crate::message::Message;
    use test_log::test;

    #[test]
    fn raw_send_receive() {
        crate::block_on(raw_send_receive_async());
    }

    async fn raw_send_receive_async() {
        #[cfg(not(feature = "tokio"))]
        let (p0, p1) = std::os::unix::net::UnixStream::pair()
            .map(|(p0, p1)| {
                (
                    async_io::Async::new(p0).unwrap(),
                    async_io::Async::new(p1).unwrap(),
                )
            })
            .unwrap();
        #[cfg(feature = "tokio")]
        let (p0, p1) = tokio::net::UnixStream::pair().unwrap();

        let conn0 = Connection::new(p0.split(), vec![]);
        let conn1 = Connection::new(p1.split(), vec![]);

        let msg = Message::method(
            None::<()>,
            None::<()>,
            "/",
            Some("org.zbus.p2p"),
            "Test",
            &(),
        )
        .unwrap();

        conn0.send_message(&msg).await.unwrap();

        let ret = conn1.receive_message().await.unwrap();
        assert_eq!(ret.to_string(), "Method call Test");
    }
}
