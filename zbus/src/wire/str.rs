/// This type is used for keeping strings in a [`Variant`], among other things.
///
/// [`Variant`]: crate::Variant::Str
pub use zcheapstr::CheapStr as Str;

use crate::wire::{Basic, Type};

impl Basic for Str<'_> {
    const SIGNATURE_CHAR: char = <&str>::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = <&str>::SIGNATURE_STR;
}

impl Type for Str<'_> {
    const SIGNATURE: &'static crate::Signature = &crate::Signature::Str;
}
