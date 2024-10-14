#[cfg(all(feature = "vsock", not(feature = "tokio")))]
#[cfg(not(feature = "tokio"))]
use async_io::Async;

#[cfg(any(
    all(feature = "vsock", not(feature = "tokio")),
    feature = "tokio-vsock"
))]
use crate::{Error, Result};

#[cfg(feature = "tokio-vsock")]
use super::{Socket, Split};

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
#[async_trait::async_trait]
impl super::ReadHalf for std::sync::Arc<async_io::Async<vsock::VsockStream>> {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> super::RecvmsgResult {
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

    fn auth_mechanism(&self) -> crate::AuthMechanism {
        crate::AuthMechanism::Anonymous
    }
}

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
#[async_trait::async_trait]
impl super::WriteHalf for std::sync::Arc<async_io::Async<vsock::VsockStream>> {
    async fn sendmsg(
        &mut self,
        buf: &[u8],
        #[cfg(unix)] fds: &[std::os::fd::BorrowedFd<'_>],
    ) -> std::io::Result<usize> {
        use std::io;

        #[cfg(unix)]
        if !fds.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fds cannot be sent with a vsock stream",
            ));
        }

        futures_util::AsyncWriteExt::write(&mut self.as_ref(), buf).await
    }

    async fn close(&mut self) -> std::io::Result<()> {
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
    type ReadHalf = tokio_vsock::OwnedReadHalf;
    type WriteHalf = tokio_vsock::OwnedWriteHalf;

    fn split(self) -> Split<Self::ReadHalf, Self::WriteHalf> {
        let (read, write) = self.into_split();

        Split { read, write }
    }
}

#[cfg(feature = "tokio-vsock")]
#[async_trait::async_trait]
impl super::ReadHalf for tokio_vsock::OwnedReadHalf {
    async fn recvmsg(&mut self, buf: &mut [u8]) -> super::RecvmsgResult {
        use tokio::io::{AsyncReadExt, ReadBuf};

        let mut read_buf = ReadBuf::new(buf);
        self.read_buf(&mut read_buf).await.map(|_| {
            let ret = read_buf.filled().len();
            #[cfg(unix)]
            let ret = (ret, vec![]);

            ret
        })
    }

    fn auth_mechanism(&self) -> crate::conn::AuthMechanism {
        crate::conn::AuthMechanism::Anonymous
    }
}

#[cfg(feature = "tokio-vsock")]
#[async_trait::async_trait]
impl super::WriteHalf for tokio_vsock::OwnedWriteHalf {
    async fn sendmsg(
        &mut self,
        buf: &[u8],
        #[cfg(unix)] fds: &[std::os::fd::BorrowedFd<'_>],
    ) -> std::io::Result<usize> {
        use std::io;
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

    async fn close(&mut self) -> std::io::Result<()> {
        tokio::io::AsyncWriteExt::shutdown(self).await
    }
}

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
type Stream = Async<vsock::VsockStream>;
#[cfg(feature = "tokio-vsock")]
type Stream = tokio_vsock::VsockStream;

#[cfg(any(
    all(feature = "vsock", not(feature = "tokio")),
    feature = "tokio-vsock"
))]
pub(crate) async fn connect(addr: &crate::address::transport::Vsock<'_>) -> Result<Stream> {
    use crate::address;

    let Some(cid) = addr.cid() else {
        return Err(address::Error::MissingValue("cid".into()).into());
    };
    let Some(port) = addr.port() else {
        return Err(address::Error::MissingValue("port".into()).into());
    };

    #[cfg(all(feature = "vsock", not(feature = "tokio")))]
    {
        let stream = crate::Task::spawn_blocking(
            move || vsock::VsockStream::connect_with_cid_port(cid, port),
            "connect vsock",
        )
        .await?;
        Ok(Async::new(stream).map_err(|e| Error::InputOutput(e.into()))?)
    }

    #[cfg(feature = "tokio-vsock")]
    Stream::connect(tokio_vsock::VsockAddr::new(cid, port))
        .await
        .map_err(|e| Error::InputOutput(e.into()))
}
