<img src="zbus-pixels.gif" alt="zbus illustration" style="width: 100%;">

# zbus

[![CI Pipeline Status](https://github.com/dbus2/zbus/actions/workflows/rust.yml/badge.svg)](https://github.com/dbus2/zbus/actions/workflows/rust.yml)

A Rust API for [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html) communication. The
goal is to provide a safe and simple high- and low-level API akin to
[GDBus](https://developer.gnome.org/gio/stable/gdbus-convenience.html), that doesn't depend on C
libraries.

The project is divided into the following subcrates:

* [`zbus`] and [`zbus_macros`]: The main subcrates that provide the API to interact with D-Bus.
* [`zvariant`] and [`zvariant_derive`]: API for encoding/decoding of data to/from D-Bus wire
  format.
* [`zbus_names`]: A collection of types for various [D-Bus bus names][dbn].
* [`zbus_xml`]: API to handle D-Bus introspection description XML.
* [`zbus_xmlgen`]: A developer tool to generate Rust code from D-Bus interface description XML.

## Getting Started

The best way to get started with zbus is the [book](https://dbus2.github.io/zbus/), where we start
with basic D-Bus concepts and explain with code samples, how zbus makes D-Bus easy.

## Example code

We'll create a simple D-Bus service and client to demonstrate the usage of zbus. Note that these
examples assume that a D-Bus broker is setup on your machine and you've a session bus running
(`DBUS_SESSION_BUS_ADDRESS` environment variable must be set). This is guaranteed to be the case on
a typical Linux desktop session.

### Server

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
    let _conn = connection::Builder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
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
right, please [file an issue](https://github.com/dbus2/zbus/issues/new).

## Portability

Supported targets include Unix, Windows and macOS with Linux as the main (and tested) target.
Integration tests of zbus crate currently require a session bus running on the build host.

## License

MIT license [LICENSE-MIT](LICENSE-MIT)

## Alternative Crates

[dbus-rs][dbrs] relies on the battle tested libdbus C library to send and receive messages.
Companion crates add [Tokio support][dbrs-tokio], [server builder without macros][dbrs-cr], and
[code generation][dbrs-cg].

There are many other D-Bus crates out there with various levels of maturity and features.

[`zbus`]: zbus/README.md
[`zbus_macros`]: zbus_macros/README.md
[`zbus_names`]: zbus_names/README.md
[`zbus_xml`]: zbus_xml/README.md
[`zbus_xmlgen`]: zbus_xmlgen/README.md
[`zvariant`]: zvariant/README.md
[`zvariant_derive`]: zvariant_derive/README.md
[dbn]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names
[dbrs]: https://github.com/diwic/dbus-rs/
[dbrs-tokio]: https://github.com/diwic/dbus-rs/tree/master/dbus-tokio
[dbrs-cr]: https://github.com/diwic/dbus-rs/tree/master/dbus-crossroads
[dbrs-cg]: https://github.com/diwic/dbus-rs/tree/master/dbus-codegen
