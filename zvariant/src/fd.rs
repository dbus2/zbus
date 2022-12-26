use serde::{ser::Error, Deserialize, Deserializer, Serialize, Serializer};
use static_assertions::assert_impl_all;
use std::{
    cmp::PartialEq,
    os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd},
    sync::Arc,
};

use crate::{Basic, EncodingFormat, Signature, Type};

/// A [`OwnedFd`](https://doc.rust-lang.org/std/os/fd/struct.OwnedFd.html) wrapper.
///
/// We wrap the `OwnedFd` type so that we can implement [`Serialize`] and [`Deserialize`] for it.
/// File descriptors are serialized in a special way and you need to use specific [serializer] and
/// [deserializer] API when file descriptors are or could be involved.
///
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [`Deserialize`]: https://docs.serde.rs/serde/de/trait.Deserialize.html
/// [deserializer]: fn.from_slice_fds.html
/// [serializer]: fn.to_bytes_fds.html
#[derive(Debug)]
pub struct Fd(OwnedFd);

assert_impl_all!(Fd: Send, Sync, Unpin);

impl Basic for Fd {
    const SIGNATURE_CHAR: char = 'h';
    const SIGNATURE_STR: &'static str = "h";

    fn alignment(format: EncodingFormat) -> usize {
        u32::alignment(format)
    }
}

impl Type for Fd {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked(Self::SIGNATURE_STR)
    }
}

impl<T: Into<OwnedFd>> From<T> for Fd {
    fn from(fd: T) -> Self {
        Self(fd.into())
    }
}

impl FromRawFd for Fd {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self(OwnedFd::from_raw_fd(fd))
    }
}

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl AsFd for Fd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

#[derive(Debug)]
pub struct FdWrapper(Arc<Fd>);

assert_impl_all!(FdWrapper: Send, Sync, Unpin);

impl FdWrapper {
    pub(crate) fn new(fd: Fd) -> Self {
        Self(Arc::new(fd))
    }

    pub fn into_inner(self) -> Arc<Fd> {
        self.0
    }
}

impl Clone for FdWrapper {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl PartialEq for FdWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_raw_fd() == other.0.as_raw_fd()
    }
}

impl Serialize for FdWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dupped = self.0 .0.try_clone().map_err(Error::custom)?;
        let raw_fd = dupped.into_raw_fd();
        serializer.serialize_i32(raw_fd)
    }
}

impl<'de> Deserialize<'de> for FdWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_fd = RawFd::deserialize(deserializer)?;
        let owned_fd = unsafe { OwnedFd::from_raw_fd(raw_fd) };

        Ok(Self(Arc::new(Fd::from(owned_fd))))
    }
}

impl Basic for FdWrapper {
    const SIGNATURE_CHAR: char = 'h';
    const SIGNATURE_STR: &'static str = "h";

    fn alignment(format: EncodingFormat) -> usize {
        u32::alignment(format)
    }
}

impl Type for FdWrapper {
    fn signature() -> Signature<'static> {
        Signature::from_static_str_unchecked(Self::SIGNATURE_STR)
    }
}

impl AsRawFd for FdWrapper {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
