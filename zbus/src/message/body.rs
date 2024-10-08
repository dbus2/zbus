use zvariant::{
    serialized::{self, Data},
    Signature, Type,
};

use crate::{Error, Message, Result};

/// The body of a message.
///
/// This contains the bytes and the signature of the body.
#[derive(Clone, Debug)]
pub struct Body {
    data: Data<'static, 'static>,
    msg: Message,
    signature: Signature,
}

impl Body {
    pub(super) fn new(data: Data<'static, 'static>, msg: Message) -> Self {
        let body_sig = msg.header().signature().clone();

        Self {
            data,
            msg,
            signature: body_sig,
        }
    }

    /// Deserialize the body using the contained signature.
    pub fn deserialize<'s, B>(&'s self) -> Result<B>
    where
        B: zvariant::DynamicDeserialize<'s>,
    {
        let header = self.msg.header();
        let body_sig = header.signature();

        self.data
            .deserialize_for_dynamic_signature(body_sig)
            .map_err(Error::from)
            .map(|b| b.0)
    }

    /// Deserialize the body (without checking signature matching).
    pub fn deserialize_unchecked<'d, 'm: 'd, B>(&'m self) -> Result<B>
    where
        B: serde::de::Deserialize<'d> + Type,
    {
        self.data.deserialize().map_err(Error::from).map(|b| b.0)
    }

    /// The signature of the body.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// The length of the body in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Whether the body is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get a reference to the underlying byte encoding of the message.
    pub fn data(&self) -> &serialized::Data<'static, 'static> {
        &self.data
    }

    /// Reference to the message this body belongs to.
    pub fn message(&self) -> &Message {
        &self.msg
    }
}
