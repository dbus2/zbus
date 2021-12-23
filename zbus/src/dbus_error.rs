use crate::{Message, MessageHeader, Result};

/// A trait that needs to be implemented by error types to be returned from D-Bus methods.
///
/// Typically, you'd use the [`crate::fdo::Error`] since that covers quite a lot of failures but
/// occasionally you might find yourself needing to use a custom error type. You'll need to
/// implement this trait for your error type. The easiest way to achieve that is to make use of the
/// [`DBusError` macro][dm].
///
/// [dm]: macro.DBusError.html
pub trait DBusError {
    /// Generate an error reply message for the given method call.
    fn create_reply(&self, msg: &MessageHeader<'_>) -> Result<Message>;

    fn name(&self) -> &str;

    fn description(&self) -> &str;
}
