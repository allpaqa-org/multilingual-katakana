import * as fs from "node:fs";
import { createRequire } from "node:module";
import * as os from "node:os";
import * as path from "node:path";

// `createRequire(import.meta.url)` is required (rather than the bare
// `require` global) because tsup/esbuild's `shims: true` option only
// provides `__dirname`/`__filename`-equivalent shims, not a `require`
// global, in the ESM build output (`dist/index.js`). Without this, dynamic
// `require()` calls compile to esbuild's `__require` helper, which throws
// "Dynamic require ... is not supported" under real ESM, silently forcing
// every ESM consumer onto the JS fallback. `import.meta.url` itself is
// polyfilled correctly by the same `shims: true` option in the CJS output,
// so this one expression resolves correctly in both build targets.
const nodeRequire = createRequire(import.meta.url);

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
 * Detects whether the current Linux process is running against musl libc
 * (e.g. Alpine) rather than glibc, without any runtime dependency (this
 * mirrors the dependency-free detection used by napi-rs's own generated
 * `bindings.js`, intentionally reimplemented here rather than pulling in
 * the `detect-libc` package — see ADR-0005 / ADR-0006).
 *
 * `process.report` (available on all supported Node.js versions) reports
 * `header.glibcVersionRuntime` only when linked against glibc, so its
 * absence is a reliable glibc-vs-musl signal without shelling out. The
 * `ldd` fallback only runs on the rare runtime that lacks `process.report`.
 */
function isMusl(): boolean {
  if (typeof process === "undefined" || typeof process.report?.getReport !== "function") {
    try {
      const lddPath = nodeRequire("node:child_process").execSync("which ldd").toString().trim();
      return fs.readFileSync(lddPath, "utf8").includes("musl");
    } catch {
      return true;
    }
  }
  try {
    // `getReport()` is a heavyweight diagnostics API that can enumerate
    // network/handle state; skip that work and guard against any runtime
    // that fails to produce a report at all (sandboxed environments etc.)
    // so a detection hiccup falls back safely instead of throwing out of
    // every `toKatakana` call.
    process.report.excludeNetwork = true;
    const report = process.report.getReport() as { header?: { glibcVersionRuntime?: string } };
    return !report.header?.glibcVersionRuntime;
  } catch {
    return false;
  }
}

/**
 * Resolves the `linux-<arch>-gnu`/`linux-<arch>-musl` triple for a given
 * Linux arch. Split out of `platformArchTriple` purely to keep that
 * function's cyclomatic complexity within the project's limit.
 */
function linuxTriple(arch: string): string | null {
  if (arch !== "x64" && arch !== "arm64") return null;
  return isMusl() ? `linux-${arch}-musl` : `linux-${arch}-gnu`;
}

/**
 * Maps a Node.js platform/arch pair to the napi-rs platform-package triple
 * used for prebuilt binary distribution (matches the
 * `@allpaqa/multilingual-katakana-<triple>` optionalDependencies pattern).
 * Covers all Tier 1 platforms from `docs/V0.4.0_BINDINGS_SCOPE.md` §5.2.
 *
 * Defaults to the real `os.platform()`/`os.arch()`; the parameters exist
 * only so tests can exercise every branch (including musl detection)
 * without needing to run on every target platform.
 */
function platformArchTriple(
  platform: string = os.platform(),
  arch: string = os.arch(),
): string | null {
  if (platform === "darwin" && arch === "arm64") return "darwin-arm64";
  if (platform === "darwin" && arch === "x64") return "darwin-x64";
  if (platform === "linux") return linuxTriple(arch);
  if (platform === "win32" && arch === "x64") return "win32-x64-msvc";
  if (platform === "win32" && arch === "arm64") return "win32-arm64-msvc";
  return null;
}

/**
 * Walks up from `startDir` looking for this package's own root (identified
 * by a `package.json` whose `name` is `@allpaqa/multilingual-katakana`),
 * then returns its `native/` subdirectory if present. Anchoring on the
 * package's own manifest (rather than matching any directory literally
 * named `native`) avoids accidentally resolving into an unrelated
 * `node_modules/native` package or a consumer's own `native/` folder when
 * this package is installed as a dependency.
 *
 * Uses `__dirname` (rather than `import.meta.url`) so this resolves
 * identically in both the ESM and CJS build outputs: tsup's `shims: true`
 * option provides a working `__dirname` in both bundles, while `import.meta`
 * is unavailable in CommonJS output.
 */
function findNativeDir(startDir: string): string | null {
  let dir = startDir;
  for (let i = 0; i < 6; i++) {
    const pkgJsonPath = path.join(dir, "package.json");
    if (fs.existsSync(pkgJsonPath)) {
      try {
        const pkg = JSON.parse(fs.readFileSync(pkgJsonPath, "utf8")) as { name?: string };
        if (pkg.name === "@allpaqa/multilingual-katakana") {
          const candidate = path.join(dir, "native");
          return fs.existsSync(candidate) ? candidate : null;
        }
      } catch {
        // Malformed package.json; keep walking up.
      }
    }
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
      const mod = nodeRequire(candidate) as NativeBackend;
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

/**
 * Test-only re-export of the platform/arch-to-triple mapping (including
 * musl detection) so all Tier 1 platform branches can be exercised without
 * needing to actually run on every target OS/arch/libc combination.
 */
export const platformArchTripleForTests = platformArchTriple;
