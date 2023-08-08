#![cfg(target_os = "windows")]

use crate::{
    addr::{transport::AutolaunchScope, DBusAddr},
    win32::{read_shm, Mutex},
    Error, Result,
};

pub fn autolaunch_bus_address(scope: Option<&AutolaunchScope<'_>>) -> Result<DBusAddr<'static>> {
    if scope.is_some() {
        return Err(Error::Address(
            "autolaunch with scope isn't supported yet".into(),
        ));
    }

    let mutex = Mutex::new("DBusAutolaunchMutex")?;
    let _guard = mutex.lock();

    let addr = read_shm("DBusDaemonAddressInfo")?;
    let addr = String::from_utf8(addr)
        .map_err(|e| Error::Address(format!("Unable to parse address as UTF-8: {}", e)))?;

    addr.try_into()
}
