#![no_main]
use libfuzzer_sys::fuzz_target;

use zvariant::{EncodingContext as Context, from_slice, to_bytes, Value};
use byteorder::LE;

fuzz_target!(|data: &[u8]| {
    let ctx = Context::<LE>::new_dbus(0);
    if let Ok(decoded) = from_slice::<_, Value>(data, ctx) {
        to_bytes(ctx, &decoded).unwrap();
    }
});