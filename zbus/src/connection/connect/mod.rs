use std::{future::Future, pin::Pin};
use tracing::debug;

use crate::{
    address::{transport::Transport, Address},
    Error, Guid, OwnedGuid, Result,
};

use super::socket::{self, BoxedSplit};

mod macos;
mod win32;

pub(crate) async fn connect_address(
    address: &[Address<'_>],
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
    Err(Error::Address("No connectable address".into()))
}

fn connect(
    addr: &Address<'_>,
) -> Pin<Box<dyn Future<Output = ConnectResult> + Send + Sync + 'static>> {
    let addr = addr.to_owned();
    Box::pin(async move {
        let guid = match addr.guid() {
            Some(g) => Some(Guid::try_from(g.as_ref())?.into()),
            _ => None,
        };
        let split = match addr.transport()? {
            Transport::Tcp(t) => socket::tcp::connect(&t).await?.into(),
            Transport::NonceTcp(t) => socket::tcp::connect_nonce(&t).await?.into(),
            #[cfg(any(unix, not(feature = "tokio")))]
            Transport::Unix(u) => socket::unix::connect(&u).await?.into(),
            #[cfg(any(
                all(feature = "vsock", not(feature = "tokio")),
                feature = "tokio-vsock"
            ))]
            Transport::Vsock(v) => socket::vsock::connect(&v).await?.into(),
            #[cfg(target_os = "macos")]
            Transport::Launchd(l) => macos::connect(&l).await?.into(),
            #[cfg(target_os = "windows")]
            Transport::Autolaunch(l) => {
                return win32::connect(&l).await;
            }
            _ => {
                return Err(Error::Address(format!("Unhandled address: {}", addr)));
            }
        };
        Ok((split, guid))
    })
}

type ConnectResult = Result<(BoxedSplit, Option<OwnedGuid>)>;
