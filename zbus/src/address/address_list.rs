use std::{borrow::Cow, fmt};

use super::{DBusAddr, Error, Result, ToDBusAddrs};

/// A bus address list.
///
/// D-Bus addresses are `;`-separated.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DBusAddrList<'a> {
    addr: Cow<'a, str>,
}

impl<'a> ToDBusAddrs<'a> for DBusAddrList<'a> {
    type Iter = DBusAddrListIter<'a>;

    /// Get an iterator over the D-Bus addresses.
    fn to_dbus_addrs(&'a self) -> Self::Iter {
        DBusAddrListIter::new(self)
    }
}

impl<'a> Iterator for DBusAddrListIter<'a> {
    type Item = Result<DBusAddr<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.data.len() {
            return None;
        }

        let mut addr = &self.data[self.next_index..];
        if let Some(end) = addr.find(';') {
            addr = &addr[..end];
            self.next_index += end + 1;
        } else {
            self.next_index = self.data.len();
        }

        Some(DBusAddr::try_from(addr))
    }
}

/// An iterator of D-Bus addresses.
pub struct DBusAddrListIter<'a> {
    data: &'a str,
    next_index: usize,
}

impl<'a> DBusAddrListIter<'a> {
    fn new(list: &'a DBusAddrList<'_>) -> Self {
        Self {
            data: list.addr.as_ref(),
            next_index: 0,
        }
    }
}

impl<'a> TryFrom<String> for DBusAddrList<'a> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(Self {
            addr: Cow::Owned(value),
        })
    }
}

impl<'a> TryFrom<&'a str> for DBusAddrList<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        Ok(Self {
            addr: Cow::Borrowed(value),
        })
    }
}

impl fmt::Display for DBusAddrList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.addr)
    }
}
