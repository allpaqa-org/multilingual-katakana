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

export function composeHangulJamo(text: string): string {
  let result = "";
  const len = text.length;
  let i = 0;

  while (i < len) {
    const code = text.charCodeAt(i);
    if (code >= 0x1100 && code <= 0x1112 && i + 1 < len) {
      const nextCode = text.charCodeAt(i + 1);
      if (nextCode >= 0x1161 && nextCode <= 0x1175) {
        const l = code - 0x1100;
        const v = nextCode - 0x1161;
        let t = 0;
        let advance = 2;

        if (i + 2 < len) {
          const trailingCode = text.charCodeAt(i + 2);
          if (trailingCode >= 0x11a8 && trailingCode <= 0x11c2) {
            t = trailingCode - 0x11a7;
            advance = 3;
          }
        }

        result += String.fromCharCode(0xac00 + (l * 21 + v) * 28 + t);
        i += advance;
        continue;
      }
    }
    result += text[i];
    i++;
  }
  return result;
}

export function convertKorean(text: string): string {
  if (!isKorean(text)) return text;

  let processed = composeHangulJamo(text);
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
