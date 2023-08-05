use std::{borrow::Cow, fmt};

use crate::{Error, Result};

use super::{percent::decode_percents_str, DBusAddr, KeyValFmt, KeyValFmtAdd};

/// TCP IP address family
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum TcpFamily {
    /// IPv4
    IPv4,
    /// IPv6
    IPv6,
}

impl fmt::Display for TcpFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IPv4 => write!(f, "ipv4"),
            Self::IPv6 => write!(f, "ipv6"),
        }
    }
}

impl TryFrom<&str> for TcpFamily {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "ipv4" => Ok(Self::IPv4),
            "ipv6" => Ok(Self::IPv6),
            _ => Err(Error::Address(format!("Unknown TCP family: {s}"))),
        }
    }
}

/// `tcp:` D-Bus transport.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Tcp<'a> {
    host: Option<Cow<'a, str>>,
    bind: Option<Cow<'a, str>>,
    port: Option<u16>,
    family: Option<TcpFamily>,
}

impl<'a> Tcp<'a> {
    /// DNS name or IP address.
    pub fn host(&self) -> Option<&str> {
        self.host.as_ref().map(|v| v.as_ref())
    }

    /// Used in a listenable address to configure the interface on which the server will listen:
    /// either the IP address of one of the local machine's interfaces (most commonly `127.0.0.1`),
    /// or a DNS name that resolves to one of those IP addresses, or `*` to listen on all interfaces
    /// simultaneously.
    pub fn bind(&self) -> Option<&str> {
        self.bind.as_ref().map(|v| v.as_ref())
    }

    /// The TCP port the server will open. A zero value let the server choose a free port provided
    /// from the underlaying operating system.
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// If set, provide the type of socket family.
    pub fn family(&self) -> Option<TcpFamily> {
        self.family
    }
}

impl KeyValFmtAdd for Tcp<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv.add("host", self.host())
            .add("bind", self.bind())
            .add("port", self.port())
            .add("family", self.family())
    }
}

impl<'a> TryFrom<&'a DBusAddr<'a>> for Tcp<'a> {
    type Error = Error;

    fn try_from(s: &'a DBusAddr<'a>) -> Result<Self> {
        let mut res = Tcp::default();
        for (k, v) in s.key_val_iter() {
            match (k, v) {
                ("host", Some(v)) => {
                    res.host = Some(decode_percents_str(v)?);
                }
                ("bind", Some(v)) => {
                    res.bind = Some(decode_percents_str(v)?);
                }
                ("port", Some(v)) => {
                    res.port = Some(
                        decode_percents_str(v)?
                            .parse()
                            .map_err(|e| Error::Address(format!("Invalid port: {e}")))?,
                    );
                }
                ("family", Some(v)) => {
                    res.family = Some(decode_percents_str(v)?.as_ref().try_into()?);
                }
                _ => continue,
            }
        }
        Ok(res)
    }
}
