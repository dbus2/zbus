# FAQ

## How to use a struct as a dictionary?

Since the use of a dictionary, specifically one with strings as keys and variants as value (i-e
`a{sv}`) is very common in the D-Bus world and use of HashMaps isn't as convenient and type-safe as
a struct, you might find yourself wanting to use a struct as a dictionary.

`zvariant` provides convenient macros for making this possible: [`TypeDict`], [`SerializeDict`] and
[`DeserializeDict`]. Here is a simple example:

```rust
use zbus::{
    dbus_proxy, dbus_interface, fdo::Result,
    zvariant::{DeserializeDict, SerializeDict, TypeDict},
};

#[derive(DeserializeDict, SerializeDict, TypeDict)]
pub struct Dictionary {
    field1: u16,
    #[zvariant(rename = "another-name")]
    field2: i64,
    optional_field: Option<String>,
}

#[dbus_proxy(
    interface = "org.zbus.DictionaryGiver",
    default_path = "/org/zbus/DictionaryGiver",
    default_service = "org.zbus.DictionaryGiver",
)]
trait DictionaryGiver {
    fn give_me(&self) -> Result<Dictionary>;
}

struct DictionaryGiverInterface;

#[dbus_interface(interface = "org.zbus.DictionaryGiver")]
impl DictionaryGiverInterface {
    fn give_me(&self) -> Result<Dictionary> {
        Ok(Dictionary {
            field1: 1,
            field2: 4,
            optional_field: Some(String::from("blah")),
        })
    }
}
```

## Why do async tokio API calls from interface methods not work?

Many of the tokio (and tokio-based) APIs assume the tokio runtime to be driving the async machinery
and since by default, zbus runs the `ObjectServer` in its own internal runtime thread, it's not
possible to use these APIs from interface methods.

Not to worry, though! There is a very easy way around this unfortunate issue:

* Disable the internal runtime thread.
* Launch a tokio task to tick the internal runtime.

Here is an example:

```rust,no_run
use tokio::{
    io::AsyncReadExt,
    sync::mpsc::{channel, Sender},
};
use zbus::{
    dbus_interface,
    fdo::{self, Result},
};

struct OurInterface(Sender<()>);

#[dbus_interface(interface = "org.fdo.OurInterface")]
impl OurInterface {
    async fn quit(&self) -> fdo::Result<()> {
        self.0
            .send(())
            .await
            .map_err(|_| fdo::Error::Failed("shouldn't happen".to_string()))
    }

    async fn read_file(&self, path: &str) -> fdo::Result<String> {
        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|_| fdo::Error::FileNotFound(format!("Failed to open {}", path)))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .await
            .map_err(|_| fdo::Error::Failed(format!("Failed to read {}", path)))?;

        Ok(contents)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (sender, mut receiver) = channel::<()>(1);
    let conn = zbus::ConnectionBuilder::session()?
        .serve_at("/our", OurInterface(sender))?
        .name("org.fdo.OurInterface")?
        .internal_executor(false)
        .build()
        .await?;

    tokio::spawn(async move {
        loop {
            conn.executor().tick().await;
        }
    });

    receiver.recv().await.unwrap();

    Ok(())
}
```

Please note that by default zbus relies on `async-io` crate to communicate with the bus, which uses
its own thread. If you'd like to avoid that and have a fuller integration with `tokio`, you need to
do **a bit** more work:

1. Enable `tokio` feature of zbus.
2. Manually create the `tokio::net::UnixStream` for the `zbus::Connection` to use:

```rust
use std::error::Error;
use tokio::net::UnixStream;
use zbus::{Address, ConnectionBuilder};

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let stream = match Address::session()? {
        Address::Unix(s) => UnixStream::connect(s).await?,
        _ => unimplemented!(),
    };
    let conn = ConnectionBuilder::socket(stream)
        .internal_executor(false)
        .build()
        .await?;
    let executor_conn = conn.clone();
    tokio::task::spawn(async move {
        loop {
            executor_conn.executor().tick().await;
        }
    });
    let proxy = zbus::fdo::DBusProxy::new(&conn).await?;
    let features = proxy.features().await?;
    print!("Bus Features: ");
    for (i, feature) in features.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{}", feature);
    }
    println!(".");

    Ok(())
}
```

[`TypeDict`]: https://docs.rs/zvariant/3.0.0/zvariant/derive.TypeDict.html
[`SerializeDict`]: https://docs.rs/zvariant/3.0.0/zvariant/derive.SerializeDict.html
[`DeserializeDict`]: https://docs.rs/zvariant/3.0.0/zvariant/derive.DeserializeDict.html
