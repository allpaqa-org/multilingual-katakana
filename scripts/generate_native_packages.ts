import * as fs from "node:fs";
import * as path from "node:path";

/**
 * Single source of truth for the 8 Tier 1 native platform packages
 * (docs/V0.4.0_BINDINGS_SCOPE.md §5.2). Regenerates:
 *   - bindings/node/npm/<triple>/package.json (one per platform)
 *   - the `optionalDependencies` block in bindings/node/package.json
 * from .github/native-platforms.json, so the platform list, npm
 * os/cpu/libc fields, description text, and version pins can never drift
 * out of sync across files (previously this data was hand-duplicated in
 * 8 separate package.json files, a workflow matrix, and a version-sync
 * script).
 *
 * Usage:
 *   bun run scripts/generate_native_packages.ts          # write files
 *   bun run scripts/generate_native_packages.ts --check  # verify only (CI)
 */

const rootDir = path.resolve(__dirname, "..");
const platformsPath = path.join(rootDir, ".github/native-platforms.json");
const mainPackagePath = path.join(rootDir, "bindings/node/package.json");

interface Platform {
  triple: string;
  label: string;
  "npm-os": string[];
  "npm-cpu": string[];
  "npm-libc"?: string[];
}

function readJson(filePath: string): Record<string, unknown> {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

const platforms = readJson(platformsPath) as unknown as Platform[];
const mainPackage = readJson(mainPackagePath);
const packageName = mainPackage.name as string; // "@allpaqa/multilingual-katakana"
const version = mainPackage.version as string;

function platformPackageName(triple: string): string {
  return `${packageName}-${triple}`;
}

function buildPlatformPackageJson(platform: Platform): Record<string, unknown> {
  const { triple, label } = platform;
  const fileName = `multilingual-katakana.${triple}.node`;

  const pkg: Record<string, unknown> = {
    name: platformPackageName(triple),
    version,
    description: `Native NAPI-RS addon for ${label} — internal platform binary for ${packageName}. Not intended for direct use.`,
    main: fileName,
    files: [fileName],
    author: "allpaqa",
    license: "MIT",
    repository: {
      type: "git",
      url: "git+https://github.com/allpaqa-org/multilingual-katakana.git",
      directory: `bindings/node/npm/${triple}`,
    },
    homepage: "https://github.com/allpaqa-org/multilingual-katakana#readme",
    bugs: {
      url: "https://github.com/allpaqa-org/multilingual-katakana/issues",
    },
    os: platform["npm-os"],
    cpu: platform["npm-cpu"],
    publishConfig: {
      access: "public",
    },
  };

  if (platform["npm-libc"]) {
    pkg.libc = platform["npm-libc"];
  }

  return pkg;
}

function serialize(value: unknown): string {
  return `${JSON.stringify(value, null, 2)}\n`;
}

function writeOrCheck(
  filePath: string,
  content: string,
  checkOnly: boolean,
  mismatches: string[],
): void {
  const existing = fs.existsSync(filePath) ? fs.readFileSync(filePath, "utf8") : null;
  if (existing === content) return;

  if (checkOnly) {
    mismatches.push(filePath);
    return;
  }

  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

function main(): void {
  const checkOnly = process.argv.includes("--check");
  const mismatches: string[] = [];

  for (const platform of platforms) {
    const pkg = buildPlatformPackageJson(platform);
    const filePath = path.join(rootDir, "bindings/node/npm", platform.triple, "package.json");
    writeOrCheck(filePath, serialize(pkg), checkOnly, mismatches);
  }

  // Regenerate the optionalDependencies block, preserving every other
  // field (and its position) in the main package.json.
  const updatedMainPackage = { ...mainPackage };
  updatedMainPackage.optionalDependencies = Object.fromEntries(
    platforms.map((p) => [platformPackageName(p.triple), version]),
  );
  writeOrCheck(mainPackagePath, serialize(updatedMainPackage), checkOnly, mismatches);

  if (checkOnly) {
    if (mismatches.length > 0) {
      console.error("Generated native platform packages are out of date. Run:");
      console.error("  bun run scripts/generate_native_packages.ts");
      console.error("and commit the result. Out-of-date files:");
      for (const file of mismatches) {
        console.error(`  - ${path.relative(rootDir, file)}`);
      }
      process.exit(1);
    }
    console.log(`All ${platforms.length} native platform packages are up to date.`);
    return;
  }

  console.log(
    `Generated ${platforms.length} platform package.json files and updated optionalDependencies in bindings/node/package.json.`,
  );
}

main();
