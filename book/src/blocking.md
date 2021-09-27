> **Note**
>
> This version of the book is based on zbus 2.0 API, which is currently in beta stages. For using the
> sample code in this book, you'll need to explicitly depend on the
> [latest beta](https://crates.io/crates/zbus/2.0.0-beta.6).
>
> The 1.0 version of this book is available [here](https://dbus.pages.freedesktop.org/zbus/1.0/).

# Blocking API

While zbus API being primarily asynchronous (since 2.0) is a great thing, it could easily feel
daunting for simple use cases. Not to worry! In the spirit of "ease" being a primary goal of zbus,
it provides blocking wrapper types, under the [blocking module].

## Establishing a connection

The only difference to that of [asynchronous `Connection` API] is that you use
[`blocking::Connection`] type instead. This type's API is almost identical to that of `Connection`,
except all its methods are blocking. One notable difference is that there is no equivalent of
[`futures::sink::Sink`] implementation provided. There is however [`blocking::MessageStream`] type,
that implements [`std::iter::Iterator`].

## Client

Similar to `blocking::Connection`, you use `blocking::Proxy` type. Its constructors require
`blocking::Connection` instead of `Connection`. Moreover, `dbus_proxy` macro generates a
`blocking::Proxy` wrapper for you as well. Let's convert the last example in the previous chapter,
to use the blocking connection and proxy:

```rust,no_run
use zbus::{blocking::Connection, dbus_proxy, Result};
use zvariant::{ObjectPath, OwnedObjectPath};

#[dbus_proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Manager",
    default_path = "/org/freedesktop/GeoClue2/Manager"
)]
trait Manager {
    #[dbus_proxy(object = "Client")]
    /// The method normally returns an `ObjectPath`.
    /// With the object attribute, we can make it return a `ClientProxy` directly.
    fn get_client(&self);
}

#[dbus_proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Client"
)]
trait Client {
    fn start(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;

    #[dbus_proxy(property)]
    fn set_desktop_id(&mut self, id: &str) -> Result<()>;

    #[dbus_proxy(signal)]
    fn location_updated(&self, old: ObjectPath<'_>, new: ObjectPath<'_>) -> Result<()>;
}

#[dbus_proxy(
    default_service = "org.freedesktop.GeoClue2",
    interface = "org.freedesktop.GeoClue2.Location"
)]
trait Location {
    #[dbus_proxy(property)]
    fn latitude(&self) -> Result<f64>;
    #[dbus_proxy(property)]
    fn longitude(&self) -> Result<f64>;
}
let conn = Connection::system().unwrap();
let manager = ManagerProxyBlocking::new(&conn).unwrap();
let mut client = manager.get_client().unwrap();
// Gotta do this, sorry!
client.set_desktop_id("org.freedesktop.zbus").unwrap();

let (tx, rx) = std::sync::mpsc::channel();
client
    .connect_location_updated(move |_old, new| {
        let location = LocationProxyBlocking::builder(&conn)
            .path(new.as_str())
            .unwrap()
            .build()
            .unwrap();
        println!(
            "Latitude: {}\nLongitude: {}",
            location.latitude().unwrap(),
            location.longitude().unwrap(),
        );
        tx.send(()).unwrap();
    })
    .unwrap();

client.start().unwrap();

// Wait till there is a signal that was handled.
rx.recv().unwrap();
```

As you can see, nothing changed in the `dbus_proxy` usage here and the rest largely remained the
same as well.

### Blocking Signal Streams

Currently zbus does not provide a blocking wrapper for signal stream. However, we intend to fix this
in the future.

### Watching for properties

Similar to signals, the only blocking API to watch for changes in properties is currently
callback-based only:

```rust,no_run
# use std::{thread::sleep, time::Duration};
# use zbus::{blocking::Connection, dbus_proxy, Result};
#
#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    #[dbus_proxy(property)]
    fn log_level(&self) -> zbus::Result<String>;
}

fn main() -> Result<()> {
    let connection = Connection::session()?;

    let proxy = SystemdManagerProxyBlocking::new(&connection)?;
    proxy.connect_log_level_changed(|v| {
        println!("LogLevel changed: {:?}", v);
    });

    // Do other things or go to sleep.
    sleep(Duration::from_secs(60));

    Ok(())
}
```

## Server

Similarly here, you'd use [`blocking::ObjectServer`] that is associated with every
[`blocking::Connection`] instance. While there is no blocking version of `Interface`,
`dbus_interface` allows you to write non-async methods.

**Note:** Even though you can write non-async methods, these methods are still called from an async
context. Therefore, you can not use blocking API that's a wrapper of an async API. This is not a
limitation of zbus but rather a [well-known general problem][wkgp] in the Rust async/await world.
The [`blocking` crate] provides an easy way around this problem.

```rust,no_run
# use std::error::Error;
# use zbus::{SignalContext, blocking::{ObjectServer, Connection}, dbus_interface, fdo};
#
use event_listener::Event;

struct Greeter {
    name: String,
    done: Event,
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }

    // Rude!
    fn go_away(&self) {
        self.done.notify(1);
    }

    /// A "GreeterName" property.
    #[dbus_interface(property)]
    fn greeter_name(&self) -> &str {
        &self.name
    }

    /// A setter for the "GreeterName" property.
    ///
    /// Additionally, a `greeter_name_changed` method has been generated for you if you need to
    /// notify listeners that "GreeterName" was updated. It will be automatically called when
    /// using this setter.
    #[dbus_interface(property)]
    fn set_greeter_name(&mut self, name: String) {
        self.name = name;
    }

    /// A signal; the implementation is provided by the macro.
    #[dbus_interface(signal)]
    async fn greeted_everyone(ctxt: &SignalContext<'_>) -> zbus::Result<()>;
}

fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session()?;
    let greeter = Greeter {
        name: "GreeterName".to_string(),
        done: event_listener::Event::new(),
    };
    let done_listener = greeter.done.listen();
    connection
        .object_server_mut()
        .at("/org/zbus/MyGreeter", greeter)?;
    connection.request_name("org.zbus.MyGreeter")?;

    done_listener.wait();

    Ok(())
}
```

[asynchronous `Connection` API]: https://docs.rs/zbus/2.0.0-beta.7/zbus/struct.Connection.html
[`blocking::Connection`]: https://docs.rs/zbus/2.0.0-beta.7/zbus/blocking/struct.Connection.html
[`futures::sink::Sink`]: https://docs.rs/futures/latest/futures/sink/trait.Sink.html
[`std::iter::Iterator`]: https://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html
[blocking module]: https://docs.rs/zbus/2.0.0-beta.7/zbus/blocking/index.html
[wkgp]: https://rust-lang.github.io/wg-async-foundations/vision/shiny_future/users_manual.html#caveat-beware-the-async-sandwich
[`blocking` crate]: https://docs.rs/blocking/
