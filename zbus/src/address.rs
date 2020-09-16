use crate::{Error, Result};
use nix::sys::socket::{self, AddressFamily, SockAddr, SockFlag, SockType, UnixAddr};
use std::os::unix::io::FromRawFd;
use std::os::unix::net::UnixStream;
use std::str::FromStr;

/// A bus address
#[derive(Debug, PartialEq)]
pub(crate) enum Address {
    /// A path on the filesystem
    Path(String),
    /// An abstract path (Linux-only)
    Abstract(String),
}

impl Address {
    pub(crate) fn connect(&self) -> Result<UnixStream> {
        match self {
            Address::Path(p) => Ok(UnixStream::connect(p)?),
            Address::Abstract(p) => {
                // FIXME: Use std API once std supports abstract sockets:
                //
                // https://github.com/rust-lang/rust/issues/42048
                let addr = SockAddr::Unix(UnixAddr::new_abstract(p.as_bytes())?);
                let raw = socket::socket(
                    AddressFamily::Unix,
                    SockType::Stream,
                    SockFlag::empty(),
                    None,
                )?;
                socket::connect(raw, &addr)?;

                Ok(unsafe { UnixStream::from_raw_fd(raw) })
            }
        }
    }
}

impl FromStr for Address {
    type Err = Error;

    /// Parse a D-BUS address and return its path if we recognize it
    fn from_str(address: &str) -> Result<Self> {
        // Options are given separated by commas
        let first = address.split(',').next().unwrap();
        let parts = first.split(':').collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(Error::Address("address has no colon".into()));
        }
        if parts[0] != "unix" {
            return Err(Error::Address(format!(
                "unsupported transport '{}'",
                parts[0]
            )));
        }

        let pathparts = parts[1].split('=').collect::<Vec<&str>>();
        if pathparts.len() != 2 {
            return Err(Error::Address("address is missing '='".into()));
        }
        match pathparts[0] {
            "path" => Ok(Address::Path(pathparts[1].to_owned())),
            "abstract" => Ok(Address::Abstract(pathparts[1].to_owned())),
            _ => Err(Error::Address(
                "unix address is missing path or abstract".to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Address;
    use crate::Error;
    use std::str::FromStr;

    #[test]
    fn parse_dbus_addresses() {
        match Address::from_str("foo").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "address has no colon"),
            _ => panic!(),
        }
        match Address::from_str("tcp:localhost").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "unsupported transport 'tcp'"),
            _ => panic!(),
        }
        assert_eq!(
            Address::Path("/tmp/dbus-foo".into()),
            Address::from_str("unix:path=/tmp/dbus-foo").unwrap()
        );
        assert_eq!(
            Address::Path("/tmp/dbus-foo".into()),
            Address::from_str("unix:path=/tmp/dbus-foo,guid=123").unwrap()
        );
    }
}
