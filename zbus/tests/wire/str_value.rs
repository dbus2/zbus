use zbus::{
    Variant,
    wire::{LE, serialized::Context, to_bytes},
};

#[test]
fn str_value() {
    let string = String::from("hello world");
    basic_type_test!(LE, string, 16, String, 4);
    basic_type_test!(LE, string, 16, &str, 4);

    let string = "hello world";
    basic_type_test!(LE, string, 16, &str, 4);
    basic_type_test!(LE, string, 16, String, 4);

    // As Variant
    let v: Variant<'_> = string.into();
    assert_eq!(v.value_signature(), "s");
    assert_eq!(v, Variant::new("hello world"));
    value_test!(LE, v, 20);

    let v: String = v.try_into().unwrap();
    assert_eq!(v, "hello world");

    // Characters are treated as strings
    basic_type_test!(LE, 'c', 6, char, 4);

    // As Variant
    let v: Variant<'_> = "c".into();
    assert_eq!(v.value_signature(), "s");
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 10);
    let (v, _) = encoded.deserialize::<Variant<'_>>().unwrap();
    assert_eq!(v, Variant::new("c"));
}
