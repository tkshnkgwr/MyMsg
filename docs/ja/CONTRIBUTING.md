# MyMsg 開発参加ガイド (CONTRIBUTING)

`MyMsg` プロジェクトへのご協力をご検討いただき、誠にありがとうございます！
本書では、Issue 報告、機能提案、プルリクエスト (PR) の作成ルール、ブランチ戦略、およびコミット規約について説明します。

---

## 1. 開発への参加方法

### 1.1 バグ報告 (Bug Reports)
- 事前に既存の Issue を検索し、重複がないか確認してください。
- OS 環境、実行したコマンド、期待した挙動、実際の挙動を明記してください。

### 1.2 機能提案 (Feature Requests)
- `MyMsg` の基本設計方針（「軽量・低負荷・単一バイナリ」）に合致する提案を歓迎します。

---

## 2. ブランチ戦略 & 開発フロー

1. 本リポジトリを Fork し、最新の `main` ブランチから作業ブランチを作成します。
   ```bash
   git checkout -b feature/your-feature-name
   # または
   git checkout -b fix/your-bug-fix
   ```
2. コードの変更を実施し、関連する単体テストを追加または更新します。
3. リントおよびテストがすべて通過することを確認します。
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
4. コミットを作成し、Fork リポジトリに Push した後、`main` ブランチ宛に Pull Request を作成してください。

---

## 3. コミットメッセージ規約

[Conventional Commits](https://www.conventionalcommits.org/ja/v1.0.0/) に準拠したメッセージ形式を推奨します：

- `feat: ...` : 新機能の追加
- `fix: ...` : バグ修正
- `docs: ...` : ドキュメントのみの変更
- `refactor: ...` : 仕様変更を伴わないコード改善
- `test: ...` : テストコードの追加・修正
- `chore: ...` : ビルド設定や補助ツールの更新
