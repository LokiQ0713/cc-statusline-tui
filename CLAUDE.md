# cc-statusline-tui

Interactive CLI tool to configure the Claude Code statusline.

## Project Info

- Package: `cc-statusline-tui`
- GitHub: `https://github.com/LokiQ0713/cc-statusline-tui`
- Registry: npm public registry (ships prebuilt Rust binaries as platform-specific optionalDependencies, no postinstall) + crates.io
- Install: `npx cc-statusline-tui` or `cargo install cc-statusline-tui`

## Tech Stack

- Language: Rust (2021 edition)
- Entry: `src/main.rs`
- Dependencies: `serde` / `serde_json` (config serialization), `crossterm` (terminal UI), `dirs` (home directory), `chrono` (time parsing), `tempfile` (tests)
- System deps: none (was jq/perl/curl in the JS version)

## File Structure

- `src/main.rs` — Entry point, dispatches `--render` vs wizard
- `src/config.rs` — Config structs, load/save, path helpers
- `src/i18n.rs` — i18n (en/zh/ja/ko/es/pt/ru), static translations
- `src/styles.rs` — ANSI color codes, rainbow/gradient rendering, bar formatting (incl. traffic-light gradient)
- `src/render.rs` — Render pipeline: reads stdin JSON, outputs formatted statusline (model appends `effort.level`)
- `src/log.rs` — Error logging to `~/.claude/statusline/statusline.log`
- `src/install.rs` — Save config, copy binary, update settings.json
- `src/wizard/` — Interactive TUI wizard (fixed layout — language → preview → confirm → install)
  - `mod.rs` — Fixed-layout installer flow
  - `terminal.rs` — Terminal control (raw mode, cursor, key reading)
  - `select.rs` — Single-select component
  - `confirm.rs` — Yes/No confirmation
  - `spinner.rs` — Loading spinner
  - `preview.rs` — Preview rendering (sample data)

## npm Distribution (esbuild-style platform packages)

- `package.json` — Main npm package (thin wrapper with optionalDependencies)
- `cli.js` — Resolves platform binary from node_modules, executes it
- `npm/` — Platform package templates (binary added by CI)
  - `darwin-arm64/package.json` — macOS ARM64
  - `darwin-x64/package.json` — macOS x64
  - `linux-x64/package.json` — Linux x64
  - `linux-arm64/package.json` — Linux ARM64
- npm auto-installs only the matching platform package (via `os`/`cpu` fields)
- No postinstall scripts, no runtime downloads

## Key Directories

- `~/.claude/statusline/` — Runtime directory
  - `config.json` — User configuration
  - `bin` — Compiled binary (copied during install)
  - `statusline.log` — Error log

## Development

```bash
cargo run               # Run wizard
cargo run -- --render   # Test render pipeline (reads JSON from stdin)
cargo test              # Run all tests
cargo clippy -- -D warnings  # Lint check
```

## CI/CD & Release

- `.github/workflows/ci.yml` — push/PR to main: `cargo check` + `cargo test` + `cargo clippy -- -D warnings`
- `.github/workflows/release.yml` — `v*` tag: cross-compile 4 targets, publish to npm + crates.io + GitHub Release
- Secrets (repo Settings → Secrets → Actions): `NPM_TOKEN`, `CARGO_REGISTRY_TOKEN`

### Release a new version

`npm version` does NOT sync the Rust side (there are no `scripts` in `package.json`), so bump the version manually across every file, then tag and push:

```bash
cargo test && cargo clippy -- -D warnings    # verify locally first
# Bump to X.Y.Z in ALL of:
#   - Cargo.toml    → package.version
#   - package.json  → "version" AND all 4 optionalDependencies
cargo check                                   # syncs the version in Cargo.lock
git commit -am "release: vX.Y.Z"
git tag vX.Y.Z
git push origin main --tags                   # triggers release.yml
```

Choose X.Y.Z by semver: patch = bugfix, minor = new feature, major = breaking (removed/renamed feature or changed config schema).

Version must stay in sync across: `Cargo.toml`, `Cargo.lock` (the `cc-statusline-tui` entry), `package.json` (`version` + all `optionalDependencies`). `npm/*/package.json` platform versions are written by CI at release time.

### Common release failures

- `ENEEDAUTH` / `E403` → check `NPM_TOKEN`; a published version can't be overwritten, so bump instead
- crates.io failed → check `CARGO_REGISTRY_TOKEN`
- GitHub Release missing → ensure `permissions: contents: write` in release.yml
- `upload-artifact`/`download-artifact` strips file permissions → publish-npm has explicit `chmod +x`
- Version mismatch across `Cargo.toml` / `Cargo.lock` / `package.json` → publish jobs fail; bump all of them together

## Error Handling Convention

All user-facing errors must include an AI analysis hint:

```
Tip: Copy this error to AI for analysis
See https://github.com/LokiQ0713/cc-statusline-tui#troubleshooting
```

## Key Internals

- Binary output: `~/.claude/statusline/bin` (compiled Rust binary)
- Config file: `~/.claude/statusline/config.json`
- Auto-updates: `statusLine` field in `~/.claude/settings.json`
- Fixed layout (not user-reorderable): row 1 = model · cost · path · context; row 2 = 5h · 7d usage (no bars). The wizard only picks a language, then installs; `render` still reads `rows` from `config.json`, so the layout lives in `Config::default()`.
- Model segment appends the reasoning effort (`effort.level` from the stdin JSON) when the model reports one; absent otherwise.
- Context bar uses the `semantic` style: a green→yellow→red traffic-light gradient interpolated in RGB (green at 0%, yellow at 50%, red at 100%).
