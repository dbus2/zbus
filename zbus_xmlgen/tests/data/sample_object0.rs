#[dbus_proxy(interface = "com.example.SampleInterface0", assume_defaults = true)]
trait SampleInterface0 {

    /// BarplexSig method
    fn barplex_sig(&self, rule: &(&[i32], i32, std::collections::HashMap<&str, &str>, i32, &[i32], i32, &[&str], i32, bool)) -> zbus::Result<Vec<(String, zbus::zvariant::OwnedObjectPath)>>;

    /// Bazify method
    fn bazify(&self, bar: &(i32, i32, u32)) -> zbus::Result<zbus::zvariant::OwnedValue>;

    /// Frobate method
    fn frobate(&self, foz: i32, foo: i32) -> zbus::Result<(String, std::collections::HashMap<u32, String>)>;

    /// MogrifyMe method
    fn mogrify_me(&self, bar: &(i32, i32, &[zbus::zvariant::Value<'_>])) -> zbus::Result<()>;

    /// Odyssey method
    #[allow(clippy::too_many_arguments)]
    fn odyssey(&self, odysseus: i32, penelope: &str, telemachus: u32, circe: i32, athena: bool, polyphemus: i32, calypso: &zbus::zvariant::Value<'_>) -> zbus::Result<()>;

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

    /// Matryoshkas property
    #[dbus_proxy(property)]
    #[allow(clippy::type_complexity)]
    fn matryoshkas(&self) -> zbus::Result<Vec<(zbus::zvariant::OwnedObjectPath, i32, Vec<String>, u64, std::collections::HashMap<String, zbus::zvariant::OwnedValue>)>>;
}
