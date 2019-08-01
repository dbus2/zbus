mod message;
pub use message::*;

mod message_field;
pub use message_field::*;

mod connection;
pub use connection::*;

mod variant;
pub use variant::*;

mod variant_type;
pub use variant_type::*;

mod object_path;
pub use object_path::*;

mod signature;
pub use signature::*;

mod structure;
pub use structure::*;

mod utils;

#[cfg(test)]
mod tests {
    use crate::{Signature, VariantType};

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

            let all_fields = reply.get_fields().unwrap();
            all_fields
                .iter()
                .find(|f| {
                    f.code() == crate::MessageFieldCode::Signature
                        && f.value()
                            .map(|v| {
                                v.get::<Signature>()
                                    .map(|s| s.as_str() == <(&str)>::SIGNATURE_STR)
                                    .unwrap_or(false)
                            })
                            .unwrap_or(false)
                })
                .unwrap();
            let body = reply.get_body().unwrap();
            let v = crate::Variant::from_data(&body, "s").unwrap();
            let id = v.get::<(&str)>().unwrap();
            println!("Machine ID: {}", id);
        }

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "NameHasOwner",
                Some(crate::Variant::from("org.freedesktop.DBus")),
            )
            .unwrap();

        let all_fields = reply.get_fields().unwrap();
        all_fields
            .iter()
            .find(|f| {
                f.code() == crate::MessageFieldCode::Signature
                    && f.value()
                        .map(|v| {
                            v.get::<Signature>()
                                .map(|s| s.as_str() == bool::SIGNATURE_STR)
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
            })
            .unwrap();
        let body = reply.get_body().unwrap();
        let v = crate::Variant::from_data(&body, bool::SIGNATURE_STR).unwrap();
        assert!(v.get::<bool>().unwrap());

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "GetNameOwner",
                Some(crate::Variant::from("org.freedesktop.DBus")),
            )
            .unwrap();

        let all_fields = reply.get_fields().unwrap();
        all_fields
            .iter()
            .find(|f| {
                f.code() == crate::MessageFieldCode::Signature
                    && f.value()
                        .map(|v| {
                            v.get::<Signature>()
                                .map(|s| s.as_str() == <(&str)>::SIGNATURE_STR)
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
            })
            .unwrap();
        let body = reply.get_body().unwrap();
        let v = crate::Variant::from_data(&body, "s").unwrap();
        let owner = v.get::<(&str)>().unwrap();
        println!("Owner of 'org.freedesktop.DBus' is: {}", owner);
    }
}
