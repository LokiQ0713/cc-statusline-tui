//! Interactive TUI wizard -- fixed-layout installer.
//!
//! Launched when the binary is run without arguments. The statusline layout is
//! fixed (no per-module editing, styling, or reordering); the wizard only lets
//! the user pick a language, previews the resulting statusline, and installs it
//! on confirm.
//!
//! Fixed layout:
//! - Row 1: model  cost  path  context
//! - Row 2: 5h usage  7d usage (no progress bars)
//!
//! Submodules provide reusable TUI components:
//! - `select` / `confirm` -- input prompts
//! - `terminal` -- crossterm abstraction (raw mode, cursor, key reading)
//! - `spinner` -- braille loading animation
//! - `preview` -- statusline preview using sample data

pub mod confirm;
pub mod preview;
pub mod select;
pub mod spinner;
pub mod terminal;

use crate::config::Config;
use crate::i18n::{self, t, SUPPORTED_LANGS};

// ── Public entry point ──────────────────────────────────────────────────────

pub fn run() {
    // Install panic hook to restore terminal state on crash
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::Show);
        default_hook(info);
    }));

    // Preflight check
    if !dirs::home_dir()
        .map(|h| h.join(".claude").exists())
        .unwrap_or(false)
    {
        eprintln!("{}", t("msg.noClaudeCode"));
        eprintln!("{}", t("msg.installClaudeCode"));
        std::process::exit(1);
    }

    // Default language: previous choice if valid, otherwise English.
    let existing_config = crate::config::load_config();
    let default_lang = if !existing_config.lang.is_empty()
        && SUPPORTED_LANGS.contains(&existing_config.lang.as_str())
    {
        existing_config.lang.clone()
    } else {
        "en".to_string()
    };

    let lang_opts: Vec<select::SelectOption> = [
        ("en", "English"),
        ("zh", "\u{4e2d}\u{6587}"),
        ("ja", "\u{65e5}\u{672c}\u{8a9e}"),
        ("ko", "\u{d55c}\u{ad6d}\u{c5b4}"),
        ("es", "Espa\u{f1}ol"),
        ("pt", "Portugu\u{ea}s"),
        ("ru", "\u{420}\u{443}\u{441}\u{441}\u{43a}\u{438}\u{439}"),
    ]
    .into_iter()
    .map(|(value, label)| select::SelectOption {
        value: value.into(),
        label: label.into(),
        hint: None,
    })
    .collect();

    // Language selection → preview → confirm. Back/No returns to language.
    loop {
        let lang = match select::select(
            "Language / \u{8bed}\u{8a00}",
            &lang_opts,
            Some(&default_lang),
            &mut |_| {},
            None,
        ) {
            select::SelectResult::Selected(v) => v,
            _ => std::process::exit(0),
        };

        i18n::set_lang(&lang);
        let config = Config {
            lang: lang.clone(),
            ..Config::default()
        };

        show_header(&config, t("step.confirm"));
        match confirm::confirm(t("prompt.save"), true, None) {
            confirm::ConfirmResult::Yes => {
                do_save(&config);
                return;
            }
            confirm::ConfirmResult::No | confirm::ConfirmResult::Back => continue,
            confirm::ConfirmResult::Cancelled => {
                eprintln!("{}", t("msg.cancelled"));
                std::process::exit(0);
            }
        }
    }
}

// ── UI helpers ──────────────────────────────────────────────────────────────

fn show_header(config: &Config, step_label: &str) {
    terminal::clear_screen();
    println!();
    println!(
        "  \x1b[1mClaude Statusline Configurator\x1b[0m \x1b[2m\u{2014} {}\x1b[0m",
        step_label
    );
    println!("  \x1b[2m{}\x1b[0m", "\u{2500}".repeat(56));

    // Multi-line preview
    let label = t("msg.preview");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let rows = preview::render_rows(config, now);

    if rows.is_empty() {
        println!("  \x1b[2m{}\x1b[0m", label);
    } else {
        // The first printed row carries the label; the rest align under it.
        for (i, row) in rows.iter().enumerate() {
            let prefix = if i == 0 {
                label.to_string()
            } else {
                " ".repeat(label.len())
            };
            println!("  \x1b[2m{}\x1b[0m {}", prefix, row);
        }
    }

    println!("  \x1b[2m{}\x1b[0m", "\u{2500}".repeat(56));
}

fn do_save(config: &Config) {
    let sp = spinner::Spinner::start(t("msg.saving"));
    match crate::install::save_and_apply(config) {
        Ok(()) => {
            sp.stop(t("msg.saved"));
            println!("\n  {}", t("msg.restart"));
        }
        Err(e) => {
            sp.stop(t("msg.saveFailed"));
            eprintln!("  {}", e);
        }
    }
}
