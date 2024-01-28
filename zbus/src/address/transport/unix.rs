use std::{collections::HashMap, ffi::OsString};

/// A Unix domain socket transport in a D-Bus address.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unix {
    path: UnixPath,
}

impl Unix {
    /// Create a new Unix transport with the given path.
    pub fn new(path: UnixPath) -> Self {
        Self { path }
    }

    /// The path.
    pub fn path(&self) -> &UnixPath {
        &self.path
    }

    /// Take the path, consuming `self`.
    pub fn take_path(self) -> UnixPath {
        self.path
    }

    #[cfg(any(unix, not(feature = "tokio")))]
    pub(super) fn from_options(opts: HashMap<&str, &str>) -> crate::Result<Self> {
        let path = opts.get("path");
        let abs = opts.get("abstract");
        let dir = opts.get("dir");
        let tmpdir = opts.get("tmpdir");
        let path = match (path, abs, dir, tmpdir) {
            (Some(p), None, None, None) => UnixPath::File(OsString::from(p)),
            #[cfg(target_os = "linux")]
            (None, Some(p), None, None) => UnixPath::Abstract(p.as_bytes().to_owned()),
            #[cfg(not(target_os = "linux"))]
            (None, Some(_), None, None) => {
                return Err(crate::Error::Address(
                    "abstract sockets currently Linux-only".to_owned(),
                ));
            }
            (None, None, Some(p), None) => UnixPath::Dir(OsString::from(p)),
            (None, None, None, Some(p)) => UnixPath::TmpDir(OsString::from(p)),
            _ => {
                return Err(crate::Error::Address("unix: address is invalid".to_owned()));
            }
        };

        Ok(Self::new(path))
    }
}

/// A Unix domain socket path in a D-Bus address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnixPath {
    /// A path to a unix domain socket on the filesystem.
    File(OsString),
    /// A abstract unix domain socket name.
    #[cfg(target_os = "linux")]
    Abstract(Vec<u8>),
    /// A listenable address using the specified path, in which a socket file with a random file
    /// name starting with 'dbus-' will be created by the server. See [UNIX domain socket address]
    /// reference documentation.
    ///
    /// This address is mostly relevant to server (typically bus broker) implementations.
    ///
    /// [UNIX domain socket address]: https://dbus.freedesktop.org/doc/dbus-specification.html#transports-unix-domain-sockets-addresses
    Dir(OsString),
    /// The same as UnixDir, except that on platforms with abstract sockets, the server may attempt
    /// to create an abstract socket whose name starts with this directory instead of a path-based
    /// socket.
    ///
    /// This address is mostly relevant to server (typically bus broker) implementations.
    TmpDir(OsString),
}
