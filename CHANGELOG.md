# Changelog

Notable changes to this project are documented here.

## [Unreleased]

### Documentation

- Defined the v0.4.0 bindings scope in `docs/V0.4.0_BINDINGS_SCOPE.md`: v0.4.0
  ships a NAPI-RS Node.js native backend behind the unchanged TypeScript API,
  while Python (v0.5.0), C# (v0.6.0), and a standalone WebAssembly package
  follow in later releases.

## [0.3.0] - 2026-09-30

### Added

- Added French dictionary-based phrase and word conversion.
- Added Vietnamese and Thai conversion support.
- Expanded the Spanish dictionary with common phrases and words.
- Increased the shared TypeScript and Rust specification suite to 163 cases.

### Changed

- Updated the release roadmap to target native and multilingual bindings in v0.4.0.

### Breaking

- Added language flags to Rust's public `KatakanaOptions` struct. Callers that
  construct the struct by specifying every field must provide the new fields or
  use `..Default::default()`.

## [0.2.1] - 2026-09-29

### Changed

- Expanded the English pronunciation dictionary with irregular words.
- Improved conversion for common streaming and multi-platform text.

## [0.2.0] - 2026-09-29

### Added

- Added the Rust core crate, including build-time dictionary generation.
- Added shared specification tests for TypeScript and Rust; all 103 cases pass
  in both implementations.
- Added continuous integration, automated npm publishing, and draft release
  workflows.

## [0.1.0] - 2026-09-28

### Added

- Initial release of the zero-runtime-dependency multilingual Katakana
  converter.
- Added the TypeScript/Node.js package and initial multilingual conversion
  dictionaries.
