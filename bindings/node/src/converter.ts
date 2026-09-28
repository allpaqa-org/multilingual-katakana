import { convertChinese } from "./languages/chinese";
import { convertCyrillic } from "./languages/cyrillic";
import { getEnglishWord, phonicsToKatakana } from "./languages/english";
import { convertKorean } from "./languages/korean";
import { getSlangWord, replaceSlangPhrases } from "./languages/slang";
import { getSpanishWord, replaceSpanishPhrases, spanishPreprocess } from "./languages/spanish";
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
  const enableSlang = opts.enableSlang ?? true;
  const enableEnglish = opts.enableEnglish ?? true;
  const lower = match.toLowerCase();

  // 6a. Check special slang first (gg, ez, w, etc.)
  if (enableSlang) {
    const slang = getSlangWord(lower);
    if (slang !== undefined) {
      return slang;
    }
  }

  // 6b. Check common Spanish words (hola, amigo, gracias, etc.)
  if (enableSpanish) {
    const spanish = getSpanishWord(lower);
    if (spanish !== undefined) {
      return spanish;
    }
  }

  // 6c. Check English words dictionary (from CMU dict pre-conversion)
  if (enableEnglish) {
    const english = getEnglishWord(lower);
    if (english !== undefined) {
      return english;
    }
  }

  // 6d. Fallback:
  if (enableEnglish) {
    const preprocessed = spanishPreprocess(match);
    return phonicsToKatakana(preprocessed);
  }
  if (enableSpanish) {
    return spanishPreprocess(match);
  }
  return match;
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

    // 5. Spanish multi-word phrases
    if (opts.enableSpanish) {
      result = replaceSpanishPhrases(result);
    }

    // 6. Slang multi-word phrases
    if (opts.enableSlang) {
      result = replaceSlangPhrases(result);
    }

    // 7. Word-level conversion (Slang -> Spanish -> English words -> Phonics fallback)
    if (opts.enableEnglish || opts.enableSlang || opts.enableSpanish) {
      result = result.replace(
        /[A-Za-zñáéíóúüäößàâèêëîïôûùçÑÁÉÍÓÚÜÄÖÀÂÈÊËÎÏÔÛÙÇ]+('[A-Za-z]+)?/g,
        (match) => this.resolveWord(match, opts),
      );
    }

    // 8. Prosody normalization
    if (opts.normalizeProsody) {
      result = normalizeProsody(result);
    }

    // 9. Restore excluded tokens
    return restoreExcluded(result, tokenMap);
  }

  public transform(text: string, options?: KatakanaOptions): string {
    return this.convert(text, options);
  }
}

const defaultConverter = new KatakanaConverter();

export function toKatakana(text: string, options?: KatakanaOptions): string {
  return defaultConverter.convert(text, options);
}
