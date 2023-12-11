#![no_main]
mod utils;

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    utils::fuzz_for_context(
        data,
        zvariant::serialized::Context::new_gvariant(endi::LE, 0),
    );
    utils::fuzz_for_context(
        data,
        zvariant::serialized::Context::new_gvariant(endi::BE, 0),
    );
});
