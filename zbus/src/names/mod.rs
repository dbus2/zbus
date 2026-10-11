//! Types for the various [D-Bus bus names][dbn].
//!
//! D-Bus has several kinds of names — unique connection names, well-known service names,
//! interface names, error names, member names and property names — and each kind has its own
//! syntax rules. This module provides a type per kind: a name is validated once, when it is
//! created, and is then known to be valid everywhere it travels.
//!
//! Each type has an owned counterpart ([`OwnedBusName`], [`OwnedInterfaceName`] and so on) for
//! when the borrowed form is inconvenient, and all of them implement [`Type`] and convert to
//! and from [`Variant`], so they can be sent over the bus directly.
//!
//! Validation failures are reported as [`crate::Error::InvalidName`], and conversions between
//! two name kinds that cannot succeed as [`crate::Error::InvalidNameConversion`].
//!
//! # Example
//!
//! ```
//! use zbus::names::InterfaceName;
//!
//! let name = InterfaceName::try_from("org.freedesktop.DBus").unwrap();
//! assert_eq!(name, "org.freedesktop.DBus");
//!
//! // An interface name needs at least two elements separated by a dot.
//! InterfaceName::try_from("no-dots").unwrap_err();
//! ```
//!
//! [dbn]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names
//! [`Type`]: trait@crate::Type
//! [`Variant`]: enum@crate::Variant

mod bus_name;
pub use bus_name::*;

mod unique_name;
pub use unique_name::*;

mod well_known_name;
pub use well_known_name::*;

mod interface_name;
pub use interface_name::*;

mod member_name;
pub use member_name::*;

mod property_name;
pub use property_name::*;

mod error_name;
pub use error_name::*;

/// Deprecated alias of [`crate::Error`].
#[deprecated(
    since = "6.0.0",
    note = "zbus_names was merged into zbus; use `zbus::Error`"
)]
pub type Error = crate::Error;

/// Deprecated alias of [`crate::Result`].
#[deprecated(
    since = "6.0.0",
    note = "zbus_names was merged into zbus; use `zbus::Result`"
)]
pub type Result<T> = crate::Result<T>;

mod utils;
