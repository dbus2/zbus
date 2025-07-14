use zbus::{interface, Connection};

struct TestInterface {
    dbus_connection: Connection,
    count: u8,
}

#[interface(name = "org.zbus.MyGreeter1")]
impl TestInterface {
    // Can be `async` as well.
    async fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        let _interface = self
            .dbus_connection
            .object_server()
            .interface_unchecked::<_, TestInterface>("/test")
            .await
            .unwrap();
        format!("Hello {}! I have been called {} times.", name, self.count)
    }
}

#[cfg(test)]
mod test {
    use crate::issue::issue_1430::TestInterface;

    #[tokio::test]
    async fn test_deadlock() {
        let connection = zbus::connection::Connection::session().await.unwrap();
        let test_interface = TestInterface {
            count: 0,
            dbus_connection: connection.clone(),
        };
        connection
            .object_server()
            .at("/test", test_interface)
            .await
            .unwrap();
        let interface = connection
            .object_server()
            .interface::<_, TestInterface>("/test")
            .await
            .unwrap();
        let _ = interface.get_mut().await.say_hello("deadlock").await;
    }
}
