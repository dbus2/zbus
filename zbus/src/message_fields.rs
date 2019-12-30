use core::convert::{TryFrom, TryInto};

use zvariant::{Array, Encode};
use zvariant::{Structure, Variant};

use crate::{MessageField, MessageFieldError};

// It's actually 10 (and even not that) but let's round it to next 8-byte alignment
const MAX_FIELDS_IN_MESSAGE: usize = 16;

#[derive(Debug)]
pub struct MessageFields(Vec<MessageField>);

impl MessageFields {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_from_vec(fields: Vec<MessageField>) -> Self {
        Self(fields)
    }

    pub fn add(&mut self, field: MessageField) {
        self.0.push(field);
    }

    pub fn get(&self) -> &Vec<MessageField> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<MessageField> {
        &mut self.0
    }

    pub fn into_inner(self) -> Vec<MessageField> {
        self.0
    }
}

impl Default for MessageFields {
    fn default() -> Self {
        Self(Vec::with_capacity(MAX_FIELDS_IN_MESSAGE))
    }
}

impl From<MessageFields> for Array {
    fn from(fields: MessageFields) -> Self {
        let mut v: Vec<Variant> = vec![];

        for field in fields.into_inner() {
            v.push(field.into_inner().to_variant());
        }

        Array::new_from_vec_unchecked(v)
    }
}

impl TryFrom<Array> for MessageFields {
    type Error = MessageFieldError;

    fn try_from(array: Array) -> Result<Self, MessageFieldError> {
        let mut fields = MessageFields::new();

        let vec: Vec<Structure> = array.try_into().map_err(MessageFieldError::from)?;
        for structure in vec {
            let field = structure.try_into()?;

            fields.add(field);
        }

        Ok(fields)
    }
}

impl std::ops::Deref for MessageFields {
    type Target = Vec<MessageField>;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl std::ops::DerefMut for MessageFields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
