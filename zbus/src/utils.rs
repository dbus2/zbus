use std::{fmt, future::poll_fn, pin::Pin};

use futures_core::Stream;

#[cfg(unix)]
pub(crate) const FDS_MAX: usize = 1024; // this is hardcoded in sdbus - nothing in the spec

pub(crate) fn padding_for_8_bytes(value: usize) -> usize {
    padding_for_n_bytes(value, 8)
}

pub(crate) fn padding_for_n_bytes(value: usize, align: usize) -> usize {
    let len_rounded_up = value.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);

    len_rounded_up.wrapping_sub(value)
}

/// Formats a [`bitflags`] type declared with the `impl` form of `bitflags!` for its `Debug`
/// implementation.
///
/// The `impl` form leaves `Debug` to the type, so this writes what the `struct` form of
/// `bitflags!` generates: `Name(A | B)`, any bits that no flag defines as a trailing hex number
/// (`Name(A | 0x8)`), and `Name(0x0)` for an empty set.
pub(crate) fn fmt_bitflags<F>(name: &str, flags: &F, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    F: bitflags::Flags,
    F::Bits: bitflags::parser::WriteHex,
{
    write!(f, "{name}(")?;
    if flags.is_empty() {
        f.write_str("0x0")?;
    } else {
        bitflags::parser::to_writer(flags, &mut *f)?;
    }
    f.write_str(")")
}

/// The next item of `stream`, or `None` once it has ended.
pub(crate) async fn next<S>(stream: &mut S) -> Option<S::Item>
where
    S: Stream + Unpin,
{
    poll_fn(|cx| Pin::new(&mut *stream).poll_next(cx)).await
}

/// Helper trait for macro-generated code.
///
/// This trait allows macros to refer to the `Ok` and `Err` types of a [Result] that is behind a
/// type alias.  This is currently required because the macros for properties expect a Result
/// return value, but the macro-generated `receive_` functions need to refer to the actual
/// type without the associated error.
#[doc(hidden)]
pub trait ResultAdapter {
    type Ok;
    type Err;
}

impl<T, E> ResultAdapter for Result<T, E> {
    type Ok = T;
    type Err = E;
}

/// Runs a future to completion on the calling thread, and [zruntime](https://docs.rs/zruntime)
/// with it.
///
/// This is for a program that has no async runtime of its own. Put the async code in one
/// call to this function; the call blocks the thread until the future completes. In
/// between polls of the future, the calling thread also runs zruntime's scheduler and I/O
/// reactor for every connection built in it, and two threads that each call this drive
/// their own connections, in parallel. A handful of blocking system calls — a DNS lookup,
/// a nonce-file read, a peer-credential lookup — run on a short-lived worker thread of
/// their own instead, so they never hold up the calling thread. If the call returns while
/// a connection is still alive, a helper thread takes over that connection's work until
/// the next call, or until the connection is gone.
///
/// Because the connections' work runs on the calling thread in between polls, the future must
/// not block that thread waiting for it. A synchronous wait for a reply, or a busy loop until a
/// signal arrives, never finishes.
///
/// Do not call this from inside a task zbus is running, such as a method of a served interface
/// or a future inside another `block_on` call. It panics there, because it would be waiting for
/// the very thread it is on. Do not call it from another runtime's task either: it blocks that
/// task's thread until the future completes, which deadlocks the program if the future needs
/// that thread to make progress.
#[cfg(all(feature = "default-rt", not(feature = "tokio")))]
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    crate::runtime::zruntime::block_on(future)
}

/// Runs a future to completion on the calling thread.
///
/// In a build with neither `default-rt` nor `tokio` enabled, every connection runs on the
/// runtime given to [`Builder::runtime`]. This call drives no connection itself: it only polls
/// the future passed in, blocking the calling thread until it is done.
///
/// Do not call this from another runtime's task. It blocks that task's thread until the future
/// completes, which deadlocks the program if the future needs that thread to make progress.
///
/// [`Builder::runtime`]: crate::connection::Builder::runtime
#[cfg(not(any(feature = "default-rt", feature = "tokio")))]
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    futures_lite::future::block_on(future)
}

/// Runs a future to completion, blocking the calling thread until it is done.
///
/// This is for a program that has no async runtime of its own. Put the async code in one call to
/// this function; the call blocks the thread until the future completes. With the `tokio`
/// feature, the call runs inside a Tokio runtime that zbus creates on first use and keeps for
/// the rest of the process, so a connection built inside the future runs on Tokio as well.
///
/// That runtime is single-threaded: the tasks of the connections on it, such as reading incoming
/// messages and serving method calls, only run while a thread is inside this function, and on
/// one such thread at a time. With the `tokio-multithread` feature, it is a multi-threaded
/// runtime instead, whose worker threads keep running those tasks in parallel, whether or not
/// any thread is inside this function.
///
/// Do not call this from another runtime's task. From inside a Tokio async task it panics, as
/// Tokio's own `block_on` does. From any other runtime's task it blocks that task's thread until
/// the future completes, which deadlocks the program if the future needs that thread to make
/// progress.
#[cfg(feature = "tokio")]
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use std::sync::OnceLock;

    static TOKIO_RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

    TOKIO_RT
        .get_or_init(|| {
            #[cfg(feature = "tokio-multithread")]
            let mut builder = tokio::runtime::Builder::new_multi_thread();
            #[cfg(not(feature = "tokio-multithread"))]
            let mut builder = tokio::runtime::Builder::new_current_thread();

            builder
                .enable_io()
                .enable_time()
                .build()
                .expect("launch of tokio runtime")
        })
        .block_on(future)
}
