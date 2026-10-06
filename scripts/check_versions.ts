import * as fs from "node:fs";
import * as path from "node:path";

/**
 * Lockstep version check (docs/V0.4.0_BINDINGS_SCOPE.md §7.4,
 * docs/RELEASE_PROCESS.md §3). Every manifest that carries the project
 * version must agree:
 *   - package.json, bindings/node/package.json (+ its
 *     `@allpaqa/multilingual-katakana-*` optionalDependencies pins)
 *   - bindings/node/npm/<triple>/package.json
 *   - crates/<crate>/Cargo.toml ([package] version)
 *   - bindings/python/pyproject.toml ([project] version) and any
 *     `__version__ = "..."` under bindings/python (PEP 440 spelling)
 *   - bindings/dotnet/Directory.Build.props <Version> (+ any <Version> in
 *     bindings/dotnet .csproj files)
 *
 * The reference version is the root package.json, or the release tag when
 * given. PEP 440 manifests are compared against the PEP 440 spelling of the
 * reference (0.4.0-rc.1 -> 0.4.0rc1).
 *
 * Usage:
 *   bun run scripts/check_versions.ts                # manifests agree with each other
 *   bun run scripts/check_versions.ts --tag v1.2.3   # ...and with the tag
 * Without --tag, GITHUB_REF_NAME is used when GITHUB_REF_TYPE is "tag".
 */

const rootDir = path.resolve(__dirname, "..");

type Flavor = "semver" | "pep440";

interface Entry {
  file: string;
  label: string;
  version: string | null;
  flavor: Flavor;
}

const SEMVER_RE = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/;
const SKIP_DIRS = new Set(["node_modules", "target", "build", "dist", "bin", "obj", "__pycache__"]);

function rel(file: string): string {
  return path.relative(rootDir, file).split(path.sep).join("/");
}

function readText(file: string): string | null {
  return fs.existsSync(file) ? fs.readFileSync(file, "utf8") : null;
}

/** Returns the `version = "..."` of a TOML table, without a full TOML parser. */
export function tomlTableVersion(text: string, table: string): string | null {
  let inTable = false;
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trim();
    if (line.startsWith("[")) {
      inTable = line === `[${table}]`;
      continue;
    }
    const match = inTable ? /^version\s*=\s*"([^"]*)"/.exec(line) : null;
    if (match) return match[1];
  }
  return null;
}

/** Maps a SemVer prerelease to its PEP 440 spelling (0.4.0-rc.1 -> 0.4.0rc1). */
export function toPep440(version: string): string {
  const match = /^(\d+\.\d+\.\d+)-(alpha|beta|rc|a|b)\.?(\d+)$/.exec(version);
  if (!match) return version;
  const tag: Record<string, string> = { alpha: "a", a: "a", beta: "b", b: "b", rc: "rc" };
  return `${match[1]}${tag[match[2]]}${match[3]}`;
}

function listDirs(dir: string): string[] {
  if (!fs.existsSync(dir)) return [];
  return fs
    .readdirSync(dir, { withFileTypes: true })
    .filter((d) => d.isDirectory())
    .map((d) => path.join(dir, d.name))
    .sort();
}

function walkFiles(dir: string, ext: string, out: string[] = []): string[] {
  if (!fs.existsSync(dir)) return out;
  for (const d of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, d.name);
    if (d.isDirectory() && !d.name.startsWith(".") && !SKIP_DIRS.has(d.name)) {
      walkFiles(full, ext, out);
    } else if (d.isFile() && d.name.endsWith(ext)) {
      out.push(full);
    }
  }
  return out.sort();
}

function npmEntries(): Entry[] {
  const entries: Entry[] = [];
  const add = (file: string, label: string, version: unknown) =>
    entries.push({
      file: rel(file),
      label,
      version: typeof version === "string" ? version : null,
      flavor: "semver",
    });

  const manifests = [
    path.join(rootDir, "package.json"),
    path.join(rootDir, "bindings/node/package.json"),
    ...listDirs(path.join(rootDir, "bindings/node/npm")).map((d) => path.join(d, "package.json")),
  ];
  for (const file of manifests) {
    const text = readText(file);
    if (text === null) continue;
    const pkg = JSON.parse(text) as {
      version?: string;
      optionalDependencies?: Record<string, string>;
    };
    add(file, "version", pkg.version);
    for (const [name, pin] of Object.entries(pkg.optionalDependencies ?? {})) {
      if (name.startsWith("@allpaqa/multilingual-katakana-")) {
        add(file, `optionalDependencies["${name}"]`, pin);
      }
    }
  }
  return entries;
}

function cargoEntries(): Entry[] {
  return listDirs(path.join(rootDir, "crates"))
    .map((d) => path.join(d, "Cargo.toml"))
    .filter((file) => fs.existsSync(file))
    .map((file) => ({
      file: rel(file),
      label: "[package] version",
      version: tomlTableVersion(fs.readFileSync(file, "utf8"), "package"),
      flavor: "semver" as Flavor,
    }));
}

function pythonEntries(): Entry[] {
  const entries: Entry[] = [];
  const pyproject = path.join(rootDir, "bindings/python/pyproject.toml");
  const text = readText(pyproject);
  if (text !== null) {
    const version = tomlTableVersion(text, "project");
    entries.push({ file: rel(pyproject), label: "[project] version", version, flavor: "pep440" });
  }
  for (const file of walkFiles(path.join(rootDir, "bindings/python"), ".py")) {
    const match = /^__version__\s*=\s*["']([^"']*)["']/m.exec(fs.readFileSync(file, "utf8"));
    if (match) {
      entries.push({ file: rel(file), label: "__version__", version: match[1], flavor: "pep440" });
    }
  }
  return entries;
}

function dotnetEntries(): Entry[] {
  const dotnetDir = path.join(rootDir, "bindings/dotnet");
  const props = path.join(dotnetDir, "Directory.Build.props");
  const files = [props, ...walkFiles(dotnetDir, ".csproj")];
  const entries: Entry[] = [];
  for (const file of files) {
    const text = readText(file);
    if (text === null) continue;
    const versions = [...text.matchAll(/<Version>\s*([^<]*?)\s*<\/Version>/g)].map((m) => m[1]);
    // Directory.Build.props must declare <Version>; .csproj files only if they override it.
    if (file === props && versions.length === 0) versions.push("");
    for (const version of versions) {
      entries.push({
        file: rel(file),
        label: "<Version>",
        version: version || null,
        flavor: "semver",
      });
    }
  }
  return entries;
}

export function collectEntries(): Entry[] {
  return [...npmEntries(), ...cargoEntries(), ...pythonEntries(), ...dotnetEntries()];
}

function parseTagArg(argv: string[]): string | null {
  const index = argv.findIndex((a) => a === "--tag" || a.startsWith("--tag="));
  if (index >= 0) {
    const arg = argv[index];
    return arg.startsWith("--tag=") ? arg.slice("--tag=".length) : (argv[index + 1] ?? "");
  }
  const refName = process.env.GITHUB_REF_NAME;
  return process.env.GITHUB_REF_TYPE === "tag" && refName ? refName : null;
}

function expectedFor(entry: Entry, reference: string): string {
  return entry.flavor === "pep440" ? toPep440(reference) : reference;
}

function report(errors: string[]): void {
  const annotate = process.env.GITHUB_ACTIONS === "true";
  for (const error of errors) console.error(annotate ? `::error::${error}` : `ERROR: ${error}`);
}

function main(): number {
  const tag = parseTagArg(process.argv.slice(2));
  const tagVersion = tag === null ? null : tag.replace(/^v/, "");
  if (tagVersion !== null && !SEMVER_RE.test(tagVersion)) {
    report([`Tag "${tag}" is not a vX.Y.Z[-prerelease] version.`]);
    return 1;
  }

  const entries = collectEntries();
  const root = entries.find((e) => e.file === "package.json" && e.label === "version");
  const reference = tagVersion ?? root?.version ?? null;
  if (reference === null) {
    report(["Could not read the root package.json version."]);
    return 1;
  }

  const source = tagVersion === null ? "root package.json" : `tag ${tag}`;
  const errors = entries
    .filter((e) => e.version !== expectedFor(e, reference))
    .map(
      (e) =>
        `${e.file} ${e.label} is ${e.version === null ? "missing" : `"${e.version}"`}, expected "${expectedFor(e, reference)}" (${source})`,
    );

  if (errors.length > 0) {
    report(errors);
    console.error(`\n${errors.length} of ${entries.length} version locations do not match.`);
    return 1;
  }
  console.log(`OK: all ${entries.length} version locations match ${reference} (${source}).`);
  return 0;
}

if (import.meta.main) {
  process.exit(main());
}
