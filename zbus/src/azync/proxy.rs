use async_broadcast::{broadcast, InactiveReceiver, Sender as Broadcaster};
use async_lock::Mutex;
use async_recursion::async_recursion;
use async_task::Task;
use futures_core::{future::BoxFuture, stream};
use futures_util::stream::StreamExt;
use once_cell::sync::OnceCell;
use slotmap::{new_key_type, SlotMap};
use static_assertions::assert_impl_all;
use std::{
    borrow::Cow,
    collections::HashMap,
    convert::{TryFrom, TryInto},
    future::ready,
    io::{self, ErrorKind},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use zvariant::{ObjectPath, OwnedValue, Value};

use crate::{
    azync::Connection,
    fdo::{self, AsyncIntrospectableProxy, AsyncPropertiesProxy},
    Error, Message, MessageHeader, MessageType, Result,
};

type SignalHandler = Box<dyn for<'msg> FnMut(&'msg Message) -> BoxFuture<'msg, Result<()>> + Send>;

new_key_type! {
    /// The ID for a registered signal handler.
    pub struct SignalHandlerId;
}

assert_impl_all!(SignalHandlerId: Send, Sync, Unpin);

struct SignalHandlerInfo {
    signal_name: &'static str,
    handler: SignalHandler,
}

type PropertyChangedEvent = Arc<(String, Option<OwnedValue>)>;

type PropertyChangedHandler =
    Box<dyn for<'v> FnMut(Option<&'v Value<'_>>) -> BoxFuture<'v, ()> + Send>;

new_key_type! {
    /// The ID for a registered proprety changed handler.
    pub struct PropertyChangedHandlerId;
}

pub(crate) struct PropertyChangedHandlerInfo {
    property_name: &'static str,
    handler: PropertyChangedHandler,
}

// Hold proxy properties related data.
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub(crate) struct ProxyProperties<'a> {
    pub(crate) proxy: OnceCell<AsyncPropertiesProxy<'a>>,
    pub(crate) values: Mutex<HashMap<String, OwnedValue>>,
    task: OnceCell<Task<()>>,
    #[derivative(Debug = "ignore")]
    pub(crate) changed_handlers:
        Mutex<SlotMap<PropertyChangedHandlerId, PropertyChangedHandlerInfo>>,
    broadcaster: Broadcaster<PropertyChangedEvent>,
    receiver: InactiveReceiver<PropertyChangedEvent>,
}

/// The asynchronous sibling of [`crate::Proxy`].
///
/// This API is mostly the same as [`crate::Proxy`], except it is asynchronous. One of the
/// implications of asynchronous API is that apart from the signal handling through connecting and
/// disconnecting handlers (using [`Proxy::connect_signal`] and [`Proxy::disconnect_signal`] methods),
/// you can also receive signals through a stream using [`Proxy::receive_signal`] method.
///
/// Another implication of asynchronous API is that we do not need [`crate::SignalReceiver`] here.
/// The [`futures` crate] provides API to combine futures and streams already.
///
/// # Example
///
/// ```
/// use std::result::Result;
/// use std::error::Error;
/// use async_io::block_on;
/// use zbus::azync::{Connection, Proxy};
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     block_on(run())
/// }
///
/// async fn run() -> Result<(), Box<dyn Error>> {
///     let connection = Connection::new_session().await?;
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

#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub(crate) struct ProxyInner<'a> {
    pub(crate) conn: Connection,
    pub(crate) destination: Cow<'a, str>,
    pub(crate) path: ObjectPath<'a>,
    pub(crate) interface: Cow<'a, str>,
    dest_unique_name: OnceCell<String>,
    #[derivative(Debug = "ignore")]
    sig_handlers: Mutex<SlotMap<SignalHandlerId, SignalHandlerInfo>>,
    #[derivative(Debug = "ignore")]
    signal_msg_stream: OnceCell<Mutex<Connection>>,
}

pub struct PropertyStream<'a, T> {
    name: &'a str,
    stream: stream::BoxStream<'static, PropertyChangedEvent>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T> stream::Stream for PropertyStream<'a, T>
where
    T: TryFrom<zvariant::OwnedValue> + Unpin,
{
    type Item = Option<T>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let m = self.get_mut();
        let (name, stream) = (m.name, m.stream.as_mut());
        // there must be a way to simplify the following code..
        let p = stream::Stream::poll_next(stream, cx);
        match p {
            Poll::Ready(Some(item)) => {
                if item.0 == name {
                    if let Some(Ok(v)) = item.1.clone().map(T::try_from) {
                        Poll::Ready(Some(Some(v)))
                    } else {
                        Poll::Ready(Some(None))
                    }
                } else {
                    Poll::Pending
                }
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'a> ProxyProperties<'a> {
    pub(crate) fn new() -> Self {
        // note: do we need to make this configurable?
        let (mut sender, receiver) = broadcast(1);
        sender.set_overflow(true);
        let receiver = receiver.deactivate();

        Self {
            proxy: Default::default(),
            values: Default::default(),
            task: Default::default(),
            changed_handlers: Default::default(),
            broadcaster: sender,
            receiver,
        }
    }

    async fn changed(&self, property_name: &str, value: Option<&Value<'_>>) {
        if self.broadcaster.receiver_count() > 0 {
            // Ignore event errors.
            // TODO: We should still log in case of error when we've logging.
            let _res = self
                .broadcaster
                .broadcast(Arc::new((
                    property_name.to_string(),
                    value.map(OwnedValue::from),
                )))
                .await;
        }

        let mut handlers = self.changed_handlers.lock().await;
        for info in handlers
            .values_mut()
            .filter(|info| info.property_name == property_name)
        {
            (*info.handler)(value).await;
        }
    }
}

impl<'a> ProxyInner<'a> {
    pub(crate) fn new(
        conn: Connection,
        destination: Cow<'a, str>,
        path: ObjectPath<'a>,
        interface: Cow<'a, str>,
    ) -> Self {
        Self {
            conn,
            destination,
            path,
            interface,
            dest_unique_name: OnceCell::new(),
            sig_handlers: Mutex::new(SlotMap::with_key()),
            signal_msg_stream: OnceCell::new(),
        }
    }

    // panic if dest_unique_name has not been resolved before
    fn matching_signal<'m>(&self, msg: &'m Message, h: &'m MessageHeader<'m>) -> Option<&'m str> {
        if msg.primary_header().msg_type() != MessageType::Signal {
            return None;
        }
        let uniq = self.dest_unique_name.get().unwrap();
        if h.interface() == Ok(Some(&self.interface))
            && h.sender() == Ok(Some(uniq))
            && h.path() == Ok(Some(&self.path))
        {
            h.member().ok().flatten()
        } else {
            None
        }
    }
}

impl<'a> Proxy<'a> {
    /// Create a new `Proxy` for the given destination/path/interface.
    pub async fn new<E>(
        conn: &Connection,
        destination: &'a str,
        path: impl TryInto<ObjectPath<'a>, Error = E>,
        interface: &'a str,
    ) -> Result<Proxy<'a>>
    where
        E: Into<Error>,
    {
        crate::ProxyBuilder::new_bare(conn)
            .destination(destination)
            .path(path)?
            .interface(interface)
            .build_async()
            .await
    }

    /// Create a new `Proxy` for the given destination/path/interface, taking ownership of all
    /// passed arguments.
    pub async fn new_owned<E>(
        conn: Connection,
        destination: String,
        path: impl TryInto<ObjectPath<'static>, Error = E>,
        interface: String,
    ) -> Result<Proxy<'a>>
    where
        E: Into<Error>,
    {
        crate::ProxyBuilder::new_bare(&conn)
            .destination(destination)
            .path(path)?
            .interface(interface)
            .build_async()
            .await
    }

    /// Register a changed handler for the property named `property_name`.
    ///
    /// Once a handler is successfully registered, call [`Self::next_signal`] to wait for the next
    /// signal to arrive and be handled by its registered handler. A unique ID for the handler is
    /// returned, which can be used to deregister this handler using
    /// [`Self::disconnect_property_changed`] method.
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

        let id = self
            .properties
            .changed_handlers
            .lock()
            .await
            .insert(PropertyChangedHandlerInfo {
                property_name,
                handler: Box::new(handler),
            });
        Ok(id)
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
            .changed_handlers
            .lock()
            .await
            .remove(handler_id)
            .is_some())
    }

    /// Get a reference to the associated connection.
    pub fn connection(&self) -> &Connection {
        &self.inner.conn
    }

    /// Get a reference to the destination service name.
    pub fn destination(&self) -> &str {
        &self.inner.destination
    }

    /// Get a reference to the object path.
    pub fn path(&self) -> &ObjectPath<'_> {
        &self.inner.path
    }

    /// Get a reference to the interface.
    pub fn interface(&self) -> &str {
        &self.inner.interface
    }

    /// Introspect the associated object, and return the XML description.
    ///
    /// See the [xml](xml/index.html) module for parsing the result.
    pub async fn introspect(&self) -> fdo::Result<String> {
        let proxy = AsyncIntrospectableProxy::builder(&self.inner.conn)
            .destination(self.inner.destination.as_ref())
            .path(&self.inner.path)?
            .build()?;

        proxy.introspect().await
    }

    #[async_recursion]
    async fn properties_proxy(&self) -> Result<&AsyncPropertiesProxy<'static>> {
        match self.properties.proxy.get() {
            Some(proxy) => Ok(proxy),
            None => {
                let proxy = AsyncPropertiesProxy::builder(&self.inner.conn)
                    .destination(self.inner.destination.to_string())
                    // Safe because already checked earlier
                    .path(self.inner.path.to_owned())
                    .unwrap()
                    // does not have properties and do not recurse!
                    .cache_properties(false)
                    .build_async()
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

        let mut stream = proxy.receive_properties_changed().await?;
        let properties = self.properties.clone();
        let task = self.inner.conn.executor().spawn(async move {
            while let Some(changed) = stream.next().await {
                if let Ok(args) = changed.args() {
                    let mut values = properties.values.lock().await;
                    for inval in args.invalidated_properties() {
                        properties.changed(inval, None).await;
                        values.remove(*inval);
                    }

                    for (property_name, value) in args
                        .changed_properties()
                        .iter()
                        .map(|(k, v)| (k.to_string(), OwnedValue::from(v)))
                    {
                        // we should notify after insert, but this requires extra lookup atm
                        properties.changed(&property_name, Some(&value)).await;
                        values.insert(property_name, value);
                    }
                }
            }
        });
        self.properties.task.set(task).unwrap();

        if let Ok(values) = proxy.get_all(&self.inner.interface).await {
            self.properties.values.lock().await.extend(values);
        }

        Ok(())
    }

    async fn get_cached_property(&self, property_name: &str) -> Option<OwnedValue> {
        self.properties
            .values
            .lock()
            .await
            .get(property_name)
            .cloned()
    }

    async fn set_cached_property(&self, property_name: String, value: Option<OwnedValue>) {
        let mut values = self.properties.values.lock().await;
        if let Some(value) = value {
            values.insert(property_name, value);
        } else {
            values.remove(&property_name);
        }
    }

    async fn get_proxy_property(&self, property_name: &str) -> Result<OwnedValue> {
        Ok(self
            .properties_proxy()
            .await?
            .get(&self.inner.interface, property_name)
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
            if let Some(value) = self.get_cached_property(property_name).await {
                value
            } else {
                let value = self.get_proxy_property(property_name).await?;
                self.set_cached_property(property_name.to_string(), Some(value.clone()))
                    .await;
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
            .set(&self.inner.interface, property_name, &value.into())
            .await
    }

    /// Call a method and return the reply.
    ///
    /// Typically, you would want to use [`call`] method instead. Use this method if you need to
    /// deserialize the reply message manually (this way, you can avoid the memory
    /// allocation/copying, by deserializing the reply to an unowned type).
    ///
    /// [`call`]: struct.Proxy.html#method.call
    pub async fn call_method<B>(&self, method_name: &str, body: &B) -> Result<Arc<Message>>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        self.inner
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
    pub async fn call<B, R>(&self, method_name: &str, body: &B) -> Result<R>
    where
        B: serde::ser::Serialize + zvariant::Type,
        R: serde::de::DeserializeOwned + zvariant::Type,
    {
        let reply = self.call_method(method_name, body).await?;
        // Since we don't keep the reply msg around and user still might use the FDs after this
        // call returns, we must disown the FDs so we don't end up closing them after the call.
        reply.disown_fds();

        Ok(reply.body()?)
    }

    /// Create a stream for signal named `signal_name`.
    ///
    /// # Errors
    ///
    /// Apart from general I/O errors that can result from socket communications, calling this
    /// method will also result in an error if the destination service has not yet registered its
    /// well-known name with the bus (assuming you're using the well-known name as destination).
    pub async fn receive_signal(&self, signal_name: &'static str) -> Result<SignalStream<'a>> {
        let subscription_id = if self.inner.conn.is_bus() {
            let id = self
                .inner
                .conn
                .subscribe_signal(
                    self.destination(),
                    self.path().clone(),
                    self.interface(),
                    signal_name,
                )
                .await?;

            Some(id)
        } else {
            None
        };

        self.destination_unique_name().await?;
        let proxy = self.inner.clone();
        let stream = self
            .inner
            .conn
            .clone()
            .filter(move |m| {
                ready(
                    m.as_ref()
                        .ok()
                        .and_then(|m| {
                            m.header()
                                .map(|h| proxy.matching_signal(m, &h) == Some(signal_name))
                                .ok()
                        })
                        .unwrap_or(false),
                )
            })
            // Safety: Filter above ensures we only get `Ok(msg)`.
            .map(|msg| msg.unwrap());

        Ok(SignalStream {
            stream: stream.boxed(),
            conn: self.inner.conn.clone(),
            subscription_id,
        })
    }

    /// Register a handler for signal named `signal_name`.
    ///
    /// Once a handler is successfully registered, call [`Self::next_signal`] to wait for the next
    /// signal to arrive and be handled by its registered handler. A unique ID for the handler is
    /// returned, which can be used to deregister this handler using [`Self::disconnect_signal`]
    /// method.
    ///
    /// ### Errors
    ///
    /// This method can fail if addition of the relevant match rule on the bus fails. You can
    /// safely `unwrap` the `Result` if you're certain that associated connection is not a bus
    /// connection.
    pub async fn connect_signal<H>(
        &self,
        signal_name: &'static str,
        handler: H,
    ) -> fdo::Result<SignalHandlerId>
    where
        for<'msg> H: FnMut(&'msg Message) -> BoxFuture<'msg, Result<()>> + Send + 'static,
    {
        // Ensure the stream.
        self.msg_stream().await;

        let id = self
            .inner
            .sig_handlers
            .lock()
            .await
            .insert(SignalHandlerInfo {
                signal_name,
                handler: Box::new(handler),
            });

        if self.inner.conn.is_bus() {
            let _ = self
                .inner
                .conn
                .subscribe_signal(
                    self.destination(),
                    self.path().clone(),
                    self.interface(),
                    signal_name,
                )
                .await?;
        }

        Ok(id)
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
        match self.inner.sig_handlers.lock().await.remove(handler_id) {
            Some(handler_info) => {
                if self.inner.conn.is_bus() {
                    let _ = self
                        .inner
                        .conn
                        .unsubscribe_signal(
                            self.destination(),
                            self.path().clone(),
                            self.interface(),
                            handler_info.signal_name,
                        )
                        .await?;
                }

                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Receive and handle the next incoming signal on the associated connection.
    ///
    /// This method will wait for signal messages on the associated connection and call any
    /// handlers registered through the [`Self::connect_signal`] method.
    ///
    /// If the signal message was handled by a handler, `Ok(None)` is returned. Otherwise, the
    /// received message is returned.
    ///
    /// # Errors
    ///
    /// This method returns the same errors as [`Self::receive_signal`].
    pub async fn next_signal(&self) -> Result<Option<Arc<Message>>> {
        let mut stream = self.msg_stream().await.lock().await;
        let msg = stream
            .next()
            .await
            .ok_or_else(|| Error::Io(io::Error::new(ErrorKind::BrokenPipe, "socket closed")))??;

        if self.handle_signal(&msg).await? {
            Ok(None)
        } else {
            Ok(Some(msg))
        }
    }

    /// Handle the provided signal message.
    ///
    /// Call any handlers registered through the [`Self::connect_signal`] method for the provided
    /// signal message.
    ///
    /// If no errors are encountered, `Ok(true)` is returned if any handlers where found and called for,
    /// the signal; `Ok(false)` otherwise.
    pub async fn handle_signal(&self, msg: &Message) -> Result<bool> {
        let mut handlers = self.inner.sig_handlers.lock().await;
        if handlers.is_empty() {
            return Ok(false);
        }

        self.destination_unique_name().await?;
        let h = match msg.header() {
            Ok(h) => h,
            _ => return Ok(false),
        };
        let signal_name = match self.inner.matching_signal(msg, &h) {
            Some(signal) => signal,
            _ => return Ok(false),
        };

        let mut handled = false;
        for info in handlers
            .values_mut()
            .filter(|info| info.signal_name == signal_name)
        {
            (*info.handler)(msg).await?;
            handled = true;
        }

        Ok(handled)
    }

    /// Resolves the destination name to the associated unique connection name.
    ///
    /// Typically you would want to create the [`Proxy`] with the well-known name of the destination
    /// service but signal messages only specify the unique name of the peer (except for signals
    /// from `org.freedesktop.DBus` service). This means we have no means to check the sender of
    /// the message. While in most cases this will not be a problem, it becomes a problem if you
    /// need to communicate with multiple services exposing the same interface, over the same
    /// connection. Hence the need for this method.
    pub(crate) async fn destination_unique_name(&self) -> Result<&str> {
        if let Some(name) = self.inner.dest_unique_name.get() {
            // Already resolved the name.
            return Ok(name);
        }

        let destination = &self.inner.destination;
        let unique_name = if destination.starts_with(':') || destination == "org.freedesktop.DBus" {
            destination.to_string()
        } else {
            fdo::AsyncDBusProxy::new(&self.inner.conn)
                .await?
                .get_name_owner(destination)
                .await?
        };
        self.inner
            .dest_unique_name
            .set(unique_name)
            // programmer (probably our) error if this fails.
            .expect("Attempted to set dest_unique_name twice");

        Ok(self.inner.dest_unique_name.get().unwrap())
    }

    async fn msg_stream(&self) -> &Mutex<Connection> {
        match self.inner.signal_msg_stream.get() {
            Some(stream) => stream,
            None => {
                let stream = self.inner.conn.clone();
                self.inner
                    .signal_msg_stream
                    .set(Mutex::new(stream))
                    .unwrap_or_else(|_| panic!("Attempted to set stream twice"));

                // Safety: We just set it in the previous line.
                self.inner
                    .signal_msg_stream
                    .get()
                    .expect("message stream not set")
            }
        }
    }

    /// Get a stream to receive property changed events.
    ///
    /// Note that zbus doesn't queue the updates. If the listener is slower than the receiver, it
    /// will only receive the last update.
    pub async fn receive_property_stream<'n, T>(&self, name: &'n str) -> PropertyStream<'n, T> {
        PropertyStream {
            name,
            stream: self.properties.receiver.activate_cloned().boxed(),
            phantom: std::marker::PhantomData,
        }
    }
}

/// A [`stream::Stream`] implementation that yields signal [messages](`Message`).
///
/// Use [`Proxy::receive_signal`] to create an instance of this type.
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct SignalStream<'s> {
    #[derivative(Debug = "ignore")]
    stream: stream::BoxStream<'s, Arc<Message>>,
    conn: Connection,
    subscription_id: Option<u64>,
}

assert_impl_all!(SignalStream<'_>: Send, Unpin);

impl stream::Stream for SignalStream<'_> {
    type Item = Arc<Message>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        stream::Stream::poll_next(self.get_mut().stream.as_mut(), cx)
    }
}

impl std::ops::Drop for SignalStream<'_> {
    fn drop(&mut self) {
        if let Some(id) = self.subscription_id.take() {
            self.conn.queue_unsubscribe_signal(id);
        }
    }
}

impl<'a> From<crate::Proxy<'a>> for Proxy<'a> {
    fn from(proxy: crate::Proxy<'a>) -> Self {
        proxy.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_io::block_on;
    use futures_util::future::FutureExt;
    use ntest::timeout;
    use std::{future::ready, sync::Arc};
    use test_env_log::test;

    #[test]
    #[timeout(1000)]
    fn signal_stream() {
        block_on(test_signal_stream()).unwrap();
    }

    async fn test_signal_stream() -> Result<()> {
        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let conn = Connection::new_session().await?;
        let unique_name = conn.unique_name().unwrap();

        let proxy = fdo::AsyncDBusProxy::new(&conn).await?;

        let well_known = "org.freedesktop.zbus.async.ProxySignalStreamTest";
        let owner_changed_stream = proxy
            .receive_signal("NameOwnerChanged")
            .await?
            .filter(|msg| {
                if let Ok((name, _, new_owner)) = msg.body::<(&str, &str, &str)>() {
                    return ready(new_owner == unique_name && name == well_known);
                }

                ready(false)
            });

        let name_acquired_stream = proxy.receive_signal("NameAcquired").await?.filter(|msg| {
            if let Ok(name) = msg.body::<&str>() {
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
            .request_name(well_known, fdo::RequestNameFlags::ReplaceExisting.into())
            .await?;
        assert_eq!(reply, fdo::RequestNameReply::PrimaryOwner);

        let (changed_signal, acquired_signal) = futures_util::join!(
            owner_changed_stream.into_future(),
            name_acquired_stream.into_future(),
        );

        let changed_signal = changed_signal.0.unwrap();
        let (acquired_name, _, new_owner) = changed_signal.body::<(&str, &str, &str)>().unwrap();
        assert_eq!(acquired_name, well_known);
        assert_eq!(new_owner, unique_name);

        let acquired_signal = acquired_signal.0.unwrap();
        assert_eq!(acquired_signal.body::<&str>().unwrap(), well_known);

        Ok(())
    }

    #[test]
    #[timeout(1000)]
    fn signal_connect() {
        block_on(test_signal_connect()).unwrap();
    }

    async fn test_signal_connect() -> Result<()> {
        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let conn = Connection::new_session().await?;
        let owner_change_signaled = Arc::new(Mutex::new(false));
        let name_acquired_signaled = Arc::new(Mutex::new(false));
        let name_acquired_signaled2 = Arc::new(Mutex::new(false));

        let proxy = fdo::AsyncDBusProxy::new(&conn).await?;
        let well_known = "org.freedesktop.zbus.async.ProxySignalConnectTest";
        let unique_name = conn.unique_name().unwrap().to_string();
        let name_owner_changed_id = {
            let signaled = owner_change_signaled.clone();

            proxy
                .connect_signal("NameOwnerChanged", move |m| {
                    let signaled = signaled.clone();
                    let unique_name = unique_name.clone();

                    async move {
                        let (name, _, new_owner) = m.body::<(&str, &str, &str)>()?;
                        if name != well_known {
                            // Meant for the other testcase then
                            return Ok(());
                        }
                        assert_eq!(new_owner, unique_name);
                        *signaled.lock().await = true;

                        Ok(())
                    }
                    .boxed()
                })
                .await?
        };
        let name_acquired_id = {
            let signaled = name_acquired_signaled.clone();
            // `NameAcquired` is emitted twice, first when the unique name is assigned on
            // connection and secondly after we ask for a specific name.
            proxy
                .connect_signal("NameAcquired", move |m| {
                    let signaled = signaled.clone();

                    async move {
                        if m.body::<&str>()? == well_known {
                            *signaled.lock().await = true;
                        }

                        Ok(())
                    }
                    .boxed()
                })
                .await?
        };
        // Test multiple handers for the same signal
        let name_acquired_id2 = {
            let signaled = name_acquired_signaled2.clone();
            // `NameAcquired` is emitted twice, first when the unique name is assigned on
            // connection and secondly after we ask for a specific name.
            proxy
                .connect_signal("NameAcquired", move |m| {
                    let signaled = signaled.clone();

                    async move {
                        if m.body::<&str>()? == well_known {
                            *signaled.lock().await = true;
                        }

                        Ok(())
                    }
                    .boxed()
                })
                .await?
        };

        fdo::DBusProxy::new(&crate::Connection::from(conn))?
            .request_name(well_known, fdo::RequestNameFlags::ReplaceExisting.into())
            .unwrap();

        loop {
            proxy.next_signal().await?;

            if *owner_change_signaled.lock().await
                && *name_acquired_signaled.lock().await
                && *name_acquired_signaled2.lock().await
            {
                break;
            }
        }

        assert!(proxy.disconnect_signal(name_owner_changed_id).await?);
        assert!(!proxy.disconnect_signal(name_owner_changed_id).await?);
        assert!(proxy.disconnect_signal(name_acquired_id).await?);
        assert!(proxy.disconnect_signal(name_acquired_id2).await?);

        Ok(())
    }
}
