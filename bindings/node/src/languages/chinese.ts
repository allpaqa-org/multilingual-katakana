import chineseData from "../dicts/chinese_data.json";

const CHINESE_MARKER_REGEX = new RegExp(chineseData.marker_pattern);

export function isChinese(text: string): boolean {
  // 1. Japanese Absolute Guard: If text has any Hiragana or Katakana, 100% Japanese!
  if (/[\u3040-\u309F\u30A0-\u30FF]/.test(text)) {
    return false;
  }

  // 2. Must contain CJK Ideographs (漢字/汉字)
  if (!/[\u4E00-\u9FFF]/.test(text)) {
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
  for (const [phrase, katakana] of Object.entries(chineseData.taiwan_phrases)) {
    if (result.includes(phrase)) {
      result = result.split(phrase).join(katakana);
    }
  }
  return result;
}

export function convertChinese(text: string): string {
  // 1. Taiwan phrases & greetings are replaced first (even in mixed Japanese text)
  let result = replaceTaiwanPhrases(text);

  // 2. If pure Chinese text, convert all remaining Hanzi blocks
  if (isChinese(text)) {
    result = result.replace(/[\u4E00-\u9FFF]+/g, (hanziMatch) => {
      let out = "";
      for (const char of hanziMatch) {
        out += (chineseData.hanzi_to_katakana as Record<string, string>)[char] || char;
      }
      return out;
    });

    // 3. Normalize Chinese punctuation to Japanese punctuation
    result = result
      .replace(/，/g, "、")
      .replace(/。/g, "。")
      .replace(/！/g, "！")
      .replace(/？/g, "？");

    // 4. Remove unnecessary spaces around punctuation
    result = result.replace(/\s+([、。！？!?,.])/g, "$1").replace(/([、。！？!?,.])\s+/g, "$1");

    result = result.replace(/[\s\u3000]+/g, " ").trim();
  }

  return result;
}
