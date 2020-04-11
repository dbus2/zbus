mod address;
pub use address::*;

mod message;
pub use message::*;

mod message_header;
pub use message_header::*;

mod message_field;
pub use message_field::*;

mod message_fields;
pub use message_fields::*;

mod connection;
pub use connection::*;

mod utils;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use zvariant::{Array, FromVariant};
    use zvariant::{Variant, VariantValue};

    use crate::{Message, MessageFlags};

    #[test]
    fn msg() {
        let mut m = Message::method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus.Peer"),
            "GetMachineId",
            &(),
        )
        .unwrap();
        m.modify_primary_header(|primary| {
            primary.set_flags(MessageFlags::NoAutoStart);
            primary.set_serial_num(11);

            Ok(())
        })
        .unwrap();
        let primary = m.primary_header().unwrap();
        assert!(primary.serial_num() == 11);
        assert!(primary.flags() == MessageFlags::NoAutoStart);
    }

    #[test]
    fn basic_connection() {
        crate::Connection::new_session()
            .map_err(|e| {
                println!("error: {}", e);

                e
            })
            .unwrap();
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
                    &(),
                )
                .unwrap();

            assert!(reply
                .body_signature()
                .map(|s| s == <&str>::signature())
                .unwrap());
            let id: &str = reply.body().unwrap();
            println!("Machine ID: {}", id);
        }

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "NameHasOwner",
                &"org.freedesktop.DBus",
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s == bool::signature())
            .unwrap());
        assert!(reply.body::<bool>().unwrap());

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "GetNameOwner",
                &"org.freedesktop.DBus",
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s == <&str>::signature())
            .unwrap());
        let owner: &str = reply.body().unwrap();
        println!("Owner of 'org.freedesktop.DBus' is: {}", owner);

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus.Properties"),
                "GetAll",
                &"org.freedesktop.DBus",
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s.as_str() == "a{sv}")
            .unwrap());
        let hashmap: HashMap<&str, Variant> = reply.body().unwrap();

        // "Features" property
        let features = Array::from_variant_ref(&hashmap["Features"]).unwrap();
        println!("org.freedesktop.DBus.Features on /org/freedesktop/DBus:");
        for feature in features.get() {
            print!(" {}", <&str>::from_variant_ref(feature).unwrap());
        }
        println!();

        // "Interfaces" property
        let interfaces = Array::from_variant_ref(&hashmap["Interfaces"]).unwrap();
        println!("org.freedesktop.DBus.Interfaces on /org/freedesktop/DBus:");
        for interface in interfaces.get() {
            print!(" {}", <&str>::from_variant_ref(interface).unwrap());
        }
        println!();
    }
}
