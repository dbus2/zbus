use std::{error, fmt};

/// An error that can happen when parsing a D-Bus address
#[derive(Debug, PartialEq)]
pub struct AddressError(String);

impl error::Error for AddressError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A bus address
#[derive(Debug, PartialEq)]
pub(crate) enum Address {
    /// A path on the filesystem
    Path(String),
    /// An abstract path (Linux-only)
    Abstract(String),
}

/// Parse a D-BUS address and return its path if we recognize it
pub(crate) fn parse_dbus_address(address: &str) -> Result<Address, AddressError> {
    // Options are given separated by commas
    let first = address.split(',').next().unwrap();
    let parts = first.split(':').collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Err(AddressError("address has no colon".into()));
    }
    if parts[0] != "unix" {
        return Err(AddressError(format!(
            "unsupported transport '{}'",
            parts[0]
        )));
    }

    let pathparts = parts[1].split('=').collect::<Vec<&str>>();
    if pathparts.len() != 2 {
        return Err(AddressError("address is missing '='".into()));
    }
    match pathparts[0] {
        "path" => Ok(Address::Path(pathparts[1].to_owned())),
        "abstract" => Ok(Address::Abstract(pathparts[1].to_owned())),
        _ => Err(AddressError(
            "unix address is missing path or abstract".to_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_dbus_address, Address, AddressError};

    #[test]
    fn parse_dbus_addresses() {
        assert_eq!(
            Err(AddressError("address has no colon".into())),
            parse_dbus_address("foo")
        );
        assert_eq!(
            Err(AddressError("unsupported transport 'tcp'".into())),
            parse_dbus_address("tcp:localhost")
        );
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
