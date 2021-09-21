> **Note**
>
> This version of the book is based on zbus 2.0 API, which is currently in beta stages. For using the
> sample code in this book, you'll need to explicitly depend on the
> [latest beta](https://crates.io/crates/zbus/2.0.0-beta.6).
> 
> The 1.0 version of this book is available [here](https://dbus.pages.freedesktop.org/zbus/1.0/).

# Writing a server interface

In this chapter, we are going to implement a server with a method "SayHello", to greet back the
calling client.

We will first discuss the need to associate a service name with the server. Then we are going to
manually handle incoming messages using the low-level API. Finally, we will present the
`ObjectServer` higher-level API and some of its more advanced concepts.

## Taking a service name

As we know from the chapter on [D-Bus concepts], each connection on the bus is given a unique name
(such as ":1.27"). This could be all you need, depending on your use case, and the design of your
D-Bus API. However, typically services use a service name (aka *well-known name*) so peers (clients,
in this context) can easily discover them.

In this example, that is exactly what we're going to do and request the bus for the service name of
our choice:

```rust,no_run
use zbus::{Connection, Result};

#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session()
        .await?;
    connection
        .request_name("org.zbus.MyGreeter")
        .await?;

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
use futures_util::stream::TryStreamExt;

# #[async_std::main]
# async fn main() -> zbus::Result<()> {
#    let connection = zbus::Connection::session()
#        .await?;
#    connection
#        .request_name("org.zbus.MyGreeter")
#        .await?;
#
let mut stream = zbus::MessageStream::from(&connection);
while let Some(msg) = stream.try_next().await? {
    let msg_header = msg.header()?;
    dbg!(&msg);

    match msg_header.message_type()? {
        zbus::MessageType::MethodCall => {
            // real code would check msg_header path(), interface() and member()
            // handle invalid calls, introspection, errors etc
            let arg: &str = msg.body()?;
            connection.reply(&msg, &(format!("Hello {}!", arg))).await?;

            break;
        }
        _ => continue,
    }
}

# Ok(())
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

One can write an `impl` block with a set of methods and let the `dbus_interface` procedural macro
write the D-Bus message handling details. It will dispatch the incoming method calls to their
respective handlers, as well as replying to introspection requests. It also has support for
properties and signal emission.

Let see how to use it:

```rust,no_run
# use zbus::{SignalContext, ObjectServer, Connection, dbus_interface, fdo, Result};
#
use event_listener::Event;

struct Greeter {
    name: String,
    done: Event,
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }

    // Rude!
    async fn go_away(&self) {
        self.done.notify(1);
    }

    /// A "GreeterName" property.
    #[dbus_interface(property)]
    async fn greeter_name(&self) -> &str {
        &self.name
    }

    /// A setter for the "GreeterName" property.
    ///
    /// Additionally, a `greeter_name_changed` method has been generated for you if you need to
    /// notify listeners that "GreeterName" was updated. It will be automatically called when
    /// using this setter.
    #[dbus_interface(property)]
    async fn set_greeter_name(&mut self, name: String) {
        self.name = name;
    }

    /// A signal; the implementation is provided by the macro.
    #[dbus_interface(signal)]
    async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()>;
}

#[async_std::main]
async fn main() -> Result<()> {
    let connection = Connection::session()
        .await?;
    let greeter = Greeter {
        name: "GreeterName".to_string(),
        done: event_listener::Event::new(),
    };
    let done_listener = greeter.done.listen();
    connection
        .object_server_mut()
        .await
        .at("/org/zbus/MyGreeter", greeter)?;
    connection
        .request_name("org.zbus.MyGreeter")
        .await?;

    done_listener.wait();

    Ok(())
}
```

(check it works with the same `busctl` command as last time)

This time, we can also introspect the server:

```bash
$ busctl --user introspect org.zbus.MyGreeter /org/zbus/MyGreeter
NAME                                TYPE      SIGNATURE RESULT/VALUE FLAGS
org.freedesktop.DBus.Introspectable interface -         -             -
.Introspect                         method    -         s             -
org.freedesktop.DBus.Peer           interface -         -             -
.GetMachineId                       method    -         s             -
.Ping                               method    -         -             -
org.freedesktop.DBus.Properties     interface -         -             -
.Get                                method    ss        v             -
.GetAll                             method    s         a{sv}         -
.Set                                method    ssv       -             -
.PropertiesChanged                  signal    sa{sv}as  -             -
org.zbus.MyGreeter1                 interface -         -             -
.GoAway                             method    -         -             -
.SayHello                           method    s         s             -
.GreeterName                        property  s         "GreeterName" emits-change writable
.GreetedEveryone                    signal    -         -             -
```

Easy-peasy!

### Method errors

There are two possibilities for the return value of interface methods. The first is for infallible
method calls, where the return type is a directly serializable value, like the `String` in
`say_hello()` above.

The second is a result return value, where the `Ok` variant is the serializable value, and the
error is any error type that has an async `reply(&self, &zbus::Connection, &zbus::Message)`
method. The `zbus::fdo::Error` type implements this method, and should cover most common use cases.
However, when a custom error type needs to be emitted from the method as an error reply, it
can be created with `derive(zbus::DBusError)`, and used in the returned `Result<T, E>`.

### Sending signals

As you might have noticed in the previous example, the signal methods don't take a `&self` argument
but a `SignalContext` reference. This allows to emit signals whether from inside or outside of the
`dbus_interface` methods' context. To make things simpler, `dbus_interface` methods can receive a
`SignalContext` passed to them using the special `zbus(signal_context)` attribute:

Please refer to [`dbus_interface` documentation][didoc] for an example and list of other special
attributes you can make use of.

### Notifying property changes

For each property declared through the `dbus_interface` macro, a `<property_name>_changed` method is
generated that emits the necessary property change signal. Here is how to use it with the previous
example code:

```rust,no_run
# use zbus::dbus_interface;
# 
# struct Greeter {
#     name: String
# }
# 
# #[dbus_interface(name = "org.zbus.MyGreeter1")]
# impl Greeter {
#     #[dbus_interface(property)]
#     async fn greeter_name(&self) -> &str {
#         &self.name
#     }
# 
#     #[dbus_interface(property)]
#     async fn set_greeter_name(&mut self, name: String) {
#         self.name = name;
#     }
# }
#
# #[async_std::main]
# async fn main() -> zbus::Result<()> {
# let connection = zbus::Connection::session().await?;
# let mut object_server = connection.object_server_mut().await;
use zbus::InterfaceDerefMut;

object_server.with_mut(
    "/org/zbus/MyGreeter",
    |mut iface: InterfaceDerefMut<Greeter>, signal_ctxt| async move {
        iface.name = String::from("👋");
        iface.greeter_name_changed(&signal_ctxt).await
    },
).await?;
# Ok(())
# }
```

[D-Bus concepts]: concepts.html#bus-name--service-name
[didoc]: https://docs.rs/zbus/2.0.0-beta.6/zbus/attr.dbus_interface.html
