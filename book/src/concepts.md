# Some D-Bus concepts to help newcomers

## Bus

A D-Bus "bus" is a kind of server that handles several connections in a
bus-topology fashion. As such, it relays messages between connected endpoints,
and allows to discover endpoints or sending broadcast messages (signals).

Typically, a Linux system has a system bus, and a session bus. The latter is
per-user. It is also possible to have private buses or no bus at all (i-e direct
peer-to-peer communication instead).

## Bus name / service name

An endpoint can have various names, which allows to address messages to it on
the bus.

Usually, you care about the *well-known bus name*, which is typically the name
of a service on your system (for example, the [`org.freedesktop.Notifications`]
service).

## Object path

It is quite typical for a service to expose a single object path, such as
`/org/freedesktop/Notifications`, and you may not have to care much about this
concept after all.

But it is worth to understand that D-Bus organizes the interfaces by object
paths, which roughly should match instances of various objects in the process,
organized in a tree fashion.

## Interfaces

A D-Bus interface can have methods, properties and signals. The XML description
of an interface is mostly a machine-level detail. zbus macros will handle the
introspection of your interfaces for you. In the next chapters, we will see how
to translate an introspection XML to a Rust proxy.

## Good practices & API design

It is recommended to organise the service name, object paths and interface name
by using fully-qualified domain names, in order to avoid potential conflicts.

Please read the [D-Bus API Design Guidelines] carefully for other
similar considerations.

## 💡 Not yet async!

We are working on improving zbus in various aspects. One of them is making zbus
asynchronous, so read/write operations will not be potentially blocking your
thread. Note that usually D-Bus messages are fairly small, and shouldn't block.
Nevertheless, this is high in our priority list.

Onwards to implementation details & examples!

[`org.freedesktop.Notifications`]: https://people.gnome.org/~mccann/docs/notification-spec/notification-spec-latest.html
[D-Bus API Design Guidelines]: https://dbus.freedesktop.org/doc/dbus-api-design.html
