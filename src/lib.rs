mod message;
pub use message::*;

mod message_field;
pub use message_field::*;

mod connection;
pub use connection::*;

mod variant;
pub use variant::*;

#[cfg(test)]
mod tests {
    use crate::variant::{Signature, VariantType};

    #[test]
    fn it_works() {
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
                f.code == crate::message_field::MessageFieldCode::Signature
                    && f.value.get().unwrap_or(Signature("")).0 == <(&str)>::SIGNATURE_STR
            })
            .unwrap();
        let body = reply.get_body().unwrap();
        let v = crate::variant::Variant::from_data(&body, "s").unwrap();
        let id = v.get::<(&str)>().unwrap();
        println!("Machine ID: {}", id);

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "GetNameOwner",
                Some(crate::variant::Variant::from("org.freedesktop.DBus")),
            )
            .unwrap();

        let all_fields = reply.get_fields().unwrap();
        all_fields
            .iter()
            .find(|f| {
                f.code == crate::message_field::MessageFieldCode::Signature
                    && f.value.get().unwrap_or(Signature("")).0 == <(&str)>::SIGNATURE_STR
            })
            .unwrap();
        let body = reply.get_body().unwrap();
        let v = crate::variant::Variant::from_data(&body, "s").unwrap();
        let owner = v.get::<(&str)>().unwrap();
        println!("Owner of 'org.freedesktop.DBus' is: {}", owner);
    }
}
