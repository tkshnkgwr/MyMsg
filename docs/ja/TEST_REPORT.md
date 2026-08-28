# MyMsg テスト実行報告書 (TEST_REPORT)

本書は、`MyMsg` v0.1.0 における単体テストおよび動作確認の実行結果を記録したエビデンス報告書です。

---

## 1. 実行環境情報

| 項目 | 値 |
| :--- | :--- |
| **OS** | Windows 11 (x86_64) |
| **Rust バージョン** | 1.80+ / stable-x86_64-pc-windows-msvc |
| **Cargo バージョン** | cargo 1.80+ |
| **対象バージョン** | v0.1.0 |
| **実行日時** | 2026-08-28 |

---

## 2. 単体テスト実行結果

```
running 9 tests
test tests::test_calculate_window_dimensions ... ok
test tests::test_clamp_delay_seconds ... ok
test tests::test_resolve_message_priority ... ok
test tests::test_parse_color_named_and_typo ... ok
test tests::test_parse_icon ... ok
test tests::test_parse_theme ... ok
test tests::test_resolve_message_newlines ... ok
test tests::test_resolve_theme_palette ... ok
test tests::test_parse_color_hex ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**結果**: 9件中 9件合格 (Pass 100%)

---

## 3. CLI 機能手動検証結果

| 検証項目 | 実行コマンド | 検証結果 | 判定 |
| :--- | :--- | :--- | :---: |
| **ヘルプ出力** | `MyMsg.exe --help` | コンソールに日本語 Usage・Options が即座に出力される | PASS |
| **バージョン出力** | `MyMsg.exe --version` | `mymsg 0.1.0` がコンソールに出力される | PASS |
| **日本語フォント描画** | `MyMsg.exe "日本語通知テスト"` | 文字化けせず游ゴシック/メイリオで鮮明に表示 | PASS |
| **キーボード即時終了** | `Esc` / `Enter` 押下 | 0ミリ秒遅延で即時プロセス終了（Exit Code 0） | PASS |
| **点滅エフェクト** | `MyMsg.exe "警告" -b` | 0.5秒周期で正確にアルファ明滅アニメーション | PASS |
| **遅延タイマー** | `MyMsg.exe "3秒後" -d 3` | 正確に3秒待機後にGUI起動 | PASS |
| **カラーパース** | `MyMsg.exe "青" -c bule` | タイポ補正され青色で表示 | PASS |
| **通知アイコン表示** | `MyMsg.exe "警告" -i warn` | ⚠ シンボルが黄色でメッセージ左側に表示 | PASS |
| **テーマ切り替え** | `MyMsg.exe "ライト" -t light` | 明るい背景と濃紺テキストで表示 | PASS |
| **複数行折り返し&中央揃え** | `MyMsg.exe "1行目\n2行目"` | 上部余白なくウィンドウ中央に上下左右中央揃えで配置 | PASS |
| **マルチモニター追従** | サブ画面から `MyMsg.exe "通知"` | カーソルのある画面の作業領域中央にポップアップ表示 | PASS |

---

## 4. 総括
すべての単体テスト（全9件）および手動検証において期待通りの動作が確認され、安定稼働水準を満たしています。

