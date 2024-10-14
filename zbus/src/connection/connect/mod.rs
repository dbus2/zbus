use tracing::debug;

use crate::{
    address::{transport::Transport, OwnedAddress},
    Error, Guid, OwnedGuid, Result,
};

use super::socket::{self, BoxedSplit};

mod macos;
mod win32;

pub(crate) async fn connect_address(
    address: &[OwnedAddress],
) -> Result<(BoxedSplit, Option<OwnedGuid>)> {
    for addr in address {
        match connect(addr).await {
            Ok(res) => {
                return Ok(res);
            }
            Err(e) => {
                debug!("Failed to connect to: {}", e);
                continue;
            }
        }
    }
    Err(Error::Failure("No connectable address".into()))
}

async fn connect(addr: &OwnedAddress) -> ConnectResult {
    let guid = match addr.guid() {
        Some(g) => Some(Guid::try_from(g)?.into()),
        _ => None,
    };
    let split = match addr.transport() {
        Transport::Tcp(t) => socket::tcp::connect(t).await?.into(),
        Transport::NonceTcp(t) => socket::tcp::connect_nonce(t).await?.into(),
        #[cfg(any(unix, not(feature = "tokio")))]
        Transport::Unix(u) => socket::unix::connect(u).await?.into(),
        #[cfg(any(
            all(feature = "vsock", not(feature = "tokio")),
            feature = "tokio-vsock"
        ))]
        Transport::Vsock(v) => socket::vsock::connect(v).await?.into(),
        #[cfg(target_os = "macos")]
        Transport::Launchd(l) => macos::connect(l).await?.into(),
        #[cfg(target_os = "windows")]
        Transport::Autolaunch(l) => {
            return win32::connect(l).await;
        }
        _ => {
            tracing::debug!("Unsupported address: {addr}");
            return Err(Error::Unsupported);
        }
    };
    Ok((split, guid))
}

type ConnectResult = Result<(BoxedSplit, Option<OwnedGuid>)>;
