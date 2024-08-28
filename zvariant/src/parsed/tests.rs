use super::{signature::*, *};
use std::str::FromStr;

macro_rules! validate {
        ($($signature:literal => $expected:expr),+) => {
            $(
                assert!(validate($signature.as_bytes()).is_ok());
                let parsed = Signature::from_str($signature).unwrap();
                assert_eq!(parsed, $expected);
                assert_eq!(parsed, $signature);
            )+
        };
    }

#[test]
fn validate_strings() {
    validate!(
        "" => Signature::Unit,
        "y" => Signature::U8,
        "b" => Signature::Bool,
        "n" => Signature::I16,
        "q" => Signature::U16,
        "i" => Signature::I32,
        "u" => Signature::U32,
        "x" => Signature::I64,
        "t" => Signature::U64,
        "d" => Signature::F64,
        "s" => Signature::Str,
        "g" => Signature::Signature,
        "o" => Signature::ObjectPath,
        "v" => Signature::Variant,
        "xs" => Signature::structure(&[&Signature::I64, &Signature::Str][..]),
        "(ysa{sd})" => Signature::static_structure(
            &[
                &Signature::U8,
                &Signature::Str,
                &Signature::Dict {
                    key: ChildSignature::Static { child: &Signature::Str },
                    value: ChildSignature::Static { child: &Signature::F64 },
                },
            ]
        ),
        "a(y)" => Signature::static_array(
            &Signature::Structure(FieldsSignatures::Static { fields: &[&Signature::U8] }),
        ),
        "a{yy}" => Signature::static_dict(&Signature::U8, &Signature::U8),
        "(yy)" => Signature::static_structure(&[&Signature::U8, &Signature::U8]),
        "a{sd}" => Signature::static_dict(&Signature::Str, &Signature::F64),
        "a{yy}" => Signature::static_dict(&Signature::U8, &Signature::U8),
        "a{sv}" => Signature::static_dict(&Signature::Str, &Signature::Variant),
        "a{sa{sv}}" => Signature::static_dict(
            &Signature::Str,
            &Signature::Dict {
                key: ChildSignature::Static {
                child: &Signature::Str,
                },
                value: ChildSignature::Static {
                child: &Signature::Variant
                }
            },
        ),
        "a{sa(ux)}" => Signature::static_dict(
            &Signature::Str,
            &Signature::Array(ChildSignature::Static {
                child: &Signature::Structure(FieldsSignatures::Static {
                    fields: &[&Signature::U32, &Signature::I64]
                }),
            }),
        ),
        "(x)" => Signature::static_structure(&[&Signature::I64]),
        "(x(isy))" => Signature::static_structure(&[
            &Signature::I64,
            &Signature::Structure(FieldsSignatures::Static {
                fields: &[&Signature::I32, &Signature::Str, &Signature::U8]
            }),
        ]),
        "(xa(isy))" => Signature::static_structure(&[
            &Signature::I64,
            &Signature::Array(ChildSignature::Static {
                child: &Signature::Structure(FieldsSignatures::Static {
                    fields: &[&Signature::I32, &Signature::Str, &Signature::U8]
                }),
            }),
        ]),
        "(xa(s))" => Signature::static_structure(&[
            &Signature::I64,
            &Signature::Array(ChildSignature::Static {
                child: &Signature::Structure(FieldsSignatures::Static {
                    fields: &[&Signature::Str]
                }),
            }),
        ]),
        "((yyyyuu)a(yv))" => Signature::static_structure(&[
            &Signature::Structure(FieldsSignatures::Static {
                fields: &[
                    &Signature::U8, &Signature::U8, &Signature::U8, &Signature::U8,
                    &Signature::U32, &Signature::U32,
                ],
            }),
            &Signature::Array(ChildSignature::Static {
                child: &Signature::Structure(FieldsSignatures::Static {
                    fields: &[&Signature::U8, &Signature::Variant],
                }),
            }),
        ])
    );
    #[cfg(unix)]
    validate!("h" => Signature::Fd);
}

macro_rules! invalidate {
        ($($signature:literal),+) => {
            $(
                assert!(validate($signature.as_bytes()).is_err());
            )+
        };
    }

#[test]
fn invalid_strings() {
    invalidate!(
        "a",
        "a{}",
        "a{y",
        "a{y}",
        "a{y}a{y}",
        "a{y}a{y}a{y}",
        "z",
        "()",
        "(x",
        "(x())",
        "(xa()",
        "(xa(s)",
        "(xs",
        "xs)",
        "s/",
        "a{yz}"
    );
}
