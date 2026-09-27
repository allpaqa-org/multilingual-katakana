import slangDict from "../dicts/slang.json";

export function replaceSlangPhrases(text: string): string {
  let result = text;
  const lowerResult = result.toLowerCase();
  for (const [phrase, katakana] of Object.entries(slangDict.phrases)) {
    if (lowerResult.includes(phrase)) {
      const regex = new RegExp(`\\b${phrase}\\b`, "gi");
      result = result.replace(regex, katakana);
    }
  }
  return result;
}

export function getSlangWord(word: string): string | undefined {
  return (slangDict.slang as Record<string, string>)[word.toLowerCase()];
}
