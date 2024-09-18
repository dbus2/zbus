#![cfg(target_os = "macos")]

use super::socket;
use crate::{
    address::{transport::Transport, Address},
    process::run,
    Error, Result,
};

async fn launchd_bus_address(env_key: &str) -> Result<Address<'static>> {
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

    Ok(format!("unix:path={}", addr.trim()).try_into()?)
}

pub(crate) async fn connect(
    l: &crate::address::transport::Launchd<'_>,
) -> Result<socket::unix::Stream> {
    let addr = launchd_bus_address(l.env()).await?;

    match addr.transport()? {
        Transport::Unix(t) => socket::unix::connect(&t).await,
        _ => Err(Error::Address(format!("Address is unsupported: {}", addr))),
    }
}

#[cfg(test)]
mod tests {
    use crate::address::{transport::Transport, Address};

    #[test]
    fn connect_launchd_session_bus() {
        let addr: Address<'_> = "launchd:env=DBUS_LAUNCHD_SESSION_BUS_SOCKET"
            .try_into()
            .unwrap();
        let launchd = match addr.transport().unwrap() {
            Transport::Launchd(l) => l,
            _ => unreachable!(),
        };
        crate::utils::block_on(super::connect(&launchd)).unwrap();
    }
}
