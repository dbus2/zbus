use crate::{
    names::{BusName, MemberName},
    zvariant::ObjectPath,
    Connection, Error, Interface, Result,
};
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct SignalEmitter<'s> {
    conn: &'s Connection,
    path: ObjectPath<'s>,
}

impl<'s> SignalEmitter<'s> {
    /// Create a new signal emitter for the given connection and object path.
    pub fn new<P>(conn: &'s Connection, path: P) -> Result<Self>
    where
        P: TryInto<ObjectPath<'s>>,
        P::Error: Into<Error>,
    {
        path.try_into()
            .map(|p| Self { conn, path: p })
            .map_err(Into::into)
    }

    /// Get a reference to the associated connection.
    pub fn connection(&self) -> &'s Connection {
        self.conn
    }

    /// Get a reference to the associated object path.
    pub fn path(&self) -> &ObjectPath<'s> {
        &self.path
    }

    #[doc(hidden)]
    pub fn emit<'d, 'i, 'm, D, I, M, B>(
        &self,
        dest: Option<D>,
        signal_name: M,
        body: &B,
    ) -> Result<()>
    where
        D: TryInto<BusName<'d>>,
        I: Interface,
        M: TryInto<MemberName<'m>>,
        D::Error: Into<Error>,
        M::Error: Into<Error>,
        B: serde::ser::Serialize + zvariant::DynamicType,
    {
        self.conn
            .emit_signal(dest, &self.path, I::name(), signal_name, body)
    }
}
