# ADR-0004: Defer per-platform `optionalDependencies` publication to a v0.4.x follow-up

## Context

`docs/V0.4.0_BINDINGS_SCOPE.md` planned for v0.4.0 to ship the NAPI-RS
native backend as a drop-in replacement, distributed the standard NAPI-RS
way: one small "loader" package with `optionalDependencies` on ~8
platform-specific binary packages (`@allpaqa/multilingual-katakana-<platform>`),
each containing a single prebuilt `.node` file for that OS/arch
combination.

Publishing that in the same release as the core native-backend
implementation would require simultaneously solving: reserving 8 new npm
package namespaces, a CI cross-build matrix producing verified binaries for
every target, install/smoke-testing the packed tarballs, and migrating
`publish.yml` from a long-lived `NPM_TOKEN` to OIDC Trusted Publishing
(which also needs to cover the new packages). Bundling all of that into the
v0.4.0 GA would have significantly delayed shipping the (already complete
and tested) native backend code itself.

## Decision

v0.4.0 GA ships the native backend implementation, loader, differential
testing, and Safe Failure fallback, but **without** per-platform npm
distribution. Only a single locally built binary (darwin-arm64) exists
under `bindings/node/native/` at GA time. Because the loader always falls
back to the pure-TypeScript implementation when no native binary is
present or loadable (Safe Failure), this is functionally safe: users get
correct output either way, just without the native speedup until the
follow-up ships.

The remaining distribution work (namespace reservation,
`optionalDependencies` wiring, CI build matrix, OIDC migration, rc
validation, GA promotion) is tracked as a dedicated follow-up (see #15).

## Consequences

- Users who install `@allpaqa/multilingual-katakana` from npm today do not
  yet benefit from the native speedup; they transparently use the
  pure-TypeScript implementation.
- The native backend code, its tests, and its Safe Failure behavior are
  already fully validated and released, so the follow-up work is purely
  additive (packaging/CI/publishing), not further core implementation.
- This intentionally splits "ship the feature" from "ship the
  distribution", accepting a temporary gap in perceived value in exchange
  for lower release risk and a clean rollback boundary if the
  packaging/CI work surfaces problems.
