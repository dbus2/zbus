use core::convert::TryFrom;

use crate::{Array, MessageField, MessageFieldError};
use crate::{Structure, Variant, VariantType};

// It's actually 10 (and even not that) but let's round it to next 8-byte alignment
const MAX_FIELDS_IN_MESSAGE: usize = 16;

#[derive(Debug)]
pub struct MessageFields(Vec<MessageField>);

impl MessageFields {
    pub fn new() -> Self {
        Self(Vec::with_capacity(MAX_FIELDS_IN_MESSAGE))
    }

    pub fn new_from_vec(fields: Vec<MessageField>) -> Self {
        Self(fields)
    }

    pub fn add(&mut self, field: MessageField) {
        self.0.push(field);
    }

    pub fn inner(&self) -> &Vec<MessageField> {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut Vec<MessageField> {
        &mut self.0
    }

    pub fn take_inner(self) -> Vec<MessageField> {
        self.0
    }
}

impl From<MessageFields> for Array {
    fn from(fields: MessageFields) -> Self {
        let mut v: Vec<Variant> = vec![];

        for field in fields.take_inner() {
            v.push(field.take_inner().to_variant());
        }

        Array::new_from_vec(v)
    }
}

impl TryFrom<Array> for MessageFields {
    type Error = MessageFieldError;

    fn try_from(array: Array) -> Result<Self, MessageFieldError> {
        let mut fields = MessageFields::new();

        for value in array.take_inner() {
            let structure =
                Structure::take_from_variant(value).map_err(|e| MessageFieldError::from(e))?;
            let field = MessageField::try_from(structure)?;

            fields.add(field);
        }

        Ok(fields)
    }
}

impl std::ops::Deref for MessageFields {
    type Target = Vec<MessageField>;

    fn deref(&self) -> &Self::Target {
        self.inner()
    }
}

impl std::ops::DerefMut for MessageFields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner_mut()
    }
}
