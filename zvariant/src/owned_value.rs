use std::{collections::HashMap, convert::TryFrom, hash::BuildHasher};

use crate::Value;

/// Owned [`Value`](enum.Value.html)
pub type OwnedValue = Value<'static>;

impl TryFrom<OwnedValue> for Vec<OwnedValue> {
    type Error = crate::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        if let Value::Array(v) = value {
            Self::try_from(v)
        } else {
            Err(crate::Error::IncorrectType)
        }
    }
}

impl<'k, 'v, K, V, H> TryFrom<OwnedValue> for HashMap<K, V, H>
where
    K: crate::Basic + TryFrom<Value<'k>, Error = crate::Error> + std::hash::Hash + std::cmp::Eq,
    V: TryFrom<Value<'v>, Error = crate::Error>,
    H: BuildHasher + Default,
{
    type Error = crate::Error;

    fn try_from(value: OwnedValue) -> Result<Self, Self::Error> {
        if let Value::Dict(v) = value {
            Self::try_from(v)
        } else {
            Err(crate::Error::IncorrectType)
        }
    }
}

// Without specialization, conflicts with `impl<T> From<T> for T;` from `core`
/*
impl<'a> From<Value<'a>> for OwnedValue {
    fn from(v: Value<'a>) -> Self {
        // TODO: add into_owned, avoiding copy if already owned..
        v.to_owned()
    }
}
*/

impl<'a> From<&Value<'a>> for OwnedValue {
    fn from(v: &Value<'a>) -> Self {
        v.to_owned()
    }
}

// Overlaps with `impl<'de: 'a, 'a> Deserialize<'de> for Value<'a>`
// But that does not replace this because of the `'de: 'a` bound...
/*
impl<'de> Deserialize<'de> for OwnedValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Value::deserialize(deserializer)?.into())
    }
}
*/

#[cfg(test)]
mod tests {
    use byteorder::LE;
    use std::{convert::TryFrom, error::Error, result::Result};

    use crate::{from_slice, to_bytes, EncodingContext, OwnedValue, Value};

    #[cfg(feature = "enumflags2")]
    #[test]
    fn bitflags() -> Result<(), Box<dyn Error>> {
        #[repr(u32)]
        #[derive(enumflags2::BitFlags, Copy, Clone, Debug)]
        pub enum Flaggy {
            One = 0x1,
            Two = 0x2,
        }

        let v = Value::from(0x2u32);
        let ov: OwnedValue = v.into();
        assert_eq!(<enumflags2::BitFlags<Flaggy>>::try_from(ov)?, Flaggy::Two);
        Ok(())
    }

    #[test]
    fn from_value() -> Result<(), Box<dyn Error>> {
        let v = Value::from("hi!");
        let ov: OwnedValue = v.into();
        assert_eq!(<&str>::try_from(&ov)?, "hi!");
        Ok(())
    }

    #[test]
    fn serde() -> Result<(), Box<dyn Error>> {
        let ec = EncodingContext::<LE>::new_dbus(0);
        let ov: OwnedValue = Value::from("hi!").into();
        let ser = to_bytes(ec, &ov)?;
        let de: Value<'_> = from_slice(&ser, ec)?;
        assert_eq!(<&str>::try_from(&de)?, "hi!");
        Ok(())
    }
}
