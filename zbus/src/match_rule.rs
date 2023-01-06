use std::{convert::TryFrom, ops::Deref};

use serde::{de, Deserialize, Serialize};
use static_assertions::assert_impl_all;
use zvariant::Structure;

use crate::{
    names::{BusName, InterfaceName, MemberName, UniqueName},
    zvariant::{ObjectPath, Str, Type},
    Error, MatchRuleBuilder, MessageType, Result,
};

/// A bus match rule for subscribing to specific messages.
///
/// This is mainly used by peer to subscribe to specific signals as by default the bus will not
/// send out most broadcasted signals. This API is intended to make it easy to create and parse
/// match rules. See the [match rules section of the D-Bus specification][mrs] for a description of
/// each possible element of a match rule.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use zbus::MatchRule;
/// use std::convert::TryFrom;
///
/// // Let's take the most typical example of match rule to subscribe to properties' changes:
/// let rule = MatchRule::builder()
///     .msg_type(zbus::MessageType::Signal)
///     .sender("org.freedesktop.DBus")?
///     .interface("org.freedesktop.DBus.Properties")?
///     .member("PropertiesChanged")?
///     .add_arg("org.zbus")?
///     .build();
/// let rule_str = rule.to_string();
/// assert_eq!(
///     rule_str,
///     "type='signal',\
///      sender='org.freedesktop.DBus',\
///      interface='org.freedesktop.DBus.Properties',\
///      member='PropertiesChanged',\
///      arg0='org.zbus'",
/// );
///
/// // Let's parse it back.
/// let parsed_rule = MatchRule::try_from(rule_str.as_str())?;
/// assert_eq!(rule, parsed_rule);
///
/// // Now for `ObjectManager::InterfacesAdded` signal.
/// let rule = MatchRule::builder()
///     .msg_type(zbus::MessageType::Signal)
///     .sender("org.zbus")?
///     .interface("org.freedesktop.DBus.ObjectManager")?
///     .member("InterfacesAdded")?
///     .arg_path(0, "/org/zbus/NewPath")?
///     .build();
/// let rule_str = rule.to_string();
/// assert_eq!(
///     rule_str,
///     "type='signal',\
///      sender='org.zbus',\
///      interface='org.freedesktop.DBus.ObjectManager',\
///      member='InterfacesAdded',\
///      arg0path='/org/zbus/NewPath'",
/// );
///
/// // Let's parse it back.
/// let parsed_rule = MatchRule::try_from(rule_str.as_str())?;
/// assert_eq!(rule, parsed_rule);
///
/// # Ok(())
/// # }
/// ```
///
/// # Caveats
///
/// The `PartialEq` implementation assumes arguments in both rules are in the same order.
///
/// [mrs]: https://dbus.freedesktop.org/doc/dbus-specification.html#message-bus-routing-match-rules
#[derive(Clone, Debug, PartialEq, Eq, Hash, Type)]
#[zvariant(signature = "s")]
pub struct MatchRule<'m> {
    pub(crate) msg_type: Option<MessageType>,
    pub(crate) sender: Option<BusName<'m>>,
    pub(crate) interface: Option<InterfaceName<'m>>,
    pub(crate) member: Option<MemberName<'m>>,
    pub(crate) path_spec: Option<MatchRulePathSpec<'m>>,
    pub(crate) destination: Option<UniqueName<'m>>,
    pub(crate) args: Vec<(u8, Str<'m>)>,
    pub(crate) arg_paths: Vec<(u8, ObjectPath<'m>)>,
    pub(crate) arg0namespace: Option<InterfaceName<'m>>,
}

assert_impl_all!(MatchRule<'_>: Send, Sync, Unpin);

impl<'m> MatchRule<'m> {
    /// Create a builder for `MatchRuleBuilder`.
    pub fn builder() -> MatchRuleBuilder<'m> {
        MatchRuleBuilder::new()
    }

    /// The sender, if set.
    pub fn sender(&self) -> Option<&BusName<'_>> {
        self.sender.as_ref()
    }

    /// The message type, if set.
    pub fn msg_type(&self) -> Option<MessageType> {
        self.msg_type
    }

    /// The interfac, if set.
    pub fn interface(&self) -> Option<&InterfaceName<'_>> {
        self.interface.as_ref()
    }

    /// The member name if set.
    pub fn member(&self) -> Option<&MemberName<'_>> {
        self.member.as_ref()
    }

    /// The path or path namespace, if set.
    pub fn path_spec(&self) -> Option<&MatchRulePathSpec<'_>> {
        self.path_spec.as_ref()
    }

    /// The destination, if set.
    pub fn destination(&self) -> Option<&UniqueName<'_>> {
        self.destination.as_ref()
    }

    /// The arguments.
    pub fn args(&self) -> &[(u8, Str<'_>)] {
        self.args.as_ref()
    }

    /// The argument paths.
    pub fn arg_paths(&self) -> &[(u8, ObjectPath<'_>)] {
        self.arg_paths.as_ref()
    }

    /// Match messages whose first argument is within the specified namespace.
    ///
    /// Note that while the spec allows this to be any string that's a valid bus or interface name
    /// except that it can have no `.`, we only allow valid interface names. The reason is not only
    /// to keep things simple and type safe at the same time but also for the fact that use cases of
    /// only matching on the first component of a bus or interface name are unheard of.
    pub fn arg0namespace(&self) -> Option<&InterfaceName<'_>> {
        self.arg0namespace.as_ref()
    }

    /// Creates an owned clone of `self`.
    pub fn to_owned(&self) -> MatchRule<'static> {
        MatchRule {
            msg_type: self.msg_type,
            sender: self.sender.as_ref().map(|s| s.to_owned()),
            interface: self.interface.as_ref().map(|i| i.to_owned()),
            member: self.member.as_ref().map(|m| m.to_owned()),
            path_spec: self.path_spec.as_ref().map(|p| p.to_owned()),
            destination: self.destination.as_ref().map(|d| d.to_owned()),
            args: self.args.iter().map(|(i, s)| (*i, s.to_owned())).collect(),
            arg_paths: self
                .arg_paths
                .iter()
                .map(|(i, p)| (*i, p.to_owned()))
                .collect(),
            arg0namespace: self.arg0namespace.as_ref().map(|a| a.to_owned()),
        }
    }

    /// Creates an owned clone of `self`.
    pub fn into_owned(self) -> MatchRule<'static> {
        MatchRule {
            msg_type: self.msg_type,
            sender: self.sender.map(|s| s.into_owned()),
            interface: self.interface.map(|i| i.into_owned()),
            member: self.member.map(|m| m.into_owned()),
            path_spec: self.path_spec.map(|p| p.into_owned()),
            destination: self.destination.map(|d| d.into_owned()),
            args: self
                .args
                .into_iter()
                .map(|(i, s)| (i, s.into_owned()))
                .collect(),
            arg_paths: self
                .arg_paths
                .into_iter()
                .map(|(i, p)| (i, p.into_owned()))
                .collect(),
            arg0namespace: self.arg0namespace.map(|a| a.into_owned()),
        }
    }

    /// Match the given message against this rule.
    ///
    /// # Caveats
    ///
    /// Since this method doesn't have any knowledge of names on the bus (or even connection to a
    /// bus) matching always succeeds for:
    ///
    /// * `sender` in the rule (if set) that is a well-known name. The `sender` on a message is
    ///   always a unique name.
    /// * `destination` in the rule when `destination` on the `msg` is a well-known name. The
    ///   `destination` on match rule is always a unique name.
    pub fn matches(&self, msg: &zbus::Message) -> Result<bool> {
        let hdr = msg.header()?;

        // Start with message type.
        if let Some(msg_type) = self.msg_type() {
            if msg_type != msg.message_type() {
                return Ok(false);
            }
        }

        // Then check sender.
        if let Some(sender) = self.sender() {
            match sender {
                BusName::Unique(name) if Some(name) != hdr.sender()? => {
                    return Ok(false);
                }
                BusName::Unique(_) => (),
                // We can't match against a well-known name.
                BusName::WellKnown(_) => (),
            }
        }

        // The interface.
        if let Some(interface) = self.interface() {
            match msg.interface().as_ref() {
                Some(msg_interface) if interface != msg_interface => return Ok(false),
                Some(_) => (),
                None => return Ok(false),
            }
        }

        // The member.
        if let Some(member) = self.member() {
            match msg.member().as_ref() {
                Some(msg_member) if member != msg_member => return Ok(false),
                Some(_) => (),
                None => return Ok(false),
            }
        }

        // The destination.
        if let Some(destination) = self.destination() {
            match hdr.destination()? {
                Some(BusName::Unique(name)) if destination != name => {
                    return Ok(false);
                }
                Some(BusName::Unique(_)) | None => (),
                // We can't match against a well-known name.
                Some(BusName::WellKnown(_)) => (),
            };
        }

        // The path.
        if let Some(path_spec) = self.path_spec() {
            let msg_path = match msg.path() {
                Some(p) => p,
                None => return Ok(false),
            };
            match path_spec {
                MatchRulePathSpec::Path(path) if path != &msg_path => return Ok(false),
                MatchRulePathSpec::PathNamespace(path_ns)
                    if !msg_path.starts_with(path_ns.as_str()) =>
                {
                    return Ok(false);
                }
                MatchRulePathSpec::Path(_) | MatchRulePathSpec::PathNamespace(_) => (),
            }
        }

        // The arg0 namespace.
        if let Some(arg0_ns) = self.arg0namespace() {
            if let Ok(arg0) = msg.body_unchecked::<InterfaceName<'_>>() {
                if !arg0.starts_with(arg0_ns.as_str()) {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        // Args
        if self.args().is_empty() && self.arg_paths().is_empty() {
            return Ok(true);
        }
        let structure = match msg.body::<Structure<'_>>() {
            Ok(s) => s,
            Err(_) => return Ok(false),
        };
        let args = structure.fields();

        for (i, arg) in self.args() {
            match args.get(*i as usize) {
                Some(msg_arg) => match <&str>::try_from(msg_arg) {
                    Ok(msg_arg) if arg != msg_arg => return Ok(false),
                    Ok(_) => (),
                    Err(_) => return Ok(false),
                },
                None => return Ok(false),
            }
        }

        // Path args
        for (i, path) in self.arg_paths() {
            match args.get(*i as usize) {
                Some(msg_arg) => match <ObjectPath<'_>>::try_from(msg_arg) {
                    Ok(msg_arg) if *path != msg_arg => return Ok(false),
                    Ok(_) => (),
                    Err(_) => return Ok(false),
                },
                None => return Ok(false),
            }
        }

        Ok(true)
    }
}

impl ToString for MatchRule<'_> {
    fn to_string(&self) -> String {
        let mut s = String::new();

        if let Some(msg_type) = self.msg_type() {
            let type_str = match msg_type {
                MessageType::Error => "error",
                MessageType::Invalid => panic!("invalid message type"),
                MessageType::MethodCall => "method_call",
                MessageType::MethodReturn => "method_return",
                MessageType::Signal => "signal",
            };
            add_match_rule_string_component(&mut s, "type", type_str);
        }
        if let Some(sender) = self.sender() {
            add_match_rule_string_component(&mut s, "sender", sender);
        }
        if let Some(interface) = self.interface() {
            add_match_rule_string_component(&mut s, "interface", interface);
        }
        if let Some(member) = self.member() {
            add_match_rule_string_component(&mut s, "member", member);
        }
        if let Some(destination) = self.destination() {
            add_match_rule_string_component(&mut s, "destination", destination);
        }
        if let Some(path_spec) = self.path_spec() {
            let (key, value) = match path_spec {
                MatchRulePathSpec::Path(path) => ("path", path),
                MatchRulePathSpec::PathNamespace(ns) => ("path_namespace", ns),
            };
            add_match_rule_string_component(&mut s, key, value);
        }
        for (i, arg) in self.args() {
            add_match_rule_string_component(&mut s, &format!("arg{i}"), arg);
        }
        for (i, arg_path) in self.arg_paths() {
            add_match_rule_string_component(&mut s, &format!("arg{i}path"), arg_path);
        }
        if let Some(arg0namespace) = self.arg0namespace() {
            add_match_rule_string_component(&mut s, "arg0namespace", arg0namespace)
        }

        s
    }
}

fn add_match_rule_string_component(rule: &mut String, key: &str, value: &str) {
    if !rule.is_empty() {
        rule.push(',');
    }
    rule.push_str(key);
    rule.push('=');
    rule.push('\'');
    rule.push_str(value);
    rule.push('\'');
}

impl<'m> TryFrom<&'m str> for MatchRule<'m> {
    type Error = Error;

    fn try_from(s: &'m str) -> Result<Self> {
        let components = s.split(',');
        if components.clone().peekable().peek().is_none() {
            return Err(Error::InvalidMatchRule);
        }
        let mut builder = MatchRule::builder();
        for component in components {
            let (key, value) = component.split_once('=').ok_or(Error::InvalidMatchRule)?;
            if key.is_empty()
                || value.len() < 3
                || !value.starts_with('\'')
                || !value.ends_with('\'')
            {
                return Err(Error::InvalidMatchRule);
            }
            let value = &value[1..value.len() - 1];
            builder = match key {
                "type" => {
                    let msg_type = match value {
                        "error" => MessageType::Error,
                        "method_call" => MessageType::MethodCall,
                        "method_return" => MessageType::MethodReturn,
                        "signal" => MessageType::Signal,
                        _ => return Err(Error::InvalidMatchRule),
                    };
                    builder.msg_type(msg_type)
                }
                "sender" => builder.sender(value)?,
                "interface" => builder.interface(value)?,
                "member" => builder.member(value)?,
                "path" => builder.path(value)?,
                "path_namespace" => builder.path_namespace(value)?,
                "destination" => builder.destination(value)?,
                "arg0namespace" => builder.arg0namespace(value)?,
                key if key.starts_with("arg") => {
                    if let Some(trailing_idx) = key.find("path") {
                        let idx = key[3..trailing_idx]
                            .parse::<u8>()
                            .map_err(|_| Error::InvalidMatchRule)?;
                        builder.arg_path(idx, value)?
                    } else {
                        let idx = key[3..]
                            .parse::<u8>()
                            .map_err(|_| Error::InvalidMatchRule)?;
                        builder.arg(idx, value)?
                    }
                }
                _ => return Err(Error::InvalidMatchRule),
            };
        }

        Ok(builder.build())
    }
}

impl<'de: 'm, 'm> Deserialize<'de> for MatchRule<'m> {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let name = <&str>::deserialize(deserializer)?;

        Self::try_from(name).map_err(|e| de::Error::custom(e.to_string()))
    }
}

impl Serialize for MatchRule<'_> {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// The path or path namespace.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MatchRulePathSpec<'m> {
    Path(ObjectPath<'m>),
    PathNamespace(ObjectPath<'m>),
}

assert_impl_all!(MatchRulePathSpec<'_>: Send, Sync, Unpin);

impl<'m> MatchRulePathSpec<'m> {
    /// Creates an owned clone of `self`.
    fn to_owned(&self) -> MatchRulePathSpec<'static> {
        match self {
            MatchRulePathSpec::Path(path) => MatchRulePathSpec::Path(path.to_owned()),
            MatchRulePathSpec::PathNamespace(ns) => MatchRulePathSpec::PathNamespace(ns.to_owned()),
        }
    }

    /// Creates an owned clone of `self`.
    pub fn into_owned(self) -> MatchRulePathSpec<'static> {
        match self {
            MatchRulePathSpec::Path(path) => MatchRulePathSpec::Path(path.into_owned()),
            MatchRulePathSpec::PathNamespace(ns) => {
                MatchRulePathSpec::PathNamespace(ns.into_owned())
            }
        }
    }
}

/// Owned sibling of [`MatchRule`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Type)]
pub struct OwnedMatchRule(#[serde(borrow)] MatchRule<'static>);

assert_impl_all!(OwnedMatchRule: Send, Sync, Unpin);

impl OwnedMatchRule {
    /// Convert to the inner `MatchRule`, consuming `self`.
    pub fn into_inner(self) -> MatchRule<'static> {
        self.0
    }

    /// Get a reference to the inner `MatchRule`.
    pub fn inner(&self) -> &MatchRule<'static> {
        &self.0
    }
}

impl Deref for OwnedMatchRule {
    type Target = MatchRule<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<OwnedMatchRule> for MatchRule<'static> {
    fn from(o: OwnedMatchRule) -> Self {
        o.into_inner()
    }
}

impl<'unowned, 'owned: 'unowned> From<&'owned OwnedMatchRule> for MatchRule<'unowned> {
    fn from(rule: &'owned OwnedMatchRule) -> Self {
        rule.inner().clone()
    }
}

impl From<MatchRule<'_>> for OwnedMatchRule {
    fn from(rule: MatchRule<'_>) -> Self {
        OwnedMatchRule(rule.into_owned())
    }
}

impl TryFrom<&'_ str> for OwnedMatchRule {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Ok(Self::from(MatchRule::try_from(value)?))
    }
}

impl<'de> Deserialize<'de> for OwnedMatchRule {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|r| {
                MatchRule::try_from(r.as_str())
                    .map(|r| r.to_owned())
                    .map_err(|e| de::Error::custom(e.to_string()))
            })
            .map(Self)
    }
}

impl PartialEq<MatchRule<'_>> for OwnedMatchRule {
    fn eq(&self, other: &MatchRule<'_>) -> bool {
        self.0 == *other
    }
}
