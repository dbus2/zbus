use crate::{Error, Result};
use nb_connect::unix;
use polling::{Event, Poller};
use std::{ffi::OsString, os::unix::net::UnixStream, str::FromStr};

/// A bus address
#[derive(Debug, PartialEq)]
pub(crate) enum Address {
    /// A path on the filesystem
    Unix(OsString),
}

impl Address {
    pub(crate) fn connect(&self, nonblocking: bool) -> Result<UnixStream> {
        match self {
            Address::Unix(p) => {
                let stream = unix(p)?;

                let poller = Poller::new()?;
                poller.add(&stream, Event::writable(0))?;
                poller.wait(&mut Vec::new(), None)?;

                stream.set_nonblocking(nonblocking)?;

                Ok(stream)
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
        let path = match pathparts[0] {
            "path" => OsString::from(pathparts[1]),
            "abstract" => {
                let mut s = OsString::from("\0");
                s.push(pathparts[1]);

                s
            }
            _ => {
                return Err(Error::Address(
                    "unix address is missing path or abstract".to_owned(),
                ))
            }
        };
        Ok(Address::Unix(path))
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
            Address::Unix("/tmp/dbus-foo".into()),
            Address::from_str("unix:path=/tmp/dbus-foo").unwrap()
        );
        assert_eq!(
            Address::Unix("/tmp/dbus-foo".into()),
            Address::from_str("unix:path=/tmp/dbus-foo,guid=123").unwrap()
        );
    }
}
