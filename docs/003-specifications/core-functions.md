# GHWT コア機能（get/new）実装仕様書

**Date**: 2025-06-01  
Version 1.0 — 2025-06-01

## 概要

本ドキュメントは GHWT (Git Worktree Tool) のコア機能である `ghwt get` と `ghwt new` コマンドの詳細実装仕様書である。
Nick Nisi式レイアウトの自動構築手順を明確化し、リポジトリ取得とWorktree作成の基盤を確立する。

## コマンド一覧

### MVPコマンド

| コマンド | 概要 | 主な用途 |
|----------|------|----------|
| `ghwt get <url>` | Git リポジトリを取得してGHWT形式に変換 | 新しいリポジトリの初期セットアップ |
| `ghwt new <branch>` | 新しいWorktreeを作成 | ブランチごとの並行開発環境構築 |

### 基本的なワークフロー

```bash
# 1. リポジトリを取得してGHWT形式に変換
ghwt get git@github.com:myorg/myapp.git

# 2. リポジトリディレクトリに移動
cd ~/ghwt/myapp

# 3. 新機能用のWorktreeを作成
cd "$(ghwt new feature-auth)"

# 4. 別のブランチでも並行作業
cd ~/ghwt/myapp
cd "$(ghwt new hotfix-123)"
```

## 関連ドキュメント

- [../002-requirements/requirements-ja.md](../002-requirements/requirements-ja.md) - 基本要件
- [cli-interface.md](cli-interface.md) - CLI インターフェース仕様
- [filesystem-layout.md](filesystem-layout.md) - ファイルシステム規約
- [error-handling.md](error-handling.md) - エラーハンドリング
- [data-structures.md](data-structures.md) - 内部データ構造

## 変更履歴

| 日付 | バージョン | 変更内容 |
|------|------------|----------|
| 2025-06-01 | 1.0 | 初版作成 |

---

## 1. `ghwt get` コマンド仕様

### 1.1 概要

`ghwt get` コマンドは、Git リポジトリを取得してGHWT形式のディレクトリレイアウトに変換するコマンドです。
指定されたURLからリポジトリをbare cloneし、Nick Nisi式の「1 Repository = 1 Directory」原則に基づいた構造を自動構築します。

**主な機能:**
- Git URLからのリポジトリ取得
- bare repository + gitfile 構造の自動構築
- 初期Worktreeの作成（オプション）
- 既存リポジトリとの競合回避

### 1.2 基本構文

```bash
ghwt get <url> [options]
```

### 1.3 実装手順

#### 1.3.1 URL解析とリポジトリ名抽出

**リポジトリ名抽出ルール:**
| 入力 URL | 抽出されるリポジトリ名 |
|----------|----------------------|
| `git@github.com:user/repo.git` | `repo` |
| `https://github.com/user/repo.git` | `repo` |
| `https://github.com/user/repo` | `repo` |
| `git@gitlab.com:group/subgroup/project.git` | `project` |
| `file:///path/to/repo.git` | `repo` |

**正規化アルゴリズム:**
1. URL の最後のパス要素を取得
2. .git 拡張子を除去
3. ファイルシステム安全な文字のみ保持 ([a-zA-Z0-9_-])
4. 先頭・末尾の特殊文字を除去

**重複チェック（既存リポジトリとの競合回避）:**
- `~/ghwt/<repo_name>` の存在確認
- `--force` オプション未指定時はエラー終了
- `--force` オプション指定時は既存ディレクトリを削除

#### 1.3.2 bare repository 作成

**安全な作成プロセス:**
1. 一時ディレクトリでの構築（`~/ghwt/.tmp/<repo>-<timestamp>/`）
2. bare clone の実行
3. gitfile の作成
4. `.wt/` ディレクトリの初期化
5. 最終位置への移動
6. 失敗時の自動クリーンアップ

**Git clone コマンド実行:**
```bash
git clone --bare <url> <repo_path>/.bare
```

**実行時オプション:**
- `--depth <n>`: shallow clone の深度指定
- `--single-branch`: 単一ブランチのみクローン
- `--filter=blob:none`: blobless clone（大容量リポジトリ対応）

#### 1.3.3 gitfile 作成

**gitfile 仕様:**
- ファイル名: `.git`
- 内容: `gitdir: ./.bare` (改行なし)
- 権限: 644

#### 1.3.4 ディレクトリ構造初期化

**`.wt/` ディレクトリの作成:**
- パス: `<repo_path>/.wt/`
- 権限: 755
- 目的: 全worktreeの格納

#### 1.3.5 リモート設定の調整

**fetch refspec の設定:**
```bash
cd <repo_path>
git config remote.origin.fetch "+refs/heads/*:refs/remotes/origin/*"
```

**デフォルトブランチの検出:**
1. remote HEAD の確認（`git symbolic-ref refs/remotes/origin/HEAD`）
2. フォールバック: main, master, develop の順で確認

### 1.4 オプション仕様

| オプション | 短縮形 | 型 | デフォルト | 説明 |
|-----------|--------|----|---------|----|
| `--name` | `-n` | string | URL から抽出 | カスタムリポジトリ名 |
| `--branch` | `-b` | string | デフォルトブランチ | 初期Worktreeブランチ指定 |
| `--shallow` | - | boolean | false | shallow clone の実行 |
| `--depth` | `-d` | integer | - | shallow clone の深度 |
| `--no-checkout` | - | boolean | false | 初期チェックアウトをスキップ |
| `--force` | `-f` | boolean | false | 既存ディレクトリの上書き |

### 1.5 初期Worktree作成（オプション）

**`--no-checkout` が指定されていない場合:**
```bash
cd <repo_path>
git worktree add .wt/<normalized_branch> <branch>
```

---

## 2. `ghwt new` コマンド仕様

### 2.1 概要

`ghwt new` コマンドは、GHWT管理下リポジトリ内で新しいWorktreeを作成するコマンドです。
カレントディレクトリからリポジトリを自動判定し、指定されたブランチに対応するWorktreeを `.wt/` ディレクトリ内に作成します。

**主な機能:**
- カレントディレクトリからのリポジトリ自動判定
- 新しいWorktreeの作成
- ローカル・リモート・新規ブランチの自動判定
- ブランチ追跡設定の自動化
- 既存Worktreeとの競合回避

**使用前提:**
- GHWT管理下のリポジトリディレクトリまたはWorktree内で実行

### 2.2 基本構文

```bash
ghwt new <branch> [options]
```

### 2.3 実装手順

#### 2.3.1 リポジトリ解決

**カレントディレクトリからのリポジトリ自動判定:**
1. GHWT管理下のworktreeかチェック（`.git`ファイルの`gitdir`を確認）
2. GHWT管理下のリポジトリルートかチェック（`.bare`ディレクトリの存在確認）
3. 見つからない場合はエラー

**判定失敗時のエラーメッセージ:**
```
Error: Not in a GHWT repository
Hint: Navigate to a GHWT repository directory first:
  cd ~/ghwt/myrepo
  ghwt new <branch>
```

#### 2.3.2 ブランチ解決

**ブランチ存在確認と種別判定:**
- **Local**: ローカルブランチが存在
- **Remote**: リモートブランチが存在
- **New**: 新規ブランチとして作成

**確認手順:**
1. ローカルブランチの確認（`git show-ref --verify refs/heads/<branch>`）
2. リモートブランチの確認（`git branch -r --list "*/<branch>"`）
3. 新規ブランチとして扱う

#### 2.3.3 Worktree作成

**ユーザーのコマンドと内部処理の対応:**

| ブランチの状態 | ユーザーの操作 | 内部で実行されるgitコマンド |
|---------------|---------------|---------------------------|
| **ローカルブランチが存在** | `ghwt new main`<br/>（GHWT管理下ディレクトリで実行） | `git worktree add .wt/main main` |
| **リモートブランチが存在** | `ghwt new feature-auth` | `git worktree add --track -b feature-auth .wt/feature-auth origin/feature-auth` |
| **新規ブランチを作成** | `ghwt new new-feature` | `git worktree add -b new-feature .wt/new-feature` |

**処理の流れ:**
1. カレントディレクトリからリポジトリを自動判定
2. ブランチの存在確認（ローカル → リモート → 新規の順）
3. 判定結果に応じて適切な`git worktree add`コマンドを実行
4. 作成されたWorktreeのパスを出力

これにより、ユーザーはリポジトリ名を指定する必要がなく、シンプルな`ghwt new <branch>`コマンドだけで適切なWorktreeが作成されます。

#### 2.3.4 パス出力

**重要: 最終行に作成されたパスを出力**
- 標準出力の最終行に絶対パスを出力
- 形式: `/absolute/path/to/worktree`

### 2.4 オプション仕様

| オプション | 短縮形 | 型 | デフォルト | 説明 |
|-----------|--------|----|---------|----|
| `--force` | `-f` | boolean | false | 既存 worktree の上書き |

---

## 3. エラーハンドリング

### 3.1 `ghwt get` のエラーパターン

#### 3.1.1 ネットワークエラー
- 接続失敗
- 認証失敗
- ホスト名解決失敗
- タイムアウト
- SSL/TLS エラー

**エラーメッセージ例:**
```
Error: Failed to connect to remote repository
Details: Connection timed out after 30 seconds
Hint: Check your network connection and try again
```

#### 3.1.2 既存リポジトリとの競合
```
Error: Repository already exists at ~/ghwt/myrepo
Hint: Use --force to overwrite or --name to specify a different name
```

#### 3.1.3 ディスク容量不足
```
Error: Insufficient disk space
Details: Required: 1.2GB, Available: 800MB
Hint: Free up disk space and try again
```

#### 3.1.4 権限エラー
```
Error: Permission denied
Details: Cannot write to ~/ghwt/myrepo
Hint: Check directory permissions or run with appropriate privileges
```

### 3.2 `ghwt new` のエラーパターン

#### 3.2.1 リポジトリが見つからない
```
Error: Repository not found
Details: No GHWT repository found in current directory or specified path
Hint: Run 'ghwt get <url>' to clone a repository first
```

#### 3.2.2 ブランチが存在しない
```
Error: Branch 'feature/new-feature' does not exist
Details: No local or remote branch found with this name
Hint: Create the branch first or check the branch name spelling
```

#### 3.2.3 既存Worktreeとの競合
```
Error: Worktree already exists at ~/ghwt/myrepo/.wt/feature-new-feature
Hint: Use --force to overwrite or remove the existing worktree first
```

#### 3.2.4 Git操作の失敗
```
Error: Failed to create worktree
Details: git worktree add failed with exit code 1
Caused by: fatal: 'feature/new-feature' is already checked out at '~/ghwt/myrepo/.wt/feature-new-feature'
Hint: The branch is already in use by another worktree
```

---

## 完了条件チェックリスト

- [x] `docs/003-specifications/core-functions.md` ファイルが作成されている
- [x] `ghwt get` の完全な実装手順が定義されている
- [x] `ghwt new` の完全な実装手順が定義されている
- [x] 各コマンドの概要が明確に記述されている
- [x] 全エラーパターンの処理方法が明記されている

---

*本仕様書は GHWT プロジェクトのコア機能実装の基盤仕様として、開発チームの実装作業を支援します。*