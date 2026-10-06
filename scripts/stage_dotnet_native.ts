import { spawnSync } from "node:child_process";
import * as fs from "node:fs";
import * as path from "node:path";

/**
 * Builds the C ABI crate (crates/multilingual-katakana-ffi) in release mode
 * and stages the host's native library into bindings/dotnet/native/<rid>/,
 * where both `dotnet test` and `dotnet pack` pick it up.
 *
 * Usage:
 *   bun run scripts/stage_dotnet_native.ts
 *
 * Only the host RID is produced; CI stages the other RIDs from its matrix.
 */

const rootDir = path.resolve(__dirname, "..");
const LIB_BASE = "allpaqa_multilingual_katakana";

const OS_MAP: Record<string, { rid: string; file: string }> = {
  win32: { rid: "win", file: `${LIB_BASE}.dll` },
  darwin: { rid: "osx", file: `lib${LIB_BASE}.dylib` },
  linux: { rid: "linux", file: `lib${LIB_BASE}.so` },
};

const ARCH_MAP: Record<string, string> = { x64: "x64", arm64: "arm64", ia32: "x86" };

function isMusl(): boolean {
  if (process.platform !== "linux") return false;
  const report = process.report?.getReport() as { header?: { glibcVersionRuntime?: string } };
  return !report?.header?.glibcVersionRuntime;
}

const os = OS_MAP[process.platform];
const arch = ARCH_MAP[process.arch];
if (!os || !arch) {
  console.error(`Unsupported host: ${process.platform}/${process.arch}`);
  process.exit(1);
}
const rid = `${os.rid}${isMusl() ? "-musl" : ""}-${arch}`;

const build = spawnSync("cargo", ["build", "--release", "-p", "multilingual-katakana-ffi"], {
  cwd: rootDir,
  stdio: "inherit",
});
if (build.status !== 0) process.exit(build.status ?? 1);

const src = path.join(rootDir, "target", "release", os.file);
const destDir = path.join(rootDir, "bindings", "dotnet", "native", rid);
fs.mkdirSync(destDir, { recursive: true });
fs.copyFileSync(src, path.join(destDir, os.file));
console.log(`Staged ${os.file} -> bindings/dotnet/native/${rid}/`);
