use std::{borrow::Cow, convert::TryInto, marker::PhantomData, sync::Arc};

use async_io::block_on;
use futures_core::future::BoxFuture;
use static_assertions::assert_impl_all;
use zvariant::ObjectPath;

use crate::{azync, Error, Result};

/// Builder for proxies.
#[derive(Debug)]
pub struct ProxyBuilder<'a, T = ()> {
    conn: azync::Connection,
    destination: Option<Cow<'a, str>>,
    path: Option<ObjectPath<'a>>,
    interface: Option<Cow<'a, str>>,
    proxy_type: PhantomData<T>,
    cache: bool,
}

impl<'a, T> Clone for ProxyBuilder<'a, T> {
    fn clone(&self) -> Self {
        Self {
            conn: self.conn.clone(),
            destination: self.destination.clone(),
            path: self.path.clone(),
            interface: self.interface.clone(),
            cache: self.cache,
            proxy_type: PhantomData,
        }
    }
}

assert_impl_all!(ProxyBuilder<'_>: Send, Sync, Unpin);

impl<'a, T> ProxyBuilder<'a, T> {
    /// Create a new [`ProxyBuilder`] for the given connection.
    pub fn new_bare<C>(conn: &C) -> Self
    where
        C: Clone + Into<azync::Connection>,
    {
        Self {
            conn: conn.clone().into(),
            destination: None,
            path: None,
            interface: None,
            cache: true,
            proxy_type: PhantomData,
        }
    }
}

impl<'a, T> ProxyBuilder<'a, T> {
    /// Set the proxy destination address.
    pub fn destination<D: Into<Cow<'a, str>>>(mut self, destination: D) -> Self {
        self.destination = Some(destination.into());
        self
    }

    /// Set the proxy path.
    pub fn path<E, P: TryInto<ObjectPath<'a>, Error = E>>(mut self, path: P) -> Result<Self>
    where
        E: Into<Error>,
    {
        self.path = Some(path.try_into().map_err(Into::into)?);
        Ok(self)
    }

    /// Set the proxy interface.
    pub fn interface<I: Into<Cow<'a, str>>>(mut self, interface: I) -> Self {
        self.interface = Some(interface.into());
        self
    }

    /// Set whether to cache properties.
    pub fn cache_properties(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }

    /// Build a proxy from the builder.
    ///
    /// # Panics
    ///
    /// Panics if the builder is lacking the necessary details to build a proxy.
    pub fn build(self) -> Result<T>
    where
        T: From<azync::Proxy<'a>>,
    {
        block_on(self.build_async())
    }

    /// Build a proxy from the builder, asynchronously.
    ///
    /// # Panics
    ///
    /// Panics if the builder is lacking the necessary details to build a proxy.
    pub fn build_async(self) -> BoxFuture<'a, Result<T>>
    where
        T: From<azync::Proxy<'a>>,
    {
        let conn = self.conn;
        let destination = self.destination.expect("missing `destination`");
        let path = self.path.expect("missing `path`");
        let interface = self.interface.expect("missing `interface`");
        let cache = self.cache;

        let proxy = azync::Proxy {
            inner: Arc::new(azync::ProxyInner::new(conn, destination, path, interface)),
            properties: Default::default(),
        };

        Box::pin(async move {
            if cache {
                proxy.cache_properties().await?;
            }

            Ok(proxy.into())
        })
    }
}

impl<'a, T> ProxyBuilder<'a, T>
where
    T: ProxyDefault,
{
    /// Create a new [`ProxyBuilder`] for the given connection.
    pub fn new<C>(conn: &C) -> Self
    where
        C: Clone + Into<azync::Connection>,
    {
        Self {
            conn: conn.clone().into(),
            destination: Some(T::DESTINATION.into()),
            path: Some(T::PATH.try_into().expect("invalid default path")),
            interface: Some(T::INTERFACE.into()),
            cache: true,
            proxy_type: PhantomData,
        }
    }
}

/// Trait for the default associated values of a proxy.
///
/// The trait is automatically implemented by the [`dbus_proxy`] macro on your behalf, and may be
/// later used to retrieve the associated constants.
///
/// [`dbus_proxy`]: attr.dbus_proxy.html
pub trait ProxyDefault {
    const INTERFACE: &'static str;
    const DESTINATION: &'static str;
    const PATH: &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Connection;
    use test_env_log::test;

    #[test]
    fn builder() {
        let conn = Connection::new_session().unwrap();

        let builder = ProxyBuilder::<azync::Proxy<'_>>::new_bare(&conn)
            .destination("org.freedesktop.DBus")
            .path("/some/path")
            .unwrap()
            .interface("org.freedesktop.Interface")
            .cache_properties(false);
        assert!(matches!(
            builder.clone().destination.unwrap(),
            Cow::Borrowed(_)
        ));
        let proxy = builder.build().unwrap();
        assert!(matches!(proxy.inner.destination, Cow::Borrowed(_)));
        assert!(matches!(proxy.inner.interface, Cow::Borrowed(_)));
    }
}
