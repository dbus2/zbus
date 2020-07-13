# Writing a client proxy

In this chapter, we are going to see how to make low-level D-Bus method calls. Then we are going to
dive in, and derive from a trait to make a convenient Rust binding. Finally, we will learn about
*xmlgen*, a tool to help us generate a boilerplate trait from the XML of an introspected service.

To make this learning "hands-on", we are going to call and bind the cross-desktop notification
service (please refer to this
[reference](https://people.gnome.org/~mccann/docs/notification-spec/notification-spec-latest.html)
document for further details on this API).

Let's start by playing with the service from the shell, and notify the desktop with [`busctl`]:

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
              hints: HashMap<&str, &Value>,
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
service name and object path, and `new_for(connection, service_name, object_path)` if you need to
specify something different.

This should help to avoid the kind of mistakes we saw earlier. It's also a bit easier to use, thanks
to Rust type inference. This makes it also possible to have higher-level types, they fit more
naturally with the rest of the code. You can further document the D-Bus API or provide additional
helpers.

### Properties

Interfaces can have associated properties, which can be read or set with the
`org.freedesktop.DBus.Properties` interface. Here again, the `#[dbus_proxy]` attribute comes to the
rescue to help you. You can annotate a trait method to be a getter:

```rust
# use zbus::dbus_proxy;
#
#[dbus_proxy]
trait MyInterface {
    #[dbus_proxy(property)]
    fn state(&self) -> zbus::Result<String>;
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

## Generating the trait from an XML interface

zbus git repository contains a [developer-friendly tool], that can generate Rust traits from a given
D-Bus introspection XML for you.

**Note:** This tool should not be considered a drop-in Rust-specific replacement for similar tools
available for low-level languages, such as [`gdbus-codegen`]. Unlike those tools, this is only meant
as a starting point to generate the code, once. In many cases, you will want to tweak the generated
code.

Let's first fetch the XML interface of the notification service:

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
        arg_6: std::collections::HashMap<&str, zvariant::Value>,
        arg_7: i32,
    ) -> zbus::Result<u32>;
}
```

It should be usable as such. But you may as well improve a bit the naming of the arguments, use
better types (using `BitFlags`, structs or other custom types), add extra documentation, and other
functions to make the binding more pleasing to use from Rust.

For example, the generated `GetServerInformation` method can be improved to a nicer version:

```rust
# use serde::{Serialize, Deserialize};
# use zvariant_derive::Type;
# use zbus::dbus_proxy;
#
#[derive(Debug, Type, Serialize, Deserialize)]
pub struct ServerInformation {
    /// The product name of the server.
    pub name: String,

    /// The vendor name. For example, "KDE," "GNOME," "freedesktop.org," or "Microsoft".
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
PolicyKit](https://gitlab.freedesktop.org/zeenix/zbus/-/blob/master/zbus_polkit/src/policykit1.rs),
for example, which was implemented starting from the *xmlgen* output.

**TODO:** add support for signal handlers.

There you have it, a Rust-friendly binding for your D-Bus service!

[`busctl`]: https://www.freedesktop.org/software/systemd/man/busctl.html
[developer-friendly tool]: https://gitlab.freedesktop.org/zeenix/zbus/-/tree/master/zbus_xmlgen
[`gdbus-codegen`]: https://developer.gnome.org/gio/stable/gdbus-codegen.html
