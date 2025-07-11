# コントリビューションガイド

このドキュメントは、GHWTプロジェクトへの貢献方法について説明します。

## 開発環境のセットアップ

1.  Rustのインストール: [README-ja.md](./README-ja.md) の指示に従ってください。
2.  `rustfmt` (フォーマッター) のインストール: `rustup component add rustfmt`
3.  `clippy` (リンター) のインストール: `rustup component add clippy`

## コーディングスタイル

コードのフォーマットには `rustfmt` を使用します。設定はプロジェクトルートの `rustfmt.toml` に定義されています。
コミット前に `cargo fmt` を実行してください。

コードの静的解析には `clippy` を使用します。設定はプロジェクトルートの `clippy.toml` に定義されています。
コミット前に `cargo clippy -- -D warnings` を実行し、エラーや警告がないことを確認してください。

## (参考) VS Code での推奨設定

VS Code と `rust-analyzer` 拡張機能を使用する場合、以下の設定をプロジェクトの `.vscode/settings.json` (Git管理外) に記述すると、保存時に自動でフォーマットが実行されるようになります。

```json
// .vscode/settings.json (このファイルは .gitignore に追加することを推奨します)
{
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.tabSize": 4,
        "editor.insertSpaces": true
    },
    "files.eol": "\n" // 改行コードをLFに統一
}
```

## プルリクエストのプロセス

TODO: プルリクエストの作成手順、レビュープロセスなどを記述する

## Issueの報告

TODO: Issueの報告テンプレートやガイドラインを記述する

