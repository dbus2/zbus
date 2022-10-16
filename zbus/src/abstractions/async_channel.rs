//! Our MPMC async channel wrapper.
//!
//! This is a wrapper around the `tokio::sync::mpsc` or `async_channel`, depending on whether tokio
//! feature is enabled or not, respectively. The tokio's implementation is MPSC so we make it MPMC
//! by wrapping the receiver in an Arc.
//!
//! This mod and types are kept public so that our external tests can use this API. We keep all API
//! here hidden from docs.

use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(not(feature = "tokio"))]
use async_channel::TrySendError;
#[cfg(feature = "tokio")]
use futures_core::ready;
#[cfg(feature = "tokio")]
use futures_util::pin_mut;
#[cfg(feature = "tokio")]
use std::{
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};
#[cfg(feature = "tokio")]
use tokio::sync::{mpsc::error::TrySendError, Mutex};

use futures_core::{FusedStream, Stream};
use tracing::debug;

#[doc(hidden)]
pub fn channel<T>(cap: usize) -> (Sender<T>, Receiver<T>) {
    #[cfg(not(feature = "tokio"))]
    {
        let (s, r) = async_channel::bounded(cap);

        (Sender { inner: s }, Receiver { inner: r })
    }

    #[cfg(feature = "tokio")]
    {
        let (s, r) = tokio::sync::mpsc::channel(cap);
        let receiver = Receiver {
            inner: Arc::new(Mutex::new(r)),
            is_terminated: AtomicBool::new(false),
        };

        (Sender { inner: s }, receiver)
    }
}

#[derive(Debug)]
#[doc(hidden)]
pub struct Sender<T> {
    #[cfg(not(feature = "tokio"))]
    inner: async_channel::Sender<T>,
    #[cfg(feature = "tokio")]
    inner: tokio::sync::mpsc::Sender<T>,
}

impl<T> Sender<T> {
    pub async fn send(&self, value: T) -> Option<T> {
        match self.inner.send(value).await {
            Err(e) => {
                // This happens if the channel is being dropped, which only happens when the
                // receive_msg task is running at the time the last Connection is dropped.
                // So it's unlikely that it'd be interesting to the user. Hence debug not
                // warn.
                debug!("Error sending error: {}", e);
                Some(e.0)
            }
            Ok(()) => None,
        }
    }

    #[allow(unused)]
    pub fn try_send(&self, value: T) -> Option<T> {
        match self.inner.try_send(value) {
            Err(e) => {
                // This happens if the channel is being dropped, which only happens when the
                // receive_msg task is running at the time the last Connection is dropped.
                // So it's unlikely that it'd be interesting to the user. Hence debug not
                // warn.
                debug!("Error sending error: {}", e);
                match e {
                    TrySendError::Full(v) => Some(v),
                    TrySendError::Closed(v) => Some(v),
                }
            }
            Ok(()) => None,
        }
    }

    // Used by tests.
    #[allow(unused)]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[cfg(not(feature = "tokio"))]
#[derive(Debug)]
#[doc(hidden)]
pub struct Receiver<T> {
    inner: async_channel::Receiver<T>,
}
#[cfg(feature = "tokio")]
#[derive(Debug)]
#[doc(hidden)]
pub struct Receiver<T> {
    inner: Arc<Mutex<tokio::sync::mpsc::Receiver<T>>>,
    is_terminated: AtomicBool,
}

impl<T> Receiver<T> {
    pub async fn recv(&self) -> Option<T> {
        #[cfg(not(feature = "tokio"))]
        match self.inner.recv().await {
            Err(e) => {
                debug!("Error receiving: {}", e);
                None
            }
            Ok(value) => Some(value),
        }

        #[cfg(feature = "tokio")]
        {
            let item = self.inner.lock().await.recv().await;
            if item.is_none() {
                self.is_terminated.store(true, SeqCst);
            }

            item
        }
    }

    // Used by tests.
    #[allow(unused)]
    pub async fn close(&self) {
        #[cfg(not(feature = "tokio"))]
        {
            self.inner.close();
        }
        #[cfg(feature = "tokio")]
        {
            self.inner.lock().await.close();
            self.is_terminated.store(true, SeqCst);
        }
    }
}

impl<T> Stream for Receiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        #[cfg(not(feature = "tokio"))]
        {
            Pin::new(&mut self.get_mut().inner).poll_next(cx)
        }
        #[cfg(feature = "tokio")]
        {
            let lock_fut = self.inner.lock();
            pin_mut!(lock_fut);
            let mut inner = ready!(lock_fut.poll(cx));
            inner.poll_recv(cx)
        }
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            #[cfg(feature = "tokio")]
            is_terminated: AtomicBool::new(self.is_terminated.load(SeqCst)),
        }
    }
}

impl<T> FusedStream for Receiver<T> {
    fn is_terminated(&self) -> bool {
        #[cfg(not(feature = "tokio"))]
        {
            self.inner.is_terminated()
        }
        #[cfg(feature = "tokio")]
        {
            self.is_terminated.load(SeqCst)
        }
    }
}
