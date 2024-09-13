use std::marker::PhantomData;

use super::{DBusAddr, Error, KeyValFmt, KeyValFmtAdd, Result};

/// `systemd:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#transports-systemd>
#[derive(Debug, PartialEq, Eq)]
pub struct Systemd<'a> {
    // use a phantom lifetime for eventually future fields and consistency
    phantom: PhantomData<&'a ()>,
}

impl<'a> TryFrom<&'a DBusAddr<'a>> for Systemd<'a> {
    type Error = Error;

    fn try_from(_s: &'a DBusAddr<'a>) -> Result<Self> {
        Ok(Systemd {
            phantom: PhantomData,
        })
    }
}

impl KeyValFmtAdd for Systemd<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv
    }
}
