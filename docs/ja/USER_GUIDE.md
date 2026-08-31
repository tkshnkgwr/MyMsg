# MyMsg ユーザーマニュアル (USER_GUIDE)

本書は、`MyMsg` の実践的な使用方法、逆引きコマンド集、および各種シェル（PowerShell、Bash、バッチファイル）との連携実例を解説します。

---

## 1. 基本操作

### 1.1 最もシンプルな起動
```powershell
MyMsg "ビルドが正常に完了しました！"
```
現在マウスカーソルが存在するモニター（操作中のアクティブ画面）の中央最前面に通知ウィンドウが表示されます。
マルチディスプレイ環境でも、作業中の画面に確実にポップアップします。

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

### Q. アイコン付きでエラーや警告をわかりやすく通知したい
```powershell
# 警告アイコン
MyMsg "ディスク使用量が90%を超えています！" -i warn

# エラーアイコン + 点滅
MyMsg "データベース接続に失敗しました" -i error -b

# 完了・成功アイコン
MyMsg "すべてのデプロイが完了しました" -i ok
```

### Q. 改行を含めた複数行メッセージを表示したい
```powershell
MyMsg "【処理完了】\n成功: 120件\nスキップ: 3件\n失敗: 0件" -i ok
```

### Q. ライトモード（白基調）でスッキリ表示したい
```powershell
MyMsg "定例ミーティングの時間です" -t light -i info
```

### Q. 10分後や指定時刻（12:00 / 17:30）にリマインダーを表示したい
```powershell
# 10分後に通知（秒数、単位指定、時刻指定すべて可能）
MyMsg "10分経過しました。休憩してください。" -d 10m -c gold -i info

# 12:00（お昼）に通知
MyMsg "お昼休憩の時間です！" -d 12:00 -i ok

# 夕方18:00に右下トースト通知
MyMsg "定時です。業務を終了しましょう。" -d 18:00 --toast -i info
```

### Q. マウス位置に関係なく、メイン画面（プライマリモニター）の中央に確実に出したい
```powershell
MyMsg "メインモニター通知" --monitor primary -i ok
```

### Q. 5秒後に自動で閉じるようにしたい（チラ見せ通知）
```powershell
MyMsg "処理が完了しました（5秒後に自動消去）" --timeout 5 -i ok
```

### Q. GUIウィンドウを出さず、OS標準のトースト通知（画面右下）で通知したい
```powershell
MyMsg "バックグラウンドタスクが完了しました" --toast -i ok
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
