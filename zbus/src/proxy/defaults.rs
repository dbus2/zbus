/// Trait for the default associated values of a proxy.
///
/// The trait is automatically implemented by the [`proxy`] macro on your behalf, and may be later
/// used to retrieve the associated constants.
///
/// [`proxy`]: attr.proxy.html
pub trait ProxyDefault {
    const INTERFACE: Option<&'static str>;
    const DESTINATION: Option<&'static str>;
    const PATH: Option<&'static str>;
}

impl ProxyDefault for super::Proxy<'_> {
    const INTERFACE: Option<&'static str> = None;
    const DESTINATION: Option<&'static str> = None;
    const PATH: Option<&'static str> = None;
}
