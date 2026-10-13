<img src="zbus-pixels.gif" alt="zbus illustration" style="width: 100%;">

# zbus

[![CI Pipeline Status](https://github.com/z-galaxy/zbus/actions/workflows/rust.yml/badge.svg)](https://github.com/z-galaxy/zbus/actions/workflows/rust.yml)

A Rust API for [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html) communication. The
goal is to provide a safe and simple high- and low-level API akin to
[GDBus](https://developer.gnome.org/gio/stable/gdbus-convenience.html), that doesn't depend on C
libraries.

The project is divided into the following subcrates:

* [`zbus`]: The main subcrate. It provides the API to interact with D-Bus, the [D-Bus wire
  format][wf] (what used to be the `zvariant` crate) and the [bus name types][bn] (what used to be
  the `zbus_names` crate). With `default-features = false` you get the wire format and the name
  types alone, without any of the D-Bus API.
* [`zbus_macros`]: The procedural macros behind `#[proxy]`, `#[interface]`, `#[derive(DBusError)]`
  and the wire-format derives. `zbus` re-exports all of them, so you rarely depend on it directly.
* [`zbus_xml`]: API to handle D-Bus introspection description XML.
* [`zbus_xmlgen`]: A developer tool to generate Rust code from D-Bus interface description XML.
* [`zbus_utils`]: The D-Bus signature parser, name validators and derive-macro plumbing
  shared by `zbus` and `zbus_macros`.

[zgvariant] is a sibling project. It implements [GVariant], the format zbus itself dropped in
6.0. From its 2.0 release it builds on `zbus_utils` too, so its signature type is the same one
zbus uses; zgvariant 1.x depends on `zvariant_utils` 4.x — this crate under its old name — and its
signature type is a distinct one.

## Getting Started

The best way to get started with zbus is the [book](https://z-galaxy.github.io/zbus/), where we start
with basic D-Bus concepts and explain with code samples, how zbus makes D-Bus easy.

## Example code

We'll create a simple D-Bus service and client to demonstrate the usage of zbus. Note that these
examples assume that a D-Bus broker is setup on your machine and you've a session bus running
(`DBUS_SESSION_BUS_ADDRESS` environment variable must be set). This is guaranteed to be the case on
a typical Linux desktop session.

### Service

A simple service that politely greets whoever calls its `SayHello` method:

```rust,no_run
use std::{error::Error, future::pending};
use zbus::{connection, interface};

struct Greeter {
    count: u64
}

#[interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }
}

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let greeter = Greeter { count: 0 };
    let _conn = connection::Builder::session()
        .name("org.zbus.MyGreeter")
        .serve_at("/org/zbus/MyGreeter", greeter)
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
```

You can use the following command to test it:

```bash
$ busctl --user call org.zbus.MyGreeter /org/zbus/MyGreeter org.zbus.MyGreeter1 SayHello s "Maria"
s "Hello Maria! I have been called 1 times."
```

### Client

Now let's write the client-side code for `MyGreeter` service:

```rust,no_run
use zbus::{Connection, Result, proxy};

#[proxy(
    interface = "org.zbus.MyGreeter1",
    default_service = "org.zbus.MyGreeter",
    default_path = "/org/zbus/MyGreeter"
)]
trait MyGreeter {
    async fn say_hello(&self, name: &str) -> Result<String>;
}

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `proxy` macro creates `MyGreeterProxy` based on `Notifications` trait.
    let proxy = MyGreeterProxy::new(&connection).await?;
    let reply = proxy.say_hello("Maria").await?;
    println!("{reply}");

    Ok(())
}
```

## Getting Help

If you need help in using these crates, are looking for ways to contribute, or just want to hang out
with the cool kids, please come chat with us in the
[`#zbus:matrix.org`](https://matrix.to/#/#zbus:matrix.org) Matrix room. If something doesn't seem
right, please [file an issue](https://github.com/z-galaxy/zbus/issues/new).

## Security

If you discover a security vulnerability, please report it privately following our
[Security Policy](SECURITY.md). We take security seriously and will respond promptly to reports.

## Portability

Supported targets include Unix, Windows and macOS with Linux as the main target, and WASI, with
the limits [the zbus crate's documentation][zbwasi] describes. Integration tests of zbus crate
currently require a session bus running on the build host.

[zbwasi]: zbus/README.md#wasi

## License

MIT license [LICENSE-MIT](LICENSE-MIT)

## Alternative Crates

[dbus-rs][dbrs] relies on the battle tested libdbus C library to send and receive messages.
Companion crates add [Tokio support][dbrs-tokio], [server builder without macros][dbrs-cr], and
[code generation][dbrs-cg].

There are many other D-Bus crates out there with various levels of maturity and features.

## Sponsors

<a href="https://codspeed.io/?utm_source=oss-sponsorship&utm_medium=z-galaxy">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://codspeed.io/codspeed-logo-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://codspeed.io/codspeed-logo-light.svg">
    <img alt="CodSpeed logo" src="https://codspeed.io/codspeed-logo-light.svg" width="400">
  </picture>
</a>

[`zbus`]: zbus/README.md
[`zbus_macros`]: zbus_macros/README.md
[`zbus_xml`]: zbus_xml/README.md
[`zbus_xmlgen`]: zbus_xmlgen/README.md
[`zbus_utils`]: zbus_utils/README.md
[wf]: https://docs.rs/zbus/latest/zbus/wire/index.html
[bn]: https://docs.rs/zbus/latest/zbus/names/index.html
[zgvariant]: https://github.com/z-galaxy/zgvariant
[GVariant]: https://developer.gnome.org/documentation/specifications/gvariant-specification-1.0.html
[dbrs]: https://github.com/diwic/dbus-rs/
[dbrs-tokio]: https://github.com/diwic/dbus-rs/tree/master/dbus-tokio
[dbrs-cr]: https://github.com/diwic/dbus-rs/tree/master/dbus-crossroads
[dbrs-cg]: https://github.com/diwic/dbus-rs/tree/master/dbus-codegen
