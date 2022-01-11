# zbus_xmlgen

[![](https://img.shields.io/crates/v/zbus_xmlgen)](https://crates.io/crates/zbus_xmlgen)

A binary crate that provides a developer tool to generate [zbus]-based Rust code from D-Bus XML
interface descriptions. It can be used to generate the code directly from a running D-Bus system,
session or other service, or using a preexisting XML file for input.

**Status:** Stable.

## Usage

```shell
$ cargo install zbus_xmlgen
$ zbus-xmlgen --system org.freedesktop.login1 /org/freedesktop/login1
$ zbus-xmlgen --session org.freedesktop.ScreenSaver /org/freedesktop/ScreenSaver
$ zbus-xmlgen --address unix:abstract=/home/user/.cache/ibus/dbus-fpxKwgbJ org.freedesktop.IBus /org/freedesktop/IBus
$ zbus-xmlgen interface.xml
```

[zbus]: https://crates.io/crates/zbus
