//! D-Bus address handling.
//!
//! Server addresses consist of a transport name followed by a colon, and then an optional,
//! comma-separated list of keys and values in the form key=value.
//!
//! See also:
//!
//! * [Server addresses] in the D-Bus specification.
//!
//! [Server addresses]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses

// note: assumes values are utf-8 encoded - this should be clarified in the spec
// otherwise, fail to read them or use lossy representation for display
//
// assumes that empty key=val is accepted, so "transport:,,guid=..." is valid
//
// allows key only, so "transport:foo,bar" is ok
//
// mostly ignores unknown keys and transport

use std::env;

#[cfg(all(unix, not(target_os = "macos")))]
use nix::unistd::Uid;

use crate::Result;

pub mod transport;

mod address;
pub use address::{DBusAddr, ToDBusAddrs};

mod address_list;
pub use address_list::{DBusAddrList, DBusAddrListIter};

mod percent;
pub use percent::*;

/// Get the address for session socket respecting the DBUS_SESSION_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// $XDG_RUNTIME_DIR/bus
pub fn session() -> Result<DBusAddrList<'static>> {
    match env::var("DBUS_SESSION_BUS_ADDRESS") {
        Ok(val) => DBusAddrList::try_from(val),
        _ => {
            #[cfg(windows)]
            {
                #[cfg(feature = "windows-gdbus")]
                return DBusAddrList::try_from("autolaunch:");

                #[cfg(not(feature = "windows-gdbus"))]
                return DBusAddrList::try_from("autolaunch:scope=*user");
            }

            #[cfg(all(unix, not(target_os = "macos")))]
            {
                let runtime_dir = env::var("XDG_RUNTIME_DIR")
                    .unwrap_or_else(|_| format!("/run/user/{}", Uid::effective()));
                let path = format!("unix:path={runtime_dir}/bus");

                DBusAddrList::try_from(path)
            }

            #[cfg(target_os = "macos")]
            return DBusAddrList::try_from("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET");
        }
    }
}

/// Get the address for system bus respecting the DBUS_SYSTEM_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// /var/run/dbus/system_bus_socket
pub fn system() -> Result<DBusAddrList<'static>> {
    match env::var("DBUS_SYSTEM_BUS_ADDRESS") {
        Ok(val) => DBusAddrList::try_from(val),
        _ => {
            #[cfg(all(unix, not(target_os = "macos")))]
            return DBusAddrList::try_from("unix:path=/var/run/dbus/system_bus_socket");

            #[cfg(windows)]
            return DBusAddrList::try_from("autolaunch:");

            #[cfg(target_os = "macos")]
            return DBusAddrList::try_from("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Cow, ffi::OsStr};

    use crate::addr::transport::{AutolaunchScope, TcpFamily};

    use super::{
        transport::{Transport, UnixAddrKind},
        DBusAddr,
    };

    #[test]
    fn parse_err() {
        assert_eq!(
            DBusAddr::try_from("").unwrap_err().to_string(),
            "address error: DBusAddr has no transport"
        );
        assert_eq!(
            DBusAddr::try_from("foo").unwrap_err().to_string(),
            "address error: DBusAddr has no transport"
        );
        DBusAddr::try_from("foo:opt").unwrap();
        assert_eq!(
            DBusAddr::try_from("foo:opt=1,opt=2")
                .unwrap_err()
                .to_string(),
            "address error: Duplicate key `opt`"
        );
        assert_eq!(
            DBusAddr::try_from("foo:opt=%1").unwrap_err().to_string(),
            "address error: Incomplete percent-encoded sequence"
        );
        assert_eq!(
            DBusAddr::try_from("foo:opt=%1z").unwrap_err().to_string(),
            "address error: Invalid hexadecimal character in percent-encoded sequence"
        );
        assert_eq!(
            DBusAddr::try_from("foo:opt=1\rz").unwrap_err().to_string(),
            "address error: Invalid character in address"
        );

        let addr = DBusAddr::try_from("foo:guid=9406e28972c595c590766c9564ce623f").unwrap();
        addr.guid().unwrap().unwrap();
    }

    #[test]
    fn parse_unix() {
        let addr =
            DBusAddr::try_from("unix:path=/tmp/dbus-foo,guid=9406e28972c595c590766c9564ce623f")
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
            "address error: Invalid `unix:` address, missing required key"
        );
        assert_eq!(
            DBusAddr::try_from("unix:path=/blah,abstract=foo").unwrap_err().to_string(),
            "address error: Invalid address: only one of `path` `dir` `tmpdir` `abstract` or `runtime` expected"
        );
        assert_eq!(
            DBusAddr::try_from("unix:runtime=no")
                .unwrap_err()
                .to_string(),
            "address error: Invalid runtime=no value"
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
            "address error: Missing env="
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
            "address error: Missing path="
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
            "address error: Invalid port: invalid digit found in string"
        );
    }
}
