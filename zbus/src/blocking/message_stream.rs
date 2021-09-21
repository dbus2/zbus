use futures_util::StreamExt;
use static_assertions::assert_impl_all;
use std::sync::Arc;

use crate::{block_on, blocking::Connection, Message, Result};

/// A blocking wrapper of [`crate::MessageStream`].
///
/// Just like [`crate::MessageStream`] must be continuously polled, you must continuously iterate
/// over this type until it's consumed or dropped.
#[derive(derivative::Derivative, Clone)]
#[derivative(Debug)]
pub struct MessageStream(pub(crate) crate::MessageStream);

assert_impl_all!(MessageStream: Send, Sync, Unpin);

impl MessageStream {
    /// Get a reference to the underlying async message stream.
    pub fn inner(&self) -> &crate::MessageStream {
        &self.0
    }

    /// Get the underlying async message stream, consuming `self`.
    pub fn into_inner(self) -> crate::MessageStream {
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
        let azync = crate::MessageStream::from(conn.into_inner());

        Self(azync)
    }
}

impl From<&Connection> for MessageStream {
    fn from(conn: &Connection) -> Self {
        Self::from(conn.clone())
    }
}
