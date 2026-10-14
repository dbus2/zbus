//! An external runtime, in two layers.
//!
//! [`Builder::runtime`] takes any implementation of the runtime traits and a connection must not
//! be generic over it, so the implementation is boxed once behind the object-safe mirrors of
//! those traits that make up the lower layer here: [`ErasedRuntime`] and one trait per thing a
//! runtime hands out. Everything generic is erased on the way through: what a task produces
//! becomes a `Box<dyn Any + Send>` and a future becomes a trait object of its own.
//!
//! The upper layer puts the public traits back on top of the boxed mirrors: [`ExternalTask`]
//! carries the type of the value again, and [`Arc<dyn ErasedRuntime>`](ErasedRuntime) implements
//! [`traits::Runtime`] itself. That is what lets the rest of the crate treat a runtime it was
//! handed as one more implementation of the traits, reached exactly as the built-in backends are.
//!
//! What the layers cost, all of it on this path alone. A registration costs one allocation and so
//! does a sleep. A spawn costs two, the future and the handle, and a third as the task ends for
//! the value it produced, which a task that produces nothing does not pay. A call to the blocking
//! hook costs three — the work, the value it hands back and the future that downcasts that value
//! — on top of the boxed future the hook returns whichever runtime it is called on. A connect
//! costs the one allocation of the boxed future its method returns, and a spawned process costs
//! none, its method returning that future boxed already.
//!
//! Readiness is mirrored without erasing anything: the operation a registration runs hands its
//! result back through the caller's own captures, so an I/O call costs no allocation here.
//!
//! [`Builder::runtime`]: crate::connection::Builder::runtime

#[cfg(any(unix, windows))]
use std::path::Path;
#[cfg(unix)]
use std::process::{Command, ExitStatus, Stdio};
use std::{
    any::Any,
    fmt,
    future::Future,
    io,
    marker::PhantomData,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use super::{Interest, IoSource, traits};

pub(crate) type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub(crate) trait ErasedRuntime: Send + Sync {
    fn register_io_source(&self, source: IoSource) -> io::Result<Box<dyn ErasedRegistration>>;
    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()>;
    fn spawn(
        &self,
        name: &str,
        future: BoxFuture<'static, Box<dyn Any + Send>>,
    ) -> Box<dyn ErasedTask>;
    fn connect_tcp(&self, address: SocketAddr) -> BoxFuture<'_, io::Result<IoSource>>;
    #[cfg(any(unix, windows))]
    fn connect_unix<'a>(&'a self, path: &'a Path) -> BoxFuture<'a, io::Result<IoSource>>;
    #[cfg(unix)]
    fn spawn_process(
        &self,
        command: Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> io::Result<BoxFuture<'static, io::Result<ExitStatus>>>;
    fn spawn_blocking(
        &self,
        work: Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>,
    ) -> BoxFuture<'static, Box<dyn Any + Send>>;
}

impl fmt::Debug for dyn ErasedRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<external runtime>")
    }
}

impl<R> ErasedRuntime for R
where
    R: traits::Runtime,
{
    fn register_io_source(&self, source: IoSource) -> io::Result<Box<dyn ErasedRegistration>> {
        traits::Runtime::register_io_source(self, source)
            .map(|registration| Box::new(registration) as Box<dyn ErasedRegistration>)
    }

    fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()> {
        Box::pin(traits::Runtime::sleep(self, duration))
    }

    fn spawn(
        &self,
        name: &str,
        future: BoxFuture<'static, Box<dyn Any + Send>>,
    ) -> Box<dyn ErasedTask> {
        Box::new(traits::Runtime::spawn(self, name, future))
    }

    fn connect_tcp(&self, address: SocketAddr) -> BoxFuture<'_, io::Result<IoSource>> {
        Box::pin(traits::Runtime::connect_tcp(self, address))
    }

    #[cfg(any(unix, windows))]
    fn connect_unix<'a>(&'a self, path: &'a Path) -> BoxFuture<'a, io::Result<IoSource>> {
        Box::pin(traits::Runtime::connect_unix(self, path))
    }

    #[cfg(unix)]
    fn spawn_process(
        &self,
        command: Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> io::Result<BoxFuture<'static, io::Result<ExitStatus>>> {
        traits::Runtime::spawn_process(self, command, stdin, stdout, stderr)
    }

    fn spawn_blocking(
        &self,
        work: Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>,
    ) -> BoxFuture<'static, Box<dyn Any + Send>> {
        traits::Runtime::spawn_blocking(self, work)
    }
}

/// A boxed runtime is a runtime again, so that a connection on one dispatches as it does on a
/// backend of its own: through [`traits::Runtime`], whichever runtime it was built with.
impl traits::Runtime for Arc<dyn ErasedRuntime> {
    type RegisteredIoSource = Box<dyn ErasedRegistration>;
    type Sleep = BoxFuture<'static, ()>;
    type Task<T>
        = ExternalTask<T>
    where
        T: Send + 'static;

    fn register_io_source(&self, source: IoSource) -> io::Result<Self::RegisteredIoSource> {
        ErasedRuntime::register_io_source(&**self, source)
    }

    fn sleep(&self, duration: Duration) -> Self::Sleep {
        ErasedRuntime::sleep(&**self, duration)
    }

    fn spawn<T>(
        &self,
        name: &str,
        future: impl Future<Output = T> + Send + 'static,
    ) -> ExternalTask<T>
    where
        T: Send + 'static,
    {
        ExternalTask::spawn(&**self, name, future)
    }

    fn connect_tcp(
        &self,
        address: SocketAddr,
    ) -> impl Future<Output = io::Result<IoSource>> + Send {
        ErasedRuntime::connect_tcp(&**self, address)
    }

    // An `async fn` rather than the mirror's boxed future returned as it is: the mirror ties both
    // of its borrows to one lifetime, which a returned future that names two lifetimes of its own
    // cannot be, but a future that holds both can.
    #[cfg(any(unix, windows))]
    async fn connect_unix(&self, path: &Path) -> io::Result<IoSource> {
        ErasedRuntime::connect_unix(&**self, path).await
    }

    #[cfg(unix)]
    fn spawn_process(
        &self,
        command: Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> io::Result<BoxFuture<'static, io::Result<ExitStatus>>> {
        ErasedRuntime::spawn_process(&**self, command, stdin, stdout, stderr)
    }

    fn spawn_blocking<T>(&self, work: impl FnOnce() -> T + Send + 'static) -> BoxFuture<'static, T>
    where
        T: Send + 'static,
    {
        // The erased runtime only takes work that produces an opaque value, so the result comes
        // back to be downcast to what `work` returned.
        let work = Box::new(move || Box::new(work()) as Box<dyn Any + Send>);
        let outcome = ErasedRuntime::spawn_blocking(&**self, work);

        Box::pin(async move {
            *outcome
                .await
                .downcast()
                .expect("blocking work hands back the value it produced")
        })
    }
}

/// The object-safe mirror of [`traits::PollIo`].
///
/// The operation returns nothing, so that nothing has to be boxed for it: the typed caller wraps
/// its own operation in a closure that keeps the value it produced.
pub(crate) trait ErasedRegistration: Send + Sync {
    fn poll_io(
        &self,
        cx: &mut Context<'_>,
        interest: Interest,
        operation: &mut dyn FnMut() -> io::Result<()>,
    ) -> Poll<io::Result<()>>;
}

impl fmt::Debug for dyn ErasedRegistration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<external registration>")
    }
}

impl<R> ErasedRegistration for R
where
    R: traits::PollIo,
{
    fn poll_io(
        &self,
        cx: &mut Context<'_>,
        interest: Interest,
        operation: &mut dyn FnMut() -> io::Result<()>,
    ) -> Poll<io::Result<()>> {
        traits::PollIo::poll_io(self, cx, interest, operation)
    }
}

impl traits::PollIo for Box<dyn ErasedRegistration> {
    fn poll_io<T>(
        &self,
        cx: &mut Context<'_>,
        interest: Interest,
        mut operation: impl FnMut() -> io::Result<T>,
    ) -> Poll<io::Result<T>> {
        // The mirror hands nothing back, so what the operation produced travels in a local the
        // wrapping closure fills in.
        let mut value = None;
        let ready = ErasedRegistration::poll_io(&**self, cx, interest, &mut || {
            value = Some(operation()?);

            Ok(())
        });

        match ready {
            Poll::Ready(result) => Poll::Ready(result.map(|()| {
                value
                    .take()
                    .expect("a successful operation produced a value")
            })),
            Poll::Pending => {
                // A value taken off the socket and then dropped here is bytes gone from the
                // stream, which surfaces much later as a message that will not parse, so this
                // one holds wherever it runs.
                assert!(
                    value.is_none(),
                    "a registration must not report `Pending` for an operation that produced a \
                     value",
                );

                Poll::Pending
            }
        }
    }
}

pub(crate) trait ErasedTask:
    Future<Output = io::Result<Box<dyn Any + Send>>> + Send + Sync + Unpin
{
    fn detach(self: Box<Self>);
}

impl fmt::Debug for dyn ErasedTask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<external task>")
    }
}

impl<H> ErasedTask for H
where
    H: traits::TaskHandle<Box<dyn Any + Send>>,
{
    fn detach(self: Box<Self>) {
        traits::TaskHandle::detach(*self)
    }
}

/// A task on an external runtime, carrying the type of what that task produces.
///
/// The mirror hands every output back as an opaque value, and this is where it becomes the type
/// the caller spawned again.
pub(crate) struct ExternalTask<T> {
    inner: Box<dyn ErasedTask>,
    // `fn() -> T` carries none of `T`'s auto traits, leaving them to `inner`; what makes that
    // sound is `Runtime::spawn`'s `T: Send`.
    value: PhantomData<fn() -> T>,
}

impl<T> ExternalTask<T>
where
    T: Send + 'static,
{
    /// Spawns `future` on `runtime`, boxing what it produces on the way out.
    ///
    /// That box is made as the task ends and undone as the output is read, so a spawn pays for
    /// it once and a task that produces nothing does not pay for it at all.
    pub(crate) fn spawn(
        runtime: &dyn ErasedRuntime,
        name: &str,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Self {
        let future = Box::pin(async move { Box::new(future.await) as Box<dyn Any + Send> });

        Self {
            inner: runtime.spawn(name, future),
            value: PhantomData,
        }
    }
}

impl<T> fmt::Debug for ExternalTask<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl<T> Future for ExternalTask<T>
where
    T: 'static,
{
    type Output = io::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.get_mut().inner)
            .poll(cx)
            .map_ok(|value| *value.downcast().expect(PRODUCED))
    }
}

impl<T> traits::TaskHandle<T> for ExternalTask<T>
where
    T: Send + 'static,
{
    fn detach(self) {
        ErasedTask::detach(self.inner)
    }
}

const PRODUCED: &str = "a task hands back the value its future produced";
