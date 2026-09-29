import thaiDict from "../dicts/thai.json";

type ThaiRow = readonly [string, string, string, string, string, string, string, string];

const rows: Record<string, ThaiRow> = {
  ก: ["カ", "キ", "ク", "ク", "ケ", "ケ", "コ", "コ"],
  ข: ["カ", "キ", "ク", "ク", "ケ", "ケ", "コ", "コ"],
  ฃ: ["カ", "キ", "ク", "ク", "ケ", "ケ", "コ", "コ"],
  ค: ["カ", "キ", "ク", "ク", "ケ", "ケ", "コ", "コ"],
  ฅ: ["カ", "キ", "ク", "ク", "ケ", "ケ", "コ", "コ"],
  ฆ: ["カ", "キ", "ク", "ク", "ケ", "ケ", "コ", "コ"],
  ง: ["ガ", "ギ", "グ", "グ", "ゲ", "ゲ", "ゴ", "ゴ"],
  จ: ["ジャ", "ジ", "ジュ", "ジュ", "ジェ", "ジェ", "ジョ", "ジョ"],
  ฉ: ["チャ", "チ", "チュ", "チュ", "チェ", "チェ", "チョ", "チョ"],
  ช: ["チャ", "チ", "チュ", "チュ", "チェ", "チェ", "チョ", "チョ"],
  ฌ: ["チャ", "チ", "チュ", "チュ", "チェ", "チェ", "チョ", "チョ"],
  ซ: ["サ", "シ", "ス", "ス", "セ", "セ", "ソ", "ソ"],
  ศ: ["サ", "シ", "ス", "ス", "セ", "セ", "ソ", "ソ"],
  ษ: ["サ", "シ", "ス", "ス", "セ", "セ", "ソ", "ソ"],
  ส: ["サ", "シ", "ス", "ス", "セ", "セ", "ソ", "ソ"],
  ญ: ["ヤ", "イ", "ユ", "ユ", "イェ", "イェ", "ヨ", "ヨ"],
  ย: ["ヤ", "イ", "ユ", "ユ", "イェ", "イェ", "ヨ", "ヨ"],
  ฎ: ["ダ", "ディ", "ドゥ", "ドゥ", "デ", "デ", "ド", "ド"],
  ด: ["ダ", "ディ", "ドゥ", "ドゥ", "デ", "デ", "ド", "ド"],
  ฏ: ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"],
  ต: ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"],
  ฐ: ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"],
  ฑ: ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"],
  ฒ: ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"],
  ถ: ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"],
  ท: ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"],
  ธ: ["タ", "ティ", "トゥ", "トゥ", "テ", "テ", "ト", "ト"],
  ณ: ["ナ", "ニ", "ヌ", "ヌ", "ネ", "ネ", "ノ", "ノ"],
  น: ["ナ", "ニ", "ヌ", "ヌ", "ネ", "ネ", "ノ", "ノ"],
  บ: ["バ", "ビ", "ブ", "ブ", "ベ", "ベ", "ボ", "ボ"],
  ป: ["パ", "ピ", "プ", "プ", "ペ", "ペ", "ポ", "ポ"],
  ผ: ["パ", "ピ", "プ", "プ", "ペ", "ペ", "ポ", "ポ"],
  พ: ["パ", "ピ", "プ", "プ", "ペ", "ペ", "ポ", "ポ"],
  ภ: ["パ", "ピ", "プ", "プ", "ペ", "ペ", "ポ", "ポ"],
  ฝ: ["ファ", "フィ", "フ", "フ", "フェ", "フェ", "フォ", "フォ"],
  ฟ: ["ファ", "フィ", "フ", "フ", "フェ", "フェ", "フォ", "フォ"],
  ม: ["マ", "ミ", "ム", "ム", "メ", "メ", "モ", "モ"],
  ร: ["ラ", "リ", "ル", "ル", "レ", "レ", "ロ", "ロ"],
  ล: ["ラ", "リ", "ル", "ル", "レ", "レ", "ロ", "ロ"],
  ฬ: ["ラ", "リ", "ル", "ル", "レ", "レ", "ロ", "ロ"],
  ว: ["ワ", "ウィ", "ウ", "ウ", "ウェ", "ウェ", "ウォ", "ウォ"],
  ห: ["ハ", "ヒ", "フ", "フ", "ヘ", "ヘ", "ホ", "ホ"],
  ฮ: ["ハ", "ヒ", "フ", "フ", "ヘ", "ヘ", "ホ", "ホ"],
  อ: ["ア", "イ", "ウ", "ウ", "エ", "エ", "オ", "オ"],
};

const phraseEntries = Object.entries(thaiDict.phrases).sort(([a], [b]) => b.length - a.length);
const leadingVowels = new Set(["เ", "แ", "โ", "ใ", "ไ"]);
const toneMarks = new Set(["่", "้", "๊", "๋"]);
const thaiScript = /^[\u0e00-\u0e7f]$/;
const vowelData: Record<string, [number, boolean]> = {
  า: [0, true],
  "ั": [0, false],
  "ิ": [1, false],
  "ี": [1, true],
  "ึ": [2, false],
  "ื": [2, true],
  "ุ": [3, false],
  "ู": [3, true],
  อ: [7, true],
  ะ: [0, false],
};

export function isThai(text: string): boolean {
  return /[\u0e00-\u0e7f]/.test(text);
}

export function replaceThaiPhrases(text: string): string {
  let result = text;
  for (const [phrase, katakana] of phraseEntries) {
    result = result.split(phrase).join(katakana);
  }
  return result;
}

function isLeadingVowel(char: string): boolean {
  return leadingVowels.has(char);
}

function isThaiChar(char: string): boolean {
  return thaiScript.test(char);
}

function render(row: ThaiRow, column: number, elongate: boolean): string {
  return `${row[column]}${elongate ? "ー" : ""}`;
}

interface ThaiInitial {
  row: ThaiRow;
  leading: string;
  nextIndex: number;
}

interface ThaiVowel {
  column?: number;
  elongate: boolean;
  diphthong: string;
  nextIndex: number;
}

function parseThaiInitial(text: string[], start: number): ThaiInitial | undefined {
  let leading = "";
  let consonantIndex = start;
  if (isLeadingVowel(text[start])) {
    if (start + 1 >= text.length || !rows[text[start + 1]]) return undefined;
    leading = text[start];
    consonantIndex += 1;
  }
  const row = rows[text[consonantIndex]];
  if (!row) return undefined;
  return { row, leading, nextIndex: consonantIndex + 1 };
}

function skipToneMarks(text: string[], start: number): number {
  let index = start;
  while (index < text.length && toneMarks.has(text[index])) index += 1;
  return index;
}

function parseLeadingThaiVowel(text: string[], start: number, leading: string): ThaiVowel {
  let nextIndex = start;
  const column = leading === "เ" ? 4 : leading === "แ" ? 5 : 6;
  if (text[start] === "ะ") {
    nextIndex += 1;
    return { column, elongate: false, diphthong: "", nextIndex };
  }
  if (leading === "เ" && text[start] === "า") {
    nextIndex += 1;
    return { elongate: false, diphthong: "オ", nextIndex };
  }
  return { column, elongate: true, diphthong: "", nextIndex };
}

function parseThaiVowel(text: string[], start: number, leading: string): ThaiVowel {
  if (leading === "ใ" || leading === "ไ") {
    return { elongate: false, diphthong: "イ", nextIndex: start };
  }
  if (leading) return parseLeadingThaiVowel(text, start, leading);

  const parsed = vowelData[text[start]];
  return parsed
    ? { column: parsed[0], elongate: parsed[1], diphthong: "", nextIndex: start + 1 }
    : { elongate: false, diphthong: "", nextIndex: start };
}

function findAmbiguousThaiRunEnd(text: string[], start: number): number | undefined {
  if (start >= text.length || isLeadingVowel(text[start]) || !rows[text[start]]) {
    return undefined;
  }
  let end = start;
  while (end < text.length && isThaiChar(text[end])) end += 1;
  return end;
}

function convertThaiSyllableRun(text: string[]): string {
  let output = "";
  let i = 0;
  while (i < text.length) {
    const start = i;
    const initial = parseThaiInitial(text, start);
    if (!initial) {
      output += text[start];
      i += 1;
      continue;
    }

    const vowelStart = skipToneMarks(text, initial.nextIndex);
    const vowel = parseThaiVowel(text, vowelStart, initial.leading);
    i = skipToneMarks(text, vowel.nextIndex);
    const ambiguousEnd = findAmbiguousThaiRunEnd(text, i);
    if (ambiguousEnd !== undefined) {
      output += text.slice(start, ambiguousEnd).join("");
      i = ambiguousEnd;
      continue;
    }
    output += vowel.diphthong
      ? `${initial.row[0]}${vowel.diphthong}`
      : render(initial.row, vowel.column ?? 0, vowel.elongate);
  }
  return output;
}

export function convertThaiSyllables(text: string): string {
  return convertThaiSyllableRun(Array.from(text));
}

export function convertThai(text: string): string {
  const phrased = replaceThaiPhrases(text);
  if (!isThai(phrased)) return phrased;
  return convertThaiSyllables(phrased);
}
