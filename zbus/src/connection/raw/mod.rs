mod connection;
mod socket;

mod stream;
pub(crate) use stream::Stream;

mod macos;
mod win32;

pub use connection::Connection;
pub use socket::Socket;

#[cfg(not(feature = "tokio"))]
pub(crate) type TcpStream = std::net::TcpStream;
#[cfg(feature = "tokio")]
pub(crate) use tokio::net::TcpStream;

#[cfg(all(unix, not(feature = "tokio")))]
pub(crate) type UnixStream = std::os::unix::net::UnixStream;
#[cfg(all(windows, not(feature = "tokio")))]
pub(crate) type UnixStream = uds_windows::UnixStream;
#[cfg(all(unix, feature = "tokio"))]
pub(crate) use tokio::net::UnixStream;

#[cfg(all(feature = "vsock", not(feature = "tokio")))]
pub(crate) type VsockStream = vsock::VsockStream;
#[cfg(feature = "tokio-vsock")]
pub(crate) use tokio_vsock::VsockStream;
