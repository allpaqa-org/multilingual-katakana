export {
  escapeExcluded,
  KatakanaConverter,
  resolveWord,
  restoreExcluded,
  toKatakana,
} from "./converter";
export {
  convertChinese,
  isChinese,
  replaceTaiwanPhrases,
} from "./languages/chinese";
export { convertCyrillic, isCyrillic } from "./languages/cyrillic";
export { getEnglishWord, phonicsToKatakana } from "./languages/english";
export { getFrenchWord, replaceFrenchPhrases } from "./languages/french";
export { convertKorean, isKorean } from "./languages/korean";
export { getSlangWord, replaceSlangPhrases } from "./languages/slang";
export {
  getSpanishWord,
  replaceSpanishPhrases,
  spanishPreprocess,
} from "./languages/spanish";
export {
  convertThai,
  convertThaiSyllables,
  isThai,
  replaceThaiPhrases,
} from "./languages/thai";
export {
  getVietnameseWord,
  replaceVietnamesePhrases,
  vietnamesePreprocess,
} from "./languages/vietnamese";
export { normalizeProsody } from "./normalizers/prosody";
export type { KatakanaOptions } from "./types";
