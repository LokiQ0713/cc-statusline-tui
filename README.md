# cc-statusline

[![CI](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/ci.yml)
[![Release](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml/badge.svg)](https://github.com/LokiQ0713/cc-statusline-tui/actions/workflows/release.yml)
[![crates.io](https://img.shields.io/crates/v/cc-statusline-tui)](https://crates.io/crates/cc-statusline-tui)

> Your Claude Code statusline is boring. Let's fix that.

![statusline preview](preview.png)

An opinionated, **zero-config** statusline for Claude Code. Install once — there's nothing to configure.

## Install

### Cargo

```bash
cargo install cc-statusline-tui
cc-statusline          # registers itself in ~/.claude/settings.json
```

### Homebrew

```bash
brew tap LokiQ0713/cc-statusline-tui
brew install cc-statusline
cc-statusline
```

Then restart Claude Code (or start a new session).

## What You Get

A fixed two-row statusline — no prompts, no config file:

```
🔥 Opus4.8 high  $1.20  ~/project  ▓▓▓▓▓▓▓░░░ 60% 600K/1M
5h: 47% 2h30m   7d: 28% 3d5h
```

| Segment | Looks Like | What It Does |
|---------|-----------|--------------|
| Model + effort | `🔥 Opus4.8 high` | Model name, plus reasoning effort (`low`…`max`) when the model reports one |
| Cost | `$0.42` | Session cost so far |
| Path | `~/project` | Current directory |
| Context | `▓▓▓▓░░░ 60% 600K/1M` | Context window: traffic-light bar + % + size |
| Usage 5h / 7d | `5h: 47% 2h30m` | Rate-limit windows: % used + reset countdown |

The context bar is a smooth **green → yellow → red** gradient (green at 0%, yellow at 50%, red at 100%).

## How It Works

1. `cc-statusline` copies its binary to `~/.claude/statusline/bin/`
2. It sets `statusLine` in `~/.claude/settings.json` (your other settings are preserved)
3. On every refresh, Claude Code runs `… --render`, which reads the status JSON on stdin and prints the line

The layout is fixed in the binary, so there is no config file and nothing to edit.

## Requirements

- Claude Code installed (`~/.claude/` exists)
- No runtime dependencies

## Security and Privacy

- The **usage segment** reads rate-limit data directly from Claude Code's native stdin JSON (`rate_limits` field) — no external API calls, no keychain access, no network
- No telemetry, no analytics, no data sent anywhere
- For full details see [SECURITY.md](SECURITY.md)

## Uninstall

```bash
# Remove the binary
rm -rf ~/.claude/statusline/

# Remove the statusline from Claude Code settings:
# edit ~/.claude/settings.json and delete the "statusLine" key

# Uninstall the package
cargo uninstall cc-statusline-tui
# or: brew uninstall cc-statusline
```

## Troubleshooting

| Problem | Fix |
|---------|-----|
| "Binary not found" | Re-run `cc-statusline` to reinstall |
| "Is a directory" error | Check that `~/.claude/statusline/bin/cc-statusline` is a file, not a directory |
| Changes not visible | Restart Claude Code after installing |

## Contributing

Found a bug? Want a feature? [Open an issue](https://github.com/LokiQ0713/cc-statusline-tui/issues). PRs welcome.

## License

MIT
