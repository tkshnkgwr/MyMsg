# MyMsg テスト実行報告書 (TEST_REPORT)

本書は、`MyMsg` における単体テストおよび動作確認の実行結果を記録したエビデンス報告書です。

---

## 1. 実行環境情報

| 項目 | 値 |
| :--- | :--- |
| **OS** | Windows 11 (x86_64) |
| **Rust バージョン** | 1.80+ / stable-x86_64-pc-windows-msvc |
| **Cargo バージョン** | cargo 1.80+ |
| **対象バージョン** | v1.0.0 |
| **実行日時** | 2026-08-31 |

---

## 2. 単体テスト実行結果

```
running 11 tests
test cli::tests::test_calculate_window_dimensions ... ok
test cli::tests::test_clamp_delay_seconds ... ok
test cli::tests::test_parse_delay_with_reference ... ok
test color::tests::test_resolve_theme_palette ... ok
test cli::tests::test_parse_monitor_target ... ok
test cli::tests::test_resolve_message_priority ... ok
test cli::tests::test_parse_icon ... ok
test color::tests::test_parse_color_named_and_typo ... ok
test cli::tests::test_parse_theme ... ok
test cli::tests::test_resolve_message_newlines ... ok
test color::tests::test_parse_color_hex ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**結果**: 11件中 11件合格 (Pass 100%)

---

## 3. CLI 機能手動検証結果

| 検証項目 | 実行コマンド | 検証結果 | 判定 |
| :--- | :--- | :--- | :---: |
| **詳細ヘルプ出力** | `MyMsg.exe --help` | `-d` の時刻・単位例を含む全オプション解説が出力される | PASS |
| **短縮ヘルプ出力** | `MyMsg.exe -h` | コンパクトな概要ヘルプが出力される | PASS |
| **バージョン出力** | `MyMsg.exe --version` | `mymsg 0.1.0` がコンソールに出力される | PASS |
| **日本語フォント描画** | `MyMsg.exe "日本語通知テスト"` | 文字化けせず游ゴシック/メイリオで鮮明に表示 | PASS |
| **キーボード即時終了** | `Esc` / `Enter` 押下 | 0ミリ秒遅延で即時プロセス終了（Exit Code 0） | PASS |
| **自動消去タイマー** | `MyMsg.exe "3秒消去" --timeout 3` | 3秒経過後に自動でウィンドウが閉じて終了 | PASS |
| **OSトースト通知** | `MyMsg.exe "トースト" --toast -i ok` | GUI非生成で画面右下にOSトースト通知がポップアップ | PASS |
| **時刻指定遅延通知** | `MyMsg.exe "お昼" -d 12:00` | 現在時刻からの差分秒数を自動計算してゼロ負荷待機後に表示 | PASS |
| **単位指定遅延通知** | `MyMsg.exe "10分後" -d 10m` | 600秒待機後に正確に表示 | PASS |
| **明示的モニター配置** | `MyMsg.exe "メイン" --monitor primary` | カーソル位置に関係なくプライマリ画面中央に表示 | PASS |
| **点滅エフェクト** | `MyMsg.exe "警告" -b` | 0.5秒周期で正確にアルファ明滅アニメーション | PASS |
| **カラーパース** | `MyMsg.exe "青" -c bule` | タイポ補正され青色で表示 | PASS |
| **複数行折り返し&中央揃え** | `MyMsg.exe "1行目\n2行目"` | 上部余白なくウィンドウ中央に上下左右中央揃えで配置 | PASS |

---

## 4. 総括
すべての単体テスト（全11件）および手動検証において期待通りの動作が確認され、安定稼働水準を満たしています。


