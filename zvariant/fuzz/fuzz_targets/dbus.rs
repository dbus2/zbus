#![no_main]
mod utils;

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    utils::fuzz_for_context(data, zvariant::serialized::Context::new_dbus(endi::LE, 0));
    utils::fuzz_for_context(data, zvariant::serialized::Context::new_dbus(endi::BE, 0));
});
