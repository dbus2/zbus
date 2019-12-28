//! This crate provides API for encoding/decoding of data to/from [D-Bus] wire format. This binary wire format is simple
//! and very efficient and hence useful outside of [D-Bus] context as well. A slightly modified form of this format,
//! [GVariant] is also very common and will be supported by a future version of this crate.
//!
//! The core traits are [`Encode`] and [`Decode`], for (surprise surprise) encoding and decoding of data. All data types
//! that can be encoded to wire-format, implement these traits. As decoding require allocation, one exception here is
//! `&str`. It only implements [`Encode`], while its owned sibling, `String` implements both traits.
//!
//! All data types have a signature, which is a string denoting the type in question. These data types are divided into
//! two groups, basic and container. Most of the basic types match 1-1 with all the primitive Rust types. The only two
//! exceptions being, [`Signature`] and [`ObjectPath`], which are really just strings. The marker trait, [`Basic`], is
//! implemented by all the basic types, [except `f64`]. There is also [`SimpleDecode`] trait, which is for types (mostly
//! basic) whose signature is always the same.
//!
//! For container types, we provide custom data types:
//!
//! * [`Array`]: An unordered collection of items of the same type. API is provided to transform this into, and from a
//! [`Vec`].
//! * [`Structure`]: An ordered collection of items of different types.
//! * [`DictEntry`]: A key-value pair. The key must be a basic type.
//! * [`Dict`]: An Array of DictEntry. API is provided to transform this into, and from a [`HashMap`].
//! * [`Variant`]: A generic container, in the form of an enum that holds exactly one value of any of the other types.
//!
//! [D-Bus]: https://dbus.freedesktop.org/doc/dbus-specification.html
//! [GVariant]: https://developer.gnome.org/glib/stable/glib-GVariant.html
//! [`Decode`]: trait.Decode.html
//! [`Encode`]: trait.Encode.html
//! [`Signature`]: struct.Signature.html
//! [`ObjectPath`]: struct.ObjectPath.html
//! [`Basic`]: trait.Basic.html
//! [except `f64`]: trait.Basic.html
//! [`SimpleDecode`]: trait.SimpleDecode.html
//! [`Array`]: struct.Array.html
//! [`Structure`]: struct.Structure.html
//! [`DictEntry`]: struct.DictEntry.html
//! [`Dict`]: struct.Dict.html
//! [`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//! [`Variant`]: struct.Variant.html

mod basic;
pub use basic::*;

mod variant;
pub use variant::*;

mod decode;
pub use decode::*;

mod encode;
pub use encode::*;

mod variant_error;
pub use variant_error::*;

mod str;
pub use crate::str::*;

mod signature;
pub use crate::signature::*;

mod object_path;
pub use crate::object_path::*;

mod simple_decode;
pub use simple_decode::*;

mod structure;
pub use structure::*;

mod array;
pub use array::*;

mod dict_entry;
pub use dict_entry::*;

mod dict;
pub use dict::*;

mod shared_data;
pub use shared_data::*;

mod utils;
