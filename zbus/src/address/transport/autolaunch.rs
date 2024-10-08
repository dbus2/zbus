use std::marker::PhantomData;
#[cfg(target_os = "windows")]
use std::{borrow::Cow, fmt};

#[cfg(target_os = "windows")]
use super::percent::decode_percents_str;
use super::{Address, Error, KeyValFmt, KeyValFmtAdd, Result};

/// Scope of autolaunch (Windows only)
#[cfg(target_os = "windows")]
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

#[cfg(target_os = "windows")]
impl fmt::Display for AutolaunchScope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InstallPath => write!(f, "*install-path"),
            Self::User => write!(f, "*user"),
            Self::Other(o) => write!(f, "{o}"),
        }
    }
}

#[cfg(target_os = "windows")]
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
    #[cfg(target_os = "windows")]
    scope: Option<AutolaunchScope<'a>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Autolaunch<'a> {
    #[cfg(target_os = "windows")]
    /// Scope of autolaunch (Windows only)
    pub fn scope(&self) -> Option<&AutolaunchScope<'a>> {
        self.scope.as_ref()
    }
}

impl<'a> TryFrom<&'a Address<'a>> for Autolaunch<'a> {
    type Error = Error;

    fn try_from(s: &'a Address<'a>) -> Result<Self> {
        #[allow(unused_mut)]
        let mut res = Autolaunch::default();

        for (k, v) in s.key_val_iter() {
            match (k, v) {
                #[cfg(target_os = "windows")]
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
        #[cfg(target_os = "windows")]
        let kv = kv.add("scope", self.scope());
        kv
    }
}
