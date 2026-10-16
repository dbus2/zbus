# zvariant_derive

[![](https://docs.rs/zvariant_derive/badge.svg)](https://docs.rs/zvariant_derive/) [![](https://img.shields.io/crates/v/zvariant_derive)](https://crates.io/crates/zvariant_derive)

> **Deprecated:** zvariant and its derives are merged into [zbus] as of zbus 6.0, and
> zvariant_derive receives no further releases. Depend on
> `zbus = { version = "6", default-features = false }` instead, and take the derives from the
> `zbus` root or from `zbus::wire`, where `Value` and `OwnedValue` are named `Variant` and
> `OwnedVariant`. The [upgrade guide] covers the move.

This crate provides derive macros helpers for [`zvariant`]. The `zvariant` crate re-exports these
macros for your convenience so you do not need to use this crate directly.

**Status:** Deprecated.

## Example code

```rust
use zvariant::{serialized::Context, to_bytes, Type, LE};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct Struct<'s> {
    field1: u16,
    field2: i64,
    field3: &'s str,
}

assert_eq!(Struct::SIGNATURE, "(qxs)");
let s = Struct {
    field1: 42,
    field2: i64::max_value(),
    field3: "hello",
};
let ctxt = Context::new_dbus(LE, 0);
let encoded = to_bytes(ctxt, &s).unwrap();
let decoded: Struct = encoded.deserialize().unwrap().0;
assert_eq!(decoded, s);
```

[`zvariant`]: https://crates.io/crates/zvariant
[zbus]: https://crates.io/crates/zbus
[upgrade guide]: https://z-galaxy.github.io/zbus/upgrading-to-6.html
