use std::collections::HashMap;
use zbus::{Variant, wire::serialized::Context};

#[test]
fn struct_byte_array() {
    let ctxt = Context::new(zbus::wire::LE, 0);
    let value: (Vec<u8>, HashMap<String, Variant<'_>>) = (Vec::new(), HashMap::new());
    let value = zbus::wire::to_bytes(ctxt, &value).unwrap();
    #[cfg(feature = "serde_bytes")]
    let (bytes, map): (&serde_bytes::Bytes, HashMap<&str, Variant<'_>>) = value
        .deserialize()
        .expect("Could not deserialize serde_bytes::Bytes in struct.")
        .0;
    #[cfg(not(feature = "serde_bytes"))]
    let (bytes, map): (&[u8], HashMap<&str, Variant<'_>>) = value
        .deserialize()
        .expect("Could not deserialize u8 slice in struct")
        .0;

    assert!(bytes.is_empty());
    assert!(map.is_empty());
}
