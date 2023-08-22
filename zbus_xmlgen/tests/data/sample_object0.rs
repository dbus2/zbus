#[dbus_proxy(interface = "com.example.SampleInterface0", assume_defaults = true)]
trait SampleInterface0 {

    /// Bazify method
    fn bazify(&self, bar: &(i32, i32, u32)) -> zbus::Result<zbus::zvariant::OwnedValue>;

    /// Frobate method
    fn frobate(&self, foz: i32, foo: i32) -> zbus::Result<(String, std::collections::HashMap<u32, String>)>;

    /// MogrifyMe method
    fn mogrify_me(&self, bar: &(i32, i32, &[zbus::zvariant::Value<'_>])) -> zbus::Result<()>;

    /// Changed signal
    #[dbus_proxy(signal)]
    fn changed(&self, new_value: bool) -> zbus::Result<()>;

    /// Changed2 signal
    #[dbus_proxy(signal)]
    fn changed2(&self, new_value: bool, new_value2: bool) -> zbus::Result<()>;

    /// Bar property
    #[dbus_proxy(property)]
    fn bar(&self) -> zbus::Result<u8>;
    #[dbus_proxy(property)]
    fn set_bar(&self, value: u8) -> zbus::Result<()>;
}
