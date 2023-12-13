#![no_main]
mod utils;

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    utils::fuzz_for_context(
        data,
        zvariant::serialized::Context::new_dbus(zvariant::LE, 0),
    );
    utils::fuzz_for_context(
        data,
        zvariant::serialized::Context::new_dbus(zvariant::BE, 0),
    );
});
