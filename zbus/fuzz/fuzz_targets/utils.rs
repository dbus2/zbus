use zbus::{
    Variant,
    wire::{
        serialized::{Context, Data},
        to_bytes,
    },
};

pub fn fuzz_for_context(bytes: &[u8], ctx: Context) {
    let data = Data::new(bytes, ctx);
    if let Ok((decoded, _)) = data.deserialize::<Variant>() {
        to_bytes(ctx, &decoded).unwrap();
    }
}
