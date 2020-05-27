use std::convert::{TryFrom, TryInto};
use zvariant::{OwnedValue, Value};

use crate::{Connection, Error, Result};

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

    pub fn try_get<T>(&self, property_name: &str) -> Result<T>
    where
        T: TryFrom<OwnedValue>,
    {
        PropertiesProxy::new_for(self.conn, self.destination, self.path)?
            .get(self.interface, property_name)?
            .try_into()
            .map_err(|_| Error::InvalidReply)
    }

    pub fn try_set<'t, T: 't>(&self, property_name: &str, value: T) -> Result<()>
    where
        T: Into<Value<'t>>,
    {
        PropertiesProxy::new_for(self.conn, self.destination, self.path)?.set(
            self.interface,
            property_name,
            &value.into(),
        )
    }

    pub fn call<B, R>(&self, method_name: &str, body: &B) -> Result<R>
    where
        B: serde::ser::Serialize + zvariant::Type,
        R: serde::de::DeserializeOwned + zvariant::Type,
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
                let r = reply.body()?;
                reply.disown_fds();
                Ok(r)
            }
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Proxy;
    use crate::{Connection, Result};

    #[test]
    fn basic() {
        let c = Connection::new_session().unwrap();
        let p = Proxy::new(
            &c,
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus.Peer",
        )
        .unwrap();
        let _id: Result<String> = p.call("GetMachineId", &());
    }
}
