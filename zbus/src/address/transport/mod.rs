//! D-Bus supported transports.

use std::fmt;

use super::{percent, Address, Error, KeyValFmt, KeyValFmtAdd, Result};

mod autolaunch;
pub use autolaunch::Autolaunch;
#[cfg(target_os = "windows")]
pub use autolaunch::AutolaunchScope;

#[cfg(target_os = "macos")]
mod launchd;
#[cfg(target_os = "macos")]
pub use launchd::Launchd;

mod nonce_tcp;
pub use nonce_tcp::NonceTcp;

#[cfg(target_os = "linux")]
mod systemd;
#[cfg(target_os = "linux")]
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
    #[cfg(target_os = "macos")]
    /// launchd transport.
    Launchd(launchd::Launchd<'a>),
    #[cfg(target_os = "linux")]
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
            #[cfg(target_os = "macos")]
            Self::Launchd(_) => write!(f, "launchd"),
            #[cfg(target_os = "linux")]
            Self::Systemd(_) => write!(f, "systemd"),
            Self::Tcp(_) => write!(f, "tcp"),
            Self::NonceTcp(_) => write!(f, "nonce-tcp"),
            Self::Unixexec(_) => write!(f, "unixexec"),
            Self::Autolaunch(_) => write!(f, "autolaunch"),
            Self::Vsock(_) => write!(f, "vsock"),
        }
    }
}

pub(crate) trait TransportImpl<'a> {
    fn for_address(s: &'a Address<'a>) -> Result<Self>
    where
        Self: Sized;
}

impl KeyValFmtAdd for Transport<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        match self {
            Self::Unix(t) => t.key_val_fmt_add(kv),
            #[cfg(target_os = "macos")]
            Self::Launchd(t) => t.key_val_fmt_add(kv),
            #[cfg(target_os = "linux")]
            Self::Systemd(t) => t.key_val_fmt_add(kv),
            Self::Tcp(t) => t.key_val_fmt_add(kv),
            Self::NonceTcp(t) => t.key_val_fmt_add(kv),
            Self::Unixexec(t) => t.key_val_fmt_add(kv),
            Self::Autolaunch(t) => t.key_val_fmt_add(kv),
            Self::Vsock(t) => t.key_val_fmt_add(kv),
        }
    }
}

impl<'a> TransportImpl<'a> for Transport<'a> {
    fn for_address(s: &'a Address<'a>) -> Result<Self> {
        let col = s.addr.find(':').ok_or(Error::MissingTransport)?;
        match &s.addr[..col] {
            "unix" => Ok(Self::Unix(Unix::for_address(s)?)),
            #[cfg(target_os = "macos")]
            "launchd" => Ok(Self::Launchd(Launchd::for_address(s)?)),
            #[cfg(target_os = "linux")]
            "systemd" => Ok(Self::Systemd(Systemd::for_address(s)?)),
            "tcp" => Ok(Self::Tcp(Tcp::for_address(s)?)),
            "nonce-tcp" => Ok(Self::NonceTcp(NonceTcp::for_address(s)?)),
            "unixexec" => Ok(Self::Unixexec(Unixexec::for_address(s)?)),
            "autolaunch" => Ok(Self::Autolaunch(Autolaunch::for_address(s)?)),
            "vsock" => Ok(Self::Vsock(Vsock::for_address(s)?)),
            _ => Err(Error::UnknownTransport),
        }
    }
}
