use zbus::{
    Basic, Variant,
    wire::{LE, NATIVE_ENDIAN, serialized::Context, to_bytes},
};

#[test]
fn f64_value() {
    let encoded = f64_type_test(99999.99999_f64, 8, 16);
    assert!((NATIVE_ENDIAN.read_f64(&encoded) - 99999.99999_f64).abs() < f64::EPSILON);
}

fn f64_type_test(
    value: f64,
    expected_len: usize,
    expected_value_len: usize,
) -> zbus::wire::serialized::Data<'static, 'static> {
    // Lie that we're starting at byte 1 in the overall message to test padding
    let ctxt = Context::new(NATIVE_ENDIAN, 1);
    let encoded = to_bytes(ctxt, &value).unwrap();
    let padding = zbus::wire::padding_for_n_bytes(1, 8);
    assert_eq!(
        encoded.len(),
        expected_len + padding,
        "invalid encoding using `to_bytes`"
    );

    let decoded: f64 = encoded.deserialize().unwrap().0;
    assert!(
        (decoded - value).abs() < f64::EPSILON,
        "invalid decoding using `from_slice`"
    );

    // Now encode w/o padding
    let ctxt = Context::new(NATIVE_ENDIAN, 0);
    let encoded = to_bytes(ctxt, &value).unwrap();
    assert_eq!(
        encoded.len(),
        expected_len,
        "invalid encoding using `to_bytes`"
    );

    f64_type_test_as_value(value, expected_value_len);
    encoded
}

#[allow(dead_code)]
pub fn f64_type_test_as_value(value: f64, expected_value_len: usize) {
    let v: Variant<'_> = value.into();
    assert_eq!(v.value_signature(), f64::SIGNATURE_STR);
    assert_eq!(v, Variant::F64(value));
    f64_value_test(v.try_clone().unwrap(), expected_value_len);
    let v: f64 = v.try_into().unwrap();
    assert!((v - value).abs() < f64::EPSILON);
}

#[allow(dead_code)]
pub fn f64_value_test(v: Variant<'_>, expected_value_len: usize) {
    let ctxt = Context::new(LE, 0);
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(
        encoded.len(),
        expected_value_len,
        "invalid encoding using `to_bytes`"
    );
    let decoded: Variant<'_> = encoded.deserialize().unwrap().0;
    assert!(decoded == v, "invalid decoding using `from_slice`");
}
