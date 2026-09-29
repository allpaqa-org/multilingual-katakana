import { defineConfig } from "tsup";

export default defineConfig([
  {
    // Primary Node.js build: `import`/`require` consumers get the full
    // pipeline, including the NAPI-RS native backend attempt (behind
    // `#native-backend` -> `./backend/loader.ts`, the `node` import
    // condition).
    entry: ["src/index.ts"],
    format: ["esm", "cjs"],
    dts: true,
    clean: true,
    sourcemap: false,
    minify: false,
    shims: true,
  },
  {
    // Browser/Deno/Edge/Worker build: esbuild's `platform: "browser"`
    // resolves `#native-backend` to its `default` condition
    // (`./backend/loader.browser.ts`), a stub that never references Node
    // built-ins and always reports the native backend unavailable, so the
    // pure-TypeScript pipeline runs unconditionally. Exposed via the
    // package's `browser` export condition; unused by Node/Bun consumers.
    entry: { "index.browser": "src/index.ts" },
    format: ["esm"],
    platform: "browser",
    dts: false,
    clean: false,
    sourcemap: false,
    minify: false,
    shims: true,
  },
]);
