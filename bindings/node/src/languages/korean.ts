import koreanDict from "../dicts/korean.json";

export function isKorean(text: string): boolean {
  return /[\uAC00-\uD7AF\u1100-\u11FF]/.test(text);
}

function decomposeHangulSyllable(code: number): { l: number; v: number; t: number } | null {
  if (code < 0xac00 || code > 0xd7a3) {
    return null;
  }
  const idx = code - 0xac00;
  const l = Math.floor(idx / 588);
  const v = Math.floor((idx % 588) / 28);
  const t = idx % 28;
  return { l, v, t };
}

export function convertKorean(text: string): string {
  if (!isKorean(text)) return text;

  let processed = text;
  // 1. Common frequent phrases priority override
  for (const [phrase, katakana] of Object.entries(koreanDict.phrases)) {
    if (processed.includes(phrase)) {
      processed = processed.split(phrase).join(katakana);
    }
  }

  // 2. Mathematical phonetic decomposition for remaining Hangul characters
  let result = "";
  const len = processed.length;

  for (let i = 0; i < len; i++) {
    const code = processed.charCodeAt(i);
    const syllable = decomposeHangulSyllable(code);

    if (!syllable) {
      result += processed[i];
      continue;
    }

    const { l, v, t } = syllable;
    const base = koreanDict.chosung_jungsung_map[l]?.[v] || "";

    let batchimSound = koreanDict.jongsung_map[t] || "";

    // Check liaison (連音化) with next character if it has silent initial ㅇ (l === 11)
    if (t > 0 && i + 1 < len) {
      const nextCode = processed.charCodeAt(i + 1);
      const nextSyllable = decomposeHangulSyllable(nextCode);

      if (nextSyllable && nextSyllable.l === 11) {
        if (t === 20) {
          batchimSound = "ッ";
        }
      }
    }

    result += base + batchimSound;
  }

  // 3. Remove unnecessary spaces before punctuation
  result = result.replace(/\s+([、。！？!?,.])/g, "$1");
  return result.replace(/[\s\u3000]+/g, " ").trim();
}
