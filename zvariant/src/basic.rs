use crate::{serialized::Format, Signature, Type};

/// Trait for basic types.
///
/// All basic types are also [`Type`] implementers.
///
/// [`Type`]: trait.Type.html
/// [`Value`]: enum.Value.html
pub trait Basic: Type {
    /// The type signature, as a character.
    const SIGNATURE_CHAR: char;
    /// The type signature, as a string.
    const SIGNATURE_STR: &'static str;

    /// The required padding alignment for the given format.
    ///
    /// The default implementation covers all possible cases so you should never need to override
    /// it.
    fn alignment(format: Format) -> usize {
        Self::SIGNATURE.alignment(format)
    }
}

impl<B: ?Sized> Basic for &B
where
    B: Basic,
{
    const SIGNATURE_CHAR: char = B::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = B::SIGNATURE_STR;
}

macro_rules! impl_type {
    ($for:ty) => {
        impl Type for $for {
            const SIGNATURE: &'static Signature = {
                match Self::SIGNATURE_CHAR {
                    'y' => &Signature::U8,
                    'b' => &Signature::Bool,
                    'n' => &Signature::I16,
                    'q' => &Signature::U16,
                    'i' => &Signature::I32,
                    'u' => &Signature::U32,
                    'x' => &Signature::I64,
                    't' => &Signature::U64,
                    'd' => &Signature::F64,
                    's' => &Signature::Str,
                    'g' => &Signature::Signature,
                    'o' => &Signature::ObjectPath,
                    'v' => &Signature::Variant,
                    #[cfg(unix)]
                    'h' => &Signature::Fd,
                    _ => unreachable!(),
                }
            };
        }
    };
}

impl Basic for u8 {
    const SIGNATURE_CHAR: char = 'y';
    const SIGNATURE_STR: &'static str = "y";
}
impl_type!(u8);

impl Basic for std::num::NonZeroU8 {
    const SIGNATURE_CHAR: char = u8::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = u8::SIGNATURE_STR;
}
impl_type!(std::num::NonZeroU8);

// No i8 type in D-Bus/GVariant, let's pretend it's i16
impl Basic for i8 {
    const SIGNATURE_CHAR: char = i16::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = i16::SIGNATURE_STR;
}
impl_type!(i8);

impl Basic for std::num::NonZeroI8 {
    const SIGNATURE_CHAR: char = i8::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = i8::SIGNATURE_STR;
}
impl_type!(std::num::NonZeroI8);

impl Basic for bool {
    const SIGNATURE_CHAR: char = 'b';
    const SIGNATURE_STR: &'static str = "b";
}
impl_type!(bool);

impl Basic for i16 {
    const SIGNATURE_CHAR: char = 'n';
    const SIGNATURE_STR: &'static str = "n";
}
impl_type!(i16);

impl Basic for std::num::NonZeroI16 {
    const SIGNATURE_CHAR: char = i16::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = i16::SIGNATURE_STR;
}
impl_type!(std::num::NonZeroI16);

impl Basic for u16 {
    const SIGNATURE_CHAR: char = 'q';
    const SIGNATURE_STR: &'static str = "q";
}
impl_type!(u16);

impl Basic for std::num::NonZeroU16 {
    const SIGNATURE_CHAR: char = u16::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = u16::SIGNATURE_STR;
}
impl_type!(std::num::NonZeroU16);

impl Basic for i32 {
    const SIGNATURE_CHAR: char = 'i';
    const SIGNATURE_STR: &'static str = "i";
}
impl_type!(i32);

impl Basic for std::num::NonZeroI32 {
    const SIGNATURE_CHAR: char = i32::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = i32::SIGNATURE_STR;
}
impl_type!(std::num::NonZeroI32);

impl Basic for u32 {
    const SIGNATURE_CHAR: char = 'u';
    const SIGNATURE_STR: &'static str = "u";
}
impl_type!(u32);

impl Basic for std::num::NonZeroU32 {
    const SIGNATURE_CHAR: char = u32::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = u32::SIGNATURE_STR;
}
impl_type!(std::num::NonZeroU32);

impl Basic for i64 {
    const SIGNATURE_CHAR: char = 'x';
    const SIGNATURE_STR: &'static str = "x";
}
impl_type!(i64);

impl Basic for std::num::NonZeroI64 {
    const SIGNATURE_CHAR: char = i64::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = i64::SIGNATURE_STR;
}
impl_type!(std::num::NonZeroI64);

impl Basic for u64 {
    const SIGNATURE_CHAR: char = 't';
    const SIGNATURE_STR: &'static str = "t";
}
impl_type!(u64);

impl Basic for std::num::NonZeroU64 {
    const SIGNATURE_CHAR: char = u64::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = u64::SIGNATURE_STR;
}
impl_type!(std::num::NonZeroU64);

// No f32 type in D-Bus/GVariant, let's pretend it's f64
impl Basic for f32 {
    const SIGNATURE_CHAR: char = f64::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = f64::SIGNATURE_STR;
}
impl_type!(f32);

impl Basic for f64 {
    const SIGNATURE_CHAR: char = 'd';
    const SIGNATURE_STR: &'static str = "d";
}
impl_type!(f64);

impl Basic for str {
    const SIGNATURE_CHAR: char = 's';
    const SIGNATURE_STR: &'static str = "s";
}
impl_type!(str);

impl Basic for String {
    const SIGNATURE_CHAR: char = 's';
    const SIGNATURE_STR: &'static str = "s";
}
impl_type!(String);

impl Basic for char {
    const SIGNATURE_CHAR: char = <&str>::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = <&str>::SIGNATURE_STR;
}
impl_type!(char);
