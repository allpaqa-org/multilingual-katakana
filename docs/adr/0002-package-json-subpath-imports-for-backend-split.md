# ADR-0002: Use Node.js `package.json` subpath imports (`#native-backend`) to split browser/Node code paths

## Context

The native NAPI-RS backend loader (`bindings/node/src/backend/loader.ts`)
statically imports Node built-ins (`node:fs`, `node:os`, `node:path`,
`node:module`) to locate and `require()` the native addon. These modules do
not exist in the browser, Deno, Cloudflare Workers, or other non-Node
runtimes, so any code path that even *attempts* to resolve them must never
be reached — or even bundled — when building for those targets.

`bindings/node/src/converter.ts` (the shared entry point used by both the
Node and browser builds) needs a single import statement that resolves to
the real loader under Node and to a safe no-op stand-in everywhere else,
without branching logic at the call site.

## Decision

We declare a package.json `imports` subpath condition:

```json
"imports": {
  "#native-backend": {
    "node": "./src/backend/loader.ts",
    "default": "./src/backend/loader.browser.ts"
  }
}
```

and import it from `converter.ts` as `import { getNativeBackend } from
"#native-backend"`. Node always resolves the `node` condition to the real
loader. esbuild/tsup resolves the `default` condition to
`loader.browser.ts` whenever building with `platform: "browser"`.
`loader.browser.ts` re-exports the same function signatures as the real
loader but always reports the native backend as unavailable, so
`KatakanaConverter` falls back to the pure-TypeScript pipeline
unconditionally in these environments.

## Consequences

- Browser/Deno/Edge bundles never contain references to `node:fs`,
  `node:os`, `node:path`, or `node:module`, so bundlers that error on
  unresolvable Node built-ins (or that would otherwise pull in large
  polyfills) are unaffected.
- The public API and behavior are identical across environments: only the
  presence of the native speedup differs, and that difference is invisible
  to callers (Safe Failure).
- Adding a new backend-loading environment (e.g. a future WASM loader)
  means adding another `imports` condition key here rather than branching
  inside `converter.ts`.
- This relies on the bundler correctly setting `platform: "browser"` (or
  the equivalent condition) during the browser build step; if that build
  configuration drifts, browser bundles could silently pull in the
  Node-only loader.
