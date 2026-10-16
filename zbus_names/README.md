# zbus_names

[![](https://docs.rs/zbus_names/badge.svg)](https://docs.rs/zbus_names/) [![](https://img.shields.io/crates/v/zbus_names)](https://crates.io/crates/zbus_names)

> **Deprecated:** zbus_names is merged into [`zbus`] as of zbus 6.0 and receives no further
> releases. Depend on `zbus = { version = "6", default-features = false }` instead, which provides
> the D-Bus name types without the D-Bus communication API, and use `zbus::names` in place of
> `zbus_names`. The [upgrade guide] covers the move.

This crate provides collection of types for various [D-Bus bus names][dbn].

This is used by [`zbus`] (and in future by [`zbus_macros`] as well) crate. Other D-Bus crates are also
encouraged to use this API in the spirit of cooperation. :)

For convenience, `zbus` re-exports this crate as `names`, so you do not need to depend directly on
this crate if you already depend on `zbus`.

**Status:** Deprecated.

[dbn]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names
[`zbus`]: https://crates.io/crates/zbus
[`zbus_macros`]: https://crates.io/crates/zbus_macros
[upgrade guide]: https://z-galaxy.github.io/zbus/upgrading-to-6.html
