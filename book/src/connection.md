# Establishing a connection

The first thing you will have to do is to connect to a D-Bus bus or to a D-Bus peer. This is the
entry point of the zbus API.

## Connection to the bus

To connect to the session bus (the *per-user* bus), simply call `Connection::session()`. It
returns an instance of the connection (if all went well). Similarly, to connect to the system bus
(to communicate with services such as [NetworkManager], [BlueZ] or [PID1]), use
`Connection::system()`.

Moreover, it also implements [`futures::sink::Sink`] and can be converted to a [`MessageStream`]
that implements [`futures::stream::Stream`], which can be used to conveniently send and receive
messages, for the times when low-level API is more appropriate for your use case.

**Note:** it is common for a D-Bus library to provide a "shared" connection to a bus for a process:
all `session()` share the same underlying connection for example. At the time of this writing,
zbus doesn't do that.

**Note:** on Windows, there is no standard implicit way to connect to a session bus. zbus provides
opt-in compatibility to the GDBus session bus discovery mechanism via the `windows-gdbus` feature.
This mechanism uses a machine-wide mutex however, so only one GDBus session bus can run at a time.

## Using a custom bus address

You may also specify a custom bus with `Connection::new_for_address()` which takes a D-Bus address
[as specified in the
specification](https://dbus.freedesktop.org/doc/dbus-specification.html#addresses).

## Peer to peer connection

Peer-to-peer connections are bus-less[^bus-less], and the initial handshake protocol is a bit
different. There is the notion of client & server endpoints, but that distinction doesn't matter
once the connection is established (both ends are equal, and can send any messages).

To create a bus-less peer-to-peer connection on Unix, you can make a `socketpair()` (or have a
listening socket server, accepting multiple connections), and hand over the socket FDs to
`Connection::new_unix_server` and `Connection::new_unix_client` for each side. After success, you
can call the `Connection` methods to send and receive messages on both ends.

See the `unix_p2p` test in the [zbus source code] for a simple example.

[NetworkManager]: https://developer.gnome.org/NetworkManager/stable/spec.html
[BlueZ]: https://git.kernel.org/pub/scm/bluetooth/bluez.git/tree/doc
[PID1]: https://www.freedesktop.org/wiki/Software/systemd/dbus/
[zbus source code]: https://gitlab.freedesktop.org/dbus/zbus/-/blob/main/zbus/src/connection.rs
[`futures::stream::Stream`]: https://docs.rs/futures/latest/futures/stream/trait.Stream.html
[`futures::sink::Sink`]: https://docs.rs/futures/latest/futures/sink/trait.Sink.html
[`MessageStream`]: https://docs.rs/zbus/2.0.0/zbus/struct.MessageStream.html

[^bus-less] Unless you implemented them, none of the bus methods will exist.
