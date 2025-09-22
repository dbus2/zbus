# FAQ

<!-- toc -->

## How to use a struct as a dictionary?

Since the use of a dictionary, specifically one with strings as keys and variants as value (i-e
`a{sv}`) is very common in the D-Bus world and use of HashMaps isn't as convenient and type-safe as
a struct, you might find yourself wanting to use a struct as a dictionary.

It's possible to do so, using either of the following methods:

1. Using the `SerializeDict` and `DeserializeDict` derive macros from `zvariant`. This is the best
  option for simple cases.
2. Using the `Serialize` and `Deserialize` derive macros from the `serde` crate. This is the best
  option for more complex cases, where you need more fine-grained control over the serialization
  and/or deserialization process.

Here is a simple example using `SerializeDict` and `DeserializeDict`:

```rust,noplayground
use zbus::{
    proxy, interface, fdo::Result,
    zvariant::{Type, SerializeDict, DeserializeDict},
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

#[proxy(
    interface = "org.zbus.DictionaryGiver",
    default_path = "/org/zbus/DictionaryGiver",
    default_service = "org.zbus.DictionaryGiver",
)]
trait DictionaryGiver {
    fn give_me(&self) -> Result<Dictionary>;
}

struct DictionaryGiverInterface;

#[interface(interface = "org.zbus.DictionaryGiver")]
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

Now let's say you want to grab all the extra entries that are not explicitly defined in the
struct. You can not do that with `DeserializeDict` but you can with `serde::Deserialize`:

```rust,noplayground
use std::collections::HashMap;
use zbus::zvariant::{Type, OwnedValue, as_value::{self, optional}};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Type)]
#[zvariant(signature = "dict")]
pub struct Dictionary {
    #[serde(with = "as_value")]
    field1: u16,
    #[serde(rename = "another-name", with = "as_value")]
    field2: i64,
    #[serde(
        with = "optional",
        skip_serializing_if = "Option::is_none",
        default,
    )]
    optional_field: Option<String>,
    #[serde(flatten)]
    the_rest: HashMap<String, OwnedValue>,
}
```

Since the fields have to be transformed from/into `zvariant::Value`, make sure to use the `with`
attribute with the appropriate helper module from `zvariant::as_value` module.

Moreover, since D-Bus does not have a concept of nullable types, it's important to ensure that
`skip_serializing_if` and `default` attributes are used for optional fields. Fortunately, you can
make use of the `default` container attribute if your struct can implemented the `Default` trait:

```rust,noplayground
# use zbus::zvariant::{Type, as_value::{self, optional}};
# use serde::{Deserialize, Serialize};
#
#[derive(Default, Deserialize, Serialize, Type)]
#[zvariant(signature = "dict")]
#[serde(default)]
pub struct Dictionary {
    #[serde(with = "as_value")]
    field1: u16,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    optional_field1: Option<i64>,
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    optional_field2: Option<String>,
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
zbus = { version = "5", default-features = false, features = ["tokio"] }
```

**Note**: On Windows, the `async-io` feature is currently required for UNIX domain socket support,
see [the corresponding tokio issue on GitHub][tctiog].

## I'm experiencing hangs, what could be wrong?

There are typically two reasons this can happen with zbus:

### 1. A `interface` method that takes a `&mut self` argument is taking too long

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

- Add the `#[zbus(property(emits_changed_signal = "false"))]` annotation to the property for which
  you desire to disable caching on. For more information about all the possible values for
  `emits_changed_signal` refer to [`proxy`] documentation.

- Use `proxy::Builder` to build your proxy instance and use [`proxy::Builder::uncached_properties`]
  method to list all properties you wish to disable caching for.

- In order to disable caching for either type of proxy use the [`proxy::Builder::cache_properites`]
  method.

## How do I use `Option<T>` with zbus?

While `Option<T>` is a very commonly used type in Rust, there is unfortunately [no concept of a
nullable-type in the D-Bus protocol][nonull]. However, there are two ways to simulate it:

### 1. Designation of a special value as `None`

This is the simplest way to simulate `Option<T>`. Typically the
default value for the type in question is a good choice. For example the empty string (`""`) is
often used as `None` for strings and string-based types. Note however that this solution can not
work for all types, for example `bool`.

Since this is the most widely used solution in the D-Bus world and is even used by the [D-Bus
standard interfaces][dsi], `zvariant` provides a custom type for this, [`Optional<T>`] that makes
it super easy to simulate a nullable type, especially if the contained type implements the `Default`
trait.

### 2. Encoding as an array (`a?`)

The idea here is to represent `None` case with 0 elements (empty array) and the `Some` case with 1
element. `zvariant` and `zbus` provide `option-as-array` Cargo feature, which when enabled, allows
the (de)serialization of `Option<T>`. Unlike the previous solution, this solution can be used with
all types. However, it does come with some caveats and limitations:

  1. Since the D-Bus type signature does not provide any hints about the array being in fact a
    nullable type, this can be confusing for users of generic tools like [`d-feet`]. It is therefore
    highly recommended that service authors document each use of `Option<T>` in their D-Bus
    interface documentation.
  2. Currently it is not possible to use `Option<T>` for `interface` and `proxy` property
    methods.
  3. Both the sender and receiver must agree on use of this encoding. If the sender sends `T`, the
    receiver will not be able to decode it successfully as `Option<T>` and vice versa.
  4. While `zvariant::Value` can be converted into `Option<T>`, the reverse is currently not
    possible.

Due to these limitations, `option-as-array` feature is not enabled by default and must be explicitly
enabled.

**Note**: We hope to be able to remove #2 and #4, once [specialization] lands in stable Rust.

## How do enums work?

By default, `zvariant` encodes an unit-type enum as a `u32`, denoting the variant index. Other enums
are encoded as a structure whose first field is the variant index and the second one are the
variant's field(s). The only caveat here is that all variants must have the same number and types
of fields. Names of fields don't matter though. You can make use of [`Value`] or [`OwnedValue`] if you want to encode different data in different fields. Here is a simple example:

```rust,noplayground
use zbus::zvariant::{serialized::Context, to_bytes, Type, LE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
enum Enum<'s> {
    Variant1 { field1: u16, field2: i64, field3: &'s str },
    Variant2(u16, i64, &'s str),
    Variant3 { f1: u16, f2: i64, f3: &'s str },
}

let e = Enum::Variant3 {
    f1: 42,
    f2: i64::max_value(),
    f3: "hello",
};
let ctxt = Context::new_dbus(LE, 0);
let encoded = to_bytes(ctxt, &e).unwrap();
let decoded: Enum = encoded.deserialize().unwrap().0;
assert_eq!(decoded, e);
```

Enum encoding can be adjusted by using the [`serde_repr`] crate and by annotating the representation of the enum with `repr`:

```rust,noplayground
use zbus::zvariant::{serialized::Context, to_bytes, Type, LE};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Deserialize_repr, Serialize_repr, Type, PartialEq, Debug)]
#[repr(u8)]
enum UnitEnum {
    Variant1,
    Variant2,
    Variant3,
}

let ctxt = Context::new_dbus(LE, 0);
let encoded = to_bytes(ctxt, &UnitEnum::Variant2).unwrap();
let e: UnitEnum = encoded.deserialize().unwrap().0;
assert_eq!(e, UnitEnum::Variant2);
```

Unit enums can also be (de)serialized as strings:

```rust,noplayground
use zbus::zvariant::{serialized::Context, to_bytes, Type, LE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
#[zvariant(signature = "s")]
enum StrEnum {
    Variant1,
    Variant2,
    Variant3,
}

let ctxt = Context::new_dbus(LE, 0);
let encoded = to_bytes(ctxt, &StrEnum::Variant2).unwrap();
let e: StrEnum = encoded.deserialize().unwrap().0;
assert_eq!(e, StrEnum::Variant2);
let s: &str = encoded.deserialize().unwrap().0;
assert_eq!(s, "Variant2");
```

[`proxy::Builder::uncached_properties`]: https://docs.rs/zbus/5/zbus/proxy/struct.Builder.html#method.uncached_properties
[`proxy::Builder::cache_properites`]: https://docs.rs/zbus/5/zbus/proxy/struct.Builder.html#method.cache_properties
[`proxy`]: https://docs.rs/zbus/5/zbus/attr.proxy.html
[tctiog]: https://github.com/tokio-rs/tokio/issues/2201
[`Type`]: https://docs.rs/zvariant/5/zvariant/derive.Type.html
[`MessageStream`]: https://docs.rs/zbus/5/zbus/struct.MessageStream.html
[nonull]: https://gitlab.freedesktop.org/dbus/dbus/-/issues/25
[dsi]: http://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces
[`Optional<T>`]: https://docs.rs/zvariant/5/zvariant/struct.Optional.html
[`d-feet`]: https://wiki.gnome.org/Apps/DFeet
[specialization]: https://rust-lang.github.io/rfcs/1210-impl-specialization.html
[`Value`]: https://docs.rs/zvariant/5/zvariant/enum.Value.html
[`OwnedValue`]: https://docs.rs/zvariant/5/zvariant/struct.OwnedValue.html
[`serde_repr`]: https://crates.io/crates/serde_repr
