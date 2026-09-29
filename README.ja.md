# @allpaqa/multilingual-katakana

> **Bridge Global Streamers to Japanese Anime & Character TTS**  
> 世界中のコメントを日本のキャラクターボイス（VOICEVOX, COEIROINK, AivisSpeech, VOICEPEAK 等）で愛らしく読み上げる、ゼロ依存・爆速多言語カタカナ化ライブラリ。

[English](README.md) | **日本語**

[![npm version](https://img.shields.io/npm/v/@allpaqa/multilingual-katakana.svg)](https://www.npmjs.com/package/@allpaqa/multilingual-katakana)
[![CI](https://github.com/allpaqa-org/multilingual-katakana/actions/workflows/ci.yml/badge.svg)](https://github.com/allpaqa-org/multilingual-katakana/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success.svg)](package.json)
[![Tests: 100%](https://img.shields.io/badge/tests-163%20passed-brightgreen.svg)](spec/cases/)
[![Complexity: CC<=15](https://img.shields.io/badge/complexity-CC%20%3C%3D%2015-success.svg)](scripts/check_complexity.ts)

---

## 🌟 特徴 (Features)

1. **🚀 完全ゼロ外部依存 (Zero Runtime Dependencies)**:
   - `dependencies` は**常に空（`{}`）**。重い辞書ライブラリ（CMU 辞書 38MB や pinyin-pro 3.5MB 等）を完全撤廃し、わずか **~86KB** のバンドルサイズに事前コンパイル内包。
2. **⚡ 圧倒的な爆速変換 (<0.1ms)**:
   - 単語探索・音節数学的分解・O(1) 辞書ルックアップにより、フレーズあたり 0.05ms〜0.1ms で完了。従来の多言語 TTS 前処理パイプラインと比較して **約50倍高速化**。
3. **🌐 多言語 ＋ 配信・ゲームスラング対応**:
   - **英語**: 外来語テーブル ＋ フォニックス・フォールバック
   - **中国語**: 台湾ネットスラング（886, 748, 草泥馬 等）＋ 普通話ピンイン音節変換
   - **韓国語**: ハングル音節の数学的分解（初声×588 ＋ 中声×28 ＋ 終声）＋ 連音化 ＋ 配信スラング
   - **ロシア語（キリル文字）**: 発音規則に基づく音素マッピング
   - **スペイン語**: 特殊文字（ñ, ll, rr）、アクセント記号、倒置疑問符/感嘆符（¡, ¿）の正規化
   - **フランス語**: リエゾンや語末の黙字を含む、フランス語固有性の高い単語・定型句辞書
   - **ベトナム語**: 声調記号の正規化、フレーズ・単語辞書、二重字の処理
   - **タイ語**: フレーズ辞書と保守的な開音節変換。曖昧な閉音節は原文を維持
   - **配信スラング**: 各種配信プラットフォーム（YouTube Live, Twitch, Kick, Discord 等）やゲーム特有の用語・略語（`gg`, `gg wp`, `ez`, `pog`, `poggers`, `kekw`, `afk`, `brb`, `lol`, `w`, `ww`, `草`）
4. **🛡️ 日本語漢字保護 (Safe Kanji Guard)**:
   - 日本語の純粋な漢字（`了解`, `初見歓迎`, `神回`, `配信開始`, `感謝`, `最高`, `優勝` 等）を中国語ピンインと絶対に誤判定しない安全設計。
5. **🎭 アニメ声・キャラクターTTS最適化 (Japanglish Prosody)**:
   - カタカナ単語間のスペース自動連結（`ハロー ガイズ` ➔ `ハローガイズ`）
   - 西欧約物の日本語正規化（`!` ➔ `！`, `?` ➔ `？`, `.` ➔ `。`, `,` ➔ `、`）
6. **🔒 区間ベースの保護・エスケープ機能 (`options.exclude`)**:
   - Bot の教育辞書（!remember）、URL、メンション（`@user`）、コード（`C++`, `node.js`）を変換から保護。
   - Unicode 私用領域（PUA: U+E000〜）と区間抽出（Interval-based Tokenizer）により、トークン衝突・二重置換・正規表現誤爆を 100% 排除。

---

## 🛡️ コア設計原則: Safe Failure（安全な失敗・優雅な縮退）

> **「ライブ配信において、発音が多少不格好なのは許容されるが、コメントを消す・URLを破壊する・TTSの読み上げを停止させることは致命的である。」**

`multilingual-katakana` は、以下の **Safe Failure** 原則を絶対契約として遵守します：
- **変換不能 ➔ 原文をそのまま維持**: 対応外の言語（アラビア文字、デーヴァナーガリー文字、グルジア文字等）や未知の記号・絵文字が混在しても、**絶対にテキストを消去・欠落させません**。原文をそのまま残し、TTSエンジンやフォールバック読み上げに委ねます。
- **日本語領域の絶対保護**: 平仮名・片仮名および配信で頻出する漢字（`初見歓迎`, `神回`, `了解` 等）を、外国語として誤爆変換することは絶対にありません。
- **オプションの厳格なスコープ**: 英語を無効（`enableEnglish: false`）にした場合、未知語がフォニックス（英語音声学）で誤って変形されるのを防ぎます。

---

## 🎯 目標 (Goals) と非目標 (Non-Goals)

### 目標 (Goals)
- 世界中の多言語コメントを、日本のキャラクターボイス（VOICEVOX等）で可愛く・愛らしく読み上げる。
- ライブ配信中のコメント読み飛ばし（スキップ）やミュートを激減させる。
- **完全ゼロ外部依存**・**2〜9 µs（マイクロ秒）**の爆速処理により、配信の音声レイテンシを極小化する。
- 配信者や視聴者が、リスニングや翻訳の手間なく、ノリと熱量を直感的に推測できる音響情報を提供する。

### 非目標 (Non-Goals)
- ❌ **ネイティブ発音の完全再現**: 学術的な発音記号の再現ではなく、あくまで「ジャパングリッシュ／愛らしいキャラボイス」としての成立を目的とします。
- ❌ **機械翻訳**: 意味を日本語に翻訳する機能ではありません（文字をカタカナの音へ翻字します）。
- ❌ **辞書の無限巨大化**: 10万語規模の巨大辞書を抱え込んでメモリや起動時間を肥大化させることは目指しません。
- ❌ **完全な多言語形態素解析**: 重厚な自然言語処理パーサーではなく、軽量なルールベース・ヒューリスティクスを優先します。

---

## 🤖 なぜ機械学習（LLM / AI）を使わないのか？

> *「LLM や機械学習モデルを使ったほうが学術的に高精度なのでは？」*  
> **はい。しかし、配信 TTS の前処理という目的関数に対して、それは根本的に過剰（Overkill）です。**

| 項目 | 機械学習 / LLM | `multilingual-katakana` |
|---|---|---|
| **レイテンシ** | 200 ms 〜 2,000 ms（配信の会話テンポが破綻） | **0.002 ms 〜 0.009 ms (2〜9 µs)** |
| **外部依存** | PyTorch, ONNX, GPUドライバ, 外部API | **完全ゼロ外部ランタイム依存**（純粋コード） |
| **フットプリント** | 数百MB 〜 数GB | **~86 KB (Node) / 完全自己完結 (Rust)** |
| **コスト・オフライン** | API課金、クラウド常時接続必須 | **100% 無料・完全オフライン動作** |
| **決定性** | ハルシネーション（予期せぬ読みの崩壊） | **100% 決定論的（仕様テスト済み）** |

---

## ⚡ シナリオ別実測ベンチマーク (Rust Core)

Apple Silicon 上でのリリースビルド実測値（キャッシュ温置、`multilingual-katakana-core`）：

| シナリオ | 文字数 | レイテンシ (µs) | レイテンシ (ms) | スループット (phrases/sec) |
|---|---|---|---|---|
| **短文チャット** (`gg wp bro`) | ~10 文字 | **1.90 µs** | 0.0019 ms | **526,000 ops/s** |
| **標準的な配信コメント** (`初見です！Hello streamer! 今日も配信楽しみにしてました！`) | ~40 文字 | **2.64 µs** | 0.0026 ms | **378,000 ops/s** |
| **多言語混在コメント** (`Hello! 你好! 안녕하세요! muchas gracias bro pog! Привет!`) | ~65 文字 | **9.43 µs** | 0.0094 ms | **106,000 ops/s** |
| **URL・メンション保護** (`check https://twitch.tv/example @streamer nice play gg!`) | ~55 文字 | **2.93 µs** | 0.0029 ms | **340,000 ops/s** |
| **極端な長文コピペ** (配信チャットの長文リピート) | ~810 文字 | **65.97 µs** | 0.0660 ms | **15,000 ops/s** |

## 📦 インストール (Installation)

```bash
# Bun
bun add @allpaqa/multilingual-katakana

# npm
npm install @allpaqa/multilingual-katakana

# pnpm / yarn
pnpm add @allpaqa/multilingual-katakana
yarn add @allpaqa/multilingual-katakana
```

ESM (ECMAScript Modules) および CommonJS (CJS)、TypeScript 型定義（`.d.ts`）に完全対応しています。

---

## 🚀 クイックスタート (Usage)

### 基本的な使い方 (`toKatakana`)

```typescript
import { toKatakana } from "@allpaqa/multilingual-katakana";

// 英語・スラング
console.log(toKatakana("hello world! gg wp!"));
// => "ハローワールド！ジージーウェルプレイド！"

// 韓国語（ハングル分解）
console.log(toKatakana("안녕하세요! 방송 너무 재밌어요 파이팅!"));
// => "アンニョンハセヨ！パンソンノムチェミッソヨパイティン！"

// スペイン語
console.log(toKatakana("¡Hola amigo! Muchas gracias señor"));
// => "オラアミゴ！ムチャスグラシアスセニョール"

// ロシア語（キリル文字）
console.log(toKatakana("Привет, как дела? Спасибо!"));
// => "プリヴィエト、カクジェラ？スパシーバ！"

// ベトナム語・タイ語
console.log(toKatakana("xin chào! สวัสดีครับ"));
// => "シンチャオ！サワッディークラップ"

// 中国語 & Safe Kanji Guard（日本語の漢字はピンイン化されず保護されます）
console.log(toKatakana("初見歓迎！ 886 谢谢大家"));
// => "初見歓迎！ バイバイ シェシェダージャー"
```

### 特定テキスト・URL・メンションの保護 (`options.exclude`)

Bot の教育機能や特定の単語、URL、ユーザーメンションなどをカタカナ変換させずにそのまま残すことができます。

```typescript
import { toKatakana } from "@allpaqa/multilingual-katakana";

// 単語の除外（Bot教育機能・独自辞書との連携）
toKatakana("hello bot nice to meet you", {
  exclude: ["bot", "nice"],
});
// => "ハロー bot nice トゥーミートユー"

// 正規表現による URL やメンションの保護
toKatakana("check https://example.com @streamer_123 gg", {
  exclude: [/https?:\/\/\S+/, /@\w+/],
});
// => "チェック https://example.com @streamer_123 ジージー"

// カタカナ単語の保護（周辺のスペース結合に巻き込まれず維持）
toKatakana("hello ワラ world", {
  exclude: ["ワラ"],
});
// => "ハロー ワラ ワールド"
```

### インスタンス再利用 (`KatakanaConverter`)

共通オプションを保持したコンバーターインスタンスを使い回すことができます。

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

## ⚙️ オプション一覧 (Options)

| オプション | 型 | デフォルト | 説明 |
|---|---|---|---|
| `exclude` | `(string \| RegExp)[]` | `[]` | 変換から保護する文字列または正規表現 |
| `enableEnglish` | `boolean` | `true` | 英単語テーブル ＋ フォニックス変換の有効化 |
| `enableChinese` | `boolean` | `true` | 台湾スラング・ピンイン音節変換の有効化 |
| `enableKorean` | `boolean` | `true` | ハングル音節数学的分解・慣用句変換の有効化 |
| `enableCyrillic` | `boolean` | `true` | ロシア語キリル文字音素マッピングの有効化 |
| `enableSpanish` | `boolean` | `true` | スペイン語挨拶・特殊文字変換の有効化 |
| `enableFrench` | `boolean` | `true` | フランス語固有性の高い単語・フレーズ辞書の有効化（未登録語は通常のフォールバック） |
| `enableVietnamese` | `boolean` | `true` | ベトナム語フレーズ・単語辞書と声調記号処理の有効化 |
| `enableThai` | `boolean` | `true` | タイ語フレーズ辞書・保守的な音節変換の有効化（曖昧な閉音節は原文維持） |
| `enableSlang` | `boolean` | `true` | 配信・ゲームスラング（gg, w, pog等）変換の有効化 |
| `normalizeProsody` | `boolean` | `true` | カタカナ間の空白自動除去・約物正規化の有効化 |

---

## 🏛️ アーキテクチャと品質保証 (Architecture & Quality Gates)

本リポジトリは、言語中立なテスト仕様契約（**Spec-Driven Development**）に基づいて設計されています。

- **仕様契約 (`spec/cases/*.json`)**:
  12 スイート（英語、中国語、韓国語、ロシア語、スペイン語、ベトナム語、タイ語、スラング、漢字保護、プロソディ、複合コメント、失敗モード）に及ぶ全テストケースが JSON で定義されており、TypeScript 版と Rust コアで同一のテストを 100% パスします。
- **品質基準 (Quality Gates)**:
  - ✅ **テスト全件パス**: TypeScript・Rust 共通の仕様テスト163件がすべて成功
  - ✅ **静的解析**: Biome による 0 errors, 0 warnings
  - ✅ **コード複雑度**: 全関数が **CC（Cyclomatic Complexity） <= 15**、Cognitive Complexity <= 15 を厳守
  - ✅ **超高速 Rust コア**: `crates/multilingual-katakana-core` にて平均 **3.38 µs** / 秒間約30万フレーズの圧倒的性能

```bash
# TypeScript バインディングの検証
bun run test
bun run check
bun run check:complexity
bun run validate:spec

# Rust コアの検証
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

CI/CD 自動化や配布プロトコルについては [リリース手順書 (docs/RELEASE_PROCESS.md)](docs/RELEASE_PROCESS.md) をご覧ください。

---

## 🗺️ ロードマップ (Roadmap)

- [x] **v0.1.0**: TypeScript ゼロ依存実装（Dual ESM/CJS, PUA エスケープ保護, 10+言語/スラング対応）
- [x] **v0.2.0**: Core ロジックの Rust 化（`crates/multilingual-katakana-core` による Single Source of Truth 化、<0.005ms、完全自己完結）
- [x] **v0.3.0**: フランス語辞書・スペイン語辞書拡充とベトナム語・タイ語対応。**破壊的変更:** Rust の公開構造体 `KatakanaOptions` に言語フラグを追加するため、全フィールドを指定する既存の構造体リテラルには新フィールドの追加が必要です（または `..Default::default()` を使用）。
- [ ] **v0.4.0**: NAPI-RS による Node.js ネイティブバックエンド（`toKatakana` / `KatakanaConverter` の API は無変更のドロップイン高速化。プラットフォーム別バイナリを `optionalDependencies` で配布し、非対応環境では Pure TypeScript へ自動フォールバック、`dependencies` は `{}` を維持）。詳細は [v0.4.0 バインディング スコープ定義](docs/V0.4.0_BINDINGS_SCOPE.md) を参照。
- [ ] **v0.5.0**: Python バインディング（PyO3 + maturin、PyPI への abi3 wheel 配布）
- [ ] **v0.6.0**: C# / .NET バインディング（C ABI + RID 別ネイティブアセット同梱の NuGet 配布）
- [ ] **今後（需要次第）**: ブラウザ・Edge ランタイム向けスタンドアロン WebAssembly パッケージ（`@allpaqa/multilingual-katakana-wasm`）

---

## 📄 ライセンス (License)

[MIT License](LICENSE) © 2026 allpaqa
