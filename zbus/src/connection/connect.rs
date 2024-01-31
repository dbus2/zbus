use tracing::debug;

use crate::{
    address::{transport::Transport, DBusAddr},
    legacy_address, Error, Guid, OwnedGuid, Result,
};

use super::socket::{self, BoxedSplit};

async fn connect(addr: &DBusAddr<'_>) -> Result<(BoxedSplit, Option<OwnedGuid>)> {
    let guid = match addr.guid() {
        Some(g) => Some(Guid::try_from(g.as_ref())?.into()),
        _ => None,
    };
    let split = match addr.transport()? {
        Transport::Tcp(t) => socket::tcp::connect(&t).await?.into(),
        _ => {
            // safety: unwrap() for code transition => addr is valid already
            let legacy: crate::Address = addr.to_string().parse().unwrap();
            match legacy.connect().await {
                #[cfg(any(unix, not(feature = "tokio")))]
                Ok(legacy_address::transport::Stream::Unix(stream)) => stream.into(),
                Ok(legacy_address::transport::Stream::Tcp(stream)) => stream.into(),
                #[cfg(any(
                    all(feature = "vsock", not(feature = "tokio")),
                    feature = "tokio-vsock"
                ))]
                Ok(legacy_address::transport::Stream::Vsock(stream)) => stream.into(),
                _ => return Err(Error::Address("unhandled address".into())),
            }
        }
    };
    Ok((split, guid))
}

pub(crate) async fn connect_address(
    address: &[DBusAddr<'_>],
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
