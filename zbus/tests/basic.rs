use std::collections::HashMap;

use enumflags2::BitFlags;
use ntest::timeout;
use test_log::test;
use tracing::{debug, instrument};
use zbus::block_on;

use zbus_names::UniqueName;
use zvariant::{OwnedValue, Type};

use zbus::{
    blocking,
    fdo::{RequestNameFlags, RequestNameReply},
    message::Message,
    Connection, Result,
};

#[test]
fn msg() {
    let m = Message::method_call("/org/freedesktop/DBus", "GetMachineId")
        .unwrap()
        .destination("org.freedesktop.DBus")
        .unwrap()
        .interface("org.freedesktop.DBus.Peer")
        .unwrap()
        .build(&())
        .unwrap();
    let hdr = m.header();
    assert_eq!(hdr.path().unwrap(), "/org/freedesktop/DBus");
    assert_eq!(hdr.interface().unwrap(), "org.freedesktop.DBus.Peer");
    assert_eq!(hdr.member().unwrap(), "GetMachineId");
}

#[test]
#[timeout(15000)]
#[instrument]
fn basic_connection() {
    let connection = blocking::Connection::session()
        .map_err(|e| {
            debug!("error: {}", e);

            e
        })
        .unwrap();
    // Hello method is already called during connection creation so subsequent calls are
    // expected to fail but only with a D-Bus error.
    match connection.call_method(
        Some("org.freedesktop.DBus"),
        "/org/freedesktop/DBus",
        Some("org.freedesktop.DBus"),
        "Hello",
        &(),
    ) {
        Err(zbus::Error::MethodError(_, _, _)) => (),
        Err(e) => panic!("{}", e),

        _ => panic!(),
    };
}

#[test]
#[timeout(15000)]
fn basic_connection_async() {
    block_on(test_basic_connection()).unwrap();
}

async fn test_basic_connection() -> Result<()> {
    let connection = Connection::session().await?;

    match connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "Hello",
            &(),
        )
        .await
    {
        Err(zbus::Error::MethodError(_, _, _)) => (),
        Err(e) => panic!("{}", e),

        _ => panic!(),
    };

    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
#[test]
#[timeout(15000)]
fn fdpass_systemd() {
    use std::{fs::File, os::unix::io::AsRawFd};
    use zvariant::OwnedFd;

    let connection = blocking::Connection::system().unwrap();

    let reply = connection
        .call_method(
            Some("org.freedesktop.systemd1"),
            "/org/freedesktop/systemd1",
            Some("org.freedesktop.systemd1.Manager"),
            "DumpByFileDescriptor",
            &(),
        )
        .unwrap();

    let fd: OwnedFd = reply.body().deserialize().unwrap();
    assert!(fd.as_raw_fd() >= 0);
    let f = File::from(std::os::fd::OwnedFd::from(fd));
    f.metadata().unwrap();
}

#[test]
#[instrument]
#[timeout(15000)]
fn freedesktop_api() {
    let connection = blocking::Connection::session()
        .map_err(|e| {
            debug!("error: {}", e);

            e
        })
        .unwrap();

    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "RequestName",
            &(
                "org.freedesktop.zbus.sync",
                BitFlags::from(RequestNameFlags::ReplaceExisting),
            ),
        )
        .unwrap();

    let body = reply.body();
    assert!(body.signature().map(|s| s == "u").unwrap());
    let reply: RequestNameReply = body.deserialize().unwrap();
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

    let body = reply.body();
    assert!(body.signature().map(|s| <&str>::SIGNATURE == s).unwrap());
    let id: &str = body.deserialize().unwrap();
    debug!("Unique ID of the bus: {}", id);

    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "NameHasOwner",
            &"org.freedesktop.zbus.sync",
        )
        .unwrap();

    let body = reply.body();
    assert!(body.signature().map(|s| bool::SIGNATURE == s).unwrap());
    assert!(body.deserialize::<bool>().unwrap());

    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "GetNameOwner",
            &"org.freedesktop.zbus.sync",
        )
        .unwrap();

    let body = reply.body();
    assert!(body.signature().map(|s| <&str>::SIGNATURE == s).unwrap());
    assert_eq!(
        body.deserialize::<UniqueName<'_>>().unwrap(),
        *connection.unique_name().unwrap(),
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

    let body = reply.body();
    assert!(body.signature().map(|s| s == "a{sv}").unwrap());
    let hashmap: HashMap<&str, OwnedValue> = body.deserialize().unwrap();

    let pid: u32 = (&hashmap["ProcessID"]).try_into().unwrap();
    debug!("DBus bus PID: {}", pid);

    #[cfg(unix)]
    {
        let uid: u32 = (&hashmap["UnixUserID"]).try_into().unwrap();
        debug!("DBus bus UID: {}", uid);
    }
}

#[test]
#[timeout(15000)]
fn freedesktop_api_async() {
    block_on(test_freedesktop_api()).unwrap();
}

#[instrument]
async fn test_freedesktop_api() -> Result<()> {
    let connection = Connection::session().await?;

    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "RequestName",
            &(
                "org.freedesktop.zbus.async",
                BitFlags::from(RequestNameFlags::ReplaceExisting),
            ),
        )
        .await
        .unwrap();

    let body = reply.body();
    assert!(body.signature().map(|s| s == "u").unwrap());
    let reply: RequestNameReply = body.deserialize().unwrap();
    assert_eq!(reply, RequestNameReply::PrimaryOwner);

    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "GetId",
            &(),
        )
        .await
        .unwrap();

    let body = reply.body();
    assert!(body.signature().map(|s| <&str>::SIGNATURE == s).unwrap());
    let id: &str = body.deserialize().unwrap();
    debug!("Unique ID of the bus: {}", id);

    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "NameHasOwner",
            &"org.freedesktop.zbus.async",
        )
        .await
        .unwrap();

    let body = reply.body();
    assert!(body.signature().map(|s| bool::SIGNATURE == s).unwrap());
    assert!(body.deserialize::<bool>().unwrap());

    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "GetNameOwner",
            &"org.freedesktop.zbus.async",
        )
        .await
        .unwrap();

    let body = reply.body();
    assert!(body.signature().map(|s| <&str>::SIGNATURE == s).unwrap());
    assert_eq!(
        body.deserialize::<UniqueName<'_>>().unwrap(),
        *connection.unique_name().unwrap(),
    );

    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "GetConnectionCredentials",
            &"org.freedesktop.DBus",
        )
        .await
        .unwrap();

    let body = reply.body();
    assert!(body.signature().map(|s| s == "a{sv}").unwrap());
    let hashmap: HashMap<&str, OwnedValue> = body.deserialize().unwrap();

    let pid: u32 = (&hashmap["ProcessID"]).try_into().unwrap();
    debug!("DBus bus PID: {}", pid);

    #[cfg(unix)]
    {
        let uid: u32 = (&hashmap["UnixUserID"]).try_into().unwrap();
        debug!("DBus bus UID: {}", uid);
    }

    Ok(())
}
