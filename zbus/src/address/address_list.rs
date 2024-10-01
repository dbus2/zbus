use std::{borrow::Cow, fmt};

use super::{Address, Error, Result, ToAddresses};

/// A bus address list.
///
/// D-Bus addresses are `;`-separated.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AddressList<'a> {
    addr: Cow<'a, str>,
}

impl<'a> ToAddresses<'a> for AddressList<'a> {
    type Iter = AddressListIter<'a>;

    /// Get an iterator over the D-Bus addresses.
    fn to_addresses(&'a self) -> Self::Iter {
        AddressListIter::new(self)
    }
}

impl<'a> Iterator for AddressListIter<'a> {
    type Item = Result<Address<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.data.len() {
            return None;
        }

        let mut addr = &self.data[self.next_index..];
        if let Some(end) = addr.find(';') {
            addr = &addr[..end];
            self.next_index += end + 1;
        } else {
            self.next_index = self.data.len();
        }

        Some(Address::try_from(addr))
    }
}

/// An iterator of D-Bus addresses.
pub struct AddressListIter<'a> {
    data: &'a str,
    next_index: usize,
}

impl<'a> AddressListIter<'a> {
    fn new(list: &'a AddressList<'_>) -> Self {
        Self {
            data: list.addr.as_ref(),
            next_index: 0,
        }
    }
}

impl<'a> TryFrom<String> for AddressList<'a> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(Self {
            addr: Cow::Owned(value),
        })
    }
}

impl<'a> TryFrom<&'a str> for AddressList<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self> {
        Ok(Self {
            addr: Cow::Borrowed(value),
        })
    }
}

impl fmt::Display for AddressList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.addr)
    }
}
