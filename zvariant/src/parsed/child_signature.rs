use std::{ops::Deref, rc::Rc};

use super::Signature;

/// A child signature of a container signature.
#[derive(Debug, Clone)]
pub enum ChildSignature {
    /// A static child signature.
    Static { child: &'static Signature },
    /// A dynamic child signature.
    Dynamic { child: Rc<Signature> },
}

impl ChildSignature {
    /// The underlying child `Signature`.
    pub fn signature(&self) -> &Signature {
        match self {
            ChildSignature::Static { child } => child,
            ChildSignature::Dynamic { child } => child,
        }
    }
}

impl Deref for ChildSignature {
    type Target = Signature;

    fn deref(&self) -> &Self::Target {
        self.signature()
    }
}

impl From<Rc<Signature>> for ChildSignature {
    fn from(child: Rc<Signature>) -> Self {
        ChildSignature::Dynamic { child }
    }
}

impl From<Signature> for ChildSignature {
    fn from(child: Signature) -> Self {
        ChildSignature::Dynamic {
            child: Rc::new(child),
        }
    }
}

impl From<&'static Signature> for ChildSignature {
    fn from(child: &'static Signature) -> Self {
        ChildSignature::Static { child }
    }
}
