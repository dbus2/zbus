use std::marker::PhantomData;

use super::{percent::decode_percents_str, Address, Error, KeyValFmt, KeyValFmtAdd, Result};

/// `vsock:` D-Bus transport.
#[derive(Debug, PartialEq, Eq)]
pub struct Vsock<'a> {
    // no cid means ANY
    cid: Option<u32>,
    // no port means ANY
    port: Option<u32>,
    // use a phantom lifetime for eventually future fields and consistency
    phantom: PhantomData<&'a ()>,
}

impl<'a> Vsock<'a> {
    /// The VSOCK port.
    pub fn port(&self) -> Option<u32> {
        self.port
    }

    /// The VSOCK CID.
    pub fn cid(&self) -> Option<u32> {
        self.cid
    }
}

impl<'a> TryFrom<&'a Address<'a>> for Vsock<'a> {
    type Error = Error;

    fn try_from(s: &'a Address<'a>) -> Result<Self> {
        let mut port = None;
        let mut cid = None;

        for (k, v) in s.key_val_iter() {
            match (k, v) {
                ("port", Some(v)) => {
                    port = Some(
                        decode_percents_str(v)?
                            .parse()
                            .map_err(|_| Error::InvalidValue(k.into()))?,
                    );
                }
                ("cid", Some(v)) => {
                    cid = Some(
                        decode_percents_str(v)?
                            .parse()
                            .map_err(|_| Error::InvalidValue(k.into()))?,
                    )
                }
                _ => continue,
            }
        }

        Ok(Vsock {
            port,
            cid,
            phantom: PhantomData,
        })
    }
}

impl KeyValFmtAdd for Vsock<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv.add("cid", self.cid()).add("port", self.port())
    }
}
