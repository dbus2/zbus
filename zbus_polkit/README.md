# zbus_polkit

[![](https://docs.rs/zbus_polkit/badge.svg)](https://docs.rs/zbus_polkit/) [![](https://img.shields.io/crates/v/zbus_polkit)](https://crates.io/crates/zbus_polkit)

A crate to interact with [PolicyKit], a toolkit for defining and handling authorizations. It is used
for allowing unprivileged processes to speak to privileged processes.

**Status:** Stable.

#### Example code

```rust,no_run
use zbus::Connection;
use zbus_polkit::policykit1::*;

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = Connection::system().await?;
    let proxy = AuthorityProxy::new(&connection).await?;
    let subject = Subject::new_for_owner(std::process::id(), None, None)?;
    let result = proxy.check_authorization(
        &subject,
        "org.zbus.BeAwesome",
        &std::collections::HashMap::new(),
        CheckAuthorizationFlags::AllowUserInteraction.into(),
        "",
    ).await?;

    Ok(())
}
```

[PolicyKit]: https://gitlab.freedesktop.org/polkit/polkit/
