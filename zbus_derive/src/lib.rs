//! This crate provides derive macros helpers for zbus.
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemTrait};

mod error;
mod proxy;
mod utils;

/// Attribute macro for defining a D-Bus proxy (using zbus [`Proxy`]).
///
/// The macro must be applied on a `trait T`. A matching `impl T` will provide the proxy. The proxy
/// instance can be created with the associated `new()` or `new_for()` methods. The former doesn't take
/// any argument and uses the default service name and path. The later allows you to specify both.
///
/// Each trait method will be expanded to call to the associated D-Bus remote interface.
///
/// Trait methods accept `dbus_proxy` attributes:
///
/// * `name` - override the D-Bus name (pascal case form by default)
///
/// * `property` - expose the method as a property. If the method takes an argument, it must be a
///   setter, with a `set_` prefix. Otherwise, it's a getter.
///
/// * `signal` - not yet implemented.
///
/// (the expanded `impl` also provides an `introspect()` method, for convenience)
///
/// # Example
///
/// ```
///# use std::error::Error;
/// use zbus_derive::dbus_proxy;
/// use zbus::{Connection, Result};
/// use zvariant::Value;
///
/// #[dbus_proxy(
///     interface = "org.test.SomeIface",
///     default_service = "org.test.SomeService",
///     default_path = "/org/test/SomeObject"
/// )]
/// trait SomeIface {
///     fn do_this(&self, with: &str, some: u32, arg: &Value) -> Result<bool>;
///
///     #[dbus_proxy(property)]
///     fn a_property(&self) -> Result<String>;
///
///     #[dbus_proxy(property)]
///     fn set_a_property(&self, a_property: &str) -> Result<()>;
/// };
///
/// let connection = Connection::new_session()?;
/// let proxy = SomeIfaceProxy::new(&connection)?;
/// let _ = proxy.do_this("foo", 32, &Value::new(true));
/// let _ = proxy.set_a_property("val");
///
///# Ok::<_, Box<dyn Error + Send + Sync>>(())
/// ```
///
/// [`zbus_polkit`] is a good example of how to bind a real D-Bus API.
///
/// [`zbus_polkit`]: https://docs.rs/zbus_polkit/1.0.0/zbus_polkit/policykit1/index.html
/// [`Proxy`]: https://docs.rs/zbus/1.0.0/zbus/struct.Proxy.html
#[proc_macro_attribute]
pub fn dbus_proxy(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemTrait);
    proxy::expand(args, input)
}

/// Derive macro for defining a D-Bus error.
///
/// This macro helps to implement an [`Error`] suitable for D-Bus handling with zbus. It will expand
/// an `enum E` with [`Error`] traits implementation, and `TryFrom<zbus::Error>`. The later will
/// help to match received method errors to known or expected errors.
///
/// Additionnally, the derived `impl E` will provide the following convenience methods:
///
/// * `name(&self)` - get the associated D-Bus error name.
///
/// * `description(&self)` - get the associated error description (the first argument of an error
///   message)
///
/// * `reply(&self, &zbus::Connection, &zbus::Message)` - send this error as reply to the message.
///
/// Note: it is recommended that errors take a single argument `String` which describes it in
/// a human-friendly fashion (support for other arguments is limited or TODO currently).
///
/// # Example
///
/// ```
/// use zbus_derive::DBusError;
///
/// #[derive(DBusError, Debug)]
/// #[dbus_error(prefix = "org.myservice.App")]
/// enum Error {
///     FileNotFound(String),
///     OutOfMemory,
/// }
/// ```
///
/// [`Error`]: http://doc.rust-lang.org/std/error/trait.Error.html
/// [`zbus::Error`]: https://docs.rs/zbus/1.0.0/zbus/enum.Error.html
#[proc_macro_derive(DBusError, attributes(dbus_error))]
pub fn derive_dbus_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    error::expand_derive(input)
}
