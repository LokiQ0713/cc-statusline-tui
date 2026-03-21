# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability, please email **lokiq0713@gmail.com**. Do NOT open a public issue.

You can expect an initial response within **72 hours**.

## Scope

The following components are in scope:

- The CLI tool (`cc-statusline` binary)
- The npm postinstall script (`postinstall.js`)
- The render pipeline (`--render` mode)

## Network Activity

This tool makes outbound network requests to the following services:

| Service | Purpose | Authentication |
|---------|---------|----------------|
| Binance public API | Fetch cryptocurrency prices for the crypto segment | None |
| Anthropic OAuth API | Fetch API usage data for the usage segment | Reads OAuth token from macOS Keychain |
| GitHub Releases | Download platform-specific binary during `npm postinstall` | None |

No other network requests are made.

## File System Access

The tool reads and writes the following paths:

- `~/.claude/statusline/config.json` -- user configuration
- `~/.claude/statusline/bin/` -- compiled binary (copied during install)
- `~/.claude/statusline/statusline.log` -- error log
- `~/.claude/settings.json` -- updates the `statusLine` field
- `/tmp/claude-statusline-*` -- cache files for crypto prices and usage data

## Data Collection

This tool does **not** collect telemetry, analytics, or any user data. All configuration and cache data stays on your local machine.
