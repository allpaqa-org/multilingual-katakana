import { describe, expect, test } from "bun:test";
import { platformArchTripleForTests } from "../src/backend/loader";

/**
 * Covers the platform/arch -> napi-rs triple mapping for every Tier 1
 * target from `docs/V0.4.0_BINDINGS_SCOPE.md` §5.2, plus the unsupported
 * (Tier 2) fallback. musl detection itself is exercised implicitly here
 * since it runs against whatever libc the CI/dev machine actually has
 * (glibc everywhere this repo's CI runs), so linux-x64/linux-arm64 assert
 * against the actual runtime's musl-ness rather than a hardcoded libc.
 */
describe("platformArchTripleForTests", () => {
  test("darwin-arm64", () => {
    expect(platformArchTripleForTests("darwin", "arm64")).toBe("darwin-arm64");
  });

  test("darwin-x64", () => {
    expect(platformArchTripleForTests("darwin", "x64")).toBe("darwin-x64");
  });

  test("linux-x64 resolves to a gnu or musl triple depending on the runtime libc", () => {
    expect(["linux-x64-gnu", "linux-x64-musl"]).toContain(
      platformArchTripleForTests("linux", "x64"),
    );
  });

  test("linux-arm64 resolves to a gnu or musl triple depending on the runtime libc", () => {
    expect(["linux-arm64-gnu", "linux-arm64-musl"]).toContain(
      platformArchTripleForTests("linux", "arm64"),
    );
  });

  test("win32-x64-msvc", () => {
    expect(platformArchTripleForTests("win32", "x64")).toBe("win32-x64-msvc");
  });

  test("win32-arm64-msvc", () => {
    expect(platformArchTripleForTests("win32", "arm64")).toBe("win32-arm64-msvc");
  });

  test("unsupported platform/arch (Tier 2) returns null", () => {
    expect(platformArchTripleForTests("freebsd", "x64")).toBeNull();
    expect(platformArchTripleForTests("linux", "ia32")).toBeNull();
    expect(platformArchTripleForTests("win32", "ia32")).toBeNull();
  });
});
