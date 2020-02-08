use crate::utils::*;
use crate::Signature;

// TODO:
// * proc derive macro to implement VariantValue for structures
pub trait VariantValue {
    /// Get the signature for the implementing type.
    ///
    /// # Example
    ///
    /// TODO
    ///
    fn signature() -> Signature<'static>;
}

impl<V> VariantValue for [V]
where
    V: VariantValue,
{
    #[inline]
    fn signature() -> Signature<'static> {
        Signature::from(format!("a{}", V::signature().as_str()))
    }
}

impl<V> VariantValue for &[V]
where
    V: VariantValue,
{
    #[inline]
    fn signature() -> Signature<'static> {
        <[V]>::signature()
    }
}

impl<V> VariantValue for Vec<V>
where
    V: VariantValue,
{
    #[inline]
    fn signature() -> Signature<'static> {
        <[V]>::signature()
    }
}

// TODO: implement when we support GVariant support
// impl<V> VariantValue for Option<V>

////////////////////////////////////////////////////////////////////////////////

impl<V> VariantValue for [V; 0]
where
    V: VariantValue,
{
    #[inline]
    fn signature() -> Signature<'static> {
        let mut sig = String::with_capacity(255);
        sig.push(STRUCT_SIG_START_CHAR);
        sig.push_str(V::signature().as_str());
        sig.push(STRUCT_SIG_END_CHAR);

        Signature::from(sig)
    }
}

// Arrays are serialized as tupples by Serde so we gotta do the same.
macro_rules! array_impls {
    ($($len:tt)+) => {
        $(
            impl<V> VariantValue for [V; $len]
            where
                V: VariantValue,
            {
                #[inline]
                fn signature() -> Signature<'static> {
                    let mut sig = String::with_capacity(255);
                    sig.push(STRUCT_SIG_START_CHAR);
                    let field_sig = V::signature();
                    for _i in 0..$len {
                        sig.push_str(field_sig.as_str());
                    }
                    sig.push(STRUCT_SIG_END_CHAR);

                    Signature::from(sig)
                }
            }
        )+
    }
}

array_impls! {
    01 02 03 04 05 06 07 08 09 10
    11 12 13 14 15 16 17 18 19 20
    21 22 23 24 25 26 27 28 29 30
    31 32
}

////////////////////////////////////////////////////////////////////////////////

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> VariantValue for ($($name,)+)
            where
                $($name: VariantValue,)+
            {
                #[inline]
                fn signature() -> Signature<'static> {
                    let mut sig = String::with_capacity(255);
                    sig.push(STRUCT_SIG_START_CHAR);
                    $(
                        sig.push_str($name::signature().as_str());
                    )+
                    sig.push(STRUCT_SIG_END_CHAR);

                    Signature::from(sig)
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}
