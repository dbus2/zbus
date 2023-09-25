#[cfg(unix)]
use super::fd::Fd;
use std::{
    borrow::Cow,
    ops::{Bound, Deref, Range, RangeBounds},
    sync::Arc,
};

use byteorder::ByteOrder;

use crate::EncodingContext;

/// Represents serialized bytes in a specific format.
///
/// On Unix platforms, it also contains a list of file descriptors, whose indexes are included in
/// the serialized bytes. By packing them together, we ensure that the file descriptors are never
/// closed before the serialized bytes are dropped.
#[derive(Clone, Debug)]
pub struct Data<'bytes, 'fds, B: ByteOrder> {
    inner: Arc<Inner<'bytes, 'fds>>,
    context: EncodingContext<B>,
    range: Range<usize>,
}

#[derive(Debug)]
pub struct Inner<'bytes, 'fds> {
    bytes: Cow<'bytes, [u8]>,
    #[cfg(unix)]
    fds: Vec<Fd<'fds>>,
    #[cfg(not(unix))]
    _fds: std::marker::PhantomData<&'fds ()>,
}

impl<'bytes, 'fds, B: ByteOrder> Data<'bytes, 'fds, B> {
    /// Create a new `EncodedBytes` instance containing borrowed file descriptors.
    ///
    /// This method is only available on Unix platforms.
    #[cfg(unix)]
    pub fn new_borrowed_fds<T>(
        bytes: T,
        context: EncodingContext<B>,
        fds: Vec<std::os::fd::BorrowedFd<'fds>>,
    ) -> Self
    where
        T: Into<Cow<'bytes, [u8]>>,
    {
        let bytes = bytes.into();
        let range = Range {
            start: 0,
            end: bytes.len(),
        };
        Data {
            inner: Arc::new(Inner {
                bytes,
                fds: fds.into_iter().map(Into::into).collect(),
            }),
            range,
            context,
        }
    }

    /// The serialized bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.inner.bytes[self.range.start..self.range.end]
    }

    /// The encoding context.
    pub fn context(&self) -> EncodingContext<B> {
        self.context
    }

    /// The file descriptors that are references by the serialized bytes.
    ///
    /// This method is only available on Unix platforms.
    #[cfg(unix)]
    pub fn fds(&self) -> &[impl std::os::fd::AsFd + std::fmt::Debug + 'fds] {
        &self.inner.fds
    }

    /// Returns a slice of `self` for the provided range.
    ///
    /// # Panics
    ///
    /// Requires that begin <= end and end <= self.len(), otherwise slicing will panic.
    pub fn slice(&self, range: impl RangeBounds<usize>) -> Data<'bytes, 'fds, B> {
        let len = self.range.end - self.range.start;
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };
        assert!(
            start <= end,
            "range start must not be greater than end: {start:?} > {end:?}",
        );
        assert!(end <= len, "range end out of bounds: {end:?} > {len:?}");

        let context = EncodingContext::new(self.context.format(), self.context.position() + start);
        let range = Range {
            start: self.range.start + start,
            end: self.range.start + end,
        };

        Data {
            inner: self.inner.clone(),
            context,
            range,
        }
    }
}

impl<'bytes, B: ByteOrder> Data<'bytes, 'static, B> {
    /// Create a new `EncodedBytes` instance.
    pub fn new<T>(bytes: T, context: EncodingContext<B>) -> Self
    where
        T: Into<Cow<'bytes, [u8]>>,
    {
        let bytes = bytes.into();
        let range = Range {
            start: 0,
            end: bytes.len(),
        };
        Data {
            inner: Arc::new(Inner {
                bytes,
                #[cfg(unix)]
                fds: vec![],
                #[cfg(not(unix))]
                _fds: std::marker::PhantomData,
            }),
            context,
            range,
        }
    }

    /// Create a new `Data` instance containing owned file descriptors.
    ///
    /// This method is only available on Unix platforms.
    #[cfg(unix)]
    pub fn new_fds<T>(bytes: T, context: EncodingContext<B>, fds: Vec<std::os::fd::OwnedFd>) -> Self
    where
        T: Into<Cow<'bytes, [u8]>>,
    {
        let bytes = bytes.into();
        let range = Range {
            start: 0,
            end: bytes.len(),
        };
        Data {
            inner: Arc::new(Inner {
                bytes,
                fds: fds.into_iter().map(Into::into).collect(),
            }),
            context,
            range,
        }
    }
}

impl<B: ByteOrder> Deref for Data<'_, '_, B> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.bytes()
    }
}

impl<B: ByteOrder, T> AsRef<T> for Data<'_, '_, B>
where
    T: ?Sized,
    for<'bytes, 'fds> <Data<'bytes, 'fds, B> as Deref>::Target: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.deref().as_ref()
    }
}
