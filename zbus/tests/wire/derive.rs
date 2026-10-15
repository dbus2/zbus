use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::{
    OwnedVariant, Type, Variant,
    wire::{Error, LE, serialized::Context, to_bytes},
};

#[test]
fn derive() {
    #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
    struct Struct<'s> {
        field1: u16,
        field2: i64,
        field3: &'s str,
    }

    assert_eq!(Struct::SIGNATURE, "(qxs)");
    let s = Struct {
        field1: 0xFF_FF,
        field2: 0xFF_FF_FF_FF_FF_FF,
        field3: "hello",
    };
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &s).unwrap();
    assert_eq!(encoded.len(), 26);
    let decoded: Struct<'_> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, s);

    #[derive(Deserialize, Serialize, Type)]
    struct UnitStruct;

    assert_eq!(UnitStruct::SIGNATURE, <()>::SIGNATURE);
    let encoded = to_bytes(ctxt, &UnitStruct).unwrap();
    assert_eq!(encoded.len(), 0);
    let _: UnitStruct = encoded.deserialize().unwrap().0;

    #[repr(u8)]
    #[derive(Deserialize_repr, Serialize_repr, Type, Variant, OwnedVariant, Debug, PartialEq)]
    enum Enum {
        Variant1,
        Variant2,
        Variant3,
    }

    assert_eq!(Enum::SIGNATURE, u8::SIGNATURE);
    let encoded = to_bytes(ctxt, &Enum::Variant3).unwrap();
    assert_eq!(encoded.len(), 1);
    let decoded: Enum = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, Enum::Variant3);

    assert_eq!(Variant::from(Enum::Variant1), Variant::U8(0));
    assert_eq!(Enum::try_from(Variant::U8(2)), Ok(Enum::Variant3));
    assert_eq!(Enum::try_from(Variant::U8(4)), Err(Error::IncorrectType));

    #[repr(i64)]
    #[derive(Deserialize_repr, Serialize_repr, Type, Variant, OwnedVariant, Debug, PartialEq)]
    enum Enum2 {
        Variant1,
        Variant2,
        Variant3,
    }

    assert_eq!(Enum2::SIGNATURE, i64::SIGNATURE);
    let encoded = to_bytes(ctxt, &Enum2::Variant2).unwrap();
    assert_eq!(encoded.len(), 8);
    let decoded: Enum2 = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, Enum2::Variant2);

    assert_eq!(Variant::from(Enum2::Variant1), Variant::I64(0));
    assert_eq!(Enum2::try_from(Variant::I64(2)), Ok(Enum2::Variant3));
    assert_eq!(Enum2::try_from(Variant::I64(4)), Err(Error::IncorrectType));

    #[derive(Deserialize, Serialize, Type, Variant, OwnedVariant, Debug, PartialEq)]
    enum NoReprEnum {
        Variant1,
        Variant2,
        Variant3,
    }

    // issue#265: Panic on deserialization of a structure w/ a unit enum as its last field.
    let encoded = to_bytes(ctxt, &(NoReprEnum::Variant2,)).unwrap();
    let _: (NoReprEnum,) = encoded.deserialize().unwrap().0;

    assert_eq!(NoReprEnum::SIGNATURE, u32::SIGNATURE);
    let encoded = to_bytes(ctxt, &NoReprEnum::Variant2).unwrap();
    assert_eq!(encoded.len(), 4);
    let decoded: NoReprEnum = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, NoReprEnum::Variant2);

    #[derive(Deserialize, Serialize, Type, Variant, OwnedVariant, Debug, PartialEq)]
    #[zbus(signature = "s", rename_all = "snake_case")]
    enum StrEnum {
        VariantOne,
        Variant2,
        Variant3,
    }

    assert_eq!(StrEnum::SIGNATURE, <&str>::SIGNATURE);
    let encoded = to_bytes(ctxt, &StrEnum::Variant2).unwrap();
    assert_eq!(encoded.len(), 13);
    let decoded: StrEnum = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, StrEnum::Variant2);

    assert_eq!(
        StrEnum::try_from(Variant::Str("variant_one".into())),
        Ok(StrEnum::VariantOne)
    );
    assert_eq!(
        StrEnum::try_from(Variant::Str("variant2".into())),
        Ok(StrEnum::Variant2)
    );
    assert_eq!(
        StrEnum::try_from(Variant::Str("variant4".into())),
        Err(Error::IncorrectType)
    );
    assert_eq!(
        StrEnum::try_from(Variant::U32(0)),
        Err(Error::IncorrectType)
    );

    #[derive(Deserialize, Serialize, Type)]
    enum NewType {
        Variant1(f64),
        Variant2(f64),
    }
    assert_eq!(NewType::SIGNATURE, "(ud)");

    #[derive(Deserialize, Serialize, Type)]
    enum StructFields {
        Variant1(u16, i64, &'static str),
        Variant2 {
            field1: u16,
            field2: i64,
            field3: &'static str,
        },
    }
    assert_eq!(StructFields::SIGNATURE, "(u(qxs))");

    #[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
    struct AStruct<'s> {
        field1: u16,
        field2: &'s [u8],
        field3: &'s [u8],
        field4: i64,
    }
    assert_eq!(AStruct::SIGNATURE, "(qayayx)");
    let s = AStruct {
        field1: 0xFF_FF,
        field2: &[77u8; 8],
        field3: &[77u8; 8],
        field4: 0xFF_FF_FF_FF_FF_FF,
    };
    let encoded = to_bytes(ctxt, &s).unwrap();
    assert_eq!(encoded.len(), 40);
    let decoded: AStruct<'_> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, s);
}

#[test]
fn newtype_converts_from_and_to_value_of_any_lifetime() {
    // A newtype without a lifetime of its own, such as a set of flags over the integer that holds
    // them.
    #[derive(
        Deserialize, Serialize, Type, Variant, OwnedVariant, Debug, PartialEq, Clone, Copy,
    )]
    struct Flags(u32);

    fn from_value(value: Variant<'_>) -> Result<Flags, Error> {
        Flags::try_from(value)
    }

    fn into_value<'v>(flags: Flags) -> Variant<'v> {
        flags.into()
    }

    // A `Variant` that borrows from the buffer it was decoded from, as one taken out of a received
    // message does.
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &Variant::from(0x5u32)).unwrap();
    let (value, _): (Variant<'_>, _) = encoded.deserialize().unwrap();
    assert_eq!(from_value(value).unwrap(), Flags(0x5));
    assert_eq!(into_value(Flags(0x5)), Variant::U32(0x5));

    // The `'static` case keeps working.
    let value: Variant<'static> = Flags(0x2).into();
    assert_eq!(Flags::try_from(value).unwrap(), Flags(0x2));
}
