use zbus::{
    Error, MaxDepthExceeded, Variant,
    wire::{LE, serialized::Context, to_bytes},
};

#[test]
fn recursion_limits() {
    let ctxt = Context::new(LE, 0);
    // Total container depth exceeds limit (64)
    let mut value = Variant::from(0u8);
    for _ in 0..64 {
        value = Variant::Variant(Box::new(value));
    }
    assert!(matches!(
        to_bytes(ctxt, &value),
        Err(Error::MaxDepthExceeded(MaxDepthExceeded::Container))
    ));

    // Array depth exceeds limit (32)
    let vec = vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![
        vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![
            vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![
                0u8,
            ]]]]]]]]]]],
        ]]]]]]]]]]],
    ]]]]]]]]]]];
    assert!(matches!(
        to_bytes(ctxt, &vec),
        Err(Error::MaxDepthExceeded(MaxDepthExceeded::Array))
    ));

    // Struct depth exceeds limit (32)
    let tuple = ((((((((((((((((((
        (((((((((((((((0u8,),),),),),),),),),),),),),),),
    ),),),),),),),),),),),),),),),),),);
    assert!(matches!(
        to_bytes(ctxt, &tuple),
        Err(Error::MaxDepthExceeded(MaxDepthExceeded::Structure))
    ));

    // total depth exceeds limit (64) with struct, array and variant.
    let mut value = Variant::from(0u8);
    for _ in 0..32 {
        value = Variant::Variant(Box::new(value));
    }
    let tuple_array =
        (
            ((((((((((((((((vec![vec![vec![vec![vec![vec![vec![vec![vec![
                vec![vec![vec![vec![vec![vec![vec![value]]]]]]],
            ]]]]]]]]],),),),),),),),),),),),),),),),),
        );
    assert!(matches!(
        to_bytes(ctxt, &tuple_array),
        Err(Error::MaxDepthExceeded(MaxDepthExceeded::Container))
    ));

    // TODO:
    //
    // * Test deserializers.
}
