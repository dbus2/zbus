use std::{borrow::Cow, ffi::OsStr};

use super::{
    percent::{decode_percents, decode_percents_os_str, decode_percents_str, EncData, EncOsStr},
    Address, Error, KeyValFmt, Result, TransportImpl,
};

/// A sub-type of `unix:` transport.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnixAddrKind<'a> {
    /// Path of the unix domain socket.
    Path(Cow<'a, OsStr>),
    /// Directory in which a socket file with a random file name starting with 'dbus-' should be
    /// created by a server.
    Dir(Cow<'a, OsStr>),
    /// The same as "dir", except that on platforms with abstract sockets, a server may attempt to
    /// create an abstract socket whose name starts with this directory instead of a path-based
    /// socket.
    Tmpdir(Cow<'a, OsStr>),
    /// Unique string in the abstract namespace, often syntactically resembling a path but
    /// unconnected to the filesystem namespace
    Abstract(Cow<'a, [u8]>),
    /// Listen on $XDG_RUNTIME_DIR/bus.
    Runtime,
}

impl UnixAddrKind<'_> {
    fn fmt_key_val<'s: 'b, 'b>(&'s self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        match self {
            UnixAddrKind::Path(p) => kv.add("path", Some(EncOsStr(p))),
            UnixAddrKind::Dir(p) => kv.add("dir", Some(EncOsStr(p))),
            UnixAddrKind::Tmpdir(p) => kv.add("tmpdir", Some(EncOsStr(p))),
            UnixAddrKind::Abstract(p) => kv.add("abstract", Some(EncData(p))),
            UnixAddrKind::Runtime => kv.add("runtime", Some("yes")),
        }
    }
}

/// `unix:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#transports-unix-domain-sockets-addresses>
#[derive(Debug, PartialEq, Eq)]
pub struct Unix<'a> {
    kind: UnixAddrKind<'a>,
}

impl<'a> Unix<'a> {
    /// One of the various `unix:` addresses.
    pub fn kind(&self) -> &UnixAddrKind<'a> {
        &self.kind
    }
}

impl<'a> TransportImpl<'a> for Unix<'a> {
    fn for_address(s: &'a Address<'a>) -> Result<Self> {
        let mut kind = None;
        let mut iter = s.key_val_iter();
        for (k, v) in &mut iter {
            match k {
                "path" | "dir" | "tmpdir" => {
                    let v = v.ok_or_else(|| Error::MissingValue(k.into()))?;
                    let v = decode_percents_os_str(v)?;
                    kind = Some(match k {
                        "path" => UnixAddrKind::Path(v),
                        "dir" => UnixAddrKind::Dir(v),
                        "tmpdir" => UnixAddrKind::Tmpdir(v),
                        // can't happen, we matched those earlier
                        _ => panic!(),
                    });

                    break;
                }
                "abstract" => {
                    let v = v.ok_or_else(|| Error::MissingValue(k.into()))?;
                    let v = decode_percents(v)?;
                    kind = Some(UnixAddrKind::Abstract(v));

                    break;
                }
                "runtime" => {
                    let v = v.ok_or_else(|| Error::MissingValue(k.into()))?;
                    let v = decode_percents_str(v)?;
                    if v != "yes" {
                        return Err(Error::InvalidValue(k.into()));
                    }
                    kind = Some(UnixAddrKind::Runtime);

                    break;
                }
                _ => continue,
            }
        }
        let Some(kind) = kind else {
            return Err(Error::Other(
                "invalid `unix:` address, missing required key".into(),
            ));
        };
        for (k, _) in iter {
            match k {
                "path" | "dir" | "tmpdir" | "abstract" | "runtime" => {
                    return Err(Error::Other("invalid address, only one of `path` `dir` `tmpdir` `abstract` or `runtime` expected".into()));
                }
                _ => (),
            }
        }

        Ok(Unix { kind })
    }

    fn fmt_key_val<'s: 'b, 'b>(&'s self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        self.kind().fmt_key_val(kv)
    }
}
