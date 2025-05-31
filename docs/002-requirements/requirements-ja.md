# GHWT 要件定義書  
Version 1.0 — 2025-06-01

## 概要

本ドキュメントは GHWT プロジェクトの要件定義書である。
プロジェクトの目的、機能要件、非機能要件、制約条件などを明確に定義し、
開発・テスト・運用フェーズ全体で参照できる単一の信頼ソース (SSoT) として機能する。

## 関連ドキュメント

- [../001-adr/](../001-adr/) - アーキテクチャ決定記録
- [../003-specifications/](../003-specifications/) - 詳細仕様書群
- [../../README.md](../../README.md) - プロジェクト概要

## 変更履歴

| 日付 | バージョン | 変更内容 |
|------|------------|----------|
| 2025-06-01 | 1.0 | 仕様書フィードバック反映、機能要件・非機能要件の大幅拡充 |
| 2025-05-31 | 0.3 | ADR の分離、ディレクトリ構成の変更 |
| 2025-05-31 | 0.2 | Nick Nisi 式アプローチの詳細化、具体例の追加 |
| 2025-05-31 | 0.1 | 初版作成 |

---

## 0. 目的

| No | 目的 |
|----|------|
| 0-1 | Git の **bare repository + worktree** レイアウトを自動化し、安全かつ高速に Worktree を運用できる CLI を提供する。 |
| 0-2 | ghq ユーザー／非 ghq ユーザーの両方が同一 UX で利用可能にする。 |
| 0-3 | 本書を実装・テスト・運用フェーズ全体で参照できる **単一の信頼ソース (SSoT)** とする。 |

---

## 1. 背景

### 1.1 現状の課題

現代のソフトウェア開発では、複数のブランチを同時に扱う必要性が急激に増加している。フィーチャー開発、バグ修正、コードレビュー、ホットフィックス対応など、開発者は日常的に複数のコンテキストを切り替えながら作業を行う。

従来の `git checkout` によるブランチ切り替えでは、以下の問題が発生する：

1. **コンテキストスイッチのコスト**: ブランチ切り替えのたびに依存関係の再インストール（`npm install` など）が必要
2. **作業状態の喪失**: 未コミットの変更を一時的に退避する必要性
3. **IDE の混乱**: プロジェクト設定やインデックスの再構築による待機時間
4. **並行作業の困難**: 複数のブランチで同時にビルドやテストを実行できない

### 1.2 Git Worktree の可能性と限界

Git 2.5 で導入された `git worktree` は、これらの問題を解決する強力な機能である。しかし、標準的な使用方法では新たな問題が生じる：

**標準的な worktree 使用例:**
```bash
git worktree add ../feature-branch origin/main
git worktree add ../hotfix-123 origin/main
git worktree add ../review-pr-456 origin/main
```

この方法では、リポジトリ周辺に多数のディレクトリが散乱し、以下の問題が発生する：

### 1.3 具体的な問題例

以下は、3つの異なるリポジトリで worktree を多用した場合の典型的なディレクトリ構造である：

```
~/projects/
├── myapp/                    # メインリポジトリ
├── myapp-feature-auth/       # 認証機能ブランチ
├── myapp-feature-payment/    # 決済機能ブランチ
├── myapp-hotfix-security/    # セキュリティ修正
├── myapp-review-pr-123/      # PR レビュー用
├── myapp-review-pr-124/      # PR レビュー用
├── myapp-experimental/       # 実験的機能
├── backend-api/              # バックエンド API リポジトリ
├── backend-api-v2-migration/ # API v2 移行ブランチ
├── backend-api-performance/  # パフォーマンス改善
├── backend-api-hotfix-db/    # DB 関連修正
├── frontend-ui/              # フロントエンド UI リポジトリ
├── frontend-ui-redesign/     # UI リデザイン
├── frontend-ui-mobile/       # モバイル対応
├── frontend-ui-a11y/         # アクセシビリティ改善
└── frontend-ui-review-pr-89/ # PR レビュー用
```

この状況では以下の深刻な問題が発生する：

1. **視認性の悪化**: どのディレクトリがどのリポジトリに属するかが不明確
2. **誤操作のリスク**: 似た名前のディレクトリ間での混乱
3. **残骸の蓄積**: 削除し忘れた古い worktree が永続的に残存
4. **IDE の混乱**: プロジェクト検索時に無関係なディレクトリまで対象となる
5. **パフォーマンス劣化**: ファイル検索やインデックス作成の対象範囲が不必要に拡大

### 1.4 Nick Nisi 式アプローチの採用

この問題を解決するため、Nick Nisi 氏が提唱する「bare repository + 構造化 worktree」アプローチを採用する。

**参考資料:**
- [How I use git worktrees - Nick Nisi](https://nicknisi.com/posts/git-worktrees/)
- [Git Workshop - GitHub Repository](https://github.com/nicknisi/git-workshop)

Nick Nisi 式では、以下の構造を採用する：

```
~/projects/myapp/
├── .bare/          # bare repository（Git データベース）
├── .git            # gitfile（.bare への参照）
└── .wt/            # worktree 専用ディレクトリ
    ├── main/       # メインブランチ
    ├── feature-auth/
    ├── feature-payment/
    ├── hotfix-security/
    └── review-pr-123/
```

この構造により、**1 リポジトリ = 1 ディレクトリ** の原則を維持し、視認性と安全性を大幅に向上させる。

### 1.5 自動化の必要性

しかし、Nick Nisi 式を手動で実装するには以下の複雑な手順が必要である：

1. `git clone --bare` でリポジトリを取得
2. `.bare` ディレクトリへの移動と設定
3. `gitdir: ./.bare` を含む `.git` ファイルの作成
4. リモート追跡ブランチの設定修正
5. 各 worktree の作成と管理

これらの手順を毎回手動で実行するのは非効率的であり、設定ミスのリスクも高い。GHWT は、この複雑なプロセスを完全に自動化し、開発者が本質的な作業に集中できる環境を提供する。

---

## 2. 用語

| 用語 | 定義 |
|------|------|
| **bare repository** | `.bare/` に配置する `git clone --bare` の内容 |
| **worktree** | `.wt/<branch>/` に配置するチェックアウト済みディレクトリ |
| **gitfile** | `.git` ファイルに `gitdir: ./.bare` だけを書いたポインタ |
| **GHWT root** | Worktree 体系のルート。`$GHQ_ROOT` が無ければ **`~/ghwt`** を使用（隠しディレクトリにしない理由: GUI ファイラで視認可能に保つため） |
| **XDG Base Directory** | Unix 系 OS での設定ファイル配置標準。`$XDG_CONFIG_HOME` または `~/.config` を使用 |

---

## 3. アーキテクチャ決定記録 (ADR)

重要なアーキテクチャ決定とその根拠については、別途 **[../001-adr/](../001-adr/)** を参照。

主要な決定項目：
- [ADR-001](../001-adr/001-bare-repository.md): Bare Repository + Gitfile アーキテクチャの採用
- [ADR-002](../001-adr/002-cli-scope.md): CLI の責務範囲の限定
- [ADR-003](../001-adr/003-directory-principle.md): 1 Repository = 1 Directory 原則
- [ADR-004](../001-adr/004-rust-implementation.md): Rust 実装の選択
- [ADR-005](../001-adr/005-mit-license.md): MIT ライセンスの採用
- [ADR-006](../001-adr/006-toml-config.md): TOML 設定ファイルと XDG Base Directory Specification の採用

---

## 4. 前提条件

| 項目 | 内容 |
|------|------|
| 対応 OS | macOS / Linux / WSL (Windows PowerShell ネイティブは対象外) |
| 対応シェル | bash / zsh（fish・PowerShell は後方フェーズで対応） |
| 必須ソフト | Git ≥ 2.38 |
| 推奨ツール | ghq ≥ 1.4 （任意） |

---

## 5. 機能要件 (MVP: Minimum Viable Product)

### 5.1 基本コマンド機能

| ID | 機能 | コマンド | 詳細仕様 |
|----|------|----------|----------|
| **R1** | リポジトリ取得・変換 | `ghwt get <url>` | bare clone + Nick Nisi式レイアウト化<br>オプション: `--branch`, `--depth`, `--force`, `--no-checkout` |
| **R2** | 既存リポジトリ変換 | `ghwt migrate <path>` | 既存リポジトリをGHWT形式に変換 |
| **W1** | Worktree 生成 | `ghwt new [<repo>] <branch>` | 完了パスを **stdout 最終行** に出力<br>オプション: `--track`, `--force`, `--detach` |
| **L1** | Worktree 一覧 | `ghwt ls` | repo/branch/path/age を列挙<br>オプション: `--all`, `--repo`, `--format`, `--sort`, `--json` |
| **D1** | Worktree 削除 | `ghwt rm <repo> <branch>` | 指定Worktreeの安全な削除<br>オプション: `--force`, `--prune` |
| **D2** | Worktree プルーン | `ghwt prune` | 古いWorktreeのクリーンアップ<br>オプション: `--expire`, `--dry-run`, `--force` |

### 5.2 補助コマンド機能

| ID | 機能 | コマンド | 詳細仕様 |
|----|------|----------|----------|
| **C1** | Worktree作成+移動 | `ghwt-newcd [<repo>] <branch>` | シェルスクリプトで提供、作成後に自動cd |
| **C2** | 対話的選択 | `ghwt fzf` | fzfによるWorktree選択インターフェース |

### 5.3 設定管理機能

| ID | 機能 | コマンド | 詳細仕様 |
|----|------|----------|----------|
| **S1** | 設定値設定 | `ghwt config set <key> <value>` | TOML設定ファイルの値を設定 |
| **S2** | 設定値取得 | `ghwt config get <key>` | 指定キーの現在値を表示 |
| **S3** | 設定一覧表示 | `ghwt config list` | 全設定値を表示、`--json`オプション対応 |
| **S4** | 設定値削除 | `ghwt config unset <key>` | 設定値をデフォルトに戻す |
| **S5** | 設定検証 | `ghwt config validate` | 設定ファイルの妥当性検証 |

### 5.4 グローバルオプション

| オプション | 短縮形 | 説明 | 対応コマンド |
|-----------|--------|------|-------------|
| `--help` | `-h` | ヘルプ表示 | 全コマンド |
| `--version` | `-V` | バージョン表示 | 全コマンド |
| `--verbose` | `-v` | 詳細出力 | 全コマンド |
| `--quiet` | `-q` | 静寂モード | 全コマンド |
| `--json` | - | JSON形式出力 | `ls`, `config list` |
| `--config` | `-c` | 設定ファイルパス指定 | 全コマンド |

### 5.5 出力形式仕様

#### 5.5.1 標準出力形式
- **成功メッセージ**: 人間が読みやすい形式
- **パス出力**: `ghwt new`は最終行に作成パスを出力（スクリプト連携用）
- **テーブル形式**: `ghwt ls`のデフォルト出力

#### 5.5.2 JSON出力形式
- **構造化データ**: 機械処理用の完全な情報
- **メタデータ**: バージョン、生成日時を含む
- **一貫性**: 全コマンドで統一されたスキーマ

#### 5.5.3 エラー出力形式
- **標準エラー**: エラーメッセージは stderr に出力
- **詳細情報**: `--verbose` フラグで詳細なエラー情報
- **ヒント**: 解決方法の提案を含む

---

## 6. 非機能要件

### 6.1 性能要件

| 項目 | 指標 | 測定条件 |
|------|------|----------|
| Worktree一覧表示 | ≥ 100 worktree/秒 | ローカルファイルシステム |
| Worktree作成 | ≤ 1 秒 | ローカルブランチ、SSD環境 |
| リポジトリ取得 | ネットワーク速度に依存 | リモートクローン時 |
| 設定ファイル読み込み | ≤ 10ms | 標準的な設定ファイル |

### 6.2 配布・インストール要件

| 方法 | 対象OS | 詳細 |
|------|--------|------|
| Homebrew Formula | macOS | `brew install ghwt` |
| Cargo Install | 全対応OS | `cargo install ghwt` |
| 静的バイナリ | Linux/macOS | GitHub Releases |
| パッケージマネージャ | Linux | apt/yum/pacman対応（将来） |

### 6.3 設定管理要件

| 項目 | 仕様 | 詳細 |
|------|------|------|
| 設定ファイル形式 | TOML | 人間が編集可能 |
| 配置場所 | XDG Base Directory | `~/.config/ghwt/config.toml` |
| 環境変数対応 | あり | `GHWT_ROOT`, `GHWT_CONFIG` |
| 設定の優先順位 | 環境変数 > 設定ファイル > デフォルト | - |

### 6.4 信頼性・安全性要件

| 項目 | 仕様 | 詳細 |
|------|------|------|
| エラー回復 | 自動/手動 | `git worktree prune` 実行提案 |
| データ整合性 | 検証機能 | 設定ファイル・リポジトリ状態の検証 |
| 権限管理 | 最小権限 | 必要最小限のファイルアクセス |
| バックアップ | 設定保護 | 設定変更時の自動バックアップ |

### 6.5 国際化要件

| 項目 | 仕様 | 詳細 |
|------|------|------|
| 対応言語 | 日本語・英語 | エラーメッセージ・ヘルプ |
| ロケール検出 | 自動 | `GHWT_LANG`, `LANG`, `LC_ALL` |
| 文字エンコーディング | UTF-8 | 全テキスト処理 |

### 6.6 エラーハンドリング要件

| 項目 | 仕様 | 詳細 |
|------|------|------|
| エラーコード体系 | 16種類 | 0-15の統一コード |
| エラーメッセージ | 構造化 | メッセージ・詳細・ヒント |
| 回復戦略 | 段階的 | 自動回復 → ユーザー確認 → 手動対応 |
| ログ出力 | 詳細モード | `--verbose` フラグで詳細情報 |

---

## 7. 技術仕様

### 7.1 データ構造仕様

#### 7.1.1 基本データ構造
- **Repository**: リポジトリ情報管理（name, url, path, created_at, last_accessed, default_branch, remote_name）
- **Worktree**: Worktree情報管理（repository, branch, path, created_at, last_accessed, tracking_branch, is_detached, commit_hash）

#### 7.1.2 設定管理構造
- **Config**: アプリケーション設定統合管理
- **CoreConfig**: コア機能設定（root）

#### 7.1.3 エラー処理構造
- **GhwtError**: 統合エラー型（全エラーの統一インターフェース）
- **GitError**: Git関連エラー（リポジトリ・Worktree操作エラー）
- **ConfigError**: 設定関連エラー（設定ファイル読み書きエラー）
- **ValidationError**: バリデーションエラー（入力値検証エラー）
- **SerializationError**: シリアライゼーションエラー（JSON/TOML処理エラー）
- **NetworkError**: ネットワークエラー（リモートアクセスエラー）
- **PermissionError**: 権限エラー（ファイルアクセス権限エラー）

### 7.2 設定ファイルスキーマ

```toml
# ~/.config/ghwt/config.toml
[core]
root = "~/ghwt"                    # GHWT ルートディレクトリ
```

### 7.3 エラーコード体系

| コード | 名前 | 説明 | 典型的な原因 |
|--------|------|------|-------------|
| 0 | Success | 正常終了 | - |
| 1 | InvalidArgs | 引数・オプションエラー | 不正なコマンドライン引数 |
| 2 | GitError | Git関連エラー | リポジトリ不存在、Git操作失敗 |
| 3 | NetworkError | ネットワークエラー | リモートアクセス失敗 |
| 4 | FileSystemError | ファイルシステムエラー | ファイル読み書き失敗 |
| 5 | ConfigError | 設定エラー | 設定ファイル不正 |
| 6 | WorktreeError | Worktreeエラー | Worktree作成・削除失敗 |
| 7 | PermissionError | 権限エラー | アクセス権限不足 |
| 8 | DependencyError | 依存関係エラー | 必要ツール不存在 |
| 9 | InternalError | 内部エラー | プログラムバグ |
| 10 | UserCancelled | ユーザー操作キャンセル | Ctrl+C、確認プロンプトでNo |
| 11 | ResourceError | リソース不足エラー | ディスク容量不足 |
| 12 | VersionError | バージョン互換性エラー | Git版本不適合 |
| 13 | AuthenticationError | 認証エラー | SSH鍵認証失敗 |
| 14 | TimeoutError | タイムアウトエラー | ネットワーク操作タイムアウト |
| 15 | IntegrityError | データ整合性エラー | 破損したリポジトリ |

---

## 8. 操作例

```bash
# bare repo 取得
ghwt get git@github.com:myorg/alpha.git

# ブランチ用 worktree 作成 → すぐ移動
cd "$(ghwt new alpha featureA)"      # または ghwt-newcd …

# 一覧表示（テーブル形式）
ghwt ls

# 一覧表示（JSON形式）
ghwt ls --json

# 特定リポジトリのWorktree一覧
ghwt ls --repo alpha

# 14日以上放置された worktree をクリーン
ghwt prune --expire 14d

# 設定管理
ghwt config set core.root ~/workspace
ghwt config get core.root
ghwt config list --json
ghwt config validate

# 対話的選択
ghwt fzf

# Worktree削除
ghwt rm alpha featureA

# 既存リポジトリの変換
ghwt migrate ~/projects/existing-repo
```

---

## 9. ディレクトリレイアウト例

```
~/ghwt/alpha/
├── .bare/          # bare repository
├── .git            # gitfile → ./.bare
└── .wt/
    ├── main/
    ├── featureA/
    └── featureB/

~/.config/ghwt/
└── config.toml     # 設定ファイル
```

---

## 10. 完了条件

### 10.1 機能完了条件
- [ ] 全基本コマンド（get, new, ls, rm, prune）の実装
- [ ] 全補助コマンド（ghwt-newcd, ghwt fzf）の実装
- [ ] 完全な設定管理機能（set, get, list, unset, validate）
- [ ] JSON出力対応（ls, config list）
- [ ] 国際化対応（日本語・英語）

### 10.2 品質完了条件
- [ ] 全エラーケースのテストカバレッジ ≥ 90%
- [ ] 性能要件の達成（ls ≥ 100 worktree/秒、new ≤ 1秒）
- [ ] 設定ファイルバリデーションの完全実装
- [ ] エラー回復機能の実装

### 10.3 配布完了条件
- [ ] Homebrew Formula の作成・公開
- [ ] cargo install 対応
- [ ] 静的バイナリのGitHub Releases公開
- [ ] ドキュメントの完全性確認

---

## 11. ライセンス

本プロジェクトは MIT License で公開する。
再配布時はライセンス文と著作権表示の保持が必要。 