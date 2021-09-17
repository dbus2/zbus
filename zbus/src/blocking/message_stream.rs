use futures_util::StreamExt;
use static_assertions::assert_impl_all;
use std::sync::Arc;

use async_io::block_on;

use crate::{azync, blocking::Connection, Message, Result};

/// Synchronous sibling of [`azync::MessageStream`].
///
/// Just like [`azync::MessageStream`] must be continuously polled, you must continuously iterate
/// over this type until it's consumed or dropped.
#[derive(derivative::Derivative, Clone)]
#[derivative(Debug)]
pub struct MessageStream(pub(crate) azync::MessageStream);

assert_impl_all!(MessageStream: Send, Sync, Unpin);

impl MessageStream {
    /// Get a reference to the underlying async message stream.
    pub fn inner(&self) -> &azync::MessageStream {
        &self.0
    }

    /// Get the underlying async message stream, consuming `self`.
    pub fn into_inner(self) -> azync::MessageStream {
        self.0
    }
}

impl Iterator for MessageStream {
    type Item = Result<Arc<Message>>;

    fn next(&mut self) -> Option<Self::Item> {
        block_on(self.0.next())
    }
}

impl From<Connection> for MessageStream {
    fn from(conn: Connection) -> Self {
        let azync = azync::MessageStream::from(conn.into_inner());

        Self(azync)
    }
}

impl From<&Connection> for MessageStream {
    fn from(conn: &Connection) -> Self {
        Self::from(conn.clone())
    }
}
