import chineseData from "../dicts/chinese_data.json";
import hanziClass from "../dicts/hanzi_class.json";

const CHINESE_MARKER_REGEX = new RegExp(chineseData.marker_pattern);
const HANZI_TO_KATAKANA = chineseData.hanzi_to_katakana as Record<string, string>;
const TAIWAN_PHRASES = Object.entries(chineseData.taiwan_phrases);

// Character classes (generated from Unihan + hand-curated; see
// scripts/generate_hanzi_class.ts and dicts/hanzi_class.json).
const SIMPLIFIED_ONLY = new Set(hanziClass.simplified_only.join(""));
const JAPANESE_ONLY = new Set(hanziClass.japanese_only.join(""));
const JAPANESE_GUARD_WORDS: string[] = hanziClass.japanese_guard_words;
const ZH_CONTEXT_SLANG = Object.entries(hanziClass.zh_context_slang as Record<string, string>);

const HANZI_RUN_REGEX = /[一-鿿]+/g;
// Kana plus the Japanese-only iteration/closing marks 々 and 〆.
const JAPANESE_SCRIPT_REGEX = /[぀-ゟ゠-ヿ々〆]/;
// Gap characters that end a sentence/clause: runs separated by these are not linked.
const SENTENCE_BREAK_REGEX = /[、。！？!?.\n]/;

/** Classification of one maximal CJK ideograph run. */
type RunClass = "zh" | "ja" | "mixed" | "undecided";

interface HanziRun {
  start: number;
  end: number;
  text: string;
  cls: RunClass;
}

export function isChinese(text: string): boolean {
  // 1. Japanese Absolute Guard: If text has any Hiragana or Katakana, 100% Japanese!
  if (/[぀-ゟ゠-ヿ]/.test(text)) {
    return false;
  }

  // 2. Must contain CJK Ideographs (漢字/汉字)
  if (!/[一-鿿]/.test(text)) {
    return false;
  }

  // 3. Check for distinctive Simplified / Traditional Chinese grammar particles and characters
  if (CHINESE_MARKER_REGEX.test(text)) {
    return true;
  }

  // 4. Check for common Chinese phrases/greetings
  for (const word of chineseData.common_words) {
    if (text.includes(word)) {
      return true;
    }
  }

  return false;
}

export function replaceTaiwanPhrases(text: string): string {
  let result = text;
  for (const [phrase, katakana] of TAIWAN_PHRASES) {
    if (result.includes(phrase)) {
      result = result.split(phrase).join(katakana);
    }
  }
  return result;
}

function hasChar(text: string, set: Set<string>): boolean {
  for (const char of text) {
    if (set.has(char)) return true;
  }
  return false;
}

function hasAnyWord(text: string, words: readonly string[]): boolean {
  return words.some((word) => text.includes(word));
}

/** Evidence-based class of a run, from its own characters only. */
function classifyByEvidence(run: string): RunClass {
  const zh = hasChar(run, SIMPLIFIED_ONLY) || TAIWAN_PHRASES.some(([p]) => run.includes(p));
  const ja = hasChar(run, JAPANESE_ONLY) || hasAnyWord(run, JAPANESE_GUARD_WORDS);
  if (zh && ja) return "mixed";
  if (zh) return "zh";
  return ja ? "ja" : "undecided";
}

function findRuns(text: string): HanziRun[] {
  const runs: HanziRun[] = [];
  for (const m of text.matchAll(HANZI_RUN_REGEX)) {
    const start = m.index ?? 0;
    runs.push({ start, end: start + m[0].length, text: m[0], cls: classifyByEvidence(m[0]) });
  }
  return runs;
}

/** Nearest non-whitespace characters before and after a run. */
function neighbours(text: string, run: HanziRun): [string, string] {
  const before = text.slice(0, run.start).trimEnd();
  const after = text.slice(run.end).trimStart();
  return [before.slice(-1), after.slice(0, 1)];
}

/** Rules (a) kana adjacency and (b) delimiter hints for a SHARED-only run. */
function classifyByContext(text: string, run: HanziRun, commentHasKana: boolean): RunClass {
  const prev = text[run.start - 1] ?? "";
  const next = text[run.end] ?? "";
  if (JAPANESE_SCRIPT_REGEX.test(prev) || JAPANESE_SCRIPT_REGEX.test(next)) return "ja";
  const [before, after] = neighbours(text, run);
  if (before === "、" || after === "、") return "ja";
  if (!commentHasKana && (before === "，" || after === "，")) return "zh";
  return "undecided";
}

/** Two consecutive runs are linked when no kana / sentence break lies between them. */
function isLinked(text: string, left: HanziRun, right: HanziRun): boolean {
  const gap = text.slice(left.end, right.start);
  return !JAPANESE_SCRIPT_REGEX.test(gap) && !SENTENCE_BREAK_REGEX.test(gap);
}

function linkedClass(text: string, runs: HanziRun[], i: number): RunClass {
  const prev = i > 0 && isLinked(text, runs[i - 1], runs[i]) ? runs[i - 1].cls : "undecided";
  const next =
    i + 1 < runs.length && isLinked(text, runs[i], runs[i + 1]) ? runs[i + 1].cls : "undecided";
  // Japanese wins over Chinese (Safe Kanji Guard); mixed runs never propagate.
  if (prev === "ja" || next === "ja") return "ja";
  return prev === "zh" || next === "zh" ? "zh" : "undecided";
}

/** Rule (c): propagate decided classes through linked neighbours until stable. */
function propagateLinks(text: string, runs: HanziRun[]): void {
  let changed = true;
  while (changed) {
    changed = false;
    for (let i = 0; i < runs.length; i++) {
      if (runs[i].cls !== "undecided") continue;
      const cls = linkedClass(text, runs, i);
      if (cls !== "undecided") {
        runs[i].cls = cls;
        changed = true;
      }
    }
  }
}

/** Rule (d): whole-comment prior, never applied to comments containing kana. */
function priorClass(text: string, runs: HanziRun[], commentHasKana: boolean): RunClass {
  if (commentHasKana) return "ja";
  let rest = "";
  let last = 0;
  for (const run of runs) {
    if (run.cls === "ja" || run.cls === "mixed") {
      rest += text.slice(last, run.start);
      last = run.end;
    }
  }
  rest += text.slice(last);
  return isChinese(rest) ? "zh" : "ja";
}

/** Classify every maximal CJK run of `text` as zh / ja / mixed. */
function classifyRuns(text: string): HanziRun[] {
  const runs = findRuns(text);
  const commentHasKana = JAPANESE_SCRIPT_REGEX.test(text);
  for (const run of runs) {
    if (run.cls === "undecided") run.cls = classifyByContext(text, run, commentHasKana);
  }
  propagateLinks(text, runs);
  if (runs.some((run) => run.cls === "undecided")) {
    const prior = priorClass(text, runs, commentHasKana);
    for (const run of runs) {
      if (run.cls === "undecided") run.cls = prior;
    }
  }
  return runs;
}

function convertHanziRun(run: string): string {
  let out = "";
  for (const char of replaceTaiwanPhrases(run)) {
    out += HANZI_TO_KATAKANA[char] || char;
  }
  return out;
}

function isAsciiAlnum(char: string | undefined): boolean {
  return char !== undefined && /[0-9A-Za-z]/.test(char);
}

/** Replace Chinese-context slang tokens such as `886` (bounded by non-alphanumerics). */
function replaceContextSlang(text: string): string {
  let result = text;
  for (const [token, katakana] of ZH_CONTEXT_SLANG) {
    let out = "";
    let last = 0;
    let pos = result.indexOf(token);
    while (pos !== -1) {
      const end = pos + token.length;
      if (!isAsciiAlnum(result[pos - 1]) && !isAsciiAlnum(result[end])) {
        out += result.slice(last, pos) + katakana;
        last = end;
      }
      pos = result.indexOf(token, end);
    }
    result = out + result.slice(last);
  }
  return result;
}

function normalizeChinesePunctuation(text: string): string {
  return text
    .replace(/，/g, "、")
    .replace(/\s+([、。！？!?,.])/g, "$1")
    .replace(/([、。！？!?,.])\s+/g, "$1")
    .replace(/[\s　]+/g, " ")
    .trim();
}

export function convertChinese(text: string): string {
  const runs = classifyRuns(text);
  if (!runs.some((run) => run.cls === "zh")) {
    // No Chinese run: only exact Taiwan phrase matches are replaced (as before).
    return replaceTaiwanPhrases(text);
  }

  // Taiwan phrases are replaced first inside every run; only `zh` runs are
  // then converted character by character. `ja` and `mixed` runs keep their
  // Kanji untouched (Safe Kanji Guard / safe failure).
  let result = "";
  let last = 0;
  for (const run of runs) {
    result += text.slice(last, run.start);
    result += run.cls === "zh" ? convertHanziRun(run.text) : replaceTaiwanPhrases(run.text);
    last = run.end;
  }
  result += text.slice(last);

  return normalizeChinesePunctuation(replaceContextSlang(result));
}
