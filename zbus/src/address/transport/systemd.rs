use std::marker::PhantomData;

use super::{Address, KeyValFmt, Result, TransportImpl};

/// `systemd:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#transports-systemd>
#[derive(Debug, PartialEq, Eq)]
pub struct Systemd<'a> {
    // use a phantom lifetime for eventually future fields and consistency
    phantom: PhantomData<&'a ()>,
}

impl<'a> Systemd<'a> {
    /// Convert into owned version, with 'static lifetime.
    pub fn into_owned(&self) -> Systemd<'static> {
        Systemd {
            phantom: PhantomData,
        }
    }
}

impl<'a> TransportImpl<'a> for Systemd<'a> {
    fn for_address(_addr: &'a Address<'a>) -> Result<Self> {
        Ok(Systemd {
            phantom: PhantomData,
        })
    }

    fn fmt_key_val<'s: 'b, 'b>(&'s self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv
    }
}
