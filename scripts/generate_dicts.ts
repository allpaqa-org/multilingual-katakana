import * as fs from "fs";
import * as path from "path";

const refDir =
  "/Users/th/Library/CloudStorage/GoogleDrive-takuyayukat@gmail.com/マイドライブ/配信/Dev/twitch_text_to_speech_bot/src/tts/transformers";
const dictsDir = path.resolve(__dirname, "../dicts");

const katakanaTs = fs.readFileSync(path.join(refDir, "katakana.ts"), "utf8");
const chineseTs = fs.readFileSync(path.join(refDir, "chinese.ts"), "utf8");
const koreanTs = fs.readFileSync(path.join(refDir, "korean.ts"), "utf8");

function extractConst(code: string, name: string): any {
  const regex = new RegExp(`const ${name}[^=]*= ([\\s\\S]*?);\\n\\n`);
  const match = code.match(regex);
  if (!match) throw new Error("Could not find " + name);
  return eval("(" + match[1] + ")");
}

// 1. Slang
const specialSlang = extractConst(katakanaTs, "SPECIAL_SLANG");
const slangPhrases: Record<string, string> = {};
const slangWords: Record<string, string> = {};
for (const [k, v] of Object.entries(specialSlang)) {
  if (k.includes(" ")) {
    slangPhrases[k] = v as string;
  } else {
    slangWords[k] = v as string;
  }
}
const slangOutput = {
  slang: specialSlang,
  phrases: slangPhrases,
  words: slangWords,
};
fs.writeFileSync(
  path.join(dictsDir, "slang.json"),
  JSON.stringify(slangOutput, null, 2) + "\n",
  "utf8"
);
console.log("Wrote dicts/slang.json");

// 2. Spanish
const spanishPhrases = extractConst(katakanaTs, "SPANISH_PHRASES");
const spanishCharacterMappings = {
  ña: "ニャ",
  ñe: "ニェ",
  ñi: "ニ",
  ño: "ニョ",
  ñu: "ニュ",
  ñ: "ニャ",
  ll: "リャ",
  rr: "ル",
};
const spanishAccents = {
  á: "a",
  é: "e",
  í: "i",
  ó: "o",
  ú: "u",
  ü: "u",
  ä: "e",
  ö: "o",
  ß: "ss",
  à: "a",
  â: "a",
  è: "e",
  ê: "e",
  ë: "e",
  î: "i",
  ï: "i",
  ô: "o",
  û: "u",
  ù: "u",
  ç: "s",
};
const spanishOutput = {
  phrases: spanishPhrases,
  character_mappings: spanishCharacterMappings,
  accents: spanishAccents,
  inverted_punctuation: ["¡", "¿"],
};
fs.writeFileSync(
  path.join(dictsDir, "spanish.json"),
  JSON.stringify(spanishOutput, null, 2) + "\n",
  "utf8"
);
console.log("Wrote dicts/spanish.json");

// 3. Cyrillic
const cyrillicPhrases = extractConst(katakanaTs, "CYRILLIC_PHRASES");
const cyrillicMap = extractConst(katakanaTs, "CYRILLIC_MAP");
const cyrillicOutput = {
  phrases: cyrillicPhrases,
  letters: cyrillicMap,
};
fs.writeFileSync(
  path.join(dictsDir, "cyrillic.json"),
  JSON.stringify(cyrillicOutput, null, 2) + "\n",
  "utf8"
);
console.log("Wrote dicts/cyrillic.json");

// 4. Chinese
const taiwanPhrases = extractConst(chineseTs, "TAIWAN_PHRASES");
const chineseCommonWords = extractConst(chineseTs, "CHINESE_COMMON_WORDS");
const pinyinTable = extractConst(chineseTs, "PINYIN_TABLE");
const markerPattern =
  "[这为们过对发会个么谁让说话见还没从听点赞這們麼誰裡點沒很得嗎吧啦喔呢讚]";
const markerCharacters = Array.from(
  new Set("这为们过对发会个么谁让说话见还没从听点赞這們麼誰裡點沒很得嗎吧啦喔呢讚".split(""))
);
const chineseOutput = {
  taiwan_phrases: taiwanPhrases,
  marker_pattern: markerPattern,
  marker_characters: markerCharacters,
  common_words: chineseCommonWords,
  pinyin_table: pinyinTable,
};
fs.writeFileSync(
  path.join(dictsDir, "chinese.json"),
  JSON.stringify(chineseOutput, null, 2) + "\n",
  "utf8"
);
console.log("Wrote dicts/chinese.json");

// 5. Korean
const koreanPhrases = extractConst(koreanTs, "COMMON_KOREAN_PHRASES");
const chosungJungsungMapRaw = extractConst(koreanTs, "CHOSUNG_JUNGSUNG_MAP");
const jongsungMapRaw = extractConst(koreanTs, "JONGSUNG_MAP");

// Format chosung_jungsung_map as array of arrays (0 to 18)
const chosungJungsungList: string[][] = [];
for (let i = 0; i <= 18; i++) {
  chosungJungsungList.push(chosungJungsungMapRaw[i] || []);
}

// Format jongsung_map as array of strings (0 to 27)
const jongsungList: string[] = [];
for (let i = 0; i <= 27; i++) {
  jongsungList.push(jongsungMapRaw[i] ?? "");
}

const koreanOutput = {
  phrases: koreanPhrases,
  chosung_jungsung_map: chosungJungsungList,
  jongsung_map: jongsungList,
};
fs.writeFileSync(
  path.join(dictsDir, "korean.json"),
  JSON.stringify(koreanOutput, null, 2) + "\n",
  "utf8"
);
console.log("Wrote dicts/korean.json");
