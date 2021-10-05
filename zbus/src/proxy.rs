use async_channel::{bounded, Receiver};
use async_recursion::async_recursion;
use event_listener::{Event, EventListener};
use futures_core::{future::BoxFuture, ready, stream};
use futures_util::stream::{FuturesUnordered, StreamExt};
use once_cell::sync::OnceCell;
use slotmap::{new_key_type, SlotMap};
use static_assertions::assert_impl_all;
use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex as SyncMutex, RwLock},
    task::{Context, Poll},
};

use zbus_names::{
    BusName, InterfaceName, MemberName, OwnedUniqueName, OwnedWellKnownName, UniqueName,
    WellKnownName,
};
use zvariant::{ObjectPath, Optional, OwnedValue, Value};

use crate::{
    fdo::{self, IntrospectableProxy, PropertiesProxy},
    Connection, Error, Message, ProxyBuilder, Result, SignalHandler, SignalHandlerKey,
};

/// The ID for a registered signal handler.
#[derive(Debug, Copy, Clone)]
pub struct SignalHandlerId(SignalHandlerKey);

assert_impl_all!(SignalHandlerId: Send, Sync, Unpin);

type PropertyChangedHandler =
    Box<dyn for<'v> FnMut(Option<&'v Value<'_>>) -> BoxFuture<'v, ()> + Send>;

new_key_type! {
    /// The ID for a registered proprety changed handler.
    struct PropertyChangedHandlerKey;
}

/// The ID for a registered proprety changed handler.
#[derive(Debug, Copy, Clone)]
pub struct PropertyChangedHandlerId {
    name: &'static str,
    key: PropertyChangedHandlerKey,
}

#[derive(Default, derivative::Derivative)]
#[derivative(Debug)]
struct PropertyValue {
    value: Option<OwnedValue>,
    #[derivative(Debug = "ignore")]
    handlers: Option<SlotMap<PropertyChangedHandlerKey, PropertyChangedHandler>>,
    event: Event,
}

// Hold proxy properties related data.
pub(crate) struct ProxyProperties<'a> {
    pub(crate) proxy: OnceCell<PropertiesProxy<'a>>,
    values: SyncMutex<HashMap<String, PropertyValue>>,
    task: OnceCell<SignalHandlerId>,
}

impl<'a> std::fmt::Debug for ProxyProperties<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProxyProperties")
            .field("values", &self.values)
            .finish_non_exhaustive()
    }
}

/// A client-side interface proxy.
///
/// A `Proxy` is a helper to interact with an interface on a remote object.
///
/// # Example
///
/// ```
/// use std::result::Result;
/// use std::error::Error;
/// use async_io::block_on;
/// use zbus::{Connection, Proxy};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     block_on(run())
/// }
///
/// async fn run() -> Result<(), Box<dyn Error>> {
///     let connection = Connection::session().await?;
///     let p = Proxy::new(
///         &connection,
///         "org.freedesktop.DBus",
///         "/org/freedesktop/DBus",
///         "org.freedesktop.DBus",
///     ).await?;
///     // owned return value
///     let _id: String = p.call("GetId", &()).await?;
///     // borrowed return value
///     let _id: &str = p.call_method("GetId", &()).await?.body()?;
///
///     Ok(())
/// }
/// ```
///
/// # Note
///
/// It is recommended to use the [`dbus_proxy`] macro, which provides a more convenient and
/// type-safe *façade* `Proxy` derived from a Rust trait.
///
/// ## Current limitations:
///
/// At the moment, `Proxy` doesn't:
///
/// * cache properties
/// * track the current name owner
/// * prevent auto-launching
///
/// [`futures` crate]: https://crates.io/crates/futures
/// [`dbus_proxy`]: attr.dbus_proxy.html
#[derive(Clone, Debug)]
pub struct Proxy<'a> {
    pub(crate) inner: Arc<ProxyInner<'a>>,
    // Use a 'static as we can't self-reference ProxyInner fields
    // eventually, we could make destination/path inside an Arc
    // but then we would have other issues with async 'static closures
    pub(crate) properties: Arc<ProxyProperties<'static>>,
}

assert_impl_all!(Proxy<'_>: Send, Sync, Unpin);

/// This is required to avoid having the Drop impl extend the lifetime 'a, which breaks zbus_xmlgen
/// (and possibly other crates).
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub(crate) struct ProxyInnerStatic {
    #[derivative(Debug = "ignore")]
    pub(crate) conn: Connection,
    // A list of the keys so that dropping the Proxy will disconnect the signals
    sig_handlers: SyncMutex<Vec<SignalHandlerKey>>,
    dest_name_watcher: OnceCell<SignalHandlerKey>,
}

#[derive(Debug)]
pub(crate) struct ProxyInner<'a> {
    inner_without_borrows: ProxyInnerStatic,
    pub(crate) destination: BusName<'a>,
    pub(crate) path: ObjectPath<'a>,
    pub(crate) interface: InterfaceName<'a>,
    // Keep it in an Arc so that dest_name_update_task can keep its own ref to it.
    dest_unique_name: Arc<RwLock<Option<OwnedUniqueName>>>,
}

impl Drop for ProxyInnerStatic {
    fn drop(&mut self) {
        for id in self.sig_handlers.get_mut().expect("lock poisoned") {
            self.conn.queue_remove_signal_handler(*id);
        }
        if let Some(id) = self.dest_name_watcher.get() {
            self.conn.queue_remove_signal_handler(*id);
        }
    }
}

pub struct PropertyStream<'a, T> {
    name: &'a str,
    event: EventListener,
    properties: Arc<ProxyProperties<'static>>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T> stream::Stream for PropertyStream<'a, T>
where
    T: TryFrom<zvariant::OwnedValue> + Unpin,
{
    type Item = Option<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let m = self.get_mut();
        ready!(Pin::new(&mut m.event).poll(cx));
        let values = m.properties.values.lock().expect("lock poisoned");
        let entry = values
            .get(m.name)
            .expect("PropertyStream with no corresponding property");
        m.event = entry.event.listen();
        let value = entry.value.as_ref().cloned();
        Poll::Ready(Some(value.and_then(|v| T::try_from(v).ok())))
    }
}

impl<'a> ProxyProperties<'a> {
    pub(crate) fn new() -> Self {
        Self {
            proxy: Default::default(),
            values: Default::default(),
            task: Default::default(),
        }
    }

    fn update_cache<'f>(
        &self,
        changed: &'f HashMap<&'f str, Value<'f>>,
        invalidated: Vec<&'f str>,
    ) -> impl Future<Output = ()> + 'f {
        let mut values = self.values.lock().expect("lock poisoned");
        let futures = FuturesUnordered::new();

        for inval in invalidated {
            if let Some(entry) = values.get_mut(&*inval) {
                entry.value = None;
                entry.event.notify(usize::MAX);
                if let Some(handlers) = &mut entry.handlers {
                    for handler in handlers.values_mut() {
                        futures.push(handler(None));
                    }
                }
            }
        }

        for (property_name, value) in changed {
            let entry = values
                .entry(property_name.to_string())
                .or_insert_with(PropertyValue::default);

            entry.value = Some(OwnedValue::from(value));
            entry.event.notify(usize::MAX);
            if let Some(handlers) = &mut entry.handlers {
                for handler in handlers.values_mut() {
                    futures.push(handler(Some(value)));
                }
            }
        }

        futures.collect()
    }
}

impl<'a> ProxyInner<'a> {
    pub(crate) fn new(
        conn: Connection,
        destination: BusName<'a>,
        path: ObjectPath<'a>,
        interface: InterfaceName<'a>,
    ) -> Self {
        Self {
            inner_without_borrows: ProxyInnerStatic {
                conn,
                sig_handlers: SyncMutex::new(Vec::new()),
                dest_name_watcher: OnceCell::new(),
            },
            destination,
            path,
            interface,
            dest_unique_name: Arc::new(RwLock::new(None)),
        }
    }

    /// Resolves the destination name to the associated unique connection name and watches for any changes.
    ///
    /// Typically you would want to create the [`Proxy`] with the well-known name of the destination
    /// service but signal messages only specify the unique name of the peer (except for signals
    /// from `org.freedesktop.DBus` service). This means we have no means to check the sender of
    /// the message. While in most cases this will not be a problem, it becomes a problem if you
    /// need to communicate with multiple services exposing the same interface, over the same
    /// connection. Hence the need for this method.
    ///
    /// This is only called when the user show interest in receiving a signal so that we don't end up
    /// doing all this needlessly.
    pub(crate) async fn destination_unique_name(&self) -> Result<()> {
        if !self.inner_without_borrows.conn.is_bus() {
            // Names don't mean much outside the bus context.
            return Ok(());
        }

        let destination = &self.destination;
        match destination {
            BusName::Unique(name) => {
                if self
                    .dest_unique_name
                    .read()
                    .expect("lock poisoned")
                    .is_none()
                {
                    *self.dest_unique_name.write().expect("lock poisoned") =
                        Some(name.to_owned().into());
                }
            }
            BusName::WellKnown(well_known_name) => {
                if self.inner_without_borrows.dest_name_watcher.get().is_some() {
                    // Already watching over the bus for any name updates so nothing to do here.
                    return Ok(());
                }

                let conn = &self.inner_without_borrows.conn;
                let dest_unique_name = self.dest_unique_name.clone();
                let well_known_name = OwnedWellKnownName::from(well_known_name.to_owned());
                let signal_expr = format!(
                    concat!(
                        "type='signal',",
                        "sender='org.freedesktop.DBus',",
                        "path='/org/freedesktop/DBus',",
                        "interface='org.freedesktop.DBus',",
                        "member='NameOwnerChanged',",
                        "arg0='{}'"
                    ),
                    well_known_name
                );

                let id = conn
                    .add_signal_handler(SignalHandler::signal(
                        ObjectPath::from_str_unchecked("/org/freedesktop/DBus"),
                        InterfaceName::from_str_unchecked("org.freedesktop.DBus"),
                        MemberName::from_str_unchecked("NameOwnerChanged"),
                        signal_expr,
                        move |msg| {
                            let dest_unique_name = dest_unique_name.clone();
                            let well_known_name = well_known_name.clone();
                            let sender_ok = msg.header().ok().map_or(false, |h| {
                                h.sender()
                                    == Ok(Some(&UniqueName::from_str_unchecked(
                                        "org.freedesktop.DBus",
                                    )))
                            });
                            if sender_ok {
                                match msg.body::<(
                                    WellKnownName<'_>,
                                    Optional<UniqueName<'_>>,
                                    Optional<UniqueName<'_>>,
                                )>() {
                                    Ok((name, _, new_owner)) if name == well_known_name => {
                                        let unique_name =
                                            new_owner.as_ref().map(|n| n.to_owned().into());
                                        *dest_unique_name.write().expect("lock poisoned") =
                                            unique_name;
                                    }
                                    _ => {}
                                }
                            }
                            Box::pin(async move {})
                        },
                    ))
                    .await?;

                if let Err(id) = self.inner_without_borrows.dest_name_watcher.set(id) {
                    conn.remove_signal_handler(id).await?;
                }

                let unique_name = match fdo::DBusProxy::new(&self.inner_without_borrows.conn)
                    .await?
                    .get_name_owner(destination.as_ref())
                    .await
                {
                    // That's ok. The destination isn't available right now.
                    Err(fdo::Error::NameHasNoOwner(_)) => None,
                    res => Some(res?),
                };

                *self.dest_unique_name.write().expect("lock poisoned") = unique_name;
            }
        }

        Ok(())
    }
}

impl<'a> Proxy<'a> {
    /// Create a new `Proxy` for the given destination/path/interface.
    pub async fn new<D, P, I>(
        conn: &Connection,
        destination: D,
        path: P,
        interface: I,
    ) -> Result<Proxy<'a>>
    where
        D: TryInto<BusName<'a>>,
        P: TryInto<ObjectPath<'a>>,
        I: TryInto<InterfaceName<'a>>,
        D::Error: Into<Error>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
    {
        ProxyBuilder::new_bare(conn)
            .destination(destination)?
            .path(path)?
            .interface(interface)?
            .build()
            .await
    }

    /// Create a new `Proxy` for the given destination/path/interface, taking ownership of all
    /// passed arguments.
    pub async fn new_owned<D, P, I>(
        conn: Connection,
        destination: D,
        path: P,
        interface: I,
    ) -> Result<Proxy<'a>>
    where
        D: TryInto<BusName<'static>>,
        P: TryInto<ObjectPath<'static>>,
        I: TryInto<InterfaceName<'static>>,
        D::Error: Into<Error>,
        P::Error: Into<Error>,
        I::Error: Into<Error>,
    {
        ProxyBuilder::new_bare(&conn)
            .destination(destination)?
            .path(path)?
            .interface(interface)?
            .build()
            .await
    }

    /// Register a changed handler for the property named `property_name`.
    ///
    /// A unique ID for the handler is returned, which can be used to deregister this handler
    /// using [`Self::disconnect_property_changed`] method.
    ///
    /// *Note:* The signal handler will be called by the executor thread of the [`Connection`].
    /// See the [`Connection::executor`] documentation for an example of how you can run the
    /// executor (and in turn all the signal handlers called) in your own thread.
    ///
    /// # Errors
    ///
    /// The current implementation requires cached properties. It returns an [`Error::Unsupported`]
    /// if the proxy isn't setup with cache.
    pub async fn connect_property_changed<H>(
        &self,
        property_name: &'static str,
        handler: H,
    ) -> Result<PropertyChangedHandlerId>
    where
        for<'v> H: FnMut(Option<&'v Value<'_>>) -> BoxFuture<'v, ()> + Send + 'static,
    {
        if !self.has_cached_properties() {
            return Err(Error::Unsupported);
        }

        let mut values = self.properties.values.lock().expect("lock poisoned");
        let entry = values
            .entry(property_name.to_string())
            .or_insert_with(PropertyValue::default);
        let handlers = entry.handlers.get_or_insert_with(SlotMap::with_key);
        let key = handlers.insert(Box::new(handler));
        Ok(PropertyChangedHandlerId {
            key,
            name: property_name,
        })
    }

    /// Deregister the property handler with the ID `handler_id`.
    ///
    /// This method returns `Ok(true)` if a handler with the id `handler_id` is found and removed;
    /// `Ok(false)` otherwise.
    pub async fn disconnect_property_changed(
        &self,
        handler_id: PropertyChangedHandlerId,
    ) -> Result<bool> {
        Ok(self
            .properties
            .values
            .lock()
            .expect("lock poisoned")
            .get_mut(handler_id.name)
            .and_then(|e| e.handlers.as_mut())
            .map_or(false, |h| h.remove(handler_id.key).is_some()))
    }

    /// Get a reference to the associated connection.
    pub fn connection(&self) -> &Connection {
        &self.inner.inner_without_borrows.conn
    }

    /// Get a reference to the destination service name.
    pub fn destination(&self) -> &BusName<'_> {
        &self.inner.destination
    }

    /// Get a reference to the object path.
    pub fn path(&self) -> &ObjectPath<'_> {
        &self.inner.path
    }

    /// Get a reference to the interface.
    pub fn interface(&self) -> &InterfaceName<'_> {
        &self.inner.interface
    }

    /// Introspect the associated object, and return the XML description.
    ///
    /// See the [xml](xml/index.html) module for parsing the result.
    pub async fn introspect(&self) -> fdo::Result<String> {
        let proxy = IntrospectableProxy::builder(&self.inner.inner_without_borrows.conn)
            .destination(&self.inner.destination)?
            .path(&self.inner.path)?
            .build()
            .await?;

        proxy.introspect().await
    }

    #[async_recursion]
    async fn properties_proxy(&self) -> Result<&PropertiesProxy<'static>> {
        match self.properties.proxy.get() {
            Some(proxy) => Ok(proxy),
            None => {
                let proxy = PropertiesProxy::builder(&self.inner.inner_without_borrows.conn)
                    // Safe because already checked earlier
                    .destination(self.inner.destination.to_owned())
                    .unwrap()
                    // Safe because already checked earlier
                    .path(self.inner.path.to_owned())
                    .unwrap()
                    // does not have properties and do not recurse!
                    .cache_properties(false)
                    .build()
                    .await?;
                // doesn't matter if another thread sets it before
                let _ = self.properties.proxy.set(proxy);
                // but we must have a Ok() here
                self.properties.proxy.get().ok_or_else(|| panic!())
            }
        }
    }

    pub(crate) async fn cache_properties(&self) -> Result<()> {
        let proxy = self.properties_proxy().await?;
        let interface = self.interface().to_owned();
        let properties = self.properties.clone();
        let id = proxy
            .connect_properties_changed(move |iface, changed, invalidated| {
                let matches = iface == interface;
                let properties = properties.clone();
                Box::pin(async move {
                    if matches {
                        properties.update_cache(&changed, invalidated).await;
                    }
                })
            })
            .await?;

        if let Err(id) = self.properties.task.set(id) {
            proxy.disconnect_signal(id).await?;
        }

        if let Ok(values) = proxy.get_all(self.inner.interface.as_ref()).await {
            for (name, value) in values {
                self.set_cached_property(name, Some(value));
            }
        }

        Ok(())
    }

    /// Get the cached value of the property `property_name`.
    ///
    /// This returns `None` if the property is not in the cache.  This could be because the cache
    /// was invalidated by an update, because caching was disabled for this property or proxy, or
    /// because the cache has not yet been populated.  Use `get_property` to fetch the value from
    /// the peer.
    pub fn cached_property<T>(&self, property_name: &str) -> fdo::Result<Option<T>>
    where
        T: TryFrom<OwnedValue>,
    {
        self.properties
            .values
            .lock()
            .expect("lock poisoned")
            .get(property_name)
            .and_then(|e| e.value.as_ref())
            .cloned()
            .map(T::try_from)
            .transpose()
            .map_err(|_| Error::InvalidReply.into())
    }

    fn set_cached_property(&self, property_name: String, value: Option<OwnedValue>) {
        let mut values = self.properties.values.lock().expect("lock poisoned");
        let entry = values
            .entry(property_name)
            .or_insert_with(PropertyValue::default);
        entry.value = value;
    }

    async fn get_proxy_property(&self, property_name: &str) -> Result<OwnedValue> {
        Ok(self
            .properties_proxy()
            .await?
            .get(self.inner.interface.as_ref(), property_name)
            .await?)
    }

    fn has_cached_properties(&self) -> bool {
        self.properties.task.get().is_some()
    }

    /// Get the property `property_name`.
    ///
    /// Get the property value from the cache (if caching is enabled on this proxy) or call the
    /// `Get` method of the `org.freedesktop.DBus.Properties` interface.
    pub async fn get_property<T>(&self, property_name: &str) -> fdo::Result<T>
    where
        T: TryFrom<OwnedValue>,
    {
        let value = if self.has_cached_properties() {
            if let Some(value) = self.cached_property(property_name)? {
                return Ok(value);
            } else {
                let value = self.get_proxy_property(property_name).await?;
                self.set_cached_property(property_name.to_string(), Some(value.clone()));
                value
            }
        } else {
            self.get_proxy_property(property_name).await?
        };

        value.try_into().map_err(|_| Error::InvalidReply.into())
    }

    /// Set the property `property_name`.
    ///
    /// Effectively, call the `Set` method of the `org.freedesktop.DBus.Properties` interface.
    pub async fn set_property<'t, T: 't>(&self, property_name: &str, value: T) -> fdo::Result<()>
    where
        T: Into<Value<'t>>,
    {
        self.properties_proxy()
            .await?
            .set(self.inner.interface.as_ref(), property_name, &value.into())
            .await
    }

    /// Call a method and return the reply.
    ///
    /// Typically, you would want to use [`call`] method instead. Use this method if you need to
    /// deserialize the reply message manually (this way, you can avoid the memory
    /// allocation/copying, by deserializing the reply to an unowned type).
    ///
    /// [`call`]: struct.Proxy.html#method.call
    pub async fn call_method<'m, M, B>(&self, method_name: M, body: &B) -> Result<Arc<Message>>
    where
        M: TryInto<MemberName<'m>>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + zvariant::DynamicType,
    {
        self.inner
            .inner_without_borrows
            .conn
            .call_method(
                Some(&self.inner.destination),
                self.inner.path.as_str(),
                Some(&self.inner.interface),
                method_name,
                body,
            )
            .await
    }

    /// Call a method and return the reply body.
    ///
    /// Use [`call_method`] instead if you need to deserialize the reply manually/separately.
    ///
    /// [`call_method`]: struct.Proxy.html#method.call_method
    pub async fn call<'m, M, B, R>(&self, method_name: M, body: &B) -> Result<R>
    where
        M: TryInto<MemberName<'m>>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + zvariant::DynamicType,
        R: serde::de::DeserializeOwned + zvariant::Type,
    {
        let reply = self.call_method(method_name, body).await?;

        Ok(reply.body()?)
    }

    /// Create a stream for signal named `signal_name`.
    ///
    /// # Errors
    ///
    /// Apart from general I/O errors that can result from socket communications, calling this
    /// method will also result in an error if the destination service has not yet registered its
    /// well-known name with the bus (assuming you're using the well-known name as destination).
    pub async fn receive_signal<M>(&self, signal_name: M) -> Result<SignalStream>
    where
        M: TryInto<MemberName<'static>>,
        M::Error: Into<Error>,
    {
        // Time to try & resolve the destination name & track changes to it.
        self.inner.destination_unique_name().await?;

        let signal_name = signal_name.try_into().map_err(Into::into)?;
        let expr = format!(
            "type='signal',sender='{}',path='{}',interface='{}',member='{}'",
            self.destination(),
            self.path(),
            self.interface(),
            &signal_name,
        );

        let dest_unique_name = self.inner.dest_unique_name.clone();
        let conn = self.inner.inner_without_borrows.conn.clone();
        let (send, recv) = bounded(64);

        let handler_id = conn
            .add_signal_handler(SignalHandler::signal(
                self.path().to_owned(),
                self.interface().to_owned(),
                signal_name,
                expr,
                move |msg| {
                    let dest_unique_name = dest_unique_name.clone();
                    let send = send.clone();
                    Box::pin(async move {
                        if let Ok(h) = msg.header() {
                            if let Ok(s) = h.sender() {
                                if s == dest_unique_name.read().expect("lock poisoned").as_deref() {
                                    let _ = send.send(msg.clone()).await;
                                }
                            }
                        }
                    })
                },
            ))
            .await?;

        Ok(SignalStream {
            stream: recv,
            conn,
            handler_id,
        })
    }

    /// Create a stream for all signals emitted by this service.
    ///
    /// # Errors
    ///
    /// Apart from general I/O errors that can result from socket communications, calling this
    /// method will also result in an error if the destination service has not yet registered its
    /// well-known name with the bus (assuming you're using the well-known name as destination).
    pub async fn receive_all_signals(&self) -> Result<SignalStream> {
        // Time to try & resolve the destination name & track changes to it.
        self.inner.destination_unique_name().await?;

        let expr = format!(
            "type='signal',sender='{}',path='{}',interface='{}'",
            self.destination(),
            self.path(),
            self.interface(),
        );

        let dest_unique_name = self.inner.dest_unique_name.clone();
        let conn = self.inner.inner_without_borrows.conn.clone();
        let (send, recv) = bounded(64);

        let handler_id = conn
            .add_signal_handler(SignalHandler::signal(
                self.path().to_owned(),
                self.interface().to_owned(),
                None,
                expr,
                move |msg| {
                    let dest_unique_name = dest_unique_name.clone();
                    let send = send.clone();
                    Box::pin(async move {
                        if let Ok(h) = msg.header() {
                            if let Ok(s) = h.sender() {
                                if s == dest_unique_name.read().expect("lock poisoned").as_deref() {
                                    let _ = send.send(msg.clone()).await;
                                }
                            }
                        }
                    })
                },
            ))
            .await?;

        Ok(SignalStream {
            stream: recv,
            conn,
            handler_id,
        })
    }

    /// Register a handler for signal named `signal_name`.
    ///
    /// A unique ID for the handler is returned, which can be used to deregister this handler using
    /// [`Self::disconnect_signal`] method.
    ///
    /// *Note:* The signal handler will be called by the executor thread of the [`Connection`].
    /// See the [`Connection::executor`] documentation for an example of how you can run the
    /// executor (and in turn all the signal handlers called) in your own thread.
    ///
    /// ### Errors
    ///
    /// This method can fail if addition of the relevant match rule on the bus fails. You can
    /// safely `unwrap` the `Result` if you're certain that associated connection is not a bus
    /// connection.
    pub async fn connect_signal<M, H>(
        &self,
        signal_name: M,
        mut handler: H,
    ) -> fdo::Result<SignalHandlerId>
    where
        M: TryInto<MemberName<'static>>,
        M::Error: Into<Error>,
        for<'msg> H: FnMut(&'msg Message) -> BoxFuture<'msg, ()> + Send + 'static,
    {
        // Time to try resolve the destination name & track changes to it.
        self.inner.destination_unique_name().await?;

        let signal_name = signal_name.try_into().map_err(Into::into)?;
        let expr = format!(
            "type='signal',sender='{}',path='{}',interface='{}',member='{}'",
            self.destination(),
            self.path(),
            self.interface(),
            &signal_name,
        );

        let msg_handler = SignalHandler::signal(
            self.path().to_owned(),
            self.interface().to_owned(),
            signal_name,
            expr,
            move |msg| handler(msg),
        );
        let id = self
            .inner
            .inner_without_borrows
            .conn
            .add_signal_handler(msg_handler)
            .await?;

        self.inner
            .inner_without_borrows
            .sig_handlers
            .lock()
            .expect("lock poisoned")
            .push(id);

        Ok(SignalHandlerId(id))
    }

    /// Deregister the signal handler with the ID `handler_id`.
    ///
    /// This method returns `Ok(true)` if a handler with the id `handler_id` is found and removed;
    /// `Ok(false)` otherwise.
    ///
    /// ### Errors
    ///
    /// This method can fail if removal of the relevant match rule on the bus fails. You can
    /// safely `unwrap` the `Result` if you're certain that associated connection is not a bus
    /// connection.
    pub async fn disconnect_signal(&self, handler_id: SignalHandlerId) -> fdo::Result<bool> {
        self.inner
            .inner_without_borrows
            .sig_handlers
            .lock()
            .expect("lock poisoned")
            .retain(|id| *id != handler_id.0);
        Ok(self
            .inner
            .inner_without_borrows
            .conn
            .remove_signal_handler(handler_id.0)
            .await?)
    }

    /// Get a stream to receive property changed events.
    ///
    /// Note that zbus doesn't queue the updates. If the listener is slower than the receiver, it
    /// will only receive the last update.
    pub async fn receive_property_stream<'n, T>(&self, name: &'n str) -> PropertyStream<'n, T> {
        let mut values = self.properties.values.lock().expect("lock poisoned");
        let entry = values
            .entry(name.to_string())
            .or_insert_with(PropertyValue::default);
        let event = entry.event.listen();

        PropertyStream {
            name,
            event,
            properties: self.properties.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

/// A [`stream::Stream`] implementation that yields signal [messages](`Message`).
///
/// Use [`Proxy::receive_signal`] to create an instance of this type.
#[derive(Debug)]
pub struct SignalStream {
    stream: Receiver<Arc<Message>>,
    conn: Connection,
    handler_id: SignalHandlerKey,
}

assert_impl_all!(SignalStream: Send, Sync, Unpin);

impl stream::Stream for SignalStream {
    type Item = Arc<Message>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        stream::Stream::poll_next(Pin::new(&mut self.get_mut().stream), cx)
    }
}

impl std::ops::Drop for SignalStream {
    fn drop(&mut self) {
        self.conn.queue_remove_signal_handler(self.handler_id);
    }
}

impl<'a> From<crate::blocking::Proxy<'a>> for Proxy<'a> {
    fn from(proxy: crate::blocking::Proxy<'a>) -> Self {
        proxy.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use event_listener::Event;
    use zbus_names::UniqueName;

    use super::*;
    use async_io::block_on;
    use futures_util::{future::FutureExt, join};
    use ntest::timeout;
    use std::{future::ready, sync::Arc};
    use test_env_log::test;
    use zvariant::Optional;

    #[test]
    #[timeout(15000)]
    fn signal_stream() {
        block_on(test_signal_stream()).unwrap();
    }

    async fn test_signal_stream() -> Result<()> {
        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let conn = Connection::session().await?;
        let unique_name = conn.unique_name().unwrap();

        let proxy = fdo::DBusProxy::new(&conn).await?;

        let well_known = "org.freedesktop.zbus.async.ProxySignalStreamTest";
        let owner_changed_stream = proxy
            .receive_signal("NameOwnerChanged")
            .await?
            .filter(|msg| {
                if let Ok((name, _, new_owner)) = msg.body::<(
                    BusName<'_>,
                    Optional<UniqueName<'_>>,
                    Optional<UniqueName<'_>>,
                )>() {
                    ready(match &*new_owner {
                        Some(new_owner) => *new_owner == *unique_name && name == well_known,
                        None => false,
                    })
                } else {
                    ready(false)
                }
            });

        let name_acquired_stream = proxy.receive_signal("NameAcquired").await?.filter(|msg| {
            if let Ok(name) = msg.body::<BusName<'_>>() {
                return ready(name == well_known);
            }

            ready(false)
        });

        let _prop_stream =
            proxy
                .receive_property_stream("SomeProp")
                .await
                .filter(|v: &Option<u32>| {
                    dbg!(v);
                    ready(false)
                });

        let reply = proxy
            .request_name(
                well_known.try_into()?,
                fdo::RequestNameFlags::ReplaceExisting.into(),
            )
            .await?;
        assert_eq!(reply, fdo::RequestNameReply::PrimaryOwner);

        let (changed_signal, acquired_signal) = futures_util::join!(
            owner_changed_stream.into_future(),
            name_acquired_stream.into_future(),
        );

        let changed_signal = changed_signal.0.unwrap();
        let (acquired_name, _, new_owner) = changed_signal
            .body::<(
                BusName<'_>,
                Optional<UniqueName<'_>>,
                Optional<UniqueName<'_>>,
            )>()
            .unwrap();
        assert_eq!(acquired_name, well_known);
        assert_eq!(*new_owner.as_ref().unwrap(), *unique_name);

        let acquired_signal = acquired_signal.0.unwrap();
        assert_eq!(acquired_signal.body::<&str>().unwrap(), well_known);

        Ok(())
    }

    #[test]
    #[timeout(15000)]
    fn signal_connect() {
        block_on(test_signal_connect()).unwrap();
    }

    async fn test_signal_connect() -> Result<()> {
        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let conn = Connection::session().await?;

        let owner_change_signaled = Arc::new(Event::new());
        let owner_change_listener = owner_change_signaled.listen();

        let name_acquired_signaled = Arc::new(Event::new());
        let name_acquired_listener = name_acquired_signaled.listen();

        let name_acquired_signaled2 = Arc::new(Event::new());
        let name_acquired_listener2 = name_acquired_signaled2.listen();

        let proxy = fdo::DBusProxy::new(&conn).await?;
        let well_known = "org.freedesktop.zbus.async.ProxySignalConnectTest";
        let unique_name = conn.unique_name().unwrap().clone();
        let name_owner_changed_id = {
            proxy
                .connect_signal("NameOwnerChanged", move |m| {
                    let unique_name = unique_name.clone();
                    let signaled = owner_change_signaled.clone();

                    async move {
                        let (name, _, new_owner) = m
                            .body::<(
                                BusName<'_>,
                                Optional<UniqueName<'_>>,
                                Optional<UniqueName<'_>>,
                            )>()
                            .unwrap();
                        if name != well_known {
                            // Meant for the other testcase then
                            return;
                        }
                        assert_eq!(*new_owner.as_ref().unwrap(), *unique_name);
                        signaled.notify(1);
                    }
                    .boxed()
                })
                .await?
        };
        // `NameAcquired` is emitted twice, first when the unique name is assigned on
        // connection and secondly after we ask for a specific name.
        let name_acquired_id = proxy
            .connect_signal("NameAcquired", move |m| {
                let signaled = name_acquired_signaled.clone();
                async move {
                    if m.body::<&str>().unwrap() == well_known {
                        signaled.notify(1);
                    }
                }
                .boxed()
            })
            .await?;
        // Test multiple handers for the same signal
        let name_acquired_id2 = proxy
            .connect_signal("NameAcquired", move |m| {
                let signaled = name_acquired_signaled2.clone();
                async move {
                    if m.body::<&str>().unwrap() == well_known {
                        signaled.notify(1);
                    }
                }
                .boxed()
            })
            .await?;

        crate::blocking::fdo::DBusProxy::new(&crate::blocking::Connection::from(conn))?
            .request_name(
                well_known.try_into()?,
                fdo::RequestNameFlags::ReplaceExisting.into(),
            )
            .unwrap();

        join!(
            owner_change_listener,
            name_acquired_listener,
            name_acquired_listener2,
        );

        assert!(proxy.disconnect_signal(name_owner_changed_id).await?);
        assert!(!proxy.disconnect_signal(name_owner_changed_id).await?);
        assert!(proxy.disconnect_signal(name_acquired_id).await?);
        assert!(proxy.disconnect_signal(name_acquired_id2).await?);

        Ok(())
    }
}
