use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use zvariant_derive::Type;

use crate::{MessageError, MessageFields};

pub const PRIMARY_HEADER_SIZE: usize = 12;
pub const MIN_MESSAGE_SIZE: usize = PRIMARY_HEADER_SIZE + 4;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize_repr, PartialEq, Serialize_repr, Type)]
pub enum EndianSig {
    Big = b'B',
    Little = b'l',
}

// Such a shame I've to do this manually
impl TryFrom<u8> for EndianSig {
    type Error = MessageError;

    fn try_from(val: u8) -> Result<EndianSig, MessageError> {
        match val {
            b'B' => Ok(EndianSig::Big),
            b'l' => Ok(EndianSig::Little),
            _ => Err(MessageError::IncorrectEndian),
        }
    }
}

#[cfg(target_endian = "big")]
pub const NATIVE_ENDIAN_SIG: EndianSig = EndianSig::Big;
#[cfg(target_endian = "little")]
pub const NATIVE_ENDIAN_SIG: EndianSig = EndianSig::Little;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize_repr, PartialEq, Serialize_repr, Type)]
pub enum MessageType {
    Invalid = 0,
    MethodCall = 1,
    MethodReturn = 2,
    Error = 3,
    Signal = 4,
}

// Such a shame I've to do this manually
impl From<u8> for MessageType {
    fn from(val: u8) -> MessageType {
        match val {
            1 => MessageType::MethodCall,
            2 => MessageType::MethodReturn,
            3 => MessageType::Error,
            4 => MessageType::Signal,
            _ => MessageType::Invalid,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize_repr, PartialEq, Serialize_repr, Type)]
pub enum MessageFlags {
    /// No flags.
    None = 0x0,
    /// This message does not expect method return replies or error replies, even if it is of a type
    /// that can have a reply; the reply should be omitted.
    ///
    /// Note that `MessageType::MethodCall` is the only message type currently defined in the
    /// specification that can expect a reply, so the presence or absence of this flag in the other
    /// three message types that are currently documented is meaningless: replies to those message
    /// types should not be sent, whether this flag is present or not.
    NoReplyExpected = 0x1,
    /// The bus must not launch an owner for the destination name in response to this message.
    NoAutoStart = 0x2,
    /// This flag may be set on a method call message to inform the receiving side that the caller
    /// is prepared to wait for interactive authorization, which might take a considerable time to
    /// complete. For instance, if this flag is set, it would be appropriate to query the user for
    /// passwords or confirmation via Polkit or a similar framework.
    AllowInteractiveAuth = 0x4,
}

// Such a shame I've to do this manually
impl From<u8> for MessageFlags {
    fn from(val: u8) -> MessageFlags {
        match val {
            0x1 => MessageFlags::NoReplyExpected,
            0x2 => MessageFlags::NoAutoStart,
            0x4 => MessageFlags::AllowInteractiveAuth,
            // According to the spec, unknown flags must be ignored
            _ => MessageFlags::None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct MessagePrimaryHeader {
    endian_sig: EndianSig,
    msg_type: MessageType,
    flags: MessageFlags,
    protocol_version: u8,
    body_len: u32,
    serial_num: u32,
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct MessageHeader<'m> {
    primary: MessagePrimaryHeader,
    #[serde(borrow)]
    fields: MessageFields<'m>,
    end: ((),), // To ensure header end on 8-byte boundry
}

impl MessagePrimaryHeader {
    pub fn new(msg_type: MessageType) -> Self {
        Self {
            endian_sig: NATIVE_ENDIAN_SIG,
            msg_type,
            flags: MessageFlags::None,
            protocol_version: 1,
            body_len: 0, // set to 0 at first
            serial_num: u32::max_value(),
        }
    }
    pub fn endian_sig(&self) -> EndianSig {
        self.endian_sig
    }

    pub fn set_endian_sig(&mut self, sig: EndianSig) {
        self.endian_sig = sig;
    }

    pub fn msg_type(&self) -> MessageType {
        self.msg_type
    }

    pub fn set_msg_type(&mut self, msg_type: MessageType) {
        self.msg_type = msg_type;
    }

    pub fn flags(&self) -> MessageFlags {
        self.flags
    }

    pub fn set_flags(&mut self, flags: MessageFlags) {
        self.flags = flags;
    }

    pub fn protocol_version(&self) -> u8 {
        self.protocol_version
    }

    pub fn set_protocol_version(&mut self, version: u8) {
        self.protocol_version = version;
    }

    pub fn body_len(&self) -> u32 {
        self.body_len
    }

    pub fn set_body_len(&mut self, len: u32) {
        self.body_len = len;
    }

    pub fn serial_num(&self) -> u32 {
        self.serial_num
    }

    pub fn set_serial_num(&mut self, serial: u32) {
        self.serial_num = serial;
    }
}

impl<'m> MessageHeader<'m> {
    pub fn new(primary: MessagePrimaryHeader, fields: MessageFields<'m>) -> Self {
        Self {
            primary,
            fields,
            end: ((),),
        }
    }

    pub fn primary(&self) -> &MessagePrimaryHeader {
        &self.primary
    }

    pub fn primary_mut(&mut self) -> &mut MessagePrimaryHeader {
        &mut self.primary
    }

    pub fn take_primary(self) -> MessagePrimaryHeader {
        self.primary
    }

    pub fn fields(&self) -> &MessageFields {
        &self.fields
    }

    pub fn fields_mut<'s: 'm>(&'s mut self) -> &'s mut MessageFields {
        &mut self.fields
    }

    pub fn take_fields(self) -> MessageFields<'m> {
        self.fields
    }
}
