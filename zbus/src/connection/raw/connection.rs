use crate::async_lock::{Mutex, MutexGuard};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use event_listener::{Event, EventListener};

use crate::message::Message;

use super::socket::WriteHalf;

/// A low-level representation of a D-Bus connection
///
/// This wrapper is agnostic on the actual transport, using the `socket::WriteHalf` trait to
/// abstract it. It is compatible with sockets both in blocking or non-blocking
/// mode.
///
/// This wrapper abstracts away the serialization & buffering considerations of the
/// protocol, and allows interaction based on messages, rather than bytes.
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Connection<W> {
    activity_event: Arc<Event>,
    socket: Mutex<W>,
}

impl<W: WriteHalf> Connection<W> {
    pub(crate) fn new(socket: W) -> Connection<W> {
        Connection {
            activity_event: Arc::new(Event::new()),
            socket: Mutex::new(socket),
        }
    }

    /// Attempt to send a message.
    ///
    /// This method will thus only block if the socket is in blocking mode.
    pub async fn send_message(&self, msg: &Message) -> crate::Result<()> {
        self.activity_event.notify(usize::MAX);
        let mut socket = self.socket.lock().await;
        let mut pos = 0;
        let data = msg.as_bytes();
        while pos < data.len() {
            #[cfg(unix)]
            let fds = if pos == 0 { msg.fds() } else { vec![] };
            pos += socket
                .sendmsg(
                    &data[pos..],
                    #[cfg(unix)]
                    &fds,
                )
                .await?;
        }
        Ok(())
    }

    /// Close the connection.
    ///
    /// After this call, all reading and writing operations will fail.
    pub async fn close(&self) -> crate::Result<()> {
        self.activity_event.notify(usize::MAX);
        self.socket.lock().await.close().await.map_err(Into::into)
    }

    /// Access the underlying write half of the socket.
    ///
    /// This method is intended to provide access to the socket in order to access certain
    /// properties (e.g peer credentials).
    ///
    /// You should not try to write to it directly, as it may corrupt the internal state of this
    /// wrapper.
    pub async fn socket_write(&self) -> impl DerefMut<Target = W> + '_ {
        pub struct SocketDeref<'s, W: WriteHalf> {
            socket: MutexGuard<'s, W>,
        }

        impl<W> Deref for SocketDeref<'_, W>
        where
            W: WriteHalf,
        {
            type Target = W;

            fn deref(&self) -> &Self::Target {
                &self.socket
            }
        }

        impl<W> DerefMut for SocketDeref<'_, W>
        where
            W: WriteHalf,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.socket
            }
        }

        SocketDeref {
            socket: self.socket.lock().await,
        }
    }

    pub(crate) fn monitor_activity(&self) -> EventListener {
        self.activity_event.listen()
    }

    pub(crate) fn activity_event(&self) -> Arc<Event> {
        self.activity_event.clone()
    }
}
