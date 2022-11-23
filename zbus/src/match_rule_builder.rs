use std::convert::TryInto;

use static_assertions::assert_impl_all;

use crate::{
    names::{BusName, InterfaceName, MemberName, UniqueName},
    zvariant::{ObjectPath, Str},
    Error, MatchRule, MatchRulePathSpec, MessageType, Result,
};

const MAX_ARGS: u8 = 64;

/// Builder for [`MatchRule`].
///
/// This is created by [`MatchRule::builder`].
pub struct MatchRuleBuilder<'m>(MatchRule<'m>);

assert_impl_all!(MatchRuleBuilder<'_>: Send, Sync, Unpin);

impl<'m> MatchRuleBuilder<'m> {
    /// Build the `MatchRule`.
    pub fn build(self) -> MatchRule<'m> {
        self.0
    }

    /// Set the sender.
    pub fn sender<B>(mut self, sender: B) -> Result<Self>
    where
        B: TryInto<BusName<'m>>,
        B::Error: Into<Error>,
    {
        self.0.sender = Some(sender.try_into().map_err(Into::into)?);

        Ok(self)
    }

    /// Set the message type.
    pub fn msg_type(mut self, msg_type: MessageType) -> Self {
        self.0.msg_type = Some(msg_type);

        self
    }

    /// Set the interface.
    pub fn interface<I>(mut self, interface: I) -> Result<Self>
    where
        I: TryInto<InterfaceName<'m>>,
        I::Error: Into<Error>,
    {
        self.0.interface = Some(interface.try_into().map_err(Into::into)?);

        Ok(self)
    }

    /// Set the member.
    pub fn member<M>(mut self, member: M) -> Result<Self>
    where
        M: TryInto<MemberName<'m>>,
        M::Error: Into<Error>,
    {
        self.0.member = Some(member.try_into().map_err(Into::into)?);

        Ok(self)
    }

    /// Set the path.
    ///
    /// Note: Since both a path and a path namespace are not allowed to appear in a match rule at
    /// the same time, this overrides any path namespace previously set.
    pub fn path<P>(mut self, path: P) -> Result<Self>
    where
        P: TryInto<ObjectPath<'m>>,
        P::Error: Into<Error>,
    {
        self.0.path_spec = path
            .try_into()
            .map(MatchRulePathSpec::Path)
            .map(Some)
            .map_err(Into::into)?;

        Ok(self)
    }

    /// Set the path namespace.
    ///
    /// Note: Since both a path and a path namespace are not allowed to appear in a match rule at
    /// the same time, this overrides any path previously set.
    pub fn path_namespace<P>(mut self, path_namespace: P) -> Result<Self>
    where
        P: TryInto<ObjectPath<'m>>,
        P::Error: Into<Error>,
    {
        self.0.path_spec = path_namespace
            .try_into()
            .map(MatchRulePathSpec::PathNamespace)
            .map(Some)
            .map_err(Into::into)?;

        Ok(self)
    }

    /// Set the destination.
    pub fn destination<B>(mut self, destination: B) -> Result<Self>
    where
        B: TryInto<UniqueName<'m>>,
        B::Error: Into<Error>,
    {
        self.0.destination = Some(destination.try_into().map_err(Into::into)?);

        Ok(self)
    }

    /// Append an arguments.
    ///
    /// Use this in instead of [`MatchRuleBuilder::arg`] if you want to sequentially add args.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidMatchRule`] on attempt to add the 65th argument.
    pub fn add_arg<S>(self, arg: S) -> Result<Self>
    where
        S: Into<Str<'m>>,
    {
        let idx = self.0.args.len() as u8;

        self.arg(idx, arg)
    }

    /// Add an argument of a specified index.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidMatchRule`] if `idx` is greater than 64.
    pub fn arg<S>(mut self, idx: u8, arg: S) -> Result<Self>
    where
        S: Into<Str<'m>>,
    {
        if idx >= MAX_ARGS {
            return Err(Error::InvalidMatchRule);
        }
        let value = (idx, arg.into());
        let vec_idx = match self.0.args().binary_search_by(|(i, _)| i.cmp(&idx)) {
            Ok(i) => {
                // If the argument is already present, replace it.
                self.0.args.remove(i);

                i
            }
            Err(i) => i,
        };
        self.0.args.insert(vec_idx, value);

        Ok(self)
    }

    /// Append a path argument.
    ///
    /// Use this in instead of [`MatchRuleBuilder::arg_path`] if you want to sequentially add args.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidMatchRule`] on attempt to add the 65th path argument.
    pub fn add_arg_path<P>(self, arg_path: P) -> Result<Self>
    where
        P: TryInto<ObjectPath<'m>>,
        P::Error: Into<Error>,
    {
        let idx = self.0.arg_paths.len() as u8;

        self.arg_path(idx, arg_path)
    }

    /// Add a path argument of a specified index.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidMatchRule`] if `idx` is greater than 64.
    pub fn arg_path<P>(mut self, idx: u8, arg_path: P) -> Result<Self>
    where
        P: TryInto<ObjectPath<'m>>,
        P::Error: Into<Error>,
    {
        if idx >= MAX_ARGS {
            return Err(Error::InvalidMatchRule);
        }

        let value = (idx, arg_path.try_into().map_err(Into::into)?);
        let vec_idx = match self.0.arg_paths().binary_search_by(|(i, _)| i.cmp(&idx)) {
            Ok(i) => {
                // If the argument is already present, replace it.
                self.0.arg_paths.remove(i);

                i
            }
            Err(i) => i,
        };
        self.0.arg_paths.insert(vec_idx, value);

        Ok(self)
    }

    /// Set 0th argument's namespace.
    pub fn arg0namespace<I>(mut self, namespace: I) -> Result<Self>
    where
        I: TryInto<InterfaceName<'m>>,
        I::Error: Into<Error>,
    {
        self.0.arg0namespace = Some(namespace.try_into().map_err(Into::into)?);

        Ok(self)
    }

    /// Create a builder for `MatchRuleBuilder`.
    pub(crate) fn new() -> Self {
        Self(MatchRule {
            msg_type: None,
            sender: None,
            interface: None,
            member: None,
            path_spec: None,
            destination: None,
            args: Vec::with_capacity(MAX_ARGS as usize),
            arg_paths: Vec::with_capacity(MAX_ARGS as usize),
            arg0namespace: None,
        })
    }
}
