use std::error;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use zvariant::IntoVariant;
use zvariant::{Error as VariantError, Variant};
use zvariant::{ObjectPath, Signature};
use zvariant_derive::VariantValue;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Deserialize_repr, PartialEq, Serialize_repr, VariantValue)]
pub enum MessageFieldCode {
    Invalid = 0,     // Not a valid field name.
    Path = 1,        // The object to send a call to, or the object a signal is emitted from.
    Interface = 2,   // The interface to invoke a method call on, or that a signal is emitted from.
    Member = 3,      // The member, either the method name or signal name.
    ErrorName = 4,   // The name of the error that occurred, for errors
    ReplySerial = 5, //	The serial number of the message this message is a reply to.
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

#[derive(Debug)]
pub enum MessageFieldError {
    InsufficientData,
    InvalidCode,
    InvalidUtf8,
    Variant(VariantError),
}

impl error::Error for MessageFieldError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            MessageFieldError::Variant(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for MessageFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageFieldError::InsufficientData => write!(f, "insufficient data"),
            MessageFieldError::InvalidCode => write!(f, "invalid field code"),
            MessageFieldError::InvalidUtf8 => write!(f, "invalid UTF-8"),
            MessageFieldError::Variant(e) => write!(f, "{}", e),
        }
    }
}

impl From<VariantError> for MessageFieldError {
    fn from(val: VariantError) -> MessageFieldError {
        MessageFieldError::Variant(val)
    }
}

#[derive(Debug, Serialize, Deserialize, VariantValue)]
pub struct MessageField<'v> {
    code: MessageFieldCode,
    #[serde(borrow)]
    value: Variant<'v>,
}

impl<'v> MessageField<'v> {
    pub fn code(&self) -> MessageFieldCode {
        self.code
    }

    //pub fn value<'d, 'f: 'd, V>(&'f self) -> Result<V, MessageFieldError>
    //where
    //    B: serde::de::Deserialize<'d> + VariantValue,
    //{
    pub fn value(&self) -> &Variant {
        &self.value
    }

    pub fn path<'o: 'v>(path: impl Into<ObjectPath<'o>>) -> Self {
        Self {
            code: MessageFieldCode::Path,
            value: path.into().into_variant(),
        }
    }

    pub fn interface<'i: 'v>(interface: &'i str) -> Self {
        Self {
            code: MessageFieldCode::Interface,
            value: interface.into_variant(),
        }
    }

    pub fn member<'m: 'v>(member: &'m str) -> Self {
        Self {
            code: MessageFieldCode::Member,
            value: member.into_variant(),
        }
    }

    pub fn error_name<'e: 'v>(error_name: &'e str) -> Self {
        Self {
            code: MessageFieldCode::ErrorName,
            value: error_name.into_variant(),
        }
    }

    pub fn reply_serial(serial: u32) -> Self {
        Self {
            code: MessageFieldCode::ReplySerial,
            value: serial.into_variant(),
        }
    }

    pub fn destination<'d: 'v>(destination: &'d str) -> Self {
        Self {
            code: MessageFieldCode::Destination,
            value: destination.into_variant(),
        }
    }

    pub fn sender<'s: 'v>(sender: &'s str) -> Self {
        Self {
            code: MessageFieldCode::Sender,
            value: sender.into_variant(),
        }
    }

    pub fn signature<'s: 'v>(signature: impl Into<Signature<'s>>) -> Self {
        Self {
            code: MessageFieldCode::Signature,
            value: signature.into().into_variant(),
        }
    }

    pub fn unix_fds(fd: u32) -> Self {
        Self {
            code: MessageFieldCode::UnixFDs,
            value: fd.into_variant(),
        }
    }
}
