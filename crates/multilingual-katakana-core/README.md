# multilingual-katakana-core

Ultra-fast (<0.005ms), zero-runtime-dependency multilingual to Katakana converter for Japanese Text-to-Speech (TTS) engines (VOICEVOX, COEIROINK, etc.).

Supports English, Chinese (Mandarin/Taiwanese), Korean (Hangul), Russian (Cyrillic), Spanish, and stream/gaming slang.

## Features

- **Blazing Fast**: ~3.3 µs per phrase (~300,000 phrases/sec).
- **Zero Runtime Dependencies**: Compiled-in zero-allocation static lookup tables.
- **Safe Kanji Guard**: Strictly protects Japanese pure Kanji phrases (e.g. 了解, 初見歓迎, 神回) from being falsely converted to Chinese Pinyin.
- **Collision-Safe Exclusion**: Protect user names, Bot triggers, URLs, and code blocks using `options.exclude` with Unicode PUA tokens.

## Usage

```rust
use multilingual_katakana_core::{to_katakana, KatakanaOptions};

// Simple conversion
assert_eq!(to_katakana("hello world", None), "ハローワールド");
assert_eq!(to_katakana("안녕하세요", None), "アンニョンハセヨ");
assert_eq!(to_katakana("謝謝", None), "シエシエ");
assert_eq!(to_katakana("Привет", None), "プリヴィエト");
assert_eq!(to_katakana("muchas gracias", None), "ムチャスグラシアス");
assert_eq!(to_katakana("gg wp", None), "ジージーウェルプレイド");

// With exclude protection
let opts = KatakanaOptions::new().with_exclude("bot");
assert_eq!(to_katakana("hello bot nice", Some(&opts)), "ハロー bot ナイス");
```

## License

MIT
