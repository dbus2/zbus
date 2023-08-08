#[cfg(not(feature = "tokio"))]
use async_io::Async;
use std::{ffi::OsString, future::Future, pin::Pin};

use crate::{
    addr::{
        transport::{NonceTcp, Tcp, TcpFamily, Transport, UnixAddrKind},
        DBusAddr, ToDBusAddrs,
    },
    Error, Result,
};

#[cfg(any(feature = "vsock", feature = "tokio-vsock"))]
use crate::addr::transport::Vsock;

#[cfg(target_os = "macos")]
use crate::addr::transport::Launchd;

#[cfg(target_os = "windows")]
use crate::addr::transport::Autolaunch;

#[cfg(all(any(unix, windows), not(feature = "tokio")))]
type UnixStream = Async<super::UnixStream>;
#[cfg(all(unix, feature = "tokio"))]
use super::UnixStream;

#[cfg(not(feature = "tokio"))]
type TcpStream = Async<super::TcpStream>;
#[cfg(feature = "tokio")]
use super::TcpStream;

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
type VsockStream = Async<super::VsockStream>;
#[cfg(feature = "tokio-vsock")]
use super::VsockStream;

#[derive(Debug)]
pub(crate) enum Stream {
    #[cfg(any(unix, all(windows, not(feature = "tokio"))))]
    Unix(UnixStream),
    Tcp(TcpStream),
    #[cfg(any(
        all(feature = "vsock", not(feature = "tokio")),
        feature = "tokio-vsock"
    ))]
    Vsock(VsockStream),
}

async fn tcp_stream_connect(host: &str, port: u16, family: Option<TcpFamily>) -> Result<TcpStream> {
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
            match TcpStream::connect(addr).await {
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
        TcpStream::connect((host, port))
            .await
            .map_err(|e| Error::InputOutput(e.into()))
    }
}

impl Stream {
    async fn connect_unix(addr: &UnixAddrKind<'_>) -> Result<Stream> {
        let mut s = OsString::from("\0");

        let p = match addr {
            // We should construct a SocketAddr instead, but this is not supported by all APIs
            // So we limit ourself to utf-8 paths
            UnixAddrKind::Path(p) => p.as_ref(),
            UnixAddrKind::Abstract(a) => {
                s.push(
                    std::str::from_utf8(a)
                        .map_err(|_| Error::Address("Unhandled abstract path".into()))?,
                );
                s.as_os_str()
            }
            _ => return Err(Error::Address("Address is not connectable".into())),
        };

        #[cfg(not(feature = "tokio"))]
        {
            #[cfg(windows)]
            {
                let p = p.to_os_string();
                let stream = crate::Task::spawn_blocking(
                    move || uds_windows::UnixStream::connect(p),
                    "unix stream connection",
                )
                .await?;
                Async::new(stream)
                    .map(Stream::Unix)
                    .map_err(|e| Error::InputOutput(e.into()))
            }

            #[cfg(not(windows))]
            {
                UnixStream::connect(p)
                    .await
                    .map(Stream::Unix)
                    .map_err(|e| Error::InputOutput(e.into()))
            }
        }

        #[cfg(feature = "tokio")]
        {
            #[cfg(unix)]
            {
                UnixStream::connect(p)
                    .await
                    .map(Stream::Unix)
                    .map_err(|e| Error::InputOutput(e.into()))
            }

            #[cfg(not(unix))]
            {
                let _ = p;
                Err(Error::Unsupported)
            }
        }
    }

    #[cfg(target_os = "macos")]
    async fn connect_launchd(addr: &Launchd<'_>) -> Result<Stream> {
        let addr = super::macos::launchd_bus_address(addr.env()).await?;
        match addr.transport()? {
            Transport::Unix(t) => Self::connect_unix(t.kind()).await,
            _ => Err(Error::Address(format!("Address is unsupported: {}", addr))),
        }
    }

    async fn connect_tcp(addr: &Tcp<'_>) -> Result<Stream> {
        let Some(host) = addr.host() else {
            return Err(Error::Address("No host in address".into()));
        };
        let Some(port) = addr.port() else {
            return Err(Error::Address("No port in address".into()));
        };

        tcp_stream_connect(host, port, addr.family())
            .await
            .map(Stream::Tcp)
    }

    async fn connect_nonce_tcp(addr: &NonceTcp<'_>) -> Result<Stream> {
        let Some(host) = addr.host() else {
            return Err(Error::Address("No host in address".into()));
        };
        let Some(port) = addr.port() else {
            return Err(Error::Address("No port in address".into()));
        };
        let Some(noncefile) = addr.noncefile() else {
            return Err(Error::Address("No noncefile in address".into()));
        };

        let mut stream = tcp_stream_connect(host, port, addr.family()).await?;

        #[cfg(not(feature = "tokio"))]
        {
            let nonce = std::fs::read(noncefile)?;
            let mut nonce = &nonce[..];

            while !nonce.is_empty() {
                let len = stream
                    .write_with_mut(|s| std::io::Write::write(s, nonce))
                    .await?;
                nonce = &nonce[len..];
            }
        }

        #[cfg(feature = "tokio")]
        {
            let nonce = tokio::fs::read(noncefile).await?;
            tokio::io::AsyncWriteExt::write_all(&mut stream, &nonce).await?;
        }

        Ok(Stream::Tcp(stream))
    }

    #[cfg(target_os = "windows")]
    async fn connect_autolaunch(addr: &Autolaunch<'_>) -> Result<Stream> {
        let addr = super::win32::autolaunch_bus_address(addr.scope())?;

        if let Transport::Autolaunch(_) = addr.transport()? {
            return Err(Error::Address("Recursive autolaunch: address".into()));
        }

        Self::connect_addr(addr).await
    }

    #[cfg(any(feature = "vsock", feature = "tokio-vsock"))]
    async fn connect_vsock(addr: &Vsock<'_>) -> Result<Stream> {
        let Some(cid) = addr.cid() else {
            return Err(Error::Address("No cid in address".into()));
        };
        let Some(port) = addr.port() else {
            return Err(Error::Address("No port in address".into()));
        };

        #[cfg(all(feature = "vsock", not(feature = "tokio")))]
        {
            let stream = crate::Task::spawn_blocking(
                move || vsock::VsockStream::connect_with_cid_port(cid, port),
                "connect vsock",
            )
            .await
            .map_err(|e| Error::Address(format!("Failed to connect: {e}")))?;
            Async::new(stream).map(Stream::Vsock).map_err(Into::into)
        }

        #[cfg(feature = "tokio-vsock")]
        VsockStream::connect(cid, port)
            .await
            .map(Stream::Vsock)
            .map_err(Into::into)
    }

    fn connect_addr(addr: DBusAddr<'_>) -> Pin<Box<dyn Future<Output = Result<Stream>> + '_>> {
        Box::pin(async move {
            match addr.transport()? {
                Transport::Unix(t) => Self::connect_unix(t.kind()).await,
                #[cfg(target_os = "macos")]
                Transport::Launchd(t) => Self::connect_launchd(&t).await,
                Transport::Tcp(t) => Self::connect_tcp(&t).await,
                Transport::NonceTcp(t) => Self::connect_nonce_tcp(&t).await,
                #[cfg(target_os = "windows")]
                Transport::Autolaunch(t) => Self::connect_autolaunch(&t).await,
                #[cfg(any(feature = "vsock", feature = "tokio-vsock"))]
                Transport::Vsock(t) => Self::connect_vsock(&t).await,
                _ => Err(Error::Address(format!("Address is unsupported: {}", addr))),
            }
        })
    }

    pub(crate) async fn connect<A>(addr: A) -> Result<Stream>
    where
        A: for<'a> ToDBusAddrs<'a>,
    {
        let mut last_err = None;
        for addr in addr.to_dbus_addrs() {
            let addr = match addr {
                Ok(addr) => addr,
                Err(e) => {
                    last_err = Some(e);
                    continue;
                }
            };
            match Self::connect_addr(addr).await {
                Ok(l) => return Ok(l),
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.unwrap_or_else(|| Error::Address("Could not resolve to any addresses".into())))
    }
}

#[cfg(test)]
mod tests {
    use super::Stream;

    #[test]
    fn connect_tcp() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let addr = format!("tcp:host=localhost,port={port}");
        crate::utils::block_on(async { Stream::connect(addr).await }).unwrap();
    }

    #[test]
    fn connect_nonce_tcp() {
        use std::io::Write;

        const TEST_COOKIE: &[u8] = b"VERILY SECRETIVE";

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();

        let mut cookie = tempfile::NamedTempFile::new().unwrap();
        cookie.as_file_mut().write_all(TEST_COOKIE).unwrap();

        let mut encoded_path = String::new();
        crate::addr::encode_percents(&mut encoded_path, cookie.path().to_str().unwrap().as_ref())
            .unwrap();

        let addr = format!("nonce-tcp:host=localhost,port={port},noncefile={encoded_path}");

        let (sender, receiver) = std::sync::mpsc::sync_channel(1);

        std::thread::spawn(move || {
            use std::io::Read;

            let mut client = listener.incoming().next().unwrap().unwrap();

            let mut buf = [0u8; 16];
            client.read_exact(&mut buf).unwrap();

            sender.send(buf == TEST_COOKIE).unwrap();
        });

        crate::utils::block_on(Stream::connect(addr)).unwrap();

        let saw_cookie = receiver
            .recv_timeout(std::time::Duration::from_millis(100))
            .expect("nonce file content hasn't been received by server thread in time");

        assert!(
            saw_cookie,
            "nonce file content has been received, but was invalid"
        );
    }
}
