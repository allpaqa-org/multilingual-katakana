import vietnameseDict from "../dicts/vietnamese.json";

const accentMap = vietnameseDict.accents as Record<string, string>;
const digraphMap: Record<string, Record<string, string>> = {
  nh: { a: "ニャ", e: "ニェ", i: "ニ", o: "ニョ", u: "ニュ" },
  gi: { a: "ジャ", e: "ジェ", o: "ジョ", u: "ジュ" },
  tr: { a: "チャ", e: "チェ", i: "チ", o: "チョ", u: "チュ" },
};
const digraphFallback: Record<string, string> = { nh: "ニ", gi: "ジ", tr: "チ" };

function normalizeVietnameseVowel(char: string): string {
  const normalized = accentMap[char] ?? accentMap[char.toLowerCase()];
  return normalized ? (char === char.toLowerCase() ? normalized : normalized.toUpperCase()) : char;
}

function preprocessDigraph(
  chars: string[],
  index: number,
): { text: string; length: number } | undefined {
  const pair = chars
    .slice(index, index + 2)
    .join("")
    .toLowerCase();
  const digraph = digraphMap[pair];
  if (!digraph) return undefined;

  const vowel = chars[index + 2]
    ? normalizeVietnameseVowel(chars[index + 2]).toLowerCase()
    : undefined;
  const converted = vowel ? digraph[vowel] : undefined;
  return { text: converted ?? digraphFallback[pair], length: converted ? 3 : 2 };
}

export function vietnamesePreprocess(text: string): string {
  const chars = Array.from(text);
  let result = "";
  let i = 0;

  while (i < chars.length) {
    const digraph = preprocessDigraph(chars, i);
    if (digraph) {
      result += digraph.text;
      i += digraph.length;
      continue;
    }

    const char = chars[i];
    result += char === "đ" || char === "Đ" ? "d" : normalizeVietnameseVowel(char);
    i += 1;
  }
  return result;
}

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

const vietnamesePhrasePatterns = Object.entries(vietnameseDict.phrases)
  .filter(([phrase]) => phrase.includes(" "))
  .sort(([a], [b]) => b.length - a.length)
  .map(([phrase, katakana]) => {
    const pattern = phrase.split(" ").map(escapeRegExp).join("\\s+");
    return {
      regex: new RegExp(`(?<![\\p{L}\\p{N}_])${pattern}(?![\\p{L}\\p{N}_])`, "giu"),
      katakana,
    };
  });

export function replaceVietnamesePhrases(text: string): string {
  let result = text;
  for (const { regex, katakana } of vietnamesePhrasePatterns) {
    result = result.replace(regex, katakana);
  }
  return result;
}

export function getVietnameseWord(word: string): string | undefined {
  return (vietnameseDict.phrases as Record<string, string>)[word.toLowerCase()];
}
