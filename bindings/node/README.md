# @allpaqa/multilingual-katakana

> **Bridge Global Streamers to Japanese Anime & Character TTS**  
> Ultra-fast (<0.1ms), zero-dependency multilingual to Katakana phonetic converter designed for Japanese TTS engines (VOICEVOX, COEIROINK, AivisSpeech, VOICEPEAK, OpenJTalk, etc.).

**English** | [🇯🇵 日本語](README.ja.md)

[![npm version](https://img.shields.io/npm/v/@allpaqa/multilingual-katakana.svg)](https://www.npmjs.com/package/@allpaqa/multilingual-katakana)
[![CI](https://github.com/allpaqa-org/multilingual-katakana/actions/workflows/ci.yml/badge.svg)](https://github.com/allpaqa-org/multilingual-katakana/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success.svg)](package.json)
[![Tests: 100%](https://img.shields.io/badge/tests-124%20passed-brightgreen.svg)](spec/cases/)
[![Complexity: CC<=15](https://img.shields.io/badge/complexity-CC%20%3C%3D%2015-success.svg)](scripts/check_complexity.ts)

---

## 💡 Why multilingual-katakana?

Japanese character-voice TTS engines (such as **VOICEVOX, COEIROINK, AivisSpeech, and VOICEPEAK**) only accept Japanese phonemes (Katakana / Hiragana / Kanji). When global stream comments arrive in English, Chinese, Korean, Russian, Spanish, or internet slang, traditional pipelines either:
- Crash or fail to pronounce non-Japanese characters, or
- Require massive runtime dependencies (e.g. 38MB CMU pronouncing dictionary + 3.5MB pinyin libraries), adding unacceptable latency (>10ms) to live stream readouts.

`@allpaqa/multilingual-katakana` solves this by delivering an **all-in-one, zero-dependency, ultra-fast (<0.1ms)** phonetic conversion pipeline that sounds natural and adorable in Japanese character voices.

---

## 🌟 Key Features

1. **🚀 Zero Runtime Dependencies (`dependencies: {}`)**:
   - Strictly **zero runtime dependencies**. Heavy dictionaries and syllable mappings are pre-compiled and inlined into a lightweight **~86KB** bundle.
2. **⚡ Ultra-Fast Processing (<0.1ms per phrase)**:
   - Instant O(1) dictionary lookups, mathematical Hangul decomposition, and efficient interval tokenization make it **~50x faster** than traditional multi-library setups.
3. **🌐 10+ Languages & Gaming / Streaming Slang**:
   - **English**: Common loanwords, CMU pronunciation rules, and phonics fallback.
   - **Chinese**: Mandarin Hanzi Pinyin syllables + Taiwan stream slang (`886`, `748`, `草泥馬`).
   - **Korean**: Mathematical Hangul decomposition (`choseong * 588 + jungseong * 28 + jongseong`), liaison sound assimilation, and cheering phrases (`화이팅`).
   - **Russian (Cyrillic)**: Phonetic transliteration into Japanese syllabary.
   - **Spanish**: Accented vowels, inverted marks (`¡`, `¿`), and digraphs (`ñ`, `ll`, `rr`).
   - **Streaming & Gaming Slang**: Multi-platform streaming (YouTube Live, Twitch, Kick, Discord) and gaming terms (`gg`, `gg wp`, `ez`, `pog`, `poggers`, `kekw`, `afk`, `brb`, `lol`, `w`, `ww`, `草`).
4. **🛡️ Safe Kanji Guard (Japanese Kanji Protection)**:
   - Pure Japanese Kanji commonly found in stream titles and comments (`了解`, `初見歓迎`, `神回`, `配信開始`, `感謝`, `最高`, `優勝`, etc.) are strictly protected and never mistakenly converted into Chinese Pinyin.
5. **🎭 Anime & Character TTS Optimization (Japanglish Prosody)**:
   - Automatically collapses unnatural spaces between consecutive Katakana words (`ハロー ガイズ` ➔ `ハローガイズ`).
   - Normalizes Western punctuation to Japanese equivalents (`!` ➔ `！`, `?` ➔ `？`, `.` ➔ `。`, `,` ➔ `、`).
6. **🔒 Interval-Based Escaping & Text Protection (`options.exclude`)**:
   - Protects user-defined bot dictionary entries (`!remember`), URLs, mentions (`@streamer`), and code tokens (`C++`, `node.js`) from phonetic conversion without placeholder collisions.

---

## 📦 Installation

```bash
# Bun
bun add @allpaqa/multilingual-katakana

# npm
npm install @allpaqa/multilingual-katakana

# pnpm / yarn
pnpm add @allpaqa/multilingual-katakana
yarn add @allpaqa/multilingual-katakana
```

Full support for Dual ESM (ECMAScript Modules), CommonJS (CJS), and TypeScript type declarations (`.d.ts`).

---

## 🚀 Quick Start

### Basic Conversion (`toKatakana`)

```typescript
import { toKatakana } from "@allpaqa/multilingual-katakana";

// English & Gaming Slang
console.log(toKatakana("hello world! gg wp!"));
// => "ハローワールド！ジージーウェルプレイド！"

// Korean (Hangul decomposition & cheering)
console.log(toKatakana("안녕하세요! 방송 너무 재밌어요 파이팅!"));
// => "アンニョンハセヨ！パンソンノムチェミッソヨパイティン！"

// Spanish (Silent H, inverted marks, accents)
console.log(toKatakana("¡Hola amigo! Muchas gracias señor"));
// => "オラアミゴ！ムチャスグラシアスセニョール"

// Russian (Cyrillic transliteration)
console.log(toKatakana("Привет, как дела? Спасибо!"));
// => "プリヴィエト、カクジェラ？スパシーバ！"

// Chinese with Safe Kanji Guard (Japanese Kanji preserved, Hanzi/slang converted)
console.log(toKatakana("初見歓迎！ 886 谢谢大家"));
// => "初見歓迎！ バイバイ シェシェダージャー"
```

### Text Protection & Escaping (`options.exclude`)

Protect custom words, URLs, handles, or technical terms from being converted:

```typescript
import { toKatakana } from "@allpaqa/multilingual-katakana";

// Exclude custom words (e.g. Bot dictionary / custom pronunciations)
toKatakana("hello bot nice to meet you", {
  exclude: ["bot", "nice"],
});
// => "ハロー bot nice トゥーミートユー"

// Exclude URLs and mentions with Regular Expressions
toKatakana("check https://example.com @streamer_123 gg", {
  exclude: [/https?:\/\/\S+/, /@\w+/],
});
// => "チェック https://example.com @streamer_123 ジージー"

// Exclude Japanese Katakana terms (prevents space collapse around them)
toKatakana("hello ワラ world", {
  exclude: ["ワラ"],
});
// => "ハロー ワラ ワールド"
```

### Reusable Converter Instance (`KatakanaConverter`)

Create a converter instance with preset default options for maximum efficiency in message-handling pipelines:

```typescript
import { KatakanaConverter } from "@allpaqa/multilingual-katakana";

const converter = new KatakanaConverter({
  exclude: ["VOICEVOX", /https?:\/\/\S+/],
  normalizeProsody: true,
  enableSlang: true,
});

const output = converter.convert("hello VOICEVOX fan!");
// => "ハロー VOICEVOX ファン！"
```

---

## ⚙️ Options Reference

| Option | Type | Default | Description |
|---|---|---|---|
| `exclude` | `(string \| RegExp)[]` | `[]` | Specific words or regex patterns protected from Katakana conversion. |
| `enableEnglish` | `boolean` | `true` | Enable English loanword table and phonics fallback. |
| `enableChinese` | `boolean` | `true` | Enable Chinese Mandarin Pinyin and Taiwan streaming slang conversion. |
| `enableKorean` | `boolean` | `true` | Enable mathematical Hangul decomposition and common phrase conversion. |
| `enableCyrillic` | `boolean` | `true` | Enable Russian Cyrillic phonetic transliteration. |
| `enableSpanish` | `boolean` | `true` | Enable Spanish greeting phrases, accents, and digraph conversions. |
| `enableSlang` | `boolean` | `true` | Enable gaming/streaming slang conversion (`gg`, `ez`, `w`, `pog`, etc.). |
| `normalizeProsody` | `boolean` | `true` | Enable Katakana space removal and Japanese punctuation normalization. |

---

## 🏛️ Architecture & Quality Gates

This repository is built following **Spec-Driven Development** with language-neutral conformance suites:

- **Cross-Language Test Specifications (`spec/cases/*.json`)**:
  103 canonical test cases across 9 suites (English, Chinese, Korean, Cyrillic, Spanish, Slang, Kanji Guard, Prosody, Mixed Stream Comments).
- **Strict Quality Gates**:
  - ✅ **100% Test Pass Rate**: 124 tests pass in ~35ms.
  - ✅ **Biome Linter & Formatter**: 0 errors, 0 warnings.
  - ✅ **Complexity Guard**: Every function enforces **Cyclomatic Complexity <= 15** and Cognitive Complexity <= 15.

```bash
# Run tests
bun run test

# Lint & Format
bun run check

# Check code complexity (CC <= 15)
bun run check:complexity

# Validate spec test schemas
bun run validate:spec
```

---

## 🗺️ Roadmap

- [x] **v0.1.0**: Zero-dependency TypeScript implementation (Dual ESM/CJS, PUA interval escaping, 10+ languages/slang).
- [ ] **v0.2.0**: Core Rust engine (`crates/multilingual-katakana-core`) as Single Source of Truth.
- [ ] **v0.3.0**: Native polyglot bindings via NAPI-RS (Node.js native, WebAssembly, Python, and C#).

---

## 📄 License

[MIT License](LICENSE) © 2026 allpaqa
