use std::rc::Rc;

use super::Signature;

/// Signatures of the fields of a [`Signature::Structure`].
#[derive(Debug, Clone)]
pub enum FieldsSignatures {
    Static {
        fields: &'static [&'static Signature],
    },
    Dynamic {
        fields: Rc<[Signature]>,
    },
}

impl FieldsSignatures {
    /// A iterator over the fields' signatures.
    pub fn iter(&self) -> impl Iterator<Item = &Signature> {
        use std::slice::Iter;

        enum Fields<'a> {
            Static(Iter<'static, &'static Signature>),
            Dynamic(Iter<'a, Signature>),
        }

        impl<'a> Iterator for Fields<'a> {
            type Item = &'a Signature;

            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Fields::Static(iter) => iter.next().copied(),
                    Fields::Dynamic(iter) => iter.next(),
                }
            }
        }

        match self {
            FieldsSignatures::Static { fields } => Fields::Static(fields.iter()),
            FieldsSignatures::Dynamic { fields } => Fields::Dynamic(fields.iter()),
        }
    }
}

impl From<Rc<[Signature]>> for FieldsSignatures {
    fn from(fields: Rc<[Signature]>) -> Self {
        FieldsSignatures::Dynamic { fields }
    }
}

impl From<Vec<Signature>> for FieldsSignatures {
    fn from(fields: Vec<Signature>) -> Self {
        FieldsSignatures::Dynamic {
            fields: fields.into(),
        }
    }
}

impl<const N: usize> From<[Signature; N]> for FieldsSignatures {
    fn from(fields: [Signature; N]) -> Self {
        FieldsSignatures::Dynamic {
            fields: fields.into(),
        }
    }
}

impl From<&'static [&'static Signature]> for FieldsSignatures {
    fn from(fields: &'static [&'static Signature]) -> Self {
        FieldsSignatures::Static { fields }
    }
}
