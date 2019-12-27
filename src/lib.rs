mod address;
pub use address::*;

mod basic;
pub use basic::*;

mod message;
pub use message::*;

mod message_field;
pub use message_field::*;

mod message_fields;
pub use message_fields::*;

mod connection;
pub use connection::*;

mod variant;
pub use variant::*;

mod decode;
pub use decode::*;

mod encode;
pub use encode::*;

mod variant_error;
pub use variant_error::*;

mod str;
pub use crate::str::*;

mod signature;
pub use crate::signature::*;

mod object_path;
pub use crate::object_path::*;

mod simple_decode;
pub use simple_decode::*;

mod structure;
pub use structure::*;

mod array;
pub use array::*;

mod dict_entry;
pub use dict_entry::*;

mod dict;
pub use dict::*;

mod shared_data;
pub use shared_data::*;

mod utils;

#[cfg(test)]
mod tests {
    use core::convert::{TryFrom, TryInto};
    use std::collections::HashMap;

    use crate::{Array, Decode, Dict, Encode};
    use crate::{Structure, Variant};

    #[test]
    fn basic_connection() {
        let mut connection = crate::Connection::new_session()
            .map_err(|e| {
                println!("error: {}", e);

                e
            })
            .unwrap();

        // Hello method is already called during connection creation so subsequent calls are expected to fail but only
        // with a D-Bus error.
        match connection.call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "Hello",
            None,
        ) {
            Err(crate::ConnectionError::MethodError(_, _)) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn freedesktop_api() {
        let mut connection = crate::Connection::new_session()
            .map_err(|e| {
                println!("error: {}", e);

                e
            })
            .unwrap();

        if std::env::var("GET_MACHINE_ID").unwrap_or(String::from("1")) == "1" {
            let reply = connection
                .call_method(
                    Some("org.freedesktop.DBus"),
                    "/org/freedesktop/DBus",
                    Some("org.freedesktop.DBus.Peer"),
                    "GetMachineId",
                    None,
                )
                .unwrap();

            assert!(reply
                .body_signature()
                .map(|s| s.as_str() == <String>::SIGNATURE_STR)
                .unwrap());
            let body = reply.body(Some(<String>::SIGNATURE_STR.into())).unwrap();
            let id = String::from_variant(&body.fields()[0]).unwrap();
            println!("Machine ID: {}", id);
        }

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "NameHasOwner",
                Some(Structure::new().add_field(String::from("org.freedesktop.DBus"))),
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s.as_str() == bool::SIGNATURE_STR)
            .unwrap());
        let body = reply.body(Some(bool::SIGNATURE_STR.into())).unwrap();
        assert!(bool::from_variant(&body.fields()[0]).unwrap());

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "GetNameOwner",
                Some(Structure::new().add_field(String::from("org.freedesktop.DBus"))),
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s.as_str() == <String>::SIGNATURE_STR)
            .unwrap());
        let body = reply.body(None).unwrap();
        let owner = String::from_variant(&body.fields()[0]).unwrap();
        println!("Owner of 'org.freedesktop.DBus' is: {}", owner);

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus.Properties"),
                "GetAll",
                Some(Structure::new().add_field(String::from("org.freedesktop.DBus"))),
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s.as_str() == "a{sv}")
            .unwrap());
        let body = reply.body(Some("a{sv}".into())).unwrap();
        let mut fields = body.take_fields();
        let array = Array::take_from_variant(fields.remove(0)).unwrap();
        let dict = Dict::try_from(array).unwrap();
        let mut hashmap: HashMap<String, Variant> = dict.try_into().unwrap();

        // "Features" property
        let features: Vec<String> = Array::take_from_variant(hashmap.remove("Features").unwrap())
            .unwrap()
            .try_into()
            .unwrap();
        println!("org.freedesktop.DBus.Features on /org/freedesktop/DBus:");
        for feature in features {
            print!(" {}", feature);
        }
        println!("");

        // "Interfaces" property
        let interfaces: Vec<String> =
            Array::take_from_variant(hashmap.remove("Interfaces").unwrap())
                .unwrap()
                .try_into()
                .unwrap();
        println!("org.freedesktop.DBus.Interfaces on /org/freedesktop/DBus:");
        for interface in interfaces {
            print!(" {}", interface);
        }
        println!("");
    }
}
