use std::{borrow::Cow, ffi::OsStr};

use super::{
    percent::{decode_percents_os_str, decode_percents_str, EncOsStr},
    tcp::TcpFamily,
    DBusAddr, Error, KeyValFmt, KeyValFmtAdd, Result,
};

/// `nonce-tcp:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#transports-nonce-tcp-sockets>
#[derive(Debug, Default, PartialEq, Eq)]
pub struct NonceTcp<'a> {
    host: Option<Cow<'a, str>>,
    bind: Option<Cow<'a, str>>,
    port: Option<u16>,
    family: Option<TcpFamily>,
    noncefile: Option<Cow<'a, OsStr>>,
}

impl<'a> NonceTcp<'a> {
    /// If set, the DNS name or IP address.
    pub fn host(&self) -> Option<&str> {
        self.host.as_ref().map(|v| v.as_ref())
    }

    /// If set, the listenable address.
    ///
    /// Used in a listenable address to configure the interface on which the server will listen:
    /// either the IP address of one of the local machine's interfaces (most commonly `127.0.0.1`),
    /// or a DNS name that resolves to one of those IP addresses, or `*` to listen on all interfaces
    /// simultaneously.
    pub fn bind(&self) -> Option<&str> {
        self.bind.as_ref().map(|v| v.as_ref())
    }

    /// If set, the TCP port.
    ///
    /// The TCP port the server will open. A zero value let the server choose a free port provided
    /// from the underlying operating system.
    pub fn port(&self) -> Option<u16> {
        self.port
    }

    /// If set, the type of socket family.
    pub fn family(&self) -> Option<TcpFamily> {
        self.family
    }

    /// If set, the nonce file location.
    ///
    /// File location containing the secret. This is only meaningful in connectable addresses.
    pub fn noncefile(&self) -> Option<&OsStr> {
        self.noncefile.as_ref().map(|v| v.as_ref())
    }
}

impl KeyValFmtAdd for NonceTcp<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv.add("host", self.host())
            .add("bind", self.bind())
            .add("port", self.port())
            .add("family", self.family())
            .add("noncefile", self.noncefile().map(EncOsStr))
    }
}

impl<'a> TryFrom<&'a DBusAddr<'a>> for NonceTcp<'a> {
    type Error = Error;

    fn try_from(s: &'a DBusAddr<'a>) -> Result<Self> {
        let mut res = NonceTcp::default();
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
                            .map_err(|_| Error::InvalidValue("port".into()))?,
                    );
                }
                ("family", Some(v)) => {
                    res.family = Some(decode_percents_str(v)?.as_ref().try_into()?);
                }
                ("noncefile", Some(v)) => {
                    res.noncefile = Some(decode_percents_os_str(v)?);
                }
                _ => continue,
            }
        }

        Ok(res)
    }
}
