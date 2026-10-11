// Test through both generic and specific API (wrt byte order)
#[macro_export]
macro_rules! basic_type_test {
    ($endian:expr, $test_value:expr, $expected_len:expr, $expected_ty:ty, $align:literal) => {{
        // Lie that we're starting at byte 1 in the overall message to test padding
        let ctxt = zbus::wire::serialized::Context::new($endian, 1);
        let encoded = zbus::wire::to_bytes(ctxt, &$test_value).unwrap();
        let padding = zbus::wire::padding_for_n_bytes(1, $align);

        assert_eq!(
            encoded.len(),
            $expected_len + padding,
            "invalid encoding using `to_bytes`"
        );
        let (decoded, parsed): ($expected_ty, _) = encoded.deserialize().unwrap();
        assert!(decoded == $test_value, "invalid decoding");
        assert!(parsed == encoded.len(), "invalid parsing");

        // Now encode w/o padding
        let ctxt = zbus::wire::serialized::Context::new($endian, 0);
        let encoded = zbus::wire::to_bytes(ctxt, &$test_value).unwrap();
        assert_eq!(
            encoded.len(),
            $expected_len,
            "invalid encoding using `to_bytes`"
        );

        encoded
    }};
    (
        $endian:expr,
        $test_value:expr,
        $expected_len:expr,
        $expected_ty:ty,
        $align:literal,
        $kind:ident,
        $expected_value_len:expr
    ) => {{
        let encoded = basic_type_test!($endian, $test_value, $expected_len, $expected_ty, $align);

        // As Variant
        let v: zbus::Variant<'_> = $test_value.into();
        assert_eq!(
            v.value_signature(),
            <$expected_ty as zbus::Basic>::SIGNATURE_STR
        );
        assert_eq!(v, zbus::Variant::$kind($test_value));
        value_test!($endian, v, $expected_value_len);

        let v: $expected_ty = v.try_into().unwrap();
        assert_eq!(v, $test_value);

        encoded
    }};
}

#[macro_export]
macro_rules! value_test {
    ($endian:expr, $test_value:expr, $expected_len:expr) => {{
        let ctxt = zbus::wire::serialized::Context::new($endian, 0);
        let encoded = zbus::wire::to_bytes(ctxt, &$test_value).unwrap();
        assert_eq!(
            encoded.len(),
            $expected_len,
            "invalid encoding using `to_bytes`"
        );
        let (decoded, parsed): (zbus::Variant<'_>, _) = encoded.deserialize().unwrap();
        assert!(decoded == $test_value, "invalid decoding");
        assert!(parsed == encoded.len(), "invalid parsing");

        encoded
    }};
}

#[cfg(unix)]
#[macro_export]
macro_rules! fd_value_test {
    (
        $endian:expr,
        $test_value:expr,
        $expected_len:expr,
        $align:literal,
        $expected_value_len:expr
    ) => {{
        use std::os::fd::AsFd;

        // Lie that we're starting at byte 1 in the overall message to test padding
        let ctxt = zbus::wire::serialized::Context::new($endian, 1);
        let encoded = zbus::wire::to_bytes(ctxt, &$test_value).unwrap();
        let padding = zbus::wire::padding_for_n_bytes(1, $align);
        assert_eq!(
            encoded.len(),
            $expected_len + padding,
            "invalid encoding using `to_bytes`"
        );
        #[cfg(unix)]
        let (_, parsed): (zbus::Fd<'_>, _) = encoded.deserialize().unwrap();
        assert!(
            parsed == encoded.len(),
            "invalid parsing using `from_slice`"
        );

        // Now encode w/o padding
        let ctxt = zbus::wire::serialized::Context::new($endian, 0);
        let encoded = zbus::wire::to_bytes(ctxt, &$test_value).unwrap();
        assert_eq!(
            encoded.len(),
            $expected_len,
            "invalid encoding using `to_bytes`"
        );

        // As Variant
        let v: zbus::Variant<'_> = $test_value.into();
        assert_eq!(v.value_signature(), zbus::Fd::SIGNATURE_STR);
        assert_eq!(v, zbus::Variant::Fd($test_value));
        let encoded = zbus::wire::to_bytes(ctxt, &v).unwrap();
        assert_eq!(encoded.fds().len(), 1, "invalid encoding using `to_bytes`");
        assert_eq!(
            encoded.len(),
            $expected_value_len,
            "invalid encoding using `to_bytes`"
        );
        let (decoded, parsed): (zbus::Variant<'_>, _) = encoded.deserialize().unwrap();
        assert_eq!(
            decoded,
            zbus::Fd::from(encoded.fds()[0].as_fd()).into(),
            "invalid decoding using `from_slice`"
        );
        assert_eq!(parsed, encoded.len(), "invalid parsing using `from_slice`");

        let v: zbus::Fd<'_> = v.try_into().unwrap();
        assert_eq!(v, $test_value);
    }};
}
