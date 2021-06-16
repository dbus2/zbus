[![pipeline status](https://gitlab.freedesktop.org/dbus/zbus/badges/main/pipeline.svg)](https://gitlab.freedesktop.org/dbus/zbus/-/commits/main)

# zbus

A Rust API for [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html) communication. The
goal is to provide a safe and simple high- and low-level API akin to
[GDBus](https://developer.gnome.org/gio/stable/gdbus-convenience.html), that doesn't depend on C
libraries.

The project is divided into three main crates:

## zbus

[![](https://docs.rs/zbus/badge.svg)](https://docs.rs/zbus/) [![](https://img.shields.io/crates/v/zbus)](https://crates.io/crates/zbus)

The zbus crate provides the main API you will use to interact with D-Bus from Rust. It takes care of
the establishment of a connection, the creation, sending and receiving of different kind of D-Bus
messages (method calls, signals etc) for you.

zbus crate is currently Linux-specific[^otheros].

**Status:** Stable[^stability].

### Dependencies

  * nix
  * byteorder
  * serde
  * serde_repr
  * enumflags2
  * derivative
  * serde-xml-rs (optional)

### Getting Started

The best way to get started with zbus is the [book](https://dbus.pages.freedesktop.org/zbus/),
where we start with basic D-Bus concepts and explain with code samples, how zbus makes D-Bus easy.

### Example code

#### Client

This code display a notification on your Freedesktop.org-compatible OS:

```rust,no_run
use std::collections::HashMap;
use std::error::Error;

use zbus::dbus_proxy;
use zvariant::Value;

#[dbus_proxy]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}

fn main() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;

    let proxy = NotificationsProxy::new(&connection)?;
    let reply = proxy.notify(
        "my-app",
        0,
        "dialog-information",
        "A summary",
        "Some body",
        &[],
        HashMap::new(),
        5000,
    )?;
    dbg!(reply);

    Ok(())
}
```

#### Server

A simple service that politely greets whoever calls its `SayHello` method:

```rust,no_run
use std::error::Error;
use zbus::{dbus_interface, fdo};

struct Greeter {
    count: u64
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called: {}", name, self.count)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;
    fdo::DBusProxy::new(&connection)?.request_name(
        "org.zbus.MyGreeter",
        fdo::RequestNameFlags::ReplaceExisting.into(),
    )?;

    let mut object_server = zbus::ObjectServer::new(&connection);
    let mut greeter = Greeter { count: 0 };
    object_server.at("/org/zbus/MyGreeter", greeter)?;
    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("{}", err);
        }
    }
}
```

You can use the following command to test it:

```bash
$ busctl --user call org.zbus.MyGreeter /org/zbus/MyGreeter org.zbus.MyGreeter1 SayHello s "Maria"
Hello Maria!
s
```

### Asynchronous API

Runtime-agnostic async/await-compatible API for both
[(not so) low-level](https://docs.rs/zbus/latest/zbus/azync/connection/struct.Connection.html)
message handling and
[high-level client-side proxy](https://dbus.pages.freedesktop.org/zbus/async.html#client) is also
provided. High-level server-side API coming soon.

### Compatibility with async runtimes

zbus is runtime-agnostic and should work out of the box with different Rust async runtimes. However,
in order to achieve that, zbus spawns a thread per connection to handle various internal tasks. If
that is something you would like to avoid, you need to:
  * disable the `internal-executor` feature (which is a default feature).
  * Ensure the [internal executor keeps ticking continuously][iektc].

## zvariant

[![](https://docs.rs/zvariant/badge.svg)](https://docs.rs/zvariant/) [![](https://img.shields.io/crates/v/zvariant)](https://crates.io/crates/zvariant)

This crate provides API for encoding/decoding of data to/from D-Bus wire format. This binary wire
format is simple and very efficient and hence useful outside of D-Bus context as well. A modified
form of this format, [GVariant](https://developer.gnome.org/glib/stable/glib-GVariant.html) is very
commonly used for efficient storage of arbitrary data and is also supported by this crate.

**Status:** Stable.

### Dependencies

* byteorder
* serde
* arrayvec (optional)
* enumflags2 (optional)

### Example code

```rust
use std::collections::HashMap;
use byteorder::LE;
use zvariant::{from_slice, to_bytes};
use zvariant::EncodingContext as Context;

// All serialization and deserialization API, needs a context.
let ctxt = Context::<LE>::new_dbus(0);
// You can also use the more efficient GVariant format:
// let ctxt = Context::<LE>::new_gvariant(0);

// i16
let encoded = to_bytes(ctxt, &42i16).unwrap();
let decoded: i16 = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, 42);

// strings
let encoded = to_bytes(ctxt, &"hello").unwrap();
let decoded: &str = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, "hello");

// tuples
let t = ("hello", 42i32, true);
let encoded = to_bytes(ctxt, &t).unwrap();
let decoded: (&str, i32, bool) = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, t);

// Vec
let v = vec!["hello", "world!"];
let encoded = to_bytes(ctxt, &v).unwrap();
let decoded: Vec<&str> = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, v);

// Dictionary
let mut map: HashMap<i64, &str> = HashMap::new();
map.insert(1, "123");
map.insert(2, "456");
let encoded = to_bytes(ctxt, &map).unwrap();
let decoded: HashMap<i64, &str> = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded[&1], "123");
assert_eq!(decoded[&2], "456");
```

## zvariant_derive

[![](https://docs.rs/zvariant_derive/badge.svg)](https://docs.rs/zvariant_derive/) [![](https://img.shields.io/crates/v/zvariant_derive)](https://crates.io/crates/zvariant_derive)

This crate provides a derive macro to easily implement [`Type` trait](https://docs.rs/zvariant/2.0.0/zvariant/trait.Type.html) on structs and enums.

**Status:** Stable.

### Dependencies

* proc-macro2
* syn
* quote

### Example code

```rust
use zvariant::{EncodingContext, from_slice, to_bytes};
use zvariant::{derive::Type, Type};
use serde::{Deserialize, Serialize};
use byteorder::LE;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct<'s> {
    field1: u16,
    field2: i64,
    field3: &'s str,
}

assert_eq!(Struct::signature(), "(qxs)");
let s = Struct {
    field1: 42,
    field2: i64::max_value(),
    field3: "hello",
};
let ctxt = EncodingContext::<LE>::new_dbus(0);
let encoded = to_bytes(ctxt, &s).unwrap();
let decoded: Struct = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, s);
```

## Other crates

Apart from the three crates described above, zbus project also provides a few other crates:

### zbus_macros

[![](https://docs.rs/zbus_macros/badge.svg)](https://docs.rs/zbus_macros/) [![](https://img.shields.io/crates/v/zbus_macros)](https://crates.io/crates/zbus_macros)

This crate provides the convenient zbus macros that we already saw in action in the sample code
above. However, `zbus` crate re-exports the macros for your convenience so you do not need to use
this crate directly.

**Status:** Stable.

### zbus_polkit

[![](https://docs.rs/zbus_polkit/badge.svg)](https://docs.rs/zbus_polkit/) [![](https://img.shields.io/crates/v/zbus_polkit)](https://crates.io/crates/zbus_polkit)

A crate to interact with [PolicyKit], a toolkit for defining and handling authorizations. It is used
for allowing unprivileged processes to speak to privileged processes.

**Status:** Stable.

#### Dependencies

  * serde
  * serde_repr
  * enumflags2

#### Example code

```rust,no_run
use zbus::Connection;
use zbus_polkit::policykit1::*;

let connection = Connection::new_system().unwrap();
let proxy = AuthorityProxy::new(&connection).unwrap();
let subject = Subject::new_for_owner(std::process::id(), None, None).unwrap();
let result = proxy.check_authorization(
    &subject,
    "org.zbus.BeAwesome",
    std::collections::HashMap::new(),
    CheckAuthorizationFlags::AllowUserInteraction.into(),
    "",
);
```

### zbus_xmlgen

[![](https://img.shields.io/crates/v/zbus_xmlgen)](https://crates.io/crates/zbus_xmlgen)

A binary crate that provides a developer tool to generate Rust code from D-Bus XML interface
descriptions. It can be used to generate the code directly from a running D-Bus system, session
or other service, or using a preexisting XML file for input.

**Status:** Stable.

#### Dependencies

  * zbus
  * zvariant
  * snakecase

#### Usage

```shell
$ cargo install zbus_xmlgen
$ zbus-xmlgen --system org.freedesktop.login1 /org/freedesktop/login1
$ zbus-xmlgen --session org.freedesktop.ScreenSaver /org/freedesktop/ScreenSaver
$ zbus-xmlgen --address unix:abstract=/home/user/.cache/ibus/dbus-fpxKwgbJ org.freedesktop.IBus /org/freedesktop/IBus
$ zbus-xmlgen interface.xml
```

# Getting Help

If you need help in using these crates, are looking for ways to contribute, or just want to hang out
with the cool kids, please come chat with us in the
[`#zbus:matrix.org`](https://matrix.to/#/#zbus:matrix.org) Matrix room. If something doesn't seem
right, please [file an issue](https://gitlab.freedesktop.org/dbus/zbus/-/issues/new).

# Portability

All crates are currently Unix-only and will fail to build on non-unix. This is hopefully a temporary
limitation. Moreover, integration tests of zbus crate currently require a session bus running on the
build host.

# License

MIT license [LICENSE-MIT](LICENSE-MIT)

[PolicyKit]: https://gitlab.freedesktop.org/polkit/polkit/
[iektc]: https://docs.rs/zbus/2.0.0-beta.5/zbus/azync/struct.Connection.html#method.executor

[^otheros]: Support for other OS exist, but it is not supported to the same extent. D-Bus clients in
  javascript (running from any browser) do exist though. And zbus may also be working from the
  browser sometime in the future too, thanks to Rust 🦀 and WebAssembly 🕸.

[^stability]: We might have to change the API but zbus follows semver convention so your code
  won't just break out of the blue. Just make sure you depend on a specific major version of zbus.
