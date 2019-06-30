mod message;
pub use message::*;

mod connection;
pub use connection::*;

mod variant;
pub use variant::*;

#[cfg(test)]
mod tests {
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
            .find(|(f, v)| {
                *f == crate::message::MessageField::Signature
                    && v.get_string().unwrap_or(String::from("")) == "s"
            })
            .unwrap();
        let id = crate::variant::Variant::from_data(&reply.get_body().unwrap(), "s")
            .unwrap()
            .get_string()
            .unwrap();
        println!("Machine ID: {}", id);

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "GetNameOwner",
                Some(crate::variant::Variant::from_string("org.freedesktop.DBus")),
            )
            .unwrap();

        let all_fields = reply.get_fields().unwrap();
        all_fields
            .iter()
            .find(|(f, v)| {
                *f == crate::message::MessageField::Signature
                    && v.get_string().unwrap_or(String::from("")) == "s"
            })
            .unwrap();
        let owner = crate::variant::Variant::from_data(&reply.get_body().unwrap(), "s")
            .unwrap()
            .get_string()
            .unwrap();
        println!("Owner of 'org.freedesktop.DBus' is: {}", owner);
    }
}
