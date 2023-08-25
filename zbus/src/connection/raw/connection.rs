use std::{
    collections::VecDeque,
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard},
    task::{Context, Poll},
};

use event_listener::{Event, EventListener};

#[cfg(unix)]
use crate::OwnedFd;
use crate::{
    message::{
        header::{MAX_MESSAGE_SIZE, MIN_MESSAGE_SIZE},
        Message, PrimaryHeader,
    },
    utils::padding_for_8_bytes,
};

use super::socket::{ReadHalf, Split, WriteHalf};

use futures_core::ready;

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
    outbound: Mutex<OutBound>,
}

#[derive(Debug)]
pub struct InBound {
    buffer: Vec<u8>,
    #[cfg(unix)]
    fds: Vec<OwnedFd>,
    pos: usize,
    prev_seq: u64,
}

#[derive(Debug)]
pub struct OutBound {
    pos: usize,
    msgs: VecDeque<Arc<Message>>,
}

impl<R: ReadHalf, W: WriteHalf> Connection<R, W> {
    pub(crate) fn new(socket: Split<R, W>, raw_in_buffer: Vec<u8>) -> Connection<R, W> {
        let (read, write) = socket.take();
        Connection {
            activity_event: Event::new(),
            read_socket: Mutex::new(read),
            inbound: Mutex::new(InBound {
                pos: raw_in_buffer.len(),
                buffer: raw_in_buffer,
                #[cfg(unix)]
                fds: vec![],
                prev_seq: 0,
            }),
            write_socket: Mutex::new(write),
            outbound: Mutex::new(OutBound {
                pos: 0,
                msgs: VecDeque::new(),
            }),
        }
    }

    /// Attempt to flush the outgoing buffer
    ///
    /// This will try to write as many messages as possible from the
    /// outgoing buffer into the socket, until an error is encountered.
    ///
    /// This method will thus only block if the socket is in blocking mode.
    pub fn try_flush(&self, cx: &mut Context<'_>) -> Poll<crate::Result<()>> {
        self.activity_event.notify(usize::MAX);
        let mut outbound = self.outbound.lock().expect("lock poisoned");
        let mut write = self.write_socket.lock().expect("lock poisoned");
        while !outbound.msgs.is_empty() {
            loop {
                // `outbound` is locked and we just checked there is a message.
                let msg = outbound.msgs.front().expect("no message");
                let data = &msg.as_bytes()[outbound.pos..];
                if data.is_empty() {
                    outbound.pos = 0;
                    outbound.msgs.pop_front();
                    break;
                }
                #[cfg(unix)]
                let fds = if outbound.pos == 0 { msg.fds() } else { vec![] };
                outbound.pos += ready!(write.poll_sendmsg(
                    cx,
                    data,
                    #[cfg(unix)]
                    &fds,
                ))?;
            }
        }
        Poll::Ready(Ok(()))
    }

    /// Enqueue a message to be sent out to the socket
    ///
    /// This method will *not* write anything to the socket, you need to call
    /// `try_flush()` afterwards so that your message is actually sent out.
    pub fn enqueue_message(&self, msg: Arc<Message>) {
        self.outbound
            .lock()
            .expect("lock poisoned")
            .msgs
            .push_back(msg);
    }

    /// Attempt to read a message from the socket
    ///
    /// This methods will read from the socket until either a full D-Bus message is
    /// read or an error is encountered.
    ///
    /// If the socket is in non-blocking mode, it may read a partial message. In such case it
    /// will buffer it internally and try to complete it the next time you call
    /// `try_receive_message`.
    pub fn try_receive_message(&self, cx: &mut Context<'_>) -> Poll<crate::Result<Message>> {
        self.activity_event.notify(usize::MAX);
        let mut inbound = self.inbound.lock().expect("lock poisoned");
        let mut read = self.read_socket.lock().expect("lock poisoned");
        if inbound.pos < MIN_MESSAGE_SIZE {
            inbound.buffer.resize(MIN_MESSAGE_SIZE, 0);
            // We don't have enough data to make a proper message header yet.
            // Some partial read may be in raw_in_buffer, so we try to complete it
            // until we have MIN_MESSAGE_SIZE bytes
            //
            // Given that MIN_MESSAGE_SIZE is 16, this codepath is actually extremely unlikely
            // to be taken more than once
            while inbound.pos < MIN_MESSAGE_SIZE {
                let pos = inbound.pos;
                let res = ready!(read.poll_recvmsg(cx, &mut inbound.buffer[pos..]))?;
                let len = {
                    #[cfg(unix)]
                    {
                        let (len, fds) = res;
                        inbound.fds.extend(fds);
                        len
                    }
                    #[cfg(not(unix))]
                    {
                        res
                    }
                };
                inbound.pos += len;
                if len == 0 {
                    return Poll::Ready(Err(crate::Error::InputOutput(
                        std::io::Error::new(
                            std::io::ErrorKind::UnexpectedEof,
                            "failed to receive message",
                        )
                        .into(),
                    )));
                }
            }
        }

        let (primary_header, fields_len) = PrimaryHeader::read(&inbound.buffer)?;
        let header_len = MIN_MESSAGE_SIZE + fields_len as usize;
        let body_padding = padding_for_8_bytes(header_len);
        let body_len = primary_header.body_len() as usize;
        let total_len = header_len + body_padding + body_len;
        if total_len > MAX_MESSAGE_SIZE {
            return Poll::Ready(Err(crate::Error::ExcessData));
        }

        // By this point we have a full primary header, so we know the exact length of the complete
        // message.
        inbound.buffer.resize(total_len, 0);

        // Now we have an incomplete message; read the rest
        while inbound.buffer.len() > inbound.pos {
            let pos = inbound.pos;
            let res = ready!(read.poll_recvmsg(cx, &mut inbound.buffer[pos..]))?;
            let read = {
                #[cfg(unix)]
                {
                    let (read, fds) = res;
                    inbound.fds.extend(fds);
                    read
                }
                #[cfg(not(unix))]
                {
                    res
                }
            };
            inbound.pos += read;
        }

        // If we reach here, the message is complete; return it
        inbound.pos = 0;
        let bytes = std::mem::take(&mut inbound.buffer);
        #[cfg(unix)]
        let fds = std::mem::take(&mut inbound.fds);
        let seq = inbound.prev_seq + 1;
        inbound.prev_seq = seq;
        Poll::Ready(Message::from_raw_parts(
            bytes,
            #[cfg(unix)]
            fds,
            seq,
        ))
    }

    /// Close the connection.
    ///
    /// After this call, all reading and writing operations will fail.
    pub fn close(&self) -> crate::Result<()> {
        self.activity_event.notify(usize::MAX);
        self.write_socket
            .lock()
            .expect("lock poisoned")
            .close()
            .map_err(|e| e.into())
    }

    /// Access the underlying read half of the socket.
    ///
    /// This method is intended to provide access to the socket in order to access certain
    /// properties (e.g peer credentials).
    ///
    /// You should not try to read from it directly, as it may corrupt the internal state of this
    /// wrapper.
    pub fn socket_read(&self) -> impl Deref<Target = R> + '_ {
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
            socket: self.read_socket.lock().expect("lock poisoned"),
        }
    }

    pub(crate) fn monitor_activity(&self) -> EventListener {
        self.activity_event.listen()
    }
}

#[cfg(unix)]
#[cfg(test)]
mod tests {
    use super::{super::socket::Socket, Arc, Connection};
    use crate::message::Message;
    use futures_util::future::poll_fn;
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

        conn0.enqueue_message(Arc::new(msg));
        poll_fn(|cx| conn0.try_flush(cx)).await.unwrap();

        let ret = poll_fn(|cx| conn1.try_receive_message(cx)).await.unwrap();
        assert_eq!(ret.to_string(), "Method call Test");
    }
}
