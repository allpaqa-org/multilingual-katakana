# ADR-0001: Use `createRequire(import.meta.url)` to load the native backend from ESM

## Context

`bindings/node/src/backend/loader.ts` needs to conditionally `require()` the
NAPI-RS native addon (`crates/multilingual-katakana-node`) at runtime, since
native `.node` binaries cannot be `import`-ed with a static ESM `import`
statement.

The package is built with tsup/esbuild into both an ESM (`dist/index.js`)
and a CJS (`dist/index.cjs`) bundle, using esbuild's `shims: true` option.
That option provides `__dirname`/`__filename`-equivalent shims for the ESM
output, but it does **not** provide a `require` global. Using the bare
`require` identifier in source that is compiled to real ESM causes esbuild
to rewrite it into its own `__require` helper, which throws `Dynamic
require of "..." is not supported` at runtime — silently forcing every ESM
consumer onto the pure-TypeScript fallback and defeating the purpose of the
native backend.

## Decision

We call `createRequire(import.meta.url)` from `node:module` once at module
scope in `loader.ts` and use the resulting `nodeRequire` function instead of
the bare `require` global:

```ts
const nodeRequire = createRequire(import.meta.url);
```

`import.meta.url` is correctly polyfilled by the same `shims: true` esbuild
option in the CJS build target, so this single expression resolves
correctly in both the ESM and CJS build outputs without any conditional
branching in source.

## Consequences

- Native addon loading works identically whether the consumer imports the
  ESM or CJS build of the package.
- This pattern is Node-only: `loader.browser.ts` (selected via the
  `#native-backend` subpath import condition, see ADR-0002) never touches
  `createRequire` and always reports the native backend as unavailable.
- Any future refactor of the build pipeline (e.g. dropping esbuild's
  `shims` option, or switching bundlers) must re-verify that
  `import.meta.url` is still available in the CJS output, or this call will
  need to be replaced.
