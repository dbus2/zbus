use core::str;
use std::marker::PhantomData;

use serde::de::{Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};

use crate::wire::{Signature, Type};

/// A wrapper to deserialize a value to `T: Type + serde::Deserialize`.
///
/// When the type of a value is well-known, you may avoid the cost and complexity of wrapping to a
/// generic [`enum@crate::Variant`] and instead use this wrapper.
///
/// ```
/// # use zbus::{as_variant::{Deserialize, Serialize}, wire::{LE, serialized::Context, to_bytes}};
/// #
/// # let ctxt = Context::new(LE, 0);
/// # let array = [0, 1, 2];
/// # let v = Serialize(&array);
/// # let encoded = to_bytes(ctxt, &v).unwrap();
/// let decoded: Deserialize<[u8; 3]> = encoded.deserialize().unwrap().0;
/// # assert_eq!(decoded.0, array);
/// ```
pub struct Deserialize<'de, T: Type + serde::Deserialize<'de>>(
    pub T,
    std::marker::PhantomData<&'de T>,
);

impl<'de, T: Type + serde::Deserialize<'de>> serde::Deserialize<'de> for Deserialize<'de, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // `Variant`/`OwnedVariant` (and any other type whose signature is already `v`) deserialize
        // themselves from a variant, so the extra unwrapping below would expect a
        // variant-of-variant. Just delegate to their own `Deserialize` impl in that case.
        if T::SIGNATURE == &Signature::Variant {
            return Ok(Deserialize(T::deserialize(deserializer)?, PhantomData));
        }

        Ok(Deserialize(
            deserializer.deserialize_struct(
                "Variant",
                FIELDS,
                DeserializeValueVisitor(PhantomData),
            )?,
            PhantomData,
        ))
    }
}

const SIGNATURE: &str = "signature";
const VALUE: &str = "value";
const FIELDS: &[&str] = &[SIGNATURE, VALUE];

struct DeserializeValueVisitor<T>(PhantomData<T>);

impl<'de, T: Type + serde::Deserialize<'de>> Visitor<'de> for DeserializeValueVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Variant")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let sig: Signature = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        if T::SIGNATURE != &sig {
            let expected = format!("a value of signature `{}`", T::SIGNATURE);
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&sig.to_string()),
                &expected.as_str(),
            ));
        }

        seq.next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))
    }

    // `Serialize` writes a struct, which the D-Bus format hands back as the sequence above but a
    // self-describing format hands back as a map, in whatever order the producer wrote the two
    // fields. `T` is known here, so the value can be read whenever it turns up and the signature
    // only has to be checked against it.
    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut signature: Option<Signature> = None;
        let mut value: Option<T> = None;

        while let Some(field) = map.next_key::<String>()? {
            match field.as_str() {
                SIGNATURE => {
                    if signature.replace(map.next_value()?).is_some() {
                        return Err(serde::de::Error::duplicate_field(SIGNATURE));
                    }
                }
                VALUE => {
                    if value.replace(map.next_value()?).is_some() {
                        return Err(serde::de::Error::duplicate_field(VALUE));
                    }
                }
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        let signature = signature.ok_or_else(|| serde::de::Error::missing_field(SIGNATURE))?;
        if T::SIGNATURE != &signature {
            let expected = format!("a value of signature `{}`", T::SIGNATURE);
            return Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&signature.to_string()),
                &expected.as_str(),
            ));
        }

        value.ok_or_else(|| serde::de::Error::missing_field(VALUE))
    }
}

impl<'de, T: Type + serde::Deserialize<'de>> Type for Deserialize<'de, T> {
    const SIGNATURE: &'static Signature = &Signature::Variant;
}

/// Deserialize a value as a [`enum@zbus::Variant`].
pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: serde::Deserialize<'de> + Type + 'de,
{
    use serde::Deserialize as _;

    Deserialize::deserialize(deserializer).map(|v| v.0)
}

/// Deserialize an optional value as a [`enum@zbus::Variant`].
pub fn deserialize_optional<'de, T, D>(deserializer: D) -> std::result::Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::Deserialize<'de> + Type + 'de,
{
    deserialize(deserializer).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::as_variant::Serialize;

    // `Serialize` writes a struct. D-Bus reads that back as a sequence; a self-describing format
    // reads it back as a map, so both visitors have to exist for a value to survive a trip through
    // one of those.
    #[test]
    fn round_trips_through_a_self_describing_format() {
        let json = serde_json::to_string(&Serialize(&42u32)).unwrap();
        assert_eq!(json, r#"{"signature":"u","value":42}"#);

        let Deserialize(value, _) = serde_json::from_str::<Deserialize<'_, u32>>(&json).unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn reads_the_fields_in_either_order() {
        let Deserialize(value, _) =
            serde_json::from_str::<Deserialize<'_, u32>>(r#"{"value":42,"signature":"u"}"#)
                .unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn still_checks_the_signature_from_a_map() {
        let err = serde_json::from_str::<Deserialize<'_, u32>>(r#"{"signature":"s","value":42}"#)
            .err()
            .expect("a signature that is not `u` should be refused");
        assert!(err.to_string().contains("signature `u`"), "{err}");

        let err = serde_json::from_str::<Deserialize<'_, u32>>(r#"{"value":42}"#)
            .err()
            .expect("a map without a signature should be refused");
        assert!(
            err.to_string().contains("missing field `signature`"),
            "{err}"
        );
    }
}
