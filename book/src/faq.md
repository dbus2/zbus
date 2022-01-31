# FAQ

## How to use a struct as a dictionary?

Since the use of a dictionary, specifically one with strings as keys and variants as value (i-e
`a{sv}`) is very common in the D-Bus world and use of HashMaps isn't as convenient and type-safe as
a struct, you might find yourself wanting to use a struct as a dictionary.

`zvariant` provides convenient macros for making this possible: [`SerializeDict`] and
[`DeserializeDict`]. You'll also need to tell [`Type`] macro to treat the type as a dictionary using
the `signature` attribute. Here is a simple example:

```rust
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
feature and disabling `async-io` feature:

```toml
# Sample Cargo.toml snippet.
[dependencies]
zbus = { version = "2", default-features = false, features = ["tokio"] }
```

[`Type`]: https://docs.rs/zvariant/3.1.0/zvariant/derive.Type.html
[`SerializeDict`]: https://docs.rs/zvariant/3.0.0/zvariant/derive.SerializeDict.html
[`DeserializeDict`]: https://docs.rs/zvariant/3.0.0/zvariant/derive.DeserializeDict.html
