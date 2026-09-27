import spanishDict from "../dicts/spanish.json";

export function spanishPreprocess(text: string): string {
  return text
    .replace(/¡/g, "")
    .replace(/¿/g, "")
    .replace(/ña/gi, "ニャ")
    .replace(/ñe/gi, "ニェ")
    .replace(/ñi/gi, "ニ")
    .replace(/ño/gi, "ニョ")
    .replace(/ñu/gi, "ニュ")
    .replace(/ñ/gi, "ニャ")
    .replace(/ll/gi, "リャ")
    .replace(/rr/gi, "ル")
    .replace(/ü/gi, "u")
    .replace(/ä/gi, "e")
    .replace(/ö/gi, "o")
    .replace(/ß/gi, "ss")
    .replace(/à|â/gi, "a")
    .replace(/è|ê|ë/gi, "e")
    .replace(/î|ï/gi, "i")
    .replace(/ô/gi, "o")
    .replace(/û|ù/gi, "u")
    .replace(/ç/gi, "s")
    .replace(/á/gi, "a")
    .replace(/é/gi, "e")
    .replace(/í/gi, "i")
    .replace(/ó/gi, "o")
    .replace(/ú/gi, "u");
}

export function replaceSpanishPhrases(text: string): string {
  let result = text;
  const lowerResult = result.toLowerCase();
  for (const [phrase, katakana] of Object.entries(spanishDict.phrases)) {
    if (phrase.includes(" ") && lowerResult.includes(phrase)) {
      const regex = new RegExp(`\\b${phrase}\\b`, "gi");
      result = result.replace(regex, katakana);
    }
  }
  return result;
}

export function getSpanishWord(word: string): string | undefined {
  return (spanishDict.phrases as Record<string, string>)[word.toLowerCase()];
}
