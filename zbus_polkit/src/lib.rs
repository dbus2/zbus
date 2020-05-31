//! A crate to interact with PolicyKit (with [zbus]).
//!
//! [PolicyKit] is a toolkit for defining and handling authorizations. It is used for allowing
//! unprivileged processes to speak to privileged processes.
//!
//! ```no_run
//! use zbus::Connection;
//! use zbus_polkit::policykit1::*;
//!
//! let c = Connection::new_system().unwrap();
//! let p = AuthorityProxy::new(&c).unwrap();
//! let subject = Subject::new_for_owner(std::process::id(), None, None).unwrap();
//! let result = p.check_authorization(
//!     &subject,
//!     "org.zbus.BeAwesome",
//!     std::collections::HashMap::new(),
//!     CheckAuthorizationFlags::AllowUserInteraction,
//!     "",
//! );
//! ```
//!
//! [PolicyKit]: https://gitlab.freedesktop.org/polkit/polkit/
//! [zbus]: https://gitlab.freedesktop.org/zeenix/zbus

mod error;
pub use error::*;

pub mod policykit1;
