//! D-Bus supported transports.

use std::fmt;

use super::{percent, Address, Error, KeyValFmt, KeyValFmtAdd, Result};

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
pub use vsock::Vsock;

/// A D-Bus transport.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Transport<'a> {
    /// Unix Domain Sockets transport.
    Unix(unix::Unix<'a>),
    /// launchd transport.
    Launchd(launchd::Launchd<'a>),
    /// systemd transport.
    Systemd(systemd::Systemd<'a>),
    /// TCP Sockets transport.
    Tcp(tcp::Tcp<'a>),
    /// Nonce-authenticated TCP Sockets transport.
    NonceTcp(nonce_tcp::NonceTcp<'a>),
    /// Executed Subprocesses on Unix transport.
    Unixexec(unixexec::Unixexec<'a>),
    /// Autolaunch transport.
    Autolaunch(autolaunch::Autolaunch<'a>),
    /// VSOCK Sockets transport.
    Vsock(vsock::Vsock<'a>),
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
            Self::Vsock(_) => write!(f, "vsock"),
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
            Self::Vsock(t) => t.key_val_fmt_add(kv),
        }
    }
}

impl<'a> TryFrom<&'a Address<'a>> for Transport<'a> {
    type Error = Error;

    fn try_from(s: &'a Address<'a>) -> Result<Self> {
        let col = s.addr.find(':').ok_or(Error::MissingTransport)?;
        match &s.addr[..col] {
            "unix" => Ok(Self::Unix(s.try_into()?)),
            "launchd" => Ok(Self::Launchd(s.try_into()?)),
            "systemd" => Ok(Self::Systemd(s.try_into()?)),
            "tcp" => Ok(Self::Tcp(s.try_into()?)),
            "nonce-tcp" => Ok(Self::NonceTcp(s.try_into()?)),
            "unixexec" => Ok(Self::Unixexec(s.try_into()?)),
            "autolaunch" => Ok(Self::Autolaunch(s.try_into()?)),
            "vsock" => Ok(Self::Vsock(s.try_into()?)),
            _ => Err(Error::UnknownTransport),
        }
    }
}
