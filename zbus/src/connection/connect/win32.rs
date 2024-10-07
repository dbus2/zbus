#![cfg(target_os = "windows")]

use super::ConnectResult;
use crate::{
    address::{transport::Transport, Address},
    win32::autolaunch_bus_address,
    Error,
};
use std::{future::Future, pin::Pin};

pub(crate) fn connect<'l>(
    l: &'l crate::address::transport::Autolaunch<'_>,
) -> Pin<Box<dyn Future<Output = ConnectResult> + 'l>> {
    Box::pin(async move {
        if l.scope().is_some() {
            return Err(Error::Address(
                "autolaunch with scope isn't supported yet".into(),
            ));
        }

        let addr: Address<'_> = autolaunch_bus_address()?.try_into()?;

        if let Transport::Autolaunch(_) = addr.transport()? {
            return Err(Error::Address("Recursive autolaunch: address".into()));
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
