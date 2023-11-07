use std::ops::Deref;

use byteorder::ByteOrder;

use crate::EncodingContext;

/// Represents the return value of [`crate::serialized_size`] function.
///
/// It mainly contains the size of serialized bytes in a specific format.
///
/// On Unix platforms, it also contains the number of file descriptors, whose indexes are included
/// in the serialized bytes.
#[derive(Debug)]
pub struct Size<B: ByteOrder> {
    size: usize,
    context: EncodingContext<B>,
    #[cfg(unix)]
    num_fds: u32,
}

impl<B: ByteOrder> Size<B> {
    /// Create a new `EncodedSize` instance.
    pub fn new(size: usize, context: EncodingContext<B>) -> Self {
        Self {
            size,
            context,
            #[cfg(unix)]
            num_fds: 0,
        }
    }

    /// Set the number of file descriptors.
    #[cfg(unix)]
    pub fn set_num_fds(mut self, num_fds: u32) -> Self {
        self.num_fds = num_fds;
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

    /// The number file descriptors that are references by the serialized bytes.
    ///
    /// This method is only available on Unix platforms.
    #[cfg(unix)]
    pub fn num_fds(&self) -> u32 {
        self.num_fds
    }
}

impl<B: ByteOrder> Deref for Size<B> {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.size
    }
}
