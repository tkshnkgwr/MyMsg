# MyMsg リリース手順書 (RELEASE)

本書は、`MyMsg` のリリースビルド作成、バイナリ最適化、サイズ極小化、および配布パッケージングの手順を定義します。

---

## 1. リリースビルドの作成

### 基本リリースビルド
```bash
cargo build --release
```
生成物: `target/release/MyMsg.exe` (Windows) または `target/release/MyMsg` (Unix-like)

---

## 2. バイナリ最適化とサイズ極小化

`Cargo.toml` に以下の `[profile.release]` 設定を追加・適用することで、バイナリサイズを大幅に削減し、実行速度を向上させることができます。

```toml
[profile.release]
opt-level = 3          # 最高レベルの最適化
lto = true             # リンク時最適化 (Link-Time Optimization) の有効化
codegen-units = 1      # コード生成ユニットを1つに集約して最適化を最大化
panic = "abort"        # パニック時のアンワインドスタックトレースを削除
strip = true           # デバッグシンボルをバイナリから完全に除去
```

### さらにサイズを削減する場合 (UPX圧縮)
Windows 実行可能ファイルに対して `upx` を適用することで、実行可能ファイルのサイズをさらに 30%〜60% 程度削減可能です。

```bash
# UPX がインストールされている場合
upx --best --lzma target/release/MyMsg.exe
```

---

## 3. リリース前チェックリスト

- [ ] `cargo test` で全単体テストが通過すること。
- [ ] `cargo fmt --check` および `cargo clippy` に警告がないこと。
- [ ] `MyMsg.exe --help` でコンソール出力が正常に動作すること。
- [ ] 日本語文字（漢字・ひらがな）が文字化けせず綺麗に描画されること。
- [ ] `docs/ja/CHANGELOG.md` および `docs/en/CHANGELOG.md` に今回のバージョンの変更履歴が記載されていること。
- [ ] `Cargo.toml` の `version` が更新されていること。

---

## 4. 配布パッケージング

1. `target/release/MyMsg.exe`、`README_JA.md`、`LICENSE` を ZIP アーカイブに圧縮。
2. GitHub Releases にタグ（例: `v0.1.0`）を付与して ZIP ファイルを添付。
