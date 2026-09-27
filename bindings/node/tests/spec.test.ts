import { describe, expect, test } from "bun:test";
import * as fs from "node:fs";
import * as path from "node:path";
import { toKatakana } from "../src/index";

const casesDir = path.resolve(__dirname, "../../../spec/cases");
const caseFiles = fs
  .readdirSync(casesDir)
  .filter((f) => f.endsWith(".json"))
  .sort();

for (const file of caseFiles) {
  const filePath = path.join(casesDir, file);
  const cases = JSON.parse(fs.readFileSync(filePath, "utf8"));

  describe(`Spec: ${file} (${cases.length} cases)`, () => {
    for (const tc of cases) {
      test(`[${tc.id}] ${tc.description}`, () => {
        const actual = toKatakana(tc.input);
        const canonical = typeof tc.expected === "string" ? tc.expected : tc.expected.canonical;
        const accepted =
          typeof tc.expected === "object" && Array.isArray(tc.expected.accepted)
            ? tc.expected.accepted
            : [];

        const isMatch = actual === canonical || accepted.includes(actual);
        if (!isMatch) {
          expect(actual).toBe(canonical);
        } else {
          expect(isMatch).toBe(true);
        }
      });
    }
  });
}
