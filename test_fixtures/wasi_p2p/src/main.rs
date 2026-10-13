//! Runs a peer-to-peer D-Bus connection: a server and a client in one program, over TCP on the
//! loopback interface.
//!
//! CI runs it on WASI, preview 2 of which has TCP sockets, as `wasmtime run -S inherit-network`:
//! once over a pair of streams the program connects itself and hands to zbus, and once over a
//! `tcp:` address that zbus connects to. Each step prints its name, and a failure panics, which
//! traps the module and fails the run.

use std::net::{TcpListener, TcpStream};

use futures_util::future::try_join;
use zbus::{Connection, Guid, connection, interface};

fn main() {
    zbus::block_on(async {
        step("streams", streams()).await;
        step("address", address()).await;
    });
}

/// Runs `check`, and panics with its error if it fails.
async fn step(name: &str, check: impl Future<Output = zbus::Result<()>>) {
    println!("{name}...");
    if let Err(e) = check.await {
        panic!("{name}: {e}");
    }
    println!("{name}: ok");
}

/// A connection over streams connected by the program itself.
async fn streams() -> zbus::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    // The loopback interface completes the connect before the listener accepts it.
    let client = TcpStream::connect(listener.local_addr()?)?;
    let (server, _) = listener.accept()?;

    let (server, client) = try_join(
        server_builder(connection::Builder::tcp_stream(server)).build(),
        connection::Builder::tcp_stream(client).p2p().build(),
    )
    .await?;

    greet(&server, &client).await
}

/// A connection to a `tcp:` address, which zbus connects to on its own.
async fn address() -> zbus::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    let client = connection::Builder::address(format!("tcp:host=127.0.0.1,port={port}").as_str())
        .p2p()
        .build();
    // The client is polled first, so its connect is under way, and the loopback interface
    // completes it, before the listener is asked to accept it.
    let server = async {
        let (server, _) = listener.accept()?;

        server_builder(connection::Builder::tcp_stream(server))
            .build()
            .await
    };
    let (client, server) = try_join(client, server).await?;

    greet(&server, &client).await
}

/// The server's side of a connection: serving [`Greeter`], anonymously over TCP.
fn server_builder(builder: connection::Builder<'_>) -> connection::Builder<'_> {
    builder
        .server(Guid::generate())
        .p2p()
        .serve_at("/org/zbus/Greeter", Greeter)
}

/// Calls the server's [`Greeter`] from the client, and checks the reply.
async fn greet(_server: &Connection, client: &Connection) -> zbus::Result<()> {
    // A peer-to-peer connection has no bus to name a destination on.
    let reply = client
        .call_method(
            None::<&str>,
            "/org/zbus/Greeter",
            Some("org.zbus.Greeter"),
            "Greet",
            &("WASI",),
        )
        .await?;
    let greeting: String = reply.body().deserialize()?;
    assert_eq!(greeting, "Hello, WASI!");

    Ok(())
}

struct Greeter;

#[interface(name = "org.zbus.Greeter")]
impl Greeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {name}!")
    }
}
