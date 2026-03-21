# Contributing to cc-statusline

## Development Setup

Prerequisites: Rust toolchain (stable).

```bash
git clone https://github.com/LokiQ0713/cc-statusline.git
cd cc-statusline
cargo build
```

## Common Commands

```bash
cargo run               # Launch the interactive wizard
cargo run -- --render   # Test the render pipeline (reads JSON from stdin)
cargo test              # Run all tests
cargo clippy -- -D warnings  # Lint (warnings are errors)
```

### Testing the Render Pipeline

Pipe Claude Code's status JSON into the render command:

```bash
echo '{"tool_name":"Read","model":"claude-sonnet-4-20250514"}' | cargo run -- --render
```

## Pull Requests

- All tests must pass (`cargo test`).
- Clippy must be clean (`cargo clippy -- -D warnings`).
- Write descriptive commit messages that explain *why*, not just *what*.
- Keep PRs focused -- one logical change per PR.

## Issues

### Bug Reports

Include:

- Operating system and version
- Install method (npm / brew)
- Tool version (`cc-statusline --version`)
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs from `~/.claude/statusline/statusline.log`

### Feature Requests

Describe the use case and motivation. Explain what problem the feature solves before proposing a solution.

## Code Style

- Follow existing patterns in the codebase.
- Clippy warnings are treated as errors in CI.
- Keep functions small and well-named.
- Add tests for new functionality.
