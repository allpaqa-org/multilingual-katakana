# リリース自動化 & プロトコル手順書 (Release Process)

本ドキュメントは、`multilingual-katakana` における完全自律型リリース自動化プロトコル（CI/CD、ドラフトリリース、AI によるリリースノート自律更新、npm 公開）の手順とルールを定めたものです。
AI エージェントおよび開発者は本プロトコルに厳格に従ってリリース作業を実行してください。

---

## 1. リリース理念 (Release Philosophy)

1. **完全ゼロ依存 (Zero Dependencies)**
   - 外部ランタイム依存を持たず、Pure TypeScript および Pure Rust による軽量・セキュア・決定論的動作を保証します。
2. **品質検証ゲート 100% 遵守 (Quality Gate 100%)**
   - CI およびローカル環境において、全単体テスト・Biome 静的解析・コード複雑度（CC <= 15）・仕様スキーマ検証・Rust Clippy/Fmt の全項目が 100% パスすることをリリースの絶対条件とします。
3. **AI 自律更新プロトコル (`AGENTS.md` 第 5 項準拠)**
   - GitHub Actions によるドラフト作成後、AI が自律的にリリースノートをリッチ化し、ユーザーにワンクリック承認を案内する自律完結型リリース運用を行います。

---

## 2. リリース自動化アーキテクチャ概要

```text
[1. バージョン整合性更新 & ローカル事前検証]
  - package.json (root), bindings/node/package.json, crates/multilingual-katakana-core/Cargo.toml の同期更新
  - 品質ゲート（Bun + Cargo）全件グリーン確認
       ↓
[2. コミット & タグ作成 & push]
  - git commit -am "chore(release): vX.Y.Z"
  - git tag vX.Y.Z
  - git push origin main --tags
       ↓
[3. CD: release.yml 起動]
  - GitHub Actions がトリガー (push: tags: ['v*'])
  - 品質ゲート通過後、Dual ESM/CJS ビルド & npm pack
  - ドラフトリリース (draft: true) を自動作成し、dist/*.tgz を添付
       ↓
[4. AI によるリリースノート自律整形]
  - GitHub Actions 完了後、AI が gh release edit を実行
  - 変更内容・主な新機能・配布アセット一覧を整理した Markdown 本文へ自律更新
       ↓
[5. ワンクリック公開 (Publish release)]
  - ユーザーが GitHub Releases 画面で "Publish release" を 1 クリック
       ↓
[6. CD: publish.yml 起動 & 自動 npm publish]
  - release (types: [published]) イベントにより起動
  - Sigstore Provenance 付きで npm 公式レジストリへ @allpaqa/multilingual-katakana が公開完了
```

---

## 3. バージョニング整合性ルール (Monorepo Versioning Rule)

本リポジトリは TypeScript バインディングと Rust コアのモノレポ構造を採用しています。
セマンティックバージョニング（SemVer）に従い、以下の **3 箇所のバージョン番号を必ず同一（例: `0.2.0`）に同期更新** してください：

1. **ルート `package.json`**: `"version": "X.Y.Z"`
2. **Node.js バインディング `bindings/node/package.json`**: `"version": "X.Y.Z"`
3. **Rust コアクレート `crates/multilingual-katakana-core/Cargo.toml`**: `version = "X.Y.Z"`

---

## 4. 事前検証コマンド（品質検証ゲート）

リリースタグを作成する前に、ローカル環境で以下の検証コマンドをすべて実行し、**全項目が 100% グリーン（0 errors, 0 warnings）** であることを確認します：

```bash
# --- TypeScript バインディング & リポジトリ共通検証 ---
# 1. 単体テスト全件パス (124 tests PASS)
bun run test

# 2. Biome 静的解析・フォーマットチェック (0 errors, 0 warnings)
bun run check

# 3. 循環的複雑度監査 (Cyclomatic Complexity <= 15)
bun run check:complexity

# 4. 言語仕様・辞書 JSON スキーマ検証 (Spec Schema PASS)
bun run validate:spec

# 5. Dual ESM/CJS パッケージビルド (dist/ 出力検証)
bun run build

# --- Rust コア検証 ---
# 6. Cargo テスト全件パス (100% PASS, 103 spec cases)
cargo test --all-targets

# 7. Clippy 静的解析 (0 warnings required)
cargo clippy --all-targets -- -D warnings

# 8. Rust コードフォーマット検証 (0 diffs required)
cargo fmt --check
```

---

## 5. タグ作成と push

全検証をパスしたら、リリースコミットを作成し、バージョンタグを打ってリモートへ push します：

```bash
git commit -am "chore(release): vX.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```

---

## 6. ドラフトリリース自動作成と AI による自律更新

1. **GitHub Actions (`.github/workflows/release.yml`) の自動実行**:
   - `v*` タグの push を検知してワークフローが起動します。
   - Rust / Bun の品質ゲートを再検証し、Dual ESM/CJS ビルドおよび `npm pack` を実行。
   - `dist/*.tgz` アセットを添付した **ドラフトリリース (`draft: true`)** を自動生成します。

2. **AI によるリリースノート自律更新 (`AGENTS.md` 第 5 項)**:
   - AI はドラフトリリースの作成完了を検知後、GitHub CLI を実行してリリースノートを最新の変更点・新機能・アセット情報を含むリッチな Markdown に更新します：
   ```bash
   gh release edit vX.Y.Z \
     --title "vX.Y.Z: <リリース概要タイトル>" \
     --notes "<詳細な変更点・主な新機能・配布アセット一覧>"
   ```

---

## 7. ワンクリック公開と自動 npm publish

1. **ワンクリック公開**:
   - リリースノート更新完了後、AI からユーザーへ GitHub Releases の URL が案内されます。
   - ユーザーは内容を確認し、GitHub 画面上の **「Publish release」** ボタンを 1 クリックします。

2. **自動 npm publish (`.github/workflows/publish.yml`)**:
   - リリースが公開（published）されると、`.github/workflows/publish.yml` が自動起動します。
   - OIDC トークンによる Sigstore 真正性証明書（Provenance）付きで、公式 npm レジストリへ `@allpaqa/multilingual-katakana` が安全に公開されます。
