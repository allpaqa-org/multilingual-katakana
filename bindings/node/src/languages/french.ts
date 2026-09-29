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
