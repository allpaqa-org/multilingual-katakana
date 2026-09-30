# allpaqa-multilingual-katakana (Python)

> **Bridge Global Streamers to Japanese Anime & Character TTS**
> Zero-runtime-dependency multilingual to Katakana phonetic converter for
> Japanese TTS engines (VOICEVOX, COEIROINK, AivisSpeech, VOICEPEAK,
> OpenJTalk, etc.), powered by the same Rust core as the Node.js binding.

**English** | [🇯🇵 日本語](README.ja.md)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success.svg)](pyproject.toml)

> ⚠️ **Not yet published to PyPI.** This binding currently only ships as
> source, built locally with [maturin](https://www.maturin.rs/). Pre-built
> wheels for all Tier 1 platforms and a PyPI Trusted Publishing CI
> pipeline are tracked in a fast-follow issue (mirroring the Node.js
> native binding's own rollout — see
> [#28](https://github.com/allpaqa-org/multilingual-katakana/issues/28)).

---

## Why this package?

Japanese character-voice TTS engines only accept Japanese phonemes
(Katakana / Hiragana / Kanji). When global stream comments arrive in
English, Chinese, Korean, Russian, Spanish, French, Vietnamese, Thai, or
internet slang, this package converts them into natural-sounding Katakana
so they can be read aloud by a Japanese TTS voice — without pulling in
heavy pronunciation dictionaries or ML models.

This is the **Python binding**: a thin wrapper around the same
[Rust core](https://github.com/allpaqa-org/multilingual-katakana/tree/main/crates/multilingual-katakana-core)
used by the [Node.js package](https://github.com/allpaqa-org/multilingual-katakana/tree/main/bindings/node),
built with [PyO3](https://pyo3.rs/). It passes the same
[language-neutral spec contract](https://github.com/allpaqa-org/multilingual-katakana/tree/main/spec/cases)
(100% of `spec/cases/*.json`) as every other binding.

## Installation (build from source)

Not yet on PyPI (see the note above) — build and install it into a
virtualenv with [maturin](https://www.maturin.rs/):

```bash
git clone https://github.com/allpaqa-org/multilingual-katakana.git
cd multilingual-katakana/bindings/python
python -m venv .venv && source .venv/bin/activate
pip install maturin
maturin develop --release   # editable install into the active venv
# or: maturin build --release -o dist && pip install dist/*.whl
```

Zero runtime dependencies either way (`dependencies = []` in
`pyproject.toml`).

## Quick Start

```python
from multilingual_katakana import to_katakana, KatakanaOptions, KatakanaConverter

print(to_katakana("Hello guys! GG WP"))
# => "ハローガイズ！ジージーダブリューピー"

print(to_katakana("你好！谢谢乾爹"))
# => "ニーハオ！シエシエガンディエ"

# Disable a language, or reuse the same options across many calls:
options = KatakanaOptions(enable_chinese=False)
converter = KatakanaConverter(options)
print(converter.convert("你好"))
# => "你好" (passed through unchanged; Safe Failure — never dropped)
```

## Options Reference

`KatakanaOptions` is an immutable dataclass; every field defaults to `True`
except where noted:

| Field | Description |
|---|---|
| `enable_cyrillic` | Russian/Cyrillic transliteration |
| `enable_korean` | Korean Hangul decomposition |
| `enable_chinese` | Mandarin Pinyin + Taiwan stream slang |
| `enable_spanish` | Spanish accents, `¡`/`¿`, digraphs |
| `enable_french` | Curated French words/phrases |
| `enable_vietnamese` | Vietnamese tone marks and phrases |
| `enable_thai` | Thai phrase mappings and open syllables |
| `enable_slang` | Streaming/gaming slang (`gg`, `pog`, `afk`, ...) |
| `enable_english` | English loanwords/CMU-style phonics |
| `normalize_prosody` | Collapses Katakana word spacing, normalizes punctuation |

`options.exclude` (user-defined literal/regex text protection, available in
the Node.js binding) is **not yet available** in this Python binding — see
the tracking issue for native `exclude` support.

## Architecture & Quality Gates

- **Zero runtime dependencies**: `dependencies = []` in `pyproject.toml`.
- **Spec-driven**: 100% of `spec/cases/*.json` must pass — the same
  contract enforced for the TypeScript/Node.js and Rust core bindings.
- **Safe Kanji Guard / Safe Failure**: unrecognized text, emoji, and
  symbols are always passed through unchanged, never dropped or crashed on.

See the [main repository README](https://github.com/allpaqa-org/multilingual-katakana#readme)
and [`docs/V0.4.0_BINDINGS_SCOPE.md`](https://github.com/allpaqa-org/multilingual-katakana/blob/main/docs/V0.4.0_BINDINGS_SCOPE.md)
for the full multi-language bindings rollout plan.

## License

MIT — see [`LICENSE`](../../LICENSE).
