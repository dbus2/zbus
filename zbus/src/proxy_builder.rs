use std::{borrow::Cow, convert::TryInto, marker::PhantomData, sync::Arc};

use static_assertions::assert_impl_all;
use zvariant::ObjectPath;

use crate::{azync, Error, Proxy, Result};

/// Builder for [`Proxy`]
#[derive(Debug, Clone)]
pub struct ProxyBuilder<'a, T = ()> {
    conn: azync::Connection,
    destination: Option<Cow<'a, str>>,
    path: Option<ObjectPath<'a>>,
    interface: Option<Cow<'a, str>>,
    proxy_type: PhantomData<T>,
}

assert_impl_all!(ProxyBuilder<'_>: Send, Sync, Unpin);

impl<'a> ProxyBuilder<'a> {
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
        Error: From<E>,
    {
        self.path = Some(path.try_into()?);
        Ok(self)
    }

    /// Set the proxy interface.
    pub fn interface<I: Into<Cow<'a, str>>>(mut self, interface: I) -> Self {
        self.interface = Some(interface.into());
        self
    }

    /// Build a generic [`Proxy`] from the builder.
    ///
    /// Note: to build a higher-level proxy, generated from [`dbus_proxy`] macro, you should use
    /// [`ProxyBuilder::build`] instead.
    ///
    /// # Panics
    ///
    /// Panics if the builder is lacking the necessary details to build a proxy.
    ///
    /// [`dbus_proxy`]: attr.dbus_proxy.html
    pub fn build_bare(self) -> Proxy<'a> {
        self.build_bare_async().into()
    }

    /// Build a generic [`azync::Proxy`] from the builder.
    ///
    /// # Panics
    ///
    /// Panics if the builder is lacking the necessary details to build a proxy.
    pub fn build_bare_async(self) -> azync::Proxy<'a> {
        let conn = self.conn;
        let destination = self.destination.expect("missing `destination`");
        let path = self.path.expect("missing `path`");
        let interface = self.interface.expect("missing `interface`");

        azync::Proxy {
            inner: Arc::new(azync::ProxyInner::new(conn, destination, path, interface)),
        }
    }
}

impl<'a, T> ProxyBuilder<'a, T>
where
    T: ProxyDefault + From<azync::Proxy<'a>>,
{
    /// Create a new [`ProxyBuilder`] for the given connection.
    pub fn new<C>(conn: &C) -> Self
    where
        C: Clone + Into<azync::Connection>,
    {
        Self {
            conn: conn.clone().into(),
            destination: None,
            path: None,
            interface: None,
            proxy_type: PhantomData,
        }
    }

    /// Build a proxy from the builder.
    ///
    /// This function is meant to be called with higher-level proxies, generated from the
    /// [`dbus_proxy`] macro. When missing, default values are taken from [`ProxyDefault`].
    ///
    /// If you need a generic [`Proxy`], you can use [`ProxyBuilder::build_bare`] instead.
    ///
    /// [`dbus_proxy`]: attr.dbus_proxy.html
    pub fn build(self) -> T {
        let conn = self.conn;
        let destination = self.destination.unwrap_or_else(|| T::DESTINATION.into());
        let path = self
            .path
            .unwrap_or_else(|| T::PATH.try_into().expect("invalid default path"));
        let interface = self.interface.unwrap_or_else(|| T::INTERFACE.into());

        azync::Proxy {
            inner: Arc::new(azync::ProxyInner::new(conn, destination, path, interface)),
        }
        .into()
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

        let builder = ProxyBuilder::new_bare(&conn)
            .destination("org.freedesktop.DBus")
            .path("/some/path")
            .unwrap()
            .interface("org.freedesktop.Interface");
        assert!(matches!(
            builder.clone().destination.unwrap(),
            Cow::Borrowed(_)
        ));
        let proxy = builder.build_bare_async();
        assert!(matches!(proxy.inner.destination, Cow::Borrowed(_)));
        assert!(matches!(proxy.inner.interface, Cow::Borrowed(_)));
    }
}
