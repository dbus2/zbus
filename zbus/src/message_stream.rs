use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use async_broadcast::Receiver as ActiveReceiver;
use async_channel::Receiver;
use futures_core::{ready, stream};
use futures_util::stream::FusedStream;
use static_assertions::assert_impl_all;

use crate::{Connection, Error, Message, Result};

/// A [`stream::Stream`] implementation that yields [`Message`] items.
///
/// You can convert a [`Connection`] to this type.
///
/// **NOTE**: You must ensure a `MessageStream` is continuously polled or you will experience hangs.
/// If you don't need to continuously poll the `MessageStream` but need to keep it around for later
/// use, keep the connection around and convert it into a `MessageStream` when needed. The
/// conversion is not an expensive operation so you don't need to  worry about performance, unless
/// you do it very frequently. If you need to convert back and forth frequently, you may want to
/// consider keeping both a connection and stream around.
#[derive(Clone, Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct MessageStream {
    msg_receiver: ActiveReceiver<Arc<Message>>,
    error_receiver: Receiver<Error>,
}

assert_impl_all!(MessageStream: Send, Sync, Unpin);

impl stream::Stream for MessageStream {
    type Item = Result<Arc<Message>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if !this.msg_receiver.is_terminated() {
            if let Some(msg) = ready!(Pin::new(&mut this.msg_receiver).poll_next(cx)) {
                return Poll::Ready(Some(Ok(msg)));
            }
        }
        // If msg_receiver is terminated or returns None, try returning the error
        Pin::new(&mut this.error_receiver)
            .poll_next(cx)
            .map(|v| v.map(Err))
    }
}

impl From<Connection> for MessageStream {
    fn from(conn: Connection) -> Self {
        let msg_receiver = conn.msg_receiver.activate();
        let error_receiver = conn.error_receiver;

        Self {
            msg_receiver,
            error_receiver,
        }
    }
}

impl From<&Connection> for MessageStream {
    fn from(conn: &Connection) -> Self {
        Self::from(conn.clone())
    }
}
