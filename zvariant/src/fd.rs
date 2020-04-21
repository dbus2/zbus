use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::os::unix::io;

use crate::Basic;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Fd(io::RawFd);

impl Basic for Fd {
    const SIGNATURE_CHAR: char = 'h';
    const SIGNATURE_STR: &'static str = "h";
    const ALIGNMENT: usize = <u32>::ALIGNMENT;
}

impl Serialize for Fd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

impl<'de> Deserialize<'de> for Fd {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Fd(i32::deserialize(deserializer)?))
    }
}

impl From<io::RawFd> for Fd {
    fn from(value: io::RawFd) -> Self {
        Self(value)
    }
}

impl<T> From<&T> for Fd
where
    T: io::AsRawFd,
{
    fn from(t: &T) -> Self {
        Self(t.as_raw_fd())
    }
}

impl std::fmt::Display for Fd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
