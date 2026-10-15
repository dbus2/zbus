//! The context of an encoding and the containers of serialized data.
//!
//! [`Context`] says how to encode or decode: the byte order and the position of the data in the
//! message. [`Data`] holds serialized bytes, along with their file descriptors on Unix, and the
//! context they are encoded in. [`Size`] and [`Written`] are what [`serialized_size`] and
//! [`to_writer`] return.
//!
//! [`serialized_size`]: crate::wire::serialized_size
//! [`to_writer`]: crate::wire::to_writer

mod data;
pub use data::Data;
mod size;
pub use size::Size;
mod written;
pub use written::Written;
mod context;
pub use context::Context;
