use crate::Value;
use crate::{Array, Dict, Error};
use crate::{ObjectPath, Signature, Structure};

//
// Conversions from `Value` to encodable types
pub trait FromValue<'v> {
    fn from_value(value: Value<'v>) -> Result<Self, Error>
    where
        Self: Sized;
    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error>;
}

// u8

impl<'v> FromValue<'v> for u8 {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::U8(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::U8(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// bool

impl<'v> FromValue<'v> for bool {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::Bool(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::Bool(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i16

impl<'v> FromValue<'v> for i16 {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::I16(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::I16(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u16

impl<'v> FromValue<'v> for u16 {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::U16(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::U16(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i32

impl<'v> FromValue<'v> for i32 {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::I32(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::I32(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u32

impl<'v> FromValue<'v> for u32 {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::U32(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::U32(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// i64

impl<'v> FromValue<'v> for i64 {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::I64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::I64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// u64

impl<'v> FromValue<'v> for u64 {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::U64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::U64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// f64

impl<'v> FromValue<'v> for f64 {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::F64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::F64(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// &str

impl<'s, 'v: 's> FromValue<'v> for &'s str {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::Str(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::Str(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Not providing FromValue for String implementtion cause if we have Value::Str,
// it'll be valid to call String::from_value_ref() on it and we can't return reference
// to a value owned by the function itself.

// Signature

impl<'s, 'v: 's> FromValue<'v> for Signature<'s> {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::Signature(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::Signature(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// ObjectPath

impl<'o, 'v: 'o> FromValue<'v> for ObjectPath<'o> {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::ObjectPath(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::ObjectPath(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Value itself (flatten)

impl<'a, 'v: 'a> FromValue<'v> for Value<'a> {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::Value(value) = value {
            Ok(*value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::Value(value) = value {
            Ok(&*value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Array

impl<'r, 'v: 'r> FromValue<'v> for Array<'r> {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::Array(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::Array(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// DictEntry

impl<'d, 'v: 'd> FromValue<'v> for Dict<'d, 'd> {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::Dict(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::Dict(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}

// Structure

impl<'s, 'v: 's> FromValue<'v> for Structure<'s> {
    fn from_value(value: Value<'v>) -> Result<Self, Error> {
        if let Value::Structure(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }

    fn from_value_ref(value: &'v Value<'v>) -> Result<&'v Self, Error> {
        if let Value::Structure(value) = value {
            Ok(value)
        } else {
            Err(Error::IncorrectType)
        }
    }
}
