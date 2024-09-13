use std::{borrow::Cow, ffi::OsStr, fmt};

use super::{
    percent::{decode_percents_os_str, decode_percents_str, EncOsStr},
    DBusAddr, Error, KeyValFmt, KeyValFmtAdd, Result,
};

#[derive(Debug, PartialEq, Eq)]
struct Argv(usize);

impl fmt::Display for Argv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.0;

        write!(f, "argv{n}")
    }
}

/// `unixexec:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#transports-exec>
#[derive(Debug, PartialEq, Eq)]
pub struct Unixexec<'a> {
    path: Cow<'a, OsStr>,
    argv: Vec<(usize, Cow<'a, str>)>,
}

impl<'a> Unixexec<'a> {
    /// Binary to execute.
    ///
    /// Path of the binary to execute, either an absolute path or a binary name that is searched for
    /// in the default search path of the OS. This corresponds to the first argument of execlp().
    /// This key is mandatory.
    pub fn path(&self) -> &OsStr {
        self.path.as_ref()
    }

    /// Arguments.
    ///
    /// Arguments to pass to the binary as `[(nth, arg),...]`.
    pub fn argv(&self) -> &[(usize, Cow<'a, str>)] {
        self.argv.as_ref()
    }
}

impl<'a> TryFrom<&'a DBusAddr<'a>> for Unixexec<'a> {
    type Error = Error;

    fn try_from(s: &'a DBusAddr<'a>) -> Result<Self> {
        let mut path = None;
        let mut argv = Vec::new();

        for (k, v) in s.key_val_iter() {
            match (k, v) {
                ("path", Some(v)) => {
                    path = Some(decode_percents_os_str(v)?);
                }
                (k, Some(v)) if k.starts_with("argv") => {
                    let n: usize = k[4..].parse().map_err(|_| Error::InvalidValue(k.into()))?;
                    let arg = decode_percents_str(v)?;
                    argv.push((n, arg));
                }
                _ => continue,
            }
        }

        let Some(path) = path else {
            return Err(Error::MissingKey("path".into()));
        };

        argv.sort_by_key(|(num, _)| *num);

        Ok(Self { path, argv })
    }
}

impl KeyValFmtAdd for Unixexec<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, mut kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv = kv.add("path", Some(EncOsStr(self.path())));
        for (n, arg) in self.argv() {
            kv = kv.add(Argv(*n), Some(arg));
        }

        kv
    }
}
