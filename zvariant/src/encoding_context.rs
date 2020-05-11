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

/// The encoding context to use with the [serialization and deserialization] API.
///
/// This type is generic over the [ByteOrder] trait. Moreover, the encoding is dependent on the
/// position of the encoding in the entire message and hence the need to [specify] the byte
/// position of the data being serialized or deserialized. Simply pass `0` if serializing or
/// deserializing to or from the beginning of message, or the preceeding bytes end on an 8-byte
/// boundry.
///
/// # Examples
///
/// ```
/// use byteorder::LE;
///
/// use zvariant::EncodingContext as Context;
/// use zvariant::{from_slice, to_bytes};
///
/// let str_vec = vec!["Hello", "World"];
/// let ctxt = Context::<LE>::new_dbus(0);
/// let encoded = to_bytes(ctxt, &str_vec).unwrap();
///
/// // Let's decode the 2nd element of the array only
/// let ctxt = Context::<LE>::new_dbus(14);
/// let decoded: &str = from_slice(&encoded[14..], ctxt).unwrap();
/// assert_eq!(decoded, "World");
/// ```
///
/// [serialization and deserialization]: index.html#functions
/// [ByteOrder]: https://docs.rs/byteorder/1.3.4/byteorder/trait.ByteOrder.html
/// [specify]: #method.new
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
    /// Create a new encoding context.
    pub fn new(format: EncodingFormat, position: usize) -> Self {
        Self {
            format,
            position,
            b: PhantomData,
        }
    }

    /// Convenient wrapper for [`new`] to create a context for D-Bus format.
    ///
    /// [`new`]: #method.new
    pub fn new_dbus(position: usize) -> Self {
        Self::new(EncodingFormat::DBus, position)
    }

    /// The [`EncodingFormat`] of this context.
    ///
    /// [`EncodingFormat`]: enum.EncodingFormat.html
    pub fn format(self) -> EncodingFormat {
        self.format
    }

    /// The byte position of the value to be encoded or decoded, in the entire message.
    pub fn position(self) -> usize {
        self.position
    }
}
