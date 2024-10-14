use std::fmt;

use super::{Error, Result};

/// Universally-unique IDs for D-Bus addresses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guid([u8; 16]);

impl Guid {
    /// Create a Guid from the given bytes.
    pub fn new(bytes: [u8; 16]) -> Self {
        Guid(bytes)
    }

    /// Returns a byte slice of this Guid’s contents
    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
               self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7],
               self.0[8], self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15])
    }
}

impl TryFrom<&str> for Guid {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        if s.len() != 32 {
            return Err(Error::InvalidValue("guid".into()));
        }

        let mut bytes = [0u8; 16];
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            bytes[i] = u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16)
                .map_err(|_| Error::InvalidValue("guid".into()))?;
        }

        Ok(Guid(bytes))
    }
}
