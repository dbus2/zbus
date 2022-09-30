#![no_main]
mod utils;

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    utils::fuzz_for_context(data, zvariant::EncodingContext::<byteorder::LE>::new_dbus(0));
    utils::fuzz_for_context(data, zvariant::EncodingContext::<byteorder::BE>::new_dbus(0));
});
