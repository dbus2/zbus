use std::convert::TryFrom;

use serde::de::{Deserialize, Deserializer, Error};
use serde::ser::{Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

use zvariant::{ObjectPath, Signature, Str, Type, Value};
use zvariant_derive::Type;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize_repr, PartialEq, Serialize_repr, Type)]
pub enum MessageFieldCode {
    Invalid = 0,     // Not a valid field name.
    Path = 1,        // The object to send a call to, or the object a signal is emitted from.
    Interface = 2,   // The interface to invoke a method call on, or that a signal is emitted from.
    Member = 3,      // The member, either the method name or signal name.
    ErrorName = 4,   // The name of the error that occurred, for errors
    ReplySerial = 5, // The serial number of the message this message is a reply to.
    Destination = 6, // The name of the connection this message is intended for.
    Sender = 7,      // Unique name of the sending connection.
    Signature = 8,   // The signature of the message body.
    UnixFDs = 9,     // The number of Unix file descriptors that accompany the message.
}

impl From<u8> for MessageFieldCode {
    fn from(val: u8) -> MessageFieldCode {
        match val {
            1 => MessageFieldCode::Path,
            2 => MessageFieldCode::Interface,
            3 => MessageFieldCode::Member,
            4 => MessageFieldCode::ErrorName,
            5 => MessageFieldCode::ReplySerial,
            6 => MessageFieldCode::Destination,
            7 => MessageFieldCode::Sender,
            8 => MessageFieldCode::Signature,
            9 => MessageFieldCode::UnixFDs,
            _ => MessageFieldCode::Invalid,
        }
    }
}

impl<'v> MessageField<'v> {
    pub fn code(&self) -> MessageFieldCode {
        match self {
            MessageField::Path(_) => MessageFieldCode::Path,
            MessageField::Interface(_) => MessageFieldCode::Interface,
            MessageField::Member(_) => MessageFieldCode::Member,
            MessageField::ErrorName(_) => MessageFieldCode::ErrorName,
            MessageField::ReplySerial(_) => MessageFieldCode::ReplySerial,
            MessageField::Destination(_) => MessageFieldCode::Destination,
            MessageField::Sender(_) => MessageFieldCode::Sender,
            MessageField::Signature(_) => MessageFieldCode::Signature,
            MessageField::UnixFDs(_) => MessageFieldCode::UnixFDs,
            MessageField::Invalid => MessageFieldCode::Invalid,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MessageField<'v> {
    Invalid,                  // Not a valid field name.
    Path(ObjectPath<'v>), // The object to send a call to, or the object a signal is emitted from.
    Interface(Str<'v>), // The interface to invoke a method call on, or that a signal is emitted from.
    Member(Str<'v>),    // The member, either the method name or signal name.
    ErrorName(Str<'v>), // The name of the error that occurred, for errors
    ReplySerial(u32),   // The serial number of the message this message is a reply to.
    Destination(Str<'v>), // The name of the connection this message is intended for.
    Sender(Str<'v>),    // Unique name of the sending connection.
    Signature(Signature<'v>), // The signature of the message body.
    UnixFDs(u32),       // The number of Unix file descriptors that accompany the message.
}

impl<'v> Type for MessageField<'v> {
    fn signature() -> Signature<'static> {
        Signature::from_str_unchecked("(yv)")
    }
}

impl<'v> Serialize for MessageField<'v> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let tuple: (MessageFieldCode, Value) = match self {
            MessageField::Path(value) => (MessageFieldCode::Path, value.clone().into()),
            MessageField::Interface(value) => (MessageFieldCode::Interface, value.as_str().into()),
            MessageField::Member(value) => (MessageFieldCode::Member, value.as_str().into()),
            MessageField::ErrorName(value) => (MessageFieldCode::ErrorName, value.as_str().into()),
            MessageField::ReplySerial(value) => (MessageFieldCode::ReplySerial, (*value).into()),
            MessageField::Destination(value) => {
                (MessageFieldCode::Destination, value.as_str().into())
            }
            MessageField::Sender(value) => (MessageFieldCode::Sender, value.as_str().into()),
            MessageField::Signature(value) => (MessageFieldCode::Signature, value.clone().into()),
            MessageField::UnixFDs(value) => (MessageFieldCode::UnixFDs, (*value).into()),
            // This is a programmer error
            MessageField::Invalid => panic!("Attempt to serialize invalid MessageField"),
        };

        tuple.serialize(serializer)
    }
}

impl<'de: 'v, 'v> Deserialize<'de> for MessageField<'v> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (code, value) = <(MessageFieldCode, Value)>::deserialize(deserializer)?;
        Ok(match code {
            MessageFieldCode::Path => {
                MessageField::Path(ObjectPath::try_from(value).map_err(D::Error::custom)?)
            }
            MessageFieldCode::Interface => {
                MessageField::Interface(Str::try_from(value).map_err(D::Error::custom)?)
            }
            MessageFieldCode::Member => {
                MessageField::Member(Str::try_from(value).map_err(D::Error::custom)?)
            }
            MessageFieldCode::ErrorName => MessageField::ErrorName(
                Str::try_from(value)
                    .map(Into::into)
                    .map_err(D::Error::custom)?,
            ),
            MessageFieldCode::ReplySerial => {
                MessageField::ReplySerial(u32::try_from(value).map_err(D::Error::custom)?)
            }
            MessageFieldCode::Destination => MessageField::Destination(
                Str::try_from(value)
                    .map(Into::into)
                    .map_err(D::Error::custom)?,
            ),
            MessageFieldCode::Sender => MessageField::Sender(
                Str::try_from(value)
                    .map(Into::into)
                    .map_err(D::Error::custom)?,
            ),
            MessageFieldCode::Signature => {
                MessageField::Signature(Signature::try_from(value).map_err(D::Error::custom)?)
            }
            MessageFieldCode::UnixFDs => {
                MessageField::UnixFDs(u32::try_from(value).map_err(D::Error::custom)?)
            }
            MessageFieldCode::Invalid => {
                return Err(Error::invalid_value(
                    serde::de::Unexpected::Unsigned(code as u64),
                    &"A valid D-Bus message field code",
                ));
            }
        })
    }
}
