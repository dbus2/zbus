# Writing a server interface

Let see how to provide a server method "SayHello", to greet a client.

## Taking a service name

Each connection to the bus is given a unique name (such as ":1.27"), which is
enough to be reachable.

Depending on your use case, and the design of your program and protocol, it
might be enough.

In this example, we would like a simple way to reliably talk to our server. We
will ask the bus to associate our client with a service name (also called a
*[well-known name]*). This way, we don't have to lookup the unique name, which
would change every time.

To ask for a name, we send a [`RequestName`] method call to the bus, using
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

This example is not handling incoming messages yet. Any attempt to call the
server will time out (including the shell completion!).

## Handling low-level messages

As of today (*pre-1.0*), zbus doesn't have a high-level message dispatching
mechanism. Your code has to run a loop to continuously read incoming messages
(register the associated FD for input in poll/select to avoid blocking). We are
evaluating different options to make this easier, especially with *async*
support.

At the low-level, you can handle method calls by checking the incoming messages
manually.

Let's write a `SayHello` method, that takes a string as argument, and reply with
a "hello" greeting:

```rust,no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
#    let connection = zbus::Connection::new_session()?;
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
This is the crust of low-level message handling. It should give you all the
flexibility you ever need, but it is also easy to get it wrong. Fortunately,
zbus has a simpler solution to offer.

## Using the `ObjectServer`

One can write an `impl` with a set of methods and let the `dbus_interface`
procedural macro write the D-Bus details for us. It will export all the methods,
handle message dispatching, and introspection. It also has supports for
properties and some support for signals.

Let see how to use it:

```rust,no_run
# use std::error::Error;
# use zbus_derive::dbus_interface;
# use std::convert::TryInto;
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
    let mut object_server = zbus::ObjectServer::new(&connection);
    object_server.at(&"/org/zbus/MyGreeter".try_into()?, Greeter);
    loop {
        if let Err(err) = object_server.try_handle_next() {
            eprintln!("{}", err);
        }
    }

}
```

(check it works with the `busctl` call command)

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

[well-known name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
[`RequestName`]: https://dbus.freedesktop.org/doc/dbus-specification.html#bus-messages-request-name
