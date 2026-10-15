#![deny(missing_docs)]
//! Everything `#[interface]` and `#[proxy]` generate has to be documented, otherwise downstream
//! crates that opt into `missing_docs` get warnings they cannot silence.

use zbus::object_server::SignalEmitter;
use zbus_macros::interface;

/// Interface with a documented and an undocumented signal.
pub struct Test;

#[interface(name = "org.freedesktop.zbus_macros.MissingDocs")]
impl Test {
    /// A method.
    async fn method(&self) {}

    /// A documented signal.
    #[zbus(signal)]
    async fn documented(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn undocumented(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
}

/// Proxy with documented and undocumented signals that have arguments.
pub mod proxy {
    use zbus_macros::proxy;

    /// Proxy for an interface with a signal that takes more than one argument.
    #[proxy(
        interface = "org.freedesktop.zbus_macros.MissingDocsProxy",
        default_service = "org.freedesktop.zbus_macros",
        default_path = "/org/freedesktop/zbus_macros/missing_docs"
    )]
    pub trait MissingDocs {
        /// A documented signal with arguments.
        #[zbus(signal)]
        fn documented(&self, first: u8, second: &str) -> zbus::Result<()>;

        #[zbus(signal)]
        fn undocumented(&self, first: u8, second: &str) -> zbus::Result<()>;
    }
}
