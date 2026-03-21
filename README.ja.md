# cc-statusline

[![CI](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml)
[![Release](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml)
[![npm](https://img.shields.io/npm/v/cc-statusline-tui)](https://www.npmjs.com/package/cc-statusline-tui)
[![crates.io](https://img.shields.io/crates/v/cc-statusline-tui)](https://crates.io/crates/cc-statusline-tui)

> Claude Codeのステータスバー、地味すぎない？華やかにしよう。

[English](README.md) | [中文](README.zh.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Português](README.pt.md) | [Русский](README.ru.md)

![statusline preview](preview.png)

コマンドひとつ。7言語対応。後悔ゼロ。

## インストール

お好きな方法でどうぞ：

### npm

```bash
npx cc-statusline-tui
```

### Cargo

```bash
cargo install cc-statusline-tui
```

### Homebrew

```bash
brew tap LokiQ0713/cc-statusline-tui
brew install cc-statusline
```

## 何が起きるの？

TUIウィザードが立ち上がります。ポチポチ選ぶだけ。カッコよくなる。以上。

```
1/4 セグメント選択  → 何を表示するか
2/4 設定          → 見た目の調整（ライブプレビュー付き）
3/4 並び替え      → 表示順を決める
4/4 確認          → 完了！
```

カスタマイズ？面倒くさい？「デフォルトを使用」を選んでEnter2回。それでOK。誰も責めません。

## セグメント

| セグメント | 表示例 | 機能 | カスタマイズ |
|-----------|--------|------|-------------|
| Model | `🔥 Opus4.6` | 今使ってるモデルを表示 | アイコン (🔥🦊🐢🐰)、色 |
| Cost | `$0.42` | お財布へのダメージを可視化 | 色 |
| Usage | `██░░ 25% 1h43m` | 5時間レート制限：バー + % + リセットまでの時間 | バースタイル/文字/長さ、色、更新間隔 |
| Path | `~/project` | 今いるディレクトリ | 色、最大長 |
| Git | `main* ↑2↓1` | ブランチ + 未コミット + ahead/behind | 表示項目、色 |
| Context | `▓▓▓▓░░░ 60% 600K/1M` | コンテキストウィンドウの消費量 | バースタイル/文字/長さ、色 |
| Crypto | `BTC:$73748` | コーディング中に仮想通貨チェック。なぜって？そこに価格があるから。 | 通貨 (BTC/ETH/BNB/SOL)、色、更新間隔 |

## カラースタイル

| スタイル | 雰囲気 |
|---------|--------|
| **Ultrathink Rainbow** | 一文字ずつ色が変わる。そう、キラキラします。 |
| **Ultrathink Gradient** | バー全体にスムーズな虹のグラデーション。芸術的。 |
| **Traffic Light** | 緑 (≤30%) → 黄 (≤60%) → 赤 (>60%)。コンテキストウィンドウが渋滞中。 |
| Cyan / Green / Blue / Yellow | 堅実な選択 |
| Magenta / Red / Orange / Pink / Purple / White | 個性を出したいあなたに |

## 対応言語

English, 中文, 日本語, 한국어, Español, Português, Русский。

初回起動時に選択。ステータスバーがあなたの言葉を話します。

## 仕組み

1. ウィザードが設定を `~/.claude/statusline/config.json` に保存
2. バイナリが `~/.claude/statusline/bin/` に配置される
3. `~/.claude/settings.json` が自動更新される
4. Claude Codeを再起動。新しいステータスバーを堪能。同僚に自慢。

再実行すると既存の設定がデフォルトとして読み込まれます。非破壊的。約束します。

## 必要なもの

- Claude Codeがインストール済み（`~/.claude/` が存在すること）
- Node.js ≥ 18（`npx` インストール時のみ — 実行時は不要）

## セキュリティとプライバシー

あなたのデータ、勝手に触りません。安心してください：

- **Cryptoセグメント**はBinanceの公開API（`api.binance.com`）にリクエストします — 認証不要、個人データの送信なし
- **Usageセグメント**はmacOSキーチェーンからOAuthトークン（`Claude Code-credentials`）を読み取り、AnthropicのUsage APIに問い合わせます
- **npm postinstall**はGitHub Releasesからプラットフォーム対応のバイナリをダウンロードします — それ以外の外部ダウンロードなし
- すべてのデータは`/tmp/claude-statusline-*`ファイルにローカルキャッシュされます
- テレメトリなし、アナリティクスなし、他のどこにもデータを送信しません
- 詳細は[SECURITY.md](SECURITY.md)をご覧ください

## アンインストール

```bash
# 設定とバイナリを削除
rm -rf ~/.claude/statusline/

# Claude Code設定からステータスラインを削除
# ~/.claude/settings.json を編集して "statusLine" キーを削除

# キャッシュを削除
rm -f /tmp/claude-statusline-*

# パッケージマネージャーからアンインストール
npm uninstall -g cc-statusline-tui
# または: cargo uninstall cc-statusline-tui
# または: brew uninstall cc-statusline
```

## トラブルシューティング

| 問題 | 解決策 |
|------|--------|
| "Binary not found" | `npx cc-statusline-tui` を再実行して再ダウンロード |
| "Is a directory" エラー | `~/.claude/statusline/bin/cc-statusline` がファイルであり、ディレクトリでないことを確認 |
| 暗号通貨が表示されない | `/tmp/claude-statusline-crypto-lock` ディレクトリがあれば削除（古いロック） |
| 変更が反映されない | 設定保存後にClaude Codeを再起動 |

## コントリビューション

バグ発見？新機能のアイデア？[Issueを作成](https://github.com/LokiQ0713/cc-statusline-tui/issues)してください。PRも大歓迎。

## License

MIT
