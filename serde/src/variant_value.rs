use crate::Signature;

// TODO:
// * proc derive macro to implement VariantValue for structures
pub trait VariantValue: Clone {
    /// Get the signature for the implementing type.
    ///
    /// # Example
    ///
    /// TODO
    ///
    fn signature() -> Signature<'static>;
}

impl<V> VariantValue for &[V]
where
    V: VariantValue,
{
    fn signature() -> Signature<'static> {
        Signature::from(format!("a{}", V::signature().as_str()))
    }
}
