use crate::{zvariant::ObjectPath, Connection, Error, Result};
use std::convert::TryInto;

#[derive(Clone, Debug)]
pub struct SignalContext<'s> {
    conn: &'s Connection,
    path: ObjectPath<'s>,
}

impl<'s> SignalContext<'s> {
    /// Create a new signal context for the given connection and object path.
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
}
