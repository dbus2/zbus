use async_lock::Mutex;
use futures_core::{future::BoxFuture, stream};
use futures_util::{
    future::FutureExt,
    stream::{unfold, StreamExt},
};
use slotmap::{new_key_type, SlotMap};
use std::{
    borrow::Cow,
    convert::{TryFrom, TryInto},
    future::ready,
    pin::Pin,
    task::{Context, Poll},
};

use async_io::block_on;
use zvariant::{IntoObjectPath, ObjectPath, OwnedValue, Value};

use crate::{azync::Connection, Error, Message, Result};

use crate::fdo::{self, AsyncIntrospectableProxy, AsyncPropertiesProxy};

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
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Proxy<'a> {
    core: ProxyCore<'a>,
    #[derivative(Debug = "ignore")]
    sig_handlers: Mutex<SlotMap<SignalHandlerId, SignalHandlerInfo>>,
}

#[derive(Clone, Debug)]
struct ProxyCore<'a> {
    conn: Connection,
    destination: Cow<'a, str>,
    path: ObjectPath<'a>,
    interface: Cow<'a, str>,
}

impl<'a> Proxy<'a> {
    /// Create a new `Proxy` for the given destination/path/interface.
    pub fn new(
        conn: &Connection,
        destination: &'a str,
        path: impl IntoObjectPath<'a>,
        interface: &'a str,
    ) -> Result<Self> {
        Ok(Self {
            core: ProxyCore {
                conn: conn.clone(),
                destination: Cow::from(destination),
                path: path.into_object_path()?,
                interface: Cow::from(interface),
            },
            sig_handlers: Mutex::new(SlotMap::with_key()),
        })
    }

    /// Create a new `Proxy` for the given destination/path/interface, taking ownership of all
    /// passed arguments.
    pub fn new_owned(
        conn: Connection,
        destination: String,
        path: impl IntoObjectPath<'static>,
        interface: String,
    ) -> Result<Self> {
        Ok(Self {
            core: ProxyCore {
                conn,
                destination: Cow::from(destination),
                path: path.into_object_path()?,
                interface: Cow::from(interface),
            },
            sig_handlers: Mutex::new(SlotMap::with_key()),
        })
    }

    /// Get a reference to the associated connection.
    pub fn connection(&self) -> &Connection {
        &self.core.conn
    }

    /// Get a reference to the destination service name.
    pub fn destination(&self) -> &str {
        &self.core.destination
    }

    /// Get a reference to the object path.
    pub fn path(&self) -> &ObjectPath<'_> {
        &self.core.path
    }

    /// Get a reference to the interface.
    pub fn interface(&self) -> &str {
        &self.core.interface
    }

    /// Introspect the associated object, and return the XML description.
    ///
    /// See the [xml](xml/index.html) module for parsing the result.
    pub async fn introspect(&self) -> fdo::Result<String> {
        AsyncIntrospectableProxy::new_for(&self.core.conn, &self.core.destination, &self.core.path)?
            .introspect()
            .await
    }

    /// Get the property `property_name`.
    ///
    /// Effectively, call the `Get` method of the `org.freedesktop.DBus.Properties` interface.
    pub async fn get_property<T>(&self, property_name: &str) -> fdo::Result<T>
    where
        T: TryFrom<OwnedValue>,
    {
        AsyncPropertiesProxy::new_for(&self.core.conn, &self.core.destination, &self.core.path)?
            .get(&self.core.interface, property_name)
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
        AsyncPropertiesProxy::new_for(&self.core.conn, &self.core.destination, &self.core.path)?
            .set(&self.core.interface, property_name, &value.into())
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
        let reply = self
            .core
            .conn
            .call_method(
                Some(&self.core.destination),
                self.core.path.as_str(),
                Some(&self.core.interface),
                method_name,
                body,
            )
            .await;
        match reply {
            Ok(mut reply) => {
                reply.disown_fds();

                Ok(reply)
            }
            Err(e) => Err(e),
        }
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
        Ok(self.call_method(method_name, body).await?.body()?)
    }

    /// Create a stream for signal named `signal_name`.
    ///
    /// If the associated connnection is to a bus, a match rule is added for the signal on the bus
    /// so that the bus sends us the signals. Since this match rule needs to be removed when you're
    /// done with the stream, a synchronous D-Bus method call is made in the destructor of the
    /// stream. If you'd like to avoid this, you must close the stream explicitly, using the
    /// [`SignalStream::close`] method.
    pub async fn receive_signal(&self, signal_name: &'static str) -> Result<SignalStream<'a>> {
        let subscription_id = if self.core.conn.is_bus() {
            let id = self
                .core
                .conn
                .subscribe_signal(
                    self.destination(),
                    self.path(),
                    self.interface(),
                    signal_name,
                )
                .await?;

            Some(id)
        } else {
            None
        };

        let proxy = self.core.clone();
        let stream = unfold((proxy, signal_name), |(proxy, signal_name)| async move {
            proxy
                .conn
                .receive_specific(|msg| {
                    let hdr = match msg.header() {
                        Ok(hdr) => hdr,
                        Err(_) => return ready(Ok(false)).boxed(),
                    };

                    ready(Ok(hdr.primary().msg_type() == crate::MessageType::Signal
                        && hdr.interface() == Ok(Some(&proxy.interface))
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
            conn: self.core.conn.clone(),
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
        let id = self.sig_handlers.lock().await.insert(SignalHandlerInfo {
            signal_name,
            handler: Box::new(handler),
        });

        if self.core.conn.is_bus() {
            let _ = self
                .core
                .conn
                .subscribe_signal(
                    self.destination(),
                    self.path(),
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
        match self.sig_handlers.lock().await.remove(handler_id) {
            Some(handler_info) => {
                if self.core.conn.is_bus() {
                    let _ = self
                        .core
                        .conn
                        .unsubscribe_signal(
                            self.destination(),
                            self.path(),
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
    pub async fn next_signal(&self) -> Result<Option<Message>> {
        let msg = {
            // We want to keep a lock on the handlers during `receive_specific` call but we also
            // want to avoid using `handlers` directly as that somehow makes this call (or rather
            // the future of this call) not `Sync` and we get a very scary error message from
            // the compiler on using `next_signal` with `tokio::select` inside a tokio task.
            let handlers = self.sig_handlers.lock().await;
            let signals: Vec<&str> = handlers.values().map(|info| info.signal_name).collect();

            self.core
                .conn
                .receive_specific(move |msg| {
                    let ret = match msg.header() {
                        Err(_) => false,
                        Ok(hdr) => match hdr.member() {
                            Ok(None) | Err(_) => false,
                            Ok(Some(member)) => {
                                hdr.interface() == Ok(Some(self.interface()))
                                    && hdr.path() == Ok(Some(self.path()))
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
        let mut handlers = self.sig_handlers.lock().await;
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
        self.sig_handlers
            .lock()
            .await
            .values()
            .any(|info| info.signal_name == signal_name)
    }
}

/// A [`stream::Stream`] implementation that yields signal [messages](`Message`).
///
/// Use [`Proxy::receive_signal`] to create an instance of this type.
pub struct SignalStream<'s> {
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
    use enumflags2::BitFlags;
    use futures_util::future::FutureExt;
    use std::{future::ready, sync::Arc};

    #[test]
    fn signal_stream() {
        block_on(test_signal_stream()).unwrap();
    }

    async fn test_signal_stream() -> Result<()> {
        // Register a well-known name with the session bus and ensure we get the appropriate
        // signals called for that.
        let conn = Connection::new_session().await?;
        let unique_name = conn.unique_name().unwrap();

        let proxy = Proxy::new(
            &conn,
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
        )
        .unwrap();

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

        // TODO: Use fdo API when it has async proxy
        let reply = proxy
            .call_method(
                "RequestName",
                &(
                    well_known,
                    BitFlags::from(fdo::RequestNameFlags::ReplaceExisting),
                ),
            )
            .await
            .unwrap();
        let reply: fdo::RequestNameReply = reply.body().unwrap();
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

        let proxy = Proxy::new(
            &conn,
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
        )?;

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
            .unwrap()
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
