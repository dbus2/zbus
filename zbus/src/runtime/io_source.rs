//! The handle a runtime registers for readiness.

#[cfg(any(unix, target_os = "wasi"))]
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, AsSocket, BorrowedSocket, OwnedSocket, RawSocket};
use std::sync::Arc;

/// A socket or pipe that zbus owns and a runtime watches for readiness.
///
/// It is a shared owner: zbus keeps one clone for the I/O it performs itself and the runtime's
/// registration keeps whatever it needs. The descriptor stays open as long as either lives.
#[derive(Clone, Debug)]
pub struct IoSource(Arc<Owned>);

#[cfg(any(unix, target_os = "wasi"))]
type Owned = OwnedFd;
#[cfg(windows)]
type Owned = OwnedSocket;

impl IoSource {
    pub(crate) fn new(owned: Owned) -> Self {
        Self(Arc::new(owned))
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl From<OwnedFd> for IoSource {
    fn from(owned: OwnedFd) -> Self {
        Self::new(owned)
    }
}

#[cfg(windows)]
impl From<OwnedSocket> for IoSource {
    fn from(owned: OwnedSocket) -> Self {
        Self::new(owned)
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsFd for IoSource {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsRawFd for IoSource {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

#[cfg(windows)]
impl AsSocket for IoSource {
    fn as_socket(&self) -> BorrowedSocket<'_> {
        self.0.as_socket()
    }
}

#[cfg(windows)]
impl AsRawSocket for IoSource {
    fn as_raw_socket(&self) -> RawSocket {
        self.0.as_raw_socket()
    }
}

/// The readiness an I/O operation waits for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Interest {
    /// The source has bytes to be read, or has reached its end.
    Readable,
    /// The source has room for bytes to be written, or a connection under way has settled.
    Writable,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    fn owned() -> Owned {
        std::os::unix::net::UnixStream::pair().unwrap().0.into()
    }

    #[cfg(windows)]
    fn owned() -> Owned {
        std::net::TcpListener::bind("127.0.0.1:0").unwrap().into()
    }

    #[test]
    fn clone_keeps_the_descriptor_open() {
        let source = IoSource::new(owned());
        let clone = source.clone();
        drop(source);

        #[cfg(unix)]
        let _: BorrowedFd<'_> = clone.as_fd();
        #[cfg(windows)]
        let _: BorrowedSocket<'_> = clone.as_socket();
    }
}
