//! This crate provides derive macros helpers for zbus.
//!
//! At the moment, it supports basic [`Proxy`] implementation only.
//!
//! # Examples
//!
//! ```
//!# use std::error::Error;
//! use zbus_derive::dbus_proxy;
//! use zbus::{Connection, Result};
//! use zvariant::Value;
//!
//! #[dbus_proxy(
//!     interface = "org.test.SomeIface",
//!     default_service = "org.test.SomeService",
//!     default_path = "/org/test/SomeObject"
//! )]
//! trait SomeIface {
//!     fn do_this(&self, with: &str, some: u32, arg: &Value) -> Result<bool>;
//!     #[dbus_proxy(property)]
//!     fn a_property(&self) -> Result<String>;
//!     #[dbus_proxy(property)]
//!     fn set_a_property(&self, a_property: &str) -> Result<()>;
//! };
//!
//! let c = Connection::new_session()?;
//! let i = SomeIfaceProxy::new(&c)?;
//! let _ = i.do_this("foo", 32, &Value::new(true));
//! let _ = i.set_a_property("val");
//!
//!# Ok::<_, Box<dyn Error + Send + Sync>>(())
//! ```
//! [`Proxy`]: https://docs.rs/zbus/2.0.0/zbus/struct.Proxy.html
//!
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemTrait};

mod proxy;
mod utils;

#[proc_macro_attribute]
pub fn dbus_proxy(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemTrait);
    proxy::expand(args, input)
}
