use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Write,
};

use async_trait::async_trait;
use zbus_names::{InterfaceName, MemberName};
use zvariant::{OwnedValue, Value};

use crate::{
    azync::{Connection, ObjectServer, SignalContext},
    fdo, Message, Result,
};

/// The trait used to dispatch messages to an interface instance.
///
/// Note: It is not recommended to manually implement this trait. The [`dbus_interface`] macro
/// implements it for you.
///
/// [`dbus_interface`]: attr.dbus_interface.html
#[async_trait]
pub trait Interface: Any + Send + Sync {
    /// Return the name of the interface. Ex: "org.foo.MyInterface"
    fn name() -> InterfaceName<'static>
    where
        Self: Sized;

    /// Get a property value. Returns `None` if the property doesn't exist.
    async fn get(&self, property_name: &str) -> Option<fdo::Result<OwnedValue>>;

    /// Return all the properties.
    async fn get_all(&self) -> HashMap<String, OwnedValue>;

    /// Set a property value. Returns `None` if the property doesn't exist.
    async fn set(
        &mut self,
        property_name: &str,
        value: &Value<'_>,
        ctxt: &SignalContext<'_>,
    ) -> Option<fdo::Result<()>>;

    /// Call a `&self` method. Returns `None` if the method doesn't exist.
    async fn call(
        &self,
        server: &ObjectServer,
        connection: &Connection,
        msg: &Message,
        name: MemberName<'_>,
    ) -> Option<Result<u32>>;

    /// Call a `&mut self` method. Returns `None` if the method doesn't exist.
    async fn call_mut(
        &mut self,
        server: &ObjectServer,
        connection: &Connection,
        msg: &Message,
        name: MemberName<'_>,
    ) -> Option<Result<u32>>;

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
