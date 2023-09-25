use std::ops::Deref;
#[cfg(unix)]
use std::os::fd::OwnedFd;

use byteorder::ByteOrder;

use crate::EncodingContext;

/// Represents size of serialized bytes in a specific format.
///
/// On Unix platforms, it also contains a list of file descriptors, whose indexes are included in
/// the serialized bytes.
#[derive(Debug)]
pub struct Size<B: ByteOrder> {
    size: usize,
    context: EncodingContext<B>,
    #[cfg(unix)]
    fds: Vec<OwnedFd>,
}

impl<B: ByteOrder> Size<B> {
    /// Create a new `EncodedSize` instance.
    pub fn new(size: usize, context: EncodingContext<B>) -> Self {
        Self {
            size,
            context,
            #[cfg(unix)]
            fds: vec![],
        }
    }

    /// Set the file descriptors.
    #[cfg(unix)]
    pub fn set_fds(mut self, fds: Vec<OwnedFd>) -> Self {
        self.fds = fds;
        self
    }

    /// The size of the serialized bytes.
    pub fn size(&self) -> usize {
        self.size
    }

    /// The encoding context.
    pub fn context(&self) -> EncodingContext<B> {
        self.context
    }

    /// Consume `self` and return the file descriptors.
    ///
    /// This method is only available on Unix platforms.
    #[cfg(unix)]
    pub fn into_fds(self) -> Vec<OwnedFd> {
        self.fds
    }

    /// The file descriptors that are references by the serialized bytes.
    ///
    /// This method is only available on Unix platforms.
    #[cfg(unix)]
    pub fn fds(&self) -> &[impl std::os::fd::AsFd] {
        &self.fds
    }
}

impl<B: ByteOrder> Deref for Size<B> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.size
    }
}
