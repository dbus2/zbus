# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 6.0.0 - 2026-10-16

### ✨ Added
- Add accessor for all children.
- Gate the D-Bus API behind the comms feature.

### 💥 Breaking
- Return Iterators to access Interface elements.
- Borrow strings from the introspection document. #9
- Unify the error types.

### 👷 CI
- Measure the single-thread benchmarks by CPU simulation.

### ♻️ Changed
- Change Interface to store a single list of child elements.
- Use root wire types in XML handling.
- Port the in-tree callers to zbus::wire.
- Move zvariant into zbus as the wire module.
- Move zbus_names into zbus as the names module.

### 📦 Dependencies
- Drop the serde dependency.
- Bump zvariant to 5.15.

### 📝 Documentation
- Document the wire and names modules.

### 🔥 Removed
- Drop the serde leftovers of the quick-xml parser.
- Drop pre-6.0 deprecated APIs. #1915

### ✅ Testing
- Test that interface members keep document order.

## 5.2.1 - 2026-07-17

### Other
- Updated the following local packages: zvariant, zbus_names.

## 5.2.0 - 2026-07-07

### Added
- ✨ Support the Telepathy introspection extensions. #255
- ✨ Add a parser that builds the introspection tree.

### Changed
- ♻️ Port parsing and writing to the bespoke parser.

### Dependencies
- ➖ Drop the quick-xml dependency.

### Deprecated
- 🗑️ Deprecate the quick-xml error variants.

### Documentation
- 📝 Tidy the error module docs.

### Other
- 🧐 Add real-world introspection test data and benchmarks.

### Testing
- ✅ Test the parser.
- ✅ Add regression test for bare struct arg signatures.

## 5.1.1 - 2026-04-26

### Documentation
- 📝 Configure docs.rs to build for all supported targets.

### Other
- 🤖 Fix formatting of CHANGELOG files.
- 🤖 Use the default header in changelog.

## 5.1.0 - 2026-01-09

### Changed
- 🚚 Update name of Github space from dbus2 to z-galaxy.

### Dependencies
- ⬆️ Update quick-xml to 0.38.

### Fixed
- 🩹 Don't use workspace for local deps.
- 🐛 Raise XML parsing buffer size.

### Other
- 🧑‍💻 Use workspace dependencies.

### Removed
- ➖ Drop `static_assertions` dep.
