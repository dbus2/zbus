> **Note**
>
> This version of the book is based on zbus 2.0 API, which is currently in beta stages. For using the
> sample code in this book, you'll need to explicitly depend on the
> [latest beta](https://crates.io/crates/zbus/2.0.0-beta.8).
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
[`futures::sink::Sink`] implementation provided. There is however [`blocking::MessageIterator`]
type, that implements [`std::iter::Iterator`].

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

let mut location_updated = client.receive_location_updated().unwrap();

client.start().unwrap();

// Wait for the signal.
let signal = location_updated.next().unwrap();
let args = signal.args().unwrap();

let location = LocationProxyBlocking::builder(&conn)
    .path(args.new())
    .unwrap()
    .build()
    .unwrap();
println!(
    "Latitude: {}\nLongitude: {}",
    location.latitude().unwrap(),
    location.longitude().unwrap(),
);
```

As you can see, nothing changed in the `dbus_proxy` usage here and the rest largely remained the
same as well. One difference that's not obvious is that the blocking API for receiving signals,
implement [`std::iter::Iterator`] trait instead of [`futrues::stream::Stream`].

### Watching for properties

That's almost the same as receiving signals:

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
    let v = proxy.receive_log_level_changed().next().unwrap();
    println!("LogLevel changed: {:?}", v.get());

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
# use zbus::{blocking::{ObjectServer, ConnectionBuilder}, dbus_interface, fdo, SignalContext};
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
    let greeter = Greeter {
        name: "GreeterName".to_string(),
        done: event_listener::Event::new(),
    };
    let done_listener = greeter.done.listen();
    let _ = ConnectionBuilder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()?;

    done_listener.wait();

    Ok(())
}
```

[asynchronous `Connection` API]: https://docs.rs/zbus/2.0.0-beta.8/zbus/struct.Connection.html
[`blocking::Connection`]: https://docs.rs/zbus/2.0.0-beta.8/zbus/blocking/struct.Connection.html
[`futures::sink::Sink`]: https://docs.rs/futures/latest/futures/sink/trait.Sink.html
[`std::iter::Iterator`]: https://doc.rust-lang.org/nightly/std/iter/trait.Iterator.html
[blocking module]: https://docs.rs/zbus/2.0.0-beta.8/zbus/blocking/index.html
[wkgp]: https://rust-lang.github.io/wg-async-foundations/vision/shiny_future/users_manual.html#caveat-beware-the-async-sandwich
[`blocking` crate]: https://docs.rs/blocking/
[`futures::stream::Stream`]: https://docs.rs/futures/0.3.17/futures/stream/trait.Stream.html
