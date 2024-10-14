use std::borrow::Cow;

use super::{
    percent::decode_percents_str, Address, Error, KeyValFmt, KeyValFmtAdd, Result, TransportImpl,
};

/// `launchd:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#transports-launchd>
#[derive(Debug, PartialEq, Eq)]
pub struct Launchd<'a> {
    env: Cow<'a, str>,
}

impl<'a> Launchd<'a> {
    /// Environment variable.
    ///
    /// Environment variable used to get the path of the unix domain socket for the launchd created
    /// dbus-daemon.
    pub fn env(&self) -> &str {
        self.env.as_ref()
    }
}

impl<'a> TransportImpl<'a> for Launchd<'a> {
    fn for_address(s: &'a Address<'a>) -> Result<Self> {
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

        Err(Error::MissingKey("env".into()))
    }
}

impl KeyValFmtAdd for Launchd<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv.add("env", Some(self.env()))
    }
}
