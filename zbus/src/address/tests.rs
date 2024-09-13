use std::{borrow::Cow, ffi::OsStr};

use super::{
    transport::{AutolaunchScope, TcpFamily, Transport, UnixAddrKind},
    DBusAddr,
};

#[test]
fn parse_err() {
    assert_eq!(
        DBusAddr::try_from("").unwrap_err().to_string(),
        "Missing transport in address"
    );
    assert_eq!(
        DBusAddr::try_from("foo").unwrap_err().to_string(),
        "Missing transport in address"
    );
    assert_eq!(
        DBusAddr::try_from("foo:").unwrap_err().to_string(),
        "Unsupported transport in address"
    );
    assert_eq!(
        DBusAddr::try_from("tcp:opt=%1").unwrap_err().to_string(),
        "Encoding error: Incomplete percent-encoded sequence"
    );
    assert_eq!(
        DBusAddr::try_from("tcp:opt=%1z").unwrap_err().to_string(),
        "Encoding error: Invalid hexadecimal character in percent-encoded sequence"
    );
    assert_eq!(
        DBusAddr::try_from("tcp:opt=1\rz").unwrap_err().to_string(),
        "Encoding error: Invalid character in address"
    );
    assert_eq!(
        DBusAddr::try_from("tcp:guid=9406e28972c595c590766c9564ce623")
            .unwrap_err()
            .to_string(),
        "Invalid value for key: `guid`"
    );
    assert_eq!(
        DBusAddr::try_from("tcp:guid=9406e28972c595c590766c9564ce623g")
            .unwrap_err()
            .to_string(),
        "Invalid value for key: `guid`"
    );

    let addr = DBusAddr::try_from("tcp:guid=9406e28972c595c590766c9564ce623f").unwrap();
    addr.guid().unwrap();
}

#[test]
fn parse_unix() {
    let addr = DBusAddr::try_from("unix:path=/tmp/dbus-foo,guid=9406e28972c595c590766c9564ce623f")
        .unwrap();
    let Transport::Unix(u) = addr.transport().unwrap() else {
        panic!();
    };
    assert_eq!(
        u.kind(),
        &UnixAddrKind::Path(Cow::Borrowed(OsStr::new("/tmp/dbus-foo")))
    );

    assert_eq!(
        DBusAddr::try_from("unix:foo=blah").unwrap_err().to_string(),
        "Other error: invalid `unix:` address, missing required key"
    );
    assert_eq!(
            DBusAddr::try_from("unix:path=/blah,abstract=foo")
                .unwrap_err()
                .to_string(),
            "Other error: invalid address, only one of `path` `dir` `tmpdir` `abstract` or `runtime` expected"
        );
    assert_eq!(
        DBusAddr::try_from("unix:runtime=no")
            .unwrap_err()
            .to_string(),
        "Invalid value for key: `runtime`"
    );
    DBusAddr::try_from(String::from("unix:path=/tmp/foo")).unwrap();
}

#[test]
fn parse_launchd() {
    let addr = DBusAddr::try_from("launchd:env=FOOBAR").unwrap();
    let Transport::Launchd(t) = addr.transport().unwrap() else {
        panic!();
    };
    assert_eq!(t.env(), "FOOBAR");

    assert_eq!(
        DBusAddr::try_from("launchd:weof").unwrap_err().to_string(),
        "Missing key: `env`"
    );
}

#[test]
fn parse_systemd() {
    let addr = DBusAddr::try_from("systemd:").unwrap();
    let Transport::Systemd(_) = addr.transport().unwrap() else {
        panic!();
    };
}

#[test]
fn parse_tcp() {
    let addr = DBusAddr::try_from("tcp:host=localhost,bind=*,port=0,family=ipv4").unwrap();
    let Transport::Tcp(t) = addr.transport().unwrap() else {
        panic!();
    };
    assert_eq!(t.host().unwrap(), "localhost");
    assert_eq!(t.bind().unwrap(), "*");
    assert_eq!(t.port().unwrap(), 0);
    assert_eq!(t.family().unwrap(), TcpFamily::IPv4);

    let addr = DBusAddr::try_from("tcp:").unwrap();
    let Transport::Tcp(t) = addr.transport().unwrap() else {
        panic!();
    };
    assert!(t.host().is_none());
    assert!(t.bind().is_none());
    assert!(t.port().is_none());
    assert!(t.family().is_none());
}

#[test]
fn parse_nonce_tcp() {
    let addr =
        DBusAddr::try_from("nonce-tcp:host=localhost,bind=*,port=0,family=ipv6,noncefile=foo")
            .unwrap();
    let Transport::NonceTcp(t) = addr.transport().unwrap() else {
        panic!();
    };
    assert_eq!(t.host().unwrap(), "localhost");
    assert_eq!(t.bind().unwrap(), "*");
    assert_eq!(t.port().unwrap(), 0);
    assert_eq!(t.family().unwrap(), TcpFamily::IPv6);
    assert_eq!(t.noncefile().unwrap(), "foo");
}

#[test]
fn parse_unixexec() {
    let addr = DBusAddr::try_from("unixexec:path=/bin/test,argv2=foo").unwrap();
    let Transport::Unixexec(t) = addr.transport().unwrap() else {
        panic!();
    };

    assert_eq!(t.path(), "/bin/test");
    assert_eq!(t.argv(), &[(2, Cow::from("foo"))]);

    assert_eq!(
        DBusAddr::try_from("unixexec:weof").unwrap_err().to_string(),
        "Missing key: `path`"
    );
}

#[test]
fn parse_autolaunch() {
    let addr = DBusAddr::try_from("autolaunch:scope=*user").unwrap();
    let Transport::Autolaunch(t) = addr.transport().unwrap() else {
        panic!();
    };
    assert_eq!(t.scope().unwrap(), &AutolaunchScope::User);
}

#[test]
#[cfg(feature = "vsock")]
fn parse_vsock() {
    let addr = DBusAddr::try_from("vsock:cid=12,port=32").unwrap();
    let Transport::Vsock(t) = addr.transport().unwrap() else {
        panic!();
    };
    assert_eq!(t.port(), Some(32));
    assert_eq!(t.cid(), Some(12));

    assert_eq!(
        DBusAddr::try_from("vsock:port=abc")
            .unwrap_err()
            .to_string(),
        "Invalid value for key: `port`"
    );
}
