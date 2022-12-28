use io_lifetimes::AsFd;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use static_assertions::assert_impl_all;
use std::cmp::PartialEq;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

use crate::{Basic, EncodingFormat, Signature, Type};

/// A [`BorrowedFd`](https://doc.rust-lang.org/std/os/fd/struct.BorrowedFd.html) wrapper.
///
/// See also `OwnedFd` if you need a wrapper that takes ownership of the file.
///
/// We wrap the `BorrowedFd` type so that we can implement [`Serialize`] for it.
/// File descriptors are serialized in a special way and you need to use specific [serializer] and
/// API when file descriptors are or could be involved.
///
/// [`Serialize`]: https://docs.serde.rs/serde/trait.Serialize.html
/// [serializer]: fn.to_bytes_fds.html
#[derive(Debug, Clone, Copy)]
pub struct BorrowedFd<'f>(io_lifetimes::BorrowedFd<'f>);

macro_rules! fd_impl {
    ($i:ident  $(<$a:lifetime>)? ) => {
        impl $(<$a>)? Basic for $i $(<$a>)? {
            const SIGNATURE_CHAR: char = 'h';
            const SIGNATURE_STR: &'static str = "h";

            fn alignment(format: EncodingFormat) -> usize {
                u32::alignment(format)
            }
        }

        impl $(<$a>)? Type for $i $(<$a>)? {
            fn signature() -> Signature<'static> {
                Signature::from_static_str_unchecked(Self::SIGNATURE_STR)
            }
        }
    };
}

assert_impl_all!(BorrowedFd<'_>: Send, Sync, Unpin);

fd_impl!(BorrowedFd<'a>);

impl<'f, T: AsFd + 'f> From<&'f T> for BorrowedFd<'f> {
    fn from(fd: &'f T) -> Self {
        BorrowedFd(fd.as_fd())
    }
}

impl<'f> Serialize for BorrowedFd<'f> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0.as_fd().as_raw_fd())
    }
}

impl<'de> Deserialize<'de> for BorrowedFd<'static> {
    /// Deserialize into an owned fd, the underlying descriptor is duplicated.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // FIXME This is super unsafe, we can create an invalid BorrowedFd for
        // any integer. Even if the fd is valid, it could be closed from
        // somewhere else.
        let raw_fd = i32::deserialize(deserializer)?;

        Ok(unsafe { BorrowedFd::borrow_raw(raw_fd) })
    }
}

// FIXME Having this trait is very much not ideal, but it is needed to implement
// PartialEq for crate::Value.
impl<'f> PartialEq for BorrowedFd<'f> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_raw_fd() == other.0.as_raw_fd()
    }
}

impl<'f> AsFd for BorrowedFd<'f> {
    fn as_fd(&self) -> io_lifetimes::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl<'a> AsRawFd for BorrowedFd<'a> {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl<'f> BorrowedFd<'f> {
    /// Return a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value `-1`.
    pub const unsafe fn borrow_raw(fd: RawFd) -> Self {
        Self(io_lifetimes::BorrowedFd::borrow_raw(fd))
    }

    pub fn try_clone_to_owned(&self) -> std::io::Result<OwnedFd> {
        let inner = self.0.try_clone_to_owned()?;

        Ok(OwnedFd(inner))
    }
}

/// A [`OwnedFd`](https://doc.rust-lang.org/std/os/fd/struct.OwnedFd.html) wrapper.
///
/// See also [`BorrowedFd`]. This type owns the file and will close it on drop.
/// On deserialize, it will duplicate the file descriptor.
#[derive(Debug)]
pub struct OwnedFd(io_lifetimes::OwnedFd);

assert_impl_all!(OwnedFd: Send, Sync, Unpin);

fd_impl!(OwnedFd);

impl Serialize for OwnedFd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0.as_raw_fd())
    }
}

impl<'de> Deserialize<'de> for OwnedFd {
    /// Deserialize into an owned fd, the underlying descriptor is duplicated.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let fd = unsafe { BorrowedFd::borrow_raw(i32::deserialize(deserializer)?) };
        // TODO Is this duplication needed?
        //
        // We duplicate the descriptor. This catches if the descriptor is
        // invalid.
        let dup_fd = fd.try_clone_to_owned().map_err(|err| {
            let msg = format!("Could not clone the fd: {err:?}");
            serde::de::Error::custom(msg)
        })?;

        Ok(dup_fd)
    }
}

impl AsFd for OwnedFd {
    fn as_fd(&self) -> io_lifetimes::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl From<io_lifetimes::OwnedFd> for OwnedFd {
    fn from(fd: io_lifetimes::OwnedFd) -> Self {
        Self(fd)
    }
}

impl From<OwnedFd> for io_lifetimes::OwnedFd {
    fn from(fd: OwnedFd) -> io_lifetimes::OwnedFd {
        fd.0
    }
}

impl FromRawFd for OwnedFd {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        io_lifetimes::OwnedFd::from_raw_fd(fd).into()
    }
}

impl AsRawFd for OwnedFd {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl IntoRawFd for OwnedFd {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl OwnedFd {
    /// Clones the file descriptor
    pub fn try_clone(&self) -> Result<Self, std::io::Error> {
        self.0.try_clone().map(Into::into)
    }
}
