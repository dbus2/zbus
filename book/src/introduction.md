<p align="center">
  <img src="https://storage.googleapis.com/fdo-gitlab-uploads/project/avatar/3213/zbus-logomark.png"
       width="128" height="128"/>
</p>

# Introduction

**[zbus]** is a **[Rust]** binding for **[D-Bus]**. If you are not familiar with
D-Bus, you should read [what is D-Bus?] first[^outdated]. In short, zbus allows
you to do *inter-process* communication (IPC) on Linux[^otheros] from a Rust
program using the D-Bus protocol. Many projects on Linux (including systemd or
the GNOME & KDE desktops), rely on D-Bus for inter-process communications.

<p align="center">
  <img src="https://www.freedesktop.org/png/freedesktop-logo.png" height="32"/>
</p>

There are many tools and implementations available, making it easy to interact
with D-Bus programs from different languages or environments.

zbus is a 100% Rust-native implementation of the D-Bus protocol. It relies on
**[serde]** to serialize/deserialize Rust data structures. zbus also provides
convenient macros to implement higher-level bindings of D-Bus interfaces or
services, as well as server-side implementation.

## zvariant

D-Bus defines a marshalling format for its messages. The zvariant crate
implements it for the [serde] types. It is the underlying byte stream format of
D-Bus messages, but zvariant takes care of it for you.

## zbus

The establishment of a connection, the creation and the handling of various kind
of D-Bus messages (method calls, signals etc) is done thanks to the zbus crate.
This is the main API you will use to interact with D-Bus from Rust.


[zbus]: https://gitlab.freedesktop.org/zeenix/zbus
[Rust]: https://www.rust-lang.org/
[D-Bus]: https://dbus.freedesktop.org/
[what is D-Bus?]: https://www.freedesktop.org/wiki/Software/dbus/#index1h1
[serde]: https://serde.rs/

[^outdated]: D-Bus is ~15y old, unfortunately many documents out there are
    sometime aging or misleading.

[^otheros]: Support for other OS exist, but it is not supported to the same
    extent. D-Bus clients in javascript (running from any browser) do exist
    though. And zbus may also be working from the browser sometime in the future
    too, thanks to Rust 🦀 and WebAssembly 🕸.
