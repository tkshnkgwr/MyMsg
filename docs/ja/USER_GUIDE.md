# MyMsg ユーザーマニュアル (USER_GUIDE)

本書は、`MyMsg` の実践的な使用方法、逆引きコマンド集、および各種シェル（PowerShell、Bash、バッチファイル）との連携実例を解説します。

---

## 1. 基本操作

### 1.1 最もシンプルな起動
```powershell
MyMsg "ビルドが正常に完了しました！"
```
画面中央最前面に通知ウィンドウが表示されます。

### 1.2 ウィンドウの閉じ方
以下のいずれかの操作で直ちに閉じることができます：
- **`Esc` キーを押す** (最も素早い操作)
- **`Enter` キーを押す**
- ウィンドウ下部の **`[✕ 閉じる (Esc / Enter)]`** ボタンをクリックする
- ウィンドウ右上の **`✕`** ボタンをクリックする

---

## 2. 逆引きコマンド集

### Q. 目立つ大きな文字とウィンドウでエラーを通知したい
```powershell
MyMsg "致命的なエラーが発生しました！" -s large -c red
```

### Q. コンパクトなウィンドウで控えめに通知したい
```powershell
MyMsg "保存完了" -s small -c green
```

### Q. 文字を点滅させて緊急事態を知らせたい
```powershell
MyMsg "サーバーからの応答が途絶しました！" -c yellow -b
```

### Q. お気に入りのテーマカラー（HEX指定）で表示したい
```powershell
MyMsg "処理が完了しました" -c "#00FFCC" --bg-color "#111827"
```

### Q. 等幅（Monospace）フォントでコードやパスを通知したい
```powershell
MyMsg "ログ出力先: C:\Logs\app.log" -f mono
```

### Q. 10分後（600秒後）にリマインダーを表示したい
```powershell
MyMsg "10分経過しました。休憩してください。" -d 600 -c gold
```

---

## 3. スクリプト連携レシピ

### 3.1 PowerShell での活用

#### 長時間コマンド終了通知
```powershell
cargo build --release; MyMsg "Cargo ビルド完了！" -c cyan
```

#### 条件分岐による結果通知
```powershell
npm test
if ($LASTEXITCODE -eq 0) {
    MyMsg "テスト全件合格！" -c green
} else {
    MyMsg "テスト失敗！ログを確認してください" -c red -b
}
```

### 3.2 Windows バッチファイル (.bat) での活用
```bat
@echo off
echo バックアップ処理を開始します...
robocopy C:\Data D:\Backup /MIR
if %ERRORLEVEL% LEQ 1 (
    MyMsg.exe "バックアップが完了しました" -c green
) else (
    MyMsg.exe "バックアップでエラーが発生しました" -c red -b
)
```

### 3.3 Linux / macOS (Bash / Zsh) での活用
```bash
./long_running_task.sh && \
./MyMsg "Task completed successfully!" -c cyan || \
./MyMsg "Task failed with error!" -c red -b
```
