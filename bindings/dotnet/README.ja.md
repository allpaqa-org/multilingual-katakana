# Allpaqa.MultilingualKatakana (.NET)

> **世界中の配信コメントを日本のアニメ・キャラクターTTSへ橋渡し**
> 依存ゼロの多言語→カタカナ変換ライブラリ。VOICEVOX、COEIROINK、
> AivisSpeech、VOICEPEAK、OpenJTalk 等の日本語TTSエンジン向け。Node.js /
> Python バインディングと同じ Rust コアを使用しています。

[English](README.md) | **日本語**

> **まだNuGetには公開されていません。** 最初のリリースまでは、下記の手順で
> ソースからパッケージをビルドしてください。

---

## なぜこのパッケージなのか

日本語キャラクターボイスのTTSエンジンは日本語音素（カタカナ・ひらがな・漢字）
しか受け付けません。英語・中国語・韓国語・ロシア語・スペイン語・フランス語・
ベトナム語・タイ語・配信スラングで届く世界中のコメントを自然なカタカナへ変換し、
日本語TTSボイスで読み上げられるようにします。

これは **.NET バインディング** です。
[Rust コア](https://github.com/allpaqa-org/multilingual-katakana/tree/main/crates/multilingual-katakana-core)
の C ABI（`crates/multilingual-katakana-ffi`）を P/Invoke で薄くラップしたもので、
他の全バインディングと同じ
[言語中立の仕様契約](https://github.com/allpaqa-org/multilingual-katakana/tree/main/spec/cases)
（`spec/cases/*.json` の100%）をパスします。

- `netstandard2.0`（.NET Framework 4.6.2+ / .NET Core）と
  `net8.0`（`LibraryImport`、トリミング / Native AOT 対応）をターゲット。
- どちらのターゲットでも **NuGet 依存ゼロ**。
- Unity / Mono は `runtimes/{rid}/native` を自動解決しないため、対象プラットフォームの
  ネイティブライブラリをアセンブリと同じ場所（Unity の `Plugins` フォルダ等）へ手動で配置してください。

## インストール

```bash
dotnet add package Allpaqa.MultilingualKatakana
```

### ソースからビルド

```bash
git clone https://github.com/allpaqa-org/multilingual-katakana.git
cd multilingual-katakana
bun run scripts/stage_dotnet_native.ts   # cargo build + native/<ホストRID>/ へ配置
cd bindings/dotnet
dotnet test -c Release
dotnet pack src/Allpaqa.MultilingualKatakana -c Release -o ./artifacts
```

ローカルで pack した `.nupkg` には `bindings/dotnet/native/{rid}/` に配置済みの
ネイティブライブラリのみが含まれます（既定ではホストRIDのみ）。

## クイックスタート

```csharp
using Allpaqa.MultilingualKatakana;

Console.WriteLine(Katakana.ToKatakana("Hello guys! GG WP"));
// => "ハローガイズ！ジージーウェルプレイド"

Console.WriteLine(Katakana.ToKatakana("你好！谢谢乾爹"));
// => "ニーハオ！シエシエガンディエ"

// 言語を無効化したり、同じオプションを多数の呼び出しで再利用できます:
var converter = new KatakanaConverter(new KatakanaOptions { EnableChinese = false });
Console.WriteLine(converter.Convert("你好"));
// => "你好"（そのまま素通し。Safe Failure — 削除されません）
```

全メンバーはスレッドセーフです。`KatakanaConverter` は生成時にオプションを
スナップショットするため、その後 `KatakanaOptions` を変更しても影響しません。

## オプション一覧

`KatakanaOptions` の各プロパティは `bool?` です。`null`（既定値）はコアの既定値
（すべて `true` = 有効）を使用します。

| プロパティ | 説明 |
|---|---|
| `EnableCyrillic` | ロシア語/キリル文字の音写 |
| `EnableKorean` | 韓国語ハングルの分解 |
| `EnableChinese` | 中国語ピンイン + 台湾配信スラング |
| `EnableSpanish` | スペイン語のアクセント、`¡`/`¿`、二重字 |
| `EnableFrench` | 厳選したフランス語の単語・フレーズ |
| `EnableVietnamese` | ベトナム語の声調記号・フレーズ |
| `EnableThai` | タイ語フレーズと開音節 |
| `EnableSlang` | 配信/ゲームスラング（`gg`, `pog`, `afk` など） |
| `EnableEnglish` | 英語の外来語/CMU風フォニックス |
| `NormalizeProsody` | カタカナ間スペースの除去、約物の正規化 |

`options.exclude`（Node.js バインディングで利用できる、ユーザー定義の
リテラル/正規表現によるテキスト保護）は、この .NET バインディングでは
**まだ利用できません**。

## エラー

- `ArgumentNullException` — `text` が `null` の場合。
- `KatakanaException` — ネイティブライブラリを読み込めない（未対応プラット
  フォーム、ネイティブアセットの欠落、ABI バージョン不一致、セルフチェック失敗）
  またはコアがエラーを返した場合（`StatusCode`: 1 = null ポインタ、
  2 = 不正な UTF-8、3 = コア内で捕捉した panic）。読み込み失敗は初回に一度だけ
  検出・キャッシュされ、以降の呼び出しでも同じメッセージで送出されます
  （`TypeInitializationException` にはなりません）。

## 対応プラットフォーム（RID）

| OS | RID |
|---|---|
| Windows | `win-x64`, `win-x86`, `win-arm64` |
| Linux (glibc) | `linux-x64`, `linux-arm64` |
| Linux (musl / Alpine) | `linux-musl-x64` |
| macOS | `osx-x64`, `osx-arm64` |

.NET Core / .NET 5+ では `runtimes/{rid}/native/` からネイティブライブラリが
自動解決されます。

### .NET Framework での注意

.NET Framework は `runtimes/{rid}/native/` を解決しません。本パッケージは
`build/` と `buildTransitive/` の MSBuild targets を同梱しており、Windows 用
ネイティブ DLL を出力ディレクトリの `x86\`・`x64\`・`arm64\` サブフォルダへ
コピーします。初回利用時に、プロセスのアーキテクチャに合ったものを読み込みます。
`AnyCPU`（「32 ビットを優先」を含む）とプラットフォーム固有ビルドの両方で
動作します。手動で配置する場合は、これらのサブフォルダを
`Allpaqa.MultilingualKatakana.dll` と同じ場所に置いてください。

## ライセンス

MIT — [`LICENSE`](https://github.com/allpaqa-org/multilingual-katakana/blob/main/LICENSE) を参照。
