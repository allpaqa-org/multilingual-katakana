# @allpaqa/multilingual-katakana

> **Bridge Global Streamers to Japanese Anime & Character TTS**  
> Ultra-fast (<0.1ms), zero-dependency multilingual to Katakana phonetic converter designed for Japanese TTS engines (VOICEVOX, COEIROINK, AivisSpeech, VOICEPEAK, OpenJTalk, etc.).

**English** | [🇯🇵 日本語](README.ja.md)

[![npm version](https://img.shields.io/npm/v/@allpaqa/multilingual-katakana.svg)](https://www.npmjs.com/package/@allpaqa/multilingual-katakana)
[![CI](https://github.com/allpaqa-org/multilingual-katakana/actions/workflows/ci.yml/badge.svg)](https://github.com/allpaqa-org/multilingual-katakana/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success.svg)](package.json)
[![Tests: 100%](https://img.shields.io/badge/tests-163%20passed-brightgreen.svg)](spec/cases/)
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
   - **French**: Curated French-only words and phrases, including liaison and silent-final-consonant examples.
   - **Vietnamese**: Tone-mark normalization, common phrase/word mappings, and digraph handling.
   - **Thai**: Common phrase mappings plus conservative open-syllable conversion; ambiguous closed syllables are preserved.
   - **Streaming & Gaming Slang**: Multi-platform streaming (YouTube Live, Twitch, Kick, Discord) and gaming terms (`gg`, `gg wp`, `ez`, `pog`, `poggers`, `kekw`, `afk`, `brb`, `lol`, `w`, `ww`, `草`).
4. **🛡️ Safe Kanji Guard (Japanese Kanji Protection)**:
   - Pure Japanese Kanji commonly found in stream titles and comments (`了解`, `初見歓迎`, `神回`, `配信開始`, `感謝`, `最高`, `優勝`, etc.) are strictly protected and never mistakenly converted into Chinese Pinyin.
5. **🎭 Anime & Character TTS Optimization (Japanglish Prosody)**:
   - Automatically collapses unnatural spaces between consecutive Katakana words (`ハロー ガイズ` ➔ `ハローガイズ`).
   - Normalizes Western punctuation to Japanese equivalents (`!` ➔ `！`, `?` ➔ `？`, `.` ➔ `。`, `,` ➔ `、`).
6. **🔒 Interval-Based Escaping & Text Protection (`options.exclude`)**:
   - Protects user-defined bot dictionary entries (`!remember`), URLs, mentions (`@streamer`), and code tokens (`C++`, `node.js`) from phonetic conversion without placeholder collisions.

---

## 🛡️ Core Principle: Safe Failure (Graceful Degradation)

> **"In a live stream, making an imperfect pronunciation is acceptable; deleting comments, corrupting URLs, or crashing the TTS readout is catastrophic."**

`multilingual-katakana` adheres to a strict **Safe Failure** contract:
- **Unknown Input ➔ Retain Verbatim**: If a word or script cannot be converted (e.g. unsupported scripts like Arabic, Hindi, Georgian, or unknown symbols/emojis), it is **never discarded**. The original text is preserved for the TTS engine or fallback reader.
- **Japanese Sacred Guard**: Japanese Hiragana, Katakana, and common stream Kanji (`初見歓迎`, `神回`, `了解`) are never corrupted into foreign readings.
- **Strict Option Scoping**: Disabling a language (e.g. `enableEnglish: false`) guarantees that unknown words are not erroneously processed by fallback phonics.

---

## 🎯 Goals vs Non-Goals

### Goals
- Enable Japanese character TTS engines to read foreign stream comments naturally and adorably.
- Minimize skipped or muted comments during live streams.
- Drastically reduce audio stutter and latency with **zero-runtime dependencies** and **~2-9 µs** execution speed.
- Provide acoustic approximations allowing streamers and viewers to intuitively grasp the intent and hype of foreign viewers.

### Non-Goals
- ❌ **Native Pronunciation Reproduction**: We aim for natural Japanglish/anime character speech, not academic phonetic accuracy.
- ❌ **Machine Translation**: We transliterate characters to Katakana sounds; we do not translate meaning.
- ❌ **Giant Dictionary Inflation**: We intentionally do not bundle 100,000-word lexicons that bloat memory and cold-start time.
- ❌ **Full Multilingual Morphological Analysis**: Lightweight rule-based heuristics over heavyweight NLP parsers.

---

## 🤖 Why Not Machine Learning / LLMs?

> *"Could an AI model do this with higher academic accuracy?"*  
> **Yes, but it is fundamentally overkill and mismatched for the problem.**

| Feature | LLMs / Machine Learning | `multilingual-katakana` |
|---|---|---|
| **Latency** | 200 ms - 2,000 ms (unacceptable for live stream TTS) | **0.002 ms - 0.009 ms (2-9 µs)** |
| **Dependencies** | PyTorch, ONNX, external APIs, GPU drivers | **Zero runtime dependencies** (pure code) |
| **Footprint** | Hundreds of MBs to several GBs | **~86 KB bundle (Node) / self-contained (Rust)** |
| **Cost & Offline** | API tokens, cloud connectivity required | **100% free, offline, local-first** |
| **Determinism** | Hallucinations, unpredictable phoneme output | **100% deterministic, spec-tested** |

---

## ⚡ Scenario-Based Benchmarks (Rust Core)

Benchmarked on Apple Silicon (Release Profile, warm cache, `multilingual-katakana-core`):

| Scenario | Input Size | Latency (µs) | Latency (ms) | Throughput (phrases/sec) |
|---|---|---|---|---|
| **Short Chat** (`gg wp bro`) | ~10 chars | **1.90 µs** | 0.0019 ms | **526,000 ops/s** |
| **Typical Stream Comment** (`初見です！Hello streamer! 今日も配信楽しみにしてました！`) | ~40 chars | **2.64 µs** | 0.0026 ms | **378,000 ops/s** |
| **Mixed Multilingual** (`Hello! 你好! 안녕하세요! muchas gracias bro pog! Привет!`) | ~65 chars | **9.43 µs** | 0.0094 ms | **106,000 ops/s** |
| **URL & Mention Protected** (`check https://twitch.tv/example @streamer nice play gg!`) | ~55 chars | **2.93 µs** | 0.0029 ms | **340,000 ops/s** |
| **Extreme Long Text** (Repeated stream chat copypasta) | ~810 chars | **65.97 µs** | 0.0660 ms | **15,000 ops/s** |

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

### 🪶 Lightweight, Pure-TypeScript by Default

The currently published `0.4.0` package on npm is still **pure TypeScript only**. In this repository, the upcoming per-platform native package scaffolding and `optionalDependencies` wiring now exist for a future v0.4.x release, but those platform packages are not published yet. Runtime dependencies remain zero, and the default install footprint is still lightweight (~124 KB unpacked, ~26 KB gzipped) until that follow-up release ships.

When those native packages are published, the Rust NAPI-RS backend (auto-selected for extra speed) will remain an **optional accelerator** via small per-platform `optionalDependencies` (~1–3 MB each). It will never be required:

- **Prefer the lightweight, pure-JS install?** Skip the native binary entirely with `npm install @allpaqa/multilingual-katakana --omit=optional` (or `pnpm add --no-optional`, `yarn add --ignore-optional`). Output is identical — you just get the existing pure-TypeScript pipeline.
- **Installed the native binary but still want to force pure JS at runtime** (e.g. for byte-identical behavior across environments)? Set `MULTILINGUAL_KATAKANA_BACKEND=js` before your process starts.

See [`docs/V0.4.0_BINDINGS_SCOPE.md`](docs/V0.4.0_BINDINGS_SCOPE.md) for the full native-backend rollout plan.

### C# / .NET (coming soon)

```bash
dotnet add package Allpaqa.MultilingualKatakana
```

The .NET binding (`netstandard2.0` / `net8.0`, same Rust core) is **not yet published to NuGet** — it is planned for v0.6.0. Until then, build it from source; see [`bindings/dotnet/README.md`](bindings/dotnet/README.md).

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

// Vietnamese and Thai
console.log(toKatakana("xin chào! สวัสดีครับ"));
// => "シンチャオ！サワッディークラップ"

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

Want a runnable, zero-build starting point? See
[**multilingual-katakana-samples**](https://github.com/allpaqa-org/multilingual-katakana-samples) —
copy-pasteable scripts that just `npm install` and run.

**Used in production by**
[**twitch_text_to_speech_bot**](https://github.com/allpaqa-jgk/twitch_text_to_speech_bot) —
a real-time Twitch chat-to-speech bot supporting COEIROINK, VOICEVOX, Kokoro, Piper, and OS built-in voices.

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
| `enableFrench` | `boolean` | `true` | Enable curated French word and phrase mappings; unknown words use the regular fallback. |
| `enableVietnamese` | `boolean` | `true` | Enable Vietnamese phrase/word mappings and tone-mark handling. |
| `enableThai` | `boolean` | `true` | Enable Thai phrase mappings and conservative syllable conversion. Ambiguous closed syllables remain unchanged. |
| `enableSlang` | `boolean` | `true` | Enable gaming/streaming slang conversion (`gg`, `ez`, `w`, `pog`, etc.). |
| `normalizeProsody` | `boolean` | `true` | Enable Katakana space removal and Japanese punctuation normalization. |

---

## 🏛️ Architecture & Quality Gates

This repository is built following **Spec-Driven Development** with language-neutral conformance suites:

- **Cross-Language Test Specifications (`spec/cases/*.json`)**:
  163 canonical test cases across 13 suites, including French, Spanish, Vietnamese, and Thai, are shared by the TypeScript and Rust implementations.
- **Strict Quality Gates**:
  - ✅ **100% Test Pass Rate**: All 163 shared specification cases pass in TypeScript and Rust.
  - ✅ **Blazing Fast Rust Core**: ~3.3 µs per phrase (~300,000 phrases/sec) in `crates/multilingual-katakana-core`.
  - ✅ **Biome Linter & Formatter**: 0 errors, 0 warnings.
  - ✅ **Complexity Guard**: Every function enforces **Cyclomatic Complexity <= 15** and Cognitive Complexity <= 15.

```bash
# TypeScript verification
bun run test
bun run check
bun run check:complexity
bun run validate:spec

# Rust Core verification
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

See [Release Process Documentation](docs/RELEASE_PROCESS.md) for automated CI/CD and publishing protocols.

---

## 🗺️ Roadmap

- [x] **v0.1.0**: Zero-dependency TypeScript implementation (Dual ESM/CJS, PUA interval escaping, 10+ languages/slang).
- [x] **v0.2.0**: Core Rust engine (`crates/multilingual-katakana-core`) as Single Source of Truth (<0.005ms latency, self-contained).
- [x] **v0.3.0**: Curated French and expanded Spanish dictionaries plus Vietnamese and Thai support. **Breaking change:** adding public language flags to Rust `KatakanaOptions` requires downstream struct-literal callers to add the new fields or use `..Default::default()`.
- [x] **v0.4.0**: Node.js native backend architecture via NAPI-RS as a drop-in accelerator behind the unchanged `toKatakana` / `KatakanaConverter` API, with automatic pure-TypeScript fallback and `dependencies` staying `{}`. Verified for the Linux CI runner and local `darwin-arm64` dev builds; publishing prebuilt platform packages via `optionalDependencies` for zero-build end-user installs follows in v0.4.x. See the [v0.4.0 bindings scope](docs/V0.4.0_BINDINGS_SCOPE.md).
- [ ] **v0.5.0**: Python bindings (PyO3 + maturin, abi3 wheels on PyPI).
- [ ] **v0.6.0**: C# / .NET bindings (C ABI + NuGet with RID-specific native assets).
- [ ] **Later (on demand)**: Standalone WebAssembly package (`@allpaqa/multilingual-katakana-wasm`) for browsers and edge runtimes.

---

## 📄 License

[MIT License](LICENSE) © 2026 allpaqa
