import { describe, expect, test } from "bun:test";
import { platformArchTripleForTests } from "../src/backend/loader";

/**
 * Covers the platform/arch -> napi-rs triple mapping for every Tier 1
 * target from `docs/V0.4.0_BINDINGS_SCOPE.md` §5.2, plus the unsupported
 * (Tier 2) fallback. musl detection is exercised precisely by stubbing
 * `process.report.getReport()` for each branch (gnu / musl / a failing
 * report), rather than relying on whatever libc happens to be running the
 * test, so an inverted condition or a swallowed-error regression would
 * actually fail these tests.
 */
describe("platformArchTripleForTests", () => {
  test("darwin-arm64", () => {
    expect(platformArchTripleForTests("darwin", "arm64")).toBe("darwin-arm64");
  });

  test("darwin-x64", () => {
    expect(platformArchTripleForTests("darwin", "x64")).toBe("darwin-x64");
  });

  test("linux-x64 resolves to -gnu when process.report reports glibcVersionRuntime", () => {
    withStubbedReport({ header: { glibcVersionRuntime: "2.31" } }, () => {
      expect(platformArchTripleForTests("linux", "x64")).toBe("linux-x64-gnu");
    });
  });

  test("linux-arm64 resolves to -gnu when process.report reports glibcVersionRuntime", () => {
    withStubbedReport({ header: { glibcVersionRuntime: "2.31" } }, () => {
      expect(platformArchTripleForTests("linux", "arm64")).toBe("linux-arm64-gnu");
    });
  });

  test("linux-x64 resolves to -musl when process.report has no glibcVersionRuntime", () => {
    withStubbedReport({ header: {} }, () => {
      expect(platformArchTripleForTests("linux", "x64")).toBe("linux-x64-musl");
    });
  });

  test("linux-arm64 resolves to -musl when process.report has no glibcVersionRuntime", () => {
    withStubbedReport({ header: {} }, () => {
      expect(platformArchTripleForTests("linux", "arm64")).toBe("linux-arm64-musl");
    });
  });

  test("linux-x64 falls back to -gnu (never throws) when process.report.getReport throws", () => {
    withStubbedReport(
      () => {
        throw new Error("simulated sandboxed runtime failure");
      },
      () => {
        expect(platformArchTripleForTests("linux", "x64")).toBe("linux-x64-gnu");
      },
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

/**
 * Temporarily replaces `process.report.getReport` for the duration of `fn`,
 * restoring the original afterwards even if `fn` throws/asserts.
 */
function withStubbedReport(report: Record<string, unknown> | (() => never), fn: () => void): void {
  const original = process.report?.getReport;
  // @ts-expect-error -- intentionally stubbing a read-only Node.js API for testing
  process.report.getReport = typeof report === "function" ? report : () => report;
  try {
    fn();
  } finally {
    // @ts-expect-error -- restoring the stubbed API
    process.report.getReport = original;
  }
}
