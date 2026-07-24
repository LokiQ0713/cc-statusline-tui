# cc-statusline-tui

Zero-config statusline for Claude Code. The layout is fixed; running the binary just installs it.

## Project Info

- Package: `cc-statusline-tui`
- GitHub: `https://github.com/LokiQ0713/cc-statusline-tui`
- Registry: crates.io (+ Homebrew tap). npm is no longer used.
- Install: `cargo install cc-statusline-tui` (then run `cc-statusline` once), or `brew install cc-statusline`

## Tech Stack

- Language: Rust (2021 edition)
- Entry: `src/main.rs`
- Dependencies: `serde` / `serde_json` (JSON parsing), `dirs` (home directory), `chrono` (time parsing), `tempfile` (tests)
- System deps: none

## What It Does

Two modes, selected by args:

- `--render` — read the status JSON from stdin, print the ANSI statusline (called by Claude Code every refresh).
- no args — install: copy the binary to `~/.claude/statusline/bin/` and set `statusLine` in `~/.claude/settings.json`. No prompts; the layout is fixed.

Fixed layout:
- Row 1: `model · cost · path · git · context`
- Row 2: `5h · 7d` usage windows + `session id` (no progress bars)

## File Structure

- `src/main.rs` — Entry point: dispatches `--render` (render) vs no-args (install)
- `src/config.rs` — Layout structs (`Config` / `Segments` / per-segment) + path helpers. `Config::default()` IS the fixed layout; there is no config file.
- `src/styles.rs` — ANSI color codes, rainbow/gradient rendering, bar formatting (incl. traffic-light gradient)
- `src/render.rs` — Render pipeline: reads stdin JSON, applies the fixed layout, prints the statusline (model appends `effort.level`)
- `src/install.rs` — No-interaction installer: copy binary + update `settings.json`
- `src/log.rs` — Error logging to `~/.claude/statusline/statusline.log`

## Key Directories

- `~/.claude/statusline/` — Runtime directory
  - `bin` — Compiled binary (copied during install)
  - `statusline.log` — Error log

## Development

```bash
cargo run -- --render   # Test the render pipeline (reads JSON from stdin) — safe
cargo test              # Run all tests
cargo clippy -- -D warnings  # Lint check
```

Note: `cargo run` with no args runs the INSTALLER, which writes to your real
`~/.claude/settings.json`. Use `cargo run -- --render` for testing, or point
`HOME` at a scratch dir when exercising the install path.

## CI/CD & Release

- `.github/workflows/ci.yml` — push/PR to main: `cargo check` + `cargo test` + `cargo clippy -- -D warnings`
- `.github/workflows/release.yml` — `v*` tag: cross-compile 4 targets → GitHub Release (binaries) + crates.io. No npm.
- Secrets (repo Settings → Secrets → Actions): `CARGO_REGISTRY_TOKEN`

### Release a new version

```bash
cargo test && cargo clippy -- -D warnings    # verify locally first
# Bump the version in Cargo.toml (package.version)
cargo check                                   # syncs the version in Cargo.lock
git commit -am "release: vX.Y.Z"
git tag vX.Y.Z
git push origin main --tags                   # triggers release.yml
```

Choose X.Y.Z by semver: patch = bugfix, minor = new feature, major = breaking change.
Version must stay in sync across `Cargo.toml` and `Cargo.lock` (the `cc-statusline-tui` entry).

### Common release failures

- crates.io failed → check `CARGO_REGISTRY_TOKEN`
- GitHub Release missing → ensure `permissions: contents: write` in release.yml
- `upload-artifact`/`download-artifact` strips file permissions → binaries are re-`chmod`ed where needed
- Version mismatch between `Cargo.toml` and `Cargo.lock` → `cargo publish` fails; run `cargo check` after bumping

## Error Handling Convention

All user-facing errors must include an AI analysis hint:

```
Tip: Copy this error to AI for analysis
See https://github.com/LokiQ0713/cc-statusline-tui#troubleshooting
```

## Key Internals

- Binary output: `~/.claude/statusline/bin` (compiled Rust binary)
- Install target: `statusLine` field in `~/.claude/settings.json` (other settings preserved)
- No config file: `render` uses `Config::default()` directly; the layout is fixed in code.
- Model segment appends the reasoning effort (`effort.level` from the stdin JSON) when the model reports one; absent otherwise.
- Context bar uses the `semantic` style: a green→yellow→red traffic-light gradient interpolated in RGB (green ≤20%, green→yellow over 20–50%, yellow→red over 50–70%, solid red ≥70%). Bar length is 14.
- Session segment appends the top-level `session_id` (from the stdin JSON) at the end of row 2, showing only its leading 8-char prefix (a UUID's first group, e.g. `569b37c4`) in the muted `gray` style; hidden when `session_id` is missing, empty, or the wrong type.
