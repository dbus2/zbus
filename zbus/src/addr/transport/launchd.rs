use std::borrow::Cow;

use crate::{Error, Result};

use super::{percent::decode_percents_str, DBusAddr, KeyValFmt, KeyValFmtAdd};

/// `launchd:` D-Bus transport.
#[derive(Debug, PartialEq, Eq)]
pub struct Launchd<'a> {
    env: Cow<'a, str>,
}

impl<'a> Launchd<'a> {
    /// Environment variable used to get the path of the unix domain socket for the launchd created
    /// dbus-daemon.
    pub fn env(&self) -> &str {
        self.env.as_ref()
    }
}

impl<'a> TryFrom<&'a DBusAddr<'a>> for Launchd<'a> {
    type Error = Error;

    fn try_from(s: &'a DBusAddr<'a>) -> Result<Self> {
        for (k, v) in s.key_val_iter() {
            match (k, v) {
                ("env", Some(v)) => {
                    return Ok(Launchd {
                        env: decode_percents_str(v)?,
                    });
                }
                _ => continue,
            }
        }

        Err(Error::Address("Missing env=".into()))
    }
}

impl KeyValFmtAdd for Launchd<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv.add("env", Some(self.env()))
    }
}
