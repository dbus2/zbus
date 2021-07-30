use async_io::block_on;
use futures_util::{
    future::{select, Either},
    stream::StreamExt,
};
use std::{convert::TryInto, future::ready};
use zbus::fdo;
use zbus_macros::{dbus_interface, dbus_proxy, DBusError};

#[test]
fn test_proxy() {
    #[dbus_proxy(
        interface = "org.freedesktop.zbus_macros.ProxyParam",
        default_service = "org.freedesktop.zbus_macros",
        default_path = "/org/freedesktop/zbus_macros/test"
    )]
    trait ProxyParam {
        #[dbus_proxy(object = "Test")]
        fn some_method<T>(&self, test: &T);
    }

    #[dbus_proxy(
        interface = "org.freedesktop.zbus_macros.Test",
        default_service = "org.freedesktop.zbus_macros",
        default_path = "/org/freedesktop/zbus_macros/test"
    )]
    trait Test {
        /// comment for a_test()
        fn a_test(&self, val: &str) -> zbus::Result<u32>;

        /// The generated proxies implement both `zvariant::Type` and `serde::ser::Serialize`
        /// which is useful to pass in a proxy as a param. It serializes it as an `ObjectPath`.
        fn some_method<T>(&self, object_path: &T) -> zbus::Result<()>;

        #[dbus_proxy(name = "CheckRENAMING")]
        fn check_renaming(&self) -> zbus::Result<Vec<u8>>;

        #[dbus_proxy(property)]
        fn property(&self) -> fdo::Result<Vec<String>>;

        #[dbus_proxy(property)]
        fn set_property(&self, val: u16) -> fdo::Result<()>;

        #[dbus_proxy(signal)]
        fn a_signal<T>(&self, arg: u8, other: T) -> fdo::Result<()>
        where
            T: AsRef<str>;
    }

    let connection = zbus::Connection::session().unwrap();
    let proxy = TestProxy::builder(&connection)
        .cache_properties(false)
        .build()
        .unwrap();
    proxy
        .connect_a_signal(move |_arg, other: String| {
            println!("{}", other);
            Ok(())
        })
        .unwrap();
    // Let's also test signal streams.
    let connection = zbus::azync::Connection::from(connection);
    block_on(async move {
        let proxy = AsyncTestProxy::builder(&connection)
            .cache_properties(false)
            .build()
            .await
            .unwrap();
        fdo::AsyncDBusProxy::new(&connection)
            .await
            .unwrap()
            .request_name(
                "org.freedesktop.zbus_macros".try_into().unwrap(),
                fdo::RequestNameFlags::DoNotQueue.into(),
            )
            .await
            .unwrap();
        let mut stream = proxy.receive_a_signal().await.unwrap();

        let left_future = async move {
            // These calls will never happen so just testing the build mostly.
            let signal = stream.next().await.unwrap();
            let args = signal.args::<&str>().unwrap();
            assert_eq!(*args.arg(), 0u8);
            assert_eq!(*args.other(), "whatever");
        };
        futures_util::pin_mut!(left_future);
        let right_future = async {
            ready(()).await;
        };
        futures_util::pin_mut!(right_future);

        if let Either::Left((_, _)) = select(left_future, right_future).await {
            panic!("Shouldn't be receiving our dummy signal: `ASignal`");
        }
    });
}

#[test]
fn test_derive_error() {
    #[derive(Debug, DBusError)]
    #[dbus_error(prefix = "org.freedesktop.zbus")]
    enum Test {
        ZBus(zbus::Error),
        SomeExcuse,
        #[dbus_error(name = "I.Am.Sorry.Dave")]
        IAmSorryDave(String),
        LetItBe {
            desc: String,
        },
    }
}

#[test]
fn test_interface() {
    use zbus::Interface;

    struct Test<'a, T> {
        something: &'a str,
        generic: T,
    }

    #[dbus_interface(name = "org.freedesktop.zbus.Test")]
    impl<T: 'static> Test<'static, T>
    where
        T: serde::ser::Serialize + zvariant::Type,
    {
        /// Testing `no_arg` documentation is reflected in XML.
        fn no_arg(&self) {
            unimplemented!()
        }

        fn str_u32(&self, val: &str) -> zbus::fdo::Result<u32> {
            val.parse()
                .map_err(|e| zbus::fdo::Error::Failed(format!("Invalid val: {}", e)))
        }

        // TODO: naming output arguments after "RFC: Structural Records #2584"
        fn many_output(&self) -> zbus::fdo::Result<(&T, String)> {
            Ok((&self.generic, self.something.to_string()))
        }

        fn pair_output(&self) -> zbus::fdo::Result<((u32, String),)> {
            unimplemented!()
        }

        #[dbus_interface(name = "CheckVEC")]
        fn check_vec(&self) -> Vec<u8> {
            unimplemented!()
        }

        /// Testing my_prop documentation is reflected in XML.
        ///
        /// And that too.
        #[dbus_interface(property)]
        fn my_prop(&self) -> u16 {
            unimplemented!()
        }

        #[dbus_interface(property)]
        fn set_my_prop(&self, _val: u16) {
            unimplemented!()
        }

        /// Emit a signal.
        #[dbus_interface(signal)]
        fn signal(&self, arg: u8, other: &str) -> zbus::Result<()>;
    }

    const EXPECTED_XML: &str = r#"<interface name="org.freedesktop.zbus.Test">
  <!--
   Testing `no_arg` documentation is reflected in XML.
   -->
  <method name="NoArg">
  </method>
  <method name="StrU32">
    <arg name="val" type="s" direction="in"/>
    <arg type="u" direction="out"/>
  </method>
  <method name="ManyOutput">
    <arg type="u" direction="out"/>
    <arg type="s" direction="out"/>
  </method>
  <method name="PairOutput">
    <arg type="(us)" direction="out"/>
  </method>
  <method name="CheckVEC">
    <arg type="ay" direction="out"/>
  </method>
  <!--
   Emit a signal.
   -->
  <signal name="Signal">
    <arg name="arg" type="y"/>
    <arg name="other" type="s"/>
  </signal>
  <!--
   Testing my_prop documentation is reflected in XML.

   And that too.
   -->
  <property name="MyProp" type="q" access="readwrite"/>
</interface>
"#;
    let t = Test {
        something: "somewhere",
        generic: 42u32,
    };
    let mut xml = String::new();
    t.introspect_to_writer(&mut xml, 0);
    assert_eq!(xml, EXPECTED_XML);

    assert_eq!(Test::<u32>::name(), "org.freedesktop.zbus.Test");

    if false {
        // check compilation
        let c = zbus::Connection::session().unwrap();
        let m = zbus::Message::method(None::<()>, None::<()>, "/", None::<()>, "StrU32", &(42,))
            .unwrap();
        let _ = t.call(&c, &m, "StrU32".try_into().unwrap()).unwrap();
        t.signal(23, "ergo sum").unwrap();
    }
}
