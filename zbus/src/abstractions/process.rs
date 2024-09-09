#[cfg(not(feature = "tokio"))]
use async_process::{unix::CommandExt, Child};
#[cfg(target_os = "macos")]
use std::process::Output;
use std::{ffi::OsStr, io::Error, process::Stdio};
#[cfg(feature = "tokio")]
use tokio::process::Child;

/// A wrapper around the command API of the underlying async runtime.
pub struct Command(
    #[cfg(not(feature = "tokio"))] async_process::Command,
    #[cfg(feature = "tokio")] tokio::process::Command,
);

impl Command {
    /// Constructs a new `Command` for launching the program at path `program`.
    pub fn new<S>(program: S) -> Self
    where
        S: AsRef<OsStr>,
    {
        #[cfg(not(feature = "tokio"))]
        return Self(async_process::Command::new(program));

        #[cfg(feature = "tokio")]
        return Self(tokio::process::Command::new(program));
    }

    /// Sets executable argument.
    ///
    /// Set the first process argument, `argv[0]`, to something other than the
    /// default executable path.
    pub fn arg0<S>(&mut self, arg: S) -> &mut Self
    where
        S: AsRef<OsStr>,
    {
        self.0.arg0(arg);
        self
    }

    /// Adds an argument to pass to the program.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.0.arg(arg);
        self
    }

    /// Adds multiple arguments to pass to the program.
    #[cfg(target_os = "macos")]
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }

    /// Executes the command as a child process, waiting for it to finish and
    /// collecting all of its output.
    #[cfg(target_os = "macos")]
    pub async fn output(&mut self) -> Result<Output, Error> {
        self.0.output().await
    }

    /// Sets configuration for the child process's standard input (stdin) handle.
    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.0.stdin(cfg);
        self
    }

    /// Sets configuration for the child process's standard output (stdout) handle.
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.0.stdout(cfg);
        self
    }

    /// Sets configuration for the child process's standard error (stderr) handle.
    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.0.stderr(cfg);
        self
    }

    /// Executes the command as a child process, returning a handle to it.
    pub fn spawn(&mut self) -> Result<Child, Error> {
        self.0.spawn()
    }
}

/// An asynchronous wrapper around running and getting command output
#[cfg(target_os = "macos")]
pub async fn run<I, S>(program: S, args: I) -> Result<Output, Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(program).args(args).output().await
}
