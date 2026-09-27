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
export { convertKorean, isKorean } from "./languages/korean";
export { getSlangWord, replaceSlangPhrases } from "./languages/slang";
export {
  getSpanishWord,
  replaceSpanishPhrases,
  spanishPreprocess,
} from "./languages/spanish";
export { normalizeProsody } from "./normalizers/prosody";
export type { KatakanaOptions } from "./types";
