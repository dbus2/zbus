use ntest::timeout;
use test_log::test;

use zbus::{
    blocking::{self, MessageIterator},
    message::Message,
    names::UniqueName,
};

#[test]
#[timeout(15000)]
fn issue_68() {
    // Tests the fix for https://github.com/dbus2/zbus/issues/68
    //
    // While this is not an exact reproduction of the issue 68, the underlying problem it
    // produces is exactly the same: `Connection::call_method` dropping all incoming messages
    // while waiting for the reply to the method call.
    let conn = blocking::Connection::session().unwrap();
    let stream = MessageIterator::from(&conn);

    // Send a message as client before service starts to process messages
    let client_conn = blocking::Connection::session().unwrap();
    let destination = conn.unique_name().map(UniqueName::<'_>::from).unwrap();
    let msg = Message::method("/org/freedesktop/Issue68", "Ping")
        .unwrap()
        .destination(destination)
        .unwrap()
        .interface("org.freedesktop.Issue68")
        .unwrap()
        .build(&())
        .unwrap();
    let serial = msg.primary_header().serial_num();
    client_conn.send(&msg).unwrap();

    zbus::blocking::fdo::DBusProxy::new(&conn)
        .unwrap()
        .get_id()
        .unwrap();

    for m in stream {
        let msg = m.unwrap();

        if msg.primary_header().serial_num() == serial {
            break;
        }
    }
}
