# Fuzz targets for zvariant

[Fuzzing](https://en.wikipedia.org/wiki/Fuzzing) is a way to test software by feeding it random
inputs to make sure it doesn't crash. This directory contains targets to test zvariant using
[cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

Run `cargo install cargo-fuzz` to install the fuzzer, then run `cargo +nightly fuzz run dbus` or
`cargo +nightly fuzz run gvariant` from the `zvariant` directory to fuzz the dbus and gvariant
deserializers respectively.
