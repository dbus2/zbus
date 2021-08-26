use serde::{Deserialize, Serialize};
use static_assertions::assert_impl_all;
use zvariant::derive::Type;

use crate::{MessageField, MessageFieldCode};

// It's actually 10 (and even not that) but let's round it to next 8-byte alignment
const MAX_FIELDS_IN_MESSAGE: usize = 16;

/// A collection of [`MessageField`] instances.
///
/// [`MessageField`]: enum.MessageField.html
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct MessageFields<'m>(#[serde(borrow)] Vec<MessageField<'m>>);

assert_impl_all!(MessageFields<'_>: Send, Sync, Unpin);

impl<'m> MessageFields<'m> {
    /// Creates an empty collection of fields.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a [`MessageField`] to the collection of fields in the message.
    ///
    /// [`MessageField`]: enum.MessageField.html
    pub fn add<'f: 'm>(&mut self, field: MessageField<'f>) {
        self.0.push(field);
    }

    /// Replaces a [`MessageField`] from the collection of fields with one with the same code,
    /// returning the old value if present.
    ///
    /// [`MessageField`]: enum.MessageField.html
    pub fn replace<'f: 'm>(&mut self, field: MessageField<'f>) -> Option<MessageField<'m>> {
        let code = field.code();
        if let Some(found) = self.0.iter_mut().find(|f| f.code() == code) {
            return Some(std::mem::replace(found, field));
        }
        self.add(field);
        None
    }
    /// Returns a slice with all the [`MessageField`] in the message.
    ///
    /// [`MessageField`]: enum.MessageField.html
    pub fn get(&self) -> &[MessageField<'m>] {
        &self.0
    }

    /// Gets a reference to a specific [`MessageField`] by its code.
    ///
    /// Returns `None` if the message has no such field.
    ///
    /// [`MessageField`]: enum.MessageField.html
    pub fn get_field(&self, code: MessageFieldCode) -> Option<&MessageField<'m>> {
        self.0.iter().find(|f| f.code() == code)
    }

    /// Consumes the `MessageFields` and returns a specific [`MessageField`] by its code.
    ///
    /// Returns `None` if the message has no such field.
    ///
    /// [`MessageField`]: enum.MessageField.html
    pub fn into_field(self, code: MessageFieldCode) -> Option<MessageField<'m>> {
        for field in self.0 {
            if field.code() == code {
                return Some(field);
            }
        }

        None
    }
}

impl<'m> Default for MessageFields<'m> {
    fn default() -> Self {
        Self(Vec::with_capacity(MAX_FIELDS_IN_MESSAGE))
    }
}

impl<'m> std::ops::Deref for MessageFields<'m> {
    type Target = [MessageField<'m>];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{MessageField, MessageFields};

    #[test]
    fn test() {
        let mut mf = MessageFields::new();
        assert_eq!(mf.len(), 0);
        mf.add(MessageField::ReplySerial(42));
        assert_eq!(mf.len(), 1);
        mf.add(MessageField::ReplySerial(43));
        assert_eq!(mf.len(), 2);

        let mut mf = MessageFields::new();
        assert_eq!(mf.len(), 0);
        mf.replace(MessageField::ReplySerial(42));
        assert_eq!(mf.len(), 1);
        mf.replace(MessageField::ReplySerial(43));
        assert_eq!(mf.len(), 1);
    }
}
