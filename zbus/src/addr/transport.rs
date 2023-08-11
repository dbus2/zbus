//! D-Bus supported transports.

use std::fmt;

use crate::{Error, Result};

use super::{
    address::{KeyValFmt, KeyValFmtAdd},
    percent, DBusAddr,
};

mod autolaunch;
pub use autolaunch::{Autolaunch, AutolaunchScope};

mod launchd;
pub use launchd::Launchd;

mod nonce_tcp;
pub use nonce_tcp::NonceTcp;

mod systemd;
pub use systemd::Systemd;

mod tcp;
pub use tcp::{Tcp, TcpFamily};

mod unix;
pub use unix::{Unix, UnixAddrKind};

mod unixexec;
pub use unixexec::Unixexec;

mod vsock;
#[cfg(any(feature = "vsock", feature = "tokio-vsock"))]
pub use self::vsock::*;

/// A D-Bus transport.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Transport<'a> {
    Unix(unix::Unix<'a>),
    Launchd(launchd::Launchd<'a>),
    Systemd(systemd::Systemd<'a>),
    Tcp(tcp::Tcp<'a>),
    NonceTcp(nonce_tcp::NonceTcp<'a>),
    Unixexec(unixexec::Unixexec<'a>),
    Autolaunch(autolaunch::Autolaunch<'a>),
    #[cfg(any(feature = "vsock", feature = "tokio-vsock"))]
    Vsock(vsock::Vsock<'a>),
    Other(&'a str),
}

impl fmt::Display for Transport<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unix(_) => write!(f, "unix"),
            Self::Launchd(_) => write!(f, "launchd"),
            Self::Systemd(_) => write!(f, "systemd"),
            Self::Tcp(_) => write!(f, "tcp"),
            Self::NonceTcp(_) => write!(f, "nonce-tcp"),
            Self::Unixexec(_) => write!(f, "unixexec"),
            Self::Autolaunch(_) => write!(f, "autolaunch"),
            #[cfg(any(feature = "vsock", feature = "tokio-vsock"))]
            Self::Vsock(_) => write!(f, "vsock"),
            Self::Other(o) => write!(f, "{o}"),
        }
    }
}

impl KeyValFmtAdd for Transport<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        match self {
            Self::Unix(t) => t.key_val_fmt_add(kv),
            Self::Launchd(t) => t.key_val_fmt_add(kv),
            Self::Systemd(t) => t.key_val_fmt_add(kv),
            Self::Tcp(t) => t.key_val_fmt_add(kv),
            Self::NonceTcp(t) => t.key_val_fmt_add(kv),
            Self::Unixexec(t) => t.key_val_fmt_add(kv),
            Self::Autolaunch(t) => t.key_val_fmt_add(kv),
            #[cfg(any(feature = "vsock", feature = "tokio-vsock"))]
            Self::Vsock(t) => t.key_val_fmt_add(kv),
            Self::Other(_) => kv,
        }
    }
}

impl<'a> TryFrom<&'a DBusAddr<'a>> for Transport<'a> {
    type Error = Error;

    fn try_from(s: &'a DBusAddr<'a>) -> Result<Self> {
        let col = s
            .addr
            .find(':')
            .ok_or_else(|| Error::Address("DBusAddr has no transport".into()))?;
        match &s.addr[..col] {
            "unix" => Ok(Self::Unix(s.try_into()?)),
            "launchd" => Ok(Self::Launchd(s.try_into()?)),
            "systemd" => Ok(Self::Systemd(s.try_into()?)),
            "tcp" => Ok(Self::Tcp(s.try_into()?)),
            "nonce-tcp" => Ok(Self::NonceTcp(s.try_into()?)),
            "unixexec" => Ok(Self::Unixexec(s.try_into()?)),
            "autolaunch" => Ok(Self::Autolaunch(s.try_into()?)),
            #[cfg(any(feature = "vsock", feature = "tokio-vsock"))]
            "vsock" => Ok(Self::Vsock(s.try_into()?)),
            o => Ok(Self::Other(o)),
        }
    }
}
