use std::mem::forget;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct OwnedFd {
    inner: RawFd,
}

impl FromRawFd for OwnedFd {
    unsafe fn from_raw_fd(fd: RawFd) -> OwnedFd {
        OwnedFd { inner: fd }
    }
}

impl IntoRawFd for OwnedFd {
    fn into_raw_fd(self) -> RawFd {
        let v = self.inner;
        forget(self);
        v
    }
}

impl Drop for OwnedFd {
    fn drop(&mut self) {
        let _ = nix::unistd::close(self.inner);
    }
}
