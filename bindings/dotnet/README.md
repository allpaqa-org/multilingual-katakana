# Allpaqa.MultilingualKatakana (.NET)

> **Bridge Global Streamers to Japanese Anime & Character TTS**
> Zero-dependency multilingual to Katakana phonetic converter for Japanese
> TTS engines (VOICEVOX, COEIROINK, AivisSpeech, VOICEPEAK, OpenJTalk,
> etc.), powered by the same Rust core as the Node.js and Python bindings.

**English** | [日本語](https://github.com/allpaqa-org/multilingual-katakana/blob/main/bindings/dotnet/README.ja.md)

> **Not yet published to NuGet.** Until the first release, build the package
> from source (see below).

---

## Why this package?

Japanese character-voice TTS engines only accept Japanese phonemes
(Katakana / Hiragana / Kanji). When global stream comments arrive in
English, Chinese, Korean, Russian, Spanish, French, Vietnamese, Thai, or
internet slang, this package converts them into natural-sounding Katakana
so they can be read aloud by a Japanese TTS voice.

This is the **.NET binding**: a thin P/Invoke wrapper over the C ABI of the
[Rust core](https://github.com/allpaqa-org/multilingual-katakana/tree/main/crates/multilingual-katakana-core)
(`crates/multilingual-katakana-ffi`). It passes the same
[language-neutral spec contract](https://github.com/allpaqa-org/multilingual-katakana/tree/main/spec/cases)
(100% of `spec/cases/*.json`) as every other binding.

- Targets `netstandard2.0` (.NET Framework 4.6.2+, Unity/Mono, .NET Core) and
  `net8.0` (`LibraryImport`, trimming/Native AOT compatible).
- **Zero NuGet dependencies** for both target frameworks.

## Installation

```bash
dotnet add package Allpaqa.MultilingualKatakana
```

### Build from source

```bash
git clone https://github.com/allpaqa-org/multilingual-katakana.git
cd multilingual-katakana
bun run scripts/stage_dotnet_native.ts   # cargo build + stage native/<host rid>/
cd bindings/dotnet
dotnet test -c Release
dotnet pack src/Allpaqa.MultilingualKatakana -c Release -o ./artifacts
```

A locally packed `.nupkg` contains only the native libraries staged under
`bindings/dotnet/native/{rid}/` (by default, just the host RID).

## Quick Start

```csharp
using Allpaqa.MultilingualKatakana;

Console.WriteLine(Katakana.ToKatakana("Hello guys! GG WP"));
// => "ハローガイズ！ジージーウェルプレイド"

Console.WriteLine(Katakana.ToKatakana("你好！谢谢乾爹"));
// => "ニーハオ！シエシエガンディエ"

// Disable a language, or reuse the same options across many calls:
var converter = new KatakanaConverter(new KatakanaOptions { EnableChinese = false });
Console.WriteLine(converter.Convert("你好"));
// => "你好" (passed through unchanged; Safe Failure — never dropped)
```

All members are thread-safe. `KatakanaConverter` snapshots its options at
construction; later changes to the `KatakanaOptions` instance have no effect.

## Options Reference

Every `KatakanaOptions` property is a `bool?`. `null` (the default) keeps the
core default, which is `true` (enabled) for all options.

| Property | Description |
|---|---|
| `EnableCyrillic` | Russian/Cyrillic transliteration |
| `EnableKorean` | Korean Hangul decomposition |
| `EnableChinese` | Mandarin Pinyin + Taiwan stream slang |
| `EnableSpanish` | Spanish accents, `¡`/`¿`, digraphs |
| `EnableFrench` | Curated French words/phrases |
| `EnableVietnamese` | Vietnamese tone marks and phrases |
| `EnableThai` | Thai phrase mappings and open syllables |
| `EnableSlang` | Streaming/gaming slang (`gg`, `pog`, `afk`, ...) |
| `EnableEnglish` | English loanwords/CMU-style phonics |
| `NormalizeProsody` | Collapses Katakana word spacing, normalizes punctuation |

`options.exclude` (user-defined literal/regex text protection, available in
the Node.js binding) is **not yet available** in this .NET binding.

## Errors

- `ArgumentNullException` — `text` is `null`.
- `KatakanaException` — the native library could not be loaded (unsupported
  platform, missing native asset, ABI version mismatch, failed self check) or
  reported an error (`StatusCode`: 1 = null pointer, 2 = invalid UTF-8,
  3 = panic caught in the core). A load failure is detected once, cached, and
  rethrown with the same message on every call (never a
  `TypeInitializationException`).

## Supported Platforms (RIDs)

| OS | RIDs |
|---|---|
| Windows | `win-x64`, `win-x86`, `win-arm64` |
| Linux (glibc) | `linux-x64`, `linux-arm64` |
| Linux (musl / Alpine) | `linux-musl-x64` |
| macOS | `osx-x64`, `osx-arm64` |

On .NET Core / .NET 5+ the native library is resolved automatically from
`runtimes/{rid}/native/`.

### .NET Framework notes

.NET Framework does not resolve `runtimes/{rid}/native/`. The package ships
`build/` and `buildTransitive/` MSBuild targets that copy the Windows native
DLLs into `x86\`, `x64\` and `arm64\` subfolders of your output directory;
at first use the library loads the one that matches the process
architecture. This works for both `AnyCPU` (including "Prefer 32-bit") and
platform-specific builds. If you deploy manually, keep those subfolders next
to `Allpaqa.MultilingualKatakana.dll`.

## License

MIT — see [`LICENSE`](https://github.com/allpaqa-org/multilingual-katakana/blob/main/LICENSE).
