use async_lock::Mutex;
use futures_core::{future::BoxFuture, stream};
use futures_util::{
    future::FutureExt,
    stream::{unfold, StreamExt},
};
use once_cell::sync::OnceCell;
use slotmap::{new_key_type, SlotMap};
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    future::ready,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use async_io::block_on;
use zvariant::{ObjectPath, OwnedValue, Value};

use crate::{
    azync::Connection,
    fdo::{self, AsyncIntrospectableProxy, AsyncPropertiesProxy},
    Error, Message, Result,
};

type SignalHandler = Box<dyn for<'msg> FnMut(&'msg Message) -> BoxFuture<'msg, Result<()>> + Send>;

new_key_type! {
    /// The ID for a registered signal handler.
    pub struct SignalHandlerId;
}

struct SignalHandlerInfo {
    signal_name: &'static str,
    handler: SignalHandler,
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
///     )?;
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
#[derive(Debug)]
pub struct Proxy<'a> {
    pub(crate) inner: Arc<ProxyInner<'a>>,
}

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
        }
    }
}

impl<'a> Proxy<'a> {
    /// Create a new `Proxy` for the given destination/path/interface.
    pub fn new<E>(
        conn: &Connection,
        destination: &'a str,
        path: impl TryInto<ObjectPath<'a>, Error = E>,
        interface: &'a str,
    ) -> Result<Self>
    where
        Error: From<E>,
    {
        Ok(crate::ProxyBuilder::new_bare(conn)
            .destination(destination)
            .path(path)?
            .interface(interface)
            .build_bare_async())
    }

    /// Create a new `Proxy` for the given destination/path/interface, taking ownership of all
    /// passed arguments.
    pub fn new_owned<E>(
        conn: Connection,
        destination: String,
        path: impl TryInto<ObjectPath<'static>, Error = E>,
        interface: String,
    ) -> Result<Self>
    where
        Error: From<E>,
    {
        Ok(crate::ProxyBuilder::new_bare(&conn)
            .destination(destination)
            .path(path)?
            .interface(interface)
            .build_bare_async())
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
            .build();

        proxy.introspect().await
    }

    /// Get the property `property_name`.
    ///
    /// Effectively, call the `Get` method of the `org.freedesktop.DBus.Properties` interface.
    pub async fn get_property<T>(&self, property_name: &str) -> fdo::Result<T>
    where
        T: TryFrom<OwnedValue>,
    {
        let proxy = AsyncPropertiesProxy::builder(&self.inner.conn)
            .destination(self.inner.destination.as_ref())
            .path(&self.inner.path)?
            .build();

        proxy
            .get(&self.inner.interface, property_name)
            .await?
            .try_into()
            .map_err(|_| Error::InvalidReply.into())
    }

    /// Set the property `property_name`.
    ///
    /// Effectively, call the `Set` method of the `org.freedesktop.DBus.Properties` interface.
    pub async fn set_property<'t, T: 't>(&self, property_name: &str, value: T) -> fdo::Result<()>
    where
        T: Into<Value<'t>>,
    {
        let proxy = AsyncPropertiesProxy::builder(&self.inner.conn)
            .destination(self.inner.destination.as_ref())
            .path(&self.inner.path)?
            .build();

        proxy
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
    pub async fn call_method<B>(&self, method_name: &str, body: &B) -> Result<Message>
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
    /// If the associated connnection is to a bus, a match rule is added for the signal on the bus
    /// so that the bus sends us the signals. Since this match rule needs to be removed when you're
    /// done with the stream, a synchronous D-Bus method call is made in the destructor of the
    /// stream. If you'd like to avoid this, you must close the stream explicitly, using the
    /// [`SignalStream::close`] method.
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

        self.resolve_name().await?;

        let proxy = self.inner.clone();
        let stream = unfold((proxy, signal_name), |(proxy, signal_name)| async move {
            proxy
                .conn
                .receive_specific(|msg| {
                    let hdr = match msg.header() {
                        Ok(hdr) => hdr,
                        Err(_) => return ready(Ok(false)).boxed(),
                    };
                    let expected_sender = proxy.dest_unique_name.get().map(|s| s.as_str());

                    ready(Ok(hdr.primary().msg_type() == crate::MessageType::Signal
                        && hdr.interface() == Ok(Some(&proxy.interface))
                        && hdr.sender() == Ok(expected_sender)
                        && hdr.path() == Ok(Some(&proxy.path))
                        && hdr.member() == Ok(Some(signal_name))))
                    .boxed()
                })
                .await
                .ok()
                .map(|msg| (msg, (proxy, signal_name)))
        });

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
    /// safely `unwrap` the `Result` if you're certain that associated connnection is not a bus
    /// connection.
    pub async fn connect_signal<H>(
        &self,
        signal_name: &'static str,
        handler: H,
    ) -> fdo::Result<SignalHandlerId>
    where
        for<'msg> H: FnMut(&'msg Message) -> BoxFuture<'msg, Result<()>> + Send + 'static,
    {
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
    /// safely `unwrap` the `Result` if you're certain that associated connnection is not a bus
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
    pub async fn next_signal(&self) -> Result<Option<Message>> {
        let msg = {
            // We want to keep a lock on the handlers during `receive_specific` call but we also
            // want to avoid using `handlers` directly as that somehow makes this call (or rather
            // the future of this call) not `Sync` and we get a very scary error message from
            // the compiler on using `next_signal` with `tokio::select` inside a tokio task.
            let handlers = self.inner.sig_handlers.lock().await;
            let signals: Vec<&str> = handlers.values().map(|info| info.signal_name).collect();

            self.resolve_name().await?;

            self.inner
                .conn
                .receive_specific(move |msg| {
                    let ret = match msg.header() {
                        Err(_) => false,
                        Ok(hdr) => match hdr.member() {
                            Ok(None) | Err(_) => false,
                            Ok(Some(member)) => {
                                let expected_sender = self.destination_unique_name();

                                hdr.interface() == Ok(Some(self.interface()))
                                    && hdr.path() == Ok(Some(self.path()))
                                    && hdr.sender() == Ok(expected_sender)
                                    && hdr.message_type() == Ok(crate::MessageType::Signal)
                                    && signals.contains(&member)
                            }
                        },
                    };

                    ready(Ok(ret)).boxed()
                })
                .await?
        };

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

        let hdr = msg.header()?;
        if let Some(name) = hdr.member()? {
            let mut handled = false;

            for info in handlers
                .values_mut()
                .filter(|info| info.signal_name == name)
            {
                (*info.handler)(&msg).await?;

                if !handled {
                    handled = true;
                }
            }

            return Ok(handled);
        }

        Ok(false)
    }

    pub(crate) async fn has_signal_handler(&self, signal_name: &str) -> bool {
        self.inner
            .sig_handlers
            .lock()
            .await
            .values()
            .any(|info| info.signal_name == signal_name)
    }

    /// Resolve the destination name.
    ///
    /// Typically you would want to create the [`Proxy`] with the well-known name of the destination
    /// service but signal messages only specify the unique name of the peer (except for signals
    /// from `org.freedesktop.DBus` service). This means we have no means to check the sender of
    /// the message. While in most cases this will not be a problem, it becomes a problem if you
    /// need to commmunicate with multiple services exposing the same interface, over the same
    /// connection. Hence the need for this method.
    pub(crate) async fn resolve_name(&self) -> Result<()> {
        if self.inner.dest_unique_name.get().is_some() {
            // Already resolved the name.
            return Ok(());
        }

        let destination = &self.inner.destination;
        let unique_name = if destination.starts_with(':') || destination == "org.freedesktop.DBus" {
            destination.to_string()
        } else {
            fdo::AsyncDBusProxy::new(&self.inner.conn)
                .get_name_owner(destination)
                .await?
        };
        self.inner
            .dest_unique_name
            .set(unique_name)
            // programmer (probably our) error if this fails.
            .expect("Attempted to set dest_unique_name twice");

        Ok(())
    }

    pub(crate) fn destination_unique_name(&self) -> Option<&str> {
        self.inner.dest_unique_name.get().map(|s| s.as_str())
    }
}

/// A [`stream::Stream`] implementation that yields signal [messages](`Message`).
///
/// Use [`Proxy::receive_signal`] to create an instance of this type.
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct SignalStream<'s> {
    #[derivative(Debug = "ignore")]
    stream: stream::BoxStream<'s, Message>,
    conn: Connection,
    subscription_id: Option<u64>,
}

impl SignalStream<'_> {
    /// Close this stream.
    ///
    /// If not called explicitly on the stream, it is done for you but at the cost of synchronous
    /// D-Bus calls.
    pub async fn close(mut self) -> Result<()> {
        self.close_().await
    }

    async fn close_(&mut self) -> Result<()> {
        if let Some(id) = self.subscription_id.take() {
            let _ = self.conn.unsubscribe_signal_by_id(id).await?;
        }

        Ok(())
    }
}

impl stream::Stream for SignalStream<'_> {
    type Item = Message;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        stream::Stream::poll_next(self.get_mut().stream.as_mut(), cx)
    }
}

impl std::ops::Drop for SignalStream<'_> {
    fn drop(&mut self) {
        if self.subscription_id.is_some() {
            // User didn't close the stream explicitly so we've to do it synchronously ourselves.
            let _ = block_on(self.close_());
        }
    }
}

impl<'azync, 'sync: 'azync> From<crate::Proxy<'sync>> for Proxy<'azync> {
    fn from(proxy: crate::Proxy<'sync>) -> Self {
        proxy.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::future::FutureExt;
    use std::{future::ready, sync::Arc};
    use test_env_log::test;

    #[test]
    fn signal_stream() {
        block_on(test_signal_stream()).unwrap();
    }

    async fn test_signal_stream() -> Result<()> {
        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let conn = Connection::new_session().await?;
        let unique_name = conn.unique_name().unwrap();

        let proxy = fdo::AsyncDBusProxy::new(&conn);

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

        let reply = proxy
            .request_name(well_known, fdo::RequestNameFlags::ReplaceExisting.into())
            .await?;
        assert_eq!(reply, fdo::RequestNameReply::PrimaryOwner);

        let (changed_signal, acquired_signal) = futures_util::join!(
            owner_changed_stream.into_future(),
            name_acquired_stream.into_future()
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

        let proxy = fdo::AsyncDBusProxy::new(&conn);
        let well_known = "org.freedesktop.zbus.async.ProxySignalConnectTest";
        let unique_name = conn.unique_name().unwrap().to_string();
        let name_owner_changed_id = {
            let well_known = well_known.clone();
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

        fdo::DBusProxy::new(&crate::Connection::from(conn))
            .request_name(&well_known, fdo::RequestNameFlags::ReplaceExisting.into())
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

        assert_eq!(proxy.disconnect_signal(name_owner_changed_id).await?, true);
        assert_eq!(proxy.disconnect_signal(name_owner_changed_id).await?, false);
        assert_eq!(proxy.disconnect_signal(name_acquired_id).await?, true);
        assert_eq!(proxy.disconnect_signal(name_acquired_id2).await?, true);

        Ok(())
    }
}
