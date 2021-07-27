// FIXME: Drop this when the deprecated `Basic::ALIGNMENT` is dropped in the next API break.
#![allow(deprecated)]

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use static_assertions::assert_impl_all;
use std::os::unix::io;

use crate::{Basic, EncodingFormat, Signature, Type};

/// A [`RawFd`](https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html) wrapper.
///
/// We wrap the `RawFd` type so that we can implement [`Serialize`] and [`Deserialize`] for it.
/// File descriptors are serialized in a special way and you need to use specific [serializer] and
/// [deserializer] API when file descriptors are or could be involved.
///
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [deserializer]: fn.from_slice_fds.html
/// [serializer]: fn.to_bytes_fds.html
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Fd(io::RawFd);

assert_impl_all!(Fd: Send, Sync, Unpin);

impl Basic for Fd {
    const SIGNATURE_CHAR: char = 'h';
    const SIGNATURE_STR: &'static str = "h";
    const ALIGNMENT: usize = <u32>::ALIGNMENT;

    fn alignment(format: EncodingFormat) -> usize {
        u32::alignment(format)
    }
}

impl io::AsRawFd for Fd {
    fn as_raw_fd(&self) -> io::RawFd {
        self.0
    }
}

impl Type for Fd {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked(Self::SIGNATURE_STR)
    }
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
