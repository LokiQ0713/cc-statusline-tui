# cc-statusline

[![CI](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml)
[![Release](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml)
[![npm](https://img.shields.io/npm/v/cc-statusline-tui)](https://www.npmjs.com/package/cc-statusline-tui)
[![crates.io](https://img.shields.io/crates/v/cc-statusline-tui)](https://crates.io/crates/cc-statusline-tui)

> Your Claude Code statusline is boring. Let's fix that.

[中文](README.zh.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Español](README.es.md) | [Português](README.pt.md) | [Русский](README.ru.md)

![statusline preview](preview.png)

One command. Seven languages. Zero regrets.

## Install

Pick your poison:

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

## What Happens

A TUI wizard pops up. You pick stuff. It looks cool. That's it.

```
1/4 Segments      → What to show
2/4 Configure     → How it looks (live preview, instant feedback)
3/4 Reorder       → Where things go
4/4 Confirm       → Ship it
```

Too lazy to customize? Pick "Use defaults" and press Enter twice. We don't judge.

## Segments

| Segment | Looks Like | What It Does | You Can Tweak |
|---------|-----------|--------------|---------------|
| Model | `🔥 Opus4.6` | Shows which brain you're burning through | Icon (🔥🦊🐢🐰), color |
| Cost | `$0.42` | Your wallet's pain level | Color |
| Usage | `██░░ 25% 1h43m` | 5h rate limit: bar + % + reset countdown | Bar style/char/length, color, refresh |
| Path | `~/project` | Where you are | Color, max length |
| Git | `main* ↑2↓1` | Branch + dirty flag + ahead/behind | Components, color |
| Context | `▓▓▓▓░░░ 60% 600K/1M` | How much context window you've eaten | Bar style/char/length, color |
| Crypto | `BTC:$73748` | Because why not check prices while coding | Coins (BTC/ETH/BNB/SOL), color, refresh |

## Color Styles

| Style | Vibe |
|-------|------|
| **Ultrathink Rainbow** | Every character is a different color. Yes, it shimmers. |
| **Ultrathink Gradient** | Smooth rainbow across the bar. Chef's kiss. |
| **Traffic Light** | Green (≤30%) → Yellow (≤60%) → Red (>60%). Your context window is a traffic jam. |
| Cyan / Green / Blue / Yellow | The sensible choices |
| Magenta / Red / Orange / Pink / Purple / White | The expressive choices |

## Languages

English, 中文, 日本語, 한국어, Español, Português, Русский.

Selected on first run. Your statusline speaks your language.

## How It Works

1. Wizard saves config to `~/.claude/statusline/config.json`
2. Binary goes to `~/.claude/statusline/bin/`
3. `~/.claude/settings.json` gets updated automatically
4. Restart Claude Code. Admire your new statusline. Tell your coworkers.

Re-running the wizard loads your existing config as defaults. Non-destructive. Promise.

## Requirements

- Claude Code installed (`~/.claude/` exists)
- Node.js ≥ 18 (only for `npx` install — not needed at runtime or for `cargo install`)

## Security and Privacy

- The **crypto segment** makes requests to Binance public API (`api.binance.com`) — no authentication required, no personal data sent
- The **usage segment** reads an OAuth token from the macOS keychain (`Claude Code-credentials`) to query Anthropic's usage API
- The **npm postinstall** downloads a platform-specific binary from GitHub Releases — no other external downloads
- All data is cached locally in `/tmp/claude-statusline-*` files
- No telemetry, no analytics, no data sent anywhere else
- For full details see [SECURITY.md](SECURITY.md)

## Uninstall

```bash
# Remove config and binary
rm -rf ~/.claude/statusline/

# Remove statusline from Claude Code settings
# Edit ~/.claude/settings.json and delete the "statusLine" key

# Remove cache
rm -f /tmp/claude-statusline-*

# Uninstall from package manager
npm uninstall -g cc-statusline-tui
# or: cargo uninstall cc-statusline-tui
# or: brew uninstall cc-statusline
```

## Troubleshooting

| Problem | Fix |
|---------|-----|
| "Binary not found" | Run `npx cc-statusline-tui` again to re-download |
| "Is a directory" error | Check that `~/.claude/statusline/bin/cc-statusline` is a file, not a directory |
| Crypto not showing | Delete `/tmp/claude-statusline-crypto-lock` directory if it exists (stale lock) |
| Changes not visible | Restart Claude Code after saving configuration |

## Contributing

Found a bug? Want a feature? [Open an issue](https://github.com/LokiQ0713/cc-statusline-tui/issues). PRs welcome.

## License

MIT
