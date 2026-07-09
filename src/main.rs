//! Entry point for the cc-statusline binary.
//!
//! Modes, selected by arguments:
//! - `--render`: read JSON from stdin, output the ANSI statusline string
//!   (invoked by Claude Code on every status refresh).
//! - `--version` / `-V`: print the version and exit.
//! - `--help` / `-h`: print usage and exit.
//! - No args: install — copy the binary into place and register it as the
//!   `statusLine` command in `~/.claude/settings.json`. The layout is fixed,
//!   so there is nothing to configure and no prompts are shown.

mod config;
mod install;
mod log;
mod render;
mod styles;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // --render takes priority: this is how Claude Code invokes the binary.
    if args.iter().any(|a| a == "--render") {
        render::run();
    } else if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("cc-statusline {}", env!("CARGO_PKG_VERSION"));
    } else if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
    } else if args.is_empty() {
        install::run();
    } else {
        eprintln!("Unknown argument: {}\n", args.join(" "));
        print_help();
        std::process::exit(2);
    }
}

fn print_help() {
    println!(
        "cc-statusline — zero-config statusline for Claude Code\n\
         \n\
         USAGE:\n\
         \x20   cc-statusline            Install (register statusLine in ~/.claude/settings.json)\n\
         \x20   cc-statusline --render   Render the statusline (reads status JSON on stdin)\n\
         \x20   cc-statusline --version  Print version\n\
         \x20   cc-statusline --help     Print this help"
    );
}
