import cyrillicDict from "../dicts/cyrillic.json";

export function isCyrillic(text: string): boolean {
  return /[\u0400-\u04FF]/.test(text);
}

export function convertCyrillic(text: string): string {
  if (!isCyrillic(text)) return text;

  let lower = text.toLowerCase();
  for (const [phrase, katakana] of Object.entries(cyrillicDict.phrases)) {
    lower = lower.replace(new RegExp(phrase, "g"), katakana);
  }

  let result = "";
  const letters = cyrillicDict.letters as Record<string, string>;
  for (const char of lower) {
    const mapped = letters[char];
    if (mapped !== undefined) {
      result += mapped;
    } else {
      result += char;
    }
  }
  return result;
}
