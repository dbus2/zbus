#[cfg(not(feature = "tokio"))]
use async_io::Async;
use std::io;
#[cfg(unix)]
use std::os::fd::BorrowedFd;
#[cfg(not(feature = "tokio"))]
use std::{net::TcpStream, sync::Arc};

use crate::{address::transport::TcpFamily, Error, Result};

use super::{ReadHalf, RecvmsgResult, WriteHalf};
#[cfg(feature = "tokio")]
use super::{Socket, Split};

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

    #[cfg(windows)]
    async fn peer_credentials(&mut self) -> io::Result<crate::fdo::ConnectionCredentials> {
        let stream = self.clone();
        crate::Task::spawn_blocking(
            move || {
                use crate::win32::{tcp_stream_get_peer_pid, ProcessToken};

                let pid = tcp_stream_get_peer_pid(stream.get_ref())? as _;
                let sid = ProcessToken::open(if pid != 0 { Some(pid as _) } else { None })
                    .and_then(|process_token| process_token.sid())?;
                io::Result::Ok(
                    crate::fdo::ConnectionCredentials::default()
                        .set_process_id(pid)
                        .set_windows_sid(sid),
                )
            },
            "peer credentials",
        )
        .await
    }
}

#[cfg(not(feature = "tokio"))]
#[async_trait::async_trait]
impl WriteHalf for Arc<Async<TcpStream>> {
    async fn sendmsg(
        &mut self,
        buf: &[u8],
        #[cfg(unix)] fds: &[BorrowedFd<'_>],
    ) -> io::Result<usize> {
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

    async fn peer_credentials(&mut self) -> io::Result<crate::fdo::ConnectionCredentials> {
        ReadHalf::peer_credentials(self).await
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
    async fn peer_credentials(&mut self) -> io::Result<crate::fdo::ConnectionCredentials> {
        let peer_addr = self.peer_addr()?.clone();
        crate::Task::spawn_blocking(
            move || win32_credentials_from_addr(&peer_addr),
            "peer credentials",
        )
        .await
    }
}

#[cfg(feature = "tokio")]
#[async_trait::async_trait]
impl WriteHalf for tokio::net::tcp::OwnedWriteHalf {
    async fn sendmsg(
        &mut self,
        buf: &[u8],
        #[cfg(unix)] fds: &[BorrowedFd<'_>],
    ) -> io::Result<usize> {
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

    #[cfg(windows)]
    async fn peer_credentials(&mut self) -> io::Result<crate::fdo::ConnectionCredentials> {
        let peer_addr = self.peer_addr()?.clone();
        crate::Task::spawn_blocking(
            move || win32_credentials_from_addr(&peer_addr),
            "peer credentials",
        )
        .await
    }
}

#[cfg(feature = "tokio")]
#[cfg(windows)]
fn win32_credentials_from_addr(
    addr: &std::net::SocketAddr,
) -> io::Result<crate::fdo::ConnectionCredentials> {
    use crate::win32::{socket_addr_get_pid, ProcessToken};

    let pid = socket_addr_get_pid(addr)? as _;
    let sid = ProcessToken::open(if pid != 0 { Some(pid as _) } else { None })
        .and_then(|process_token| process_token.sid())?;
    Ok(crate::fdo::ConnectionCredentials::default()
        .set_process_id(pid)
        .set_windows_sid(sid))
}

#[cfg(not(feature = "tokio"))]
type Stream = Async<TcpStream>;
#[cfg(feature = "tokio")]
type Stream = tokio::net::TcpStream;

async fn connect_with(host: &str, port: u16, family: Option<TcpFamily>) -> Result<Stream> {
    #[cfg(not(feature = "tokio"))]
    {
        use std::net::ToSocketAddrs;

        let host = host.to_string();
        let addrs = crate::Task::spawn_blocking(
            move || -> Result<Vec<std::net::SocketAddr>> {
                let addrs = (host, port).to_socket_addrs()?.filter(|a| {
                    if let Some(family) = family {
                        if family == TcpFamily::IPv4 {
                            a.is_ipv4()
                        } else {
                            a.is_ipv6()
                        }
                    } else {
                        true
                    }
                });
                Ok(addrs.collect())
            },
            "connect tcp",
        )
        .await
        .map_err(|e| Error::Address(format!("Failed to receive TCP addresses: {e}")))?;

        // we could attempt connections in parallel?
        let mut last_err = Error::Address("Failed to connect".into());
        for addr in addrs {
            match Stream::connect(addr).await {
                Ok(stream) => return Ok(stream),
                Err(e) => last_err = e.into(),
            }
        }

        Err(last_err)
    }

    #[cfg(feature = "tokio")]
    {
        // FIXME: doesn't handle family
        let _ = family;
        Stream::connect((host, port))
            .await
            .map_err(|e| Error::InputOutput(e.into()))
    }
}

pub(crate) async fn connect(addr: &crate::address::transport::Tcp<'_>) -> Result<Stream> {
    let Some(host) = addr.host() else {
        return Err(Error::Address("No host in address".into()));
    };
    let Some(port) = addr.port() else {
        return Err(Error::Address("No port in address".into()));
    };

    connect_with(host, port, addr.family()).await
}

#[cfg(test)]
mod tests {
    use crate::address::{transport::Transport, DBusAddr};

    #[test]
    fn connect() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let addr: DBusAddr<'_> = format!("tcp:host=localhost,port={port}")
            .try_into()
            .unwrap();
        let tcp = match addr.transport().unwrap() {
            Transport::Tcp(tcp) => tcp,
            _ => unreachable!(),
        };
        crate::utils::block_on(super::connect(&tcp)).unwrap();
    }
}
