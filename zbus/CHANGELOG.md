# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 6.0.0 - 2026-10-16

### ✨ Added
- Run connections on Tokio on WASI.
- Add spawn_process to traits::Runtime.
- Add connect_tcp and connect_unix to traits::Runtime.
- Add a flush hook to WriteHalf.
- Run the built-in runtime on the thread inside block_on.
- Add a reactor over poll(2) and select with its own timers.
- Add a task scheduler with no thread of its own.
- Give the connection locks of zbus's own.
- Make block_on part of the public API.
- Do every socket's I/O through the connection's runtime.
- Make a build with neither backend valid.
- Accept any runtime through Builder::runtime.
- Add the traits through which a runtime serves a connection.
- Add FilePath::as_c_str.
- Mark the builders and their setters as must_use.
- Defer connection builder errors to build().
- Defer message builder errors to build().
- Defer proxy builder errors to build().
- Defer match rule builder errors to build().
- Accept more typed values in name and path conversions.
- Add Structure::builder.
- Expose common wire types at the crate root.
- Use Serde and Type for interface properties.
- Use Serde and Type for proxy properties.
- Add the explicit Tokio VSOCK builder.
- Add the explicit async-io VSOCK builder.
- Add the Tokio TCP stream builder.
- Add the Tokio Unix stream builder.
- Gate the D-Bus API behind the comms feature.
- add ConnectionCredentials::into_process_fd.

### 💥 Breaking
- Return Iterators to access Interface elements.
- Declare AsyncDrop without async-trait.
- Drop the From<hex::FromHexError> impl of Error.
- Drop the unused lifetime of StructureSeed.
- Return StartServiceReply from start_service_by_name.
- Drop the VSOCK transport.
- Drop the enumflags2 feature.
- Make MethodFlags a bitflags type.
- Make message::Flags a bitflags type.
- Make RequestNameFlags a bitflags type.
- Take and give queue sizes as NonZeroUsize.
- Hand out listeners of zbus's own types.
- Replace the async-io feature and crates with builtin-runtime.
- Remove the blocking API.
- Stop generating blocking proxies.
- Drop the properties cache from proxies without properties.
- Drop the format from the wire Context.
- Drop the Format dispatch from Signature.
- Unify the error types.

### 👷 CI
- Measure the single-thread benchmarks by CPU simulation.

### ♻️ Changed
- Rename DispatchResult2 to DispatchResult.
- Convert Interest to zruntime's in a private function.
- Poll the runtime-less block_on's future without futures-lite.
- Race a timeout's future and its sleep without futures-lite.
- Take the next item of a stream without futures-lite.
- Rename Value to Variant. #1947
- Register sockets with Tokio on Windows through its own stream.
- Make the unixexec and ibus features non-default.
- Connect to an address without cloning it.
- Remove redundant downcast checks.
- Take the blocking-work thread from zruntime.
- Take the async locks from zruntime.
- Rename the zruntime feature to default-rt.
- Hand blocking work's outcome over on its own lock.
- Rename the builtin-runtime feature to zruntime.
- Move the runtime benchmarks to zruntime.
- Build the built-in runtime on the zruntime crate.
- Give each thread that calls block_on a runtime of its own.
- Run the default connection on zbus's own runtime.
- Stop using the blocking API.
- Run transport helper processes on the connection's runtime.
- Hand blocking work to the runtime when it is asked for.
- Connect every transport on the connection's runtime.
- Dispatch every runtime arm through the trait.
- Take a connection's tasks and timers from one runtime.
- Rename runtime/async_lock.rs to locks.rs.
- Rename the `abstractions` module to `runtime`.
- Rename Body::deserialize_unchecked.
- Make `tracing` an optional feature. #331
- Put the service-side ObjectManager behind a feature.
- Put the ibus transport behind a feature.
- Put the unixexec transport behind a feature.
- Keep the signal path non-generic.
- Keep the method call path non-generic.
- Build signature mismatch errors out of line.
- Write introspection doc comments through a helper.
- Convert method handler errors in one place.
- Build replies in one non-generic step.
- Write message bodies through a trait object.
- Use the root wire API in callers and examples.
- Add `service` feature. #1135
- Add `proxy` feature.
- Declare the standard interface proxies separately.
- Convert property values in memory.
- Remove no-op `async-fs` cargo feature. #1287
- Gate integration tests in modules.
- Migrate VSOCK stream callers.
- Migrate TCP stream callers.
- Migrate Unix stream callers.
- Rename zvariant_utils to zbus_utils.
- Tidy the imports the crate merge left behind.
- Move the zvariant derives into zbus_macros.
- Port the in-tree callers to zbus::wire.
- Move zvariant into zbus as the wire module.
- Move zbus_names into zbus as the names module.
- Reach the bus name types through the re-export.
- Deduplicate the default-interface name checks.

### 📦 Dependencies
- Update tokio to v1.53.3 (#2028).
- Drop futures-lite from the library's dependencies.
- Drop the Tokio features nothing uses.
- Drop the async-recursion dependency.
- Move to zruntime's broadcast channel.
- Build on zruntime's Event instead of event-listener.
- Move to zruntime's shared runtime API.
- Drop the async-lock feature and crate.
- Drop proc-macro-crate (#1365).
- Bump zvariant to 5.15.

### 🗑️ Deprecated
- Add deprecated zvariant aliases for the wire module.

### 📝 Documentation
- Document the public items of fdo, proxy and the error types.
- Document the public items of connection and address.
- Document the public items of names, message and match_rule.
- Document the public items of the wire module.
- Say how the set_max_queued methods treat a smaller size.
- Say what block_on runs on with the tokio feature.
- List the object-manager feature in the README.
- Document the support of WASI. #1452
- Point the book at the runtime traits' docs.
- Move the example Socket implementation to the trait's docs.
- Document the runtime trait's connects and helper process.
- Document the fields of `Error::Connection`.
- Document the built-in runtime.
- Point synchronous programs at zbus::block_on.
- Document the runtime and socket changes of 6.0.
- Correct the wire module feature overview.
- Document the `proxy` and `service` features.
- Add usage example to ObjectManager docs.
- Document async-io TCP stream stability.
- Document async-io Unix stream stability.
- Correct the coexistence story for the renamed crate.
- Update the book for zbus 6.0.
- Document the wire and names modules.
- Make two intra-doc links feature-independent.

### 🐛 Fixed
- Fail where WASI has no default bus address or machine ID.
- Make `PropertyStream` yield changes if service starts later.
- Convert `Message` to `Error::FDO` for matching method errors.
- End a connection whose socket reader task is gone.
- Accept a map in as_value::Deserialize.
- Carry any D-Bus error in `zbus::Error`.
- Let property handlers fail with any `DBusError`. #313
- Make `ObjectServer::at` hand back the interface on conflict. #235
- Decide blocking proxy generation on the zbus side.
- Fix a small build issue for unix targets.
- Reject non-basic dictionary keys. #1637
- Keep the async executor on async-io.
- Don't leak auto-created nodes when interface addition fails.
- Don't destroy objects still in use on interface removal.
- Prune empty ancestor nodes on interface removal. #1916

### ⚙️ Other
- 🚨 Warn about undocumented public items.
- 📦️ Describe the wire format in the package metadata.
- ✏️ Fix the proxy name in the README example.
- 💣 Ask for no process ID on WASI.
- 🛂 Authenticate as the user `Builder::user_id` names on WASI.
- 🔌 Have no unix-domain connect on WASI.
- 🔌 Do a connection's socket I/O on WASI.
- 🧵 Run blocking work on the calling thread on WASI.
- ✏️ Fix a typo in the AuthMechanism docs.
- 🧑‍💻 Convert lifetime-free newtypes from Values of any lifetime.
- 💬 Reword the `Error::Connection` message.
- 🦺 Replace unsafe downcast API w/ std's.
- 🧵 Use a single-threaded Tokio runtime in block_on by default.
- 🚨 Collapse nested `if`s into let chains.
- 🚨 Use `MAX` rather than the deprecated `max_value()` in examples.
- 🚨 Box the recursive connect calls instead of using async-recursion.
- 💬 Don't name the Tokio runtime flavour in block_on's panic message.
- 🚨 Fix clippy warnings in tests.
- 🏷️ Use String as owned name NoneType.
- 🏷️ Implement Basic for scalar representations.
- 🏷️ Use BitFlags' numeric wire type.
- 🚨 Fix clippy drain_collect lints.

### ⚡️ Performance
- Pipeline the whole client-side handshake.
- Collect an exited helper process without a blocking worker.
- Hand a task's name over instead of copying it.
- Queue a spawned task under the lock that registers it.
- Keep a task's output in its own cell.
- Give a block_on the seat while the helper thread waits.
- Look the runtime up once per block_on waker, not per wake.
- Hash task ids as themselves in the live map.
- Drop a finished task's handle without a cancellation error.
- Serialize the message header in one pass.
- Serialize message bodies in one pass.
- Box the address in Error::Connection.
- Sort interfaces by name in managed objects.
- Don't require an owned value for property setters.

### 🔥 Removed
- Drop the xdg-dbus-proxy workarounds. #781
- Drop pre-6.0 deprecated APIs. #1915
- Remove the feature-dependent VSOCK builder.
- Remove the feature-dependent TCP builder.
- Remove the feature-dependent Unix builder.

### 🔒️ Security
- Make FilePath to std conversions sound & lossless. #1906

### ✅ Testing
- Test a flags type defined with bitflags.
- Test `PropertyStream` of a service that starts later.
- Let criterion choose the benchmarks' time and sample count.
- Time only the operation in the per-pair runtime benchmarks.
- Run the concurrent calls benchmark's ends on their own threads.
- Benchmark what a connection costs on its runtime.
- Don't assume little-endian host in dict_value test.
- Prove a one-thread runtime runs a connection with no zbus thread.
- Pass the unixexec test where systemd-stdio-bridge is absent.
- Test that a message rejects a maybe body signature.
- Cover Serde property conversion.
- Test message streams with each runtime.
- Test additive VSOCK builder signatures.
- Test additive TCP builder signatures.
- Test additive Unix builder signatures.

## 5.19.0 - 2026-08-09

### Added
- ✨ Add Builder::async_io_unix_stream and async_io_tcp_stream.
- ✨ Make the tokio and async-io features additive.

### Changed
- ♻️ Remove unnecessary clones.

### Deprecated
- 🗑️ Deprecate unix_stream/tcp_stream when tokio is disabled.

### Documentation
- 📝 Document the tokio and async-io runtime selection.

### Fixed
- 🥅 throw Error::Connection on connection failure.
- 🥅 Add new error variant Error::Connection.

### Other
- ✏️ Fix a typo in the executor docs.

### Testing
- ✅ Test runtime selection with both backends enabled.
- ✅ add tests for Error::Connection.

## 5.18.0 - 2026-07-17

### Added
- ✨ Add Connection::closed() and is_closed() API.

### Documentation
- 📝 Document ObjectManager-ancestor deadlock in ObjectServer::at.

### Other
- 🚨 Replace manual match-and-return with the ? operator.

### Testing
- ✅ Test registering an interface from a &mut self callback.

## 5.17.0 - 2026-07-07

### Changed
- ♻️ Move pending method call to separate module.

### Other
- 🧐 Add concurrent method-call benchmark.

### Performance
- ⚡️ Route method replies directly by serial.

## 5.16.0 - 2026-05-27

### Changed
- 🔧 Restrict vsock features to Linux targets.

### Documentation
- 📝 Replace docs.rs all-features with explicit feature list.

### Fixed
- 🚑️ Fix sendmsg for iOS and other apple OS.

### Other
- 🔊 warn on GetAll error during property cache init. #1325

## 5.15.0 - 2026-04-26

### Added
- ✨ Introduce DispatchResult2 with fdo::Result for dispatch futures.
- ✨ Add Builder::build_message_stream.

### Changed
- ♻️ Port Interface and dispatch sites to DispatchResult2.

### Deprecated
- 🗑️ Deprecate DispatchResult in favour of DispatchResult2.

### Documentation
- 📝 Configure docs.rs to build for all supported targets.

### Testing
- ✅ Cover D-Bus error name preservation on property setters.
- ✅ Explicitly choose host endianess in a test.

## 5.14.0 - 2026-02-22

### Added
- ✨ Add helper for IBus connection creation. #964
- 🚸 Add Display trait to D-Bus name request reply types.

### Changed
- 🔧 Extend process module run() to all Unix platforms.

### Fixed
- 🐛 Do not use SendFlags::NOSIGNAL on Redox.

### Other
- 📦️ Add async-recursion for Unix targets.
- 🚨 silence unused import on windows.
- 🚨 silence unused warning on windows test.

## 5.13.2 - 2026-01-19

### Fixed
- 🐛 fix regression on windows build. #1686
- 🐛 Correct Peer interface to work on any arbitrary object path.

## 5.13.1 - 2026-01-11

### Fixed
- 🐛 Implement `get_machine_id()` for *BSD platforms.

## 5.13.0 - 2026-01-09

### Added
- ✨ Add crate attribute for custom crate paths.

### Changed
- 🎨 Format all files (rust 1.85).

### Fixed
- 🚑️ Send on unix sockets w/ `MSG_NOSIGNAL` flag enabled. #1657
- 🐛 Fix `get_machine_id` for macOS.

### Other
- 🧱 Fix all clippy warnings (rust 1.85).
- 🧑‍💻 Bump rust version to 1.85.
- 🔊 lower trace/instrument verbosity.

### Testing
- ✅ Add introspection test for out_args with single output.
- ✅ Remove unused imports from tests.
