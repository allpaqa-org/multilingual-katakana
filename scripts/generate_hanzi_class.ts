/**
 * Generates `dicts/hanzi_class.json` (character class data for the Safe Kanji
 * Guard, see issue #36) from the Unicode Han Database (Unihan).
 *
 * Usage:
 *   bun run scripts/generate_hanzi_class.ts [--unihan <dir containing Unihan_*.txt>]
 *
 * Without `--unihan`, Unihan.zip is downloaded from
 * https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip into the OS temp
 * directory and extracted with the system `unzip` command. This happens at
 * generation time only: the generated JSON is committed and bundled, so
 * nothing is downloaded at build time or runtime (zero runtime dependencies).
 *
 * Provenance / license: derived from Unihan (Unicode Character Database),
 * Copyright © Unicode, Inc., distributed under the Unicode License v3
 * (https://www.unicode.org/license.txt).
 *
 * What is generated vs. curated:
 * - `simplified_only` is fully generated (rule below + a reviewed allow list).
 * - `japanese_only`, `japanese_guard_words` and `zh_context_slang` are
 *   hand-curated and are read back from the existing JSON unchanged. The
 *   script only prints the automatically seeded `japanese_only` candidates
 *   (and the diff against the curated list) so a human can review them.
 *
 * The JSON is written to `dicts/` and copied to `bindings/node/src/dicts/`
 * and `crates/multilingual-katakana-core/dicts/`.
 */
import { execFileSync } from "node:child_process";
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";

const UNIHAN_URL = "https://www.unicode.org/Public/UCD/latest/ucd/Unihan.zip";
const rootDir = path.resolve(__dirname, "..");
const outFile = path.join(rootDir, "dicts/hanzi_class.json");
const copyTargets = [
  path.join(rootDir, "bindings/node/src/dicts/hanzi_class.json"),
  path.join(rootDir, "crates/multilingual-katakana-core/dicts/hanzi_class.json"),
];

const CJK_START = 0x4e00;
const CJK_END = 0x9fff;
const CHUNK = 50;

/**
 * Simplified forms that are encoded in JIS X 0208 level 2 (so the generic
 * rule excludes them) but are practically never used in ordinary modern
 * Japanese, while being very frequent in Simplified Chinese. Reviewed by hand.
 * JIS level-2 simplified forms that DO appear in Japanese (洒落, 叱咤, 痒い,
 * 奸, 泪, 刮目, 几帳, 疱瘡, names such as 姜/杰/冲, ...) are deliberately absent.
 */
const SIMPLIFIED_JIS2_ALLOW =
  "个于从价儿册决况听坏夸广并弃弯愿挂无档梦气烟犹盖粮网耻苹范荐迹韵厂凭";

const MUST_BE_SIMPLIFIED_ONLY = "们这说谢欢发东对么个时吗";
const MUST_BE_SHARED = "万台后大家好神回了解初見迎配信感謝最高優勝";
const MUST_BE_JAPANESE_ONLY = "歓対図売読気楽駅実県広沢険験釈択拡恵戦様総弁辺込畑峠辻働枠";

type Fields = Map<number, Map<string, string>>;

function resolveUnihanDir(): string {
  const idx = process.argv.indexOf("--unihan");
  if (idx !== -1 && process.argv[idx + 1]) return path.resolve(process.argv[idx + 1]);
  const dir = path.join(os.tmpdir(), "multilingual-katakana-unihan");
  if (!fs.existsSync(path.join(dir, "Unihan_Variants.txt"))) {
    fs.mkdirSync(dir, { recursive: true });
    const zip = path.join(dir, "Unihan.zip");
    console.log(`Downloading ${UNIHAN_URL} ...`);
    execFileSync("curl", ["-sSfL", "-o", zip, UNIHAN_URL], { stdio: "inherit" });
    execFileSync("unzip", ["-o", "-q", zip, "-d", dir], { stdio: "inherit" });
  }
  return dir;
}

function loadFields(dir: string): Fields {
  const fields: Fields = new Map();
  for (const file of ["Unihan_Variants.txt", "Unihan_IRGSources.txt", "Unihan_OtherMappings.txt"]) {
    for (const line of fs.readFileSync(path.join(dir, file), "utf8").split("\n")) {
      if (!line.startsWith("U+")) continue;
      const [cpStr, key, value] = line.split("\t");
      const cp = Number.parseInt(cpStr.slice(2), 16);
      if (cp < CJK_START || cp > CJK_END) continue;
      if (!fields.has(cp)) fields.set(cp, new Map());
      fields.get(cp)?.set(key, value);
    }
  }
  return fields;
}

function variants(value: string | undefined): number[] {
  if (!value) return [];
  return value.split(" ").map((v) => Number.parseInt(v.split("<")[0].slice(2), 16));
}

function isJoyoOrJinmeiyo(f: Map<string, string>): boolean {
  return f.has("kJoyoKanji") || f.has("kJinmeiyoKanji");
}

/** JIS X 0208 row of the character (0 when it has no J0 source). */
function jisX0208Row(f: Map<string, string>): number {
  const src = f.get("kIRG_JSource") ?? "";
  return src.startsWith("J0-") ? Number.parseInt(src.slice(3, 5), 16) : 0;
}

/**
 * SIMPLIFIED_ONLY rule: the character is a simplified form (it has a
 * kTraditionalVariant other than itself), is neither Joyo nor Jinmeiyo kanji,
 * and is not in JIS X 0208 (level 2 entries only via SIMPLIFIED_JIS2_ALLOW).
 */
function isSimplifiedOnly(cp: number, f: Map<string, string>): boolean {
  if (!variants(f.get("kTraditionalVariant")).some((v) => v !== cp)) return false;
  if (isJoyoOrJinmeiyo(f)) return false;
  const row = jisX0208Row(f);
  if (row === 0) return true;
  return row >= 0x50 && SIMPLIFIED_JIS2_ALLOW.includes(String.fromCodePoint(cp));
}

/**
 * JAPANESE_ONLY seed (for human review only): Joyo/Jinmeiyo kanji that are
 * neither in GB 2312 / GB 12345 (G0/G1) nor in CNS 11643 planes 1-2 (T1/T2,
 * i.e. Big5), i.e. shinjitai and kokuji that standard Chinese text does not use.
 */
function isJapaneseOnlyCandidate(f: Map<string, string>): boolean {
  if (!isJoyoOrJinmeiyo(f)) return false;
  const g = f.get("kIRG_GSource") ?? "";
  const t = f.get("kIRG_TSource") ?? "";
  return !/^G[01]-/.test(g) && !/^T[12]-/.test(t);
}

function chunk(chars: string[]): string[] {
  const out: string[] = [];
  for (let i = 0; i < chars.length; i += CHUNK) out.push(chars.slice(i, i + CHUNK).join(""));
  return out;
}

function sortedChars(s: string): string[] {
  return [...new Set([...s])].sort((a, b) => (a.codePointAt(0) ?? 0) - (b.codePointAt(0) ?? 0));
}

function assertClass(label: string, chars: string, set: Set<string>, expected: boolean) {
  const bad = [...chars].filter((c) => set.has(c) !== expected);
  if (bad.length > 0) throw new Error(`${label}: unexpected classification for ${bad.join("")}`);
}

const fields = loadFields(resolveUnihanDir());
const simplified: string[] = [];
const japaneseCandidates: string[] = [];
for (const [cp, f] of [...fields.entries()].sort((a, b) => a[0] - b[0])) {
  if (isSimplifiedOnly(cp, f)) simplified.push(String.fromCodePoint(cp));
  if (isJapaneseOnlyCandidate(f)) japaneseCandidates.push(String.fromCodePoint(cp));
}

const existing = fs.existsSync(outFile) ? JSON.parse(fs.readFileSync(outFile, "utf8")) : {};
const curatedJapanese = sortedChars((existing.japanese_only ?? []).join(""));

const simplifiedSet = new Set(simplified);
const japaneseSet = new Set(curatedJapanese);
assertClass("SIMPLIFIED_ONLY", MUST_BE_SIMPLIFIED_ONLY, simplifiedSet, true);
assertClass("SIMPLIFIED_ONLY", MUST_BE_SHARED, simplifiedSet, false);
assertClass("JAPANESE_ONLY", MUST_BE_SHARED, japaneseSet, false);
if (curatedJapanese.length > 0) {
  assertClass("JAPANESE_ONLY", MUST_BE_JAPANESE_ONLY, japaneseSet, true);
}
const overlap = curatedJapanese.filter((c) => simplifiedSet.has(c));
if (overlap.length > 0) throw new Error(`Classes overlap: ${overlap.join("")}`);

const candidateSet = new Set(japaneseCandidates);
console.log(`JAPANESE_ONLY seed candidates (${japaneseCandidates.length}):`);
console.log(japaneseCandidates.join(""));
console.log(
  `Candidates NOT in curated list: ${japaneseCandidates.filter((c) => !japaneseSet.has(c)).join("")}`,
);
console.log(
  `Curated entries NOT in candidates: ${curatedJapanese.filter((c) => !candidateSet.has(c)).join("")}`,
);

const output = {
  _comment: [
    "Character class data for the Safe Kanji Guard (issue #36).",
    "simplified_only: GENERATED by scripts/generate_hanzi_class.ts from Unihan",
    "(Copyright Unicode, Inc., Unicode License v3, https://www.unicode.org/license.txt).",
    "japanese_only / japanese_guard_words / zh_context_slang: hand-curated, preserved by the script.",
    "Do not edit simplified_only by hand; re-run the script instead.",
  ],
  simplified_only: chunk(simplified),
  japanese_only: chunk(curatedJapanese),
  japanese_guard_words: existing.japanese_guard_words ?? [],
  zh_context_slang: existing.zh_context_slang ?? {},
};

const json = `${JSON.stringify(output, null, 2)}\n`;
for (const target of [outFile, ...copyTargets]) fs.writeFileSync(target, json, "utf8");
console.log(
  `✓ Wrote dicts/hanzi_class.json (simplified_only=${simplified.length}, japanese_only=${curatedJapanese.length}, guard_words=${output.japanese_guard_words.length}) and copies`,
);
