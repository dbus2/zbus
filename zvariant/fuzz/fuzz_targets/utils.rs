use zvariant::{
    serialized::{Context, Data},
    to_bytes, Value,
};

pub fn fuzz_for_context(bytes: &[u8], ctx: Context) {
    let data = Data::new(bytes, ctx);
    if let Ok((decoded, _)) = data.deserialize::<Value>() {
        to_bytes(ctx, &decoded).unwrap();
    }
}
