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
| **W1** | Worktree 生成 | `ghwt new <branch>` | 完了パスを **stdout 最終行** に出力<br>オプション: `--track`, `--force` |

### 5.2 グローバルオプション

| オプション | 短縮形 | 説明 | 対応コマンド |
|-----------|--------|------|-------------|
| `--help` | `-h` | ヘルプ表示 | 全コマンド |
| `--version` | `-V` | バージョン表示 | 全コマンド |
| `--verbose` | `-v` | 詳細出力 | 全コマンド |
| `--quiet` | `-q` | 静寂モード | 全コマンド |

### 5.3 出力形式仕様

#### 5.3.1 標準出力形式
- **成功メッセージ**: 人間が読みやすい形式
- **パス出力**: `ghwt new`は最終行に作成パスを出力（スクリプト連携用）

#### 5.3.2 エラー出力形式
- **標準エラー**: エラーメッセージは stderr に出力
- **詳細情報**: `--verbose` フラグで詳細なエラー情報
- **ヒント**: 解決方法の提案を含む

---

## 6. 非機能要件

### 6.1 性能要件

| 項目 | 指標 | 測定条件 |
|------|------|----------|
| Worktree作成 | ≤ 1 秒 | ローカルブランチ、SSD環境 |
| リポジトリ取得 | ネットワーク速度に依存 | リモートクローン時 |

### 6.2 配布・インストール要件

| 方法 | 対象OS | 詳細 |
|------|--------|------|
| Homebrew Formula | macOS | `brew install ghwt` |
| Cargo Install | 全対応OS | `cargo install ghwt` |
| 静的バイナリ | Linux/macOS | GitHub Releases |

### 6.3 信頼性・安全性要件

| 項目 | 仕様 | 詳細 |
|------|------|------|
| エラー回復 | 自動/手動 | `git worktree prune` 実行提案 |
| 権限管理 | 最小権限 | 必要最小限のファイルアクセス |

### 6.4 エラーハンドリング要件

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
- **Repository**: リポジトリ情報管理（name, url, path, created_at, default_branch）
- **Worktree**: Worktree情報管理（repository, branch, path, created_at, tracking_branch, is_detached）

#### 7.1.2 エラー処理構造
- **GhwtError**: 統合エラー型（全エラーの統一インターフェース）
- **GitError**: Git関連エラー（リポジトリ・Worktree操作エラー）
- **ValidationError**: バリデーションエラー（入力値検証エラー）
- **NetworkError**: ネットワークエラー（リモートアクセスエラー）
- **PermissionError**: 権限エラー（ファイルアクセス権限エラー）

### 7.2 エラーコード体系

| コード | 名前 | 説明 | 典型的な原因 |
|--------|------|------|-------------|
| 0 | Success | 正常終了 | - |
| 1 | InvalidArgs | 引数・オプションエラー | 不正なコマンドライン引数 |
| 2 | GitError | Git関連エラー | リポジトリ不存在、Git操作失敗 |
| 3 | NetworkError | ネットワークエラー | リモートアクセス失敗 |
| 4 | FileSystemError | ファイルシステムエラー | ファイル読み書き失敗 |
| 5 | WorktreeError | Worktreeエラー | Worktree作成・削除失敗 |
| 6 | PermissionError | 権限エラー | アクセス権限不足 |
| 7 | DependencyError | 依存関係エラー | 必要ツール不存在 |
| 8 | InternalError | 内部エラー | プログラムバグ |
| 9 | UserCancelled | ユーザー操作キャンセル | Ctrl+C、確認プロンプトでNo |

---

## 8. 操作例

```bash
# bare repo 取得
ghwt get git@github.com:myorg/alpha.git

# リポジトリディレクトリに移動
cd ~/ghwt/alpha

# ブランチ用 worktree 作成 → すぐ移動
cd "$(ghwt new featureA)"

# 別のブランチでも並行作業
cd ~/ghwt/alpha
cd "$(ghwt new hotfix-123)"
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
```

---

## 10. 完了条件

### 10.1 機能完了条件
- [ ] 基本コマンド（get, new）の実装
- [ ] エラーハンドリングの実装
- [ ] ヘルプ・バージョン表示の実装

### 10.2 品質完了条件
- [ ] 基本機能のテストカバレッジ ≥ 90%
- [ ] 性能要件の達成（new ≤ 1秒）
- [ ] エラー回復機能の実装

### 10.3 配布完了条件
- [ ] Homebrew Formula の作成・公開
- [ ] cargo install 対応
- [ ] 静的バイナリのGitHub Releases公開
- [ ] 基本ドキュメントの完全性確認

---

## 11. ライセンス

本プロジェクトは MIT License で公開する。
再配布時はライセンス文と著作権表示の保持が必要。 