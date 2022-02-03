<img src="https://gitlab.freedesktop.org/dbus/zbus/-/raw/main/zbus-pixels.gif" alt="zbus illustration" style="width: 100%;">

# zbus

[![pipeline status](https://gitlab.freedesktop.org/dbus/zbus/badges/main/pipeline.svg)](https://gitlab.freedesktop.org/dbus/zbus/-/commits/main)

A Rust API for [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html) communication. The
goal is to provide a safe and simple high- and low-level API akin to
[GDBus](https://developer.gnome.org/gio/stable/gdbus-convenience.html), that doesn't depend on C
libraries.

The project is divided into the following subcrates:

* [`zbus`] and [`zbus_macros`]
* [`zvariant`] and [`zvariant_derive`]
* [`zbus_names`]
* [`zbus_xmlgen`]
* [`zbus_polkit`]

## Getting Started

The best way to get started with zbus is the [book](https://dbus.pages.freedesktop.org/zbus/),
where we start with basic D-Bus concepts and explain with code samples, how zbus makes D-Bus easy.

## Example code

### Client

This code display a notification on your Freedesktop.org-compatible OS:

```rust,no_run
use std::{collections::HashMap, error::Error};

use zbus::{Connection, dbus_proxy};
use zvariant::Value;

#[dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: &HashMap<&str, &Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;

    // `dbus_proxy` macro creates `NotificationProxy` based on `Notifications` trait.
    let proxy = NotificationsProxy::new(&connection).await?;
    let reply = proxy.notify(
        "my-app",
        0,
        "dialog-information",
        "A summary",
        "Some body",
        &[],
        &HashMap::new(),
        5000,
    ).await?;
    dbg!(reply);

    Ok(())
}
```

### Server

A simple service that politely greets whoever calls its `SayHello` method:

```rust,no_run
use std::{
    error::Error,
    future::pending,
    time::Duration,
};
use zbus::{ObjectServer, ConnectionBuilder, dbus_interface, fdo};

struct Greeter {
    count: u64
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called: {}", name, self.count)
    }
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let greeter = Greeter { count: 0 };
    let _ = ConnectionBuilder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()
        .await?;

    // Do other things or wait forever
    pending::<()>().await;

    Ok(())
}
```

You can use the following command to test it:

```bash
$ busctl --user call org.zbus.MyGreeter /org/zbus/MyGreeter org.zbus.MyGreeter1 SayHello s "Maria"
Hello Maria!
s
```

## Getting Help

If you need help in using these crates, are looking for ways to contribute, or just want to hang out
with the cool kids, please come chat with us in the
[`#zbus:matrix.org`](https://matrix.to/#/#zbus:matrix.org) Matrix room. If something doesn't seem
right, please [file an issue](https://gitlab.freedesktop.org/dbus/zbus/-/issues/new).

## Portability

All crates are currently Unix-only with Linux as the main (and tested) target and will fail to build
on non-unix. This is hopefully a temporary limitation. Moreover, integration tests of zbus crate
currently require a session bus running on the build host.

## License

MIT license [LICENSE-MIT](LICENSE-MIT)

[`zbus`]: zbus/README.md
[`zbus_macros`]: zbus_macros/README.md
[`zbus_names`]: zbus_names/README.md
[`zbus_polkit`]: zbus_polkit/README.md
[`zbus_xmlgen`]: zbus_xmlgen/README.md
[`zvariant`]: zvariant/README.md
[`zvariant_derive`]: zvariant_derive/README.md
[dbn]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names
