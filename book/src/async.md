# Let's Go Asynchronous

Not only does zbus also provides with asynchronous API, most of the synchronous API you saw in
action already, is in fact just thin wrappers around its asynchronous counterpart. Since you're now
already familiar the synchronous API, in this chapter we'll focus on making the earlier code
samples, asynchronous.

## Establishing a connection

The only difference to that of [synchronous `Connection` API] is that you use [`azync::Connection`]
type instead. This type's API is almost identical to that of `Connection`, except its asynchronous.
Moreover, it also provides a [`futures::stream::Stream`] and [`futures::sink::Sink`] implementations
to conveniently receive and send messages, respectively for the times when low-level API is more
appropriate for your use case.

## Client

Similar to `Connection`, you use `azync::Proxy` type. Its constructors require `azync::Connection`
instead of `Connection`. Moreover, `dbus_proxy` macro generates an `azync::Proxy` wrapper for you
as well. Let's convert the last example in the previous chapter, to use the asynchronous connection
and proxy:

```rust,no_run
use futures_util::future::FutureExt;
use zbus::{azync::Connection, dbus_proxy, Result};
use zvariant::ObjectPath;

# async_io::block_on(run()).unwrap();
#
async fn run() -> Result<()> {
    #[dbus_proxy(
        default_service = "org.freedesktop.GeoClue2",
        interface = "org.freedesktop.GeoClue2.Manager",
        default_path = "/org/freedesktop/GeoClue2/Manager"
    )]
    trait Manager {
        #[dbus_proxy(object = "Client")]
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
    let conn = Connection::new_system().await?;
    let manager = AsyncManagerProxy::new(&conn).await?;
    let mut client = manager.get_client().await?;
    // Gotta do this, sorry!
    client.set_desktop_id("org.freedesktop.zbus").await?;

    client
        .connect_location_updated(move |_old, new| {
            let new = new.to_string();
            let conn = conn.clone();

            async move {
                let location = AsyncLocationProxy::builder(&conn)
                    .path(new)?
                    .build_async()
                    .await?;
                println!(
                    "Latitude: {}\nLongitude: {}",
                    location.latitude().await?,
                    location.longitude().await?,
                );

                Ok(())
            }
            .boxed()
        })
        .await?;

    client.start().await?;

    // Wait till there is a signal that was handled.
    while client.next_signal().await?.is_some() {}

    Ok(())
}
```

As you can see, nothing changed in the `dbus_proxy` usage here and the rest largely remained the
same as well.

### Receiving multiple signals, simultaneously

The asynchronous API also doesn't include an equivalent of
[`SignalReceiver`](https://docs.rs/zbus/1.5.0/zbus/struct.SignalReceiver.html). This is because
[`futures`](https://crates.io/crates/futures) crate (and others) already provide a rich API to
combine asynchronous operations in various ways. Let's see that in action by converting the above
example again to receive multiple signals on different proxies:

```rust,no_run
# use futures_util::future::FutureExt;
# use zbus::{azync::Connection, dbus_proxy, Result};
# use zvariant::ObjectPath;
#
# async_io::block_on(run()).unwrap();
#
# async fn run() -> Result<()> {
#     #[dbus_proxy(
#         default_service = "org.freedesktop.GeoClue2",
#         interface = "org.freedesktop.GeoClue2.Manager",
#         default_path = "/org/freedesktop/GeoClue2/Manager"
#     )]
#     trait Manager {
#         #[dbus_proxy(object = "Client")]
#         fn get_client(&self);
#     }
#
#     #[dbus_proxy(
#         default_service = "org.freedesktop.GeoClue2",
#         interface = "org.freedesktop.GeoClue2.Client"
#     )]
#     trait Client {
#         fn start(&self) -> Result<()>;
#         fn stop(&self) -> Result<()>;
#
#         #[dbus_proxy(property)]
#         fn set_desktop_id(&mut self, id: &str) -> Result<()>;
#
#         #[dbus_proxy(signal)]
#         fn location_updated(&self, old: ObjectPath<'_>, new: ObjectPath<'_>) -> Result<()>;
#     }
#
#     #[dbus_proxy(
#         default_service = "org.freedesktop.GeoClue2",
#         interface = "org.freedesktop.GeoClue2.Location"
#     )]
#     trait Location {
#         #[dbus_proxy(property)]
#         fn latitude(&self) -> Result<f64>;
#         #[dbus_proxy(property)]
#         fn longitude(&self) -> Result<f64>;
#     }
#     let conn = Connection::new_system().await?;
#     let manager = AsyncManagerProxy::new(&conn).await?;
#     let mut client = manager.get_client().await?;
#
	// Everything else remains the same before this point.
    client.set_desktop_id("org.freedesktop.zbus").await?;

    let props = zbus::fdo::AsyncPropertiesProxy::builder(&conn)
        .destination("org.freedesktop.GeoClue2")?
        .path(client.path())?
        .build_async()
        .await?;
    props
        .connect_properties_changed(move |iface, changed, _| {
            for (name, value) in changed.iter() {
                println!("{}.{} changed to `{:?}`", iface, name, value);
            }

            async { Ok(()) }.boxed()
        })
        .await?;

    client
        .connect_location_updated(move |_old, new| {
            let new = new.to_string();
            let conn = conn.clone();

            async move {
                let location = AsyncLocationProxy::builder(&conn)
                    .path(new)?
                    .build_async()
                    .await?;
                println!(
                    "Latitude: {}\nLongitude: {}",
                    location.latitude().await?,
                    location.longitude().await?,
                );

                Ok(())
            }
            .boxed()
        })
        .await?;

    client.start().await?;

    futures_util::try_join!(
        async {
            while props.next_signal().await?.is_some() {}

            Ok::<(),zbus::Error >(())
        },
        async {
            while client.next_signal().await?.is_some() {}

            // No need to specify type of Result each time
            Ok(())
        }
    )?;
#
#   Ok(())
# }
```

### Signal Streams: A better way

While you can connect your callbacks to receive signals (as we saw in the previous example), zbus
also provides another method of receiving signals with better ergonomics for use in typical
asynchronous Rust code: signal streams. Let's change the previous example to make use of signal
streams to see how that works:

```rust,no_run
    // Instead of `futures_util::future::FutureExt`
    use futures_util::stream::StreamExt;

# use zbus::{azync::Connection, dbus_proxy, Result};
# use zvariant::ObjectPath;
#
# async_io::block_on(run()).unwrap();
#
# async fn run() -> Result<()> {
#     #[dbus_proxy(
#         default_service = "org.freedesktop.GeoClue2",
#         interface = "org.freedesktop.GeoClue2.Manager",
#         default_path = "/org/freedesktop/GeoClue2/Manager"
#     )]
#     trait Manager {
#         #[dbus_proxy(object = "Client")]
#         fn get_client(&self);
#     }
#
#     #[dbus_proxy(
#         default_service = "org.freedesktop.GeoClue2",
#         interface = "org.freedesktop.GeoClue2.Client"
#     )]
#     trait Client {
#         fn start(&self) -> Result<()>;
#         fn stop(&self) -> Result<()>;
#
#         #[dbus_proxy(property)]
#         fn set_desktop_id(&mut self, id: &str) -> Result<()>;
#
#         #[dbus_proxy(signal)]
#         fn location_updated(&self, old: ObjectPath<'_>, new: ObjectPath<'_>) -> Result<()>;
#     }
#
#     #[dbus_proxy(
#         default_service = "org.freedesktop.GeoClue2",
#         interface = "org.freedesktop.GeoClue2.Location"
#     )]
#     trait Location {
#         #[dbus_proxy(property)]
#         fn latitude(&self) -> Result<f64>;
#         #[dbus_proxy(property)]
#         fn longitude(&self) -> Result<f64>;
#     }
#     let conn = Connection::new_system().await?;
#     let manager = AsyncManagerProxy::new(&conn).await?;
#     let mut client = manager.get_client().await?;
#
#   client.set_desktop_id("org.freedesktop.zbus").await?;
#
    // Everything else remains the same before this point.
    let props = zbus::fdo::AsyncPropertiesProxy::builder(&conn)
        .destination("org.freedesktop.GeoClue2")?
        .path(client.path())?
        .build_async()
        .await?;
    let mut props_changed_stream = props.receive_properties_changed().await?;
    let mut location_updated_stream = client.receive_location_updated().await?;

    client.start().await?;

    futures_util::try_join!(
        async {
            while let Some(signal) = props_changed_stream.next().await {
                let args = signal.args()?;

                for (name, value) in args.changed_properties().iter() {
                    println!("{}.{} changed to `{:?}`", args.interface_name(), name, value);
                }
            }

            Ok::<(), zbus::Error>(())
        },
        async {
            while let Some(signal) = location_updated_stream.next().await {
                let args = signal.args()?;

                let location = AsyncLocationProxy::builder(&conn)
                    .path(args.new())?
                    .build_async()
                    .await?;
                println!(
                    "Latitude: {}\nLongitude: {}",
                    location.latitude().await?,
                    location.longitude().await?,
                );
            }

            // No need to specify type of Result each time
            Ok(())
        }
    )?;
#
#   Ok(())
# }
```

### Watching for properties

Use the property stream API offered by the proxy to be notified of changes. The functions are named
after the properties `receive_<prop_name>_changed()`. Example:

```rust,no_run
# use std::error::Error;
# use zbus::{azync::Connection, dbus_proxy, Result};
# use futures_util::stream::StreamExt;
#
# async_io::block_on(run()).unwrap();
#
# async fn run() -> Result<()> {
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

    let connection = Connection::new_session().await?;

    let proxy = AsyncSystemdManagerProxy::new(&connection).await?;
    let mut stream = proxy.receive_log_level_changed().await;
    while let Some(v) = stream.next().await {
        println!("LogLevel changed: {:?}", v);
    }
#
#   Ok(())
# }
```

## Server

No high-level server-side API are provided yet. Rest assured, it's very high on our priority list.
Stay tuned!

[synchronous `Connection` API]: https://docs.rs/zbus/2.0.0-beta.5/zbus/struct.Connection.html
[`azync::Connection`]: https://docs.rs/zbus/2.0.0-beta.5/zbus/azync/connection/struct.Connection.html
[`futures::stream::Stream`]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
[`futures::sink::Sink`]: https://docs.rs/futures/latest/futures/sink/trait.Sink.html
