use std::{convert::TryInto, marker::PhantomData, sync::Arc};

use static_assertions::assert_impl_all;
use zbus_names::{BusName, InterfaceName};
use zvariant::ObjectPath;

use crate::{Connection, Error, Proxy, ProxyInner, Result};

/// Builder for proxies.
#[derive(Debug)]
pub struct ProxyBuilder<'a, T = ()> {
    conn: Connection,
    destination: Option<BusName<'a>>,
    path: Option<ObjectPath<'a>>,
    interface: Option<InterfaceName<'a>>,
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
    pub fn new_bare(conn: &Connection) -> Self {
        Self {
            conn: conn.clone(),
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
    pub fn destination<D>(mut self, destination: D) -> Result<Self>
    where
        D: TryInto<BusName<'a>>,
        D::Error: Into<Error>,
    {
        self.destination = Some(destination.try_into().map_err(Into::into)?);
        Ok(self)
    }

    /// Set the proxy path.
    pub fn path<P>(mut self, path: P) -> Result<Self>
    where
        P: TryInto<ObjectPath<'a>>,
        P::Error: Into<Error>,
    {
        self.path = Some(path.try_into().map_err(Into::into)?);
        Ok(self)
    }

    /// Set the proxy interface.
    pub fn interface<I>(mut self, interface: I) -> Result<Self>
    where
        I: TryInto<InterfaceName<'a>>,
        I::Error: Into<Error>,
    {
        self.interface = Some(interface.try_into().map_err(Into::into)?);
        Ok(self)
    }

    /// Set whether to cache properties.
    pub fn cache_properties(mut self, cache: bool) -> Self {
        self.cache = cache;
        self
    }

    pub(crate) fn build_internal(self) -> Proxy<'a> {
        let conn = self.conn;
        let destination = self.destination.expect("missing `destination`");
        let path = self.path.expect("missing `path`");
        let interface = self.interface.expect("missing `interface`");
        let cache = self.cache;

        Proxy {
            inner: Arc::new(ProxyInner::new(conn, destination, path, interface, cache)),
        }
    }

    /// Build a proxy from the builder.
    ///
    /// # Panics
    ///
    /// Panics if the builder is lacking the necessary details to build a proxy.
    pub async fn build(self) -> Result<T>
    where
        T: From<Proxy<'a>>,
    {
        let proxy = self.build_internal();

        Ok(proxy.into())
    }
}

impl<'a, T> ProxyBuilder<'a, T>
where
    T: ProxyDefault,
{
    /// Create a new [`ProxyBuilder`] for the given connection.
    pub fn new(conn: &Connection) -> Self {
        Self {
            conn: conn.clone(),
            destination: Some(BusName::from_static_str(T::DESTINATION).expect("invalid bus name")),
            path: Some(ObjectPath::from_static_str(T::PATH).expect("invalid default path")),
            interface: Some(
                InterfaceName::from_static_str(T::INTERFACE).expect("invalid interface name"),
            ),
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
    use test_env_log::test;

    #[test]
    #[ntest::timeout(15000)]
    fn builder() {
        async_io::block_on(builder_async());
    }

    async fn builder_async() {
        let conn = Connection::session().await.unwrap();

        let builder = ProxyBuilder::<Proxy<'_>>::new_bare(&conn)
            .destination("org.freedesktop.DBus")
            .unwrap()
            .path("/some/path")
            .unwrap()
            .interface("org.freedesktop.Interface")
            .unwrap()
            .cache_properties(false);
        assert!(matches!(
            builder.clone().destination.unwrap(),
            BusName::Unique(_),
        ));
        let proxy = builder.build().await.unwrap();
        assert!(matches!(proxy.inner.destination, BusName::Unique(_)));
    }
}
