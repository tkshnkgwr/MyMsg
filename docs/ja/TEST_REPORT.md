# MyMsg テスト実行報告書 (TEST_REPORT)

本書は、`MyMsg` における単体テストおよび動作確認の実行結果を記録したエビデンス報告書です。

---

## 1. 実行環境情報

| 項目                 | 値                                   |
| :------------------- | :----------------------------------- |
| **OS**               | Windows 11 (x86_64)                  |
| **Rust バージョン**  | 1.80+ / stable-x86_64-pc-windows-msvc|
| **Cargo バージョン** | cargo 1.80+                          |
| **対象バージョン**   | v1.3.0+                              |
| **実行日時**         | 2026-10-02                           |

---

## 2. 単体テスト実行結果

```
running 20 tests
test app::tests::test_has_next_schedule ... ok
test cli::layout::tests::test_calculate_window_dimensions ... ok
test cli::types::tests::test_parse_icon ... ok
test cli::layout::tests::test_resolve_message_priority ... ok
test cli::time::tests::test_calculate_at_wait ... ok
test cli::time::tests::test_clamp_delay_seconds ... ok
test cli::time::tests::test_parse_delay_with_reference ... ok
test cli::time::tests::test_parse_at_time ... ok
test cli::layout::tests::test_resolve_message_newlines ... ok
test cli::time::tests::test_parse_duration_and_interval ... ok
test cli::types::tests::test_parse_monitor_target ... ok
test cli::args::tests::test_cli_args_parsing_recurring ... ok
test cli::args::tests::test_cli_args_parsing_new_features ... ok
test cli::types::tests::test_parse_theme ... ok
test color::tests::test_parse_color_named_and_typo ... ok
test color::tests::test_resolve_theme_palette ... ok
test session::tests::test_is_interactive_session_callable ... ok
test sound::tests::test_play_sound_does_not_panic ... ok
test color::tests::test_parse_color_hex ... ok
test log::tests::test_append_log_flow ... ok

test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

**結果**: 20件中 20件合格 (Pass 100%)

---

## 3. CLI 機能手動検証結果

| 検証項目                     | 実行コマンド / 操作                   | 検証結果                                                      | 判定    |
| :--------------------------- | :------------------------------------ | :------------------------------------------------------------ | :-----: |
| **詳細ヘルプ出力**           | `MyMsg.exe --help`                    | `--interval`, `--at` などの解説を含む全オプションが出力される | ✅ PASS |
| **短縮ヘルプ出力**           | `MyMsg.exe -h`                        | コンパクトな概要ヘルプが出力される                            | ✅ PASS |
| **バージョン出力**           | `MyMsg.exe --version`                 | クレートバージョンがコンソールに出力される                    | ✅ PASS |
| **`--at` 複数時刻拒絶**      | `MyMsg.exe --at 09:00,12:00`          | エラーメッセージが出力され終了コード 1 で終了                 | ✅ PASS |
| **定期実行・次回待機**       | `--interval 10s` 表示中に `Enter` 押下| 今回通知を閉じて10秒後の次回待機へ移行                        | ✅ PASS |
| **定期実行・中断終了**       | `--interval 10s` 表示中に `Esc` 押下  | 即座に定期実行を中止してプロセス完全終了                      | ✅ PASS |
| **日本語フォント描画**       | `MyMsg.exe "日本語通知テスト"`        | 文字化けせず游ゴシック/メイリオで鮮明に表示                   | ✅ PASS |
| **自動消去タイマー**         | `MyMsg.exe "3秒消去" --timeout 3`     | 3秒経過後に自動でウィンドウが閉じて終了                       | ✅ PASS |
| **OSトースト通知**           | `MyMsg.exe "トースト" --toast -i ok`  | GUI非生成で画面右下にOSトースト通知がポップアップ             | ✅ PASS |
| **時刻指定遅延通知**         | `MyMsg.exe "お昼" -d 12:00`           | 現在時刻からの差分秒数を自動計算してゼロ負荷待機後に表示      | ✅ PASS |
| **点滅エフェクト**           | `MyMsg.exe "警告" -b`                 | 0.5秒周期で正確にアルファ明滅アニメーション                   | ✅ PASS |
| **クリップボードコピー**     | `MyMsg.exe "COPY_ME" --copy`          | ワンクリックでクリップボードにコピー成功                      | ✅ PASS |

---

## 4. 総括
すべての単体テスト（全20件）および手動検証において期待通りの動作が確認され、安定稼働水準を満たしています。


