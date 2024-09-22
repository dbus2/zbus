use std::sync::mpsc::channel;

use ntest::timeout;
use test_log::test;
use zbus::block_on;

use zbus::{blocking, object_server::SignalContext};

#[test]
#[timeout(15000)]
fn issue_173() {
    // Tests the fix for https://github.com/dbus2/zbus/issues/173
    //
    // The issue is caused by proxy not keeping track of its destination's owner changes
    // (service restart) and failing to receive signals as a result.
    let (tx, rx) = channel();
    let child = std::thread::spawn(move || {
        let conn = blocking::Connection::session().unwrap();
        #[zbus::proxy(
            interface = "org.freedesktop.zbus.ComeAndGo",
            default_service = "org.freedesktop.zbus.ComeAndGo",
            default_path = "/org/freedesktop/zbus/ComeAndGo"
        )]
        trait ComeAndGo {
            #[zbus(signal)]
            fn the_signal(&self) -> zbus::Result<()>;
        }

        let proxy = ComeAndGoProxyBlocking::new(&conn).unwrap();
        let signals = proxy.receive_the_signal().unwrap();
        tx.send(()).unwrap();

        // We receive two signals, each time from different unique names. W/o the fix for
        // issue#173, the second iteration hangs.
        for _ in signals.take(2) {
            tx.send(()).unwrap();
        }
    });

    struct ComeAndGo;
    #[zbus::interface(name = "org.freedesktop.zbus.ComeAndGo")]
    impl ComeAndGo {
        #[zbus(signal)]
        async fn the_signal(signal_ctxt: &SignalContext<'_>) -> zbus::Result<()>;
    }

    rx.recv().unwrap();
    for _ in 0..2 {
        let conn = blocking::connection::Builder::session()
            .unwrap()
            .serve_at("/org/freedesktop/zbus/ComeAndGo", ComeAndGo)
            .unwrap()
            .name("org.freedesktop.zbus.ComeAndGo")
            .unwrap()
            .build()
            .unwrap();

        let iface_ref = conn
            .object_server()
            .interface::<_, ComeAndGo>("/org/freedesktop/zbus/ComeAndGo")
            .unwrap();
        block_on(ComeAndGo::the_signal(iface_ref.signal_context())).unwrap();

        rx.recv().unwrap();

        // Now we release the name ownership to use a different connection (i-e new unique
        // name).
        conn.release_name("org.freedesktop.zbus.ComeAndGo").unwrap();
    }

    child.join().unwrap();
}
