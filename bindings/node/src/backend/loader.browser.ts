import type { NativeBackend } from "./loader";

/**
 * Browser/Deno/Edge/Worker-safe stand-in for `./loader.ts`.
 *
 * The real loader statically imports Node built-ins (`node:fs`, `node:os`,
 * `node:path`, `node:module`) to locate and `require()` the NAPI-RS native
 * addon. Those modules don't exist outside a Node-compatible runtime, so
 * bundlers targeting the browser (or any environment without Node-API
 * support) must never even attempt to resolve them.
 *
 * This file is selected instead of `./loader.ts` via the package's
 * `#native-backend` subpath import condition (see `package.json`'s
 * `imports` field): esbuild/tsup picks it whenever building with
 * `platform: "browser"`, and Node itself always resolves the `node`
 * condition to the real loader. It always reports the native backend as
 * unavailable, so `KatakanaConverter` falls back to the pure-TypeScript
 * pipeline unconditionally in these environments.
 */
export function getNativeBackend(): NativeBackend | null {
  return null;
}

export function resetNativeBackendCacheForTests(): void {
  // No-op: this stand-in has no cache to reset.
}

export function isNativeBackendAvailableForTests(): boolean {
  return false;
}
