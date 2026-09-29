import { getNativeBackend } from "#native-backend";
import { convertChinese } from "./languages/chinese";
import { convertCyrillic } from "./languages/cyrillic";
import { getEnglishWord, phonicsToKatakana } from "./languages/english";
import { getFrenchWord, replaceFrenchPhrases } from "./languages/french";
import { convertKorean } from "./languages/korean";
import { getSlangWord, replaceSlangPhrases } from "./languages/slang";
import { getSpanishWord, replaceSpanishPhrases, spanishPreprocess } from "./languages/spanish";
import { convertThai } from "./languages/thai";
import {
  getVietnameseWord,
  replaceVietnamesePhrases,
  vietnamesePreprocess,
} from "./languages/vietnamese";
import { normalizeProsody } from "./normalizers/prosody";
import type { KatakanaOptions } from "./types";

function resolveOptions(
  defaults?: KatakanaOptions,
  options?: KatakanaOptions,
): Required<KatakanaOptions> {
  const opts = { ...defaults, ...options };
  return {
    enableCyrillic: opts.enableCyrillic ?? true,
    enableKorean: opts.enableKorean ?? true,
    enableChinese: opts.enableChinese ?? true,
    enableSpanish: opts.enableSpanish ?? true,
    enableFrench: opts.enableFrench ?? true,
    enableVietnamese: opts.enableVietnamese ?? true,
    enableThai: opts.enableThai ?? true,
    enableSlang: opts.enableSlang ?? true,
    enableEnglish: opts.enableEnglish ?? true,
    normalizeProsody: opts.normalizeProsody ?? true,
    exclude: opts.exclude ?? [],
  };
}

interface MatchInterval {
  start: number;
  end: number;
  text: string;
}

// A high surrogate (U+D800-U+DBFF) not followed by a matching low surrogate,
// or a low surrogate (U+DC00-U+DFFF) not preceded by a matching high
// surrogate. Passing a string containing one of these to the native (Rust)
// backend would silently replace it with U+FFFD when napi converts it to a
// Rust `String` (Rust strings must be valid UTF-8/UTF-16), corrupting the
// original text -- the pure-TypeScript pipeline preserves it byte-for-byte
// instead, so inputs like this must always take the JS path.
const LONE_SURROGATE = /[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?:[^\uD800-\uDBFF]|^)[\uDC00-\uDFFF]/;

function hasLoneSurrogate(text: string): boolean {
  return LONE_SURROGATE.test(text);
}

function collectStringIntervals(text: string, str: string): MatchInterval[] {
  if (str.length === 0) return [];
  const intervals: MatchInterval[] = [];
  let pos = text.indexOf(str);
  while (pos !== -1) {
    intervals.push({ start: pos, end: pos + str.length, text: str });
    pos = text.indexOf(str, pos + str.length);
  }
  return intervals;
}

function collectRegexIntervals(text: string, regexPattern: RegExp): MatchInterval[] {
  const flags = regexPattern.flags.includes("g") ? regexPattern.flags : `${regexPattern.flags}g`;
  const regex = new RegExp(regexPattern.source, flags);
  const intervals: MatchInterval[] = [];
  for (const match of text.matchAll(regex)) {
    if (match.index !== undefined && match[0].length > 0) {
      intervals.push({
        start: match.index,
        end: match.index + match[0].length,
        text: match[0],
      });
    }
  }
  return intervals;
}

function collectIntervals(text: string, exclude: (string | RegExp)[]): MatchInterval[] {
  const intervals: MatchInterval[] = [];
  for (const item of exclude) {
    if (typeof item === "string") {
      for (const iv of collectStringIntervals(text, item)) {
        intervals.push(iv);
      }
    } else if (item instanceof RegExp) {
      for (const iv of collectRegexIntervals(text, item)) {
        intervals.push(iv);
      }
    }
  }
  return intervals;
}

function resolveIntervals(intervals: MatchInterval[]): MatchInterval[] {
  intervals.sort((a, b) => a.start - b.start || b.end - b.start - (a.end - a.start));
  const result: MatchInterval[] = [];
  let lastEnd = 0;
  for (const iv of intervals) {
    if (iv.start >= lastEnd) {
      result.push(iv);
      lastEnd = iv.end;
    }
  }
  return result;
}

export function escapeExcluded(
  text: string,
  exclude?: (string | RegExp)[],
): { text: string; tokenMap: Map<string, string> } {
  if (!exclude || exclude.length === 0) {
    return { text, tokenMap: new Map() };
  }

  const existingPua = new Set<number>();
  for (let i = 0; i < text.length; i++) {
    const code = text.charCodeAt(i);
    if (code >= 0xe000 && code <= 0xf8ff) {
      existingPua.add(code);
    }
  }

  const intervals = resolveIntervals(collectIntervals(text, exclude));
  const tokenMap = new Map<string, string>();
  const parts: string[] = [];
  let lastIndex = 0;
  let nextPuaCode = 0xe000;

  for (const iv of intervals) {
    parts.push(text.slice(lastIndex, iv.start));
    while (existingPua.has(nextPuaCode)) {
      nextPuaCode++;
    }
    if (nextPuaCode > 0xf8ff) {
      throw new Error(
        "Exceeded maximum number of protectable tokens in PUA range (U+E000 - U+F8FF)",
      );
    }
    const token = String.fromCharCode(nextPuaCode);
    existingPua.add(nextPuaCode);
    nextPuaCode++;

    tokenMap.set(token, iv.text);
    parts.push(token);
    lastIndex = iv.end;
  }
  parts.push(text.slice(lastIndex));

  return { text: parts.join(""), tokenMap };
}

export function restoreExcluded(
  text: string,
  tokenMap: Map<string, string> | Record<string, string>,
): string {
  const map = tokenMap instanceof Map ? tokenMap : new Map(Object.entries(tokenMap));
  if (map.size === 0) return text;
  return text.replace(/[\uE000-\uF8FF]/g, (char) => map.get(char) ?? char);
}

export function resolveWord(match: string, opts: KatakanaOptions): string {
  const enableSpanish = opts.enableSpanish ?? true;
  const enableFrench = opts.enableFrench ?? true;
  const enableVietnamese = opts.enableVietnamese ?? true;
  const enableSlang = opts.enableSlang ?? true;
  const enableEnglish = opts.enableEnglish ?? true;
  const lower = match.toLowerCase();

  const dictionaryResult = resolveDictionaryWord(lower, {
    enableSlang,
    enableFrench,
    enableVietnamese,
    enableSpanish,
    enableEnglish,
  });
  if (dictionaryResult !== undefined) {
    return dictionaryResult;
  }

  if (enableEnglish) {
    const vietnameseProcessed = enableVietnamese ? vietnamesePreprocess(match) : match;
    const preprocessed = enableSpanish
      ? spanishPreprocess(vietnameseProcessed)
      : vietnameseProcessed;
    return phonicsToKatakana(preprocessed);
  }
  if (enableVietnamese) {
    return vietnamesePreprocess(match);
  }
  if (enableSpanish) {
    return spanishPreprocess(match);
  }
  return match;
}

function resolveDictionaryWord(
  word: string,
  flags: {
    enableSlang: boolean;
    enableFrench: boolean;
    enableVietnamese: boolean;
    enableSpanish: boolean;
    enableEnglish: boolean;
  },
): string | undefined {
  if (flags.enableSlang) {
    const slang = getSlangWord(word);
    if (slang !== undefined) return slang;
  }
  if (flags.enableFrench) {
    const french = getFrenchWord(word);
    if (french !== undefined) return french;
  }
  if (flags.enableVietnamese) {
    const vietnamese = getVietnameseWord(word);
    if (vietnamese !== undefined) return vietnamese;
  }
  if (flags.enableSpanish) {
    const spanish = getSpanishWord(word);
    if (spanish !== undefined) return spanish;
  }
  return flags.enableEnglish ? getEnglishWord(word) : undefined;
}

export class KatakanaConverter {
  constructor(private defaultOptions?: KatakanaOptions) {}

  public resolveWord(match: string, opts: KatakanaOptions): string {
    return resolveWord(match, opts);
  }

  public convert(text: string, options?: KatakanaOptions): string {
    const opts = resolveOptions(this.defaultOptions, options);

    // 0. Escape excluded patterns (protect user-defined words, URLs, mentions, etc.)
    const { text: escaped, tokenMap } = escapeExcluded(text, opts.exclude);

    // Prefer the native (NAPI-RS) backend when available: it re-implements
    // this entire pipeline in Rust against the same spec/cases contract.
    // Only attempt it when:
    // - `resolveWord` has not been overridden by a subclass (a native call
    //   would silently bypass that override),
    // - `text` is actually a `string` (non-string input must keep throwing
    //   the same `TypeError` the JS pipeline has always thrown, instead of
    //   napi's own "failed to convert JS value" error), and
    // - `text` contains no lone surrogates (napi/Rust would silently
    //   replace them with U+FFFD, corrupting the original text; this
    //   violates the project's Safe Failure principle).
    if (
      this.resolveWord === KatakanaConverter.prototype.resolveWord &&
      typeof text === "string" &&
      !hasLoneSurrogate(escaped)
    ) {
      const native = getNativeBackend();
      if (native) {
        const nativeResult = native.toKatakanaNative(escaped, {
          enableCyrillic: opts.enableCyrillic,
          enableKorean: opts.enableKorean,
          enableChinese: opts.enableChinese,
          enableSpanish: opts.enableSpanish,
          enableFrench: opts.enableFrench,
          enableVietnamese: opts.enableVietnamese,
          enableThai: opts.enableThai,
          enableSlang: opts.enableSlang,
          enableEnglish: opts.enableEnglish,
          normalizeProsody: opts.normalizeProsody,
        });
        return restoreExcluded(nativeResult, tokenMap);
      }
    }

    return restoreExcluded(this.convertWithJsPipeline(escaped, opts), tokenMap);
  }

  private convertWithJsPipeline(escaped: string, opts: Required<KatakanaOptions>): string {
    // 1. Normalize smart curly apostrophes (from mobile / macOS) to standard ASCII apostrophe
    let result = escaped.replace(/[\u2018\u2019]/g, "'");

    // 2. Cyrillic (Russian)
    if (opts.enableCyrillic) {
      result = convertCyrillic(result);
    }

    // 3. Hangul (Korean)
    if (opts.enableKorean) {
      result = convertKorean(result);
    }

    // 4. Chinese (Kanji Guard + Taiwan phrases + Mandarin Hanzi)
    if (opts.enableChinese) {
      result = convertChinese(result);
    }

    // 4. Thai phrase dictionary and conservative syllable fallback
    if (opts.enableThai) {
      result = convertThai(result);
    }

    // 5. French dictionary phrases
    if (opts.enableFrench) {
      result = replaceFrenchPhrases(result);
    }

    // 6. Spanish multi-word phrases
    if (opts.enableSpanish) {
      result = replaceSpanishPhrases(result);
    }

    // 5b. Vietnamese multi-word phrases
    if (opts.enableVietnamese) {
      result = replaceVietnamesePhrases(result);
    }

    // 6. Slang multi-word phrases
    if (opts.enableSlang) {
      result = replaceSlangPhrases(result);
    }

    // 7. Word-level conversion (Slang -> Spanish -> English words -> Phonics fallback)
    if (
      opts.enableEnglish ||
      opts.enableSlang ||
      opts.enableSpanish ||
      opts.enableFrench ||
      opts.enableVietnamese
    ) {
      result = result.replace(
        /[A-Za-zñáéíóúüäößàâèêëîïôûùçœæãõìòăđĩũơư\u1ea0-\u1ef9\u0102\u0103\u0110\u0111\u0128\u0129\u0168\u0169\u01a0\u01a1\u01af\u01b0ÑÁÉÍÓÚÜÄÖÀÂÈÊËÎÏÔÛÙÇŒÆÃÕÌÒĂĐĨŨƠƯ]+('[A-Za-z]+)?/g,
        (match) => this.resolveWord(match, opts),
      );
    }

    // 8. Prosody normalization
    if (opts.normalizeProsody) {
      result = normalizeProsody(result);
    }

    return result;
  }

  public transform(text: string, options?: KatakanaOptions): string {
    return this.convert(text, options);
  }
}

const defaultConverter = new KatakanaConverter();

export function toKatakana(text: string, options?: KatakanaOptions): string {
  return defaultConverter.convert(text, options);
}
