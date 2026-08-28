# MyMsg 開発ガイド (DEVELOPMENT)

本書は、`MyMsg` のローカル開発環境のセットアップ、ビルド、テスト実行、コードフォーマット、およびデバッグ手順について説明します。

---

## 1. 開発環境のセットアップ

### 前提条件
- **Rust ツールチェーン**: Rust 1.80 以上 (最新の stable 推奨)
- **Git**: バージョン管理

```bash
# Rust のバージョン確認
rustc --version
cargo --version
```

### リポジトリのクローン
```bash
git clone https://github.com/tkshnkgwr/MyMsg.git
cd MyMsg
```

---

## 2. ローカルビルド & 実行

### デバッグビルド & 実行
```bash
# デバッグビルド
cargo build

# 直接実行（引数渡し）
cargo run -- "テストメッセージ" -c cyan

# ヘルプ表示確認
cargo run -- --help
```

---

## 3. テストの実行

プロジェクトに含まれる単体テストを実行します。

```bash
# 全テスト実行
cargo test

# テスト名を指定して実行
cargo test test_parse_color

# テスト出力を標準出力に表示しながら実行
cargo test -- --nocapture
```

---

## 4. コード品質と静的解析

```bash
# コードフォーマットのチェック
cargo fmt --check

# コード自動フォーマット
cargo fmt

# Clippy による静的リント解析
cargo clippy -- -D warnings
```

---

## 5. デバッグとトラブルシューティング

- **日本語が表示されない場合**:
  Windows の `%WINDIR%\Fonts\meiryo.ttc` または `msgothic.ttc` がアクセス可能か確認してください。
- **CLI の `--help` が出力されない場合**:
  `src/main.rs` で `windows_subsystem = "windows"` 属性が有効になっていないか確認してください。
