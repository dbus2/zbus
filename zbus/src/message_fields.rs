use serde::{Deserialize, Serialize};
use zvariant_derive::Type;

use crate::{MessageField, MessageFieldCode};

// It's actually 10 (and even not that) but let's round it to next 8-byte alignment
const MAX_FIELDS_IN_MESSAGE: usize = 16;

// FIXME: Use ArrayVec
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct MessageFields<'m>(#[serde(borrow)] Vec<MessageField<'m>>);

impl<'m> MessageFields<'m> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_from_vec<'f: 'm>(fields: Vec<MessageField<'f>>) -> Self {
        Self(fields)
    }

    pub fn add<'f: 'm>(&mut self, field: MessageField<'f>) {
        self.0.push(field);
    }

    pub fn get(&self) -> &Vec<MessageField<'m>> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<MessageField<'m>> {
        &mut self.0
    }

    pub fn into_inner(self) -> Vec<MessageField<'m>> {
        self.0
    }

    pub fn get_field(&self, code: MessageFieldCode) -> Option<&MessageField<'m>> {
        self.0.iter().find(|f| f.code() == code)
    }

    pub fn take_field(self, code: MessageFieldCode) -> Option<MessageField<'m>> {
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
    type Target = Vec<MessageField<'m>>;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<'m> std::ops::DerefMut for MessageFields<'m> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
