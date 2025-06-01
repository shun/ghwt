# GHWT - Git Worktree 管理ツール

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**GHWT**（Git Worktree Tool）は、Git の **bare repository + worktree** レイアウトを自動化し、安全かつ高速に複数のブランチを同時に扱えるCLIツールです。

## 🎯 なぜ GHWT が必要なのか？

現代のソフトウェア開発では、フィーチャー開発、バグ修正、コードレビュー、ホットフィックス対応など、複数のブランチを同時に扱う必要性が急激に増加しています。

### 従来の `git checkout` の問題点

- **コンテキストスイッチのコスト**: ブランチ切り替えのたびに依存関係の再インストール（`npm install` など）が必要
- **作業状態の喪失**: 未コミットの変更を一時的に退避する必要性
- **IDE の混乱**: プロジェクト設定やインデックスの再構築による待機時間
- **並行作業の困難**: 複数のブランチで同時にビルドやテストを実行できない

### 標準的な `git worktree` の問題点

```bash
# 標準的な使用方法
git worktree add ../feature-branch origin/main
git worktree add ../hotfix-123 origin/main
git worktree add ../review-pr-456 origin/main
```

この方法では、リポジトリ周辺に多数のディレクトリが散乱し、以下の問題が発生します：

```
~/projects/
├── myapp/                    # メインリポジトリ
├── myapp-feature-auth/       # 認証機能ブランチ
├── myapp-feature-payment/    # 決済機能ブランチ
├── myapp-hotfix-security/    # セキュリティ修正
├── myapp-review-pr-123/      # PR レビュー用
├── myapp-review-pr-124/      # PR レビュー用
└── ...                       # さらに増え続ける
```

- **視認性の悪化**: どのディレクトリがどのリポジトリに属するかが不明確
- **誤操作のリスク**: 似た名前のディレクトリ間での混乱
- **残骸の蓄積**: 削除し忘れた古い worktree が永続的に残存

## 🚀 GHWT のソリューション

GHWT は [Nick Nisi 式アプローチ](https://nicknisi.com/posts/git-worktrees/) を採用し、**1 リポジトリ = 1 ディレクトリ** の原則を維持します。

### 理想的なディレクトリ構造

```
~/ghwt/myapp/
├── .bare/          # bare repository（Git データベース）
├── .git            # gitfile（.bare への参照）
└── .wt/            # worktree 専用ディレクトリ
    ├── main/       # メインブランチ
    ├── feature-auth/
    ├── feature-payment/
    ├── hotfix-security/
    └── review-pr-123/
```

この構造により、視認性と安全性を大幅に向上させます。

## 📦 インストール

### Homebrew（推奨）

```bash
brew install ghwt
```

### Cargo

```bash
cargo install ghwt
```

### バイナリダウンロード

[Releases ページ](https://github.com/shun/ghwt/releases) から最新のバイナリをダウンロードしてください。

## 🔧 必須要件

- **OS**: macOS / Linux / WSL
- **Git**: バージョン 2.38 以上
- **シェル**: bash / zsh
- **推奨**: ghq ≥ 1.4（任意）

## 🚀 クイックスタート

### 1. リポジトリの取得

```bash
# 新しいリポジトリを bare clone + レイアウト化
ghwt get git@github.com:myorg/myapp.git
```

### 2. Worktree の作成と移動

```bash
# リポジトリディレクトリに移動
cd ~/ghwt/myapp

# 新しいブランチ用 worktree を作成してすぐ移動
cd "$(ghwt new feature-auth)"
```

## 📋 主要コマンド（MVP）

| コマンド | 説明 |
|----------|------|
| `ghwt get <URL>` | リポジトリを bare clone してレイアウト化 |
| `ghwt new <branch>` | 新しい worktree を作成 |

## 🚨 トラブルシューティング

### よくある問題

#### Q: `ghwt new` でエラーが発生する

```bash
# Git の状態を確認
git worktree prune
```

#### Q: リポジトリが見つからない

```bash
# GHWT管理下のディレクトリにいるか確認
pwd
ls -la  # .bare ディレクトリがあるか確認

# リポジトリディレクトリに移動してから実行
cd ~/ghwt/myrepo
ghwt new feature-branch
```

## 🎯 使用例

### 日常的なワークフロー

```bash
# 1. 新機能の開発開始
ghwt get git@github.com:myorg/myapp.git
cd ~/ghwt/myapp
cd "$(ghwt new feature/user-authentication)"

# 2. 緊急バグ修正（別ターミナル）
cd ~/ghwt/myapp
cd "$(ghwt new hotfix/security-patch)"

# 3. PR レビュー（さらに別ターミナル）
cd ~/ghwt/myapp
cd "$(ghwt new review/pr-123)"
```

### チーム開発での活用

```bash
# 複数のリポジトリを管理
ghwt get git@github.com:myorg/frontend.git
ghwt get git@github.com:myorg/backend.git

# 関連する機能を並行開発
cd ~/ghwt/frontend && cd "$(ghwt new feature/new-ui)"
cd ~/ghwt/backend && cd "$(ghwt new feature/new-api)"
```

## 🤝 コントリビューション

プロジェクトへの貢献を歓迎します！

1. このリポジトリをフォーク
2. フィーチャーブランチを作成（`ghwt-newcd ghwt feature/amazing-feature`）
3. 変更をコミット（`git commit -m 'Add amazing feature'`）
4. ブランチにプッシュ（`git push origin feature/amazing-feature`）
5. プルリクエストを作成

## 📄 ライセンス

このプロジェクトは [MIT License](LICENSE) の下で公開されています。

## 🙏 謝辞

- [Nick Nisi](https://nicknisi.com/) - bare repository + worktree アプローチのインスピレーション
- [ghq](https://github.com/x-motemen/ghq) - リポジトリ管理のベストプラクティス

## 📚 関連リンク

- [要件定義書](docs/002-requirements/requirements-ja.md)
- [アーキテクチャ決定記録](docs/001-adr/)
- [仕様書](docs/003-specifications/)
- [Git Worktree 公式ドキュメント](https://git-scm.com/docs/git-worktree)
- [Nick Nisi の Git Worktree 記事](https://nicknisi.com/posts/git-worktrees/)
