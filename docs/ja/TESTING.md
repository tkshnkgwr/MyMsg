# MyMsg テスト仕様書 (TESTING)

本書は、`MyMsg` における自動テスト（単体テスト）の設計、CLI引数テスト、およびGUI挙動の手動検証チェックリストを定義します。

---

## 1. 自動単体テスト仕様

`src/cli.rs` および `src/color.rs` 内の `mod tests` に実装されているテストケース一覧です（計11件）。

| テスト関数名 | テスト対象 | 検証項目 | 期待結果 |
| :--- | :--- | :--- | :--- |
| `test_resolve_message_priority` | `resolve_message` | 位置引数、オプション引数、未指定時の優先順位 | 位置引数 > オプション引数 > デフォルトテキスト |
| `test_resolve_message_newlines` | `resolve_message` | `\n`, `\r\n` エスケープ文字の展開 | 実際の改行文字に展開されること |
| `test_calculate_window_dimensions` | `calculate_window_dimensions` | `small`, `medium`, `large` の幅・高さ・フォントサイズ算出、カスタムフォント指定 | 正しい (w, h) と font_size が返ること |
| `test_clamp_delay_seconds` | `clamp_delay_seconds` | 遅延秒数のクランプ（0秒、30秒、86400秒、99999秒） | 0〜86400秒（24時間）の範囲に丸められること |
| `test_parse_delay_with_reference` | `parse_delay_with_reference` | 秒数、単位（s, m, h, 秒, 分, 時間）、当日時刻（11:00）、秒付き（10:50:30）、翌日繰り越し時刻（10:00） | 正確な差分待機秒数が算出されること |
| `test_parse_monitor_target` | `parse_monitor_target` | `cursor`, `primary`, `0`, `1`, `2` 等のモニター指定 | 正しい `MonitorTarget` 列挙型が返ること |
| `test_parse_icon` | `parse_icon` | `info`, `warn`, `error`, `ok` および短縮・エイリアス | 正しい `IconType` が返ること |
| `test_parse_theme` | `parse_theme` | `dark`, `light`, `system` および略称 | 正しい `ThemeMode` が返ること |
| `test_parse_color_named_and_typo` | `parse_color` | 色名（red/green）、和名（赤/青）、タイポ（bule）、1文字略称（r/g/b/y/w/k） | 正確な `Color32` RGB値が返ること |
| `test_parse_color_hex` | `parse_color` | 6桁HEX、3桁HEX、8桁RGBA、不正値 | 正確な `Color32` 値または `None` が返ること |
| `test_resolve_theme_palette` | `resolve_theme_palette` | システムテーマ判定とカスタム色指定の優先解決 | 正しい `ThemePalette` が構成されること |

### テスト実行コマンド
```bash
cargo test
```

---

## 2. 手動検証チェックリスト

### 2.1 CLI 引数 & ヘルプ検証
- [ ] `MyMsg.exe --help` で詳細なヘルプが出力され、`-d` の時刻・単位例が明記されていること。
- [ ] `MyMsg.exe -h` でコンパクトな短縮ヘルプが出力されること。
- [ ] `MyMsg.exe --version` でバージョンが出力されること。
- [ ] 不正な引数でエラーメッセージとUsageが表示されること。

### 2.2 GUI 描画 & キー操作検証
- [ ] `MyMsg.exe "テスト通知"` でウィンドウが画面中央最前面に表示されること。
- [ ] `Esc` / `Enter` キーを押すと即座にウィンドウが閉じること。
- [ ] 下部ボタン `[✕ 閉じる (Esc / Enter)]` クリックで閉じること。
- [ ] 日本語文字が文字化けせず綺麗に描画されること。

### 2.3 オプション機能検証
- [ ] `--monitor primary` でマウス位置に関係なくプライマリ画面中央に表示されること。
- [ ] `--timeout 3` で3秒後に自動でウィンドウが閉じること。
- [ ] `--toast` でGUIを立ち上げずに画面右下にOSトースト通知が表示され即時終了すること。
- [ ] `-d 10m` や `-d 12:00` で指定時間/時刻まで待機した後に通知されること。
- [ ] `-b` で文字が約0.5秒周期で明滅すること。
