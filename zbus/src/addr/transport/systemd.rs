use std::{fmt, marker::PhantomData};

use crate::{Error, Result};

use super::{DBusAddr, KeyValFmt, KeyValFmtAdd};

/// `systemd:` D-Bus transport.
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

impl<'a> fmt::Display for Systemd<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl KeyValFmtAdd for Systemd<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv
    }
}
