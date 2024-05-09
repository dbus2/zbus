use endi::Endian;
use serde::{Deserialize, Serialize};
use zvariant::{
    serialized::{Context, Data},
    Type,
};

fn generate_contexts() -> Vec<Context> {
    vec![
        Context::new_dbus(Endian::Little, 0),
        Context::new_dbus(Endian::Big, 0),
        #[cfg(feature = "gvariant")]
        Context::new_gvariant(Endian::Little, 0),
        #[cfg(feature = "gvariant")]
        Context::new_gvariant(Endian::Big, 0),
    ]
}

#[test]
fn serde_i8() {
    for context in generate_contexts() {
        let value: i8 = 42;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (i8, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_i16() {
    for context in generate_contexts() {
        let value: i16 = 42;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (i16, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_i32() {
    for context in generate_contexts() {
        let value: i32 = 42;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (i32, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_i64() {
    for context in generate_contexts() {
        let value: i64 = 42;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (i64, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_u8() {
    for context in generate_contexts() {
        let value: u8 = 42;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (u8, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_u16() {
    for context in generate_contexts() {
        let value: u16 = 42;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (u16, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_u32() {
    for context in generate_contexts() {
        let value: u32 = 42;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (u32, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_u64() {
    for context in generate_contexts() {
        let value: u64 = 42;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (u64, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_f32() {
    for context in generate_contexts() {
        let value: f32 = 42.0;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (f32, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_f64() {
    for context in generate_contexts() {
        let value: f64 = 42.0;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (f64, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_bool() {
    for context in generate_contexts() {
        let value: bool = true;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (bool, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_char() {
    for context in generate_contexts() {
        let value: char = 'a';
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (char, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_string() {
    for context in generate_contexts() {
        let value: &str = "Hello, world!";
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (&str, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_unit() {
    for context in generate_contexts() {
        let value: () = ();
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): ((), usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_unit_struct() {
    for context in generate_contexts() {
        #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
        struct UnitStruct;
        let value: UnitStruct = UnitStruct;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (UnitStruct, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_unit_variant() {
    for context in generate_contexts() {
        #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
        enum UnitVariant {
            A,
            B,
        }
        let value: UnitVariant = UnitVariant::A;
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (UnitVariant, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_newtype_struct() {
    for context in generate_contexts() {
        #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
        struct NewtypeStruct(i32);
        let value: NewtypeStruct = NewtypeStruct(42);
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (NewtypeStruct, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_newtype_variant() {
    for context in generate_contexts() {
        #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
        enum NewtypeVariant {
            A(i32),
            B(i32),
        }
        let value: NewtypeVariant = NewtypeVariant::A(42);
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (NewtypeVariant, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_seq() {
    for context in generate_contexts() {
        let value: Vec<i32> = vec![1, 2, 3];
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (Vec<i32>, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_tuple() {
    for context in generate_contexts() {
        let value: (i32, i32, i32) = (1, 2, 3);
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): ((i32, i32, i32), usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_tuple_struct() {
    for context in generate_contexts() {
        #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
        struct TupleStruct(i32, i32, i32);
        let value: TupleStruct = TupleStruct(1, 2, 3);
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (TupleStruct, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_tuple_variant() {
    for context in generate_contexts() {
        #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
        enum TupleVariant {
            A(i32, i32, i32),
            B(i32, i32, i32),
        }
        let value: TupleVariant = TupleVariant::A(1, 2, 3);
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (TupleVariant, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_map() {
    for context in generate_contexts() {
        use std::collections::HashMap;
        let mut value = HashMap::new();
        value.insert("a", 1);
        value.insert("b", 2);
        value.insert("c", 3);
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (HashMap<&str, i32>, usize) =
            serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_struct() {
    for context in generate_contexts() {
        #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
        struct Struct {
            a: i32,
            b: i32,
            c: i32,
        }
        let value: Struct = Struct { a: 1, b: 2, c: 3 };
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (Struct, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}

#[test]
fn serde_struct_variant() {
    for context in generate_contexts() {
        #[derive(Serialize, Deserialize, Type, Debug, PartialEq)]
        enum StructVariant {
            A { a: i32, b: i32, c: i32 },
            B { a: i32, b: i32, c: i32 },
        }
        let value: StructVariant = StructVariant::A { a: 1, b: 2, c: 3 };
        let serialized: Data<'_, '_> = zvariant::to_bytes(context, &value).unwrap();
        let (deserialized, decoded): (StructVariant, usize) = serialized.deserialize().unwrap();
        assert_eq!(value, deserialized);
        assert_eq!(decoded, serialized.len());
    }
}
