#![deny(rust_2018_idioms)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/dbus2/zbus/9f7a90d2b594ddc48b7a5f39fda5e00cd56a7dfb/logo.png"
)]
#![doc = include_str!("../README.md")]
#![doc(test(attr(
    warn(unused),
    deny(warnings),
    allow(dead_code),
    // W/o this, we seem to get some bogus warning about `extern crate zbus`.
    allow(unused_extern_crates),
)))]

#[cfg(doctest)]
mod doctests {
    // Repo README.
    doc_comment::doctest!("../../README.md");
    // Book markdown checks
    doc_comment::doctest!("../../book/src/client.md");
    doc_comment::doctest!("../../book/src/concepts.md");
    // The connection chapter contains a p2p example.
    #[cfg(feature = "p2p")]
    doc_comment::doctest!("../../book/src/connection.md");
    doc_comment::doctest!("../../book/src/contributors.md");
    doc_comment::doctest!("../../book/src/introduction.md");
    doc_comment::doctest!("../../book/src/service.md");
    doc_comment::doctest!("../../book/src/blocking.md");
    doc_comment::doctest!("../../book/src/faq.md");
}

#[cfg(all(not(feature = "async-io"), not(feature = "tokio")))]
mod error_message {
    #[cfg(windows)]
    compile_error!("Either \"async-io\" (default) or \"tokio\" must be enabled. On Windows \"async-io\" is (currently) required for UNIX socket support");

    #[cfg(not(windows))]
    compile_error!("Either \"async-io\" (default) or \"tokio\" must be enabled.");
}

#[cfg(windows)]
mod win32;

mod dbus_error;
pub use dbus_error::*;

mod error;
pub use error::*;

pub mod address;
pub use address::Address;

mod guid;
pub use guid::*;

pub mod message;
pub use message::Message;

#[deprecated(since = "4.0.0", note = "Use `message::Builder` instead")]
#[doc(hidden)]
pub use message::Builder as MessageBuilder;
#[deprecated(since = "4.0.0", note = "Use `message::EndianSig` instead")]
#[doc(hidden)]
pub use message::EndianSig;
#[doc(hidden)]
pub use message::Flags as MessageFlags;
#[deprecated(since = "4.0.0", note = "Use `message::Header` instead")]
#[doc(hidden)]
pub use message::Header as MessageHeader;
#[deprecated(since = "4.0.0", note = "Use `message::PrimaryHeader` instead")]
#[doc(hidden)]
pub use message::PrimaryHeader as MessagePrimaryHeader;
#[deprecated(since = "4.0.0", note = "Use `message::Sequence` instead")]
#[doc(hidden)]
pub use message::Sequence as MessageSequence;
#[deprecated(since = "4.0.0", note = "Use `message::Type` instead")]
#[doc(hidden)]
pub use message::Type as MessageType;
#[deprecated(since = "4.0.0", note = "Use `message::NATIVE_ENDIAN_SIG` instead")]
#[doc(hidden)]
pub use message::NATIVE_ENDIAN_SIG;

pub mod connection;
/// Alias for `connection` module, for convenience.
pub use connection as conn;
pub use connection::{handshake::AuthMechanism, Connection};

#[deprecated(since = "4.0.0", note = "Use `connection::Builder` instead")]
#[doc(hidden)]
pub use connection::Builder as ConnectionBuilder;

mod message_stream;
pub use message_stream::*;
mod abstractions;
pub use abstractions::*;

pub mod match_rule;
pub use match_rule::{MatchRule, OwnedMatchRule};

#[deprecated(since = "4.0.0", note = "Use `match_rule::Builder` instead")]
#[doc(hidden)]
pub use match_rule::Builder as MatchRuleBuilder;
#[deprecated(since = "4.0.0", note = "Use `match_rule::PathSpec` instead")]
#[doc(hidden)]
pub use match_rule::PathSpec as MatchRulePathSpec;

pub mod proxy;
pub use proxy::Proxy;

#[deprecated(since = "4.0.0", note = "Use `proxy::Builder` instead")]
#[doc(hidden)]
pub use proxy::Builder as ProxyBuilder;
#[deprecated(since = "4.0.0", note = "Use `proxy::CacheProperties` instead")]
#[doc(hidden)]
pub use proxy::CacheProperties;
#[deprecated(since = "4.0.0", note = "Use `proxy::MethodFlags` instead")]
#[doc(hidden)]
pub use proxy::MethodFlags;
#[deprecated(since = "4.0.0", note = "Use `proxy::OwnerChangedStream` instead")]
#[doc(hidden)]
pub use proxy::OwnerChangedStream;
#[deprecated(since = "4.0.0", note = "Use `proxy::PropertyChanged` instead")]
#[doc(hidden)]
pub use proxy::PropertyChanged;
#[deprecated(since = "4.0.0", note = "Use `proxy::PropertyStream` instead")]
#[doc(hidden)]
pub use proxy::PropertyStream;
#[deprecated(since = "4.0.0", note = "Use `proxy::ProxyDefault` instead")]
#[doc(hidden)]
pub use proxy::ProxyDefault;

pub mod object_server;
pub use object_server::ObjectServer;

#[deprecated(since = "4.0.0", note = "Use `object_server::DispatchResult` instead")]
#[doc(hidden)]
pub use object_server::DispatchResult;
#[deprecated(since = "4.0.0", note = "Use `object_server::Interface` instead")]
#[doc(hidden)]
pub use object_server::Interface;
#[deprecated(since = "4.0.0", note = "Use `object_server::InterfaceDeref` instead")]
#[doc(hidden)]
pub use object_server::InterfaceDeref;
#[deprecated(
    since = "4.0.0",
    note = "Use `object_server::InterfaceDerefMut` instead"
)]
#[doc(hidden)]
pub use object_server::InterfaceDerefMut;
#[deprecated(since = "4.0.0", note = "Use `object_server::InterfaceRef` instead")]
#[doc(hidden)]
pub use object_server::InterfaceRef;
#[deprecated(
    since = "4.0.0",
    note = "Use `object_server::ResponseDispatchNotifier` instead"
)]
#[doc(hidden)]
pub use object_server::ResponseDispatchNotifier;
#[deprecated(since = "4.0.0", note = "Use `object_server::SignalContext` instead")]
#[doc(hidden)]
pub use object_server::SignalContext;

mod utils;
pub use utils::*;

#[macro_use]
pub mod fdo;

#[deprecated(since = "4.0.0", note = "Use `connection::Socket` instead")]
#[doc(hidden)]
pub use connection::Socket;

pub mod blocking;

pub use zbus_macros::{interface, proxy, DBusError};

// Required for the macros to function within this crate.
extern crate self as zbus;

// Macro support module, not part of the public API.
#[doc(hidden)]
pub mod export {
    pub use async_trait;
    pub use futures_core;
    pub use futures_util;
    pub use ordered_stream;
    pub use serde;
    pub use static_assertions;
}

pub use zbus_names as names;
pub use zvariant;

#[cfg(test)]
mod tests {
    use crate::utils::block_on;
    use ntest::timeout;
    use test_log::test;

    use crate::Result;

    #[test]
    #[timeout(15000)]
    fn uncached_property() {
        block_on(test_uncached_property()).unwrap();
    }

    async fn test_uncached_property() -> Result<()> {
        // A dummy boolean test service. It starts as `false` and can be
        // flipped to `true`. Two properties can access the inner value, with
        // and without caching.
        #[derive(Default)]
        struct ServiceUncachedPropertyTest(bool);
        #[crate::interface(name = "org.freedesktop.zbus.UncachedPropertyTest")]
        impl ServiceUncachedPropertyTest {
            #[zbus(property)]
            fn cached_prop(&self) -> bool {
                self.0
            }
            #[zbus(property)]
            fn uncached_prop(&self) -> bool {
                self.0
            }
            async fn set_inner_to_true(&mut self) -> zbus::fdo::Result<()> {
                self.0 = true;
                Ok(())
            }
        }

        #[crate::proxy(
            interface = "org.freedesktop.zbus.UncachedPropertyTest",
            default_service = "org.freedesktop.zbus.UncachedPropertyTest",
            default_path = "/org/freedesktop/zbus/UncachedPropertyTest"
        )]
        trait UncachedPropertyTest {
            #[zbus(property)]
            fn cached_prop(&self) -> zbus::Result<bool>;

            #[zbus(property(emits_changed_signal = "false"))]
            fn uncached_prop(&self) -> zbus::Result<bool>;

            fn set_inner_to_true(&self) -> zbus::Result<()>;
        }

        let service = crate::connection::Builder::session()
            .unwrap()
            .serve_at(
                "/org/freedesktop/zbus/UncachedPropertyTest",
                ServiceUncachedPropertyTest(false),
            )
            .unwrap()
            .build()
            .await
            .unwrap();

        let dest = service.unique_name().unwrap();

        let client_conn = crate::Connection::session().await.unwrap();
        let client = UncachedPropertyTestProxy::builder(&client_conn)
            .destination(dest)
            .unwrap()
            .build()
            .await
            .unwrap();

        // Query properties; this populates the cache too.
        assert!(!client.cached_prop().await.unwrap());
        assert!(!client.uncached_prop().await.unwrap());

        // Flip the inner value so we can observe the different semantics of
        // the two properties.
        client.set_inner_to_true().await.unwrap();

        // Query properties again; the first one should incur a stale read from
        // cache, while the second one should be able to read the live/updated
        // value.
        assert!(!client.cached_prop().await.unwrap());
        assert!(client.uncached_prop().await.unwrap());

        Ok(())
    }
}
