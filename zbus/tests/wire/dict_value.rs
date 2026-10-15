use std::collections::{BTreeMap, HashMap};

use endi::NATIVE_ENDIAN;
use zbus::{
    DeserializeDict, Dict, OwnedObjectPath, SerializeDict, Str, Type, Variant,
    as_variant::{Serialize, optional},
    wire::{LE, serialized::Context, to_bytes},
};

#[test]
fn dict_value() {
    let mut map: HashMap<i64, &str> = HashMap::new();
    map.insert(1, "123");
    map.insert(2, "456");
    let ctxt = Context::new(NATIVE_ENDIAN, 0);
    let encoded = to_bytes(ctxt, &map).unwrap();
    assert_eq!(dbg!(encoded.len()), 40);
    let decoded: HashMap<i64, &str> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded[&1], "123");
    assert_eq!(decoded[&2], "456");

    // As Variant
    let v: Variant<'_> = Dict::from(map).into();
    assert_eq!(v.value_signature(), "a{xs}");
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(encoded.len(), 48);
    // Convert it back
    let dict: Dict<'_, '_> = v.try_into().unwrap();
    let map: HashMap<i64, String> = dict.try_clone().unwrap().try_into().unwrap();
    assert_eq!(map[&1], "123");
    assert_eq!(map[&2], "456");
    // Also decode it back
    let v = encoded.deserialize().unwrap().0;
    if let Variant::Dict(dict) = v {
        assert_eq!(dict.get::<i64, &str>(&1).unwrap().unwrap(), "123");
        assert_eq!(dict.get::<i64, &str>(&2).unwrap().unwrap(), "456");
    } else {
        panic!();
    }
    // Convert it to a BTreeMap too.
    let map: BTreeMap<i64, String> = dict.try_into().unwrap();
    assert_eq!(map[&1], "123");
    assert_eq!(map[&2], "456");
    // Use iterator
    let mut dict = Dict::from(map);
    let expect = [
        (Variant::from(1i64), Variant::from("123")),
        (Variant::from(2i64), Variant::from("456")),
    ];
    let expect_iter = expect.iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
    let actual = dict.iter().collect::<Vec<_>>();
    assert_eq!(actual, expect_iter);
    let actual = dict.iter().collect::<Vec<_>>();
    assert_eq!(actual, expect_iter);
    let actual = dict.iter().collect::<Vec<_>>();
    assert_eq!(actual, expect_iter);
    for (_, v) in dict.iter_mut() {
        if let Variant::Str(vv) = v {
            *vv = Str::from(vv.to_string() + "-hello");
        }
    }
    let actual = dict.into_iter().collect::<Vec<_>>();
    let expect = vec![
        (Variant::from(1i64), Variant::from("123-hello")),
        (Variant::from(2i64), Variant::from("456-hello")),
    ];
    assert_eq!(actual, expect);

    // `as_variant::Serialize` must produce the same encoding as wrapping the value in `Variant`
    // directly (https://github.com/z-galaxy/zbus/issues/868).
    let mut map = HashMap::<&str, &str>::new();
    map.insert("k", "v");
    let ser_value_encoded = to_bytes(ctxt, &Serialize(&map)).unwrap();
    let value_encoded = to_bytes(ctxt, &Variant::new(map)).unwrap();
    assert_eq!(*value_encoded, *ser_value_encoded);

    // Dict<u32, u8>
    let mut map: HashMap<u32, u8> = HashMap::new();
    map.insert(1, 2);
    // The byte order is explicit since the encoding is compared against hardcoded bytes.
    let encoded = to_bytes(Context::new(LE, 0), &map).unwrap();
    assert_eq!(
        encoded.bytes(),
        [
            0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02
        ]
    );
    let decoded: HashMap<u32, u8> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, map);

    // Now a hand-crafted Dict Variant but with a Variant as value
    let mut dict = Dict::new(<&str>::SIGNATURE, Variant::SIGNATURE);
    dict.add("hello", Variant::new("there")).unwrap();
    dict.add("bye", Variant::new("now")).unwrap();
    let v: Variant<'_> = dict.into();
    assert_eq!(v.value_signature(), "a{sv}");
    let encoded = to_bytes(ctxt, &v).unwrap();
    assert_eq!(dbg!(encoded.len()), 66);
    let v: Variant<'_> = encoded.deserialize().unwrap().0;
    if let Variant::Dict(dict) = v {
        assert_eq!(
            dict.get::<&str, Variant<'_>>(&"hello").unwrap().unwrap(),
            Variant::new("there")
        );
        assert_eq!(
            dict.get::<_, Variant<'_>>(&"bye").unwrap().unwrap(),
            Variant::new("now")
        );

        // Try converting to a HashMap
        let map = <HashMap<String, Variant<'_>>>::try_from(dict.try_clone().unwrap()).unwrap();
        assert_eq!(map["hello"], Variant::new("there"));
        assert_eq!(map["bye"], Variant::new("now"));

        // Try converting to a BTreeMap
        let map = <BTreeMap<String, Variant<'_>>>::try_from(dict).unwrap();
        assert_eq!(map["hello"], Variant::new("there"));
        assert_eq!(map["bye"], Variant::new("now"));
    } else {
        panic!();
    }

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}")]
    struct Test {
        process_id: Option<u32>,
        group_id: Option<u32>,
        user: String,
    }

    let test = Test {
        process_id: Some(42),
        group_id: None,
        user: "me".to_string(),
    };

    let encoded = to_bytes(ctxt, &test).unwrap();
    assert_eq!(encoded.len(), 51);

    let decoded: HashMap<&str, Variant<'_>> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded["process_id"], Variant::U32(42));
    assert_eq!(decoded["user"], Variant::new("me"));
    assert!(!decoded.contains_key("group_id"));

    let decoded: Test = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, test);

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug)]
    #[zbus(signature = "a{sv}")]
    struct TestMissing {
        process_id: Option<u32>,
        group_id: Option<u32>,
        user: String,
        quota: u8,
    }
    let decoded = encoded.deserialize::<TestMissing>();
    assert!(decoded.is_err());

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}")]
    struct TestSkipUnknown {
        process_id: Option<u32>,
        group_id: Option<u32>,
    }
    let _: TestSkipUnknown = encoded.deserialize().unwrap().0;

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}", deny_unknown_fields)]
    struct TestDenyUnknown {
        process_id: Option<u32>,
        group_id: Option<u32>,
    }
    let decoded = encoded.deserialize::<TestDenyUnknown>();
    assert!(decoded.is_err());

    #[derive(serde::Serialize, serde::Deserialize, Type, PartialEq, Debug, Default)]
    #[serde(default)]
    #[zbus(signature = "a{sv}")]
    struct TestParseUnknown<'s> {
        #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
        process_id: Option<u32>,
        #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
        group_id: Option<u32>,
        #[serde(flatten, borrow)]
        rest: HashMap<&'s str, Variant<'s>>,
    }
    let decoded: TestParseUnknown<'_> = encoded.deserialize().unwrap().0;
    assert_eq!(decoded.rest.len(), 1);
    assert_eq!(decoded.rest["user"], Variant::new("me"));

    #[derive(Default, Debug, SerializeDict, DeserializeDict, Type)]
    #[zbus(signature = "dict")]
    struct TestEmpty {}

    // Empty dict should be serialized to contain 4 bytes for the ARRAY length and 4 bytes for the
    // DICT_ENTRY alignment to 8 bytes boundary.
    let data = to_bytes(ctxt, &TestEmpty::default()).unwrap();

    assert_eq!(data.bytes(), &[0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn struct_with_object_path_keys() {
    let ctxt = Context::new(NATIVE_ENDIAN, 0);

    #[derive(serde::Deserialize, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{ou}")]
    #[serde(default)]
    struct Mapped {
        #[serde(rename = "/foo")]
        foo: u32,
        #[serde(rename = "/bar")]
        bar: u32,
    }

    let mut wire: HashMap<OwnedObjectPath, u32> = HashMap::new();
    wire.insert(OwnedObjectPath::try_from("/foo").unwrap(), 1);
    wire.insert(OwnedObjectPath::try_from("/bar").unwrap(), 2);

    let encoded = to_bytes(ctxt, &wire).unwrap();
    let decoded: Mapped = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, Mapped { foo: 1, bar: 2 });
}

#[test]
fn nested_dict_value() {
    let ctxt = Context::new(NATIVE_ENDIAN, 0);

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default, Clone)]
    #[zbus(signature = "a{sv}", rename_all = "PascalCase")]
    struct Adapter {
        address: Option<String>,
        name: Option<String>,
        powered: bool,
    }

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default, Clone)]
    #[zbus(signature = "a{sv}", rename_all = "PascalCase")]
    struct Media {
        supported_features: Vec<String>,
        #[zbus(rename = "SupportedUUIDs")]
        supported_uuids: Vec<String>,
    }

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sa{sv}}")]
    struct Interfaces {
        #[zbus(rename = "org.bluez.Adapter1")]
        adapter: Option<Adapter>,
        #[zbus(rename = "org.bluez.Media1")]
        media: Media,
    }

    let media = Media {
        // The UUIDs aren't value but it doens't matter for this test.
        supported_uuids: vec!["001-233-22333".to_string(), "222-222-222".to_string()],
        supported_features: vec!["tx-timestamping".to_string()],
    };
    let interfaces = Interfaces {
        adapter: Some(Adapter {
            address: Some("00:11:22:33:44:55".to_string()),
            name: Some("test-adapter".to_string()),
            powered: true,
        }),
        media: media.clone(),
    };

    let encoded = to_bytes(ctxt, &interfaces).unwrap();
    let decoded: Interfaces = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, interfaces);

    let mut outer: HashMap<&str, HashMap<&str, Variant<'_>>> = HashMap::new();
    let mut adapter_props: HashMap<&str, Variant<'_>> = HashMap::new();
    adapter_props.insert("Address", Variant::new("00:11:22:33:44:55"));
    adapter_props.insert("Name", Variant::new("test-adapter"));
    adapter_props.insert("Powered", Variant::new(true));
    outer.insert("org.bluez.Adapter1", adapter_props);
    let mut media_props: HashMap<&str, Variant<'_>> = HashMap::new();
    media_props.insert("SupportedFeatures", Variant::new(vec!["tx-timestamping"]));
    media_props.insert(
        "SupportedUUIDs",
        Variant::new(vec!["001-233-22333", "222-222-222"]),
    );
    outer.insert("org.bluez.Media1", media_props);

    let map_encoded = to_bytes(ctxt, &outer).unwrap();
    let from_map: Interfaces = map_encoded.deserialize().unwrap().0;
    assert_eq!(from_map, interfaces);

    outer.remove("org.bluez.Adapter1").unwrap();
    let encoded = to_bytes(ctxt, &outer).unwrap();
    let decoded: Interfaces = encoded.deserialize().unwrap().0;
    assert_eq!(
        decoded,
        Interfaces {
            adapter: None,
            media,
        }
    );

    // A different shape of nesting (https://github.com/z-galaxy/zbus/issues/1819): an outer
    // `a{sv}` (so the value type is `v`) where one variant value is itself an `a{sv}`. Unlike
    // `Interfaces` above, the outer value type here is `v`, so each field is wrapped in a
    // variant on the wire.
    use zbus::OwnedVariant;

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}", rename_all = "kebab-case")]
    struct InspectBundleUpdateInfo {
        compatible: Option<bool>,
        version: Option<String>,
    }

    // The reporter's case: the nested `a{sv}` is modelled as a typed `*Dict` struct field.
    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}", rename_all = "kebab-case")]
    struct InspectBundleInfo {
        manifest_hash: Option<String>,
        update: Option<InspectBundleUpdateInfo>,
    }

    // The same, but capturing the nested variant verbatim as an `OwnedVariant` field.
    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}", rename_all = "kebab-case")]
    struct InspectBundleInfoRaw {
        manifest_hash: Option<String>,
        update: Option<OwnedVariant>,
    }

    let info = InspectBundleInfo {
        manifest_hash: Some("abc".to_string()),
        update: Some(InspectBundleUpdateInfo {
            compatible: Some(true),
            version: Some("1.0".to_string()),
        }),
    };

    // Typed-struct field: round-trips through the derives.
    let encoded = to_bytes(ctxt, &info).unwrap();
    let decoded: InspectBundleInfo = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, info);

    // And it decodes from data produced the way a real D-Bus peer would send it: an `a{sv}`
    // whose `update` key holds a variant containing another `a{sv}`.
    let mut update_props: HashMap<&str, Variant<'_>> = HashMap::new();
    update_props.insert("compatible", Variant::new(true));
    update_props.insert("version", Variant::new("1.0"));
    let mut info_props: HashMap<&str, Variant<'_>> = HashMap::new();
    info_props.insert("manifest-hash", Variant::new("abc"));
    info_props.insert("update", Variant::Dict(Dict::from(update_props)));
    let map_encoded = to_bytes(ctxt, &info_props).unwrap();
    let from_map: InspectBundleInfo = map_encoded.deserialize().unwrap().0;
    assert_eq!(from_map, info);

    // `OwnedVariant` field: must capture the nested `a{sv}` variant verbatim, without an extra
    // variant-of-variant layer (the bug fixed in #1819).
    let raw: InspectBundleInfoRaw = map_encoded.deserialize().unwrap().0;
    assert_eq!(raw.manifest_hash.as_deref(), Some("abc"));
    let update = raw.update.as_ref().unwrap();
    assert_eq!(update.value_signature(), "a{sv}");
    let update_dict = update.downcast_ref::<&Dict<'_, '_>>().unwrap();
    assert!(
        update_dict
            .get::<&str, bool>(&"compatible")
            .unwrap()
            .unwrap()
    );

    // Re-serializing the `OwnedVariant` field must not double-wrap it: the bytes still read back
    // both as the same raw struct and as the typed-struct view.
    let reencoded = to_bytes(ctxt, &raw).unwrap();
    let raw_again: InspectBundleInfoRaw = reencoded.deserialize().unwrap().0;
    assert_eq!(raw_again, raw);
    let typed_again: InspectBundleInfo = reencoded.deserialize().unwrap().0;
    assert_eq!(typed_again, info);
}

#[test]
fn nested_dict_object_path_keys() {
    let ctxt = Context::new(NATIVE_ENDIAN, 0);

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{sv}", rename_all = "PascalCase")]
    struct Props {
        value: Option<u32>,
    }

    #[derive(DeserializeDict, SerializeDict, Type, PartialEq, Debug, Default)]
    #[zbus(signature = "a{oa{sv}}")]
    struct ByPath {
        #[zbus(rename = "/org/example/A")]
        a: Option<Props>,
        #[zbus(rename = "/org/example/B")]
        b: Option<Props>,
    }

    let by_path = ByPath {
        a: Some(Props { value: Some(7) }),
        b: None,
    };
    let encoded = to_bytes(ctxt, &by_path).unwrap();
    let decoded: ByPath = encoded.deserialize().unwrap().0;
    assert_eq!(decoded, by_path);

    let mut props: HashMap<&str, Variant<'_>> = HashMap::new();
    props.insert("Value", Variant::new(7u32));
    let mut outer: HashMap<OwnedObjectPath, HashMap<&str, Variant<'_>>> = HashMap::new();
    outer.insert(OwnedObjectPath::try_from("/org/example/A").unwrap(), props);
    let map_encoded = to_bytes(ctxt, &outer).unwrap();
    let from_map: ByPath = map_encoded.deserialize().unwrap().0;
    assert_eq!(from_map, by_path);
}
