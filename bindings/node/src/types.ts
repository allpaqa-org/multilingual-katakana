export interface KatakanaOptions {
  enableCyrillic?: boolean;
  enableKorean?: boolean;
  enableChinese?: boolean;
  enableSpanish?: boolean;
  enableSlang?: boolean;
  enableEnglish?: boolean;
  normalizeProsody?: boolean;
  exclude?: (string | RegExp)[];
}
