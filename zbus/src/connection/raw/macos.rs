#![cfg(target_os = "macos")]

use crate::{addr::DBusAddr, process::run, Error, Result};

pub(crate) async fn launchd_bus_address(env_key: &str) -> Result<DBusAddr<'static>> {
    let output = run("launchctl", ["getenv", env_key])
        .await
        .expect("failed to wait on launchctl output");

    if !output.status.success() {
        return Err(Error::Address(format!(
            "launchctl terminated with code: {}",
            output.status
        )));
    }

    let addr = String::from_utf8(output.stdout)
        .map_err(|e| Error::Address(format!("Unable to parse launchctl output as UTF-8: {}", e)))?;

    format!("unix:path={}", addr.trim()).try_into()
}
