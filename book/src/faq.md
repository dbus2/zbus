# FAQ

## How to use a struct as a dictionary?

Since the use of a dictionary, specifically one with strings as keys and variants as value (i-e
`a{sv}`) is very common in the D-Bus world and use of HashMaps isn't as convenient and type-safe as
a struct, you might find yourself wanting to use a struct as a dictionary.

`zvariant` provides convenient macros for making this possible: [`SerializeDict`] and
[`DeserializeDict`]. You'll also need to tell [`Type`] macro to treat the type as a dictionary using
the `signature` attribute. Here is a simple example:

```rust,noplayground
use zbus::{
    dbus_proxy, dbus_interface, fdo::Result,
    zvariant::{DeserializeDict, SerializeDict, Type},
};

#[derive(DeserializeDict, SerializeDict, Type)]
// `Type` treats `dict` is an alias for `a{sv}`.
#[zvariant(signature = "dict")]
pub struct Dictionary {
    field1: u16,
    #[zvariant(rename = "another-name")]
    field2: i64,
    optional_field: Option<String>,
}

#[dbus_proxy(
    interface = "org.zbus.DictionaryGiver",
    default_path = "/org/zbus/DictionaryGiver",
    default_service = "org.zbus.DictionaryGiver",
)]
trait DictionaryGiver {
    fn give_me(&self) -> Result<Dictionary>;
}

struct DictionaryGiverInterface;

#[dbus_interface(interface = "org.zbus.DictionaryGiver")]
impl DictionaryGiverInterface {
    fn give_me(&self) -> Result<Dictionary> {
        Ok(Dictionary {
            field1: 1,
            field2: 4,
            optional_field: Some(String::from("blah")),
        })
    }
}
```

## Why do async tokio API calls from interface methods not work?

Many of the tokio (and tokio-based) APIs assume the tokio runtime to be driving the async machinery
and since by default, zbus runs the `ObjectServer` in its own internal runtime thread, it's not
possible to use these APIs from interface methods. Moreover, by default zbus relies on `async-io`
crate to communicate with the bus, which uses its own thread.

Not to worry, though! You can enable tight integration between tokio and zbus by enabling `tokio`
feature:

```toml
# Sample Cargo.toml snippet.
[dependencies]
# Also disable the default `async-io` feature to avoid unused dependencies.
zbus = { version = "3", default-features = false, features = ["tokio"] }
```

**Note**: On Windows, the `async-io` feature is currently required for UNIX domain socket support,
see [the corresponding tokio issue on GitHub][tctiog].

## I'm experiencing hangs, what could be wrong?

There are typically two reasons this can happen with zbus:

### 1. A `dbus_interface` method that takes a `&mut self` argument is taking too long

Simply put, this is because of one of the primary rules of Rust: while a mutable reference to a
resource exists, no other references to that same resource can exist at the same time. This means
that before the method in question returns, all other method calls on the providing interface will
have to wait in line.

A typical solution here is use of interior mutability or launching tasks to do the actual work
combined with signals to report the progress of the work to clients. Both of these solutions
involve converting the methods in question to take `&self` argument instead of `&mut self`.

### 2. A stream (e.g `SignalStream`) is not being continuously polled

Please consult [`MessageStream`] documentation for details.

## Why aren't property values updating for my service that doesn't notify changes?

A common issue might arise when using a zbus proxy is that your proxy's property values aren't 
updating. This is due to zbus' default caching policy, which updates the value of a property only
when a change is signaled, primarily to minimize latency and optimize client request performance.
By default, if your service does not emit change notifications, the property values will not
update accordingly. 

However, you can disabling caching for specific properties:

- Add the `#[dbus_proxy(property(emits_changed_signal = "false"))]` annotation to the property
for which you desire to disable caching on.

- Use `proxy::Builder` to build your proxy instance and use `proxy::Builder::uncached_properties` method
to list all properties you wish to disable caching for.

- In order to disable caching for either type of proxy use the `proxy::Builder::cache_properites` 
method.

For more information about all the possible values for `emits_changed_signal` refer
 to [`dbus_proxy`](https://docs.rs/zbus/latest/zbus/attr.dbus_proxy.html) documentation.

[tctiog]: https://github.com/tokio-rs/tokio/issues/2201
[`Type`]: https://docs.rs/zvariant/latest/zvariant/derive.Type.html
[`SerializeDict`]: https://docs.rs/zvariant/latest/zvariant/derive.SerializeDict.html
[`DeserializeDict`]: https://docs.rs/zvariant/latest/zvariant/derive.DeserializeDict.html
[`MessageStream`]: https://docs.rs/zbus/latest/zbus/struct.MessageStream.html
