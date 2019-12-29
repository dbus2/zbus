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
//! # Examples
//!
//! Here are some simple encoding and decoding examples:
//!
//! ```
//! use zvariant::{Encode, EncodingFormat, SimpleDecode};
//!
//! // Encode a string
//! let format = EncodingFormat::default();
//! let encoding = "Hello world!".encode(format);
//! assert!(encoding.len() == 17);
//!
//! // and the decode it from the encoded form
//! let s = String::decode_simple(encoding, format).unwrap();
//! assert!(s == "Hello world!");
//! ```
//!
//! ```
//! use zvariant::{Decode, Encode, EncodingFormat, Variant};
//!
//! // Create a Variant from an i16
//! let v = i16::max_value().to_variant();
//! assert!(*i16::from_variant(&v).unwrap() == i16::max_value());
//! assert!(i16::is(&v));
//!
//! // Encode it
//! let format = EncodingFormat::default();
//! let encoding = v.encode_value(format);
//! assert!(encoding.len() == 2);
//!
//! // Decode it back
//! let v = Variant::from_data(encoding, v.value_signature(), format).unwrap();
//! assert!(i16::take_from_variant(v).unwrap() == i16::max_value());
//! ```
//!
//! And a complex example:
//!
//! ```
//! use core::convert::TryFrom;
//! use std::collections::HashMap;
//! use zvariant::{Array, Decode, Dict, Encode, EncodingFormat, Structure};
//!
//! // We chould directly create an Array of DictEntry too
//! let mut map: HashMap<i64, &str> = HashMap::new();
//! map.insert(1, "123");
//! map.insert(2, "456");
//! let dict: Dict = map.into();
//! let array = Array::try_from(dict).unwrap();
//!
//! // Create our not-so-simple structure
//! let s = Structure::new()
//!     .add_field(u8::max_value())
//!     .add_field(u32::max_value())
//!     .add_field(
//!         Structure::new()
//!         .add_field(i64::max_value())
//!         .add_field(true)
//!         .add_field(
//!             Structure::new()
//!             .add_field(i64::max_value())
//!             .add_field(std::f64::MAX),
//!         ),
//!     )
//!     .add_field("hello")
//!     .add_field(array);
//!
//! // Encode the structure
//! let format = EncodingFormat::default();
//! let encoding = s.encode(format);
//! // The HashMap is unordered so we can't rely on items to be in a specific order during the
//! // transformation to Vec, and size depends on the order of items because of padding rules.
//! assert!(encoding.len() == 88 || encoding.len() == 92);
//!
//! // Then we decode the structure from the encoded value
//! let s = Structure::decode(encoding, s.signature(), format).unwrap();
//! assert!(s.signature() == "(yu(xb(xd))sa{xs})");
//!
//! // Check all the fields are as expected
//! let fields = s.fields();
//! assert!(u8::is(&fields[0]));
//! assert!(*u8::from_variant(&fields[0]).unwrap() == u8::max_value());
//! assert!(u32::is(&fields[1]));
//! assert!(*u32::from_variant(&fields[1]).unwrap() == u32::max_value());
//!
//! assert!(Structure::is(&fields[2]));
//! let inner = Structure::from_variant(&fields[2]).unwrap();
//! let inner_fields = inner.fields();
//! assert!(i64::is(&inner_fields[0]));
//! assert!(*i64::from_variant(&inner_fields[0]).unwrap() == i64::max_value());
//! assert!(bool::is(&inner_fields[1]));
//! assert!(*bool::from_variant(&inner_fields[1]).unwrap());
//!
//! assert!(String::from_variant(&fields[3]).unwrap() == "hello");
//! assert!(String::is(&fields[3]));
//! ```
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
