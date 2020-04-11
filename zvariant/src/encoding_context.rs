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
    n_bytes_before: usize,
}

impl EncodingContext {
    pub fn new(format: EncodingFormat) -> Self {
        Self::new_n_bytes_before(format, 0)
    }

    pub fn new_n_bytes_before(format: EncodingFormat, n_bytes_before: usize) -> Self {
        Self {
            format,
            n_bytes_before,
        }
    }

    pub fn format(self) -> EncodingFormat {
        self.format
    }

    pub fn n_bytes_before(self) -> usize {
        self.n_bytes_before
    }
}
