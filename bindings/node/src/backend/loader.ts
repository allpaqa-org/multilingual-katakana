import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";

/**
 * Shape of the exports from the NAPI-RS native addon
 * (`crates/multilingual-katakana-node`).
 */
export interface NativeBackend {
  toKatakanaNative: (text: string, options?: Record<string, boolean | undefined>) => string;
  nativeSelfCheck: () => boolean;
}

type BackendMode = "auto" | "native" | "js";

function getBackendMode(): BackendMode {
  const raw =
    typeof process !== "undefined" ? process.env?.MULTILINGUAL_KATAKANA_BACKEND : undefined;
  return raw === "native" || raw === "js" || raw === "auto" ? raw : "auto";
}

/**
 * Maps the current Node.js platform/arch to the napi-rs platform-package
 * triple used for prebuilt binary distribution (matches the
 * `@allpaqa/multilingual-katakana-<triple>` optionalDependencies pattern).
 */
function platformArchTriple(): string | null {
  const platform = os.platform();
  const arch = os.arch();
  if (platform === "darwin" && arch === "arm64") return "darwin-arm64";
  if (platform === "darwin" && arch === "x64") return "darwin-x64";
  if (platform === "linux" && arch === "x64") return "linux-x64-gnu";
  if (platform === "linux" && arch === "arm64") return "linux-arm64-gnu";
  if (platform === "win32" && arch === "x64") return "win32-x64-msvc";
  return null;
}

/**
 * Walks up from `startDir` looking for a sibling `native/` directory. This
 * avoids hardcoding a relative path depth, which would otherwise differ
 * between running unbundled TypeScript sources (`src/backend/loader.ts`) and
 * the bundled `dist/index.{js,cjs}` output.
 *
 * Uses `__dirname` (rather than `import.meta.url`) so this resolves
 * identically in both the ESM and CJS build outputs: tsup's `shims: true`
 * option provides a working `__dirname`/`require` in the ESM bundle, while
 * `import.meta` is unavailable in CommonJS output.
 */
function findNativeDir(startDir: string): string | null {
  let dir = startDir;
  for (let i = 0; i < 6; i++) {
    const candidate = path.join(dir, "native");
    if (fs.existsSync(candidate)) return candidate;
    const parent = path.dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  return null;
}

function loadNativeModule(): NativeBackend | null {
  const triple = platformArchTriple();
  if (!triple) return null;

  const candidates: string[] = [
    // Published platform package (installed via optionalDependencies).
    `@allpaqa/multilingual-katakana-${triple}`,
  ];

  const nativeDir = findNativeDir(__dirname);
  if (nativeDir) {
    // Local development / monorepo build artifact (see bindings/node/native/).
    candidates.push(path.join(nativeDir, `multilingual-katakana.${triple}.node`));
  }

  for (const candidate of candidates) {
    try {
      const mod = require(candidate) as NativeBackend;
      if (typeof mod.toKatakanaNative === "function" && typeof mod.nativeSelfCheck === "function") {
        return mod;
      }
    } catch {
      // Try the next candidate.
    }
  }
  return null;
}

let cached: NativeBackend | null | undefined;

/**
 * Returns the native backend if it is available, loadable, enabled by
 * `MULTILINGUAL_KATAKANA_BACKEND`, and passes its self-check; otherwise
 * returns `null` so callers fall back to the pure-TypeScript pipeline.
 *
 * When `MULTILINGUAL_KATAKANA_BACKEND=native` is set and the native addon
 * is unavailable, this throws instead of silently falling back, so CI can
 * force the native backend and catch real regressions rather than getting
 * a false pass from the JS fallback.
 */
export function getNativeBackend(): NativeBackend | null {
  const mode = getBackendMode();
  if (mode === "js") return null;

  if (cached === undefined) {
    const mod = loadNativeModule();
    let ok = false;
    if (mod) {
      try {
        ok = mod.nativeSelfCheck();
      } catch {
        ok = false;
      }
    }
    cached = ok ? mod : null;
  }

  if (mode === "native" && !cached) {
    throw new Error(
      "MULTILINGUAL_KATAKANA_BACKEND=native was set but the native addon is unavailable or failed its self-check.",
    );
  }
  return cached;
}

/** Test-only helper to force re-resolution of the native backend. */
export function resetNativeBackendCacheForTests(): void {
  cached = undefined;
}

/**
 * Test-only helper that checks whether the native addon can be loaded and
 * passes its self-check, without throwing even when
 * `MULTILINGUAL_KATAKANA_BACKEND=native` would normally throw. Used to
 * decide whether forced-native conformance tests should run or be skipped
 * (e.g. on a dev machine where the Rust addon hasn't been built).
 */
export function isNativeBackendAvailableForTests(): boolean {
  const mod = loadNativeModule();
  if (!mod) return false;
  try {
    return mod.nativeSelfCheck();
  } catch {
    return false;
  }
}
