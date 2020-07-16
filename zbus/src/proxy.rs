use std::convert::{TryFrom, TryInto};
use zvariant::{OwnedValue, Value};

use crate::{Connection, Error, Message, Result};

use crate::fdo::{IntrospectableProxy, PropertiesProxy};

pub struct Proxy<'a> {
    conn: &'a Connection,
    destination: &'a str,
    path: &'a str,
    interface: &'a str,
}

impl<'a> Proxy<'a> {
    pub fn new(
        conn: &'a Connection,
        destination: &'a str,
        path: &'a str,
        interface: &'a str,
    ) -> Result<Self> {
        Ok(Self {
            conn,
            destination,
            path,
            interface,
        })
    }

    pub fn introspect(&self) -> Result<String> {
        IntrospectableProxy::new_for(self.conn, self.destination, self.path)?.introspect()
    }

    pub fn get_property<T>(&self, property_name: &str) -> Result<T>
    where
        T: TryFrom<OwnedValue>,
    {
        PropertiesProxy::new_for(self.conn, self.destination, self.path)?
            .get(self.interface, property_name)?
            .try_into()
            .map_err(|_| Error::InvalidReply)
    }

    pub fn set_property<'t, T: 't>(&self, property_name: &str, value: T) -> Result<()>
    where
        T: Into<Value<'t>>,
    {
        PropertiesProxy::new_for(self.conn, self.destination, self.path)?.set(
            self.interface,
            property_name,
            &value.into(),
        )
    }

    /// Call a method and return the response.
    ///
    /// Typically, you would want to use [`call`] method instead. Use this method if you need to
    /// parse the response manually. Typical use case would be avoid avoid memory
    /// allocations/copying, by parsing the response to an unowned type.
    ///
    /// [`call`]: struct.Proxy.html#method.call
    pub fn call_method<B>(&self, method_name: &str, body: &B) -> Result<Message>
    where
        B: serde::ser::Serialize + zvariant::Type,
    {
        let reply = self.conn.call_method(
            Some(self.destination),
            self.path,
            Some(self.interface),
            method_name,
            body,
        );
        match reply {
            Ok(mut reply) => {
                reply.disown_fds();

                Ok(reply)
            }
            Err(e) => Err(e),
        }
    }

    /// Call a method and parse the reponse.
    ///
    /// Use [`call_method`] instead if you need to parse the response manually/separately.
    ///
    /// [`call_method`]: struct.Proxy.html#method.call_method
    pub fn call<B, R>(&self, method_name: &str, body: &B) -> Result<R>
    where
        B: serde::ser::Serialize + zvariant::Type,
        R: serde::de::DeserializeOwned + zvariant::Type,
    {
        Ok(self.call_method(method_name, body)?.body()?)
    }
}

#[cfg(test)]
mod tests {
    use super::Proxy;
    use crate::Connection;

    #[test]
    fn basic() {
        let c = Connection::new_session().unwrap();
        let p = Proxy::new(
            &c,
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
        )
        .unwrap();
        let _id: &str = p.call_method("GetId", &()).unwrap().body().unwrap();
        let _owned_id: String = p.call("GetId", &()).unwrap();
    }
}
