#!/bin/bash
# このスクリプトは、リポジトリのルートディレクトリで実行してください。

# UNIVERSAL_AGENT_RULES.md へのシンボリックリンクを作成します。
# これにより、GEMINI.md, CLAUDE.md, AGENTS.md が共通ルールを参照するようになります。
ln -sf $PWD/.aicoding/UNIVERSAL_AGENT_RULES.md $PWD/GEMINI.md
ln -sf $PWD/.aicoding/UNIVERSAL_AGENT_RULES.md $PWD/CLAUDE.md
ln -sf $PWD/.aicoding/UNIVERSAL_AGENT_RULES.md $PWD/AGENTS.md
