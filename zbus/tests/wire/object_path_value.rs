use zbus::{ObjectPath, Variant, wire::LE};

#[test]
fn object_path_value() {
    let o = ObjectPath::try_from("/hello/world").unwrap();
    basic_type_test!(LE, o, 17, ObjectPath<'_>, 4);

    // As Variant
    let v: Variant<'_> = o.into();
    assert_eq!(v.value_signature(), "o");
    let encoded = value_test!(LE, v, 21);
    let v = encoded.deserialize::<Variant<'_>>().unwrap().0;
    assert_eq!(
        v,
        Variant::ObjectPath(ObjectPath::try_from("/hello/world").unwrap())
    );
}
