use std::marker::PhantomData;

/// The encoding format.
///
/// Currently only D-Bus format is supported but [GVariant] support is also planned.
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
pub struct EncodingContext<B> {
    format: EncodingFormat,
    position: usize,

    b: PhantomData<B>,
}

impl<B> EncodingContext<B>
where
    B: byteorder::ByteOrder,
{
    pub fn new(format: EncodingFormat, position: usize) -> Self {
        Self {
            format,
            position,
            b: PhantomData,
        }
    }

    pub fn new_dbus(position: usize) -> Self {
        Self::new(EncodingFormat::DBus, position)
    }

    pub fn format(self) -> EncodingFormat {
        self.format
    }

    pub fn position(self) -> usize {
        self.position
    }
}
