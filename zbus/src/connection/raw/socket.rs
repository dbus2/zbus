#[cfg(not(feature = "tokio"))]
use async_io::Async;
#[cfg(unix)]
use std::io::{IoSlice, IoSliceMut};
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};
#[cfg(not(feature = "tokio"))]
use std::{net::TcpStream, sync::Arc, task::ready};

#[cfg(all(windows, not(feature = "tokio")))]
use uds_windows::UnixStream;

#[cfg(unix)]
use nix::{
    cmsg_space,
    sys::socket::{recvmsg, sendmsg, ControlMessage, ControlMessageOwned, MsgFlags, UnixAddr},
};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

#[cfg(all(unix, not(feature = "tokio")))]
use std::os::unix::net::UnixStream;

#[cfg(unix)]
use crate::{utils::FDS_MAX, OwnedFd};

#[cfg(unix)]
fn fd_recvmsg(fd: RawFd, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
    let mut iov = [IoSliceMut::new(buffer)];
    let mut cmsgspace = cmsg_space!([RawFd; FDS_MAX]);

    let msg = recvmsg::<UnixAddr>(fd, &mut iov, Some(&mut cmsgspace), MsgFlags::empty())?;
    if msg.bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::BrokenPipe,
            "failed to read from socket",
        ));
    }
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

#[cfg(unix)]
fn fd_sendmsg(fd: RawFd, buffer: &[u8], fds: &[RawFd]) -> io::Result<usize> {
    let cmsg = if !fds.is_empty() {
        vec![ControlMessage::ScmRights(fds)]
    } else {
        vec![]
    };
    let iov = [IoSlice::new(buffer)];
    match sendmsg::<UnixAddr>(fd, &iov, &cmsg, MsgFlags::empty(), None) {
        // can it really happen?
        Ok(0) => Err(io::Error::new(
            io::ErrorKind::WriteZero,
            "failed to write to buffer",
        )),
        Ok(n) => Ok(n),
        Err(e) => Err(e.into()),
    }
}

#[cfg(unix)]
fn get_unix_pid(fd: &impl AsRawFd) -> io::Result<Option<u32>> {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

        let fd = fd.as_raw_fd();
        getsockopt(fd, PeerCredentials)
            .map(|creds| Some(creds.pid() as _))
            .map_err(|e| e.into())
    }

    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    {
        let _ = fd;
        // FIXME
        Ok(None)
    }
}

#[cfg(unix)]
fn get_unix_uid(fd: &impl AsRawFd) -> io::Result<Option<u32>> {
    let fd = fd.as_raw_fd();

    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

        getsockopt(fd, PeerCredentials)
            .map(|creds| Some(creds.uid()))
            .map_err(|e| e.into())
    }

    #[cfg(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    {
        nix::unistd::getpeereid(fd)
            .map(|(uid, _)| Some(uid.into()))
            .map_err(|e| e.into())
    }
}

// Send 0 byte as a separate SCM_CREDS message.
#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
fn send_zero_byte(fd: &impl AsRawFd) -> io::Result<usize> {
    let fd = fd.as_raw_fd();
    let iov = [std::io::IoSlice::new(b"\0")];
    sendmsg::<()>(
        fd,
        &iov,
        &[ControlMessage::ScmCreds],
        MsgFlags::empty(),
        None,
    )
    .map_err(|e| e.into())
}

#[cfg(unix)]
type PollRecvmsg = Poll<io::Result<(usize, Vec<OwnedFd>)>>;

#[cfg(not(unix))]
type PollRecvmsg = Poll<io::Result<usize>>;

/// Trait representing some transport layer over which the DBus protocol can be used
///
/// In order to allow simultaneous reading and writing, this trait requires you to split the socket
/// into a read half and a write half. The reader and writer halves can be any types that implement
/// [`ReadHalf`] and [`WriteHalf`] respectively.
///
/// The crate provides implementations for `async_io` and `tokio`'s `UnixStream` wrappers if you
/// enable the corresponding crate features (`async_io` is enabled by default).
///
/// You can implement it manually to integrate with other runtimes or other dbus transports.  Feel
/// free to submit pull requests to add support for more runtimes to zbus itself so rust's orphan
/// rules don't force the use of a wrapper struct (and to avoid duplicating the work across many
/// projects).
pub trait Socket: std::fmt::Debug + Send + Sync {
    type ReadHalf: ReadHalf + std::fmt::Debug + Send + Sync;
    type WriteHalf: WriteHalf + std::fmt::Debug + Send + Sync;

    /// Split the socket into a read half and a write half.
    fn split(self) -> Split<Self::ReadHalf, Self::WriteHalf>
    where
        Self: Sized;
}

/// The read half of a socket.
///
/// See [`Socket`] for more details.
pub trait ReadHalf: std::fmt::Debug + Send + Sync + 'static {
    /// Attempt to receive a message from the socket.
    ///
    /// On success, returns the number of bytes read as well as a `Vec` containing
    /// any associated file descriptors.
    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg;

    /// Supports passing file descriptors.
    fn can_pass_unix_fd(&self) -> bool {
        true
    }

    /// Return the peer PID.
    fn peer_pid(&self) -> io::Result<Option<u32>> {
        Ok(None)
    }

    /// Return the peer process SID, if any.
    #[cfg(windows)]
    fn peer_sid(&self) -> Option<String> {
        None
    }

    /// Return the User ID, if any.
    #[cfg(unix)]
    fn uid(&self) -> io::Result<Option<u32>> {
        Ok(None)
    }
}

/// The write half of a socket.
///
/// See [`Socket`] for more details.
pub trait WriteHalf: std::fmt::Debug + Send + Sync + 'static {
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
        #[cfg(unix)] fds: &[RawFd],
    ) -> Poll<io::Result<usize>>;

    /// The dbus daemon on `freebsd` and `dragonfly` currently requires sending the zero byte
    /// as a separate message with SCM_CREDS, as part of the `EXTERNAL` authentication on unix
    /// sockets. This method is used by the authentication machinery in zbus to send this
    /// zero byte. Socket implementations based on unix sockets should implement this method.
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        Ok(None)
    }

    /// Close the socket.
    ///
    /// After this call, it is valid for all reading and writing operations to fail.
    fn close(&self) -> io::Result<()>;
}

/// A pair of socket read and write halves.
#[derive(Debug)]
pub struct Split<R: ReadHalf, W: WriteHalf> {
    read: R,
    write: W,
}

impl<R: ReadHalf, W: WriteHalf> Split<R, W> {
    /// Create a new boxed `Split` from `socket`.
    pub fn new_boxed<S: Socket<ReadHalf = R, WriteHalf = W>>(
        socket: S,
    ) -> Split<Box<dyn ReadHalf>, Box<dyn WriteHalf>> {
        let split = socket.split();

        Split {
            read: Box::new(split.read),
            write: Box::new(split.write),
        }
    }

    /// Reference to the read half.
    pub fn read(&self) -> &R {
        &self.read
    }

    /// Mutable reference to the read half.
    pub fn read_mut(&mut self) -> &mut R {
        &mut self.read
    }

    /// Reference to the write half.
    pub fn write(&self) -> &W {
        &self.write
    }

    /// Mutable reference to the write half.
    pub fn write_mut(&mut self) -> &mut W {
        &mut self.write
    }

    /// Take the read and write halves.
    pub fn take(self) -> (R, W) {
        (self.read, self.write)
    }
}

impl ReadHalf for Box<dyn ReadHalf> {
    fn can_pass_unix_fd(&self) -> bool {
        (**self).can_pass_unix_fd()
    }

    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg {
        (**self).poll_recvmsg(cx, buf)
    }

    fn peer_pid(&self) -> io::Result<Option<u32>> {
        (**self).peer_pid()
    }

    #[cfg(windows)]
    fn peer_sid(&self) -> Option<String> {
        (&**self).peer_sid()
    }

    #[cfg(unix)]
    fn uid(&self) -> io::Result<Option<u32>> {
        (**self).uid()
    }
}

impl WriteHalf for Box<dyn WriteHalf> {
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buffer: &[u8],
        #[cfg(unix)] fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        (**self).poll_sendmsg(
            cx,
            buffer,
            #[cfg(unix)]
            fds,
        )
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        (**self).send_zero_byte()
    }

    fn close(&self) -> io::Result<()> {
        (**self).close()
    }
}

#[cfg(not(feature = "tokio"))]
impl<T> Socket for Async<T>
where
    T: std::fmt::Debug + Send + Sync,
    Arc<Async<T>>: ReadHalf + WriteHalf,
{
    type ReadHalf = Arc<Async<T>>;
    type WriteHalf = Arc<Async<T>>;

    fn split(self) -> Split<Self::ReadHalf, Self::WriteHalf> {
        let arc = Arc::new(self);

        Split {
            read: arc.clone(),
            write: arc,
        }
    }
}

#[cfg(all(unix, not(feature = "tokio")))]
impl ReadHalf for Arc<Async<UnixStream>> {
    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg {
        let (len, fds) = loop {
            match fd_recvmsg(self.as_raw_fd(), buf) {
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

    fn peer_pid(&self) -> io::Result<Option<u32>> {
        get_unix_pid(self)
    }

    #[cfg(unix)]
    fn uid(&self) -> io::Result<Option<u32>> {
        get_unix_uid(self)
    }
}

#[cfg(all(unix, not(feature = "tokio")))]
impl WriteHalf for Arc<Async<UnixStream>> {
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buffer: &[u8],
        #[cfg(unix)] fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        loop {
            match fd_sendmsg(
                self.as_raw_fd(),
                buffer,
                #[cfg(unix)]
                fds,
            ) {
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
        self.get_ref().shutdown(std::net::Shutdown::Both)
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        send_zero_byte(self).map(Some)
    }
}

#[cfg(all(unix, feature = "tokio"))]
impl Socket for tokio::net::UnixStream {
    type ReadHalf = tokio::net::unix::OwnedReadHalf;
    type WriteHalf = tokio::net::unix::OwnedWriteHalf;

    fn split(self) -> Split<Self::ReadHalf, Self::WriteHalf> {
        let (read, write) = self.into_split();

        Split { read, write }
    }
}

#[cfg(all(unix, feature = "tokio"))]
impl ReadHalf for tokio::net::unix::OwnedReadHalf {
    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg {
        let stream = self.as_ref();
        loop {
            match stream.try_io(tokio::io::Interest::READABLE, || {
                // We use own custom function for reading because we need to receive file
                // descriptors too.
                fd_recvmsg(stream.as_raw_fd(), buf)
            }) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => match stream.poll_read_ready(cx)
                {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(res) => res?,
                },
                v => return Poll::Ready(v),
            }
        }
    }

    fn peer_pid(&self) -> io::Result<Option<u32>> {
        get_unix_pid(self.as_ref())
    }

    #[cfg(unix)]
    fn uid(&self) -> io::Result<Option<u32>> {
        get_unix_uid(self.as_ref())
    }
}

#[cfg(all(unix, feature = "tokio"))]
impl WriteHalf for tokio::net::unix::OwnedWriteHalf {
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buffer: &[u8],
        #[cfg(unix)] fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        let stream = self.as_ref();
        loop {
            match stream.try_io(tokio::io::Interest::WRITABLE, || {
                fd_sendmsg(
                    stream.as_raw_fd(),
                    buffer,
                    #[cfg(unix)]
                    fds,
                )
            }) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    match stream.poll_write_ready(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(res) => res?,
                    }
                }
                v => return Poll::Ready(v),
            }
        }
    }

    fn close(&self) -> io::Result<()> {
        // FIXME: This should call `tokio::net::UnixStream::poll_shutdown` but this method is not
        // async-friendly. At the next API break, we should fix this.
        Ok(())
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        send_zero_byte(self.as_ref()).map(Some)
    }
}

#[cfg(all(windows, not(feature = "tokio")))]
impl ReadHalf for Arc<Async<UnixStream>> {
    fn can_pass_unix_fd(&self) -> bool {
        false
    }

    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg {
        futures_util::AsyncRead::poll_read(Pin::new(&mut self.as_ref()), cx, buf)
    }

    #[cfg(windows)]
    fn peer_sid(&self) -> Option<String> {
        use crate::win32::ProcessToken;

        if let Ok(Some(pid)) = self.peer_pid() {
            if let Ok(process_token) =
                ProcessToken::open(if pid != 0 { Some(pid as _) } else { None })
            {
                return process_token.sid().ok();
            }
        }

        None
    }

    fn peer_pid(&self) -> io::Result<Option<u32>> {
        #[cfg(windows)]
        {
            use crate::win32::unix_stream_get_peer_pid;

            Ok(Some(unix_stream_get_peer_pid(&self.get_ref())? as _))
        }

        #[cfg(unix)]
        get_unix_pid(self)
    }

    #[cfg(unix)]
    fn uid(&self) -> io::Result<Option<u32>> {
        get_unix_uid(self)
    }
}

#[cfg(all(windows, not(feature = "tokio")))]
impl WriteHalf for Arc<Async<UnixStream>> {
    fn poll_sendmsg(&mut self, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        futures_util::AsyncWrite::poll_write(Pin::new(&mut self.as_ref()), cx, buf)
    }

    fn close(&self) -> io::Result<()> {
        self.get_ref().shutdown(std::net::Shutdown::Both)
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        send_zero_byte(self).map(Some)
    }
}

#[cfg(not(feature = "tokio"))]
impl ReadHalf for Arc<Async<TcpStream>> {
    fn can_pass_unix_fd(&self) -> bool {
        false
    }

    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg {
        match ready!(futures_util::AsyncRead::poll_read(
            Pin::new(&mut self.as_ref()),
            cx,
            buf,
        )) {
            Err(e) => Poll::Ready(Err(e)),
            Ok(len) => {
                #[cfg(unix)]
                let ret = (len, vec![]);
                #[cfg(not(unix))]
                let ret = len;
                Poll::Ready(Ok(ret))
            }
        }
    }

    #[cfg(windows)]
    fn peer_sid(&self) -> Option<String> {
        use crate::win32::{tcp_stream_get_peer_pid, ProcessToken};

        if let Ok(pid) = tcp_stream_get_peer_pid(&self.get_ref()) {
            if let Ok(process_token) = ProcessToken::open(if pid != 0 { Some(pid) } else { None }) {
                return process_token.sid().ok();
            }
        }

        None
    }
}

#[cfg(not(feature = "tokio"))]
impl WriteHalf for Arc<Async<TcpStream>> {
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buf: &[u8],
        #[cfg(unix)] fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        #[cfg(unix)]
        if !fds.is_empty() {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a tcp stream",
            )));
        }

        futures_util::AsyncWrite::poll_write(Pin::new(&mut self.as_ref()), cx, buf)
    }

    fn close(&self) -> io::Result<()> {
        self.get_ref().shutdown(std::net::Shutdown::Both)
    }
}

#[cfg(feature = "tokio")]
impl Socket for tokio::net::TcpStream {
    type ReadHalf = tokio::net::tcp::OwnedReadHalf;
    type WriteHalf = tokio::net::tcp::OwnedWriteHalf;

    fn split(self) -> Split<Self::ReadHalf, Self::WriteHalf> {
        let (read, write) = self.into_split();

        Split { read, write }
    }
}

#[cfg(feature = "tokio")]
impl ReadHalf for tokio::net::tcp::OwnedReadHalf {
    fn can_pass_unix_fd(&self) -> bool {
        false
    }

    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg {
        use tokio::io::{AsyncRead, ReadBuf};

        let mut read_buf = ReadBuf::new(buf);
        Pin::new(self).poll_read(cx, &mut read_buf).map(|res| {
            res.map(|_| {
                let ret = read_buf.filled().len();
                #[cfg(unix)]
                let ret = (ret, vec![]);

                ret
            })
        })
    }

    #[cfg(windows)]
    fn peer_sid(&self) -> Option<String> {
        use crate::win32::{socket_addr_get_pid, ProcessToken};

        let peer_addr = match self.peer_addr() {
            Ok(addr) => addr,
            Err(_) => return None,
        };

        if let Ok(pid) = socket_addr_get_pid(&peer_addr) {
            if let Ok(process_token) = ProcessToken::open(if pid != 0 { Some(pid) } else { None }) {
                return process_token.sid().ok();
            }
        }

        None
    }
}

#[cfg(feature = "tokio")]
impl WriteHalf for tokio::net::tcp::OwnedWriteHalf {
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buf: &[u8],
        #[cfg(unix)] fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        use tokio::io::AsyncWrite;

        #[cfg(unix)]
        if !fds.is_empty() {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a tcp stream",
            )));
        }

        Pin::new(self).poll_write(cx, buf)
    }

    fn close(&self) -> io::Result<()> {
        // FIXME: This should call `tokio::net::TcpStream::poll_shutdown` but this method is not
        // async-friendly. At the next API break, we should fix this.
        Ok(())
    }
}

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
impl ReadHalf for Arc<Async<vsock::VsockStream>> {
    fn can_pass_unix_fd(&self) -> bool {
        false
    }

    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg {
        match ready!(futures_util::AsyncRead::poll_read(
            Pin::new(&mut self.as_ref()),
            cx,
            buf,
        )) {
            Err(e) => return Poll::Ready(Err(e)),
            Ok(len) => {
                #[cfg(unix)]
                let ret = (len, vec![]);
                #[cfg(not(unix))]
                let ret = len;
                Poll::Ready(Ok(ret))
            }
        }
    }
}

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
impl WriteHalf for Arc<Async<vsock::VsockStream>> {
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buf: &[u8],
        #[cfg(unix)] fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        #[cfg(unix)]
        if !fds.is_empty() {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a tcp stream",
            )));
        }

        futures_util::AsyncWrite::poll_write(Pin::new(&mut self.as_ref()), cx, buf)
    }

    fn close(&self) -> io::Result<()> {
        self.get_ref().shutdown(std::net::Shutdown::Both)
    }
}

#[cfg(feature = "tokio-vsock")]
impl Socket for tokio_vsock::VsockStream {
    type ReadHalf = tokio_vsock::ReadHalf;
    type WriteHalf = tokio_vsock::WriteHalf;

    fn split(self) -> Split<Self::ReadHalf, Self::WriteHalf> {
        let (read, write) = self.split();

        Split { read, write }
    }
}

#[cfg(feature = "tokio-vsock")]
impl ReadHalf for tokio_vsock::ReadHalf {
    fn can_pass_unix_fd(&self) -> bool {
        false
    }

    fn poll_recvmsg(&mut self, cx: &mut Context<'_>, buf: &mut [u8]) -> PollRecvmsg {
        use tokio::io::{AsyncRead, ReadBuf};

        let mut read_buf = ReadBuf::new(buf);
        Pin::new(self).poll_read(cx, &mut read_buf).map(|res| {
            res.map(|_| {
                let ret = read_buf.filled().len();
                #[cfg(unix)]
                let ret = (ret, vec![]);

                ret
            })
        })
    }
}

#[cfg(feature = "tokio-vsock")]
impl WriteHalf for tokio_vsock::WriteHalf {
    fn poll_sendmsg(
        &mut self,
        cx: &mut Context<'_>,
        buf: &[u8],
        #[cfg(unix)] fds: &[RawFd],
    ) -> Poll<io::Result<usize>> {
        use tokio::io::AsyncWrite;

        #[cfg(unix)]
        if !fds.is_empty() {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a tcp stream",
            )));
        }

        Pin::new(self).poll_write(cx, buf)
    }

    fn close(&self) -> io::Result<()> {
        // FIXME: This should be:
        //
        // tokio::io::AsyncWriteExt::shutdown(self);
        //
        // but that requires this call to be async. At the next API break, we should fix this.
        Ok(())
    }
}
