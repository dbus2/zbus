#[cfg(not(feature = "tokio"))]
use async_io::Async;
use std::io;
#[cfg(unix)]
use std::{
    future::poll_fn,
    io::{IoSlice, IoSliceMut},
    task::Poll,
};
#[cfg(not(feature = "tokio"))]
use std::{net::TcpStream, sync::Arc};

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

use crate::fdo::ConnectionCredentials;
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
async fn get_unix_peer_creds(fd: &impl AsRawFd) -> io::Result<ConnectionCredentials> {
    let fd = fd.as_raw_fd();
    // FIXME: Is it likely enough for sending of 1 byte to block, to justify a task (possibly
    // launching a thread in turn)?
    crate::Task::spawn_blocking(move || get_unix_peer_creds_blocking(fd), "peer credentials").await
}

#[cfg(unix)]
fn get_unix_peer_creds_blocking(fd: RawFd) -> io::Result<ConnectionCredentials> {
    #[cfg(any(target_os = "android", target_os = "linux"))]
    {
        use nix::sys::socket::{getsockopt, sockopt::PeerCredentials};

        let fd = fd.as_raw_fd();
        getsockopt(fd, PeerCredentials)
            .map(|creds| {
                ConnectionCredentials::default()
                    .set_process_id(creds.pid() as _)
                    .set_unix_user_id(creds.uid())
            })
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
        let fd = fd.as_raw_fd();
        let uid = nix::unistd::getpeereid(fd).map(|(uid, _)| uid.into())?;
        // FIXME: Handle pid fetching too.
        Ok(ConnectionCredentials::default().set_unix_user_id(uid))
    }
}

// Send 0 byte as a separate SCM_CREDS message.
#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
async fn send_zero_byte(fd: &impl AsRawFd) -> io::Result<usize> {
    let fd = fd.as_raw_fd();
    crate::Task::spawn_blocking(move || send_zero_byte_blocking(fd), "send zero byte").await
}

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
fn send_zero_byte_blocking(fd: RawFd) -> io::Result<usize> {
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
type RecvmsgResult = io::Result<(usize, Vec<OwnedFd>)>;

#[cfg(not(unix))]
type RecvmsgResult = io::Result<usize>;

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
#[async_trait::async_trait]
pub trait ReadHalf: std::fmt::Debug + Send + Sync + 'static {
    /// Attempt to receive a message from the socket.
    ///
    /// On success, returns the number of bytes read as well as a `Vec` containing
    /// any associated file descriptors.
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult;

    /// Supports passing file descriptors.
    ///
    /// Default implementation returns `false`.
    fn can_pass_unix_fd(&self) -> bool {
        false
    }

    /// Return the peer credentials.
    async fn peer_credentials(&self) -> io::Result<ConnectionCredentials> {
        Ok(ConnectionCredentials::default())
    }
}

/// The write half of a socket.
///
/// See [`Socket`] for more details.
#[async_trait::async_trait]
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
    async fn sendmsg(&mut self, buffer: &[u8], #[cfg(unix)] fds: &[RawFd]) -> io::Result<usize>;

    /// The dbus daemon on `freebsd` and `dragonfly` currently requires sending the zero byte
    /// as a separate message with SCM_CREDS, as part of the `EXTERNAL` authentication on unix
    /// sockets. This method is used by the authentication machinery in zbus to send this
    /// zero byte. Socket implementations based on unix sockets should implement this method.
    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    async fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        Ok(None)
    }

    /// Close the socket.
    ///
    /// After this call, it is valid for all reading and writing operations to fail.
    async fn close(&mut self) -> io::Result<()>;
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

#[async_trait::async_trait]
impl ReadHalf for Box<dyn ReadHalf> {
    fn can_pass_unix_fd(&self) -> bool {
        (**self).can_pass_unix_fd()
    }

    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        (**self).recvmsg(buf).await
    }

    async fn peer_credentials(&self) -> io::Result<ConnectionCredentials> {
        (**self).peer_credentials().await
    }
}

#[async_trait::async_trait]
impl WriteHalf for Box<dyn WriteHalf> {
    async fn sendmsg(&mut self, buffer: &[u8], #[cfg(unix)] fds: &[RawFd]) -> io::Result<usize> {
        (**self)
            .sendmsg(
                buffer,
                #[cfg(unix)]
                fds,
            )
            .await
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    async fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        (**self).send_zero_byte().await
    }

    async fn close(&mut self) -> io::Result<()> {
        (**self).close().await
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
#[async_trait::async_trait]
impl ReadHalf for Arc<Async<UnixStream>> {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        poll_fn(|cx| {
            let (len, fds) = loop {
                match fd_recvmsg(self.as_raw_fd(), buf) {
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => match self.poll_readable(cx)
                    {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(res) => res?,
                    },
                    v => break v?,
                }
            };
            Poll::Ready(Ok((len, fds)))
        })
        .await
    }

    /// Supports passing file descriptors.
    fn can_pass_unix_fd(&self) -> bool {
        true
    }

    async fn peer_credentials(&self) -> io::Result<ConnectionCredentials> {
        get_unix_peer_creds(self).await
    }
}

#[cfg(all(unix, not(feature = "tokio")))]
#[async_trait::async_trait]
impl WriteHalf for Arc<Async<UnixStream>> {
    async fn sendmsg(&mut self, buffer: &[u8], #[cfg(unix)] fds: &[RawFd]) -> io::Result<usize> {
        poll_fn(|cx| loop {
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
        })
        .await
    }

    async fn close(&mut self) -> io::Result<()> {
        let stream = self.clone();
        crate::Task::spawn_blocking(
            move || stream.get_ref().shutdown(std::net::Shutdown::Both),
            "close socket",
        )
        .await
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    async fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        send_zero_byte(self).await.map(Some)
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
#[async_trait::async_trait]
impl ReadHalf for tokio::net::unix::OwnedReadHalf {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        let stream = self.as_ref();
        poll_fn(|cx| {
            loop {
                match stream.try_io(tokio::io::Interest::READABLE, || {
                    // We use own custom function for reading because we need to receive file
                    // descriptors too.
                    fd_recvmsg(stream.as_raw_fd(), buf)
                }) {
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        match stream.poll_read_ready(cx) {
                            Poll::Pending => return Poll::Pending,
                            Poll::Ready(res) => res?,
                        }
                    }
                    v => return Poll::Ready(v),
                }
            }
        })
        .await
    }

    /// Supports passing file descriptors.
    fn can_pass_unix_fd(&self) -> bool {
        true
    }

    async fn peer_credentials(&self) -> io::Result<ConnectionCredentials> {
        get_unix_peer_creds(self.as_ref()).await
    }
}

#[cfg(all(unix, feature = "tokio"))]
#[async_trait::async_trait]
impl WriteHalf for tokio::net::unix::OwnedWriteHalf {
    async fn sendmsg(&mut self, buffer: &[u8], #[cfg(unix)] fds: &[RawFd]) -> io::Result<usize> {
        let stream = self.as_ref();
        poll_fn(|cx| loop {
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
        })
        .await
    }

    async fn close(&mut self) -> io::Result<()> {
        tokio::io::AsyncWriteExt::shutdown(self).await
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    async fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        send_zero_byte(self.as_ref()).await.map(Some)
    }
}

#[cfg(all(windows, not(feature = "tokio")))]
#[async_trait::async_trait]
impl ReadHalf for Arc<Async<UnixStream>> {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        match futures_util::AsyncReadExt::read(&mut self.as_ref(), buf).await {
            Err(e) => Err(e),
            Ok(len) => {
                #[cfg(unix)]
                let ret = (len, vec![]);
                #[cfg(not(unix))]
                let ret = len;
                Ok(ret)
            }
        }
    }

    async fn peer_credentials(&self) -> io::Result<ConnectionCredentials> {
        let stream = self.clone();
        crate::Task::spawn_blocking(
            move || {
                use crate::win32::{unix_stream_get_peer_pid, ProcessToken};

                let pid = unix_stream_get_peer_pid(&stream.get_ref())? as _;
                let sid = ProcessToken::open(if pid != 0 { Some(pid as _) } else { None })
                    .and_then(|process_token| process_token.sid())?;
                Ok(ConnectionCredentials::default()
                    .set_process_id(pid)
                    .set_windows_sid(sid))
            },
            "peer credentials",
        )
        .await
    }
}

#[cfg(all(windows, not(feature = "tokio")))]
#[async_trait::async_trait]
impl WriteHalf for Arc<Async<UnixStream>> {
    async fn sendmsg(&mut self, buf: &[u8], #[cfg(unix)] _fds: &[RawFd]) -> io::Result<usize> {
        futures_util::AsyncWriteExt::write(&mut self.as_ref(), buf).await
    }

    async fn close(&mut self) -> io::Result<()> {
        let stream = self.clone();
        crate::Task::spawn_blocking(
            move || stream.get_ref().shutdown(std::net::Shutdown::Both),
            "close socket",
        )
        .await
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    async fn send_zero_byte(&self) -> io::Result<Option<usize>> {
        send_zero_byte(self).await.map(Some)
    }
}

#[cfg(not(feature = "tokio"))]
#[async_trait::async_trait]
impl ReadHalf for Arc<Async<TcpStream>> {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        match futures_util::AsyncReadExt::read(&mut self.as_ref(), buf).await {
            Err(e) => Err(e),
            Ok(len) => {
                #[cfg(unix)]
                let ret = (len, vec![]);
                #[cfg(not(unix))]
                let ret = len;
                Ok(ret)
            }
        }
    }

    async fn peer_credentials(&self) -> io::Result<ConnectionCredentials> {
        #[cfg(windows)]
        let creds = {
            let stream = self.clone();
            crate::Task::spawn_blocking(
                move || {
                    use crate::win32::{tcp_stream_get_peer_pid, ProcessToken};

                    let pid = tcp_stream_get_peer_pid(&stream.get_ref())? as _;
                    let sid = ProcessToken::open(if pid != 0 { Some(pid as _) } else { None })
                        .and_then(|process_token| process_token.sid())?;
                    io::Result::Ok(
                        ConnectionCredentials::default()
                            .set_process_id(pid)
                            .set_windows_sid(sid),
                    )
                },
                "peer credentials",
            )
            .await
        }?;

        #[cfg(not(windows))]
        let creds = ConnectionCredentials::default();

        Ok(creds)
    }
}

#[cfg(not(feature = "tokio"))]
#[async_trait::async_trait]
impl WriteHalf for Arc<Async<TcpStream>> {
    async fn sendmsg(&mut self, buf: &[u8], #[cfg(unix)] fds: &[RawFd]) -> io::Result<usize> {
        #[cfg(unix)]
        if !fds.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a tcp stream",
            ));
        }

        futures_util::AsyncWriteExt::write(&mut self.as_ref(), buf).await
    }

    async fn close(&mut self) -> io::Result<()> {
        let stream = self.clone();
        crate::Task::spawn_blocking(
            move || stream.get_ref().shutdown(std::net::Shutdown::Both),
            "close socket",
        )
        .await
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
#[async_trait::async_trait]
impl ReadHalf for tokio::net::tcp::OwnedReadHalf {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        use tokio::io::{AsyncReadExt, ReadBuf};

        let mut read_buf = ReadBuf::new(buf);
        self.read_buf(&mut read_buf).await.map(|_| {
            let ret = read_buf.filled().len();
            #[cfg(unix)]
            let ret = (ret, vec![]);

            ret
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
#[async_trait::async_trait]
impl WriteHalf for tokio::net::tcp::OwnedWriteHalf {
    async fn sendmsg(&mut self, buf: &[u8], #[cfg(unix)] fds: &[RawFd]) -> io::Result<usize> {
        use tokio::io::AsyncWriteExt;

        #[cfg(unix)]
        if !fds.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a tcp stream",
            ));
        }

        self.write(buf).await
    }

    async fn close(&mut self) -> io::Result<()> {
        tokio::io::AsyncWriteExt::shutdown(self).await
    }
}

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
#[async_trait::async_trait]
impl ReadHalf for Arc<Async<vsock::VsockStream>> {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        match futures_util::AsyncReadExt::read(&mut self.as_ref(), buf).await {
            Err(e) => Err(e),
            Ok(len) => {
                #[cfg(unix)]
                let ret = (len, vec![]);
                #[cfg(not(unix))]
                let ret = len;
                Ok(ret)
            }
        }
    }
}

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
#[async_trait::async_trait]
impl WriteHalf for Arc<Async<vsock::VsockStream>> {
    async fn sendmsg(&mut self, buf: &[u8], #[cfg(unix)] fds: &[RawFd]) -> io::Result<usize> {
        #[cfg(unix)]
        if !fds.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a tcp stream",
            ));
        }

        futures_util::AsyncWriteExt::write(&mut self.as_ref(), buf).await
    }

    async fn close(&self) -> io::Result<()> {
        let stream = self.clone();
        crate::Task::spawn_blocking(
            move || stream.get_ref().shutdown(std::net::Shutdown::Both),
            "close socket",
        )
        .await
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
#[async_trait::async_trait]
impl ReadHalf for tokio_vsock::ReadHalf {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> RecvmsgResult {
        use tokio::io::{AsyncReadExt, ReadBuf};

        let mut read_buf = ReadBuf::new(buf);
        self.read_buf(&mut read_buf).await.map(|_| {
            let ret = read_buf.filled().len();
            #[cfg(unix)]
            let ret = (ret, vec![]);

            ret
        })
    }
}

#[cfg(feature = "tokio-vsock")]
#[async_trait::async_trait]
impl WriteHalf for tokio_vsock::WriteHalf {
    async fn sendmsg(&mut self, buf: &[u8], #[cfg(unix)] fds: &[RawFd]) -> io::Result<usize> {
        use tokio::io::AsyncWriteExt;

        #[cfg(unix)]
        if !fds.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a tcp stream",
            ));
        }

        self.write(buf).await
    }

    async fn close(&mut self) -> io::Result<()> {
        tokio::io::AsyncWriteExt::shutdown(self).await
    }
}
