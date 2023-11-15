use std::marker::PhantomData;

use static_assertions::assert_impl_all;

use crate::serialized::Format;

/// The encoding context to use with the [serialization and deserialization] API.
///
/// This type is generic over the [ByteOrder] trait. Moreover, the encoding is dependent on the
/// position of the encoding in the entire message and hence the need to [specify] the byte
/// position of the data being serialized or deserialized. Simply pass `0` if serializing or
/// deserializing to or from the beginning of message, or the preceding bytes end on an 8-byte
/// boundary.
///
/// # Examples
///
/// ```
/// use byteorder::LE;
///
/// use zvariant::serialized::Context;
/// use zvariant::to_bytes;
///
/// let str_vec = vec!["Hello", "World"];
/// let ctxt = Context::<LE>::new_dbus(0);
/// let encoded = to_bytes(ctxt, &str_vec).unwrap();
///
/// // Let's decode the 2nd element of the array only
/// let slice = encoded.slice(14..);
/// let decoded: &str = slice.deserialize().unwrap().0;
/// assert_eq!(decoded, "World");
/// ```
///
/// [serialization and deserialization]: index.html#functions
/// [ByteOrder]: https://docs.rs/byteorder/1.3.4/byteorder/trait.ByteOrder.html
/// [specify]: #method.new
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Context<B> {
    format: Format,
    position: usize,

    b: PhantomData<B>,
}

assert_impl_all!(Context<byteorder::NativeEndian>: Send, Sync, Unpin);

impl<B> Context<B>
where
    B: byteorder::ByteOrder,
{
    /// Create a new encoding context.
    pub fn new(format: Format, position: usize) -> Self {
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
        Self::new(Format::DBus, position)
    }

    /// Convenient wrapper for [`new`] to create a context for GVariant format.
    ///
    /// [`new`]: #method.new
    #[cfg(feature = "gvariant")]
    pub fn new_gvariant(position: usize) -> Self {
        Self::new(Format::GVariant, position)
    }

    /// The [`Format`] of this context.
    pub fn format(self) -> Format {
        self.format
    }

    /// The byte position of the value to be encoded or decoded, in the entire message.
    pub fn position(self) -> usize {
        self.position
    }
}
