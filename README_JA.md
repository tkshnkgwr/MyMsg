<div align="center">

# ⚡ MyMsg (マイ・メッセージ)

**低リソース環境に最適化された、超軽量・最前面固定（Always on Top）メッセージポップアップ通知CLIツール**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 2021/2024](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/tkshnkgwr/MyMsg)

[English](./README.md) | [日本語](./README_JA.md)

</div>

---

## 📌 概要

`MyMsg` は、長時間のビルド処理、バッチジョブ、スクリプト実行完了時やリマインダー通知を目的として開発された、最前面固定のメッセージポップアップ通知CLIツールです。

`Rust` および `egui/eframe` によるネイティブ実装により、起動の瞬時性・極低メモリ消費（約15〜30MB）・イベント駆動描画によるアイドル時CPU 0%を実現しています。

---

## ✨ 主な特徴

- 🪟 **最前面固定（Always on Top）**: フルスクリーン作業中や別ウィンドウ操作中でも確実に通知が視界に入ります。
- ⚡ **キーボード即時終了**: `Esc` または `Enter` キーを押すだけで瞬時にプロセス終了（終了コード 0）。
- 🎨 **柔軟・寛容なカラー指定**:
  - Web標準カラー名（`red`, `green`, `blue`, `gold`, `crimson`, `navy` 等 24色以上）
  - 日本語カラー名（`赤`, `青`, `緑`, `黄`, `白`, `黒`）
  - 1文字略称（`r`, `g`, `b`, `y`, `w`, `k`, `c`, `m`, `o`, `p`）
  - タイポ自動補正（`bule` → 青）
  - HEXカラーコード（`#RGB`, `#RRGGBB`, `#RRGGBBAA`、先頭の `#` 省略可能）
- ⏱️ **ゼロ負荷タイマー / 遅延表示（`--delay` / `-d`）**: 指定秒数待機後にポップアップ（待機中はGUI非生成でリソース消費ゼロ、最大3600秒安全ガード付き）。
- 🚨 **点滅モード（`--blink` / `-b`）**: 緊急通知用の明滅エフェクト（約0.5秒周期）。
- 🔤 **日本語フォント自動検出**: OS（Windows/macOS/Linux）標準の日本語フォントを自動検出し、文字化け（豆腐化）を防止。
- 📦 **単一バイナリ**: 外部依存ライブラリ不要のシングル実行ファイル。

---

## 🚀 インストール & ビルド

### 必要要件
- Rust 1.80 以上 (Cargo)

### ローカルビルド
```bash
git clone https://github.com/tkshnkgwr/MyMsg.git
cd MyMsg
cargo build --release
```
生成された実行ファイルは `target/release/MyMsg.exe`（または `MyMsg`）に配置されます。

---

## 📋 CLI オプション一覧

```
Usage: MyMsg.exe [OPTIONS] [MESSAGE]
```

| 引数 / オプション | 短縮形 | デフォルト値 | 説明 |
| :--- | :---: | :---: | :--- |
| `[MESSAGE]` | - | なし | 表示するメッセージ文字列（位置引数） |
| `--message <STR>` | `-m` | なし | 表示するメッセージ文字列（オプション引数） |
| `--size <SIZE>` | `-s` | `medium` | ウィンドウサイズ (`small`: 300x150, `medium`: 450x220, `large`: 650x350) |
| `--font-size <PT>` | - | 自動算出 | 文字サイズ（pt単位、指定時はサイズプリセットより優先） |
| `--color <COLOR>` | `-c` | `white` | 文字色（名前、1文字略称、タイポ、#HEX） |
| `--bg-color <COLOR>` | - | `#1a1b26` | ウィンドウ背景色（ダークネイビー調） |
| `--blink` | `-b` | `false` | メッセージ文字の明滅エフェクトを有効化 |
| `--font <FONT>` | `-f` | `default` | フォント種別 (`default`/`sans`, `mono`/`2`, `serif`/`3`, `impact`) |
| `--delay <SEC>` | `-d` | `0` | 表示までの遅延時間（秒単位、0〜3600秒） |
| `--help` | `-h` | - | ヘルプメッセージを表示して終了 |
| `--version` | `-V` | - | バージョン情報を表示して終了 |

> [!NOTE]
> メッセージ文字列は、位置引数（`MESSAGE`）が指定されている場合はそちらが最優先され、次に `-m / --message`、いずれも省略された場合は `"MyMsg: 通知が届きました"` が表示されます。

---

## 💡 使用例

### 基本通知
```powershell
# 単純なメッセージ表示
MyMsg "ビルドが完了しました！"

# オプション引数による指定
MyMsg -m "タスク完了"
```

### カラー・デザインのカスタマイズ
```powershell
# 警告カラー（赤文字・大サイズ）
MyMsg "エラーが発生しました" -c red -s large

# 1文字略称と背景色指定
MyMsg "サーバー正常起動" -c g --bg-color "#0f172a"

# HEXコード指定とフォントサイズ指定
MyMsg "処理完了" -c "#00E5FF" --font-size 32
```

### 緊急アラート & タイマー通知
```powershell
# 点滅表示（緊急通知）
MyMsg "⚠ ディスク容量が不足しています" -c yellow -b

# 5分後（300秒後）にリマインド通知（待機中リソース消費0）
MyMsg "ミーティングの時間です" -d 300 -c gold
```

### PowerShell / バッチファイル連携
```powershell
# 長時間スクリプト終了時に通知
npm run build; MyMsg "npm build 完了！" -c cyan
```

---

## 📚 ドキュメント一覧

詳細な仕様および設計書は `docs/` ディレクトリ配下に格納されています。

| 日本語ドキュメント (docs/ja/) | 英語ドキュメント (docs/en/) | 概要 |
| :--- | :--- | :--- |
| [詳細仕様書](docs/ja/SPECIFICATION.md) | [Specification](docs/en/SPECIFICATION.md) | 全引数・UI・キーバインド・終了条件の完全仕様 |
| [内部アーキテクチャ](docs/ja/ARCHITECTURE.md) | [Architecture](docs/en/ARCHITECTURE.md) | モジュール設計・データフロー・描画ループ |
| [ユーザーガイド](docs/ja/USER_GUIDE.md) | [User Guide](docs/en/USER_GUIDE.md) | 実践的な逆引きコマンド集・シェル連携 |
| [開発ガイド](docs/ja/DEVELOPMENT.md) | [Development](docs/en/DEVELOPMENT.md) | 開発環境セットアップ・単体テスト実行 |
| [リリース手順](docs/ja/RELEASE.md) | [Release Guide](docs/en/RELEASE.md) | 最適化ビルド・バイナリ配布手順 |
| [AI開発指示書](docs/ja/INSTRUCTIONS.md) | [AI Instructions](docs/en/INSTRUCTIONS.md) | AIエージェント向け運用ガイドライン |
| [テスト仕様書](docs/ja/TESTING.md) | [Testing](docs/en/TESTING.md) | 単体テスト・手動検証チェックリスト |
| [テスト実行報告書](docs/ja/TEST_REPORT.md) | [Test Report](docs/en/TEST_REPORT.md) | 初回バージョンの動作検証結果記録 |
| [開発タスクリスト](docs/ja/TODO.md) | [TODO](docs/en/TODO.md) | 実装済み機能と今後の拡張ロードマップ |
| [リソース指標](docs/ja/FOOTPRINTS.md) | [Footprints](docs/en/FOOTPRINTS.md) | メモリ・起動時間・CPU使用率ベンチマーク |
| [変更履歴](docs/ja/CHANGELOG.md) | [Changelog](docs/en/CHANGELOG.md) | バージョン別変更履歴 |
| [セキュリティ方針](docs/ja/SECURITY.md) | [Security](docs/en/SECURITY.md) | セキュリティポリシー・脆弱性報告窓口 |
| [コントリビューション](docs/ja/CONTRIBUTING.md) | [Contributing](docs/en/CONTRIBUTING.md) | 開発参加・Issue/PRガイドライン |

---

## 📄 ライセンス

本プロジェクトは [MIT License](LICENSE) の下で公開されています。
