# Writing a client proxy

In this chapter, we are going to see how to make low-level D-Bus method calls. Then we are going to
dive in, and derive from a trait to make a convenient Rust binding. Finally, we will learn about
*xmlgen*, a tool to help us generate a boilerplate trait from the XML of an introspected service.

To make this learning "hands-on", we are going to call and bind the cross-desktop notification
service (please refer to this
[reference](https://people.gnome.org/~mccann/docs/notification-spec/notification-spec-latest.html)
document for further details on this API).

Let's start by playing with the service from the shell, and notify the desktop with [`busctl`][^busctl]:

```bash
busctl --user call \
  org.freedesktop.Notifications \
  /org/freedesktop/Notifications \
  org.freedesktop.Notifications \
  Notify \
  susssasa\{sv\}i \
  "my-app" 0 "dialog-information" "A summary" "Some body" 0 0 5000
```

**Note**: `busctl` has very good auto-completion support in bash or zsh.

Running this command should pop-up a notification dialog on your desktop. If it does not, your
desktop does not support the notification service, and this example will be less interactive.
Nonetheless you can use a similar approach for other services.

This command shows us several aspects of the D-Bus communication:

 - `--user`: Connect to and use the user/session bus.

 - `call`: Send a method call message. (D-Bus also supports signals, error messages, and method
   replies)

 - **destination**: The name of the service (`org.freedesktop.Notifications`).

 - **object path**: Object/interface path (`/org/freedesktop/Notifications`).

 - **interface**: The interface name (methods are organized in interfaces, here
   `org.freedesktop.Notifications`, same name as the service).

 - **method**: The name of the method to call, `Notify`.

 - **signature**: That `susssasa{sv}i` means the method takes 8 arguments of various types. 's', for
   example, is for a string. 'as' is for array of strings.

 - The method arguments.

See [`busctl`] man page for more details.

## Low-level call from a `zbus::Connection`

zbus `Connection` has a `call_method()` method, which you can use directly:

```rust,no_run
use std::collections::HashMap;
use std::error::Error;

use zbus::Connection;
use zvariant::Value;

fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::new_session()?;

    let m = connection.call_method(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        Some("org.freedesktop.Notifications"),
        "Notify",
        &("my-app", 0u32, "dialog-information", "A summary", "Some body",
          vec![""; 0], HashMap::<&str, &Value>::new(), 5000),
    )?;
    let reply: u32 = m.body().unwrap();
    dbg!(reply);
    Ok(())
}
```

Although this is already quite flexible, and handles various details for you (such as the message
signature), it is also somewhat inconvenient and error-prone: you can easily miss arguments, or give
arguments with the wrong type or other kind of errors (what would happen if you typed `0`, instead
of `0u32`?).

Instead, we want to wrap this `Notify` D-Bus method in a Rust function. Let's see how next.

## Trait-derived proxy call

A trait declaration `T` with a `dbus_proxy` attribute will have a derived `TProxy` implemented
thanks to procedural macros. The trait methods will have respective `impl` methods wrapping the
D-Bus calls:

```rust,no_run
use std::collections::HashMap;
use std::error::Error;

use zbus::dbus_proxy;
use zvariant::Value;

#[dbus_proxy]
trait Notifications {
    /// Call the org.freedesktop.Notifications.Notify D-Bus method
    fn notify(&self,
              app_name: &str,
              replaces_id: u32,
              app_icon: &str,
              summary: &str,
              body: &str,
              actions: &[&str],
              hints: HashMap<&str, &Value<'_>>,
              expire_timeout: i32) -> zbus::Result<u32>;
}

fn main() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;

    let proxy = NotificationsProxy::new(&connection)?;
    let reply = proxy.notify("my-app", 0, "dialog-information", "A summary", "Some body",
                             &[], HashMap::new(), 5000)?;
    dbg!(reply);

    Ok(())
}
```

A `TProxy` has a few associated methods, such as `new(connection)`, using the default associated
service name and object path, and an associated builder if you need to specify something different.

This should help to avoid the kind of mistakes we saw earlier. It's also a bit easier to use, thanks
to Rust type inference. This makes it also possible to have higher-level types, they fit more
naturally with the rest of the code. You can further document the D-Bus API or provide additional
helpers.

### Properties

Interfaces can have associated properties, which can be read or set with the
`org.freedesktop.DBus.Properties` interface. Here again, the `#[dbus_proxy]` attribute comes to the
rescue to help you. You can annotate a trait method to be a getter:

```rust
# use zbus::{dbus_proxy, Result};
#
#[dbus_proxy]
trait MyInterface {
    #[dbus_proxy(property)]
    fn state(&self) -> Result<String>;
}
```

The `state()` method will translate to a `"State"` property `Get` call.

To set the property, prefix the name of the property with `set_`.

For a more real world example, let's try and read two properties from systemd's main service:

```rust,no_run
# use std::error::Error;
# use zbus::dbus_proxy;
#
#[dbus_proxy(
    interface = "org.freedesktop.systemd1.Manager",
    default_service = "org.freedesktop.systemd1",
    default_path = "/org/freedesktop/systemd1"
)]
trait SystemdManager {
    #[dbus_proxy(property)]
    fn architecture(&self) -> zbus::Result<String>;
    #[dbus_proxy(property)]
    fn environment(&self) -> zbus::Result<Vec<String>>;
}

fn main() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;

    let proxy = SystemdManagerProxy::new(&connection)?;
    println!("Host architecture: {}", proxy.architecture()?);
    println!("Environment:");
    for env in proxy.environment()? {
        println!("  {}", env);
    }

    Ok(())
}
```

You should get an output similar to this:

```none
Host architecture: x86-64
Environment variables:
  HOME=/home/zeenix
  LANG=en_US.UTF-8
  LC_ADDRESS=de_DE.UTF-8
  LC_IDENTIFICATION=de_DE.UTF-8
  LC_MEASUREMENT=de_DE.UTF-8
  LC_MONETARY=de_DE.UTF-8
  LC_NAME=de_DE.UTF-8
  LC_NUMERIC=de_DE.UTF-8
  LC_PAPER=de_DE.UTF-8
  LC_TELEPHONE=de_DE.UTF-8
  LC_TIME=de_DE.UTF-8
  ...
```

#### Watching for changes

By default, the proxy will cache the properties and watch for changes.

To be notified of a property change, you may use the synchronous API with the properties callback.
The methods are named after the properties names `connect_<prop_name>_changed`, for example:

```rust,no_run
# use std::error::Error;
# use zbus::dbus_proxy;
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

fn main() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;

    let proxy = SystemdManagerProxy::new(&connection)?;
    proxy.connect_log_level_changed(|v| {
        println!("LogLevel changed: {:?}", v);
    });

    Ok(())
}
```

(see the next chapter for the async stream API version)

### Signals

Signals are like methods, except they don't expect a reply. They are typically emitted by services
to notify interested peers of any changes to the state of the service. zbus provides you with an API
to register signal handler functions, and to receive and call them.

Let's look at this API in action, with an example where we get our location from
[Geoclue](https://gitlab.freedesktop.org/geoclue/geoclue/-/blob/master/README.md):

```rust,no_run
use zbus::{Connection, dbus_proxy, Result};
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
let conn = Connection::new_system().unwrap();
let manager = ManagerProxy::new(&conn).unwrap();
let mut client = manager.get_client().unwrap();
// Gotta do this, sorry!
client.set_desktop_id("org.freedesktop.zbus").unwrap();

client
    .connect_location_updated(move |_old, new| {
        let location = LocationProxy::builder(&conn)
            .path(new.as_str())?
            .build()
            .unwrap();
        println!(
            "Latitude: {}\nLongitude: {}",
            location.latitude()?,
            location.longitude()?,
        );

        Ok(())
    })
    .unwrap();

client.start().unwrap();

// Wait till there is a signal that was handled.
while client.next_signal().unwrap().is_some() {}
```

While the Geoclue's D-Bus API is a bit involved, we still ended-up with a not-so-complicated (~60
LOC) code for getting our location. As you may've notice, we use a blocking call to wait for a
signal on one proxy. This works fine but in the real world, you would typically have many proxies
and you'd want to wait for signals from them all at once. Not to worry, zbus provides a way to wait
on [multiple proxies at once as well](https://docs.rs/zbus/1.5.0/zbus/struct.SignalReceiver.html).

Let's make use of `SignalReceiver` and `zbus::fdo` API to make sure the client is actually started
by watching for `Active` property (that we must set to be able to get location from Geoclue)
actually getting set:

```rust,no_run
# use zbus::{Connection, dbus_proxy, Result};
# use zvariant::{ObjectPath, OwnedObjectPath};
#
# #[dbus_proxy(
#     default_service = "org.freedesktop.GeoClue2",
#     interface = "org.freedesktop.GeoClue2.Manager",
#     default_path = "/org/freedesktop/GeoClue2/Manager"
# )]
# trait Manager {
#     #[dbus_proxy(object = "Client")]
#     fn get_client(&self);
# }
#
# #[dbus_proxy(interface = "org.freedesktop.GeoClue2.Client")]
# trait Client {
#     fn start(&self) -> Result<()>;
#
#     #[dbus_proxy(property)]
#     fn set_desktop_id(&mut self, id: &str) -> Result<()>;
#
#     #[dbus_proxy(signal)]
#     fn location_updated(&self, old: ObjectPath<'_>, new: ObjectPath<'_>) -> Result<()>;
# }
#
# #[dbus_proxy(
#     default_service = "org.freedesktop.GeoClue2",
#     interface = "org.freedesktop.GeoClue2.Location"
# )]
# trait Location {
#     #[dbus_proxy(property)]
#     fn latitude(&self) -> Result<f64>;
#     #[dbus_proxy(property)]
#     fn longitude(&self) -> Result<f64>;
# }
#
# let conn = Connection::new_system().unwrap();
# let manager = ManagerProxy::new(&conn).unwrap();
# let mut client = manager.get_client().unwrap();
# // Gotta do this, sorry!
# client.set_desktop_id("org.freedesktop.zbus").unwrap();
#
// Everything else remains the same before this point.

let conn_clone = conn.clone();
client.connect_location_updated(move |_old, new| {
    let location = LocationProxy::builder(&conn_clone)
        .destination("org.freedesktop.GeoClue2")
        .path(new.as_str())?
        .build()
        .unwrap();
    println!(
        "Latitude: {}\nLongitude: {}",
        location.latitude()?,
        location.longitude()?,
    );

    Ok(())
}).unwrap();

let props = zbus::fdo::PropertiesProxy::builder(&conn)
    .destination("org.freedesktop.GeoClue2")
    .path(client.path()).unwrap()
    .build()
    .unwrap();
props.connect_properties_changed(|iface, changed, _| {
    for (name, value) in changed.iter() {
        println!("{}.{} changed to `{:?}`", iface, name, value);
    }

    Ok(())
}).unwrap();

let mut receiver = zbus::SignalReceiver::new(conn);
receiver.receive_for(&client).unwrap();
receiver.receive_for(&props).unwrap();

client.start().unwrap();

// 3 signals will be emitted, that we handle
let mut num_handled = 0;
while num_handled < 3 {
    if receiver.next_signal().unwrap().is_none() {
        num_handled += 1;
    }
}
```

## Generating the trait from an XML interface

The `zbus_xmlgen` crate provides a [developer-friendly tool], that can generate Rust traits from a
given D-Bus introspection XML for you.

**Note:** This tool should not be considered a drop-in Rust-specific replacement for similar tools
available for low-level languages, such as [`gdbus-codegen`]. Unlike those tools, this is only meant
as a starting point to generate the code, once. In many cases, you will want to tweak the generated
code.

The tool can be used to generate rust code directly from a D-Bus service running on our system:

```bash
zbus-xmlgen --session \
  org.freedesktop.Notifications \
  /org/freedesktop/Notifications
```

Alternatively you can also get the XML interface from a different source and use it to generate the
interface code. Some packages may also provide the XML directly as an installed file, allowing it to
be queried using [`pkg-config`], for example.

We can fetch the XML interface of the notification service, using the `--xml-interface` option of
the `busctl`[^busctl] command. This option was introduced to `busctl` in systemd v243.

```bash
busctl --user --xml-interface introspect \
  org.freedesktop.Notifications \
  /org/freedesktop/Notifications
```

You should get a similar output:

```xml
<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN"
                      "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
<node>
  <!-- other interfaces omitted -->
  <interface name="org.freedesktop.Notifications">
    <method name="Notify">
      <arg type="s" name="arg_0" direction="in">
      </arg>
      <arg type="u" name="arg_1" direction="in">
      </arg>
      <arg type="s" name="arg_2" direction="in">
      </arg>
      <arg type="s" name="arg_3" direction="in">
      </arg>
      <arg type="s" name="arg_4" direction="in">
      </arg>
      <arg type="as" name="arg_5" direction="in">
      </arg>
      <arg type="a{sv}" name="arg_6" direction="in">
      </arg>
      <arg type="i" name="arg_7" direction="in">
      </arg>
      <arg type="u" name="arg_8" direction="out">
      </arg>
    </method>
    <method name="CloseNotification">
      <arg type="u" name="arg_0" direction="in">
      </arg>
    </method>
    <method name="GetCapabilities">
      <arg type="as" name="arg_0" direction="out">
      </arg>
    </method>
    <method name="GetServerInformation">
      <arg type="s" name="arg_0" direction="out">
      </arg>
      <arg type="s" name="arg_1" direction="out">
      </arg>
      <arg type="s" name="arg_2" direction="out">
      </arg>
      <arg type="s" name="arg_3" direction="out">
      </arg>
    </method>
    <signal name="NotificationClosed">
      <arg type="u" name="arg_0">
      </arg>
      <arg type="u" name="arg_1">
      </arg>
    </signal>
    <signal name="ActionInvoked">
      <arg type="u" name="arg_0">
      </arg>
      <arg type="s" name="arg_1">
      </arg>
    </signal>
  </interface>
</node>
```

Save the output to a `notify.xml` file. Then call:

```bash
zbus-xmlgen notify.xml
```

This will give back effortlessly the corresponding Rust traits boilerplate
code:

```rust
# use zbus::dbus_proxy;
#
#[dbus_proxy(interface = "org.freedesktop.Notifications")]
trait Notifications {
    /// CloseNotification method
    fn close_notification(&self, arg_0: u32) -> zbus::Result<()>;

    /// GetCapabilities method
    fn get_capabilities(&self) -> zbus::Result<Vec<String>>;

    /// GetServerInformation method
    fn get_server_information(&self) -> zbus::Result<(String, String, String, String)>;

    /// Notify method
    fn notify(
        &self,
        arg_0: &str,
        arg_1: u32,
        arg_2: &str,
        arg_3: &str,
        arg_4: &str,
        arg_5: &[&str],
        arg_6: std::collections::HashMap<&str, zvariant::Value<'_>>,
        arg_7: i32,
    ) -> zbus::Result<u32>;

    /// ActionInvoked signal
    #[dbus_proxy(signal)]
    fn action_invoked(&self, arg_0: u32, arg_1: &str) -> zbus::Result<()>;

    /// NotificationClosed signal
    #[dbus_proxy(signal)]
    fn notification_closed(&self, arg_0: u32, arg_1: u32) -> zbus::Result<()>;
}
```

It should be usable as such. But you may as well improve a bit the naming of the arguments, use
better types (using `BitFlags`, structs or other custom types), add extra documentation, and other
functions to make the binding more pleasing to use from Rust.

For example, the generated `GetServerInformation` method can be improved to a nicer version:

```rust
# use serde::{Serialize, Deserialize};
# use zvariant::derive::Type;
# use zbus::dbus_proxy;
#
#[derive(Debug, Type, Serialize, Deserialize)]
pub struct ServerInformation {
    /// The product name of the server.
    pub name: String,

    /// The vendor name. For example "KDE," "GNOME," "freedesktop.org" or "Microsoft".
    pub vendor: String,

    /// The server's version number.
    pub version: String,

    /// The specification version the server is compliant with.
    pub spec_version: String,
}

trait Notifications {
    /// Get server information.
    ///
    /// This message returns the information on the server.
    fn get_server_information(&self) -> zbus::Result<ServerInformation>;
}
```

You can learn more from the zbus-ify [binding of
PolicyKit](https://gitlab.freedesktop.org/dbus/zbus/-/blob/main/zbus_polkit/src/policykit1.rs),
for example, which was implemented starting from the *xmlgen* output.

There you have it, a Rust-friendly binding for your D-Bus service!

[`busctl`]: https://www.freedesktop.org/software/systemd/man/busctl.html
[developer-friendly tool]: https://crates.io/crates/zbus_xmlgen
[`gdbus-codegen`]: https://developer.gnome.org/gio/stable/gdbus-codegen.html
[`pkg-config`]: https://www.freedesktop.org/wiki/Software/pkg-config/

[^busctl]: `busctl` is part of [`systemd`](https://www.freedesktop.org/wiki/Software/systemd/).
