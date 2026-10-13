//! The zruntime backend: [zruntime]'s reactor for readiness, its timer, its tasks, its connects,
//! its child processes and the standard hook for blocking work.
//!
//! A connection to a TCP or unix-domain socket is made by zruntime's own non-blocking connect,
//! which waits on its reactor and so occupies no thread. The socket comes back as a std one, which
//! zruntime no longer watches, for the connection to register for its traffic. zruntime has no
//! unix-domain sockets on Windows, so a unix-domain connect there is a blocking one, made on a
//! thread of the pool for blocking work.
//!
//! A helper process is spawned by zruntime's own [`Command`] and waited for by its reactor, which
//! watches a pidfd on Linux and a kqueue of the process's own on Apple's platforms and the BSDs. A
//! thread of a pool that zruntime keeps for the purpose waits where there is nothing to watch.
//! zruntime itself collects a process whose wait is dropped while it runs, once it has exited, and
//! does not kill it. zbus does not rely on that, as it awaits the wait on a task of the runtime.
//!
//! On WASI, which cannot start a thread, zruntime has no helper thread to run a connection's tasks
//! while no thread is inside [`block_on`](crate::block_on), and its pool for blocking work runs the
//! work on the thread that hands it over. So a connection there makes progress only inside
//! `block_on`. A TCP connect needs preview 2 of WASI: preview 1 cannot create a socket, and
//! zruntime's connect fails there with `Unsupported`.
//!
//! [zruntime]: https://docs.rs/zruntime
//! [`Command`]: zruntime::process::Command

#[cfg(any(unix, windows))]
use std::path::Path;
#[cfg(unix)]
use std::process::{Command, ExitStatus, Stdio};
use std::{
    borrow::Cow,
    fmt,
    future::Future,
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[cfg(unix)]
use super::io::unix_socket_address;
use super::{Interest, IoSource, traits};

/// The runtime zbus brings along by default: a thin wrapper around [`zruntime::SharedRuntime`].
#[derive(Clone, Debug)]
pub(crate) struct ZRuntime(zruntime::SharedRuntime);

impl ZRuntime {
    /// A handle on the runtime for what this thread builds, brought into being here if none is
    /// alive.
    ///
    /// What can fail is the reactor: it opens the channel a wait is broken through.
    pub(crate) fn new() -> io::Result<Self> {
        Ok(Self(zruntime::SharedRuntime::current()?))
    }

    /// Queues `future` under the diagnostic name `name` and hands back the task that joins or
    /// cancels it.
    ///
    /// An inherent method rather than part of [`traits::Runtime`], whose `spawn` is public API
    /// and takes `&str`: this one takes the `Cow` the crate-internal [`super::Runtime::spawn`]
    /// already built, rather than making every caller pay for a fresh copy of a name it already
    /// owns.
    pub(super) fn spawn_named<T>(
        &self,
        name: Cow<'static, str>,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Task<T>
    where
        T: Send + 'static,
    {
        Task(self.0.spawn(name, future))
    }
}

impl traits::Runtime for ZRuntime {
    type RegisteredIoSource = Registration;
    type Sleep = zruntime::Sleep<zruntime::Shared>;
    type Task<T>
        = Task<T>
    where
        T: Send + 'static;

    fn register_io_source(&self, source: IoSource) -> io::Result<Registration> {
        self.0.register(source).map(Registration)
    }

    fn sleep(&self, duration: Duration) -> zruntime::Sleep<zruntime::Shared> {
        self.0.sleep(duration)
    }

    fn spawn<T>(&self, name: &str, future: impl Future<Output = T> + Send + 'static) -> Task<T>
    where
        T: Send + 'static,
    {
        // A copy of its own: this signature is fixed by `traits::Runtime`, whose caller only
        // hands out a borrow, while a task's cell keeps its name for as long as it lives.
        self.spawn_named(name.to_owned().into(), future)
    }

    async fn connect_tcp(&self, address: SocketAddr) -> io::Result<IoSource> {
        let stream = zruntime::net::TcpStream::<zruntime::Shared>::connect(&self.0, address)
            .await?
            .into_std();

        Ok(IoSource::new(stream.into()))
    }

    #[cfg(unix)]
    fn connect_unix(&self, path: &Path) -> impl Future<Output = io::Result<IoSource>> + Send {
        let address = unix_socket_address(path);

        async move {
            let stream = zruntime::net::unix::UnixStream::<zruntime::Shared>::connect_addr(
                &self.0, &address?,
            )
            .await?
            .into_std();

            Ok(IoSource::new(stream.into()))
        }
    }

    // zruntime has no unix-domain sockets on Windows, so the connect is a blocking one, on a
    // thread for blocking work.
    #[cfg(windows)]
    async fn connect_unix(&self, path: &Path) -> io::Result<IoSource> {
        use socket2::{Domain, SockAddr};

        let address = SockAddr::unix(path)?;

        traits::Runtime::spawn_blocking(self, move || {
            super::io::connect_blocking(Domain::UNIX, &address)
        })
        .await
    }

    #[cfg(unix)]
    fn spawn_process(
        &self,
        command: Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> io::Result<Pin<Box<dyn Future<Output = io::Result<ExitStatus>> + Send + 'static>>> {
        // The streams go through zruntime's own builder: its `From<std::process::Command>` cannot
        // see the ones set on the std command, and gives every stream that was not set through
        // the builder the default of the method that spawns, which for `spawn` is to inherit it.
        let mut command = zruntime::process::Command::from(command);
        command.stdin(stdin).stdout(stdout).stderr(stderr);
        let child = command.spawn(&self.0);
        // The command holds the streams it was given for as long as it lives, which is longer
        // than the process needs them: a copy of the write end of a pipe that stays open here is
        // a pipe that never ends for whoever reads it.
        drop(command);
        let mut child = child?;

        // None of the streams is `Stdio::piped()`, so the child has no pipes of its own for the
        // wait to close or to read. zruntime collects a child dropped with the process still
        // running once it exits, and does not kill it, as the command was not asked to.
        Ok(Box::pin(async move { child.status().await }))
    }
}

/// Runs `future` to completion on the calling thread, running that thread's runtime alongside it:
/// see [`zruntime::block_on`].
#[cfg(not(feature = "tokio"))]
pub(crate) fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    zruntime::block_on(future)
}

/// A registration on [zruntime]'s reactor.
///
/// [zruntime]: https://docs.rs/zruntime
#[derive(Debug)]
pub(crate) struct Registration(zruntime::Registration<zruntime::Shared>);

impl traits::PollIo for Registration {
    fn poll_io<T>(
        &self,
        cx: &mut Context<'_>,
        interest: Interest,
        operation: impl FnMut() -> io::Result<T>,
    ) -> Poll<io::Result<T>> {
        self.0.poll_io(cx, zruntime_interest(interest), operation)
    }
}

/// `interest` as zruntime's own.
///
/// A function rather than a `From` impl: `Interest` is public API, and a `From` impl between it
/// and zruntime's would be too, making zruntime's type part of zbus's API.
fn zruntime_interest(interest: Interest) -> zruntime::Interest {
    // `Interest` is `non_exhaustive` for zbus's own callers, not for this crate, so an
    // exhaustive match is fine here.
    match interest {
        Interest::Readable => zruntime::Interest::Readable,
        Interest::Writable => zruntime::Interest::Writable,
    }
}

/// A task spawned on zruntime, which cancels that task when dropped.
pub(crate) struct Task<T>(zruntime::Task<T, zruntime::Shared>);

impl<T> fmt::Debug for Task<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Task").field(&self.0).finish()
    }
}

impl<T> Future for Task<T> {
    type Output = io::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.get_mut().0).poll(cx)
    }
}

impl<T> traits::TaskHandle<T> for Task<T>
where
    T: Send + 'static,
{
    fn detach(self) {
        self.0.detach();
    }
}

#[cfg(test)]
mod tests {
    use ntest::timeout;

    use super::*;
    use crate::runtime::traits::Runtime as _;

    #[test]
    #[timeout(15000)]
    fn a_spawned_task_hands_its_output_back() {
        let runtime = ZRuntime::new().expect("a runtime for this thread");
        let task = runtime.spawn("an answer", async { 42 });

        assert_eq!(zruntime::block_on(task).unwrap(), 42);
    }
}
