use std::{
    any::{Any, TypeId},
    fmt::Write,
    future::Future,
    pin::Pin,
    sync::Arc,
};

use async_io::block_on;
use zbus_names::{InterfaceName, MemberName};
use zvariant::{DynamicType, Value};

use crate::{Connection, Message, ObjectServer, Result, SignalContext};

/// A helper type returned by [`Interface`] callbacks.
pub enum DispatchResult<'a> {
    /// This interface does not support the given method
    MethodNotFound,

    /// Retry with [Interface::call_mut].
    ///
    /// This is equivalent to MethodNotFound if returned by call_mut.
    RequiresMut,

    /// The method was found and will be completed by running this Future
    Async(Pin<Box<dyn Future<Output = Result<u32>> + Send + 'a>>),

    /// The method was found and will be completed by running this closure, which may call blocking
    /// APIs.
    Blocking(Box<dyn FnOnce() -> Result<u32> + Send + 'a>),
}

impl<'a> DispatchResult<'a> {
    /// Helper for creating the Async variant
    pub fn new_async<F, T, E>(conn: &'a Connection, msg: &'a Message, f: F) -> Self
    where
        F: Future<Output = ::std::result::Result<T, E>> + Send + 'a,
        T: serde::Serialize + DynamicType + Send + Sync,
        E: zbus::DBusError + Send,
    {
        DispatchResult::Async(Box::pin(async move {
            let hdr = msg.header()?;
            match f.await {
                Ok(r) => conn.reply(msg, &r).await,
                Err(e) => conn.reply_dbus_error(&hdr, e).await,
            }
        }))
    }

    /// Helper for creating the Blocking variant
    pub fn new_blocking<F, T, E>(conn: &'a Connection, msg: &'a Message, f: F) -> Self
    where
        F: FnOnce() -> ::std::result::Result<T, E> + Send + 'a,
        T: serde::Serialize + DynamicType + Send + Sync,
        E: zbus::DBusError + Send,
    {
        DispatchResult::Blocking(Box::new(move || {
            let hdr = msg.header()?;
            match f() {
                Ok(r) => block_on(conn.reply(msg, &r)),
                Err(e) => block_on(conn.reply_dbus_error(&hdr, e)),
            }
        }))
    }
}

/// The trait used to dispatch messages to an interface instance.
///
/// Note: It is not recommended to manually implement this trait. The [`dbus_interface`] macro
/// implements it for you.
///
/// [`dbus_interface`]: attr.dbus_interface.html
pub trait Interface: Any + Send + Sync {
    /// Return the name of the interface. Ex: "org.foo.MyInterface"
    fn name() -> InterfaceName<'static>
    where
        Self: Sized;

    /// Get a property value.
    fn get<'call>(
        &'call self,
        server: &'call ObjectServer,
        connection: &'call Connection,
        msg: &'call Arc<Message>,
        property_name: &'call str,
        allow_blocking: bool,
    ) -> DispatchResult<'call>;

    /// Return all the properties.
    fn get_all<'call>(
        &'call self,
        server: &'call ObjectServer,
        connection: &'call Connection,
        msg: &'call Arc<Message>,
        allow_blocking: bool,
    ) -> DispatchResult<'call>;

    /// Set a property value.
    #[allow(clippy::too_many_arguments)]
    fn set<'call>(
        &'call mut self,
        server: &'call ObjectServer,
        connection: &'call Connection,
        msg: &'call Arc<Message>,
        property_name: &'call str,
        value: &'call Value<'_>,
        ctxt: &'call SignalContext<'_>,
        allow_blocking: bool,
    ) -> DispatchResult<'call>;

    /// Call a `&self` method.
    fn call<'call>(
        &'call self,
        server: &'call ObjectServer,
        connection: &'call Connection,
        msg: &'call Arc<Message>,
        name: MemberName<'call>,
        allow_blocking: bool,
    ) -> DispatchResult<'call>;

    /// Call a `&mut self` method.
    fn call_mut<'call>(
        &'call mut self,
        server: &'call ObjectServer,
        connection: &'call Connection,
        msg: &'call Arc<Message>,
        name: MemberName<'call>,
        allow_blocking: bool,
    ) -> DispatchResult<'call>;

    /// Write introspection XML to the writer, with the given indentation level.
    fn introspect_to_writer(&self, writer: &mut dyn Write, level: usize);
}

// FIXME: Do we really need these unsafe implementations? If so, can't they be implemented w/o
///       `unsafe` usage?
impl dyn Interface {
    /// Return Any of self
    pub(crate) fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if <dyn Interface as Any>::type_id(self) == TypeId::of::<T>() {
            // SAFETY: If type ID matches, it means object is of type T
            Some(unsafe { &*(self as *const dyn Interface as *const T) })
        } else {
            None
        }
    }

    /// Return Any of self
    pub(crate) fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if <dyn Interface as Any>::type_id(self) == TypeId::of::<T>() {
            // SAFETY: If type ID matches, it means object is of type T
            Some(unsafe { &mut *(self as *mut dyn Interface as *mut T) })
        } else {
            None
        }
    }
}
