use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use zvariant_derive::Type;

use crate::{MessageField, MessageFieldCode};

// Currently the maximum fields in a message can really be 7 (in method calls and signals) but
// let's keep it large enough in case we're wrong, as long as the size of MessageFields structure
// is below 1k.
const MAX_FIELDS_IN_MESSAGE: usize = 9;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct MessageFields<'m>(#[serde(borrow)] ArrayVec<[MessageField<'m>; MAX_FIELDS_IN_MESSAGE]>);

impl<'m> MessageFields<'m> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<'f: 'm>(&mut self, field: MessageField<'f>) {
        self.0.push(field);
    }

    pub fn get(&self) -> &[MessageField<'m>] {
        &self.0
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
        Self(ArrayVec::new())
    }
}

impl<'m> std::ops::Deref for MessageFields<'m> {
    type Target = [MessageField<'m>];

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
