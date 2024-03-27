use byteorder::LE;
use serde::Deserialize;
use std::collections::HashMap;
use zbus::zvariant::{serialized::Context, to_bytes, OwnedValue, Type};

#[derive(Deserialize, Type)]
#[zvariant(signature = "a{sv}")]
struct Outer {
    foo: OwnedValue,
}

#[test_log::test]
fn convert() {
    let ctxt = Context::<LE>::new_dbus(0);
    let value =
        <HashMap<String, OwnedValue>>::from([("foo".into(), 23.into()), ("bar".into(), 42.into())]);
    let data = to_bytes(ctxt, &value).unwrap();
    eprintln!("{data:02x?}");
    let good = data.deserialize::<Outer>().unwrap().0;
    eprintln!("{:?}", good.foo);
}
