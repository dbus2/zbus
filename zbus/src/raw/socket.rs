use async_io::Async;
use std::{
    io,
    os::unix::{
        io::{AsRawFd, FromRawFd, RawFd},
        net::UnixStream,
    },
    task::{Context, Poll},
};

use nix::{
    cmsg_space,
    sys::{
        socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags},
        uio::IoVec,
    },
};

use crate::{utils::FDS_MAX, OwnedFd};

/// Trait representing some transport layer over which the DBus protocol can be used
///
/// The crate provides an implementation of it for std's `UnixStream` on unix platforms.
/// You will want to implement this trait to integrate zbus with a async-runtime-aware
/// implementation of the socket, for example.
pub trait Socket: std::fmt::Debug + AsRawFd + Send + Sync {
    /// Attempt to receive a message from the socket
    ///
    /// On success, returns the number of bytes read as well as a `Vec` containing
    /// any associated file descriptors.
    ///
    /// This method may return an error of kind `WouldBlock` instead if blocking for
    /// non-blocking sockets.
    fn recvmsg(&mut self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)>;

    /// Attempt to send a message on the socket
    ///
    /// On success, return the number of bytes written. There may be a partial write, in
    /// which case the caller is responsible of sending the remaining data by calling this
    /// method again until everything is written or it returns an error of kind `WouldBlock`.
    ///
    /// If at least one byte has been written, then all the provided file descriptors will
    /// have been sent as well, and should not be provided again in subsequent calls.
    /// If `Err(Errorkind::Wouldblock)`, none of the provided file descriptors were sent.
    ///
    /// If the underlying transport does not support transmitting file descriptors, this
    /// will return `Err(ErrorKind::InvalidInput)`.
    fn sendmsg(&mut self, buffer: &[u8], fds: &[RawFd]) -> io::Result<usize>;

    /// Close the socket.
    ///
    /// After this call, all reading and writing operations will fail.
    ///
    /// NB: All currently implementations don't block so this method will never return
    /// `Err(Errorkind::Wouldblock)`.
    fn close(&self) -> io::Result<()>;

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// The returned socket is a reference to the same stream that this object references. Both
    /// handles will read and write the same stream of data, and options set on one stream will be
    /// propagated to the other stream.
    ///
    /// This is useful for having two independent handles to the socket, one for writing only and
    /// the other for reading only.
    fn try_clone(&self) -> io::Result<Box<dyn Socket>>;

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// The returned socket is a reference to the same stream that this object references. Both
    /// handles will read and write the same stream of data, and options set on one stream will be
    /// propagated to the other stream.
    ///
    /// This is useful for having two independent handles to the socket, one for writing only and
    /// the other for reading only.
    fn clone_async_socket(&self) -> io::Result<Box<dyn AsyncSocket>>;
}

impl Socket for Box<dyn Socket> {
    fn recvmsg(&mut self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        (**self).recvmsg(buffer)
    }

    fn sendmsg(&mut self, buffer: &[u8], fds: &[RawFd]) -> io::Result<usize> {
        (**self).sendmsg(buffer, fds)
    }

    fn close(&self) -> io::Result<()> {
        (**self).close()
    }

    fn try_clone(&self) -> io::Result<Self> {
        (**self).try_clone()
    }

    fn clone_async_socket(&self) -> io::Result<Box<dyn AsyncSocket>> {
        (**self).clone_async_socket()
    }
}

impl AsRawFd for Box<dyn Socket> {
    fn as_raw_fd(&self) -> RawFd {
        (**self).as_raw_fd()
    }
}

impl Socket for UnixStream {
    fn recvmsg(&mut self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        let iov = [IoVec::from_mut_slice(buffer)];
        let mut cmsgspace = cmsg_space!([RawFd; FDS_MAX]);

        match recvmsg(
            self.as_raw_fd(),
            &iov,
            Some(&mut cmsgspace),
            MsgFlags::empty(),
        ) {
            Ok(msg) => {
                let mut fds = vec![];
                for cmsg in msg.cmsgs() {
                    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
                    if let ControlMessageOwned::ScmCreds(_) = cmsg {
                        continue;
                    }
                    if let ControlMessageOwned::ScmRights(fd) = cmsg {
                        fds.extend(fd.iter().map(|&f| unsafe { OwnedFd::from_raw_fd(f) }));
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "unexpected CMSG kind",
                        ));
                    }
                }
                Ok((msg.bytes, fds))
            }
            Err(nix::Error::Sys(e)) => Err(e.into()),
            _ => Err(io::Error::new(io::ErrorKind::Other, "unhandled nix error")),
        }
    }

    fn sendmsg(&mut self, buffer: &[u8], fds: &[RawFd]) -> io::Result<usize> {
        let cmsg = if !fds.is_empty() {
            vec![ControlMessage::ScmRights(fds)]
        } else {
            vec![]
        };
        let iov = [IoVec::from_slice(buffer)];
        match sendmsg(self.as_raw_fd(), &iov, &cmsg, MsgFlags::empty(), None) {
            // can it really happen?
            Ok(0) => Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "failed to write to buffer",
            )),
            Ok(n) => Ok(n),
            Err(nix::Error::Sys(e)) => Err(e.into()),
            _ => Err(io::Error::new(io::ErrorKind::Other, "unhandled nix error")),
        }
    }

    fn close(&self) -> io::Result<()> {
        self.shutdown(std::net::Shutdown::Both)
    }

    fn try_clone(&self) -> io::Result<Box<dyn Socket>> {
        Ok(Box::new(self.try_clone()?))
    }

    fn clone_async_socket(&self) -> io::Result<Box<dyn AsyncSocket>> {
        Ok(Box::new(Async::new(self.try_clone()?)?))
    }
}

/// Trait representing some transport layer over which the DBus protocol can be used
///
/// The crate provides an implementation of it for std's [`UnixStream`] on unix platforms.
/// You will want to implement this trait to integrate zbus with a async-runtime-aware
/// implementation of the socket, for example.
pub trait AsyncSocket: std::fmt::Debug + Send + Sync {
    /// Attempt to receive a message from the socket.
    ///
    /// On success, returns the number of bytes read as well as a `Vec` containing
    /// any associated file descriptors.
    fn poll_recvmsg(
        &mut self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, Vec<OwnedFd>)>>;

    /// Attempt to send a message on the socket
    ///
    /// On success, return the number of bytes written. There may be a partial write, in
    /// which case the caller is responsible of sending the remaining data by calling this
    /// method again until everything is written or it returns an error of kind `WouldBlock`.
    ///
    /// If at least one byte has been written, then all the provided file descriptors will
    /// have been sent as well, and should not be provided again in subsequent calls.
    ///
    /// If the underlying transport does not support transmitting file descriptors, this
    /// will return `Err(ErrorKind::InvalidInput)`.
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buffer: &[u8],
        fds: &[RawFd],
    ) -> Poll<io::Result<usize>>;

    /// Close the socket.
    ///
    /// After this call, it is valid for all reading and writing operations to fail.
    fn close(&self) -> io::Result<()>;

    /// Creates a new independently owned handle to the underlying socket.
    ///
    /// The returned socket is a reference to the same stream that this object references. Both
    /// handles will read and write the same stream of data, and options set on one stream will be
    /// propagated to the other stream.
    ///
    /// This is useful for having two independent handles to the socket, one for writing only and
    /// the other for reading only.
    fn try_clone(&self) -> io::Result<Box<dyn AsyncSocket>>;

    /// Return the raw file descriptor backing this transport, if any.
    ///
    /// This is used to back [zbus::azync::Connection::as_raw_fd] and related functions.
    fn as_raw_fd(&self) -> RawFd;
}

impl AsyncSocket for Box<dyn AsyncSocket> {
    fn poll_recvmsg(
        &mut self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, Vec<OwnedFd>)>> {
        (&mut **self).poll_recvmsg(cx, buf)
    }
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buffer: &[u8],
        fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        (&mut **self).poll_sendmsg(cx, buffer, fds)
    }
    fn close(&self) -> io::Result<()> {
        (&**self).close()
    }
    fn try_clone(&self) -> io::Result<Box<dyn AsyncSocket>> {
        (&**self).try_clone()
    }
    fn as_raw_fd(&self) -> RawFd {
        (&**self).as_raw_fd()
    }
}

impl<S> Socket for Async<S>
where
    S: Socket + AsRawFd,
{
    fn recvmsg(&mut self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        self.get_mut().recvmsg(buffer)
    }

    fn sendmsg(&mut self, buffer: &[u8], fds: &[RawFd]) -> io::Result<usize> {
        self.get_mut().sendmsg(buffer, fds)
    }

    fn close(&self) -> io::Result<()> {
        self.get_ref().close()
    }

    fn try_clone(&self) -> io::Result<Box<dyn Socket>> {
        Socket::try_clone(self.get_ref())
    }

    fn clone_async_socket(&self) -> io::Result<Box<dyn AsyncSocket>> {
        Socket::clone_async_socket(self.get_ref())
    }
}

impl AsyncSocket for Async<UnixStream> {
    fn poll_recvmsg(
        &mut self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, Vec<OwnedFd>)>> {
        let (len, fds) = loop {
            match self.get_mut().recvmsg(buf) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => match self.poll_readable(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(res) => res?,
                },
                v => break v?,
            }
        };
        Poll::Ready(Ok((len, fds)))
    }

    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buffer: &[u8],
        fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        loop {
            match self.get_mut().sendmsg(buffer, fds) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => match self.poll_writable(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(res) => res?,
                },
                v => return Poll::Ready(v),
            }
        }
    }

    fn close(&self) -> io::Result<()> {
        self.get_ref().close()
    }

    fn try_clone(&self) -> io::Result<Box<dyn AsyncSocket>> {
        Socket::clone_async_socket(self.get_ref())
    }

    fn as_raw_fd(&self) -> RawFd {
        self.get_ref().as_raw_fd()
    }
}
