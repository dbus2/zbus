# Writing a server interface

Let see how to provide a server method "SayHello", to greet a client.

## Taking a service name

As we know from the chapter on [D-Bus concepts], each connection on the bus is given a unique name
(such as ":1.27"). This could be all you need, depending on your use case, and the design of your
D-Bus API. However, typically services use a service name (aka *well-known name*) so peers (clients,
in this context) can easily discover them.

In this example, that is exactly what we're going to do and request the bus for the service name of
our choice. To achieve that, we will we call the [`RequestName`] method on the bus, using
`zbus::fdo` module:

```rust,no_run
use std::error::Error;

use zbus::Connection;
use zbus::fdo;

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let connection = Connection::new_session()?;

    fdo::DBusProxy::new(&connection)?.request_name(
        "org.zbus.MyGreeter",
        fdo::RequestNameFlags::ReplaceExisting.into(),
    )?;

    loop {}
}
```

We can check our service is running and is associated with the service name:

```bash
$ busctl --user list | grep zbus
org.zbus.MyGreeter                             412452 server            elmarco :1.396        user@1000.service -       -
```

### ⚠ Hang on

This example is not handling incoming messages yet. Any attempt to call the server will time out
(including the shell completion!).

## Handling low-level messages

At the low-level, you can handle method calls by checking the incoming messages manually.

Let's write a `SayHello` method, that takes a string as argument, and reply with a "hello" greeting
by replacing the loop above with this code:

```rust,no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
#    let connection = zbus::Connection::new_session()?;
#    zbus::fdo::DBusProxy::new(&connection)?.request_name(
#        "org.zbus.MyGreeter",
#        zbus::fdo::RequestNameFlags::ReplaceExisting.into(),
#    )?;
#
loop {
    let msg = connection.receive_message()?;
    let msg_header = msg.header()?;
    dbg!(&msg);

    match msg_header.message_type()? {
        zbus::MessageType::MethodCall => {
            // real code would check msg_header path(), interface() and member()
            // handle invalid calls, introspection, errors etc
            let arg: &str = msg.body()?;
            connection.reply(&msg, &(format!("Hello {}!", arg)))?;
        }
        _ => continue,
    }
}
# }
```

And check if it works as expected:

```bash
$ busctl --user call org.zbus.MyGreeter /org/zbus/MyGreeter org.zbus.MyGreeter1 SayHello s "zbus"
s "Hello zbus!"
```

This is the crust of low-level message handling. It should give you all the flexibility you ever
need, but it is also easy to get it wrong. Fortunately, zbus has a simpler solution to offer.

## Using the `ObjectServer`

One can write an `impl` with a set of methods and let the `dbus_interface` procedural macro write
the D-Bus details for us. It will dispatch all the incoming method calls to their respective
handlers, and implicilty handle introspection requests. It also has support for properties and
signal emission.

Let see how to use it:

```rust,no_run
# use std::error::Error;
# use std::convert::TryInto;
# use zbus::{dbus_interface, fdo};
#
struct Greeter;

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let connection = zbus::Connection::new_session()?;
#     fdo::DBusProxy::new(&connection)?.request_name(
#         "org.zbus.MyGreeter",
#         fdo::RequestNameFlags::ReplaceExisting.into(),
#     )?;

    let object_server = zbus::ObjectServer::new(&connection);
    object_server.at(&"/org/zbus/MyGreeter".try_into()?, Greeter)?;
    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("{}", err);
        }
    }

}
```

(check it works with the same `busctl` command as last time)

This time, we can also introspect the server:

```bash
$ busctl --user introspect org.zbus.MyGreeter /org/zbus/MyGreeter
NAME                                TYPE      SIGNATURE RESULT/VALUE FLAGS
org.freedesktop.DBus.Introspectable interface -         -            -
.Introspect                         method    -         s            -
org.freedesktop.DBus.Peer           interface -         -            -
.GetMachineId                       method    -         s            -
.Ping                               method    -         -            -
org.freedesktop.DBus.Properties     interface -         -            -
.Get                                method    ss        v            -
.GetAll                             method    s         a{sv}        -
.Set                                method    ssv       -            -
.PropertiesChanged                  signal    sa{sv}as  -            -
org.zbus.MyGreeter1                 interface -         -            -
.SayHello                           method    s         s            -
```

Easy-peasy!

> **Note:** As you must have noticed, your code needed to run a loop to continuously read incoming
messages (register the associated FD for input in poll/select to avoid blocking). This is because
at the time of the this writing (*pre-1.0*), zbus neither provides an event loop API, nor any
integration with other event loop implementations. We are evaluating different options to make this
easier, especially with *async* support.

[D-Bus concepts]: concepts.html#bus-name--service-name
[`RequestName`]: https://dbus.freedesktop.org/doc/dbus-specification.html#bus-messages-request-name
