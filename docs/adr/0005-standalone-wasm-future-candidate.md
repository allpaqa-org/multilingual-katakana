# ADR-0005: Standalone WebAssembly package as a future candidate, not a v0.4.0 fallback

## Context

While designing the NAPI-RS native backend, NAPI-RS itself offers a
standard WASI-based fallback (`wasm32-wasip1-threads`) that can be bundled
alongside the native `.node` binaries so that platforms without a prebuilt
native binary still get *some* acceleration instead of falling back all
the way to pure TypeScript.

However, the project's existing pure-TypeScript implementation is already
a complete fallback that runs correctly on every JS runtime (Node, Bun,
Deno, browsers, Edge/Workers). NAPI-RS's WASI fallback would require
runtime dependencies (`@napi-rs/wasm-runtime`, `@emnapi/*`) and, in
browsers, cross-origin isolation (COOP/COEP headers) — both of which
directly conflict with the project's zero-runtime-dependency principle
(ADR-0006) and its wide-runtime-compatibility promise, for a runtime target
(browser/Deno/Edge) that is already fully served by pure TypeScript.

Separately, a *standalone* WebAssembly build (compiled from the Rust core
via `wasm32-unknown-unknown` + `wasm-bindgen`, not NAPI-RS's WASI output)
was considered as a distinct, optional future artifact whose value would be
"bit-for-bit identity with the Rust core" and a potential future
replacement path for the pure-TypeScript implementation, rather than a
fallback bundled into the main package.

## Decision

- v0.4.0's main package does **not** bundle any WASM fallback. The
  documented decision is that WASM/WASI bundling adds no acceptable value
  over the existing pure-TypeScript fallback, at the cost of new runtime
  dependencies and browser deployment constraints.
- A standalone WASM package (`@allpaqa/multilingual-katakana-wasm`, built
  with `wasm32-unknown-unknown` + `wasm-bindgen`) is recorded as a
  candidate for a *later, separate* release (v0.6.0 or beyond, "demand
  permitting"), never as a dependency of the main package's fallback path.
- Deprecating/retiring the pure-TypeScript implementation in favor of WASM
  is explicitly out of scope until at least v1.0, and only after the
  standalone WASM package has a track record.

## Consequences

- The zero-dependency principle and full-runtime-compatibility promise of
  the main package are preserved through v0.4.0 and beyond; no
  `dependencies`/`optionalDependencies`/`peerDependencies` on WASM runtime
  helpers (`@napi-rs/wasm-runtime`, `@emnapi/*`, `detect-libc`, etc.) are
  permitted in the main package.
- If a standalone WASM package is built later, it must be published and
  versioned independently, and must not be silently pulled in as a
  fallback of the main package without a new ADR revisiting this decision.
- This decision should be revisited if/when Pure TypeScript maintenance
  cost becomes high enough that the trade-off documented here changes.
