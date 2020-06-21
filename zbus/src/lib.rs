#[cfg(doctest)]
mod doctests {
    doc_comment::doctest!("../../README.md");
    // Book markdown checks
    doc_comment::doctest!("../../book/src/client.md");
    doc_comment::doctest!("../../book/src/concepts.md");
    doc_comment::doctest!("../../book/src/connection.md");
    doc_comment::doctest!("../../book/src/contributors.md");
    doc_comment::doctest!("../../book/src/introduction.md");
    doc_comment::doctest!("../../book/src/server.md");
}

mod error;
pub use error::*;

mod address;

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

mod proxy;
pub use proxy::*;

mod owned_fd;
mod utils;

mod object_server;
pub use object_server::*;

pub mod fdo;

pub mod xml;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryInto;
    use std::fs::File;
    use std::os::unix::io::{FromRawFd, RawFd};

    use enumflags2::BitFlags;
    use serde_repr::{Deserialize_repr, Serialize_repr};

    use zvariant::{Fd, OwnedValue, Type};
    use zvariant_derive::Type;

    use crate::{Message, MessageFlags};

    #[test]
    fn msg() {
        let mut m = Message::method(
            None,
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus.Peer"),
            "GetMachineId",
            &(),
        )
        .unwrap();
        m.modify_primary_header(|primary| {
            primary.set_flags(BitFlags::from(MessageFlags::NoAutoStart));
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
        let connection = crate::Connection::new_session()
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
            &(),
        ) {
            Err(crate::Error::MethodError(_, _, _)) => (),
            Err(e) => panic!("{}", e),
            _ => panic!(),
        };
    }

    #[test]
    fn fdpass_systemd() {
        let connection = crate::Connection::new_system().unwrap();

        let mut reply = connection
            .call_method(
                Some("org.freedesktop.systemd1"),
                "/org/freedesktop/systemd1",
                Some("org.freedesktop.systemd1.Manager"),
                "DumpByFileDescriptor",
                &(),
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s == <Fd>::signature())
            .unwrap());

        let fd: RawFd = reply.body().unwrap();
        reply.disown_fds();
        assert!(fd >= 0);
        let f = unsafe { File::from_raw_fd(fd) };
        f.metadata().unwrap();
    }

    #[test]
    fn freedesktop_api() {
        let mut connection = crate::Connection::new_session()
            .map_err(|e| {
                println!("error: {}", e);

                e
            })
            .unwrap();

        connection.set_default_message_handler(Box::new(|msg| {
            // Debug implementation will test it a bit
            println!("Received while waiting for a reply: {}", msg);

            Some(msg)
        }));

        // Let's try getting us a fancy name on the bus
        #[repr(u32)]
        #[derive(Type, BitFlags, Debug, PartialEq, Copy, Clone)]
        enum RequestNameFlags {
            AllowReplacement = 0x01,
            ReplaceExisting = 0x02,
            DoNotQueue = 0x04,
        }

        #[repr(u32)]
        #[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
        enum RequestNameReply {
            PrimaryOwner = 0x01,
            InQueue = 0x02,
            Exists = 0x03,
            AlreadyOwner = 0x04,
        }

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "RequestName",
                &(
                    "org.freedesktop.zbus",
                    BitFlags::from(RequestNameFlags::ReplaceExisting),
                ),
            )
            .unwrap();

        assert!(reply.body_signature().map(|s| s == "u").unwrap());
        let reply: RequestNameReply = reply.body().unwrap();
        assert_eq!(reply, RequestNameReply::PrimaryOwner);

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "GetId",
                &(),
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s == <&str>::signature())
            .unwrap());
        let id: &str = reply.body().unwrap();
        println!("Unique ID of the bus: {}", id);

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "NameHasOwner",
                &"org.freedesktop.zbus",
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
                &"org.freedesktop.zbus",
            )
            .unwrap();

        assert!(reply
            .body_signature()
            .map(|s| s == <&str>::signature())
            .unwrap());
        assert_eq!(
            Some(reply.body::<&str>().unwrap()),
            connection.unique_name()
        );

        let reply = connection
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "GetConnectionCredentials",
                &"org.freedesktop.DBus",
            )
            .unwrap();

        assert!(reply.body_signature().map(|s| s == "a{sv}").unwrap());
        let hashmap: HashMap<&str, OwnedValue> = reply.body().unwrap();

        let pid: u32 = (&hashmap["ProcessID"]).try_into().unwrap();
        println!("DBus bus PID: {}", pid);

        let uid: u32 = (&hashmap["UnixUserID"]).try_into().unwrap();
        println!("DBus bus UID: {}", uid);
    }
}
