# Some D-Bus concepts to help newcomers

<!-- toc -->

## Bus

A D-Bus "bus" is a kind of server that handles several connections in a bus-topology fashion. As
such, it relays messages between connected endpoints, and allows to discover endpoints or sending
broadcast messages (signals).

Typically, a Linux system has a system bus, and a session bus. The latter is per-user. It is also
possible to have private buses or no bus at all (i-e direct peer-to-peer communication instead).

## Bus name / service name

An endpoint can have various names, which allows to address messages to it on the bus. All endpoints
are assigned a unique name by the bus at start. Since this name is not static, most services use
something called a *well-known bus name* and typically it's this name, that you'll be concerned
with.

An example would be the [FreeDesktop Notifications Service] that uses
`org.freedesktop.Notifications` as its well-known bus name.

For further details on bus names, please refer to the [Bus names chapter] of the D-Bus specification.

## Objects and Object paths

An object is akin to the concept of an object or an instance in many programming languages. All
services expose at least one object on the bus and all clients interact with the service through
these objects. These objects can be ephemeral or they could live as long as the service itself.

Every object is identified by a string, which is referred to as its path. An example of an object
path is `/org/freedesktop/Notifications`, which identities the only object exposed by the
[FreeDesktop Notifications Service].

For further details on object paths, please refer to the [Basic types chapter] of the D-Bus
specification.

## Interfaces

An interface defines the API exposed by object on the bus. They are akin to the concept of
interfaces in many programming languages and traits in Rust. Each object can (and typically do)
provide multiple interfaces at the same time. A D-Bus interface can have methods, properties and
signals.

While each interface of a service is identified by a [unique name], its API is described by an XML
description. It is mostly a machine-level detail. Most services can be queried for this description
through a D-Bus standard [introspection interface].

zbus provides convenient macro that implements the introspection interface for services, and helper
to generate client-side Rust API, given an XML description. We'll see both of these in action in the
following chapters.

## Good practices & API design

It is recommended to organise the service name, object paths and interface name by using
fully-qualified domain names, in order to avoid potential conflicts.

Please read the [D-Bus API Design Guidelines] carefully for other similar considerations.

Onwards to implementation details & examples!

[FreeDesktop Notifications Service]: https://specifications.freedesktop.org/notification-spec/notification-spec-latest.html
[D-Bus API Design Guidelines]: https://dbus.freedesktop.org/doc/dbus-api-design.html
[Bus names chapter]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
[Basic types chapter]: https://dbus.freedesktop.org/doc/dbus-specification.html#basic-types
[unique name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-interface
[introspection interface]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces-introspectable
