#![cfg(target_os = "windows")]

use super::BoxedSplit;
use crate::{
    address::{transport::Transport, DBusAddr},
    win32::autolaunch_bus_address,
    Error, OwnedGuid, Result,
};

pub(crate) async fn connect(
    l: &crate::address::transport::Autolaunch<'_>,
) -> Result<(BoxedSplit, Option<OwnedGuid>)> {
    if l.scope().is_some() {
        return Err(Error::Address(
            "autolaunch with scope isn't supported yet".into(),
        ));
    }

    let addr: DBusAddr<'_> = autolaunch_bus_address()?.try_into()?;

    if let Transport::Autolaunch(_) = addr.transport()? {
        return Err(Error::Address("Recursive autolaunch: address".into()));
    }

    super::connect(&addr).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn connect_autolaunch_session_bus() {
        use crate::address::{transport::Transport, DBusAddr};

        let addr: DBusAddr<'_> = "autolaunch:".try_into().unwrap();
        let autolaunch = match addr.transport().unwrap() {
            Transport::Autolaunch(l) => l,
            _ => unreachable!(),
        };
        crate::utils::block_on(super::connect(&autolaunch)).unwrap();
    }
}
