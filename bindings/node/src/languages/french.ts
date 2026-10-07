import frenchDict from "../dicts/french.json";

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

const frenchPhrasePatterns = Object.entries(frenchDict.phrases)
  .filter(([phrase]) => phrase.includes(" "))
  .sort(([a], [b]) => b.length - a.length)
  .map(([phrase, katakana]) => {
    const pattern = phrase.split(" ").map(escapeRegExp).join("\\s+");
    return {
      regex: new RegExp(`(?<![\\p{L}\\p{N}_])${pattern}(?![\\p{L}\\p{N}_])`, "giu"),
      katakana,
    };
  });

const frenchCueSet = new Set((frenchDict.cues as string[]).map((w) => w.toLowerCase()));

const frenchCueReadings: Record<string, string> = frenchDict.cue_readings as Record<string, string>;

export function replaceFrenchPhrases(text: string): string {
  let result = text;
  for (const { regex, katakana } of frenchPhrasePatterns) {
    result = result.replace(regex, katakana);
  }
  return result;
}

export function getFrenchWord(word: string): string | undefined {
  return (frenchDict.phrases as Record<string, string>)[word.toLowerCase()];
}

export function getFrenchCueWord(word: string): string | undefined {
  return frenchCueReadings[word.toLowerCase()];
}

const URL_OR_MENTION_REGEX = /https?:\/\/\S+|www\.\S+|@\S+/giu;
const VIETNAMESE_VETO_REGEX = /[ơưăđƠƯĂĐ\u1EA0-\u1EF9]/u;
const FRENCH_SPECIFIC_LETTERS_REGEX = /[çœëïîû]/giu;
const FRENCH_ELISION_REGEX =
  /(?:^|[^\p{L}\p{N}_])(?:j|l|c|d|m|n|s|t|qu)['’‘][aeiouyàâéèêëîïôùûüh]|aujourd['’‘]hui/giu;
const LATIN_WORD_REGEX =
  /[A-Za-zñáéíóúüäößàâèêëîïôûùçœæãõìòăđĩũơư\u1ea0-\u1ef9\u0102\u0103\u0110\u0111\u0128\u0129\u0168\u0169\u01a0\u01a1\u01af\u01b0ÑÁÉÍÓÚÜÄÖÀÂÈÊËÎÏÔÛÙÇŒÆÃÕÌÒĂĐĨŨƠƯ]+(?:'[A-Za-z]+)?/g;

function countFrenchLetterCues(text: string): number {
  const matches = text.match(FRENCH_SPECIFIC_LETTERS_REGEX);
  if (!matches) return 0;
  return new Set(matches.map((c) => c.toLowerCase())).size;
}

function countFrenchElisionCues(text: string): number {
  const matches = text.match(FRENCH_ELISION_REGEX);
  if (!matches) return 0;
  return new Set(matches.map((m) => m.toLowerCase().trim())).size;
}

function countFrenchWordCues(text: string): number {
  const words = text.match(LATIN_WORD_REGEX);
  if (!words) return 0;
  const distinct = new Set<string>();
  for (const w of words) {
    const lower = w.toLowerCase();
    if (frenchCueSet.has(lower)) {
      distinct.add(lower);
    }
  }
  return distinct.size;
}

export function detectFrenchMode(text: string): boolean {
  const cleanText = text.replace(URL_OR_MENTION_REGEX, " ");
  if (VIETNAMESE_VETO_REGEX.test(cleanText)) {
    return false;
  }
  const hits =
    countFrenchLetterCues(cleanText) +
    countFrenchElisionCues(cleanText) +
    countFrenchWordCues(cleanText);
  return hits >= 2;
}

function applyFrenchElisions(word: string): string {
  return word
    .replace(/^j['’‘]a/g, "ジャ")
    .replace(/^j['’‘]e/g, "ジェ")
    .replace(/^j['’‘]i/g, "ジ")
    .replace(/^j['’‘]o/g, "ジョ")
    .replace(/^j['’‘]u/g, "ジュ")
    .replace(/^j['’‘]/g, "ジュ")
    .replace(/^l['’‘]a/g, "ラ")
    .replace(/^l['’‘]e/g, "レ")
    .replace(/^l['’‘]i/g, "リ")
    .replace(/^l['’‘]o/g, "ロ")
    .replace(/^l['’‘]u/g, "リュ")
    .replace(/^l['’‘]/g, "ル")
    .replace(/^c['’‘]/g, "セ")
    .replace(/^d['’‘]/g, "ド")
    .replace(/^m['’‘]/g, "ム")
    .replace(/^n['’‘]/g, "ン")
    .replace(/^s['’‘]/g, "ス")
    .replace(/^t['’‘]/g, "ト")
    .replace(/^qu['’‘]/g, "ク");
}

const IR_ENDINGS: Record<string, string> = {
  b: "ビール",
  d: "ディール",
  f: "フィール",
  g: "ジール",
  m: "ミール",
  n: "ニール",
  p: "ピール",
  r: "リール",
  s: "シール",
  t: "ティール",
  v: "ヴィール",
  l: "リール",
};

function applyFrenchEndings(word: string): string {
  let res = word.replace(/([bcdfghjklmnpqrstvwxz])(er|ez)$/g, "$1エ");
  res = res.replace(/([bdfgmnprstvl])ir$/g, (_, c) => IR_ENDINGS[c] ?? `${c}ール`);
  res = res.replace(/(?<=[aeiouyàâéèêëîïôùûü])(es|[stxdzp])$/g, "");
  res = res.replace(/(?<=[aeiouyàâéèêëîïôùûü][bcdfghjklmnpqrstvwxz]+)e$/g, "");
  return res;
}

const CE = "(?=[bcdfghjklpqrstvwxz]|$)";
const NASAL_TABLE: [RegExp, string][] = [
  [new RegExp(`bon${CE}`, "g"), "ボン"],
  [new RegExp(`mon${CE}`, "g"), "モン"],
  [new RegExp(`ton${CE}`, "g"), "トン"],
  [new RegExp(`son${CE}`, "g"), "ソン"],
  [new RegExp(`don${CE}`, "g"), "ドン"],
  [new RegExp(`con${CE}`, "g"), "コン"],
  [new RegExp(`pon${CE}`, "g"), "ポン"],
  [new RegExp(`ron${CE}`, "g"), "ロン"],
  [new RegExp(`lon${CE}`, "g"), "ロン"],
  [new RegExp(`non${CE}`, "g"), "ノン"],
  [new RegExp(`von${CE}`, "g"), "ヴォン"],
  [new RegExp(`fon${CE}`, "g"), "フォン"],
  [new RegExp(`chon${CE}`, "g"), "ション"],
  [new RegExp(`jon${CE}`, "g"), "ジョン"],
  [new RegExp(`(on|om)${CE}`, "g"), "オン"],
  [new RegExp(`b(an|en)${CE}`, "g"), "バン"],
  [new RegExp(`m(an|en)${CE}`, "g"), "マン"],
  [new RegExp(`t(an|en)${CE}`, "g"), "タン"],
  [new RegExp(`s(an|en)${CE}`, "g"), "サン"],
  [new RegExp(`d(an|en)${CE}`, "g"), "ダン"],
  [new RegExp(`p(an|en)${CE}`, "g"), "パン"],
  [new RegExp(`r(an|en)${CE}`, "g"), "ラン"],
  [new RegExp(`l(an|en)${CE}`, "g"), "ラン"],
  [new RegExp(`n(an|en)${CE}`, "g"), "ナン"],
  [new RegExp(`v(an|en)${CE}`, "g"), "ヴァン"],
  [new RegExp(`f(an|en)${CE}`, "g"), "ファン"],
  [new RegExp(`ch(an|en)${CE}`, "g"), "シャン"],
  [new RegExp(`j(an|en)${CE}`, "g"), "ジャン"],
  [new RegExp(`(an|en|am|em)${CE}`, "g"), "アン"],
  [new RegExp(`b(in|ain|ein|un)${CE}`, "g"), "バン"],
  [new RegExp(`m(in|ain|ein|un)${CE}`, "g"), "マン"],
  [new RegExp(`t(in|ain|ein|un)${CE}`, "g"), "タン"],
  [new RegExp(`s(in|ain|ein|un)${CE}`, "g"), "サン"],
  [new RegExp(`d(in|ain|ein|un)${CE}`, "g"), "ダン"],
  [new RegExp(`p(in|ain|ein|un)${CE}`, "g"), "パン"],
  [new RegExp(`r(in|ain|ein|un)${CE}`, "g"), "ラン"],
  [new RegExp(`l(in|ain|ein|un)${CE}`, "g"), "ラン"],
  [new RegExp(`v(in|ain|ein|un)${CE}`, "g"), "ヴァン"],
  [new RegExp(`f(in|ain|ein|un)${CE}`, "g"), "ファン"],
  [new RegExp(`(in|im|ain|ein|un|um)${CE}`, "g"), "アン"],
];

function applyFrenchNasals(word: string): string {
  let res = word;
  for (const [re, rep] of NASAL_TABLE) {
    res = res.replace(re, rep);
  }
  return res;
}

const VOWEL_TABLE: [RegExp, string][] = [
  [/vai/g, "ヴェ"],
  [/fai/g, "フェ"],
  [/bai/g, "ベ"],
  [/dai/g, "デ"],
  [/mai/g, "メ"],
  [/nai/g, "ネ"],
  [/pai/g, "ペ"],
  [/rai/g, "レ"],
  [/sai/g, "セ"],
  [/tai/g, "テ"],
  [/c(ai|ei)/g, "セ"],
  [/g(ai|ei)/g, "ジェ"],
  [/(ai|ei)/g, "エ"],
  [/eau/g, "オ"],
  [/au/g, "オ"],
  [/ou/g, "ウ"],
  [/croi/g, "クロワ"],
  [/troi/g, "トロワ"],
  [/droi/g, "ドロワ"],
  [/broi/g, "ブロワ"],
  [/proi/g, "プロワ"],
  [/moi/g, "モワ"],
  [/toi/g, "トワ"],
  [/soi/g, "ソワ"],
  [/voi/g, "ヴォワ"],
  [/roi/g, "ロワ"],
  [/loi/g, "ロワ"],
  [/joi/g, "ジョワ"],
  [/choi/g, "ショワ"],
  [/boi/g, "ボワ"],
  [/poi/g, "ポワ"],
  [/doi/g, "ドワ"],
  [/oi/g, "ワ"],
  [/jeu/g, "ジュ"],
  [/cheu/g, "シュ"],
  [/veu/g, "ヴ"],
  [/beu/g, "ブ"],
  [/peu/g, "プ"],
  [/meu/g, "ム"],
  [/neu/g, "ヌ"],
  [/(eu|œu|œ)/g, "ウ"],
  [/dor$/g, "ドール"],
  [/tor$/g, "トール"],
  [/por$/g, "ポール"],
  [/cor$/g, "コール"],
  [/mor$/g, "モール"],
  [/[éèêë]/g, "エ"],
  [/[àâ]/g, "ア"],
  [/[îï]/g, "イ"],
  [/drô/g, "ドロー"],
  [/trô/g, "トロー"],
  [/[ôö]/g, "オ"],
  [/[ûüù]/g, "ュ"],
  [/ç/g, "ス"],
];

function applyFrenchVowels(word: string): string {
  let res = word;
  for (const [re, rep] of VOWEL_TABLE) {
    res = res.replace(re, rep);
  }
  return res;
}

const CONSONANT_COMBO_TABLE: [RegExp, string][] = [
  [/ch/g, "シュ"],
  [/gn/g, "ニュ"],
  [/qu/g, "ク"],
  [/ph/g, "フ"],
  [/th/g, "ト"],
  [/ll/g, "ル"],
  [/h/g, ""],
  [/c([eiyエエイ])/g, "セ$1"],
  [/g([eiyエエイ])/g, "ジェ$1"],
  [/cr/g, "クロ"],
  [/dr/g, "ドロ"],
  [/tr/g, "トロ"],
  [/br/g, "ブロ"],
  [/pr/g, "プロ"],
  [/gr/g, "グロ"],
  [/fr/g, "フロ"],
  [/vr/g, "ヴロ"],
  [/cl/g, "クラ"],
  [/bl/g, "ブラ"],
  [/fl/g, "フラ"],
  [/gl/g, "グラ"],
  [/pl/g, "プラ"],
];

function applyFrenchConsonants(word: string): string {
  let res = word;
  for (const [re, rep] of CONSONANT_COMBO_TABLE) {
    res = res.replace(re, rep);
  }
  return res;
}

const CV_TABLE: [RegExp, string][] = [
  [/ja/g, "ジャ"],
  [/ju/g, "ジュ"],
  [/jo/g, "ジョ"],
  [/ji/g, "ジ"],
  [/je/g, "ジェ"],
  [/j/g, "ジュ"],
  [/va/g, "ヴァ"],
  [/vi/g, "ヴィ"],
  [/vu/g, "ヴュ"],
  [/ve/g, "ヴェ"],
  [/vo/g, "ヴォ"],
  [/v/g, "ヴ"],
  [/ka/g, "カ"],
  [/ki/g, "キ"],
  [/ku/g, "ク"],
  [/ke/g, "ケ"],
  [/ko/g, "コ"],
  [/ca/g, "カ"],
  [/cu/g, "キュ"],
  [/co/g, "コ"],
  [/sa/g, "サ"],
  [/si/g, "シ"],
  [/su/g, "ス"],
  [/se/g, "セ"],
  [/so/g, "ソ"],
  [/ta/g, "タ"],
  [/ti/g, "ティ"],
  [/tu/g, "テュ"],
  [/te/g, "テ"],
  [/to/g, "ト"],
  [/na/g, "ナ"],
  [/ni/g, "ニ"],
  [/nu/g, "ニュ"],
  [/ne/g, "ネ"],
  [/no/g, "ノ"],
  [/ma/g, "マ"],
  [/mi/g, "ミ"],
  [/mu/g, "ミュ"],
  [/me/g, "メ"],
  [/mo/g, "モ"],
  [/ra/g, "ラ"],
  [/ri/g, "リ"],
  [/ru/g, "リュ"],
  [/re/g, "レ"],
  [/ro/g, "ロ"],
  [/la/g, "ラ"],
  [/li/g, "リ"],
  [/lu/g, "リュ"],
  [/le/g, "レ"],
  [/lo/g, "ロ"],
  [/ga/g, "ガ"],
  [/gu/g, "ギュ"],
  [/go/g, "ゴ"],
  [/ba/g, "バ"],
  [/bi/g, "ビ"],
  [/bu/g, "ビュ"],
  [/be/g, "ベ"],
  [/bo/g, "ボ"],
  [/pa/g, "パ"],
  [/pi/g, "ピ"],
  [/pu/g, "ピュ"],
  [/pe/g, "ペ"],
  [/po/g, "ポ"],
  [/da/g, "ダ"],
  [/di/g, "ディ"],
  [/du/g, "デュ"],
  [/de/g, "デ"],
  [/do/g, "ド"],
  [/fa/g, "ファ"],
  [/fi/g, "フィ"],
  [/fu/g, "フュ"],
  [/fe/g, "フェ"],
  [/fo/g, "フォ"],
  [/za/g, "ザ"],
  [/zi/g, "ジ"],
  [/zu/g, "ズ"],
  [/ze/g, "ゼ"],
  [/zo/g, "ゾ"],
  [/wa/g, "ワ"],
  [/a/g, "ア"],
  [/i/g, "イ"],
  [/u/g, "ュ"],
  [/e/g, "エ"],
  [/o/g, "オ"],
  [/y/g, "イ"],
];

const STANDALONE_CONSONANTS: [RegExp, string][] = [
  [/b/g, "ブ"],
  [/c/g, "ク"],
  [/d/g, "ド"],
  [/f/g, "フ"],
  [/g/g, "グ"],
  [/k/g, "ク"],
  [/l/g, "ル"],
  [/m/g, "ム"],
  [/n/g, "ン"],
  [/p/g, "プ"],
  [/q/g, "ク"],
  [/r/g, "ル"],
  [/s/g, "ス"],
  [/t/g, "ト"],
  [/v/g, "ヴ"],
  [/x/g, "クス"],
  [/z/g, "ズ"],
];

function applyFrenchSyllables(word: string): string {
  let res = word;
  for (const [re, rep] of CV_TABLE) {
    res = res.replace(re, rep);
  }
  for (const [re, rep] of STANDALONE_CONSONANTS) {
    res = res.replace(re, rep);
  }
  return res;
}

export function frenchPhonicsToKatakana(rawWord: string): string {
  let word = rawWord.toLowerCase();
  word = applyFrenchElisions(word);
  word = applyFrenchEndings(word);
  word = applyFrenchNasals(word);
  word = applyFrenchVowels(word);
  word = applyFrenchConsonants(word);
  word = applyFrenchSyllables(word);
  word = word.replace(/ルエ/g, "レ");
  word = word.replace(/([ァ-ン])ー+/g, "$1ー");
  return word || rawWord;
}
