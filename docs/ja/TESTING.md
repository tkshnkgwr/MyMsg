# MyMsg テスト仕様書 (TESTING)

本書は、`MyMsg` における自動テスト（単体テスト）の設計、CLI引数テスト、およびGUI挙動の手動検証チェックリストを定義します。

---

## 1. 自動単体テスト仕様

`src/main.rs` 内の `mod tests` に実装されているテストケース一覧です。

| テスト関数名 | テスト対象 | 検証項目 | 期待結果 |
| :--- | :--- | :--- | :--- |
| `test_resolve_message_priority` | `resolve_message` | 位置引数、オプション引数、未指定時の優先順位 | 位置引数 > オプション引数 > デフォルトテキスト |
| `test_calculate_window_dimensions` | `calculate_window_dimensions` | `small`, `medium`, `large` の幅・高さ・フォントサイズ算出、カスタムフォント指定 | 正しい (w, h) と font_size が返ること |
| `test_clamp_delay_seconds` | `clamp_delay_seconds` | 遅延秒数のクランプ（0秒、30秒、3600秒、9999秒） | 0〜3600秒の範囲に丸められること |
| `test_parse_color_named_and_typo` | `parse_color` | 色名（red/green）、タイポ（bule）、1文字略称（r/g/b/y/w/k） | 正確な `Color32` RGB値が返ること |
| `test_parse_color_hex` | `parse_color` | 6桁HEX、3桁HEX、8桁RGBA、不正値 | 正確な `Color32` 値または `None` が返ること |

### テスト実行コマンド
```bash
cargo test
```

---

## 2. 手動検証チェックリスト

### 2.1 CLI 引数検証
- [ ] `MyMsg.exe --help` でヘルプがコンソールに出力され、即時終了すること。
- [ ] `MyMsg.exe --version` でバージョンが出力されること。
- [ ] 不正な引数（例: `MyMsg.exe --unknown`）でエラーメッセージとUsageが表示されること。

### 2.2 GUI 描画 & キー操作検証
- [ ] `MyMsg.exe "テスト通知"` でウィンドウが画面中央最前面に表示されること。
- [ ] `Esc` キーを押すと即座にウィンドウが閉じること。
- [ ] `Enter` キーを押すと即座にウィンドウが閉じること。
- [ ] 下部ボタン `[✕ 閉じる (Esc / Enter)]` クリックで閉じること。
- [ ] 日本語文字（例: 「漢字 ひらがな カタカナ 記号！」）が文字化けせず綺麗に描画されること。

### 2.3 オプション機能検証
- [ ] `-s small`, `-s large` でウィンドウサイズと文字サイズが変化すること。
- [ ] `-c red`, `-c "#00FFCC"` で指定通りの文字色で描画されること。
- [ ] `-b` で文字が約0.5秒周期で明滅すること。
- [ ] `-d 3` で3秒間スリープした後にウィンドウが表示されること。
