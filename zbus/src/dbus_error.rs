use crate::{Message, MessageHeader, Result};

/// An error type suitable for a dbus reply method
pub trait DBusError {
    /// Generate an error reply message for the given method call.
    fn create_reply(&self, msg: &MessageHeader<'_>) -> Result<Message>;

    fn name(&self) -> &str;

    fn description(&self) -> &str;
}
