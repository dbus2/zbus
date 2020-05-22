[![pipeline status](https://gitlab.freedesktop.org/zeenix/zbus/badges/master/pipeline.svg)](https://gitlab.freedesktop.org/zeenix/zbus/-/commits/master)

# zbus

A Rust API for [D-Bus](https://dbus.freedesktop.org/doc/dbus-specification.html) communication. The aim is to provide a safe and simple high- and low-level API akin to
[GDBus](https://developer.gnome.org/gio/stable/gdbus-convenience.html), that doesn't depend on C libraries.

The project is divided into three crates:

## zvariant

[![](https://docs.rs/zvariant/badge.svg)](https://docs.rs/zvariant/) [![](https://img.shields.io/crates/v/zvariant)](https://crates.io/crates/zvariant)

This crate provides API for encoding/decoding of data to/from D-Bus wire format. This crate is already in good shape
and can and should be used by other projects. This binary wire format is simple and very efficient and hence useful
outside of D-Bus context as well.

**Status:** Stable.

### Dependencies

* byteorder
* serde
* arrayvec (optional)
* enumflags2 (optional)

### Example code

```rust
use std::collections::HashMap;
use byteorder::LE;
use zvariant::{from_slice, to_bytes};
use zvariant::EncodingContext as Context;

// All serialization and deserialization API, needs a context.
let ctxt = Context::<LE>::new_dbus(0);

// i16
let encoded = to_bytes(ctxt, &42i16).unwrap();
let decoded: i16 = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, 42);

// strings
let encoded = to_bytes(ctxt, &"hello").unwrap();
let decoded: &str = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, "hello");

// tuples
let t = ("hello", 42i32, true);
let encoded = to_bytes(ctxt, &t).unwrap();
let decoded: (&str, i32, bool) = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, t);

// Vec
let v = vec!["hello", "world!"];
let encoded = to_bytes(ctxt, &v).unwrap();
let decoded: Vec<&str> = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, v);

// Dictionary
let mut map: HashMap<i64, &str> = HashMap::new();
map.insert(1, "123");
map.insert(2, "456");
let encoded = to_bytes(ctxt, &map).unwrap();
let decoded: HashMap<i64, &str> = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded[&1], "123");
assert_eq!(decoded[&2], "456");
```

## zvariant_derive

[![](https://docs.rs/zvariant_derive/badge.svg)](https://docs.rs/zvariant_derive/) [![](https://img.shields.io/crates/v/zvariant_derive)](https://crates.io/crates/zvariant_derive)

This crate provides a derive macro to easily implement [`Type` trait](https://docs.rs/zvariant/2.0.0/zvariant/trait.Type.html) on structs and enums.

**Status:** Stable.

### Dependencies

* proc-macro2
* syn
* quote

### Example code

```rust
use zvariant::{EncodingContext, from_slice, to_bytes};
use zvariant::Type;
use zvariant_derive::Type;
use serde::{Deserialize, Serialize};
use byteorder::LE;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct<'s> {
    field1: u16,
    field2: i64,
    field3: &'s str,
}

assert_eq!(Struct::signature(), "(qxs)");
let s = Struct {
    field1: 42,
    field2: i64::max_value(),
    field3: "hello",
};
let ctxt = EncodingContext::<LE>::new_dbus(0);
let encoded = to_bytes(ctxt, &s).unwrap();
let decoded: Struct = from_slice(&encoded, ctxt).unwrap();
assert_eq!(decoded, s);
```

## zbus

That's the main crate that you'll use to actually communicate with services and apps over D-Bus. At the moment you can
only connect to the session bus and call methods synchronously.

**Status:** Unstable. You've been warned!

### Dependencies

  * nix
  * byteorder
  * serde
  * serde_repr
  * enumflags2
  * serde-xml-rs (optional)

# License

MIT license [LICENSE-MIT](LICENSE-MIT)
