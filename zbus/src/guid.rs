use std::convert::TryFrom;
use std::fmt;

/// A D-Bus server GUID.
///
/// See the D-Bus specification [UUIDs chapter] for details.
///
/// [UUIDs chapter]: https://dbus.freedesktop.org/doc/dbus-specification.html#uuids
#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Guid(String);

impl Guid {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl TryFrom<&str> for Guid {
    type Error = crate::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        if value.as_bytes().len() != 32 || !value.chars().all(|c| char::is_ascii_hexdigit(&c)) {
            Err(crate::Error::InvalidGUID)
        } else {
            Ok(Guid(value.to_string()))
        }
    }
}
