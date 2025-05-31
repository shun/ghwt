# GHWT 要件定義書  
Version 0.3 — 2025-05-31

## 概要

本ドキュメントは GHWT プロジェクトの要件定義書である。
プロジェクトの目的、機能要件、非機能要件、制約条件などを明確に定義し、
開発・テスト・運用フェーズ全体で参照できる単一の信頼ソース (SSoT) として機能する。

## 関連ドキュメント

- [../001-adr/](../001-adr/) - アーキテクチャ決定記録
- [../../README.md](../../README.md) - プロジェクト概要

## 変更履歴

| 日付 | バージョン | 変更内容 |
|------|------------|----------|
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

## 5. 機能要件 (MVP)

| ID | 機能 | 詳細 |
|----|------|------|
| **R1** | リポジトリ登録／変換 | `ghwt get <URL>` で bare clone＋レイアウト化<br>`ghwt migrate <path>` で既存 repo を変換 |
| **W1** | Worktree 生成 | `ghwt new [<repo>] <branch>` → 完了したパスを **stdout 最終行**に出力 |
| **L1** | Worktree 一覧 | `ghwt ls [--json] [--all]` で repo/branch/path/age を列挙 |
| **D1** | Worktree 削除 & プルーン | `ghwt rm` / `ghwt prune --expire 30d` |
| **C1** | シェル補助 | `ghwt-newcd`, `ghwt fzf` をスクリプトで同梱 |
| **S1** | 設定管理 | `ghwt config set/get/list` で TOML 設定ファイルを操作 |

> ghq sync などは後続フェーズで追加予定。

---

## 6. 非機能要件

| 分類 | 指標 |
|------|------|
| 性能 | `ghwt ls` ≥ 100 worktree/秒、`ghwt new` ≤ 1 秒 (ローカル) |
| 配布 | Homebrew Formula (macOS)・`cargo install`・静的バイナリ |
| 設定管理 | XDG Base Directory Specification 準拠<br>TOML 形式で人間が編集可能 |
| 信頼性 | エラー時に `git worktree prune` 実行を提案（確認あり）<br>設定により完全自動実行も可能 |

---

## 7. 操作例

```bash
# bare repo 取得
ghwt get git@github.com:myorg/alpha.git

# ブランチ用 worktree 作成 → すぐ移動
cd "$(ghwt new alpha featureA)"      # または ghwt-newcd …

# 一覧表示
ghwt ls

# 14日以上放置された worktree をクリーン
ghwt prune --expire 14d

# 設定管理
ghwt config set core.auto-prune true
ghwt config get core.root
```

---

## 8. ディレクトリレイアウト例

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

## 9. ライセンス

本プロジェクトは MIT License で公開する。
再配布時はライセンス文と著作権表示の保持が必要。 