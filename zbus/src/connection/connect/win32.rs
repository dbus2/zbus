#![cfg(target_os = "windows")]

use super::ConnectResult;
use crate::{
    address::{transport::Transport, OwnedAddress},
    win32::autolaunch_bus_address,
    Error,
};
use std::{future::Future, pin::Pin};

pub(crate) fn connect<'l>(
    autolaunch: &'l crate::address::transport::Autolaunch<'_>,
) -> Pin<Box<dyn Future<Output = ConnectResult> + 'l>> {
    Box::pin(async move {
        if autolaunch.scope().is_some() {
            tracing::debug!("autolaunch with scope isn't supported yet");
            return Err(Error::Unsupported);
        }

        let addr: OwnedAddress = autolaunch_bus_address()?.try_into()?;

        if let Transport::Autolaunch(_) = addr.transport() {
            return Err(Error::Failure("Recursive autolaunch: address".into()));
        }

        super::connect(&addr).await
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn connect_autolaunch_session_bus() {
        use crate::address::{transport::Transport, Address};

        let addr: Address<'_> = "autolaunch:".try_into().unwrap();
        let autolaunch = match addr.transport().unwrap() {
            Transport::Autolaunch(l) => l,
            _ => unreachable!(),
        };
        crate::utils::block_on(super::connect(&autolaunch)).unwrap();
    }
}
