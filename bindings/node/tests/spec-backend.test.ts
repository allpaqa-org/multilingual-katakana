import { describe, expect, test } from "bun:test";
import * as fs from "node:fs";
import * as path from "node:path";
import {
  isNativeBackendAvailableForTests,
  resetNativeBackendCacheForTests,
} from "../src/backend/loader";
import { toKatakana } from "../src/index";

/**
 * Runs the full spec/cases conformance suite once per backend
 * (`js` and `native`), explicitly forcing `MULTILINGUAL_KATAKANA_BACKEND`
 * so a silent auto-fallback to the JS pipeline can never produce a false
 * pass for the native backend (see docs/V0.4.0_BINDINGS_SCOPE.md).
 *
 * The `native` suite is skipped (not failed) when the native addon simply
 * isn't built for the current checkout/platform, but fails hard if the
 * addon is present and loadable yet produces incorrect output.
 */
const casesDir = path.resolve(__dirname, "../../../spec/cases");
const caseFiles = fs
  .readdirSync(casesDir)
  .filter((f) => f.endsWith(".json"))
  .sort();

type Backend = "js" | "native";
interface TestCase {
  id: string;
  description: string;
  input: string;
  expected: string | { canonical: string; accepted?: string[] };
}

function withForcedBackend(backend: Backend, run: () => void): void {
  const previous = process.env.MULTILINGUAL_KATAKANA_BACKEND;
  process.env.MULTILINGUAL_KATAKANA_BACKEND = backend;
  resetNativeBackendCacheForTests();
  try {
    run();
  } finally {
    process.env.MULTILINGUAL_KATAKANA_BACKEND = previous === undefined ? undefined : previous;
    resetNativeBackendCacheForTests();
  }
}

function assertMatchesExpected(actual: string, expected: TestCase["expected"]): void {
  const canonical = typeof expected === "string" ? expected : expected.canonical;
  const accepted =
    typeof expected === "object" && Array.isArray(expected.accepted) ? expected.accepted : [];
  if (actual === canonical || accepted.includes(actual)) {
    expect(true).toBe(true);
    return;
  }
  expect(actual).toBe(canonical);
}

function runCaseAgainstBackend(backend: Backend, tc: TestCase): void {
  withForcedBackend(backend, () => {
    assertMatchesExpected(toKatakana(tc.input), tc.expected);
  });
}

function runForcedBackend(backend: Backend) {
  describe(`Spec conformance (forced backend=${backend})`, () => {
    for (const file of caseFiles) {
      const filePath = path.join(casesDir, file);
      const cases: TestCase[] = JSON.parse(fs.readFileSync(filePath, "utf8"));

      describe(`${file} (${cases.length} cases)`, () => {
        for (const tc of cases) {
          test(`[${tc.id}] ${tc.description}`, () => {
            runCaseAgainstBackend(backend, tc);
          });
        }
      });
    }
  });
}

runForcedBackend("js");

if (isNativeBackendAvailableForTests()) {
  runForcedBackend("native");
} else {
  describe.skip("Spec conformance (forced backend=native)", () => {
    test("skipped: native addon not built for this checkout/platform", () => {
      // Intentionally empty: see crates/multilingual-katakana-node.
    });
  });
}

/**
 * Regression coverage for a real js/native divergence found during v0.4.0
 * review: with `enableEnglish: false`, the Rust core previously chained
 * both Vietnamese and Spanish preprocessing unconditionally, corrupting
 * plain English words (e.g. "hello" -> "heリャo") that the JS pipeline
 * correctly left untouched. See crates/multilingual-katakana-core's
 * `resolve_word` fallback, which now mirrors the JS if/else-if priority.
 */
if (isNativeBackendAvailableForTests()) {
  describe("Backend parity: option-flag combinations", () => {
    const optionCases: Array<{
      input: string;
      options: Parameters<typeof toKatakana>[1];
    }> = [
      { input: "hello", options: { enableEnglish: false } },
      { input: "collab", options: { enableEnglish: false } },
      { input: "hello", options: { enableVietnamese: false } },
      { input: "hello", options: { enableSpanish: false } },
    ];

    for (const { input, options } of optionCases) {
      test(`"${input}" with ${JSON.stringify(options)} matches across backends`, () => {
        let jsResult = "";
        let nativeResult = "";
        withForcedBackend("js", () => {
          jsResult = toKatakana(input, options);
        });
        withForcedBackend("native", () => {
          nativeResult = toKatakana(input, options);
        });
        expect(nativeResult).toBe(jsResult);
      });
    }
  });
}
