use byteorder::ByteOrder;
use zvariant::{from_slice, to_bytes, EncodingContext as Context, Value};

pub fn fuzz_for_context<B: ByteOrder>(data: &[u8], ctx: Context<B>) {
    if let Ok(decoded) = from_slice::<_, Value>(data, ctx) {
        to_bytes(ctx, &decoded).unwrap();
    }
}
