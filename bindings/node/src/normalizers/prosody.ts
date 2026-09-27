export function normalizeProsody(text: string): string {
  let result = text;

  // 1. Clean up any remaining inverted punctuation
  result = result.replace(/[¡¿]/g, "");

  // 2. Normalize Western punctuation to Japanese punctuation for natural breath pauses
  result = result
    .replace(/,/g, "、")
    .replace(/\.(?=\s|$)/g, "。")
    .replace(/!/g, "！")
    .replace(/\?/g, "？");

  // 3. Connect consecutive Katakana words by removing foreign word spaces
  // e.g. "ハロー ガイズ" -> "ハローガイズ", "ユー プレイ" -> "ユープレイ"
  result = result.replace(/(?<=[\u30A0-\u30FFー])\s+(?=[\u30A0-\u30FFー])/g, "");

  // 4. Remove spaces before/after Japanese punctuation
  result = result.replace(/\s+([、。！？])/g, "$1").replace(/([、。！？])\s+/g, "$1");

  return result.replace(/[\s\u3000]+/g, " ").trim();
}
