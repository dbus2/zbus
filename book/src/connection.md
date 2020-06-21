# Making a connection

The first thing you will have to do is to connect to a D-Bus bus or to a D-Bus
peer. This is the entry point of the zbus API.

## Connection to the bus

To connect to the session bus (the *per-user* bus), simply call
`Connection::new_session()`. It returns an instance of the connection (if all
went well).

Similarly, to connect to the system bus (to communicate with services such as
[NetworkManager], [BlueZ] or [PID1]), you should use `Connection::new_system()`.

**Note:** it is common for a D-Bus library to provide a "shared" connection to a
bus for a process: all `new_session()` share the same underlying connection for
example. At this point, zbus doesn't attempt to do that.

## Using a custom bus address

You may also specify a custom D-Bus address with `Connection::new_for_address()`
which takes a D-Bus address [as specified in the
specification](https://dbus.freedesktop.org/doc/dbus-specification.html#addresses).

## Peer to peer connection

**TODO**

[NetworkManager]: https://developer.gnome.org/NetworkManager/stable/spec.html
[BlueZ]: https://git.kernel.org/pub/scm/bluetooth/bluez.git/tree/doc
[PID1]: https://www.freedesktop.org/wiki/Software/systemd/dbus/
