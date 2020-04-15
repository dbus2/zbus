use crate::Variant;
use crate::{Array, Dict, Error};
use crate::{ObjectPath, Signature, Structure};

//
// Conversions from `Variant` to encodable types
pub trait FromVariant<'v> {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error>;
}

// u8

impl<'v> FromVariant<'v> for u8 {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::U8(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::U8(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// bool

impl<'v> FromVariant<'v> for bool {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::Bool(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::Bool(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i16

impl<'v> FromVariant<'v> for i16 {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::I16(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::I16(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u16

impl<'v> FromVariant<'v> for u16 {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::U16(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::U16(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i32

impl<'v> FromVariant<'v> for i32 {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::I32(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::I32(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u32

impl<'v> FromVariant<'v> for u32 {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::U32(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::U32(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i64

impl<'v> FromVariant<'v> for i64 {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::I64(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::I64(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u64

impl<'v> FromVariant<'v> for u64 {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::U64(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::U64(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// f64

impl<'v> FromVariant<'v> for f64 {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::F64(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::F64(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// &str

impl<'s, 'v: 's> FromVariant<'v> for &'s str {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::Str(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::Str(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Not providing FromVariant for String implementtion cause if we have Variant::Str,
// it'll be valid to call String::from_variant_ref() on it and we can't return reference
// to a value owned by the function itself.

// Signature

impl<'s, 'v: 's> FromVariant<'v> for Signature<'s> {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::Signature(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::Signature(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// ObjectPath

impl<'o, 'v: 'o> FromVariant<'v> for ObjectPath<'o> {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::ObjectPath(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::ObjectPath(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Variant itself (flatten)

impl<'a, 'v: 'a> FromVariant<'v> for Variant<'a> {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::Variant(value) = variant {
            Ok(*value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::Variant(value) = variant {
            Ok(&*value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Array

impl<'r, 'v: 'r> FromVariant<'v> for Array<'r> {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::Array(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::Array(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// DictEntry

impl<'d, 'v: 'd> FromVariant<'v> for Dict<'d, 'd> {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::Dict(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::Dict(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Structure

impl<'s, 'v: 's> FromVariant<'v> for Structure<'s> {
    fn from_variant(variant: Variant<'v>) -> Result<Self, Error> {
        if let Variant::Structure(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_variant_ref(variant: &'v Variant<'v>) -> Result<&'v Self, Error> {
        if let Variant::Structure(value) = variant {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}
