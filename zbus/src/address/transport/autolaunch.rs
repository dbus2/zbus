use std::{borrow::Cow, fmt};

use super::{percent::decode_percents_str, Address, Error, KeyValFmt, KeyValFmtAdd, Result};

/// Scope of autolaunch (Windows only)
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AutolaunchScope<'a> {
    /// Limit session bus to dbus installation path.
    InstallPath,
    /// Limit session bus to the recent user.
    User,
    /// other values - specify dedicated session bus like "release", "debug" or other.
    Other(Cow<'a, str>),
}

impl fmt::Display for AutolaunchScope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InstallPath => write!(f, "*install-path"),
            Self::User => write!(f, "*user"),
            Self::Other(o) => write!(f, "{o}"),
        }
    }
}

impl<'a> TryFrom<Cow<'a, str>> for AutolaunchScope<'a> {
    type Error = Error;

    fn try_from(s: Cow<'a, str>) -> Result<Self> {
        match s.as_ref() {
            "*install-path" => Ok(Self::InstallPath),
            "*user" => Ok(Self::User),
            _ => Ok(Self::Other(s)),
        }
    }
}

/// `autolaunch:` D-Bus transport.
///
/// <https://dbus.freedesktop.org/doc/dbus-specification.html#meta-transports-autolaunch>
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Autolaunch<'a> {
    scope: Option<AutolaunchScope<'a>>,
}

impl<'a> Autolaunch<'a> {
    /// Scope of autolaunch (Windows only)
    pub fn scope(&self) -> Option<&AutolaunchScope<'a>> {
        self.scope.as_ref()
    }
}

impl<'a> TryFrom<&'a Address<'a>> for Autolaunch<'a> {
    type Error = Error;

    fn try_from(s: &'a Address<'a>) -> Result<Self> {
        let mut res = Autolaunch::default();

        for (k, v) in s.key_val_iter() {
            match (k, v) {
                ("scope", Some(v)) => {
                    res.scope = Some(decode_percents_str(v)?.try_into()?);
                }
                _ => continue,
            }
        }

        Ok(res)
    }
}

impl KeyValFmtAdd for Autolaunch<'_> {
    fn key_val_fmt_add<'a: 'b, 'b>(&'a self, kv: KeyValFmt<'b>) -> KeyValFmt<'b> {
        kv.add("scope", self.scope())
    }
}
