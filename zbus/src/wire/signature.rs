//! D-Bus type signatures.
//!
//! A [`Signature`] is the parsed form of a signature string, such as `a{sv}`. Its container types
//! are made up of [`Child`] and [`Fields`], [`Error`] is what parsing a signature can fail with
//! and [`validate`] checks a signature string.

use crate::wire::{Basic, Type};

pub use zbus_utils::signature::*;

impl Type for Signature {
    const SIGNATURE: &'static Signature = &Signature::Signature;
}

impl Basic for Signature {
    const SIGNATURE_CHAR: char = 'g';
    const SIGNATURE_STR: &'static str = "g";
}

impl From<Signature> for crate::Variant<'static> {
    fn from(value: Signature) -> Self {
        crate::Variant::Signature(value)
    }
}
