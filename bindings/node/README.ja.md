# @allpaqa/multilingual-katakana

> **Bridge Global Streamers to Japanese Anime & Character TTS**  
> 世界中のコメントを日本のキャラクターボイス（VOICEVOX, COEIROINK, AivisSpeech, VOICEPEAK 等）で愛らしく読み上げる、ゼロ依存・爆速多言語カタカナ化ライブラリ。

[English](README.md) | **日本語**

[![npm version](https://img.shields.io/npm/v/@allpaqa/multilingual-katakana.svg)](https://www.npmjs.com/package/@allpaqa/multilingual-katakana)
[![CI](https://github.com/allpaqa-org/multilingual-katakana/actions/workflows/ci.yml/badge.svg)](https://github.com/allpaqa-org/multilingual-katakana/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Zero Dependencies](https://img.shields.io/badge/dependencies-0-success.svg)](package.json)
[![Tests: 100%](https://img.shields.io/badge/tests-124%20passed-brightgreen.svg)](spec/cases/)
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
| `enableSlang` | `boolean` | `true` | 配信・ゲームスラング（gg, w, pog等）変換の有効化 |
| `normalizeProsody` | `boolean` | `true` | カタカナ間の空白自動除去・約物正規化の有効化 |

---

## 🏛️ アーキテクチャと品質保証 (Architecture & Quality Gates)

本リポジトリは、言語中立なテスト仕様契約（**Spec-Driven Development**）に基づいて設計されています。

- **仕様契約 (`spec/cases/*.json`)**:
  9 スイート（英語、中国語、韓国語、ロシア語、スペイン語、スラング、漢字保護、プロソディ、複合コメント）に及ぶ全テストケースが JSON で定義されており、TypeScript 版および将来の Rust コアで同一のテストを 100% パスします。
- **品質基準 (Quality Gates)**:
  - ✅ **テスト全件パス**: 124 件のテストが 100% 成功（約 100ms）
  - ✅ **静的解析**: Biome による 0 errors, 0 warnings
  - ✅ **コード複雑度**: 全関数が **CC（Cyclomatic Complexity） <= 15**、Cognitive Complexity <= 15 を厳守

```bash
# テストの実行
bun run test

# 静的解析 & フォーマット
bun run check

# 循環的複雑度（Cyclomatic Complexity）の監査
bun run check:complexity

# テストケース & 辞書の JSON スキーマ検証
bun run validate:spec
```

---

## 🗺️ ロードマップ (Roadmap)

- [x] **v0.1.0**: TypeScript ゼロ依存実装（Dual ESM/CJS, PUA エスケープ保護, 10+言語/スラング対応）
- [ ] **v0.2.0**: Core ロジックの Rust 化（`crates/multilingual-katakana-core` による Single Source of Truth 化）
- [ ] **v0.3.0**: 多言語バインディング展開（NAPI-RS による Node ネイティブ、WASM、Python、C#）

---

## 📄 ライセンス (License)

[MIT License](LICENSE) © 2026 allpaqa
