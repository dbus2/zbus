use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};

#[derive(Debug)]
pub(super) enum Fd<'f> {
    Borrowed(BorrowedFd<'f>),
    Owned(OwnedFd),
}

impl<'f> Fd<'f> {
    pub fn borrow(&self) -> BorrowedFd<'_> {
        match self {
            Self::Borrowed(fd) => fd.as_fd(),
            Self::Owned(fd) => fd.as_fd(),
        }
    }
}

impl<'f> From<BorrowedFd<'f>> for Fd<'f> {
    fn from(fd: BorrowedFd<'f>) -> Self {
        Self::Borrowed(fd)
    }
}

impl From<OwnedFd> for Fd<'static> {
    fn from(fd: OwnedFd) -> Self {
        Self::Owned(fd)
    }
}

impl AsRawFd for Fd<'_> {
    fn as_raw_fd(&self) -> RawFd {
        self.borrow().as_raw_fd()
    }
}

impl AsFd for Fd<'_> {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.borrow()
    }
}
