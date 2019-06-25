# zbus

A Rust crate for D-Bus communication. The aim is to provide a safe and simple high- and low-level API akin to
[GDBus](https://developer.gnome.org/gio/stable/gdbus-convenience.html), that doesn't depend on C libraries.

For now we just have:

  * Connection: Representing a D-Bus connection. You can only connect to the session bus and only on the usual UNIX
                socket path (on Fedora at least).
  * Message: Represents a D-Bus message. You can send a method and parse the reply but you'll need to write a lot of
             code right now. Connection already has code to send/receive message so mostly just need to split that into
             generic methods.
  * Variant: Handles parsing of D-Bus data types. Currently it can only handle u32 and String.

The code is very inefficient at the moment in terms of memory usage and we only have 1 testcase (although it tests
connecting to the bus, which involves the handshake and Hello method call) and 0 documentation. So consider it just a
humble beginning and a lot has to be done for this to be actually useful to anyone.

# Status

Experimental. You've been warned!

# License

MIT license ([LICENSE-MIT](LICENSE-MIT)

# Dependencies

  * nix
  * byteorder

Don't be impressed. I'm sure this list will grow soon. :)
