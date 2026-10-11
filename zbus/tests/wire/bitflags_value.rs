//! A set of bit flags, declared the way the book's FAQ recommends: a newtype over its integer that
//! derives the wire traits itself, given its flag methods by the `impl` form of the `bitflags`
//! crate's `bitflags!` macro. zbus has no support for `bitflags` and needs none, so this goes
//! through public API only, as a user's crate would.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zbus::{
    Basic, Optional, OwnedVariant, Type, Variant,
    wire::{LE, serialized::Context, to_bytes},
};

#[test]
fn signature_is_the_integers() {
    assert_eq!(Permissions::SIGNATURE, u32::SIGNATURE);
    assert_eq!(Permissions::SIGNATURE_STR, "u");
    // The `Basic` implementation is what lets the flags be a dictionary key.
    assert_eq!(<HashMap<Permissions, String>>::SIGNATURE, "a{us}");
}

#[test]
fn encodes_as_the_integer() {
    let ctxt = Context::new(LE, 0);
    let flags = Permissions::READ | Permissions::WRITE;

    let encoded = to_bytes(ctxt, &flags).unwrap();
    assert_eq!(encoded.bytes(), to_bytes(ctxt, &0x3u32).unwrap().bytes());
    let decoded: Permissions = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, flags);

    let encoded = to_bytes(ctxt, &Permissions::empty()).unwrap();
    assert_eq!(encoded.bytes(), &[0, 0, 0, 0]);
    let decoded: Permissions = encoded.deserialize().unwrap().0;
    assert!(decoded.is_empty());
}

#[test]
fn decoding_retains_unknown_bits() {
    // 0x4 is no flag of `Permissions`. It survives decoding, from the wire format and from a
    // `Variant` alike, the way `Permissions::from_bits_retain` keeps it.
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &0x5u32).unwrap();
    let decoded: Permissions = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, Permissions::from_bits_retain(0x5));
    assert!(decoded.contains(Permissions::READ));
    assert!(!decoded.contains(Permissions::WRITE));
    assert_eq!(Permissions::from_bits(decoded.bits()), None);

    let decoded = Permissions::try_from(Variant::from(0x5u32)).unwrap();
    assert_eq!(decoded.bits(), 0x5);
}

#[test]
fn value_conversions() {
    let flags = Permissions::READ | Permissions::WRITE;

    assert_eq!(Variant::from(flags), Variant::U32(0x3));
    assert_eq!(Variant::from(Permissions::empty()), Variant::U32(0));
    assert_eq!(
        Permissions::try_from(Variant::from(0x2u32)).unwrap(),
        Permissions::WRITE
    );
    // Only the integer of the flags converts.
    Permissions::try_from(Variant::from(0x2u16)).unwrap_err();
    Permissions::try_from(Variant::from("WRITE")).unwrap_err();
    // So does a `Variant` that borrows from the buffer it was decoded from, as one taken out of a
    // received message does.
    let encoded = to_bytes(Context::new(LE, 0), &Variant::from(flags)).unwrap();
    let (value, _): (Variant<'_>, _) = encoded.deserialize().unwrap();
    assert_eq!(Permissions::try_from(value).unwrap(), flags);

    let owned = OwnedVariant::try_from(flags).unwrap();
    assert_eq!(*owned, Variant::U32(0x3));
    assert_eq!(Permissions::try_from(owned).unwrap(), flags);
    let owned = OwnedVariant::try_from(Variant::from(0x1u32)).unwrap();
    assert_eq!(Permissions::try_from(owned).unwrap(), Permissions::READ);
}

#[test]
fn empty_flags_are_the_optional_null_value() {
    // The derived `Default` is the empty set, so that is what `Optional` maps to `None`.
    let opt = Optional::<Permissions>::try_from(Variant::from(0u32)).unwrap();
    assert_eq!(*opt, None);
    let opt = Optional::<Permissions>::try_from(Variant::from(0x2u32)).unwrap();
    assert_eq!(*opt, Some(Permissions::WRITE));

    let owned = OwnedVariant::try_from(Variant::from(0u32)).unwrap();
    let opt = Optional::<Permissions>::try_from(owned).unwrap();
    assert_eq!(*opt, None);
    let owned = OwnedVariant::try_from(Variant::from(0x3u32)).unwrap();
    let opt = Optional::<Permissions>::try_from(owned).unwrap();
    assert_eq!(*opt, Some(Permissions::READ | Permissions::WRITE));

    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &Optional::<Permissions>::from(None)).unwrap();
    assert_eq!(encoded.bytes(), &[0, 0, 0, 0]);
    let decoded: Optional<Permissions> = encoded.deserialize().unwrap().0;
    assert_eq!(*decoded, None);
}

#[test]
fn dictionary_key() {
    let mut dict = HashMap::new();
    dict.insert(Permissions::READ, "read-only");
    dict.insert(Permissions::READ | Permissions::WRITE, "read-write");

    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &dict).unwrap();
    let decoded: HashMap<Permissions, String> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded[&Permissions::READ], "read-only");
    assert_eq!(
        decoded[&(Permissions::READ | Permissions::WRITE)],
        "read-write"
    );

    let value = Variant::from(dict);
    assert_eq!(value.value_signature(), "a{us}");
    let decoded = HashMap::<Permissions, String>::try_from(value).unwrap();
    assert_eq!(decoded[&Permissions::READ], "read-only");
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    Type,
    Variant,
    OwnedVariant,
)]
struct Permissions(u32);

bitflags::bitflags! {
    impl Permissions: u32 {
        const READ = 0x1;
        const WRITE = 0x2;
    }
}

impl Basic for Permissions {
    const SIGNATURE_CHAR: char = u32::SIGNATURE_CHAR;
    const SIGNATURE_STR: &'static str = u32::SIGNATURE_STR;
}
