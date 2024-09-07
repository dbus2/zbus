use core::fmt;

mod child_signature;

pub use child_signature::ChildSignature;
mod fields_signatures;
pub use fields_signatures::FieldsSignatures;
pub mod signature;
pub use signature::Signature;

/// Enum representing the max depth exceeded error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid signature.
    InvalidSignature,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSignature => write!(f, "Invalid signature"),
        }
    }
}

#[cfg(test)]
mod tests;
