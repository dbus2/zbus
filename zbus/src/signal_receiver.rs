use crate::{Connection, Error, MessageHeader, Proxy, Result};
use static_assertions::assert_impl_all;
use std::{
    collections::HashMap,
    convert::{AsRef, TryFrom},
    sync::Arc,
};
use zbus_names::{InterfaceName, UniqueName};
use zvariant::ObjectPath;

#[derive(Hash, Eq, PartialEq)]
struct ProxyKey<'key> {
    interface: InterfaceName<'key>,
    destination: UniqueName<'key>,
    path: ObjectPath<'key>,
}

impl<'p, P> From<&P> for ProxyKey<'static>
where
    P: AsRef<Proxy<'p>>,
{
    fn from(proxy: &P) -> Self {
        let proxy = proxy.as_ref();
        ProxyKey {
            interface: proxy.interface().to_owned(),
            path: proxy.path().to_owned(),
            // SAFETY: we ensure proxy's name is resolved before creating a key for it, in
            // `SignalReceiver::receive_for` method.
            destination: proxy
                .destination_unique_name()
                .expect("Destination unique name")
                .clone()
                .into(),
        }
    }
}

impl<'key> TryFrom<&'key MessageHeader<'key>> for ProxyKey<'key> {
    type Error = Error;

    fn try_from(hdr: &'key MessageHeader<'key>) -> Result<Self> {
        match (
            hdr.interface()?.cloned(),
            hdr.path()?.cloned(),
            hdr.sender()?.cloned(),
        ) {
            (Some(interface), Some(path), Some(destination)) => Ok(ProxyKey {
                interface,
                destination,
                path,
            }),
            (_, _, _) => Err(Error::MissingField),
        }
    }
}

/// Receives signals for [`Proxy`] instances.
///
/// Use this to receive signals on a given connection for a bunch of proxies at the same time.
pub struct SignalReceiver<'a> {
    conn: Connection,
    proxies: HashMap<ProxyKey<'static>, &'a Proxy<'a>>,
}

assert_impl_all!(SignalReceiver<'_>: Send, Sync, Unpin);

impl<'a> SignalReceiver<'a> {
    /// Create a new `SignalReceiver` instance.
    pub fn new(conn: Connection) -> Self {
        Self {
            conn,
            proxies: HashMap::new(),
        }
    }

    /// Get a reference to the associated connection.
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Get a iterator for all the proxies in this receiver.
    pub fn proxies(&self) -> impl Iterator<Item = &&Proxy<'a>> {
        self.proxies.values()
    }

    /// Watch for signals relevant to the `proxy`.
    ///
    /// # Panics
    ///
    /// This method will panic if you try to add a proxy with a different associated connection than
    /// the one associated with this receiver.
    pub fn receive_for<P>(&mut self, proxy: &'a P) -> Result<()>
    where
        P: AsRef<Proxy<'a>>,
    {
        let proxy = proxy.as_ref();
        assert_eq!(proxy.connection().unique_name(), self.conn.unique_name());

        let key = ProxyKey::from(proxy);
        self.proxies.insert(key, proxy);

        Ok(())
    }

    /// Received and handle the next incoming signal on the associated connection.
    ///
    /// This method will wait for signal messages on the associated connection and call any
    /// handler registered (through [`Proxy::connect_signal`]) with a proxy in this receiver.
    ///
    /// If the signal message was handled by a handler, `Ok(None)` is returned. Otherwise, the
    /// received message is returned.
    pub fn next_signal(&self) -> Result<Option<Arc<crate::Message>>> {
        let msg = self.conn.receive_message()?;

        if self.handle_signal(&msg)? {
            Ok(None)
        } else {
            Ok(Some(msg))
        }
    }

    /// Handle the provided signal message.
    ///
    /// Call any handler registered (through [`Proxy::connect_signal`]) with a proxy in this receiver.
    ///
    /// If no errors are encountered, `Ok(true)` is returned if a handler was found and called for,
    /// the signal; `Ok(false)` otherwise.
    pub fn handle_signal(&self, msg: &crate::Message) -> Result<bool> {
        let hdr = msg.header()?;

        if let Ok(key) = ProxyKey::try_from(&hdr) {
            if let Some(proxy) = self.proxies.get(&key) {
                return proxy.handle_signal(msg);
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dbus_interface, dbus_proxy};
    use std::sync::{Arc, Mutex};
    use test_env_log::test;

    fn multiple_signal_iface_test() -> std::result::Result<u32, Box<dyn std::error::Error>> {
        #[dbus_proxy(interface = "org.freedesktop.zbus.MultiSignal")]
        trait MultiSignal {
            #[dbus_proxy(signal)]
            fn some_signal(&self, sig_arg: &str) -> Result<()>;

            fn emit_it(&self, arg: &str) -> Result<()>;
        }
        let conn = Connection::session()?;
        let mut receiver = SignalReceiver::new(conn.clone());

        let proxy1 = MultiSignalProxy::builder(&conn)
            .destination("org.freedesktop.zbus.MultiSignal")?
            .path("/org/freedesktop/zbus/MultiSignal/1")?
            .build()?;
        let proxy1_str = Arc::new(Mutex::new(None));
        let clone = proxy1_str.clone();
        println!("1");
        proxy1.connect_some_signal(move |s| {
            *clone.lock().unwrap() = Some(s.to_string());

            Ok(())
        })?;
        println!("2");
        receiver.receive_for(&proxy1)?;

        println!("3");
        let proxy2 = MultiSignalProxy::builder(&conn)
            .destination("org.freedesktop.zbus.MultiSignal")?
            .path("/org/freedesktop/zbus/MultiSignal/2")?
            .build()?;
        let proxy2_str = Arc::new(Mutex::new(None));
        let clone = proxy2_str.clone();
        proxy2.connect_some_signal(move |s| {
            *clone.lock().unwrap() = Some(s.to_string());

            Ok(())
        })?;
        receiver.receive_for(&proxy2)?;

        proxy1.emit_it("hi")?;
        proxy2.emit_it("bye")?;

        loop {
            receiver.next_signal()?;
            if proxy1_str.lock().unwrap().is_some() && proxy2_str.lock().unwrap().is_some() {
                break;
            }
        }

        Ok(99)
    }

    #[test]
    #[ntest::timeout(15000)]
    fn multiple_proxy_signals() {
        struct MultiSignal {
            times_called: Arc<Mutex<u8>>,
        }

        #[dbus_interface(interface = "org.freedesktop.zbus.MultiSignal")]
        impl MultiSignal {
            #[dbus_interface(signal)]
            fn some_signal(&self, sig_arg: &str) -> Result<()>;

            fn emit_it(&mut self, arg: &str) -> Result<()> {
                *self.times_called.lock().unwrap() += 1;
                self.some_signal(arg)
            }
        }

        let conn = Connection::session().unwrap();
        let mut object_server = crate::ObjectServer::new(&conn)
            .request_name("org.freedesktop.zbus.MultiSignal")
            .unwrap();
        let times_called = Arc::new(Mutex::new(0));
        let iface = MultiSignal {
            times_called: times_called.clone(),
        };
        object_server
            .at("/org/freedesktop/zbus/MultiSignal/1", iface)
            .unwrap();
        let iface = MultiSignal {
            times_called: times_called.clone(),
        };
        object_server
            .at("/org/freedesktop/zbus/MultiSignal/2", iface)
            .unwrap();

        let child = std::thread::spawn(|| multiple_signal_iface_test().unwrap());

        while *times_called.lock().unwrap() < 2 {
            object_server.try_handle_next().unwrap();
        }

        let val = child.join().expect("failed to join");
        assert_eq!(val, 99);
    }
}
