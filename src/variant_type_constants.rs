use crate::{DictEntry, ObjectPath, Signature, Structure, Variant, VariantError};

// We've to keep a separate trait for associated constants since they are incompatible with
// trait-objects, and we need and want VariantType to be trait-object compatible.
pub trait VariantTypeConstants: std::fmt::Debug {
    const SIGNATURE_CHAR: char;
    const SIGNATURE_STR: &'static str;
    const ALIGNMENT: usize;
}

// As trait-object, you can only use the `encode` method but you can downcast it to the concrete

impl VariantTypeConstants for u8 {
    const SIGNATURE_CHAR: char = 'y';
    const SIGNATURE_STR: &'static str = "y";
    const ALIGNMENT: usize = 1;
}

impl VariantTypeConstants for bool {
    const SIGNATURE_CHAR: char = 'b';
    const SIGNATURE_STR: &'static str = "b";
    const ALIGNMENT: usize = 4;
}

impl VariantTypeConstants for i16 {
    const SIGNATURE_CHAR: char = 'n';
    const SIGNATURE_STR: &'static str = "n";
    const ALIGNMENT: usize = 2;
}

impl VariantTypeConstants for u16 {
    const SIGNATURE_CHAR: char = 'q';
    const SIGNATURE_STR: &'static str = "q";
    const ALIGNMENT: usize = 2;
}

impl VariantTypeConstants for i32 {
    const SIGNATURE_CHAR: char = 'i';
    const SIGNATURE_STR: &'static str = "i";
    const ALIGNMENT: usize = 4;
}

impl VariantTypeConstants for u32 {
    const SIGNATURE_CHAR: char = 'u';
    const SIGNATURE_STR: &'static str = "u";
    const ALIGNMENT: usize = 4;
}

impl VariantTypeConstants for i64 {
    const SIGNATURE_CHAR: char = 'x';
    const SIGNATURE_STR: &'static str = "x";
    const ALIGNMENT: usize = 8;
}

impl VariantTypeConstants for u64 {
    const SIGNATURE_CHAR: char = 't';
    const SIGNATURE_STR: &'static str = "t";
    const ALIGNMENT: usize = 8;
}

impl VariantTypeConstants for f64 {
    const SIGNATURE_CHAR: char = 'd';
    const SIGNATURE_STR: &'static str = "d";
    const ALIGNMENT: usize = 8;
}

pub(crate) fn alignment_for_signature(signature: &str) -> Result<usize, VariantError> {
    match signature
        .chars()
        .next()
        .ok_or(VariantError::InsufficientData)?
    {
        // FIXME: There has to be a shorter way to do this
        u8::SIGNATURE_CHAR => Ok(u8::ALIGNMENT),
        bool::SIGNATURE_CHAR => Ok(bool::ALIGNMENT),
        i16::SIGNATURE_CHAR => Ok(i16::ALIGNMENT),
        u16::SIGNATURE_CHAR => Ok(u16::ALIGNMENT),
        i32::SIGNATURE_CHAR => Ok(i32::ALIGNMENT),
        u32::SIGNATURE_CHAR => Ok(u32::ALIGNMENT),
        i64::SIGNATURE_CHAR => Ok(i64::ALIGNMENT),
        u64::SIGNATURE_CHAR => Ok(u64::ALIGNMENT),
        f64::SIGNATURE_CHAR => Ok(f64::ALIGNMENT),
        <String>::SIGNATURE_CHAR => Ok(<String>::ALIGNMENT),
        // Doesn't matter what type for T we use here, alignment is the same
        Vec::<bool>::SIGNATURE_CHAR => Ok(Vec::<bool>::ALIGNMENT),
        ObjectPath::SIGNATURE_CHAR => Ok(ObjectPath::ALIGNMENT),
        Signature::SIGNATURE_CHAR => Ok(Signature::ALIGNMENT),
        Structure::SIGNATURE_CHAR => Ok(Structure::ALIGNMENT),
        Variant::SIGNATURE_CHAR => Ok(Variant::ALIGNMENT),
        // Doesn't matter what type for T we use here, alignment is the same
        DictEntry::<bool, bool>::SIGNATURE_CHAR => Ok(DictEntry::<bool, bool>::ALIGNMENT),
        _ => return Err(VariantError::UnsupportedType(String::from(signature))),
    }
}
