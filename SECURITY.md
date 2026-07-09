# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability, please email **lokiq0713@gmail.com**. Do NOT open a public issue.

You can expect an initial response within **72 hours**.

## Scope

The following components are in scope:

- The CLI tool (`cc-statusline` binary)
- The render pipeline (`--render` mode)
- The install path (no-args mode)

## Network Activity

This tool makes **no outbound network requests**. Usage data comes from the
status JSON that Claude Code pipes to `--render` on stdin (the `rate_limits`
field); nothing is fetched over the network at runtime.

## File System Access

The tool reads and writes the following paths:

- `~/.claude/statusline/bin/` -- compiled binary (copied during install)
- `~/.claude/statusline/statusline.log` -- error log
- `~/.claude/settings.json` -- updates the `statusLine` field

## Data Collection

This tool does **not** collect telemetry, analytics, or any user data. All configuration and cache data stays on your local machine.
