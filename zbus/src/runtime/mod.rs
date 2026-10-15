//! Integration with the async runtime that drives a connection.
//!
//! By default, a connection runs on [zruntime] (the `default-rt` feature, on by default):
//! one runtime per thread that runs [`block_on`](crate::block_on), driven by that thread, and by
//! a helper thread only for work left with no thread inside `block_on`. A connection built
//! inside a Tokio runtime runs on it instead (the `tokio` feature, with which `block_on` runs a
//! Tokio runtime itself). Any other runtime reaches a connection through an implementation of
//! [`traits::Runtime`], handed to [`Builder::runtime`].
//! The async locks a connection holds are not part of that trait: they come from zruntime, except
//! on a Tokio build, which uses Tokio's locks instead.
//! [`AsyncDrop`] is the async counterpart of [`Drop`] that zbus's own types implement.
//!
//! [`Builder::runtime`]: crate::connection::Builder::runtime
//! [zruntime]: https://docs.rs/zruntime

pub mod traits;

pub(crate) mod io;
mod io_source;
pub use io_source::{Interest, IoSource};
mod async_drop;
pub use async_drop::AsyncDrop;
#[cfg(feature = "default-rt")]
pub(crate) mod zruntime;
#[cfg(feature = "default-rt")]
pub(crate) use self::zruntime::ZRuntime;
mod erased;
pub(crate) mod locks;
mod task;
pub(crate) use task::Task;
#[cfg(test)]
pub(crate) mod test_runtime;
mod timeout;
#[cfg(feature = "tokio")]
mod tokio_rt;
#[cfg(feature = "tokio")]
use tokio_rt::Tokio;

// Only the `unixexec` and `ibus` transports and, on macOS, the `launchd` one run a program.
#[cfg(all(unix, any(feature = "unixexec", feature = "ibus", target_os = "macos")))]
pub(crate) mod process;
#[cfg(all(test, target_os = "linux", feature = "unixexec"))]
mod process_table;

#[cfg(any(unix, windows))]
use std::path::Path;
#[cfg(all(unix, any(feature = "unixexec", feature = "ibus", target_os = "macos")))]
use std::process::{Command, ExitStatus, Stdio};
use std::{borrow::Cow, future::Future, net::SocketAddr, pin::Pin, sync::Arc};

#[cfg(all(unix, any(feature = "unixexec", feature = "ibus", target_os = "macos")))]
use erased::BoxFuture;
use erased::ErasedRuntime;

use crate::Result;

/// The runtime a connection runs on, chosen once when it is built.
#[derive(Clone, Debug)]
pub(crate) enum Runtime {
    #[cfg(feature = "default-rt")]
    ZRuntime(ZRuntime),
    #[cfg(feature = "tokio")]
    Tokio(Tokio),
    /// A runtime handed to the builder, reached through the object-safe mirrors of the traits.
    External(Arc<dyn ErasedRuntime>),
}

impl Runtime {
    /// The runtime for a connection built without an explicit one.
    ///
    /// Tokio when the `tokio` feature is on and a runtime is current on this thread, otherwise
    /// [zruntime](https://docs.rs/zruntime), when the `default-rt` feature is on. This keeps the
    /// features additive: enabling `tokio` elsewhere in the dependency graph doesn't force every
    /// zbus user into a tokio runtime. Two builds have no default left and report
    /// [`Error::Unsupported`] instead, so that a connection in them has to be given an external
    /// runtime: one with neither feature on, and one with only `tokio` called from a thread where
    /// no Tokio runtime is current.
    ///
    /// [`Error::Unsupported`]: crate::Error::Unsupported
    pub(crate) fn default_for_build() -> Result<Self> {
        #[cfg(feature = "tokio")]
        if let Some(runtime) = Tokio::current() {
            return Ok(Self::Tokio(runtime));
        }
        #[cfg(feature = "default-rt")]
        {
            Ok(Self::ZRuntime(ZRuntime::new()?))
        }
        #[cfg(not(feature = "default-rt"))]
        {
            Err(crate::Error::Unsupported)
        }
    }

    /// The runtime for a connection built with an explicit one.
    ///
    /// Its every operation goes through the erased mirrors of the traits, which is what lets a
    /// connection hold any implementation without being generic over it.
    pub(crate) fn from_external<R>(runtime: R) -> Self
    where
        R: traits::Runtime,
    {
        Self::External(Arc::new(runtime))
    }

    /// Watches `source` for readiness on this runtime.
    ///
    /// Every I/O a connection performs on a socket of its own goes through the registration this
    /// returns, so a socket is only ever driven by the runtime the connection was built with.
    pub(crate) fn register_io_source(&self, source: IoSource) -> std::io::Result<io::Registration> {
        match self {
            #[cfg(feature = "default-rt")]
            Self::ZRuntime(runtime) => {
                traits::Runtime::register_io_source(runtime, source).map(io::Registration::ZRuntime)
            }
            #[cfg(feature = "tokio")]
            Self::Tokio(runtime) => {
                traits::Runtime::register_io_source(runtime, source).map(io::Registration::Tokio)
            }
            Self::External(runtime) => {
                traits::Runtime::register_io_source(runtime, source).map(io::Registration::External)
            }
        }
    }

    /// Spawns a task onto the runtime, under the diagnostic name `name`.
    ///
    /// `name` takes a `Cow` rather than a `&str` so that a caller that already owns a `String`
    /// (formatted for the occasion, say) can hand it over instead of paying for another copy of
    /// it; a literal stays borrowed and costs nothing extra.
    pub(crate) fn spawn<T>(
        &self,
        name: impl Into<Cow<'static, str>>,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Task<T>
    where
        T: Send + 'static,
    {
        let name = name.into();
        match self {
            // The zruntime backend takes the `Cow` itself rather than going through
            // `traits::Runtime`, whose `spawn` is public API and takes `&str`.
            #[cfg(feature = "default-rt")]
            Self::ZRuntime(runtime) => Task::ZRuntime(runtime.spawn_named(name, future)),
            #[cfg(feature = "tokio")]
            Self::Tokio(runtime) => Task::Tokio(traits::Runtime::spawn(runtime, &name, future)),
            Self::External(runtime) => {
                Task::External(traits::Runtime::spawn(runtime, &name, future))
            }
        }
    }

    /// Connects a stream socket to the TCP endpoint at `address`, the way this runtime does it.
    ///
    /// The socket comes back connected and non-blocking, and no longer watched by the runtime, so
    /// that the connection can register it for its own traffic.
    pub(crate) async fn connect_tcp(&self, address: SocketAddr) -> std::io::Result<IoSource> {
        match self {
            #[cfg(feature = "default-rt")]
            Self::ZRuntime(runtime) => traits::Runtime::connect_tcp(runtime, address).await,
            #[cfg(feature = "tokio")]
            Self::Tokio(runtime) => traits::Runtime::connect_tcp(runtime, address).await,
            Self::External(runtime) => traits::Runtime::connect_tcp(runtime, address).await,
        }
    }

    /// Connects a stream socket to the unix-domain socket at `path`, the way this runtime does it.
    ///
    /// A path whose first byte is zero names an abstract socket on Linux and Android. The socket
    /// comes back as [`Runtime::connect_tcp`]'s does. WASI has no unix-domain sockets.
    #[cfg(any(unix, windows))]
    pub(crate) async fn connect_unix(&self, path: &Path) -> std::io::Result<IoSource> {
        match self {
            #[cfg(feature = "default-rt")]
            Self::ZRuntime(runtime) => traits::Runtime::connect_unix(runtime, path).await,
            #[cfg(feature = "tokio")]
            Self::Tokio(runtime) => traits::Runtime::connect_unix(runtime, path).await,
            Self::External(runtime) => traits::Runtime::connect_unix(runtime, path).await,
        }
    }

    /// Runs `work` off the event loop, on whatever this runtime keeps for blocking work.
    ///
    /// The work is handed to the runtime here and not on the first poll, and the future this
    /// hands back is the runtime's own and borrows nothing from here: whoever wants the outcome
    /// awaits it, and whoever does not may drop it, with the work under way either way.
    pub(crate) fn spawn_blocking<T>(
        &self,
        work: impl FnOnce() -> T + Send + 'static,
    ) -> Pin<Box<dyn Future<Output = T> + Send + 'static>>
    where
        T: Send + 'static,
    {
        match self {
            #[cfg(feature = "default-rt")]
            Self::ZRuntime(runtime) => traits::Runtime::spawn_blocking(runtime, work),
            #[cfg(feature = "tokio")]
            Self::Tokio(runtime) => traits::Runtime::spawn_blocking(runtime, work),
            Self::External(runtime) => traits::Runtime::spawn_blocking(runtime, work),
        }
    }

    /// Spawns `command` as a child process, the way this runtime does it, and waits for it.
    ///
    /// `stdin`, `stdout` and `stderr` are the standard streams of the process. The future this
    /// hands back resolves to the exit status once the process has exited. One that is dropped
    /// before it resolves leaves the process to the runtime: see
    /// [`traits::Runtime::spawn_process`].
    #[cfg(all(unix, any(feature = "unixexec", feature = "ibus", target_os = "macos")))]
    pub(crate) fn spawn_process(
        &self,
        command: Command,
        stdin: Stdio,
        stdout: Stdio,
        stderr: Stdio,
    ) -> std::io::Result<BoxFuture<'static, std::io::Result<ExitStatus>>> {
        match self {
            #[cfg(feature = "default-rt")]
            Self::ZRuntime(runtime) => {
                traits::Runtime::spawn_process(runtime, command, stdin, stdout, stderr)
            }
            #[cfg(feature = "tokio")]
            Self::Tokio(runtime) => {
                traits::Runtime::spawn_process(runtime, command, stdin, stdout, stderr)
            }
            Self::External(runtime) => {
                traits::Runtime::spawn_process(runtime, command, stdin, stdout, stderr)
            }
        }
    }
}

#[cfg(test)]
mod tests;
