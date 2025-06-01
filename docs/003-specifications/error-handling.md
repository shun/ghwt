# GHWT エラーコード体系・エラーハンドリング完全仕様書

**Version**: 1.0  
**Date**: 2025-06-01  
**Status**: Draft  

## このドキュメントの役割

- **対象読者**: 開発者、実装担当者
- **目的**: GHWT全体の統一エラーコード体系とエラーハンドリング実装仕様を定義
- **含む内容**: エラーコード定義、エラーメッセージ仕様、国際化対応、回復戦略
- **他ドキュメントとの関係**: 全てのエラー処理の根拠となる統一仕様（他ドキュメントからの参照を想定）

## 概要

本ドキュメントは GHWT プロジェクトの統一エラーコード体系とエラーハンドリング仕様書である。
全コマンド共通のエラーコード定義、エラーメッセージの統一フォーマット、
国際化対応（日本語・英語）のエラーメッセージ仕様、およびエラー回復戦略を定義する。

## 目的

- 全コマンド共通のエラーコード体系を定義
- エラーメッセージの統一フォーマットを確立
- 国際化対応（日本語・英語）のエラーメッセージ仕様を定義
- 一貫性のあるエラー処理基盤を確立

---

## 1. エラーコード一覧

### 1.1 基本エラーコード定義

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// 正常終了
    Success = 0,
    
    /// 引数・オプションエラー
    InvalidArgs = 1,
    
    /// Git関連エラー
    GitError = 2,
    
    /// ネットワークエラー
    NetworkError = 3,
    
    /// ファイルシステムエラー
    FileSystemError = 4,
    
    /// 設定エラー
    ConfigError = 5,
    
    /// Worktreeエラー
    WorktreeError = 6,
    
    /// 権限エラー
    PermissionError = 7,
    
    /// 依存関係エラー
    DependencyError = 8,
    
    /// 内部エラー（予期しないエラー）
    InternalError = 9,
    
    /// ユーザー操作キャンセル
    UserCancelled = 10,
    
    /// リソース不足エラー
    ResourceError = 11,
    
    /// バージョン互換性エラー
    VersionError = 12,
    
    /// 認証エラー
    AuthenticationError = 13,
    
    /// タイムアウトエラー
    TimeoutError = 14,
    
    /// データ整合性エラー
    IntegrityError = 15,
}
```

### 1.2 エラーコード詳細仕様

| コード | 名前 | 説明 | 典型的な原因 |
|--------|------|------|-------------|
| 0 | Success | 正常終了 | - |
| 1 | InvalidArgs | 引数・オプションエラー | 不正なコマンドライン引数、必須パラメータ不足 |
| 2 | GitError | Git関連エラー | リポジトリ不存在、ブランチ不存在、Git操作失敗 |
| 3 | NetworkError | ネットワークエラー | リモートアクセス失敗、DNS解決失敗 |
| 4 | FileSystemError | ファイルシステムエラー | ファイル読み書き失敗、パス解決失敗 |
| 5 | ConfigError | 設定エラー | 設定ファイル不正、設定値無効 |
| 6 | WorktreeError | Worktreeエラー | Worktree作成失敗、削除失敗 |
| 7 | PermissionError | 権限エラー | ファイル・ディレクトリアクセス権限不足 |
| 8 | DependencyError | 依存関係エラー | 必要なツール不存在、バージョン不適合 |
| 9 | InternalError | 内部エラー | プログラムバグ、予期しない状態 |
| 10 | UserCancelled | ユーザー操作キャンセル | Ctrl+C、確認プロンプトでNo選択 |
| 11 | ResourceError | リソース不足エラー | ディスク容量不足、メモリ不足 |
| 12 | VersionError | バージョン互換性エラー | Git版本不適合、設定ファイル版本不適合 |
| 13 | AuthenticationError | 認証エラー | SSH鍵認証失敗、トークン無効 |
| 14 | TimeoutError | タイムアウトエラー | ネットワーク操作タイムアウト |
| 15 | IntegrityError | データ整合性エラー | 破損したリポジトリ、不整合な状態 |

---

## 2. エラーメッセージフォーマット

### 2.1 基本フォーマット

#### 標準エラー出力形式
```
Error: <message>
```

#### 詳細エラー情報（--verbose フラグ時）
```
Error: <message>
Details: <detailed_information>
Caused by: <root_cause>
```

#### ヒントメッセージ付き
```
Error: <message>
Hint: <suggestion_for_resolution>
```

### 2.2 エラーメッセージ構造

```rust
#[derive(Debug, Clone)]
pub struct ErrorMessage {
    /// エラーコード
    pub code: ExitCode,
    
    /// 主要メッセージ
    pub message: String,
    
    /// 詳細情報（オプション）
    pub details: Option<String>,
    
    /// 根本原因（オプション）
    pub caused_by: Option<String>,
    
    /// ヒント・解決方法（オプション）
    pub hint: Option<String>,
    
    /// 関連ファイル・パス（オプション）
    pub location: Option<String>,
}
```

### 2.3 出力例

#### 基本エラー
```
Error: Repository not found at '/path/to/repo'
```

#### 詳細エラー（--verbose）
```
Error: Repository not found at '/path/to/repo'
Details: The specified path does not contain a valid Git repository
Caused by: No .git directory found
Hint: Run 'git init' to initialize a new repository or check the path
```

---

## 3. エラーカテゴリ別詳細仕様

### 3.1 Git関連エラー（ExitCode::GitError = 2）

#### リポジトリが見つからない
```rust
// 日本語
"リポジトリが見つかりません: {path}"
"指定されたパスにGitリポジトリが存在しません"

// 英語
"Repository not found: {path}"
"No Git repository found at the specified path"
```

#### ブランチが存在しない
```rust
// 日本語
"ブランチ '{branch}' が存在しません"
"リモートブランチ '{remote}/{branch}' が見つかりません"

// 英語
"Branch '{branch}' does not exist"
"Remote branch '{remote}/{branch}' not found"
```

#### リモートアクセスエラー
```rust
// 日本語
"リモートリポジトリにアクセスできません: {remote}"
"認証に失敗しました"

// 英語
"Cannot access remote repository: {remote}"
"Authentication failed"
```

#### Worktree作成失敗
```rust
// 日本語
"Worktreeの作成に失敗しました: {path}"
"指定されたパスは既に使用されています: {path}"

// 英語
"Failed to create worktree: {path}"
"Path already in use: {path}"
```

### 3.2 ファイルシステムエラー（ExitCode::FileSystemError = 4）

#### 権限不足
```rust
// 日本語
"ファイルへのアクセス権限がありません: {path}"
"ディレクトリの作成権限がありません: {path}"

// 英語
"Permission denied: {path}"
"Cannot create directory: {path}"
```

#### ディスク容量不足
```rust
// 日本語
"ディスク容量が不足しています"
"ファイルの書き込みに失敗しました: 容量不足"

// 英語
"Insufficient disk space"
"Failed to write file: disk full"
```

#### パス解決失敗
```rust
// 日本語
"パスを解決できません: {path}"
"無効なパス形式です: {path}"

// 英語
"Cannot resolve path: {path}"
"Invalid path format: {path}"
```

### 3.3 設定エラー（ExitCode::ConfigError = 5）

#### 不正な設定値
```rust
// 日本語
"設定値が無効です: {key} = {value}"
"設定ファイルの形式が正しくありません: {file}"

// 英語
"Invalid configuration value: {key} = {value}"
"Invalid configuration file format: {file}"
```

#### 設定ファイル読み込み失敗
```rust
// 日本語
"設定ファイルを読み込めません: {file}"
"設定ファイルが見つかりません: {file}"

// 英語
"Cannot read configuration file: {file}"
"Configuration file not found: {file}"
```

---

## 4. 国際化対応

### 4.1 ロケール検出

```rust
pub fn detect_locale() -> Locale {
    // 環境変数の優先順位
    // 1. GHWT_LANG
    // 2. LANG
    // 3. LC_ALL
    // 4. システムデフォルト
    
    if let Ok(lang) = std::env::var("GHWT_LANG") {
        return parse_locale(&lang);
    }
    
    if let Ok(lang) = std::env::var("LANG") {
        return parse_locale(&lang);
    }
    
    if let Ok(lang) = std::env::var("LC_ALL") {
        return parse_locale(&lang);
    }
    
    Locale::English // デフォルト
}

#[derive(Debug, Clone, Copy)]
pub enum Locale {
    Japanese,
    English,
}
```

### 4.2 メッセージ切り替え

```rust
pub trait LocalizedMessage {
    fn message(&self, locale: Locale) -> String;
    fn hint(&self, locale: Locale) -> Option<String>;
}

impl LocalizedMessage for GitError {
    fn message(&self, locale: Locale) -> String {
        match (self, locale) {
            (GitError::RepositoryNotFound(path), Locale::Japanese) => {
                format!("リポジトリが見つかりません: {}", path)
            }
            (GitError::RepositoryNotFound(path), Locale::English) => {
                format!("Repository not found: {}", path)
            }
            // その他のパターン...
        }
    }
    
    fn hint(&self, locale: Locale) -> Option<String> {
        match (self, locale) {
            (GitError::RepositoryNotFound(_), Locale::Japanese) => {
                Some("'git init' でリポジトリを初期化するか、パスを確認してください".to_string())
            }
            (GitError::RepositoryNotFound(_), Locale::English) => {
                Some("Run 'git init' to initialize a repository or check the path".to_string())
            }
            // その他のパターン...
        }
    }
}
```

### 4.3 メッセージカタログ

#### 日本語メッセージ（ja.toml）
```toml
[git]
repository_not_found = "リポジトリが見つかりません: {path}"
branch_not_found = "ブランチ '{branch}' が存在しません"
remote_access_failed = "リモートリポジトリにアクセスできません: {remote}"

[filesystem]
permission_denied = "ファイルへのアクセス権限がありません: {path}"
disk_full = "ディスク容量が不足しています"
path_not_found = "パスが見つかりません: {path}"

[config]
invalid_value = "設定値が無効です: {key} = {value}"
file_not_found = "設定ファイルが見つかりません: {file}"
```

#### 英語メッセージ（en.toml）
```toml
[git]
repository_not_found = "Repository not found: {path}"
branch_not_found = "Branch '{branch}' does not exist"
remote_access_failed = "Cannot access remote repository: {remote}"

[filesystem]
permission_denied = "Permission denied: {path}"
disk_full = "Insufficient disk space"
path_not_found = "Path not found: {path}"

[config]
invalid_value = "Invalid configuration value: {key} = {value}"
file_not_found = "Configuration file not found: {file}"
```

---

## 5. エラー回復戦略

### 5.1 自動回復可能なエラー

#### Worktree自動クリーンアップ
```rust
pub enum RecoveryAction {
    /// 自動的に回復を試行
    AutoRecover,
    /// ユーザーに確認を求める
    PromptUser,
    /// 回復不可能
    NoRecovery,
}

impl GitError {
    pub fn recovery_action(&self) -> RecoveryAction {
        match self {
            GitError::WorktreeAlreadyExists(_) => RecoveryAction::PromptUser,
            GitError::OrphanedWorktree(_) => RecoveryAction::AutoRecover,
            GitError::RepositoryNotFound(_) => RecoveryAction::NoRecovery,
            // その他...
        }
    }
}
```

#### 自動回復の実装例
```rust
pub async fn auto_recover_worktree_error(error: &GitError) -> Result<(), Error> {
    match error {
        GitError::OrphanedWorktree(path) => {
            eprintln!("Warning: Cleaning up orphaned worktree at {}", path);
            git_worktree_prune().await?;
            Ok(())
        }
        _ => Err(Error::CannotRecover),
    }
}
```

### 5.2 ユーザー確認が必要なエラー

#### 確認プロンプトの実装
```rust
pub fn prompt_user_recovery(error: &GitError) -> Result<bool, Error> {
    match error {
        GitError::WorktreeAlreadyExists(path) => {
            let message = match detect_locale() {
                Locale::Japanese => {
                    format!("Worktree '{}' は既に存在します。削除して続行しますか？ [y/N]", path)
                }
                Locale::English => {
                    format!("Worktree '{}' already exists. Remove and continue? [y/N]", path)
                }
            };
            
            eprint!("{} ", message);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            Ok(input.trim().to_lowercase() == "y")
        }
        _ => Ok(false),
    }
}
```

### 5.3 `git worktree prune` 実行提案

#### 提案タイミング
1. **Worktree作成失敗時**: 既存のWorktreeパスと競合する場合
2. **Worktree一覧表示時**: 孤立したWorktreeが検出された場合
3. **定期的なメンテナンス**: `ghwt status` コマンド実行時

#### 実装例
```rust
pub fn suggest_worktree_prune(context: &str) -> String {
    match detect_locale() {
        Locale::Japanese => {
            format!(
                "{}の際に問題が発生しました。\n\
                 'git worktree prune' を実行して不要なWorktreeを削除することをお勧めします。\n\
                 実行しますか？ [y/N]",
                context
            )
        }
        Locale::English => {
            format!(
                "An issue occurred during {}.\n\
                 Consider running 'git worktree prune' to clean up unused worktrees.\n\
                 Run now? [y/N]",
                context
            )
        }
    }
}
```

---

## 6. エラーハンドリング実装ガイドライン

### 6.1 エラー型定義

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GhwtError {
    #[error("Git error: {0}")]
    Git(#[from] GitError),
    
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),
    
    #[error("Worktree error: {0}")]
    Worktree(#[from] WorktreeError),
}

impl GhwtError {
    pub fn exit_code(&self) -> ExitCode {
        match self {
            GhwtError::Git(_) => ExitCode::GitError,
            GhwtError::Config(_) => ExitCode::ConfigError,
            GhwtError::FileSystem(_) => ExitCode::FileSystemError,
            GhwtError::Network(_) => ExitCode::NetworkError,
            GhwtError::Worktree(_) => ExitCode::WorktreeError,
        }
    }
}
```

### 6.2 エラー出力の統一

```rust
pub fn print_error(error: &GhwtError, verbose: bool) {
    let locale = detect_locale();
    let message = error.localized_message(locale);
    
    eprintln!("Error: {}", message);
    
    if verbose {
        if let Some(details) = error.details(locale) {
            eprintln!("Details: {}", details);
        }
        
        if let Some(cause) = error.root_cause() {
            eprintln!("Caused by: {}", cause);
        }
    }
    
    if let Some(hint) = error.hint(locale) {
        eprintln!("Hint: {}", hint);
    }
}
```

### 6.3 main関数でのエラーハンドリング

```rust
#[tokio::main]
async fn main() {
    let result = run().await;
    
    match result {
        Ok(()) => {
            std::process::exit(ExitCode::Success as i32);
        }
        Err(error) => {
            let verbose = std::env::args().any(|arg| arg == "--verbose" || arg == "-v");
            print_error(&error, verbose);
            
            // 回復可能なエラーの場合は回復を試行
            if let Ok(()) = attempt_recovery(&error).await {
                eprintln!("Recovered from error, retrying...");
                if let Ok(()) = run().await {
                    std::process::exit(ExitCode::Success as i32);
                }
            }
            
            std::process::exit(error.exit_code() as i32);
        }
    }
}
```

---

## 7. テスト戦略

### 7.1 エラーケーステスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_codes() {
        assert_eq!(ExitCode::Success as i32, 0);
        assert_eq!(ExitCode::GitError as i32, 2);
        assert_eq!(ExitCode::ConfigError as i32, 5);
    }
    
    #[test]
    fn test_localized_messages() {
        let error = GitError::RepositoryNotFound("/path/to/repo".to_string());
        
        assert_eq!(
            error.message(Locale::Japanese),
            "リポジトリが見つかりません: /path/to/repo"
        );
        
        assert_eq!(
            error.message(Locale::English),
            "Repository not found: /path/to/repo"
        );
    }
    
    #[test]
    fn test_recovery_actions() {
        let error = GitError::OrphanedWorktree("/path".to_string());
        assert_eq!(error.recovery_action(), RecoveryAction::AutoRecover);
    }
}
```

### 7.2 統合テスト

```rust
#[tokio::test]
async fn test_error_handling_integration() {
    // 存在しないリポジトリでコマンド実行
    let result = ghwt_clone("nonexistent/repo", "/tmp/test").await;
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.exit_code(), ExitCode::GitError);
}
```

---

## 8. 完了条件チェックリスト

- [x] `docs/003-specifications/error-handling.md` ファイルが作成されている
- [x] 完全なエラーコード一覧が定義されている
- [x] エラーメッセージフォーマットが統一されている
- [x] 国際化対応の仕様が含まれている
- [x] エラー回復戦略が明記されている
- [x] Git関連エラーの詳細仕様が含まれている
- [x] ファイルシステムエラーの詳細仕様が含まれている
- [x] 設定エラーの詳細仕様が含まれている
- [x] 実装ガイドラインが含まれている
- [x] テスト戦略が含まれている

---

## 9. 関連ドキュメント

- [CLI インターフェース完全仕様書](./cli-interface.md)
- [TOML設定ファイル完全スキーマ仕様書](./config-schema.md)
- [要件定義書](../002-requirements/requirements-ja.md)
- [ADR-005: エラーハンドリング戦略](../001-adr/005-error-handling.md)

---

*本仕様書は GHWT プロジェクトの統一エラーハンドリング基盤として、一貫性のあるエラー処理体験を提供します。* 