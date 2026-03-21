# cc-statusline

Interactive CLI tool to configure the Claude Code statusline.

## Project Info

- Package: `cc-statusline`
- GitHub: `https://github.com/LokiQ0713/cc-statusline`
- Registry: npm public registry (ships prebuilt Rust binaries via npm postinstall)
- Install: `npx cc-statusline`

## Tech Stack

- Language: Rust (2021 edition)
- Entry: `src/main.rs`
- Dependencies: `serde` / `serde_json` (config serialization), `crossterm` (terminal UI), `ureq` (HTTP), `dirs` (home directory), `chrono` (time parsing), `tempfile` (tests)
- System deps: none (was jq/perl/curl in the JS version)

## File Structure

- `src/main.rs` — Entry point, dispatches `--render` vs wizard
- `src/config.rs` — Config structs, load/save, path helpers
- `src/i18n.rs` — i18n (en/zh/ja/ko/es/pt/ru), static + template translations
- `src/styles.rs` — ANSI color codes, rainbow/gradient rendering, bar formatting
- `src/render.rs` — Render pipeline: reads stdin JSON, outputs formatted statusline
- `src/cache.rs` — Stale-while-revalidate cache for crypto prices and usage data
- `src/log.rs` — Error logging to `~/.claude/statusline/statusline.log`
- `src/install.rs` — Save config, copy binary, update settings.json
- `src/wizard/` — Interactive TUI wizard
  - `mod.rs` — Wizard state machine (4-step flow)
  - `terminal.rs` — Terminal control (raw mode, cursor, key reading)
  - `select.rs` — Single-select component
  - `multiselect.rs` — Multi-select component
  - `confirm.rs` — Yes/No confirmation
  - `spinner.rs` — Loading spinner
  - `preview.rs` — Live preview rendering
  - `step_progress.rs` — Step progress indicator

## npm Distribution

- `package.json` — npm package manifest (thin shell, no Rust source)
- `cli.js` — Forwards execution to `~/.claude/statusline/bin/cc-statusline`
- `postinstall.js` — Downloads platform-specific binary from GitHub Releases on `npm install`

## Key Directories

- `~/.claude/statusline/` — Runtime directory
  - `config.json` — User configuration
  - `bin` — Compiled binary (copied during install)
  - `statusline.log` — Error log
- `/tmp/claude-statusline-*` — Cache files (crypto prices, usage data)

## Development

```bash
cargo run               # Run wizard
cargo run -- --render   # Test render pipeline (reads JSON from stdin)
cargo test              # Run all tests
cargo clippy -- -D warnings  # Lint check
```

## Versioning

Semantic versioning: MAJOR.MINOR.PATCH

- PATCH: bug fixes, copy/style tweaks
- MINOR: new segment types, new config options
- MAJOR: config format changes, binary interface changes

## Release Workflow

### Steps

```bash
# 1. Verify locally
cargo test && cargo clippy -- -D warnings

# 2. Bump version in Cargo.toml, then create git commit + tag
npm version patch      # or minor / major

# 3. Push code and tag → triggers GitHub Actions auto-publish
git push origin main --tags
```

### How it works

1. `npm version` bumps `package.json`, creates a commit and `v*` tag
2. `git push --tags` pushes the tag to GitHub
3. GitHub Actions `release.yml` triggers on `v*` tags
4. Workflow runs `npm publish` using `NPM_TOKEN` secret
5. Workflow creates a GitHub Release via `gh release create`

### Verify publish succeeded

```bash
# Check GitHub Actions status
gh run list --repo LokiQ0713/cc-statusline --workflow=release.yml --limit 3

# Check npm registry
npm view cc-statusline version

# Check GitHub Release was created
gh release list --repo LokiQ0713/cc-statusline
```

### Troubleshooting publish failures

| Error | Cause | Fix |
|-------|-------|-----|
| `ENEEDAUTH` | `NPM_TOKEN` secret missing or invalid | Add/update token in GitHub repo Settings → Secrets → Actions |
| `E403 previously published` | Version already exists on npm | Bump version again with `npm version patch` |
| GitHub Release not created | `contents: write` permission missing | Check `release.yml` has `permissions: contents: write` |

## CI/CD

- `.github/workflows/ci.yml` — push/PR to main, Rust build + test + clippy
- `.github/workflows/release.yml` — `v*` tag trigger, auto-publish to npm + GitHub Release
- Required secret: `NPM_TOKEN` (npm Automation token, add in GitHub repo Settings → Secrets → Actions)

## Error Handling Convention

All user-facing errors must include an AI analysis hint:

```
Tip: Copy this error to AI for analysis
See https://github.com/LokiQ0713/cc-statusline#troubleshooting
```

## Key Internals

- Binary output: `~/.claude/statusline/bin` (compiled Rust binary)
- Config file: `~/.claude/statusline/config.json`
- Auto-updates: `statusLine` field in `~/.claude/settings.json`
