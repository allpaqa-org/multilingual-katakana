import chineseData from "../dicts/chinese_data.json";
import hanziClass from "../dicts/hanzi_class.json";

const CHINESE_MARKER_REGEX = new RegExp(chineseData.marker_pattern);
const HANZI_TO_KATAKANA = chineseData.hanzi_to_katakana as Record<string, string>;
const TAIWAN_PHRASES = Object.entries(chineseData.taiwan_phrases);

// Character classes (generated from Unihan + hand-curated; see
// scripts/generate_hanzi_class.ts and dicts/hanzi_class.json).
// Strong Chinese evidence: Simplified-only characters + non-Joyo/Jinmeiyo markers.
const ZH_EVIDENCE = new Set(hanziClass.simplified_only.join("") + hanziClass.zh_marker_evidence);
const JAPANESE_ONLY = new Set(hanziClass.japanese_only.join(""));
const JAPANESE_GUARD_WORDS: string[] = hanziClass.japanese_guard_words;
const ZH_CONTEXT_SLANG = Object.entries(hanziClass.zh_context_slang as Record<string, string>);

const HANZI_RUN_REGEX = /[一-鿿]+/g;
// Kana plus the Japanese-only iteration/closing marks 々 and 〆.
const JAPANESE_SCRIPT_REGEX = /[぀-ゟ゠-ヿ々〆]/;
// Gap characters that end a sentence/clause: runs separated by these are not linked.
const SENTENCE_BREAK_REGEX = /[、。！？!?.\n\r]/;
// Explicit horizontal whitespace class shared with the Rust core (no line breaks,
// no reliance on `\s` / `trim()` semantics, which differ between JS and Rust).
const HWS = "\\t\\u000B\\u000C \\u00A0\\u1680\\u2000-\\u200A\\u202F\\u205F\\u3000";
const HWS_REGEX = new RegExp(`[${HWS}]`);
const HWS_SPLIT_REGEX = new RegExp(`([${HWS}]+)`);
const PUNCT = "、。！？!?,.";

/** Classification of one maximal CJK ideograph run. */
type RunClass = "zh" | "ja" | "mixed" | "undecided";

interface HanziRun {
  start: number;
  end: number;
  text: string;
  cls: RunClass;
  /** True when the run itself carries Simplified-only / marker / phrase evidence. */
  evidence: boolean;
  /** Length of a Japanese guard word kept at the start / end of a `zh` run. */
  keepHead: number;
  keepTail: number;
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

/** Longest guard word at the start (or end) of `run`, 0 when none. */
function guardEdge(run: string, atEnd: boolean): number {
  let best = 0;
  for (const word of JAPANESE_GUARD_WORDS) {
    const hit = atEnd ? run.endsWith(word) : run.startsWith(word);
    if (hit && word.length > best) best = word.length;
  }
  return best;
}

/**
 * Evidence-based class of a run, from its own characters only.
 * Japanese-only characters always win (with Chinese evidence too -> mixed).
 * Strong Chinese evidence beats guard words; a guard word at the very start
 * or end of such a run is split off and kept as Japanese.
 */
function newRun(start: number, text: string, relaxGuards: boolean): HanziRun {
  const run: HanziRun = {
    start,
    end: start + text.length,
    text,
    cls: "undecided",
    evidence: false,
    keepHead: 0,
    keepTail: 0,
  };
  const zh = hasChar(text, ZH_EVIDENCE) || TAIWAN_PHRASES.some(([p]) => text.includes(p));
  const japaneseOnly = hasChar(text, JAPANESE_ONLY);
  run.evidence = zh;
  if (japaneseOnly) {
    run.cls = zh ? "mixed" : "ja";
  } else if (zh) {
    run.cls = "zh";
    run.keepHead = guardEdge(text, false);
    run.keepTail = run.keepHead < text.length ? guardEdge(text.slice(run.keepHead), true) : 0;
  } else if (JAPANESE_GUARD_WORDS.some((w) => text.includes(w))) {
    // In a kana-free comment that reads as Chinese, a run that itself shows
    // Chinese markers / common words is not forced Japanese by guard words.
    run.cls = relaxGuards && isChinese(text) ? "undecided" : "ja";
  }
  return run;
}

function findRuns(text: string, relaxGuards: boolean): HanziRun[] {
  const runs: HanziRun[] = [];
  for (const m of text.matchAll(HANZI_RUN_REGEX)) {
    runs.push(newRun(m.index ?? 0, m[0], relaxGuards));
  }
  return runs;
}

/** Nearest non-horizontal-whitespace character before (or after) `index`. */
function nearestChar(text: string, index: number, step: -1 | 1): string {
  let i = index;
  while (i >= 0 && i < text.length && HWS_REGEX.test(text[i])) i += step;
  return text[i] ?? "";
}

/** Rules (a) kana adjacency and (b) `、` hint for a SHARED-only run. */
function classifyByContext(text: string, run: HanziRun): RunClass {
  const prev = text[run.start - 1] ?? "";
  const next = text[run.end] ?? "";
  if (JAPANESE_SCRIPT_REGEX.test(prev) || JAPANESE_SCRIPT_REGEX.test(next)) return "ja";
  if (nearestChar(text, run.start - 1, -1) === "、" || nearestChar(text, run.end, 1) === "、") {
    return "ja";
  }
  return "undecided";
}

/** Two consecutive runs are linked when no kana / sentence break lies between them. */
function isLinked(text: string, left: HanziRun, right: HanziRun): boolean {
  const gap = text.slice(left.end, right.start);
  return !JAPANESE_SCRIPT_REGEX.test(gap) && !SENTENCE_BREAK_REGEX.test(gap);
}

/** Class a run shows to its neighbour on one side (a kept guard edge is neutral). */
function edgeClass(run: HanziRun, cls: RunClass, rightSide: boolean): RunClass {
  if (cls === "zh" && (rightSide ? run.keepTail : run.keepHead) > 0) return "undecided";
  return cls;
}

/** A linked neighbour finally classified `zh` continues the Chinese sentence. */
function continuesSentence(text: string, left: HanziRun, right: HanziRun, other: HanziRun) {
  return isLinked(text, left, right) && other.cls === "zh";
}

/**
 * Runs after classification. In kana-free comments, a guard word at the edge
 * of a Chinese-evidence run is converted when the linked neighbour on that
 * side is a final `zh` run (`我是台灣人，感謝你們`); otherwise it is kept
 * Japanese (`了解谢谢` -> `了解シエシエ`). Never applied to comments with kana.
 */
function settleGuardEdges(text: string, runs: HanziRun[]): void {
  for (let i = 0; i < runs.length; i++) {
    const run = runs[i];
    if (run.cls !== "zh") continue;
    if (i > 0 && continuesSentence(text, runs[i - 1], run, runs[i - 1])) run.keepHead = 0;
    if (i + 1 < runs.length && continuesSentence(text, run, runs[i + 1], runs[i + 1])) {
      run.keepTail = 0;
    }
  }
}

function touchesClass(
  text: string,
  runs: HanziRun[],
  snapshot: RunClass[],
  i: number,
  target: RunClass,
): boolean {
  const left = i > 0 && isLinked(text, runs[i - 1], runs[i]);
  const right = i + 1 < runs.length && isLinked(text, runs[i], runs[i + 1]);
  return (
    (left && edgeClass(runs[i - 1], snapshot[i - 1], true) === target) ||
    (right && edgeClass(runs[i + 1], snapshot[i + 1], false) === target)
  );
}

/** Spread `target` through linked undecided runs until a fixpoint (snapshot-based). */
function spreadClass(text: string, runs: HanziRun[], target: RunClass): void {
  let changed = true;
  while (changed) {
    const snapshot = runs.map((run) => run.cls);
    changed = false;
    for (let i = 0; i < runs.length; i++) {
      if (snapshot[i] === "undecided" && touchesClass(text, runs, snapshot, i, target)) {
        runs[i].cls = target;
        changed = true;
      }
    }
  }
}

/**
 * Rule (c): order-independent propagation, Japanese first (Safe Kanji Guard).
 * In comments with kana, Chinese never spreads to shared-only runs.
 */
function propagateLinks(text: string, runs: HanziRun[], commentHasKana: boolean): void {
  spreadClass(text, runs, "ja");
  if (!commentHasKana) spreadClass(text, runs, "zh");
}

/** Comment text without its ja / mixed runs. */
function textWithoutJapaneseRuns(text: string, runs: HanziRun[]): string {
  let rest = "";
  let last = 0;
  for (const run of runs) {
    if (run.cls === "ja" || run.cls === "mixed") {
      rest += text.slice(last, run.start);
      last = run.end;
    }
  }
  return rest + text.slice(last);
}

function besideFullwidthComma(text: string, run: HanziRun): boolean {
  return nearestChar(text, run.start - 1, -1) === "，" || nearestChar(text, run.end, 1) === "，";
}

/**
 * Rule (d): whole-comment fallback, never applied to comments containing kana.
 * `，` only breaks the tie when the comment has real Chinese evidence.
 */
function applyFallback(text: string, runs: HanziRun[], commentHasKana: boolean): void {
  const undecided = runs.filter((run) => run.cls === "undecided");
  if (undecided.length === 0) return;
  const prior = !commentHasKana && isChinese(textWithoutJapaneseRuns(text, runs));
  const hasEvidence = !commentHasKana && runs.some((run) => run.cls === "zh" && run.evidence);
  for (const run of undecided) {
    run.cls = prior || (hasEvidence && besideFullwidthComma(text, run)) ? "zh" : "ja";
  }
}

/** Classify every maximal CJK run of `text` as zh / ja / mixed. */
function classifyRuns(text: string): HanziRun[] {
  const commentHasKana = JAPANESE_SCRIPT_REGEX.test(text);
  const runs = findRuns(text, !commentHasKana && isChinese(text));
  for (const run of runs) {
    if (run.cls === "undecided") run.cls = classifyByContext(text, run);
  }
  propagateLinks(text, runs, commentHasKana);
  applyFallback(text, runs, commentHasKana);
  if (!commentHasKana) settleGuardEdges(text, runs);
  return runs;
}

function convertHanzi(text: string): string {
  let out = "";
  for (const char of replaceTaiwanPhrases(text)) {
    out += HANZI_TO_KATAKANA[char] || char;
  }
  return out;
}

function renderRun(run: HanziRun): string {
  if (run.cls !== "zh") return replaceTaiwanPhrases(run.text);
  const head = run.text.slice(0, run.keepHead);
  const tail = run.text.slice(run.text.length - run.keepTail);
  const body = run.text.slice(run.keepHead, run.text.length - run.keepTail);
  return head + convertHanzi(body) + tail;
}

function isDigit(char: string | undefined): boolean {
  return char !== undefined && /[0-9０-９]/.test(char);
}

/** A slang token is standalone unless glued to letters/digits, `+`, or `[.,:-]digit`. */
function blocksSlang(near: string | undefined, far: string | undefined): boolean {
  if (near === undefined) return false;
  if (/[0-9A-Za-z０-９+]/.test(near)) return true;
  return /[.,:-]/.test(near) && isDigit(far);
}

/** Replace Chinese-context slang tokens such as `886`. */
function replaceContextSlang(text: string): string {
  let result = text;
  for (const [token, katakana] of ZH_CONTEXT_SLANG) {
    let out = "";
    let last = 0;
    let pos = result.indexOf(token);
    while (pos !== -1) {
      const end = pos + token.length;
      const blocked =
        blocksSlang(result[pos - 1], result[pos - 2]) || blocksSlang(result[end], result[end + 1]);
      if (!blocked) {
        out += result.slice(last, pos) + katakana;
        last = end;
      }
      pos = result.indexOf(token, end);
    }
    result = out + result.slice(last);
  }
  return result;
}

/** Marker for a gap edge: a converted Chinese run, another run, or the text boundary. */
type Edge = "zh" | "run" | "boundary";

const isPunct = (c: string) => c.length === 1 && PUNCT.includes(c);

/**
 * Whitespace rule (only next to a converted run): removed between the run
 * and punctuation / the text boundary, collapsed to one space otherwise.
 */
function cleanWhitespace(prev: string | Edge, next: string | Edge): string | null {
  if (prev === "zh" && (isPunct(next) || next === "boundary")) return "";
  if (next === "zh" && (isPunct(prev) || prev === "boundary")) return "";
  return prev === "zh" || next === "zh" ? " " : null;
}

function isPunctOnly(part: string): boolean {
  return part.length > 0 && [...part].every(isPunct);
}

/**
 * Normalize one gap between runs: `，` -> `、`, `886` slang, and horizontal
 * whitespace cleanup only next to a converted run (or punctuation attached to it).
 * Line breaks and spacing between non-CJK text are never touched.
 */
function normalizeGap(gap: string, left: Edge, right: Edge): string {
  const parts = replaceContextSlang(gap.replace(/，/g, "、")).split(HWS_SPLIT_REGEX);
  let out = "";
  for (let i = 0; i < parts.length; i++) {
    if (i % 2 === 0) {
      out += parts[i];
      continue;
    }
    const prev = i === 1 && parts[0] === "" ? left : parts[i - 1].slice(-1);
    const next = i === parts.length - 2 && parts[i + 1] === "" ? right : parts[i + 1].slice(0, 1);
    // Punctuation directly attached to a converted run: `谢谢！ 大家` -> `谢谢！大家`.
    const bridged =
      (i === 1 && left === "zh" && isPunctOnly(parts[0])) ||
      (i === parts.length - 2 && right === "zh" && isPunctOnly(parts[i + 1]));
    out += bridged ? "" : (cleanWhitespace(prev, next) ?? parts[i]);
  }
  return out;
}

function leftEdge(run: HanziRun | undefined): Edge {
  if (!run) return "boundary";
  return run.cls === "zh" && run.keepTail === 0 ? "zh" : "run";
}

function rightEdge(run: HanziRun | undefined): Edge {
  if (!run) return "boundary";
  return run.cls === "zh" && run.keepHead === 0 ? "zh" : "run";
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
  for (let i = 0; i <= runs.length; i++) {
    const run = runs[i];
    const gap = text.slice(last, run ? run.start : text.length);
    result += normalizeGap(gap, leftEdge(runs[i - 1]), rightEdge(run));
    if (!run) break;
    result += renderRun(run);
    last = run.end;
  }
  return result;
}
