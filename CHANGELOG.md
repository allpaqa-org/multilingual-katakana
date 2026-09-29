# Changelog

Notable changes to this project are documented here.

## [Unreleased]

## [0.4.0] - 2026-09-30

### Added

- Added a native Node.js backend (`crates/multilingual-katakana-node`) built
  with NAPI-RS. It re-implements the full conversion pipeline in Rust and is
  used automatically by `toKatakana` / `KatakanaConverter` when a
  platform-matching addon is present and passes a runtime self-check;
  otherwise the existing pure-TypeScript pipeline runs unchanged. The public
  TypeScript API (`toKatakana`, `KatakanaConverter`, `KatakanaOptions`) is
  unchanged. `exclude` (`RegExp`/string) continues to be resolved entirely on
  the JS side (PUA escape/restore) before/after the native call, since JS
  and Rust regex semantics differ.
- Added `MULTILINGUAL_KATAKANA_BACKEND` (`auto` | `native` | `js`) to force a
  specific backend; `native` throws instead of silently falling back if the
  addon is unavailable, so CI can catch real regressions.
- Added backend-parameterized `spec/cases` conformance tests
  (`bindings/node/tests/spec-backend.test.ts`) that run every case against
  both the `js` and `native` backends explicitly.
- Added a CI step that builds the native addon for the Linux CI runner and
  runs the full test suite with the native backend forced on, in addition to
  the default auto-detected run.

### Notes

- This release ships the native backend **architecture** and Linux-CI /
  local-dev build support. Publishing per-platform npm packages
  (`optionalDependencies`, e.g. `@allpaqa/multilingual-katakana-darwin-arm64`)
  for zero-build end-user installs is tracked as v0.4.x follow-up work; until
  those packages are published, npm consumers continue to get the
  pure-TypeScript pipeline with no behavior change. See
  `docs/V0.4.0_BINDINGS_SCOPE.md` for the full phased plan.

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
