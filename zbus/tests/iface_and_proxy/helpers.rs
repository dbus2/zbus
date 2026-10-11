use std::collections::HashMap;
use zbus::{OwnedVariant, Variant};

use super::types::IP4Adress;

pub fn check_hash_map(map: HashMap<String, String>) {
    assert_eq!(map["hi"], "hello");
    assert_eq!(map["bye"], "now");
}

pub fn check_ipv4_address(address: IP4Adress) {
    assert_eq!(
        address,
        IP4Adress {
            address: "127.0.0.1".to_string(),
            prefix: 1234,
        }
    );
}

pub fn check_ipv4_address_hashmap(address: HashMap<String, OwnedVariant>) {
    assert_eq!(
        **address.get("address").unwrap(),
        Variant::from("127.0.0.1")
    );
    assert_eq!(**address.get("prefix").unwrap(), Variant::from(1234u32));
}
