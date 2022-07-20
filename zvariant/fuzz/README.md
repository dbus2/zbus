# Fuzz targets for zvariant

[Fuzzing](https://en.wikipedia.org/wiki/Fuzzing) is a way to test software by feeding it random inputs to make sure it
doesn't crash. This directory contains targets to test zvariant using [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

Run `cargo install cargo-fuzz` to install the fuzzer, then run `cargo fuzz run dbus` or `cargo fuzz run gvariant` to
fuzz the dbus and gvariant versions respectively.