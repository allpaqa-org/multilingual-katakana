import * as fs from "fs";
import * as path from "path";

const rootDir = path.resolve(__dirname, "..");
const specDir = path.join(rootDir, "spec");
const casesDir = path.join(specDir, "cases");
const dictsDir = path.join(rootDir, "dicts");

console.log("=== Validating JSON files & Schema ===");

// 1. Check all JSON files in dicts/
const dictFiles = fs.readdirSync(dictsDir).filter((f) => f.endsWith(".json"));
for (const file of dictFiles) {
  const filePath = path.join(dictsDir, file);
  const raw = fs.readFileSync(filePath, "utf8");
  try {
    const parsed = JSON.parse(raw);
    const formatted = JSON.stringify(parsed, null, 2) + "\n";
    if (raw !== formatted) {
      console.warn(`Warning: ${file} formatting differs from 2 spaces`);
    } else {
      console.log(`✓ dicts/${file} is valid 2-space formatted JSON`);
    }
  } catch (err) {
    console.error(`✗ Error parsing ${file}:`, err);
    process.exit(1);
  }
}

// 2. Check spec/schema.json
const schemaRaw = fs.readFileSync(path.join(specDir, "schema.json"), "utf8");
const schema = JSON.parse(schemaRaw);
console.log(`✓ spec/schema.json is valid JSON`);

// Simple schema validator for TestCase
function validateTestCase(tc: any, file: string, index: number): string[] {
  const errors: string[] = [];
  if (!tc.id || typeof tc.id !== "string") {
    errors.push(`Case #${index} missing string 'id'`);
  }
  if (!tc.language || typeof tc.language !== "string") {
    errors.push(`Case #${index} missing string 'language'`);
  }
  if (typeof tc.input !== "string") {
    errors.push(`Case #${index} missing string 'input'`);
  }
  if (!tc.expected) {
    errors.push(`Case #${index} missing 'expected'`);
  } else if (typeof tc.expected === "object") {
    if (!tc.expected.canonical || typeof tc.expected.canonical !== "string") {
      errors.push(`Case #${index} expected object missing 'canonical' string`);
    }
    if (tc.expected.accepted && !Array.isArray(tc.expected.accepted)) {
      errors.push(`Case #${index} expected.accepted is not an array`);
    }
  } else if (typeof tc.expected !== "string") {
    errors.push(`Case #${index} expected must be object or string`);
  }
  if (!tc.category || typeof tc.category !== "string") {
    errors.push(`Case #${index} missing string 'category'`);
  }
  if (!Array.isArray(tc.tags) || tc.tags.some((t: any) => typeof t !== "string")) {
    errors.push(`Case #${index} tags must be array of strings`);
  }
  if (!tc.description || typeof tc.description !== "string") {
    errors.push(`Case #${index} missing string 'description'`);
  }
  return errors;
}

// 3. Check all case files in spec/cases/
const caseFiles = fs.readdirSync(casesDir).filter((f) => f.endsWith(".json"));
const allIds = new Set<string>();
let totalCases = 0;

for (const file of caseFiles) {
  const filePath = path.join(casesDir, file);
  const raw = fs.readFileSync(filePath, "utf8");
  let list: any[];
  try {
    list = JSON.parse(raw);
    const formatted = JSON.stringify(list, null, 2) + "\n";
    if (raw !== formatted) {
      console.warn(`Warning: ${file} formatting differs from 2 spaces`);
    }
  } catch (err) {
    console.error(`✗ Error parsing ${file}:`, err);
    process.exit(1);
  }

  if (!Array.isArray(list)) {
    console.error(`✗ ${file} must be an array of test cases`);
    process.exit(1);
  }

  list.forEach((tc, idx) => {
    totalCases++;
    const errs = validateTestCase(tc, file, idx);
    if (errs.length > 0) {
      console.error(`Validation errors in ${file}:`, errs);
      process.exit(1);
    }
    if (allIds.has(tc.id)) {
      console.error(`✗ Duplicate test case ID found: '${tc.id}' in ${file}`);
      process.exit(1);
    }
    allIds.add(tc.id);
  });

  console.log(`✓ spec/cases/${file} (${list.length} cases) valid`);
}

console.log(`\nAll ${totalCases} test cases in ${caseFiles.length} files passed schema validation with unique IDs!`);
