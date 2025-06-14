# ADR-007: 文書管理戦略 - 要件定義書とREADMEの役割分担

**決定日**: 2025-05-31  
**ステータス**: 採用  

## コンテキスト

ソフトウェアプロジェクトでは、異なる読者層に向けた複数の文書が必要となる。特に、開発チーム向けの技術仕様書とエンドユーザー向けのガイド文書では、目的・内容・更新頻度が大きく異なる。

従来のプロジェクトでは、これらの文書の役割が曖昧で、以下の問題が発生していた：

- 開発者が実装時に参照すべき仕様が散在
- エンドユーザーが必要な情報にアクセスしにくい
- 文書間の整合性維持が困難
- 更新責任の所在が不明確
- 変更履歴の表示順序が統一されていない

## 決定

文書を以下の2つの主要カテゴリに明確に分離し、それぞれ異なる目的と管理方針を適用する：

### 1. 要件定義書（docs/001-requirements/002-requirements-ja.md）
- **対象読者**: 開発チーム内部
- **目的**: 実装・テスト・運用フェーズでの単一の信頼ソース（SSoT）
- **内容**: 網羅的で詳細な技術仕様

### 2. README（README-ja.md）
- **対象読者**: エンドユーザー
- **目的**: プロジェクトの価値訴求と使用開始の支援
- **内容**: 実用的で体験重視のガイド

### 3. 変更履歴の表示順序
- **順序**: 降順（DESC）- 最新の変更を先頭に表示
- **根拠**: 業界標準（CHANGELOG.md、GitHub Releases等）との一貫性
- **実用性**: 「最近何が変わったか？」という最も一般的な質問に即答

## 根拠

1. **読者層の明確化**: 開発者とエンドユーザーでは必要な情報が根本的に異なる
2. **情報密度の最適化**: 各文書が特定の目的に特化することで情報効率が向上
3. **保守性の向上**: 役割が明確になることで更新責任と頻度が明確化
4. **ユーザビリティの向上**: 読者が迷わず適切な文書にアクセス可能
5. **業界標準への準拠**: 変更履歴の降順表示により、他のプロジェクトとの一貫性を保持

## 影響

- **正の影響**:
  - 開発効率の向上（開発者が仕様を迅速に参照可能）
  - ユーザー体験の向上（初見ユーザーが価値を理解しやすい）
  - 文書品質の向上（各文書が特定の目的に最適化）
  - 保守負荷の軽減（更新責任の明確化）
  - 変更履歴の実用性向上（最新情報への即座のアクセス）

- **負の影響**:
  - 文書数の増加による管理コストの上昇
  - 情報の重複リスク
  - 文書間の整合性維持の必要性

- **対応策**:
  - 定期的な文書間整合性チェックの実施
  - 共通情報の参照関係を明確化
  - 更新時のチェックリスト作成
  - 変更履歴の統一フォーマット採用

## 具体的な役割分担

### 要件定義書の責務
- 機能要件の詳細仕様（R1, W1, L1, D1, C1, S1）
- 非機能要件の具体的指標
- アーキテクチャ決定の記録と参照
- 技術的根拠と背景の詳細説明
- 実装時の判断基準

### READMEの責務
- プロジェクトの価値提案
- インストールと初期設定手順
- 基本的な使用方法とワークフロー例
- トラブルシューティング
- コミュニティ情報（コントリビューション、ライセンス等）

### 変更履歴の管理方針
- **表示順序**: 降順（最新 → 古い）
- **フォーマット**: `| 日付 | 変更内容 |` の統一テーブル形式
- **更新方法**: 新しい変更を常に先頭に追加
- **適用範囲**: ADR、要件定義書、README等の全ての変更履歴

### 更新方針
- **要件定義書**: 設計変更・機能追加時に更新
- **README**: 機能追加・改善・ユーザーフィードバック時に更新
- **整合性チェック**: 両文書の同時更新時に実施
- **変更履歴**: 各文書の更新時に対応する履歴も更新

## 参照関係

- README → 要件定義書: 詳細仕様への参照リンク
- 要件定義書 → ADR: アーキテクチャ決定への参照
- 両文書 → 関連リンク: 外部資料への統一的な参照
- 変更履歴 → 業界標準: CHANGELOG.md、Semantic Versioning等のベストプラクティスに準拠 