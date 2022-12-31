use futures_util::StreamExt;
use static_assertions::assert_impl_all;
use std::{convert::TryInto, sync::Arc};

use crate::{blocking::Connection, utils::block_on, MatchRule, Message, OwnedMatchRule, Result};

/// A blocking wrapper of [`crate::MessageStream`].
///
/// Just like [`crate::MessageStream`] must be continuously polled, you must continuously iterate
/// over this type until it's consumed or dropped.
#[derive(derivative::Derivative, Clone)]
#[derivative(Debug)]
pub struct MessageIterator(pub(crate) crate::MessageStream);

assert_impl_all!(MessageIterator: Send, Sync, Unpin);

impl MessageIterator {
    /// Get a reference to the underlying async message stream.
    pub fn inner(&self) -> &crate::MessageStream {
        &self.0
    }

    /// Get the underlying async message stream, consuming `self`.
    pub fn into_inner(self) -> crate::MessageStream {
        self.0
    }

    /// Create a message iterator for the given match rule.
    ///
    /// This is a wrapper around [`crate::MessageStream::for_match_rule`].
    ///
    /// # Example
    ///
    /// ```
    /// use zbus::{blocking::{Connection, MessageIterator}, MatchRule, fdo::NameOwnerChanged};
    ///
    ///# fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let conn = Connection::session()?;
    /// let rule = MatchRule::builder()
    ///     .msg_type(zbus::MessageType::Signal)
    ///     .sender("org.freedesktop.DBus")?
    ///     .interface("org.freedesktop.DBus")?
    ///     .member("NameOwnerChanged")?
    ///     .add_arg("org.freedesktop.zbus.MatchRuleStreamTest42")?
    ///     .build();
    /// let mut iter = MessageIterator::for_match_rule(rule, &conn)?;
    ///
    /// let rule_str = "type='signal',sender='org.freedesktop.DBus',\
    ///                 interface='org.freedesktop.DBus',member='NameOwnerChanged',\
    ///                 arg0='org.freedesktop.zbus.MatchRuleStreamTest42'";
    /// assert_eq!(
    ///     iter.match_rule().map(|r| r.to_string()).as_deref(),
    ///     Some(rule_str),
    /// );
    ///
    /// // We register 2 names, starting with the uninteresting one. If `iter` wasn't filtering
    /// // messages based on the match rule, we'd receive method return call for each of these 2
    /// // calls first.
    /// //
    /// // Note that the `NameOwnerChanged` signal will not be sent by the bus  for the first name
    /// // we register since we setup an arg filter.
    /// conn.request_name("org.freedesktop.zbus.MatchRuleStreamTest44")?;
    /// conn.request_name("org.freedesktop.zbus.MatchRuleStreamTest42")?;
    ///
    /// let msg = iter.next().unwrap()?;
    /// let signal = NameOwnerChanged::from_message(msg).unwrap();
    /// assert_eq!(signal.args()?.name(), "org.freedesktop.zbus.MatchRuleStreamTest42");
    ///
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// # Caveats
    ///
    /// Since this method relies on [`MatchRule::matches`], it inherits its caveats.
    pub fn for_match_rule<R>(rule: R, conn: &Connection) -> Result<Self>
    where
        R: TryInto<OwnedMatchRule>,
        R::Error: Into<crate::Error>,
    {
        block_on(crate::MessageStream::for_match_rule(rule, conn.inner())).map(Self)
    }

    /// The associated match rule, if any.
    pub fn match_rule(&self) -> Option<MatchRule<'_>> {
        self.0.match_rule()
    }
}

impl Iterator for MessageIterator {
    type Item = Result<Arc<Message>>;

    fn next(&mut self) -> Option<Self::Item> {
        block_on(self.0.next())
    }
}

impl From<Connection> for MessageIterator {
    fn from(conn: Connection) -> Self {
        let azync = crate::MessageStream::from(conn.into_inner());

        Self(azync)
    }
}

impl From<&Connection> for MessageIterator {
    fn from(conn: &Connection) -> Self {
        Self::from(conn.clone())
    }
}

impl From<MessageIterator> for Connection {
    fn from(iter: MessageIterator) -> Connection {
        Connection::from(crate::Connection::from(iter.0))
    }
}

impl From<&MessageIterator> for Connection {
    fn from(iter: &MessageIterator) -> Connection {
        Connection::from(crate::Connection::from(&iter.0))
    }
}
