use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::task;

pub(crate) const FDS_MAX: usize = 1024; // this is hardcoded in sdbus - nothing in the spec

pub(crate) fn padding_for_8_bytes(value: usize) -> usize {
    padding_for_n_bytes(value, 8)
}

pub(crate) fn padding_for_n_bytes(value: usize, align: usize) -> usize {
    let len_rounded_up = value.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1);

    len_rounded_up.wrapping_sub(value)
}

/// Helper trait for macro-generated code.
///
/// This trait allows macros to refer to the `Ok` and `Err` types of a [Result] that is behind a
/// type alias.  This is currently required because the macros for properties expect a Result
/// return value, but the macro-generated `receive_` functions need to refer to the actual
/// type without the associated error.
pub trait ResultAdapter {
    type Ok;
    type Err;
}

impl<T, E> ResultAdapter for Result<T, E> {
    type Ok = T;
    type Err = E;
}

thread_local! {
    static RUNNING_ASYNC: Cell<bool> = Cell::new(false);
}

pin_project_lite::pin_project! {
    struct RunAsync<F> {
        #[pin]
        future: F,
    }
}

struct RestoreRunAsyncOnDrop(bool);

impl Drop for RestoreRunAsyncOnDrop {
    fn drop(&mut self) {
        RUNNING_ASYNC.with(|v| v.set(self.0));
    }
}

impl<F: Future> Future for RunAsync<F> {
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, ctx: &mut task::Context<'_>) -> task::Poll<F::Output> {
        let _lock = RUNNING_ASYNC.with(|v| RestoreRunAsyncOnDrop(v.replace(true)));
        self.project().future.poll(ctx)
    }
}

pub(crate) fn forbid_blocking<F: Future + Send>(
    future: F,
) -> impl Future<Output = F::Output> + Send {
    RunAsync { future }
}

pub(crate) fn block_on<F: Future>(fut: F) -> F::Output {
    if RUNNING_ASYNC.with(Cell::get) {
        panic!("Attempted to call blocking API from async context");
    }
    async_io::block_on(fut)
}
