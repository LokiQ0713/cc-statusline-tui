//! Entry point for the cc-statusline binary.
//!
//! Two modes depending on arguments:
//! - `--render`: read JSON from stdin, output the ANSI statusline string
//!   (invoked by Claude Code on every status refresh).
//! - No args: install — copy the binary into place and register it as the
//!   `statusLine` command in `~/.claude/settings.json`. The layout is fixed,
//!   so there is nothing to configure and no prompts are shown.

mod config;
mod install;
mod log;
mod render;
mod styles;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--render") {
        render::run();
    } else {
        install::run();
    }
}
