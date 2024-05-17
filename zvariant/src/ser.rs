use serde::Serialize;
use std::io::{Seek, Write};

#[cfg(unix)]
use std::os::fd::OwnedFd;

#[cfg(feature = "gvariant")]
use crate::gvariant::Serializer as GVSerializer;
use crate::{
    dbus::Serializer as DBusSerializer,
    serialized::{Context, Data, Format, Size, Written},
    utils::*,
    value::ser::ValueSerializer,
    DynamicType, Error, OwnedValue, Result, Signature,
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
/// use zvariant::{serialized::Context, serialized_size, LE};
///
/// let ctxt = Context::new_dbus(LE, 0);
/// let len = serialized_size(ctxt, "hello world").unwrap();
/// assert_eq!(*len, 16);
///
/// let len = serialized_size(ctxt, &("hello world!", 42_u64)).unwrap();
/// assert_eq!(*len, 32);
/// ```
pub fn serialized_size<T>(ctxt: Context, value: &T) -> Result<Size>
where
    T: ?Sized + Serialize + DynamicType,
{
    let mut null = NullWriteSeek;
    let signature = value.dynamic_signature();
    #[cfg(unix)]
    let mut fds = FdList::Number(0);

    let len = match ctxt.format() {
        Format::DBus => {
            let mut ser = DBusSerializer::<NullWriteSeek>::new(
                signature,
                &mut null,
                #[cfg(unix)]
                &mut fds,
                ctxt,
            )?;
            value.serialize(&mut ser)?;
            ser.bytes_written()
        }
        #[cfg(feature = "gvariant")]
        Format::GVariant => {
            let mut ser = GVSerializer::<NullWriteSeek>::new(
                signature,
                &mut null,
                #[cfg(unix)]
                &mut fds,
                ctxt,
            )?;
            value.serialize(&mut ser)?;
            ser.common.bytes_written
        }
    };

    let size = Size::new(len, ctxt);
    #[cfg(unix)]
    let size = match fds {
        FdList::Number(n) => size.set_num_fds(n),
        FdList::Fds(_) => unreachable!("`Fds::Fds` is not possible here"),
    };

    Ok(size)
}

/// Serialize `T` to the given `writer`.
///
/// # Examples
///
/// ```
/// use zvariant::{serialized::{Context, Data}, to_writer, LE};
///
/// let ctxt = Context::new_dbus(LE, 0);
/// let mut cursor = std::io::Cursor::new(vec![]);
/// // SAFETY: No FDs are being serialized here so its completely safe.
/// unsafe { to_writer(&mut cursor, ctxt, &42u32) }.unwrap();
/// let encoded = Data::new(cursor.get_ref(), ctxt);
/// let value: u32 = encoded.deserialize().unwrap().0;
/// assert_eq!(value, 42);
/// ```
///
/// # Safety
///
/// On Unix systems, the returned [`Written`] instance can contain file descriptors and therefore
/// the caller is responsible for not dropping the returned [`Written`] instance before the
/// `writer`. Otherwise, the file descriptors in the `Written` instance will be closed while
/// serialized data will still refer to them. Hence why this function is marked unsafe.
///
/// On non-Unix systems, the returned [`Written`] instance will not contain any file descriptors and
/// hence is safe to drop.
///
/// [`to_writer_fds`]: fn.to_writer_fds.html
pub unsafe fn to_writer<W, T>(writer: &mut W, ctxt: Context, value: &T) -> Result<Written>
where
    W: Write + Seek,
    T: ?Sized + Serialize + DynamicType,
{
    let signature = value.dynamic_signature();

    to_writer_for_signature(writer, ctxt, &signature, value)
}

/// Serialize `T` as a byte vector.
///
/// See [`Data::deserialize`] documentation for an example of how to use this function.
pub fn to_bytes<T>(ctxt: Context, value: &T) -> Result<Data<'static, 'static>>
where
    T: ?Sized + Serialize + DynamicType,
{
    to_bytes_for_signature(ctxt, value.dynamic_signature(), value)
}

/// Serialize `T` that has the given signature, to the given `writer`.
///
/// Use this function instead of [`to_writer`] if the value being serialized does not implement
/// [`DynamicType`].
///
/// # Safety
///
/// On Unix systems, the returned [`Written`] instance can contain file descriptors and therefore
/// the caller is responsible for not dropping the returned [`Written`] instance before the
/// `writer`. Otherwise, the file descriptors in the `Written` instance will be closed while
/// serialized data will still refer to them. Hence why this function is marked unsafe.
///
/// On non-Unix systems, the returned [`Written`] instance will not contain any file descriptors and
/// hence is safe to drop.
///
/// [`to_writer`]: fn.to_writer.html
pub unsafe fn to_writer_for_signature<'s, W, S, T>(
    writer: &mut W,
    ctxt: Context,
    signature: S,
    value: &T,
) -> Result<Written>
where
    W: Write + Seek,
    S: TryInto<Signature<'s>>,
    S::Error: Into<Error>,
    T: ?Sized + Serialize,
{
    #[cfg(unix)]
    let mut fds = FdList::Fds(vec![]);

    let len = match ctxt.format() {
        Format::DBus => {
            let mut ser = DBusSerializer::new(
                signature,
                writer,
                #[cfg(unix)]
                &mut fds,
                ctxt,
            )?;

            value.serialize(&mut ser)?;
            ser.bytes_written()
        }
        #[cfg(feature = "gvariant")]
        Format::GVariant => {
            let mut ser = GVSerializer::<W>::new(
                signature,
                writer,
                #[cfg(unix)]
                &mut fds,
                ctxt,
            )?;
            value.serialize(&mut ser)?;
            ser.common.bytes_written
        }
    };

    let written = Written::new(len, ctxt);
    #[cfg(unix)]
    let written = match fds {
        FdList::Fds(fds) => written.set_fds(fds),
        FdList::Number(_) => unreachable!("`Fds::Number` is not possible here"),
    };

    Ok(written)
}

/// Serialize `T` that has the given signature, to a new byte vector.
///
/// Use this function instead of [`to_bytes`] if the value being serialized does not implement
/// [`DynamicType`]. See [`from_slice_for_signature`] documentation for an example of how to use
/// this function.
///
/// [`to_bytes`]: fn.to_bytes.html
/// [`from_slice_for_signature`]: fn.from_slice_for_signature.html#examples
pub fn to_bytes_for_signature<'s, S, T>(
    ctxt: Context,
    signature: S,
    value: &T,
) -> Result<Data<'static, 'static>>
where
    S: TryInto<Signature<'s>>,
    S::Error: Into<Error>,
    T: ?Sized + Serialize,
{
    let mut cursor = std::io::Cursor::new(vec![]);
    // SAFETY: We put the bytes and FDs in the `Data` to ensure that the data and FDs are only
    // dropped together.
    let ret = unsafe { to_writer_for_signature(&mut cursor, ctxt, signature, value) }?;
    #[cfg(unix)]
    let encoded = Data::new_fds(cursor.into_inner(), ctxt, ret.into_fds());
    #[cfg(not(unix))]
    let encoded = {
        let _ = ret;
        Data::new(cursor.into_inner(), ctxt)
    };

    Ok(encoded)
}

/// Serialize `T` to an `OwnedValue`.
///
/// # Examples
///
/// ```
/// use zvariant::{to_value, OwnedValue};
///
/// let input: u32 = 42;
/// let value: OwnedValue = to_value(&input).unwrap();
/// let output: &u32 = value.downcast_ref().unwrap();
/// assert_eq!(*output, 42);
/// ```
pub fn to_value<T>(value: &T) -> Result<OwnedValue>
where
    T: ?Sized + Serialize + DynamicType,
{
    let signature = value.dynamic_signature();
    to_value_for_signature(signature, value)
}

/// Serialize `T` to an `OwnedValue` using the given
/// signature.
///
/// Use this function instead of [`to_value`] if the
/// value being serialized does not implement [`DynamicType`].
///
/// # Examples
///
/// ```
/// use zvariant::{to_value_for_signature, OwnedValue};
///
/// let input: u32 = 42;
/// let value: OwnedValue = to_value_for_signature("u", &input).unwrap();
/// let output: &u32 = value.downcast_ref().unwrap();
/// assert_eq!(*output, 42);
/// ```
///
/// [`to_value`]: fn.to_value.html
pub fn to_value_for_signature<'s, S, T>(signature: S, value: &T) -> Result<OwnedValue>
where
    S: TryInto<Signature<'s>>,
    S::Error: Into<Error>,
    T: ?Sized + Serialize,
{
    let signature: Signature<'_> = signature.try_into().map_err(Into::into)?;
    let serializer = ValueSerializer::new(signature);
    let value = value.serialize(serializer)?;
    value.try_to_owned()
}

/// Context for all our serializers and provides shared functionality.
pub(crate) struct SerializerCommon<'ser, W> {
    pub(crate) ctxt: Context,
    pub(crate) writer: &'ser mut W,
    pub(crate) bytes_written: usize,
    #[cfg(unix)]
    pub(crate) fds: &'ser mut FdList,
}

#[cfg(unix)]
pub(crate) enum FdList {
    Fds(Vec<OwnedFd>),
    Number(u32),
}

impl<'ser, W> SerializerCommon<'ser, W>
where
    W: Write + Seek,
{
    #[cfg(unix)]
    pub(crate) fn add_fd(&mut self, fd: std::os::fd::RawFd) -> Result<u32> {
        use std::os::fd::{AsRawFd, BorrowedFd};

        match self.fds {
            FdList::Fds(fds) => {
                if let Some(idx) = fds.iter().position(|x| x.as_raw_fd() == fd) {
                    return Ok(idx as u32);
                }
                let idx = fds.len();
                // Cloning implies dup and is unfortunate but we need to return owned fds
                // and dup is not expensive (at least on Linux).
                let fd = unsafe { BorrowedFd::borrow_raw(fd) }.try_clone_to_owned()?;
                fds.push(fd);

                Ok(idx as u32)
            }
            FdList::Number(n) => {
                let idx = *n;
                *n += 1;

                Ok(idx)
            }
        }
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

    fn abs_pos(&self) -> usize {
        self.ctxt.position() + self.bytes_written
    }
}

impl<'ser, W> Write for SerializerCommon<'ser, W>
where
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
