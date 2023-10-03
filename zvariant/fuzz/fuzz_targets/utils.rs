use byteorder::ByteOrder;
use zvariant::{serialized::Data, to_bytes, EncodingContext as Context, Value};

pub fn fuzz_for_context<B: ByteOrder>(bytes: &[u8], ctx: Context<B>) {
    let data = Data::new(bytes, ctx);
    if let Ok((decoded, _)) = data.deserialize::<Value>() {
        to_bytes(ctx, &decoded).unwrap();
    }
}
