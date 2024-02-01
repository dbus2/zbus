#![cfg(target_os = "windows")]

use dbus_addr::{transport::Transport, DBusAddr};

use super::BoxedSplit;
use crate::{win32::windows_autolaunch_bus_address, Error, OwnedGuid, Result};

pub(crate) async fn connect(
    l: &dbus_addr::transport::Autolaunch<'_>,
) -> Result<(BoxedSplit, Option<OwnedGuid>)> {
    if l.scope().is_some() {
        return Err(Error::Address(
            "autolaunch with scope isn't supported yet".into(),
        ));
    }

    let addr: DBusAddr<'_> = windows_autolaunch_bus_address()?.try_into()?;

    if let Transport::Autolaunch(_) = addr.transport()? {
        return Err(Error::Address("Recursive autolaunch: address".into()));
    }

    super::connect(&addr).await
}
