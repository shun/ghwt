# ADR-006: TOML 設定ファイルと XDG Base Directory Specification の採用

**決定日**: 2025-05-31  
**ステータス**: 採用  

## コンテキスト

CLI ツールには設定ファイルが必要であり、形式と配置場所を決定する必要がある。

設定ファイル形式の選択肢：
- **YAML**: 可読性高、インデント敏感
- **JSON**: 標準的、コメント不可
- **TOML**: Rust 親和性、可読性高
- **INI**: シンプル、ネスト制限

配置場所の選択肢：
- 独自パス（例: `~/.ghwt/config`）
- XDG Base Directory Specification 準拠

## 決定

TOML 形式の設定ファイルを採用し、XDG Base Directory Specification に従って `$XDG_CONFIG_HOME/ghwt/` または `~/.config/ghwt/` に配置する。

## 根拠

1. **Rust エコシステムとの親和性**: Cargo.toml と同じ形式で、serde + toml クレートによる簡単な実装
2. **可読性と編集性**: コメント対応、人間が読みやすい構文
3. **型安全性**: Rust の構造体への直接マッピングが容易
4. **標準準拠**: XDG Base Directory Specification により他のツールとの共存が良好
5. **Git ユーザーの親しみやすさ**: Git config に近い構文

## 影響

- **正の影響**:
  - 設定ファイルの手動編集が容易
  - 他の CLI ツールとの設定ディレクトリ共存
  - Rust での実装が簡単

- **負の影響**:
  - TOML パーサーの依存関係追加
  - YAML に慣れたユーザーには学習コストが発生

- **対応策**:
  - 設定ファイルの例とドキュメントを充実
  - `ghwt config` コマンドでの設定支援

## 設定ファイル例

```toml
# ~/.config/ghwt/config.toml
[core]
auto-prune = false
root = "~/ghwt"

[ui]
pager = true
color = "auto"

[aliases]
n = "new"
l = "ls"
``` 