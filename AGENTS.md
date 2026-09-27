# multilingual-katakana Agent Guidelines

このリポジトリは、多言語テキスト（英・中・韓・露・西＋スラング）を日本語TTS向けに爆速（<0.1ms）・ゼロ依存でカタカナ化するオープンソースライブラリです。
すべてのAIエージェント（Antigravity, Cursor, Copilot 等）は、本リポジトリで作業する際に以下の設計原則と品質ゲートを厳守してください。

---

## 1. コア設計原則（Core Architectural Principles）

1. **Zero Runtime Dependencies（完全ゼロ外部依存）**:
   - `package.json` の `dependencies` は**常に空（`{}`）**を厳守すること。
   - 音素テーブルや辞書データはビルド時に事前コンパイルしてバンドルにインライン内包すること。
2. **Spec-Driven Development（仕様契約の死守）**:
   - `spec/cases/*.json` は言語中立な公開契約である。
   - 実装を変更した際は、全言語バインディング（TS / 将来の Rust）で同一のテストケースが 100% パスすること。
3. **Safe Kanji Guard（日本語漢字保護）**:
   - 日本語の漢字（了解、初見歓迎、神回 等）を中国語ピンインと絶対に誤判定しない安全設計を死守すること。
4. **Japanglish & Anime Character Experience**:
   - 目的は「正確な外国語翻訳」ではなく、「世界中のコメントを日本のキャラクターボイスで愛らしく読み上げるエンタメ演出」である。
   - カタカナ単語間のスペース自動除去、約物の日本語正規化、配信スラングの自然な読みを最優先する。

---

## 2. 品質検証ゲート（Quality Gates - 完了報告前の必須確認）

コード変更（機能追加、リファクタリング、辞書更新）を行った際は、ユーザーへの完了報告前に**必ず以下のすべてがグリーンであることを確認**すること：

1. **自動テストの実行**:
   ```bash
   cd bindings/node && bun test
   ```
   - 全テストケースが 100% PASS すること（回帰バグ厳禁）。
2. **Linter & Formatter**:
   ```bash
   cd bindings/node && bun run check
   ```
   - Biome による静的解析とフォーマットで **0 errors, 0 warnings** であること。
3. **コード複雑度の監査**:
   ```bash
   bun run scripts/check_complexity.ts
   ```
   - 全関数が **CC（Cyclomatic Complexity） <= 15**、Cognitive Complexity <= 15 を満たすこと。
4. **仕様スキーマ検証（辞書・テストケース変更時）**:
   ```bash
   bun run scripts/validate_spec.ts
   ```
5. **【補足】外部エージェント（GitHub Copilot CLI 等）による自律レビュー（任意）**:
   - Copilot CLI 等が利用可能な環境でセカンドオピニオンや品質検証を依頼する場合、非対話モードでのテスト自動実行を許可するため `--allow-all-tools` フラグを付与して呼び出すことを推奨する：
     ```bash
     copilot -p "品質ゲート（テスト・Linter・複雑度）の検証をお願いします。" --allow-all-tools --no-color
     ```

---

## 3. エスケープ・保護機能（Escape / Exclusion Architecture）

ユーザー定義単語（Botの教育機能）、URL、メンション、コード等をカタカナ変換から保護するため、`options.exclude: (string | RegExp)[]` をサポートする。
- **内部実装**: Unicode 私用領域（Private Use Area: `\uE000`〜）を使用したトークン一時退避・復元アルゴリズムを採用し、パイプラインの他の正規表現や約物正規化に一切干渉されない設計を維持すること。

---

## 4. 将来の Rust Core への移行方針

- `crates/multilingual-katakana-core` を Single Source of Truth（マスター）とし、`spec/cases/*.json` をパスさせること。
- TypeScript 版（`bindings/node`）は、Rust コアが完成した後も利用者のインターフェース（API）を 1 行も壊さずに移行できるよう、`toKatakana` および `KatakanaConverter` のシグネチャを安定させること。
