use core::{
    borrow::Borrow,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    ops::Deref,
};
use std::{borrow::Cow, sync::Arc};

use crate::{Error, OwnedUniqueName, OwnedWellKnownName, Result, UniqueName, WellKnownName};
use serde::{de, Deserialize, Serialize};
use static_assertions::assert_impl_all;
use zvariant::{NoneValue, OwnedValue, Str, Type, Value};

/// String that identifies a [bus name].
///
/// # Examples
///
/// ```
/// use core::convert::TryFrom;
/// use zbus_names::BusName;
///
/// // Valid well-known names.
/// let name = BusName::try_from("org.gnome.Service-for_you").unwrap();
/// assert!(matches!(name, BusName::WellKnown(_)));
/// assert_eq!(name, "org.gnome.Service-for_you");
/// let name = BusName::try_from("a.very.loooooooooooooooooo-ooooooo_0000o0ng.Name").unwrap();
/// assert!(matches!(name, BusName::WellKnown(_)));
/// assert_eq!(name, "a.very.loooooooooooooooooo-ooooooo_0000o0ng.Name");
///
/// // Valid unique names.
/// let name = BusName::try_from(":org.gnome.Service-for_you").unwrap();
/// assert!(matches!(name, BusName::Unique(_)));
/// assert_eq!(name, ":org.gnome.Service-for_you");
/// let name = BusName::try_from(":a.very.loooooooooooooooooo-ooooooo_0000o0ng.Name").unwrap();
/// assert!(matches!(name, BusName::Unique(_)));
/// assert_eq!(name, ":a.very.loooooooooooooooooo-ooooooo_0000o0ng.Name");
///
/// // Invalid bus names
/// BusName::try_from("").unwrap_err();
/// BusName::try_from("double..dots").unwrap_err();
/// BusName::try_from(".").unwrap_err();
/// BusName::try_from(".start.with.dot").unwrap_err();
/// BusName::try_from("1start.with.digit").unwrap_err();
/// BusName::try_from("no-dots").unwrap_err();
/// ```
///
/// [bus name]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-protocol-names-bus
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(untagged)]
pub enum BusName<'name> {
    #[serde(borrow)]
    Unique(UniqueName<'name>),
    #[serde(borrow)]
    WellKnown(WellKnownName<'name>),
}

assert_impl_all!(BusName<'_>: Send, Sync, Unpin);

impl<'name> BusName<'name> {
    /// A borrowed clone (never allocates, unlike clone).
    pub fn as_ref(&self) -> BusName<'_> {
        match self {
            BusName::Unique(name) => BusName::Unique(name.as_ref()),
            BusName::WellKnown(name) => BusName::WellKnown(name.as_ref()),
        }
    }

    /// The well-known-name as string.
    pub fn as_str(&self) -> &str {
        match self {
            BusName::Unique(name) => name.as_str(),
            BusName::WellKnown(name) => name.as_str(),
        }
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> BusName<'static> {
        match self {
            BusName::Unique(name) => BusName::Unique(name.to_owned()),
            BusName::WellKnown(name) => BusName::WellKnown(name.to_owned()),
        }
    }

    /// Creates an owned clone of `self`.
    pub fn into_owned(self) -> BusName<'static> {
        match self {
            BusName::Unique(name) => BusName::Unique(name.into_owned()),
            BusName::WellKnown(name) => BusName::WellKnown(name.into_owned()),
        }
    }

    /// Same as `try_from`, except it takes a `&'static str`.
    pub fn from_static_str(name: &'static str) -> Result<Self> {
        match Self::try_from(name)? {
            BusName::Unique(_) => Ok(BusName::Unique(UniqueName::from_static_str_unchecked(name))),
            BusName::WellKnown(_) => Ok(BusName::WellKnown(
                WellKnownName::from_static_str_unchecked(name),
            )),
        }
    }
}

impl Deref for BusName<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Borrow<str> for BusName<'_> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Display for BusName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.as_str(), f)
    }
}

impl PartialEq<str> for BusName<'_> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for BusName<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<OwnedBusName> for BusName<'_> {
    fn eq(&self, other: &OwnedBusName) -> bool {
        *self == other.0
    }
}

impl PartialEq<UniqueName<'_>> for BusName<'_> {
    fn eq(&self, other: &UniqueName<'_>) -> bool {
        match self {
            Self::Unique(name) => *name == *other,
            Self::WellKnown(_) => false,
        }
    }
}

impl PartialEq<WellKnownName<'_>> for BusName<'_> {
    fn eq(&self, other: &WellKnownName<'_>) -> bool {
        match self {
            Self::Unique(_) => false,
            Self::WellKnown(name) => *name == *other,
        }
    }
}

impl<'name> NoneValue for BusName<'name> {
    type NoneType = &'name str;

    fn null_value() -> Self::NoneType {
        <&str>::default()
    }
}

// Manual deserialize implementation to get the desired error on invalid bus names.
impl<'de: 'name, 'name> Deserialize<'de> for BusName<'name> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = <Cow<'name, str>>::deserialize(deserializer)?;

        Self::try_from(name).map_err(|e| de::Error::custom(e.to_string()))
    }
}

impl Type for BusName<'_> {
    fn signature() -> zvariant::Signature<'static> {
        <&str>::signature()
    }
}

impl<'name> From<UniqueName<'name>> for BusName<'name> {
    fn from(name: UniqueName<'name>) -> Self {
        BusName::Unique(name)
    }
}

impl<'name> From<WellKnownName<'name>> for BusName<'name> {
    fn from(name: WellKnownName<'name>) -> Self {
        BusName::WellKnown(name)
    }
}

/// Try to create an `BusName` from a string.
impl<'s> TryFrom<&'s str> for BusName<'s> {
    type Error = Error;

    fn try_from(value: &'s str) -> Result<Self> {
        match UniqueName::try_from(value) {
            Err(Error::InvalidUniqueName(unique_err)) => match WellKnownName::try_from(value) {
                Err(Error::InvalidWellKnownName(well_known_err)) => {
                    Err(Error::InvalidBusName(unique_err, well_known_err))
                }
                Err(e) => Err(e),
                Ok(name) => Ok(BusName::WellKnown(name)),
            },
            Err(e) => Err(e),
            Ok(name) => Ok(BusName::Unique(name)),
        }
    }
}

impl TryFrom<String> for BusName<'_> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(match BusName::try_from(value.as_str())? {
            BusName::Unique(_) => BusName::Unique(UniqueName::from_string_unchecked(value)),
            BusName::WellKnown(_) => {
                BusName::WellKnown(WellKnownName::from_string_unchecked(value))
            }
        })
    }
}

impl TryFrom<Arc<str>> for BusName<'_> {
    type Error = Error;

    fn try_from(value: Arc<str>) -> Result<Self> {
        // FIXME: Same as `str` impl above. Use a macro to avoid code duplication.
        match UniqueName::try_from(value.clone()) {
            Err(Error::InvalidUniqueName(unique_err)) => match WellKnownName::try_from(value) {
                Err(Error::InvalidWellKnownName(well_known_err)) => {
                    Err(Error::InvalidBusName(unique_err, well_known_err))
                }
                Err(e) => Err(e),
                Ok(name) => Ok(BusName::WellKnown(name)),
            },
            Err(e) => Err(e),
            Ok(name) => Ok(BusName::Unique(name)),
        }
    }
}

impl<'s> TryFrom<Value<'s>> for BusName<'s> {
    type Error = Error;

    fn try_from(value: Value<'s>) -> Result<Self> {
        let value = Str::try_from(value)?;
        Ok(match BusName::try_from(value.as_str())? {
            BusName::Unique(_) => BusName::Unique(UniqueName::from_string_unchecked(value.into())),
            BusName::WellKnown(_) => {
                BusName::WellKnown(WellKnownName::from_string_unchecked(value.into()))
            }
        })
    }
}

/// This never succeeds but is provided so it's easier to pass `Option::None` values for API
/// requiring `Option<TryInto<impl BusName>>`, since type inference won't work here.
impl TryFrom<()> for BusName<'_> {
    type Error = Error;

    fn try_from(_value: ()) -> Result<Self> {
        unreachable!("Conversion from `()` is not meant to actually work.");
    }
}

impl<'name> TryFrom<Cow<'name, str>> for BusName<'name> {
    type Error = Error;

    fn try_from(value: Cow<'name, str>) -> Result<Self> {
        match value {
            Cow::Borrowed(s) => Self::try_from(s),
            Cow::Owned(s) => Self::try_from(s),
        }
    }
}

impl<'s> From<BusName<'s>> for Value<'s> {
    fn from(name: BusName<'s>) -> Self {
        match name {
            BusName::Unique(name) => name.into(),
            BusName::WellKnown(name) => name.into(),
        }
    }
}

impl<'name> From<&BusName<'name>> for BusName<'name> {
    fn from(name: &BusName<'name>) -> Self {
        name.clone()
    }
}

impl TryFrom<OwnedValue> for BusName<'static> {
    type Error = Error;

    fn try_from(value: OwnedValue) -> Result<Self> {
        let value = Str::try_from(value)?;
        Ok(match BusName::try_from(value.as_str())? {
            BusName::Unique(_) => BusName::Unique(UniqueName::from_string_unchecked(value.into())),
            BusName::WellKnown(_) => {
                BusName::WellKnown(WellKnownName::from_string_unchecked(value.into()))
            }
        })
    }
}

impl From<BusName<'static>> for OwnedValue {
    fn from(name: BusName<'static>) -> Self {
        match name {
            BusName::Unique(name) => name.into(),
            BusName::WellKnown(name) => name.into(),
        }
    }
}

impl From<OwnedUniqueName> for BusName<'static> {
    fn from(name: OwnedUniqueName) -> Self {
        BusName::Unique(name.into())
    }
}

impl<'a> From<&'a OwnedUniqueName> for BusName<'a> {
    fn from(name: &'a OwnedUniqueName) -> Self {
        BusName::Unique(name.into())
    }
}

impl From<OwnedWellKnownName> for BusName<'static> {
    fn from(name: OwnedWellKnownName) -> Self {
        BusName::WellKnown(name.into())
    }
}

impl<'a> From<&'a OwnedWellKnownName> for BusName<'a> {
    fn from(name: &'a OwnedWellKnownName) -> Self {
        BusName::WellKnown(name.into())
    }
}

/// Owned sibling of [`BusName`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, PartialOrd, Ord, Type)]
pub struct OwnedBusName(#[serde(borrow)] BusName<'static>);

impl OwnedBusName {
    /// Convert to the inner `BusName`, consuming `self`.
    pub fn into_inner(self) -> BusName<'static> {
        self.0
    }

    /// Get a reference to the inner `BusName`.
    pub fn inner(&self) -> &BusName<'static> {
        &self.0
    }
}

impl Deref for OwnedBusName {
    type Target = BusName<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<str> for OwnedBusName {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for OwnedBusName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        BusName::from(self).fmt(f)
    }
}

impl From<OwnedBusName> for BusName<'static> {
    fn from(name: OwnedBusName) -> Self {
        name.into_inner()
    }
}

impl<'unowned, 'owned: 'unowned> From<&'owned OwnedBusName> for BusName<'unowned> {
    fn from(name: &'owned OwnedBusName) -> Self {
        match &name.0 {
            BusName::Unique(name) => BusName::Unique(UniqueName::from_str_unchecked(name)),
            BusName::WellKnown(name) => BusName::WellKnown(WellKnownName::from_str_unchecked(name)),
        }
    }
}

impl std::convert::From<BusName<'_>> for OwnedBusName {
    fn from(name: BusName<'_>) -> Self {
        OwnedBusName(name.into_owned())
    }
}

impl TryFrom<&'_ str> for OwnedBusName {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Self::from(BusName::try_from(value)?))
    }
}

impl TryFrom<String> for OwnedBusName {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Ok(Self::from(BusName::try_from(value)?))
    }
}

impl TryFrom<Value<'static>> for OwnedBusName {
    type Error = Error;

    fn try_from(value: Value<'static>) -> Result<Self> {
        Ok(Self::from(BusName::try_from(value)?))
    }
}

impl From<OwnedBusName> for Value<'static> {
    fn from(name: OwnedBusName) -> Self {
        name.0.into()
    }
}

impl TryFrom<OwnedValue> for OwnedBusName {
    type Error = Error;

    fn try_from(value: OwnedValue) -> Result<Self> {
        Ok(Self::from(BusName::try_from(value)?))
    }
}

impl From<OwnedBusName> for OwnedValue {
    fn from(name: OwnedBusName) -> Self {
        name.0.into()
    }
}

impl<'de> Deserialize<'de> for OwnedBusName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|n| BusName::try_from(n).map_err(|e| de::Error::custom(e.to_string())))
            .map(Self)
    }
}

impl PartialEq<&str> for OwnedBusName {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialEq<BusName<'_>> for OwnedBusName {
    fn eq(&self, other: &BusName<'_>) -> bool {
        self.0 == *other
    }
}

impl NoneValue for OwnedBusName {
    type NoneType = <BusName<'static> as NoneValue>::NoneType;

    fn null_value() -> Self::NoneType {
        BusName::null_value()
    }
}
