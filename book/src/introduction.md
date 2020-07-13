<p align="center">
  <img src="https://storage.googleapis.com/fdo-gitlab-uploads/project/avatar/3213/zbus-logomark.png"
       width="128" height="128"/>
</p>

# Introduction

**[zbus]** is a **[Rust]** crate for **[D-Bus]**. If you are not familiar with D-Bus, you should
read [what is D-Bus?] first[^outdated]. In short, zbus allows you to communicate from one program
to another, using the D-Bus protocol. In other words, it's an *inter-process* communication (IPC)
solution. It is a very popular IPC solution on Linux and many Linux services (e.g systemd,
NetworkManager) and desktop environments (e.g GNOME and KDE), rely on D-Bus for their IPC needs.
There are many tools and implementations available, making it easy to interact with D-Bus programs
from different languages and environments.

zbus is a 100% Rust-native implementation of the D-Bus protocol. It provides both an API to send
and receive messages over a connection, as well as API to interact with peers through high-level
concepts like method calls, signals and properties[^high-level-api]. Thanks to the power of Rust
macros, zbus is able to make interacting with D-Bus very easy.

zbus project provides two crates:

## zvariant

D-Bus defines a marshalling format for its messages. The [zvariant] crate provides a [serde]-based
[API] to serialize/deserialize Rust data types to/from this format.

## zbus

The [zbus crate] provides the main API you will use to interact with D-Bus from Rust. It takes care
of the establishment of a connection, the creation, sending and receiving of different kind of D-Bus
messages (method calls, signals etc) for you.

zbus crate is currently Linux-specific[^otheros].


[zbus]: https://gitlab.freedesktop.org/zeenix/zbus
[Rust]: https://www.rust-lang.org/
[D-Bus]: https://dbus.freedesktop.org/
[what is D-Bus?]: https://www.freedesktop.org/wiki/Software/dbus/#index1h1
[serde]: https://serde.rs/
[zvariant]: https://crates.io/crates/zvariant
[zbus crate]: https://crates.io/crates/zbus
[API]: https://docs.rs/zvariant/

[^outdated]: D-Bus is ~15y old, unfortunately many documents out there are
    sometime aging or misleading.

[^high-level-api]: These concepts are explained in the
[following chapter](concepts.html#interfaces).

[^otheros]: Support for other OS exist, but it is not supported to the same
    extent. D-Bus clients in javascript (running from any browser) do exist
    though. And zbus may also be working from the browser sometime in the future
    too, thanks to Rust 🦀 and WebAssembly 🕸.

<p align="center">
  <img src="https://www.freedesktop.org/png/freedesktop-logo.png" height="32"/>
</p>
