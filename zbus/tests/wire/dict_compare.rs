use zbus::{Dict, Type, Variant};

#[test]
fn dict_compare() {
    // the order in which a dict has been constructed must not play a role
    // https://github.com/z-galaxy/zbus/issues/484
    let mut dict1 = Dict::new(<&str>::SIGNATURE, Variant::SIGNATURE);
    dict1.add("first", Variant::new("value")).unwrap();
    dict1.add("second", Variant::new("value")).unwrap();

    let mut dict2 = Dict::new(<&str>::SIGNATURE, Variant::SIGNATURE);
    dict2.add("second", Variant::new("value")).unwrap();
    dict2.add("first", Variant::new("value")).unwrap();

    assert_eq!(dict1, dict2);
}
