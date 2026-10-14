//! The Tokio backend: Tokio's reactor for readiness, its timer, its tasks, its connects, its child
//! processes and its thread pool for blocking work.
//!
//! A socket or a child process of Tokio's own belongs to the runtime that is current where it is
//! made, and zbus may poll the future of a connect or of a wait from a thread with no Tokio
//! runtime current: a program that drives zbus with an executor of its own does. So the runtime
//! the connection was built on is entered for every poll of a connect and of a wait for a child
//! process, and around the spawn of one, and what they make is that runtime's wherever it is
//! polled from.
//!
//! A connect to a TCP or unix-domain socket is Tokio's own non-blocking connect. It waits on the
//! reactor and so occupies no thread, and the socket comes back as a std one that Tokio no longer
//! watches, for the connection to register for its traffic. Tokio turns a connection to a unix
//! listener whose backlog is full away at once on Linux and Android, where a blocking connect
//! would wait for room, so that connect tries again every 20 milliseconds, on Tokio's timer, for
//! as long as the future is awaited.
//!
//! The socket a connection registers is watched through Tokio's `AsyncFd` on unix. On Windows
//! Tokio watches only the sockets of its own types, each of which owns its socket, so the
//! registration there is a `TcpStream` made from a duplicate of the connection's socket handle.
//! The duplicate refers to the same socket as the handle the connection reads and writes through,
//! and the stream is there to tell when that socket is ready. Tokio has no unix-domain sockets on
//! Windows, and mio, which it watches sockets with, has none either. So a unix-domain connect
//! there is `Unsupported`, and so is the registration of a unix-domain socket.
//!
//! On WASI, which needs `--cfg tokio_unstable` for Tokio's sockets, Tokio again watches only the
//! sockets of its own types, and a descriptor cannot be duplicated there. So the registration is a
//! `TcpStream` made from the connection's descriptor itself, which hands it back to std and lets
//! go of it without closing it as the registration goes, while the connection goes on owning it.
//! Tokio's pool for blocking work cannot start a thread there, so blocking work runs as the
//! trait's default has it: on the thread that asks for it. A TCP connect needs preview 2 of WASI:
//! preview 1 cannot create a socket, and only has those that a host hands to a program.
//!
//! A helper process is spawned by Tokio's own `Command` and waited for by Tokio's reactor, which
//! watches a pidfd on Linux, or by its handler for `SIGCHLD` where there is none. Tokio collects a
//! child whose wait is dropped while the process runs only when its driver next returns from
//! waiting for events, on a best-effort basis, and a runtime with nothing else to wake it, a
//! multi-threaded one that is idle, may not do that for as long as it stays so. zbus awaits the
//! wait on a task of the runtime, so the process is collected as it exits. zbus turns Tokio's
//! `process` feature on only where a transport runs a program, so in any other build the spawn is
//! `Unsupported`, and nothing asks for it.

#[cfg(any(unix, windows))]
use std::path::Path;
#[cfg(unix)]
use std::process::{Command, ExitStatus, Stdio};
use std::{
    future::Future,
    io,
    net::SocketAddr,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

#[cfg(windows)]
use socket2::{Domain, SockRef};
#[cfg(unix)]
use tokio::io::unix::AsyncFd;
use tokio::runtime::Handle;

#[cfg(unix)]
use super::io::unix_socket_address;
use super::{Interest, IoSource, traits};
#[cfg(target_os = "wasi")]
use crate::log::warn;

/// The Tokio runtime a connection runs on.
///
/// The handle is taken when the connection is built, so its tasks, timers, blocking work,
/// connects and child processes keep going to that runtime even where the connection is later
/// polled from a thread with no Tokio runtime current.
#[derive(Clone, Debug)]
pub(crate) struct Tokio {
    handle: Handle,
}

impl Tokio {
    /// The runtime current on this thread, or `None` outside one.
    pub(crate) fn current() -> Option<Self> {
        Handle::try_current().ok().map(|handle| Self { handle })
    }
}

impl traits::Runtime for Tokio {
    type RegisteredIoSource = Registration;
    type Sleep = tokio::time::Sleep;
    type Task<T>
        = TokioTask<T>
    where
        T: Send + 'static;

    #[cfg(unix)]
    fn register_io_source(&self, source: IoSource) -> io::Result<Registration> {
        // A registration is made on the runtime of the calling thread, which is not necessarily
        // the one this connection belongs to.
        let _guard = self.handle.enter();

        // SAFETY: `source` is a shared owner of its descriptor, which closes only once every owner
        // is gone, and the `AsyncFd` keeps `source` for as long as it lives. So the descriptor
        // stays open, and is the one `as_raw_fd` returns every time, for the whole registration.
        let async_fd = unsafe { AsyncFd::register(source) }?;

        Ok(Registration(async_fd))
    }

    #[cfg(windows)]
    fn register_io_source(&self, source: IoSource) -> io::Result<Registration> {
        let socket = SockRef::from(&source);
        // A socket's address says which kind of socket it is, and every socket zbus registers is
        // connected, so it has one. Tokio cannot watch a unix-domain socket here: see the module
        // documentation.
        if socket.local_addr()?.domain() == Domain::UNIX {
            return Err(io::Error::from(io::ErrorKind::Unsupported));
        }
        // Tokio's stream owns the socket it watches, so it is given a duplicate of the handle.
        // The socket is in non-blocking mode already, and the mode belongs to the socket, which
        // both handles refer to.
        let stream = std::net::TcpStream::from(socket.try_clone()?);
        // A registration is made on the runtime of the calling thread, which is not necessarily
        // the one this connection belongs to.
        let _guard = self.handle.enter();

        tokio::net::TcpStream::from_std(stream).map(Registration)
    }

    #[cfg(target_os = "wasi")]
    fn register_io_source(&self, source: IoSource) -> io::Result<Registration> {
        use std::os::fd::{AsRawFd, FromRawFd};

        // SAFETY: `source` holds the descriptor open, and the registration keeps it for as long as
        // the stream lives. The stream does not close it: the registration hands it back to std and
        // takes the raw descriptor out of it as it goes. Where Tokio fails to start or to stop
        // watching it, as it does on a runtime that has shut down, Tokio closes it itself, and the
        // clone of `source` that holds it is leaked then, so that nothing closes it a second time.
        let stream = unsafe { std::net::TcpStream::from_raw_fd(source.as_raw_fd()) };
        // A registration is made on the runtime of the calling thread, which is not necessarily
        // the one this connection belongs to.
        let _guard = self.handle.enter();
        let stream = match tokio::net::TcpStream::from_std(stream) {
            Ok(stream) => stream,
            Err(e) => {
                std::mem::forget(source);

                return Err(e);
            }
        };

        Ok(Registration {
            stream: Some(stream),
            source: Some(source),
        })
    }

    fn sleep(&self, duration: Duration) -> tokio::time::Sleep {
        // A timer is armed on the runtime of the calling thread, as a registration is.
        let _guard = self.handle.enter();

        // Tokio times on a clock of its own, which a test can pause and advance by hand, so the
        // wait is handed over as a length for that clock to measure rather than as a deadline
        // read off the standard one.
        tokio::time::sleep(duration)
    }

    fn spawn<T>(&self, name: &str, future: impl Future<Output = T> + Send + 'static) -> TokioTask<T>
    where
        T: Send + 'static,
    {
        TokioTask::new(spawn_named(&self.handle, name, future))
    }

    #[cfg(not(all(target_os = "wasi", target_env = "p1")))]
    fn connect_tcp(
        &self,
        address: SocketAddr,
    ) -> impl Future<Output = io::Result<IoSource>> + Send {
        entered(&self.handle, async move {
            let stream = tokio::net::TcpStream::connect(address).await?.into_std()?;

            Ok(IoSource::new(stream.into()))
        })
    }

    // Preview 1 of WASI cannot create a socket, so Tokio has no connect there.
    #[cfg(all(target_os = "wasi", target_env = "p1"))]
    fn connect_tcp(
        &self,
        _address: SocketAddr,
    ) -> impl Future<Output = io::Result<IoSource>> + Send {
        std::future::ready(Err(io::Error::from(io::ErrorKind::Unsupported)))
    }

    #[cfg(unix)]
    fn connect_unix(&self, path: &Path) -> impl Future<Output = io::Result<IoSource>> + Send {
        let address = unix_socket_address(path).map(tokio::net::unix::SocketAddr::from);

        entered(&self.handle, async move {
            let address = address?;
            let stream = loop {
                match tokio::net::UnixStream::connect_addr(&address).await {
                    // A full backlog is turned away at once, where a blocking connect would wait
                    // in the kernel for room in it, and Tokio does not wait for it: this does.
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        tokio::time::sleep(BACKLOG_INTERVAL).await;
                    }
                    result => break result?,
                }
            };

            Ok(IoSource::new(stream.into_std()?.into()))
        })
    }

    // Tokio has no unix-domain sockets on Windows: see the module documentation.
    #[cfg(windows)]
    fn connect_unix(&self, _path: &Path) -> impl Future<Output = io::Result<IoSource>> + Send {
        std::future::ready(Err(io::Error::from(io::ErrorKind::Unsupported)))
    }

    #[cfg(all(unix, any(feature = "unixexec", feature = "ibus", target_os = "macos")))]
    fn spawn_process(
        &self,
        mut command: Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> io::Result<Pin<Box<dyn Future<Output = io::Result<ExitStatus>> + Send + 'static>>> {
        // Tokio's command is made from the std one and keeps the streams set on it.
        command.stdin(stdin).stdout(stdout).stderr(stderr);
        let mut command = tokio::process::Command::from(command);
        // The spawn is where Tokio registers what watches for the exit of the process: a pidfd
        // with the reactor on Linux, a handler for `SIGCHLD` with the signal driver elsewhere.
        // Both belong to the runtime that is current.
        let child = {
            let _guard = self.handle.enter();
            command.spawn()
        };
        // The command holds the streams it was given for as long as it lives, which is longer
        // than the process needs them: a copy of the write end of a pipe that stays open here is
        // a pipe that never ends for whoever reads it.
        drop(command);
        let mut child = child?;
        let handle = self.handle.clone();

        // The runtime is entered for every poll of the wait: see `entered`. None of the standard
        // streams of the child is `Stdio::piped()`, so it has no pipes of its own for the wait to
        // close or to read.
        Ok(Box::pin(
            async move { entered(&handle, child.wait()).await },
        ))
    }

    // No transport of this build runs a program, so nothing asks for one, and zbus does not turn
    // on Tokio's `process` feature, which the spawn needs.
    #[cfg(all(
        unix,
        not(any(feature = "unixexec", feature = "ibus", target_os = "macos"))
    ))]
    fn spawn_process(
        &self,
        _command: Command,
        _stdin: Stdio,
        _stdout: Stdio,
        _stderr: Stdio,
    ) -> io::Result<Pin<Box<dyn Future<Output = io::Result<ExitStatus>> + Send + 'static>>> {
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }

    // Tokio's pool cannot start a thread on WASI: the trait's default runs the work there.
    #[cfg(not(target_os = "wasi"))]
    fn spawn_blocking<T>(
        &self,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> Pin<Box<dyn Future<Output = T> + Send + 'static>>
    where
        T: Send + 'static,
    {
        let task = self.handle.spawn_blocking(work);

        Box::pin(async move {
            match task.await {
                Ok(value) => value,
                Err(e) if e.is_panic() => std::panic::resume_unwind(e.into_panic()),
                Err(_) => {
                    panic!("the Tokio runtime shut down while zbus was waiting on blocking work")
                }
            }
        })
    }
}

/// A registration on Tokio's reactor.
#[cfg(unix)]
#[derive(Debug)]
pub(crate) struct Registration(AsyncFd<IoSource>);

#[cfg(unix)]
impl traits::PollIo for Registration {
    fn poll_io<T>(
        &self,
        cx: &mut Context<'_>,
        interest: Interest,
        mut operation: impl FnMut() -> io::Result<T>,
    ) -> Poll<io::Result<T>> {
        loop {
            // Tokio hands out a guard for the readiness it has observed and only re-arms the
            // interest once that guard is cleared, so this loop cannot spin.
            let mut guard = match interest {
                Interest::Readable => std::task::ready!(self.0.poll_read_ready(cx))?,
                Interest::Writable => std::task::ready!(self.0.poll_write_ready(cx))?,
            };
            match operation() {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => guard.clear_ready(),
                result => return Poll::Ready(result),
            }
        }
    }
}

/// A registration on Tokio's reactor.
///
/// It holds the `TcpStream` made from a duplicate of the connection's socket handle, which is what
/// Tokio watches.
#[cfg(windows)]
#[derive(Debug)]
pub(crate) struct Registration(tokio::net::TcpStream);

#[cfg(windows)]
impl traits::PollIo for Registration {
    fn poll_io<T>(
        &self,
        cx: &mut Context<'_>,
        interest: Interest,
        operation: impl FnMut() -> io::Result<T>,
    ) -> Poll<io::Result<T>> {
        poll_stream_io(&self.0, cx, interest, operation)
    }
}

/// A registration on Tokio's reactor.
///
/// It holds a `TcpStream` made from the very descriptor the connection owns, which is what Tokio
/// watches, and keeps that descriptor open through the `IoSource` it holds alongside. Both are
/// there from the registration until its drop, which hands the descriptor back to std and lets go
/// of it unclosed.
#[cfg(target_os = "wasi")]
#[derive(Debug)]
pub(crate) struct Registration {
    stream: Option<tokio::net::TcpStream>,
    source: Option<IoSource>,
}

#[cfg(target_os = "wasi")]
impl traits::PollIo for Registration {
    fn poll_io<T>(
        &self,
        cx: &mut Context<'_>,
        interest: Interest,
        operation: impl FnMut() -> io::Result<T>,
    ) -> Poll<io::Result<T>> {
        let stream = self
            .stream
            .as_ref()
            .expect("a registration's stream is only taken as it is dropped");

        poll_stream_io(stream, cx, interest, operation)
    }
}

#[cfg(target_os = "wasi")]
impl Drop for Registration {
    fn drop(&mut self) {
        use std::os::fd::IntoRawFd;

        let (Some(stream), Some(source)) = (self.stream.take(), self.source.take()) else {
            return;
        };
        // Tokio stops watching the descriptor here, and the raw descriptor taken out of the std
        // stream it hands back is let go of: it is the connection's to close.
        match stream.into_std() {
            Ok(stream) => {
                let _ = stream.into_raw_fd();
            }
            Err(e) => {
                warn!("Tokio failed to stop watching a socket: {}", e);
                // Tokio has closed the descriptor as it failed. The clone of the source that holds
                // it is leaked, so that nothing closes it a second time.
                std::mem::forget(source);
            }
        }
    }
}

/// Waits for Tokio's readiness of `stream`, then runs `operation` under its `try_io`.
///
/// `try_io` runs the operation only while Tokio holds readiness for the interest, and clears that
/// readiness when the operation reports `WouldBlock`. The stream is mio's underneath, and mio's
/// poll of a Windows socket reports an event once: it is armed again by a `WouldBlock` that goes
/// through mio's own `try_io`, which Tokio's calls. Neither looks at which handle the operation
/// used. Here it reads or writes the connection's own handle, which refers to the same socket as
/// the stream's on Windows, and is the stream's very descriptor on WASI, so the readiness waited
/// for is that socket's, and so is the `WouldBlock` that clears it and arms mio again.
///
/// A `WouldBlock` sends the loop back to wait for readiness, which is `Pending` until mio reports
/// some, so the loop does not spin.
#[cfg(any(windows, target_os = "wasi"))]
fn poll_stream_io<T>(
    stream: &tokio::net::TcpStream,
    cx: &mut Context<'_>,
    interest: Interest,
    mut operation: impl FnMut() -> io::Result<T>,
) -> Poll<io::Result<T>> {
    let tokio_interest = match interest {
        Interest::Readable => tokio::io::Interest::READABLE,
        Interest::Writable => tokio::io::Interest::WRITABLE,
    };

    loop {
        std::task::ready!(match interest {
            Interest::Readable => stream.poll_read_ready(cx),
            Interest::Writable => stream.poll_write_ready(cx),
        })?;
        match stream.try_io(tokio_interest, &mut operation) {
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
            result => return Poll::Ready(result),
        }
    }
}

/// Awaits `future`, polling it with the runtime of `handle` entered.
///
/// A socket, a timer or a child of Tokio's own belongs to the runtime that is current at the moment
/// it is made, or registered again. For a connect that is the first poll, which makes the socket,
/// and the poll after a connection that a full backlog turned away, which arms a timer. For a wait
/// for a child it is a poll that finds the pidfd readable before the process can be collected,
/// which registers the pidfd again. The guard lives for one poll and never across an await, so this
/// future is `Send` whenever `future` is.
///
/// Preview 1 of WASI has neither connects nor children, so nothing there needs this.
#[cfg(not(all(target_os = "wasi", target_env = "p1")))]
async fn entered<F>(handle: &Handle, future: F) -> F::Output
where
    F: Future,
{
    use std::{future::poll_fn, pin::pin};

    let mut future = pin!(future);

    poll_fn(|cx| {
        let _guard = handle.enter();

        future.as_mut().poll(cx)
    })
    .await
}

/// How long a unix connect that a full listen backlog turned away waits before it is tried again.
#[cfg(unix)]
const BACKLOG_INTERVAL: Duration = Duration::from_millis(20);

/// Spawns `future` on `handle`, named where Tokio has somewhere to keep a name.
///
/// A task's name is what tokio-console and a task dump show it by, and the builder that takes
/// one only exists with `--cfg tokio_unstable` set *and* Tokio's own `tracing` feature on, which
/// zbus's `tracing` feature is what turns on. Anywhere else the name has nowhere to go.
fn spawn_named<F>(handle: &Handle, name: &str, future: F) -> tokio::task::JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    {
        tokio::task::Builder::new()
            .name(name)
            .spawn_on(future, handle)
            .expect("`Builder::spawn_on` never returns `Err`")
    }
    #[cfg(not(all(tokio_unstable, feature = "tracing")))]
    {
        // Tokio's plain spawn takes no name, so this one goes no further than here.
        let _ = name;

        handle.spawn(future)
    }
}

/// A tokio task handle that aborts the task when dropped, matching `async_task::Task`.
#[derive(Debug)]
pub(crate) struct TokioTask<T>(Option<tokio::task::JoinHandle<T>>);

impl<T> TokioTask<T> {
    pub(super) fn new(handle: tokio::task::JoinHandle<T>) -> Self {
        Self(Some(handle))
    }
}

impl<T> traits::TaskHandle<T> for TokioTask<T>
where
    T: Send + 'static,
{
    fn detach(mut self) {
        // Dropping a tokio `JoinHandle` detaches it.
        drop(self.0.take());
    }
}

impl<T> Drop for TokioTask<T> {
    fn drop(&mut self) {
        if let Some(handle) = self.0.take() {
            handle.abort();
        }
    }
}

impl<T> Future for TokioTask<T> {
    type Output = io::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let handle = self
            .get_mut()
            .0
            .as_mut()
            .expect("a `TokioTask` is only polled before it is detached or dropped");
        Pin::new(handle).poll(cx).map(|r| match r {
            Ok(v) => Ok(v),
            Err(e) => {
                if e.is_cancelled() {
                    Err(io::Error::other("tokio::task cancelled"))
                } else {
                    panic!("tokio::task::JoinHandle error: {e}")
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use ntest::timeout;

    use super::*;
    use crate::runtime::traits::Runtime as _;

    #[tokio::test]
    #[timeout(15000)]
    async fn a_spawned_task_hands_its_output_back() {
        let runtime = Tokio::current().expect("a Tokio runtime is current");
        let task = runtime.spawn("a task with an output", async { 42 });

        assert_eq!(task.await.unwrap(), 42);
    }

    /// A TCP connect polled from a thread with no Tokio runtime current is made on the runtime the
    /// backend belongs to.
    ///
    /// Tokio's own connect panics where it finds no runtime to register its socket with, so the
    /// connect succeeding is what shows the runtime was entered for it.
    #[test]
    #[timeout(15000)]
    fn a_tcp_connect_polled_outside_the_runtime_is_made_on_it() {
        use std::net::{Ipv4Addr, TcpListener};

        use socket2::SockRef;

        let (_tokio, runtime) = runtime_and_backend();
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();

        let source =
            futures_lite::future::block_on(runtime.connect_tcp(listener.local_addr().unwrap()))
                .unwrap();

        // The address the listener sees the connection come from is the socket's own.
        let (_accepted, peer) = listener.accept().unwrap();
        assert_eq!(
            SockRef::from(&source).local_addr().unwrap().as_socket(),
            Some(peer),
        );
    }

    /// A unix connect polled from a thread with no Tokio runtime current is made on the runtime
    /// the backend belongs to.
    #[cfg(unix)]
    #[test]
    #[timeout(15000)]
    fn a_unix_connect_polled_outside_the_runtime_is_made_on_it() {
        use std::{io::Write, mem::MaybeUninit, os::unix::net::UnixListener};

        use socket2::SockRef;

        let (_tokio, runtime) = runtime_and_backend();
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("socket");
        let listener = UnixListener::bind(&path).unwrap();

        let source = futures_lite::future::block_on(runtime.connect_unix(&path)).unwrap();

        // A byte written at the other end that comes out of the socket is what says the two ends
        // are joined, since neither end of a connection to a path names the other.
        let (mut accepted, _) = listener.accept().unwrap();
        accepted.write_all(b"!").unwrap();
        let mut byte = [MaybeUninit::uninit(); 1];
        assert_eq!(SockRef::from(&source).recv(&mut byte).unwrap(), 1);
    }

    /// A unix connect to a listener with no room left for it, polled from a thread with no Tokio
    /// runtime current, waits on the timer of the runtime the backend belongs to.
    ///
    /// Tokio turns the connection away, and the backend arms a timer to try again: a timer, like a
    /// socket, panics where it finds no runtime.
    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[test]
    #[timeout(15000)]
    fn a_unix_connect_to_a_full_backlog_polled_outside_the_runtime_waits_on_it() {
        use socket2::{Domain, SockAddr, SockRef, Socket, Type};

        let (_tokio, runtime) = runtime_and_backend();
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("socket");
        let listener = Socket::new(Domain::UNIX, Type::STREAM, None).unwrap();
        listener.bind(&SockAddr::unix(&path).unwrap()).unwrap();
        listener.listen(0).unwrap();

        // The listener's one place is taken from here on, so a further connection has to wait
        // until it accepts this one.
        let _queued = futures_lite::future::block_on(runtime.connect_unix(&path)).unwrap();
        let mut pending = std::pin::pin!(runtime.connect_unix(&path));
        assert!(
            futures_lite::future::block_on(futures_lite::future::poll_once(pending.as_mut()))
                .is_none(),
            "the connection was made before the listener had room for it",
        );

        let _accepted = listener.accept().unwrap();
        let source = futures_lite::future::block_on(pending).unwrap();

        assert!(SockRef::from(&source).peer_addr().is_ok());
    }

    /// A process spawned, and waited for, from a thread with no Tokio runtime current is spawned
    /// on the runtime the backend belongs to.
    ///
    /// The process runs for a moment, so the first poll of the wait finds it still running and the
    /// runtime's reactor is what wakes the thread once it has exited.
    #[cfg(all(unix, any(feature = "unixexec", feature = "ibus", target_os = "macos")))]
    #[test]
    #[timeout(15000)]
    fn a_process_spawned_and_waited_for_outside_the_runtime_is_made_on_it() {
        use std::process::{Command, Stdio};

        let (_tokio, runtime) = runtime_and_backend();
        let mut command = Command::new("sh");
        command.args(["-c", "sleep 0.2; exit 3"]);

        let exit = runtime
            .spawn_process(command, Stdio::null(), Stdio::null(), Stdio::null())
            .unwrap();
        let status = futures_lite::future::block_on(exit).unwrap();

        assert_eq!(status.code(), Some(3));
    }

    /// A Tokio runtime that runs with nothing inside `block_on`, and the backend for it.
    ///
    /// The thread of the test has no Tokio runtime current, which is what makes a future of the
    /// backend polled on it one that runs outside the runtime, as a program that drives zbus with
    /// an executor of its own polls it. The runtime has to be kept for as long as the backend is
    /// used: its threads are what drive the reactor and the timer.
    fn runtime_and_backend() -> (tokio::runtime::Runtime, Tokio) {
        let tokio = tokio::runtime::Runtime::new().unwrap();
        let backend =
            tokio.block_on(async { Tokio::current().expect("a Tokio runtime is current") });
        assert!(
            Tokio::current().is_none(),
            "the test's thread is inside a Tokio runtime",
        );

        (tokio, backend)
    }
}
