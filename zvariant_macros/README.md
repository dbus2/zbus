# zvariant_macros

[![](https://docs.rs/zvariant_macros/badge.svg)](https://docs.rs/zvariant_macros/) [![](https://img.shields.io/crates/v/zvariant_macros)](https://crates.io/crates/zvariant_macros)

This subcrate of the [zbus project][zp] provides convenient procedural macros to construct
[`zvariant`] types, such as `Signature` from static strings to be checked at compile-time. The main
`zvariant` create re-exports these macros, so it is generally unnecessary to use this crate directly

**Status:** Stable.

[zp]: https://gitlab.freedesktop.org/dbus/zbus/-/blob/main/README.md
[zvariant]: https://crates.io/crates/zvariant
[zbus]: https://crates.io/crates/zbus
