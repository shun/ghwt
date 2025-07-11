# GHWT 要件定義書  
Version 1.0 — 2025-06-01

## このドキュメントの役割

- **対象読者**: 開発チーム、プロダクトオーナー、ステークホルダー
- **目的**: プロジェクトの目的、機能要件、非機能要件、制約条件を定義
- **含まない内容**: 技術実装の詳細、データ構造、エラーコード体系（これらは仕様書に委譲）

## 概要

本ドキュメントは GHWT プロジェクトの要件定義書である。
プロジェクトの目的、機能要件、非機能要件、制約条件などを明確に定義し、
開発・テスト・運用フェーズ全体で参照できる単一の信頼ソース (SSoT) として機能する。

## 関連ドキュメント

- [../004-adr/](../004-adr/) - アーキテクチャ決定記録
- [../003-designs/](../003-designs/) - 詳細仕様書群
- [../../README-ja.md](../../README-ja.md) - プロジェクト概要

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

## 1. 背景と課題

### 1.1 現状の課題

現代のソフトウェア開発では、複数のブランチを同時に扱う必要性が急激に増加している。フィーチャー開発、バグ修正、コードレビュー、ホットフィックス対応など、開発者は日常的に複数のコンテキストを切り替えながら作業を行う。

### 1.2 従来手法の問題点

#### 1.2.1 `git checkout` の問題

**典型的なシナリオ:**
```bash
# 機能開発中に緊急バグ修正が必要になった場合
git checkout feature-payment     # 支払い機能開発中
# → 緊急対応要請が発生
git stash                        # 作業を一時退避
git checkout main
git checkout -b hotfix-urgent    # 緊急修正ブランチ
# → hotfix完了後、元の作業に戻る
git checkout feature-payment
git stash pop                    # 作業を復元
# → 依存関係の再インストールで時間を浪費
npm install  # またはbundle install, pip install等
```

**問題点:**
- **コンテキストスイッチのコスト**: 依存関係の再インストール（npm install、bundle install等）で数分〜数十分の待機時間
- **作業状態の喪失**: 未コミット変更の一時退避が必要で、作業の中断が頻発
- **IDE の混乱**: プロジェクト設定の再構築、インデックスの再構築で開発効率が低下
- **並行作業の困難**: 複数ブランチでの同時ビルド・テスト実行が不可能

#### 1.2.2 標準的な `git worktree` の問題

**現在の散乱した構造:**
```
~/projects/
├── myapp/                    # メインリポジトリ
├── myapp-feature-auth/       # feature/auth ブランチ
├── myapp-hotfix-123/         # hotfix/123 ブランチ  
├── myapp-develop/            # develop ブランチ
└── temp-branch-test/         # 一時的なテスト用
```

**問題点:**
- **ディレクトリの散乱**: リポジトリ関連ディレクトリが散らばり、管理が困難
- **視認性の悪化**: どのディレクトリがどのプロジェクトの何のブランチかが不明
- **誤操作のリスク**: 似た名前のディレクトリで間違った操作を実行
- **残骸の蓄積**: 不要になったworktreeが放置され、ディスク容量を圧迫
- **命名の一貫性不足**: 各開発者が独自の命名規則を使用し、チーム内で混乱

#### 1.2.3 開発チームでの実際の困りごと

**レビュー作業の場合:**
```bash
# レビュー対象のPRが複数ある場合
git checkout pr-123   # PR #123をレビュー
# → ビルド・テスト実行（5-10分）
git checkout pr-456   # 次のPR #456をレビュー  
# → 再度ビルド・テスト実行（5-10分）
# → 1つ目のPRに戻って修正確認
git checkout pr-123
# → また同じビルド時間が発生...
```

**複数環境での並行テストの場合:**
```bash
# 本来やりたいこと：複数ブランチを同時テスト
# feature-payment と feature-auth を同時に動作確認したい
# → 現状では不可能、逐次実行しかできない
```

### 1.3 解決方針

#### 1.3.1 As-Is（現状の問題）vs To-Be（GHWTでの解決）

| 観点 | As-Is（現状の問題） | To-Be（GHWTでの解決） |
|------|--------------------|--------------------|
| **ブランチ切り替え** | `git checkout` + 依存関係再インストール（5-10分） | 各ブランチが独立環境、即座に切り替え可能 |
| **並行作業** | 1つのブランチでしか作業できない | 複数ブランチで同時開発・テスト・ビルド |
| **ディレクトリ管理** | プロジェクト関連ディレクトリが散乱 | 1つのプロジェクトディレクトリに統一 |
| **セットアップ時間** | 新しいブランチ作業開始まで数分〜数十分 | `ghwt new <branch>` で数秒 |
| **レビュー効率** | PR毎にcheckout + ビルド待ち | 複数PRを並行でレビュー可能 |
| **作業状態管理** | stash/unstashで作業状態を頻繁に退避 | 各ブランチで作業状態が独立して保持 |

#### 1.3.2 To-Be：理想的な開発体験

**最終ゴール：整理されたワークスペース構造**
```
# As-Is（散乱した状態）
~/projects/
├── myapp/                    # メインリポジトリ
├── myapp-feature-auth/       # ブランチ毎に散乱
├── myapp-hotfix-123/         # 管理が困難
└── temp-branch-test/         # 残骸が蓄積

# To-Be（1つのプロジェクトディレクトリに統一）
~/ghwt/myapp/                 # プロジェクト専用ディレクトリ
├── [共有リポジトリデータ]     # 全ブランチで共有
├── [ブランチ1のワークスペース] # main
├── [ブランチ2のワークスペース] # feature-auth  
└── [ブランチ3のワークスペース] # hotfix-123

# 実現される状態
✅ 1つのプロジェクトに関連する全てが1箇所に集約
✅ 各ブランチが独立したワークスペースを持つ
✅ 依存関係・ビルド結果・IDE設定が各ブランチで独立
✅ 不要なワークスペースの管理・削除が容易
```

**シンプルなワークフロー:**
```bash
# リポジトリ取得（1回のみ）
ghwt get git@github.com:myorg/myapp.git

# 複数機能を並行開発
cd "$(ghwt new feature-payment)"    # 支払い機能開発
cd "$(ghwt new feature-auth)"       # 認証機能開発（並行）  
cd "$(ghwt new hotfix-urgent)"      # 緊急修正（並行）

# 即座にブランチ間移動、依存関係再インストール不要
# IDE設定・ビルド結果・テスト結果が各環境で独立
```

**開発者の体験改善:**
- ⚡ **即座のコンテキスト切り替え**: ブランチ移動が数秒で完了
- 🔄 **真の並行作業**: 複数ブランチで同時ビルド・テスト実行
- 📁 **整理されたワークスペース**: プロジェクト関連ファイルが1箇所に集約
- 🛡️ **作業状態の保護**: 緊急対応時も既存作業が中断されない
- 👥 **チーム内の一貫性**: 全メンバーが同一のディレクトリ構造を使用

#### 1.3.3 技術的アプローチ

**Nick Nisi式アーキテクチャの採用:**
- 「1 Repository = 1 Directory」原則による構造化
- bare repository + worktree + gitfile による技術基盤
- **詳細な技術仕様**: [../003-designs/filesystem-layout.md](../003-designs/filesystem-layout.md)

**実装方針:**
- CLI による自動化でユーザーは技術詳細を意識不要
- ghq互換でスムーズな移行体験
- **詳細な実装仕様**: [../003-designs/core-functions.md](../003-designs/core-functions.md)

---

## 2. 機能要件 (MVP: Minimum Viable Product)

### 2.1 基本コマンド機能

| ID | 機能 | コマンド | 詳細仕様 |
|----|------|----------|----------|
| **R1** | リポジトリ取得・変換 | `ghwt get <url>` | bare clone + Nick Nisi式レイアウト化 |
| **W1** | Worktree 生成 | `ghwt new <branch>` | 完了パスを **stdout 最終行** に出力 |

### 2.2 グローバルオプション

| オプション | 短縮形 | 説明 | 対応コマンド |
|-----------|--------|------|-------------|
| `--help` | `-h` | ヘルプ表示 | 全コマンド |
| `--version` | `-V` | バージョン表示 | 全コマンド |
| `--verbose` | `-v` | 詳細出力 | 全コマンド |
| `--quiet` | `-q` | 静寂モード | 全コマンド |

**詳細なコマンド仕様**: [../003-designs/cli-interface.md](../003-designs/cli-interface.md)

---

## 3. 非機能要件

### 3.1 性能要件

| 項目 | 指標 | 測定条件 |
|------|------|----------|
| Worktree作成 | ≤ 1 秒 | ローカルブランチ、SSD環境 |
| リポジトリ取得 | ネットワーク速度に依存 | リモートクローン時 |

### 3.2 配布・インストール要件

| 方法 | 対象OS | 詳細 |
|------|--------|------|
| Homebrew Formula | macOS | `brew install ghwt` |
| Cargo Install | 全対応OS | `cargo install ghwt` |
| 静的バイナリ | Linux/macOS | GitHub Releases |

### 3.3 信頼性・安全性要件

| 項目 | 仕様 | 詳細 |
|------|------|------|
| エラー回復 | 自動/手動 | `git worktree prune` 実行提案 |
| 権限管理 | 最小権限 | 必要最小限のファイルアクセス |

### 3.4 エラーハンドリング要件

| 項目 | 仕様 | 詳細 |
|------|------|------|
| エラーコード体系 | 統一化 | 詳細: [../003-designs/error-handling.md](../003-designs/error-handling.md) |
| エラーメッセージ | 構造化 | メッセージ・詳細・ヒント |
| 回復戦略 | 段階的 | 自動回復 → ユーザー確認 → 手動対応 |
| ログ出力 | 詳細モード | `--verbose` フラグで詳細情報 |

---

## 4. 前提条件

| 項目 | 内容 |
|------|------|
| 対応 OS | macOS / Linux / WSL (Windows PowerShell ネイティブは対象外) |
| 対応シェル | bash / zsh（fish・PowerShell は後方フェーズで対応） |
| 必須ソフト | Git ≥ 2.38 |
| 推奨ツール | ghq ≥ 1.4 （任意） |

---

## 5. アーキテクチャ決定記録 (ADR)

重要なアーキテクチャ決定とその根拠については、別途 **[../004-adr/](../004-adr/)** を参照。

主要な決定項目：
- [ADR-001](../004-adr/001-bare-repository.md): Bare Repository + Gitfile アーキテクチャの採用
- [ADR-002](../004-adr/002-cli-scope.md): CLI の責務範囲の限定
- [ADR-003](../004-adr/003-directory-principle.md): 1 Repository = 1 Directory 原則
- [ADR-004](../004-adr/004-rust-implementation.md): Rust 実装の選択
- [ADR-005](../004-adr/005-mit-license.md): MIT ライセンスの採用
- [ADR-006](../004-adr/006-toml-config.md): TOML 設定ファイルと XDG Base Directory Specification の採用

---

## 6. 基本操作例

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

**詳細な技術仕様**: [../003-designs/core-functions.md](../003-designs/core-functions.md)

---

## 7. 完了条件

### 7.1 機能完了条件
- [ ] 基本コマンド（get, new）の実装
- [ ] エラーハンドリングの実装
- [ ] ヘルプ・バージョン表示の実装

### 7.2 品質完了条件
- [ ] 基本機能のテストカバレッジ ≥ 90%
- [ ] 性能要件の達成（new ≤ 1秒）
- [ ] エラー回復機能の実装

### 7.3 配布完了条件
- [ ] Homebrew Formula の作成・公開
- [ ] cargo install 対応
- [ ] 静的バイナリのGitHub Releases公開
- [ ] 基本ドキュメントの完全性確認

---

## 8. ライセンス

本プロジェクトは MIT License で公開する。
再配布時はライセンス文と著作権表示の保持が必要。 