use crate::{Error, Result};

/// A bus address
#[derive(Debug, PartialEq)]
pub(crate) enum Address {
    /// A path on the filesystem
    Path(String),
    /// An abstract path (Linux-only)
    Abstract(String),
}

/// Parse a D-BUS address and return its path if we recognize it
pub(crate) fn parse_dbus_address(address: &str) -> Result<Address> {
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

#[cfg(test)]
mod tests {
    use super::{parse_dbus_address, Address};
    use crate::Error;

    #[test]
    fn parse_dbus_addresses() {
        match parse_dbus_address("foo").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "address has no colon"),
            _ => panic!(),
        }
        match parse_dbus_address("tcp:localhost").unwrap_err() {
            Error::Address(e) => assert_eq!(e, "unsupported transport 'tcp'"),
            _ => panic!(),
        }
        assert_eq!(
            Address::Path("/tmp/dbus-foo".into()),
            parse_dbus_address("unix:path=/tmp/dbus-foo").unwrap()
        );
        assert_eq!(
            Address::Path("/tmp/dbus-foo".into()),
            parse_dbus_address("unix:path=/tmp/dbus-foo,guid=123").unwrap()
        );
    }
}
