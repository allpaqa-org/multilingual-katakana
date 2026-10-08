# Changelog

Notable changes to this project are documented here.

## [Unreleased]

## [0.5.0] - 2026-10-08

### Added

- C ABI crate `crates/multilingual-katakana-ffi` and .NET binding
  `Allpaqa.MultilingualKatakana` (`netstandard2.0` / `net8.0`, 9 RIDs, NuGet Trusted
  Publishing) (#32, #34).
- Python binding (`crates/multilingual-katakana-python`, `bindings/python`)
  via PyO3 + maturin with abi3 wheels; CI wheel matrix with PyPI Trusted
  Publishing, publish gated on repository variable `PYPI_PUBLISH_ENABLED`
  (**not published in this release**) (#29, #30).
- `dicts/hanzi_class.json` and its generator `scripts/generate_hanzi_class.ts`.
  The Simplified-only class and the marker evidence are generated from
  Unihan 18.0.0 (pinned; Unicode License v3) at
  generation time only; the Japanese-only class, guard words and
  Chinese-context slang are curated by hand. No runtime dependency is added.
- 46 new `spec/cases/kanji_guard.json` cases (209 shared cases in total at that point).

### Fixed

- French mode detection and fallback readings (#39, #40): implemented French
  mode detection and phonics fallback rules, tightened French mode detection
  and aligned dictionary lookup priority, adding new spec cases in
  `spec/cases/french.json` (reaching 228 shared specification cases in total).
- Safe Kanji Guard: Japanese Kanji in comments that also contain Simplified
  Chinese are no longer read as Pinyin (#36). Chinese detection now classifies
  each maximal CJK ideograph run instead of the whole comment, using
  Simplified-only / Japanese-only character classes, Chinese marker
  characters, a small list of Japanese stream guard words (`初見`, `神回`,
  `了解`, …), kana adjacency, the `、` hint, order-independent propagation
  between linked runs and a whole-comment fallback (where `，` only breaks
  ties when the comment already has Chinese evidence). Strong Chinese
  evidence beats guard words; a guard word at the very start/end of a Chinese
  run is kept as Japanese only when no non-Japanese run is linked on that
  side in a kana-free comment (`了解谢谢` → `了解シエシエ`, but
  `我是台灣人，感謝你們` converts fully), and a kept guard edge never spreads
  Japanese to its neighbours. In comments with kana, guard edges are always
  kept and Chinese never spreads to shared-only runs. In kana-free
  comments that read as Chinese, guard words do not force Japanese on runs
  that themselves read as Chinese (`這個真的最高，大家好`). Space cleanup is now
  limited to converted runs and full-width punctuation, so ASCII text and
  line breaks elsewhere are untouched. `初見歓迎！ 886 谢谢大家` now becomes
  `初見歓迎！バイバイシエシエダージア`. Kana no longer blocks Chinese conversion
  for the whole comment (`初見です 谢谢大家` → `初見です シエシエダージア`), and a
  run that mixes Japanese-only and Simplified-only characters is left
  untouched. Identical behaviour in the TypeScript pipeline and the Rust core.
- The Chinese stream slang `886` is read as `バイバイ` when the comment
  contains a Chinese run and the token is standalone (not `+886`, `886.5`,
  `8,886`, `18860`, …); elsewhere it stays `886`.
- Added `晚上` to the Chinese common words.

### Changed

- CI: Migrated npm package publishing to Trusted Publishing via OIDC (#24).
- CI: Extracted shared Rust/Bun quality-gate composite actions into `.github/actions/` (#25).

### Documentation

- Aligned README conversion examples with actual output (#37).
- Added Package Registries & Organizations documentation section (#42).
- Milestone order change in this release: .NET published as v0.5.0; Python moved to follow pending PyPI organization approval (#43).

## [0.4.1] - 2026-10-01

### Added

- Native npm distribution for all 8 Tier 1 platforms (#15, #22): platform
  packages `@allpaqa/multilingual-katakana-<platform>` (`darwin-arm64`,
  `darwin-x64`, `linux-x64-gnu`, `linux-arm64-gnu`, `linux-x64-musl`,
  `linux-arm64-musl`, `win32-x64-msvc`, `win32-arm64-msvc`), listed as
  exact-version `optionalDependencies` of the main package so npm installs
  only the binary matching the user's platform.
- The native loader now recognises all 8 Tier 1 platforms, adding
  `linux-x64-musl`, `linux-arm64-musl` and `win32-arm64-msvc`; musl is
  detected without a runtime dependency (#21).
- CI workflow `build-native-matrix.yml`: builds each platform's addon, packs
  it as an npm tarball and smoke-tests it (real hardware for glibc, darwin
  and win32; an Alpine container under QEMU for the two musl targets), and
  publishes to npm only when a GitHub Release is published, never on a plain
  tag push (#21, #22).
- `.github/native-platforms.json` and `scripts/generate_native_packages.ts`
  define the platform metadata once and generate both the CI build matrix
  and the 8 platform `package.json` files (#22).
- CI job verifying the native backend on real darwin-x64 hardware
  (`macos-15-intel`) (#20).

### Changed

- `publish.yml` verifies that all 8 platform packages exist on npm at the
  pinned version before publishing the main package; republishing an
  already-published version is skipped instead of failing; a CI job fails
  the release if the tag, the main package version or any platform package's
  version disagree (#22).

### Removed

- Unused `bindings/node/src/dicts/chinese.json`.

### Documentation

- Introduced ADRs (`docs/adr/`), translated `docs/RELEASE_PROCESS.md` into
  English, and linked the open items in `docs/V0.4.0_BINDINGS_SCOPE.md` to
  their tracking issues (#19).
- README: described the supported TTS engines, and linked
  twitch_text_to_speech_bot as a real-world usage example.

The public API (`toKatakana`, `KatakanaConverter`, `KatakanaOptions`) is
unchanged.

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

### Fixed

- Fixed the ESM build never being able to load the native addon: the loader
  now resolves via `createRequire(import.meta.url)` instead of a bare
  `require`, which esbuild only shims correctly for CJS output. Real ESM
  consumers (the package default, `"type": "module"`) previously always
  silently fell back to the JS pipeline.
- Fixed a native/JS output divergence when `enableEnglish: false`: the Rust
  core's word-resolution fallback unconditionally chained Vietnamese and
  Spanish preprocessing, corrupting plain English words (e.g. `"hello"` ->
  `"heリャo"`) that the JS pipeline correctly left untouched. The Rust
  fallback now mirrors the JS if/else-if preprocessing priority exactly.
- Fixed several browser/edge bundler builds breaking due to the loader's
  unconditional top-level `node:fs` / `node:os` / `node:path` imports. The
  backend is now selected via a Node subpath import (`#native-backend`) that
  resolves to a Node-only loader for Node consumers and to a zero-Node-API
  stub for the `browser` export condition, restoring the pure-TypeScript
  pipeline's prior bundler compatibility. `dist/index.browser.js` is now
  built and published for bundlers that respect the `browser` condition.
- Fixed the native backend silently corrupting unpaired UTF-16 surrogates to
  U+FFFD; conversion now falls back to the JS pipeline for input containing
  a lone surrogate, matching the documented Safe Failure requirement in
  `docs/V0.4.0_BINDINGS_SCOPE.md`.
- Fixed non-string input throwing a generic `Error` on the native path
  instead of the same `TypeError` the JS pipeline throws; non-string input
  now always falls back to the JS pipeline.
- Fixed the native addon's directory discovery, which previously matched any
  ancestor directory literally named `native` (risking a false match, e.g.
  an unrelated `node_modules/native` package). It now anchors on this
  package's own `package.json` (`name: "@allpaqa/multilingual-katakana"`).
- Synced 6 slang dictionary entries (`tiktok`, `twitter`, `vtuber`,
  `youtube`, `youtuber`, `yt`) that were present in the TypeScript binding's
  dictionary but missing from the Rust core's, closing a native/JS output
  gap for those words.

### Notes

- Native/JS output parity is guaranteed at `spec/cases` conformance level,
  which already tolerates multiple valid pronunciations per input via each
  case's `expected.canonical` / `expected.accepted` fields (e.g. Cyrillic
  and inverted-punctuation Spanish greetings). A small number of these
  inputs (e.g. Cyrillic "досвидания" and inverted-punctuation Spanish
  greetings) diverge even with default options: `auto` mode returns the
  spec's `canonical` form when the native backend loads, and its `accepted`
  alternative when only the JS pipeline runs. This is a pre-existing
  characteristic of having two independently-maintained pipelines, not a
  regression introduced by the native backend, and does not affect the
  Safe Kanji Guard / Safe Failure guarantees.
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
