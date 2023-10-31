use byteorder::WriteBytesExt;
use serde::Serialize;
use std::{
    io::{Seek, Write},
    marker::PhantomData,
};

#[cfg(unix)]
use std::os::fd::OwnedFd;

#[cfg(feature = "gvariant")]
use crate::gvariant::Serializer as GVSerializer;
use crate::{
    container_depths::ContainerDepths,
    dbus::Serializer as DBusSerializer,
    serialized::{Data, Size},
    signature_parser::SignatureParser,
    utils::*,
    Basic, DynamicType, EncodingContext, EncodingFormat, Error, Result, Signature,
};

struct NullWriteSeek;

impl Write for NullWriteSeek {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl Seek for NullWriteSeek {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        Ok(std::u64::MAX) // should never read the return value!
    }
}

/// Calculate the serialized size of `T`.
///
/// # Examples
///
/// ```
/// use zvariant::{EncodingContext, serialized_size};
///
/// let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
/// let len = serialized_size(ctxt, "hello world").unwrap();
/// assert_eq!(*len, 16);
///
/// let len = serialized_size(ctxt, &("hello world!", 42_u64)).unwrap();
/// assert_eq!(*len, 32);
/// ```
pub fn serialized_size<B, T: ?Sized>(ctxt: EncodingContext<B>, value: &T) -> Result<Size<B>>
where
    B: byteorder::ByteOrder,
    T: Serialize + DynamicType,
{
    let mut null = NullWriteSeek;

    to_writer(&mut null, ctxt, value)
}

/// Serialize `T` to the given `writer`.
///
/// # Examples
///
/// ```
/// use zvariant::{EncodingContext, serialized::Data, to_writer};
///
/// let ctxt = EncodingContext::<byteorder::LE>::new_dbus(0);
/// let mut cursor = std::io::Cursor::new(vec![]);
/// to_writer(&mut cursor, ctxt, &42u32).unwrap();
/// let encoded = Data::new(cursor.get_ref(), ctxt);
/// let value: u32 = encoded.deserialize().unwrap().0;
/// assert_eq!(value, 42);
/// ```
///
/// [`to_writer_fds`]: fn.to_writer_fds.html
pub fn to_writer<B, W, T: ?Sized>(
    writer: &mut W,
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<Size<B>>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    T: Serialize + DynamicType,
{
    let signature = value.dynamic_signature();

    to_writer_for_signature(writer, ctxt, &signature, value)
}

/// Serialize `T` as a byte vector.
///
/// See [`Data::deserialize`] documentation for an example of how to use this function.
pub fn to_bytes<B, T: ?Sized>(
    ctxt: EncodingContext<B>,
    value: &T,
) -> Result<Data<'static, 'static, B>>
where
    B: byteorder::ByteOrder,
    T: Serialize + DynamicType,
{
    to_bytes_for_signature(ctxt, value.dynamic_signature(), value)
}

/// Serialize `T` that has the given signature, to the given `writer`.
///
/// Use this function instead of [`to_writer`] if the value being serialized does not implement
/// [`Type`].
///
/// [`to_writer`]: fn.to_writer.html
/// [`Type`]: trait.Type.html
pub fn to_writer_for_signature<'s, B, W, S, T: ?Sized>(
    writer: &mut W,
    ctxt: EncodingContext<B>,
    signature: S,
    value: &T,
) -> Result<Size<B>>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
    S: TryInto<Signature<'s>>,
    S::Error: Into<Error>,
    T: Serialize,
{
    #[cfg(unix)]
    let mut fds = vec![];
    let len = match ctxt.format() {
        EncodingFormat::DBus => {
            let mut ser = DBusSerializer::<B, W>::new(
                signature,
                writer,
                #[cfg(unix)]
                &mut fds,
                ctxt,
            )?;
            value.serialize(&mut ser)?;
            ser.0.bytes_written
        }
        #[cfg(feature = "gvariant")]
        EncodingFormat::GVariant => {
            let mut ser = GVSerializer::<B, W>::new(
                signature,
                writer,
                #[cfg(unix)]
                &mut fds,
                ctxt,
            )?;
            value.serialize(&mut ser)?;
            ser.0.bytes_written
        }
    };

    let size = Size::new(len, ctxt);
    #[cfg(unix)]
    let size = size.set_fds(fds);

    Ok(size)
}

/// Serialize `T` that has the given signature, to a new byte vector.
///
/// Use this function instead of [`to_bytes`] if the value being serialized does not implement
/// [`Type`]. See [`from_slice_for_signature`] documentation for an example of how to use this
/// function.
///
/// [`to_bytes`]: fn.to_bytes.html
/// [`Type`]: trait.Type.html
/// [`from_slice_for_signature`]: fn.from_slice_for_signature.html#examples
pub fn to_bytes_for_signature<'s, B, S, T: ?Sized>(
    ctxt: EncodingContext<B>,
    signature: S,
    value: &T,
) -> Result<Data<'static, 'static, B>>
where
    B: byteorder::ByteOrder,
    S: TryInto<Signature<'s>>,
    S::Error: Into<Error>,
    T: Serialize,
{
    let mut cursor = std::io::Cursor::new(vec![]);
    let ret = to_writer_for_signature(&mut cursor, ctxt, signature, value)?;
    #[cfg(unix)]
    let encoded = Data::new_fds(cursor.into_inner(), ctxt, ret.into_fds());
    #[cfg(not(unix))]
    let encoded = {
        let _ = ret;
        Data::new(cursor.into_inner(), ctxt)
    };

    Ok(encoded)
}

/// Context for all our serializers and provides shared functionality.
pub(crate) struct SerializerCommon<'ser, 'sig, B, W> {
    pub(crate) ctxt: EncodingContext<B>,
    pub(crate) writer: &'ser mut W,
    pub(crate) bytes_written: usize,
    #[cfg(unix)]
    pub(crate) fds: &'ser mut Vec<OwnedFd>,

    pub(crate) sig_parser: SignatureParser<'sig>,

    pub(crate) value_sign: Option<Signature<'static>>,

    pub(crate) container_depths: ContainerDepths,

    pub(crate) b: PhantomData<B>,
}

impl<'ser, 'sig, B, W> SerializerCommon<'ser, 'sig, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    #[cfg(unix)]
    pub(crate) fn add_fd(&mut self, fd: std::os::fd::RawFd) -> Result<u32> {
        use std::os::fd::{AsRawFd, BorrowedFd};

        if let Some(idx) = self.fds.iter().position(|x| x.as_raw_fd() == fd) {
            return Ok(idx as u32);
        }
        let idx = self.fds.len();
        // Cloning implies dup and is unfortunate but we need to return owned fds
        // and dup is not expensive (at least on Linux).
        let fd = unsafe { BorrowedFd::borrow_raw(fd) }.try_clone_to_owned()?;
        self.fds.push(fd);

        Ok(idx as u32)
    }

    pub(crate) fn add_padding(&mut self, alignment: usize) -> Result<usize> {
        let padding = padding_for_n_bytes(self.abs_pos(), alignment);
        if padding > 0 {
            let byte = [0_u8; 1];
            for _ in 0..padding {
                self.write_all(&byte)
                    .map_err(|e| Error::InputOutput(e.into()))?;
            }
        }

        Ok(padding)
    }

    pub(crate) fn prep_serialize_basic<T>(&mut self) -> Result<()>
    where
        T: Basic,
    {
        self.sig_parser.skip_char()?;
        self.add_padding(T::alignment(self.ctxt.format()))?;

        Ok(())
    }

    /// This starts the enum serialization.
    ///
    /// It's up to the caller to do the rest: serialize the variant payload and skip the `).
    pub(crate) fn prep_serialize_enum_variant(&mut self, variant_index: u32) -> Result<()> {
        // Encode enum variants as a struct with first field as variant index
        let signature = self.sig_parser.next_signature()?;
        if self.sig_parser.next_char()? != STRUCT_SIG_START_CHAR {
            return Err(Error::SignatureMismatch(
                signature.to_owned(),
                format!("expected `{STRUCT_SIG_START_CHAR}`"),
            ));
        }

        let alignment = alignment_for_signature(&signature, self.ctxt.format())?;
        self.add_padding(alignment)?;

        // Now serialize the veriant index.
        self.write_u32::<B>(variant_index)
            .map_err(|e| Error::InputOutput(e.into()))?;

        // Skip the `(`, `u`.
        self.sig_parser.skip_chars(2)?;

        Ok(())
    }

    fn abs_pos(&self) -> usize {
        self.ctxt.position() + self.bytes_written
    }
}

impl<'ser, 'sig, B, W> Write for SerializerCommon<'ser, 'sig, B, W>
where
    B: byteorder::ByteOrder,
    W: Write + Seek,
{
    /// Write `buf` and increment internal bytes written counter.
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf).map(|n| {
            self.bytes_written += n;

            n
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
