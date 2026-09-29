# ADR-0006: Enforce a zero-runtime-dependencies principle

## Context

`@allpaqa/multilingual-katakana` targets streaming/TTS pipelines where it
is embedded as one small piece of a larger tool (bots, VOICEVOX/OpeJTalk
integrations, etc.). Consumers value install size, supply-chain surface
area, and predictable behavior across every JS runtime (Node, Bun, Deno,
browsers, Edge/Workers) over marginal convenience gained from third-party
libraries. Phoneme tables and dictionary data are large enough that
depending on a runtime data-loading or ML library would be tempting, but
would reintroduce exactly the install-size and supply-chain risk the
package is designed to avoid.

## Decision

- `package.json`'s `dependencies` field is always kept empty (`{}`). This
  is treated as an invariant, not a preference.
  ([`docs/V0.4.0_BINDINGS_SCOPE.md` §NAPI-RS scope](../V0.4.0_BINDINGS_SCOPE.md)
  explicitly forbids adding third-party packages such as
  `@napi-rs/wasm-runtime`, `@emnapi/*`, or `detect-libc` to `dependencies`,
  `optionalDependencies`, or `peerDependencies` — see ADR-0005.)
- Phoneme tables and dictionary data are pre-compiled at build time (see
  `scripts/build_dictionaries.ts`) and inlined into the bundled output,
  rather than fetched or loaded via a runtime dependency.
- Any native acceleration (see ADR-0004) must degrade to the pure
  TypeScript implementation with zero required third-party runtime
  dependencies if the native artifact is unavailable (Safe Failure).

## Consequences

- Consumers can audit the package's entire runtime dependency surface by
  reading a single, permanently empty `dependencies` field.
- Every future feature (new language support, native backends, WASM
  candidates) must budget for pre-compilation/bundling data at build time
  instead of reaching for a runtime library, which constrains — but does
  not block — feature velocity.
- This principle is treated as one of the small number of cross-cutting,
  durable decisions worth recording as an ADR (per the criteria proposed
  in #14), rather than routine implementation detail.
