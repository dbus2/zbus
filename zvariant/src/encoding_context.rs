/// The encoding format.
///
/// Currently only D-Bus format is supported but [`GVariant`] support is also planned.
///
/// [GVariant]: https://developer.gnome.org/glib/stable/glib-GVariant.html
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EncodingFormat {
    DBus,
    // TODO: GVariant
}

impl Default for EncodingFormat {
    fn default() -> Self {
        EncodingFormat::DBus
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EncodingContext {
    format: EncodingFormat,
}

impl EncodingContext {
    pub fn new(format: EncodingFormat) -> Self {
        Self { format }
    }

    pub fn format(self) -> EncodingFormat {
        self.format
    }
}
