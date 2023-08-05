use std::{borrow::Cow, collections::HashSet, fmt};

use crate::{Error, Guid, Result};

use super::{
    percent::{decode_percents, decode_percents_str, Encodable},
    transport::Transport,
};

/// A bus address.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DBusAddr<'a> {
    pub(super) addr: Cow<'a, str>,
}

impl<'a> DBusAddr<'a> {
    /// The connection UUID (guid=) if any.
    pub fn guid(&self) -> Option<Result<Guid>> {
        match self.get_string("guid") {
            Some(Ok(v)) => Some(Guid::try_from(v.as_ref())),
            Some(Err(e)) => Some(Err(e)),
            _ => None,
        }
    }

    /// Transport connection details
    pub fn transport(&self) -> Result<Transport<'_>> {
        self.try_into()
    }

    fn validate(&self) -> Result<()> {
        self.transport()?;
        let mut set = HashSet::new();
        for (k, v) in self.key_val_iter() {
            if !set.insert(k) {
                return Err(Error::Address(format!("Duplicate key `{k}`")));
            }
            if let Some(v) = v {
                decode_percents(v)?;
            }
        }
        Ok(())
    }

    fn new<A: Into<Cow<'a, str>>>(addr: A) -> Result<Self> {
        let addr = addr.into();
        let addr = Self { addr };

        addr.validate()?;
        Ok(addr)
    }

    pub(super) fn key_val_iter(&'a self) -> KeyValIter<'a> {
        let mut split = self.addr.splitn(2, ':');
        // skip transport:..
        split.next();
        let kv = split.next().unwrap_or("");
        KeyValIter::new(kv)
    }

    fn get_string(&'a self, key: &str) -> Option<Result<Cow<'a, str>>> {
        for (k, v) in self.key_val_iter() {
            if key == k {
                return v.map(decode_percents_str);
            }
        }
        None
    }
}

impl DBusAddr<'_> {
    pub(crate) fn to_owned(&self) -> DBusAddr<'static> {
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
        let kv = KeyValFmt::new().add("guid", self.guid().and_then(|v| v.ok()));
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
