//! Entry point for the cc-statusline binary.
//!
//! This single binary serves two purposes depending on arguments:
//! - No args: launches the interactive TUI wizard (`wizard::run`)
//! - `--render`: reads JSON from stdin and outputs an ANSI-colored statusline
//!   string (`render::run`), invoked by Claude Code on every status refresh.
//!
//! All configuration, rendering, and installation logic is delegated to
//! submodules. See each module's doc comment for details.

mod cache;
mod config;
mod i18n;
mod install;
mod log;
mod render;
mod styles;
mod wizard;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--render") {
        render::run();
    } else {
        wizard::run();
    }
}
