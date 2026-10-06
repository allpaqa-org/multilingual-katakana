# allpaqa-multilingual-katakana (Python)

> **世界中の配信コメントを日本のアニメ・キャラクターTTSへ橋渡し**
> ランタイム依存ゼロの多言語→カタカナ変換ライブラリ。VOICEVOX、COEIROINK、
> AivisSpeech、VOICEPEAK、OpenJTalk 等の日本語TTSエンジン向け。Node.js
> バインディングと同じ Rust コアを使用しています。

[🇬🇧 English](README.md) | **日本語**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success.svg)](pyproject.toml)

> ⚠️ **まだPyPIには公開されていません。** Tier 1 abi3 wheel ビルド行列と
> PyPI Trusted Publishing パイプライン
> （[#28](https://github.com/allpaqa-org/multilingual-katakana/issues/28)、
> `.github/workflows/build-python-matrix.yml`）自体は整備済みですが、
> このパイプライン経由のリリースはまだ実行されていません。それまでは
> [maturin](https://www.maturin.rs/) によるローカルソースビルドで
> ご利用ください。

---

## なぜこのパッケージなのか

日本語キャラクターボイスのTTSエンジンは日本語音素（カタカナ・ひらがな・漢字）
しか受け付けません。英語・中国語・韓国語・ロシア語・スペイン語・フランス語・
ベトナム語・タイ語・配信スラングで届く世界中のコメントを、重い発音辞書や
機械学習モデルを持ち込まずに自然なカタカナへ変換し、日本語TTSボイスで
読み上げられるようにします。

これは **Python バインディング** です。Node.js パッケージと同じ
[Rust コア](https://github.com/allpaqa-org/multilingual-katakana/tree/main/crates/multilingual-katakana-core)
を [PyO3](https://pyo3.rs/) で薄くラップしたもので、他の全バインディング
と同じ[言語中立の仕様契約](https://github.com/allpaqa-org/multilingual-katakana/tree/main/spec/cases)
（`spec/cases/*.json` の100%）をパスします。

## インストール（ソースからビルド）

上記の通りまだPyPI未公開のため、[maturin](https://www.maturin.rs/) で
仮想環境にビルド・インストールしてください:

```bash
git clone https://github.com/allpaqa-org/multilingual-katakana.git
cd multilingual-katakana/bindings/python
python -m venv .venv && source .venv/bin/activate
pip install maturin
maturin develop --release   # 有効化中のvenvへエディタブルインストール
# または: maturin build --release -o dist && pip install dist/*.whl
```

いずれの場合もランタイム依存はゼロです（`pyproject.toml` の
`dependencies = []`）。

## クイックスタート

```python
from multilingual_katakana import to_katakana, KatakanaOptions, KatakanaConverter

print(to_katakana("Hello guys! GG WP"))
# => "ハローガイズ！ジージーウェルプレイド"

print(to_katakana("你好！谢谢乾爹"))
# => "ニーハオ！シエシエガンディエ"

# 特定言語を無効化、または同じオプションを使い回す場合:
options = KatakanaOptions(enable_chinese=False)
converter = KatakanaConverter(options)
print(converter.convert("你好"))
# => "你好"（未変換のまま維持 — Safe Failure: 削除されない）
```

## オプション一覧

`KatakanaOptions` はイミュータブルなdataclassで、特に記載のない限り全
フィールドのデフォルトは `True` です。

| フィールド | 説明 |
|---|---|
| `enable_cyrillic` | ロシア語（キリル文字）音訳 |
| `enable_korean` | 韓国語ハングル分解 |
| `enable_chinese` | 中国語ピンイン + 台湾配信スラング |
| `enable_spanish` | スペイン語アクセント、`¡`/`¿`、二重字 |
| `enable_french` | フランス語の単語・フレーズ |
| `enable_vietnamese` | ベトナム語声調記号・フレーズ |
| `enable_thai` | タイ語フレーズ・開音節変換 |
| `enable_slang` | 配信・ゲームスラング（`gg`、`pog`、`afk` 等） |
| `enable_english` | 英語外来語・発音ルール |
| `normalize_prosody` | カタカナ語間スペース除去・約物正規化 |

Node.js バインディングにある `options.exclude`（ユーザー定義の
文字列・正規表現による変換保護）は、この Python バインディングでは
**まだ利用できません** — ネイティブ `exclude` 対応は別途追跡します。

## アーキテクチャと品質ゲート

- **ランタイム依存ゼロ**: `pyproject.toml` の `dependencies = []`
- **仕様駆動**: `spec/cases/*.json` の100%パスを必須とし、TypeScript/
  Node.js・Rustコアと同一の契約を適用
- **Safe Kanji Guard / Safe Failure**: 未知の言語・絵文字・記号は常に
  原文のまま維持し、削除やクラッシュをしない

多言語バインディング全体のロードマップは
[メインリポジトリのREADME](https://github.com/allpaqa-org/multilingual-katakana#readme)
および
[`docs/V0.4.0_BINDINGS_SCOPE.md`](https://github.com/allpaqa-org/multilingual-katakana/blob/main/docs/V0.4.0_BINDINGS_SCOPE.md)
を参照してください。

## ライセンス

MIT — [`LICENSE`](../../LICENSE) を参照。
