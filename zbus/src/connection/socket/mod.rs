mod split;
pub use split::Split;

mod tcp;
mod unix;

#[cfg(not(feature = "tokio"))]
use async_io::Async;
use std::io;
#[cfg(not(feature = "tokio"))]
use std::sync::Arc;

#[cfg(unix)]
use std::os::unix::io::RawFd;

use crate::fdo::ConnectionCredentials;
#[cfg(unix)]
use crate::OwnedFd;

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
    async fn peer_credentials(&mut self) -> io::Result<ConnectionCredentials> {
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
    async fn send_zero_byte(&mut self) -> io::Result<Option<usize>> {
        Ok(None)
    }

    /// Close the socket.
    ///
    /// After this call, it is valid for all reading and writing operations to fail.
    async fn close(&mut self) -> io::Result<()>;

    /// Supports passing file descriptors.
    ///
    /// Default implementation returns `false`.
    fn can_pass_unix_fd(&self) -> bool {
        false
    }

    /// Return the peer credentials.
    async fn peer_credentials(&mut self) -> io::Result<ConnectionCredentials> {
        Ok(ConnectionCredentials::default())
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

    async fn peer_credentials(&mut self) -> io::Result<ConnectionCredentials> {
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
    async fn send_zero_byte(&mut self) -> io::Result<Option<usize>> {
        (**self).send_zero_byte().await
    }

    async fn close(&mut self) -> io::Result<()> {
        (**self).close().await
    }

    fn can_pass_unix_fd(&self) -> bool {
        (**self).can_pass_unix_fd()
    }

    async fn peer_credentials(&mut self) -> io::Result<ConnectionCredentials> {
        (**self).peer_credentials().await
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
                "fds cannot be sent with a vsock stream",
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
                "fds cannot be sent with a vsock stream",
            ));
        }

        self.write(buf).await
    }

    async fn close(&mut self) -> io::Result<()> {
        tokio::io::AsyncWriteExt::shutdown(self).await
    }
}
