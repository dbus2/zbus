//! D-Bus address handling.
//!
//! Server addresses consist of a transport name followed by a colon, and then an optional,
//! comma-separated list of keys and values in the form key=value.
//!
//! # Miscellaneous and caveats on D-Bus addresses
//!
//! * Assumes values are UTF-8 encoded.
//!
//! * Duplicated keys are accepted, last pair wins.
//!
//! * Assumes that empty `key=val` is accepted, so `transport:,,guid=...` is valid.
//!
//! * Allows key only, so `transport:foo,bar` is ok.
//!
//! * Accept unknown keys and transports.
//!
//! See also:
//!
//! * [Server addresses] in the D-Bus specification.
//!
//! [Server addresses]: https://dbus.freedesktop.org/doc/dbus-specification.html#addresses

use std::{borrow::Cow, env, fmt};

#[cfg(all(unix, not(target_os = "macos")))]
use nix::unistd::Uid;

pub mod transport;

mod address_list;
pub use address_list::{DBusAddrList, DBusAddrListIter};

mod percent;
pub use percent::*;

#[cfg(test)]
mod tests;

/// Error returned when an address is invalid.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    UnknownTransport,
    MissingTransport,
    Encoding(String),
    DuplicateKey(String),
    MissingKey(String),
    MissingValue(String),
    InvalidValue(String),
    UnknownTcpFamily(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownTransport => write!(f, "Unsupported transport in address"),
            Error::MissingTransport => write!(f, "Missing transport in address"),
            Error::Encoding(e) => write!(f, "Encoding error: {e}"),
            Error::DuplicateKey(e) => write!(f, "Duplicate key: `{e}`"),
            Error::MissingKey(e) => write!(f, "Missing key: `{e}`"),
            Error::MissingValue(e) => write!(f, "Missing value for key: `{e}`"),
            Error::InvalidValue(e) => write!(f, "Invalid value for key: `{e}`"),
            Error::UnknownTcpFamily(e) => write!(f, "Unknown TCP address family: `{e}`"),
            Error::Other(e) => write!(f, "Other error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Get the address for session socket respecting the DBUS_SESSION_BUS_ADDRESS environment
/// variable. If we don't recognize the value (or it's not set) we fall back to
/// $XDG_RUNTIME_DIR/bus
pub fn session() -> Result<DBusAddrList<'static>> {
    match env::var("DBUS_SESSION_BUS_ADDRESS") {
        Ok(val) => DBusAddrList::try_from(val),
        _ => {
            #[cfg(windows)]
            {
                DBusAddrList::try_from("autolaunch:scope=*user;autolaunch:")
            }

            #[cfg(all(unix, not(target_os = "macos")))]
            {
                let runtime_dir = env::var("XDG_RUNTIME_DIR")
                    .unwrap_or_else(|_| format!("/run/user/{}", Uid::effective()));
                let path = format!("unix:path={runtime_dir}/bus");

                DBusAddrList::try_from(path)
            }

            #[cfg(target_os = "macos")]
            {
                DBusAddrList::try_from("launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET")
            }
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

/// A bus address.
///
/// Example:
/// ```
/// use zbus::DBusAddr;
///
/// let _: DBusAddr = "unix:path=/tmp/dbus.sock".try_into().unwrap();
/// ```
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DBusAddr<'a> {
    pub(super) addr: Cow<'a, str>,
}

impl<'a> DBusAddr<'a> {
    /// The connection GUID if any.
    pub fn guid(&self) -> Option<Cow<'_, str>> {
        self.get_string("guid").and_then(|res| res.ok())
    }

    /// Transport connection details
    pub fn transport(&self) -> Result<transport::Transport<'_>> {
        self.try_into()
    }

    pub(super) fn key_val_iter(&'a self) -> KeyValIter<'a> {
        let mut split = self.addr.splitn(2, ':');
        // skip transport:..
        split.next();
        let kv = split.next().unwrap_or("");
        KeyValIter::new(kv)
    }

    fn new<A: Into<Cow<'a, str>>>(addr: A) -> Result<Self> {
        let addr = addr.into();
        let addr = Self { addr };

        addr.validate()?;

        Ok(addr)
    }

    fn validate(&self) -> Result<()> {
        self.transport()?;
        for (k, v) in self.key_val_iter() {
            let v = match v {
                Some(v) => decode_percents(v)?,
                _ => Cow::from(b"" as &[_]),
            };
            if k == "guid" {
                validate_guid(v.as_ref())?;
            }
        }

        Ok(())
    }

    // the last key=val wins
    fn get_string(&'a self, key: &str) -> Option<Result<Cow<'a, str>>> {
        let mut val = None;
        for (k, v) in self.key_val_iter() {
            if key == k {
                val = v;
            }
        }
        val.map(decode_percents_str)
    }
}

fn validate_guid(value: &[u8]) -> Result<()> {
    if value.len() != 32 || value.iter().any(|&c| !c.is_ascii_hexdigit()) {
        return Err(Error::InvalidValue("guid".into()));
    }

    Ok(())
}

impl DBusAddr<'_> {
    pub fn to_owned(&self) -> DBusAddr<'static> {
        let addr = self.addr.to_string();
        DBusAddr { addr: addr.into() }
    }
}

impl<'a> TryFrom<String> for DBusAddr<'a> {
    type Error = Error;

    fn try_from(addr: String) -> Result<Self> {
        Self::new(addr)
    }
}

impl<'a> TryFrom<&'a str> for DBusAddr<'a> {
    type Error = Error;

    fn try_from(addr: &'a str) -> Result<Self> {
        Self::new(addr)
    }
}

impl fmt::Display for DBusAddr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kv = KeyValFmt::new().add("guid", self.guid());
        let t = self.transport().map_err(|_| fmt::Error)?;
        let kv = t.key_val_fmt_add(kv);
        write!(f, "{t}:{kv}")?;
        Ok(())
    }
}

pub(super) struct KeyValIter<'a> {
    data: &'a str,
    next_index: usize,
}

impl<'a> KeyValIter<'a> {
    fn new(data: &'a str) -> Self {
        KeyValIter {
            data,
            next_index: 0,
        }
    }
}

impl<'a> Iterator for KeyValIter<'a> {
    type Item = (&'a str, Option<&'a str>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.data.len() {
            return None;
        }

        let mut pair = &self.data[self.next_index..];
        if let Some(end) = pair.find(',') {
            pair = &pair[..end];
            self.next_index += end + 1;
        } else {
            self.next_index = self.data.len();
        }
        let mut split = pair.split('=');
        // SAFETY: first split always returns something
        let key = split.next().unwrap();

        Some((key, split.next()))
    }
}

pub(crate) trait KeyValFmtAdd {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b>;
}

pub(crate) struct KeyValFmt<'a> {
    fields: Vec<(Box<dyn fmt::Display + 'a>, Box<dyn Encodable + 'a>)>,
}

impl<'a> KeyValFmt<'a> {
    fn new() -> Self {
        Self { fields: vec![] }
    }

    pub(crate) fn add<K, V>(mut self, key: K, val: Option<V>) -> Self
    where
        K: fmt::Display + 'a,
        V: Encodable + 'a,
    {
        if let Some(val) = val {
            self.fields.push((Box::new(key), Box::new(val)));
        }

        self
    }
}

impl fmt::Display for KeyValFmt<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (k, v) in self.fields.iter() {
            if !first {
                write!(f, ",")?;
            }
            write!(f, "{k}=")?;
            v.encode(f)?;
            first = false;
        }

        Ok(())
    }
}

/// A trait for objects which can be converted or resolved to one or more [`DBusAddr`] values.
pub trait ToDBusAddrs<'a> {
    type Iter: Iterator<Item = Result<DBusAddr<'a>>>;

    fn to_dbus_addrs(&'a self) -> Self::Iter;
}

impl<'a> ToDBusAddrs<'a> for DBusAddr<'a> {
    type Iter = std::iter::Once<Result<DBusAddr<'a>>>;

    /// Get an iterator over the D-Bus addresses.
    fn to_dbus_addrs(&'a self) -> Self::Iter {
        std::iter::once(Ok(self.clone()))
    }
}

impl<'a> ToDBusAddrs<'a> for str {
    type Iter = std::iter::Once<Result<DBusAddr<'a>>>;

    fn to_dbus_addrs(&'a self) -> Self::Iter {
        std::iter::once(self.try_into())
    }
}

impl<'a> ToDBusAddrs<'a> for String {
    type Iter = std::iter::Once<Result<DBusAddr<'a>>>;

    fn to_dbus_addrs(&'a self) -> Self::Iter {
        std::iter::once(self.as_str().try_into())
    }
}

impl<'a> ToDBusAddrs<'a> for Vec<Result<DBusAddr<'_>>> {
    type Iter = std::iter::Cloned<std::slice::Iter<'a, Result<DBusAddr<'a>>>>;

    /// Get an iterator over the D-Bus addresses.
    fn to_dbus_addrs(&'a self) -> Self::Iter {
        self.iter().cloned()
    }
}
