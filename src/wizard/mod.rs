pub mod confirm;
pub mod multiselect;
pub mod preview;
pub mod select;
pub mod spinner;
pub mod step_progress;
pub mod terminal;

use crate::config::{Config, CRYPTO_LIST};
use crate::i18n::{self, t, tf, SUPPORTED_LANGS};

const PREVIEW_ROW: u16 = 3;

/// All seven segment keys in canonical order.
const ALL_SEGMENTS: &[&str] = &["model", "cost", "usage", "path", "git", "context", "crypto"];

// ── Step result ─────────────────────────────────────────────────────────────

enum StepResult {
    Next,
    Back,
    Cancelled,
}

// ── Public entry point ──────────────────────────────────────────────────────

pub fn run() {
    // Preflight check
    if !dirs::home_dir()
        .map(|h| h.join(".claude").exists())
        .unwrap_or(false)
    {
        eprintln!("{}", t("msg.noClaudeCode"));
        eprintln!("{}", t("msg.installClaudeCode"));
        std::process::exit(1);
    }

    let mut config = crate::config::load_config();

    // Step 0: Language selection (no back)
    if config.lang.is_empty() || !SUPPORTED_LANGS.contains(&config.lang.as_str()) {
        let opts = vec![
            select::SelectOption {
                value: "en".into(),
                label: "English".into(),
                hint: None,
            },
            select::SelectOption {
                value: "zh".into(),
                label: "中文".into(),
                hint: None,
            },
        ];
        match select::select("Language / 语言", &opts, Some("en"), &mut |_| {}) {
            select::SelectResult::Selected(v) => {
                config.lang = v.clone();
                i18n::set_lang(&v);
            }
            _ => {
                std::process::exit(0);
            }
        }
    } else {
        i18n::set_lang(&config.lang);
    }

    // Step 0b: Mode selection (defaults vs custom) — loop to support Back
    loop {
        show_header(&config, t("step.start"));
        let mode_opts = vec![
            select::SelectOption {
                value: "defaults".into(),
                label: t("mode.defaults").into(),
                hint: Some(t("mode.defaultsHint").into()),
            },
            select::SelectOption {
                value: "custom".into(),
                label: t("mode.custom").into(),
                hint: Some(t("mode.customHint").into()),
            },
        ];
        match select::select(t("mode.prompt"), &mode_opts, Some("defaults"), &mut |_| {}) {
            select::SelectResult::Selected(v) if v == "defaults" => {
                let defaults = Config { lang: config.lang.clone(), ..Config::default() };
                show_header(&defaults, t("step.confirm"));
                match confirm::confirm(t("prompt.saveDefaults"), true) {
                    confirm::ConfirmResult::Yes => {
                        do_save(&defaults);
                        return;
                    }
                    confirm::ConfirmResult::Cancelled => {
                        std::process::exit(0);
                    }
                    _ => continue, // Back or No — go back to mode selection
                }
            }
            select::SelectResult::Selected(_) => break, // custom mode — continue to steps
            _ => {
                std::process::exit(0);
            }
        }
    }

    // Define the 4 major steps
    let mut steps = vec![
        step_progress::StepInfo {
            label: t("step.segments").to_string(),
            summary: None,
        },
        step_progress::StepInfo {
            label: tf("step.configSegment", &["1", "?"]),
            summary: None,
        },
        step_progress::StepInfo {
            label: t("step.reorder").to_string(),
            summary: None,
        },
        step_progress::StepInfo {
            label: t("step.confirm").to_string(),
            summary: None,
        },
    ];

    let mut current_step: usize = 0;
    let mut snapshots: Vec<Config> = Vec::new();

    loop {
        // Save snapshot before step
        if snapshots.len() <= current_step {
            snapshots.push(config.clone());
        }

        let result = match current_step {
            0 => step_select_segments(&mut config, &mut steps),
            1 => step_configure_segments(&mut config, &mut steps),
            2 => step_reorder(&mut config, &mut steps),
            3 => step_confirm(&config, &steps),
            _ => break,
        };

        match result {
            StepResult::Next => {
                current_step += 1;
                if current_step > 3 {
                    do_save(&config);
                    return;
                }
            }
            StepResult::Back => {
                if current_step > 0 {
                    current_step -= 1;
                    config = snapshots[current_step].clone();
                    snapshots.truncate(current_step);
                    steps[current_step].summary = None;
                }
            }
            StepResult::Cancelled => {
                eprintln!("{}", t("msg.cancelled"));
                std::process::exit(0);
            }
        }
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn show_header(config: &Config, step_label: &str) {
    terminal::clear_screen();
    println!();
    println!(
        "  \x1b[1mClaude Statusline Configurator\x1b[0m \x1b[2m— {}\x1b[0m",
        step_label
    );
    println!("  \x1b[2m{}\x1b[0m", "─".repeat(56));
    println!(
        "  \x1b[2m{}\x1b[0m {}",
        t("msg.preview"),
        preview::render_preview(config)
    );
    println!("  \x1b[2m{}\x1b[0m", "─".repeat(56));
}

fn show_screen(
    config: &Config,
    steps: &[step_progress::StepInfo],
    current_step: usize,
    step_label: &str,
) {
    show_header(config, step_label);
    println!(
        "{}",
        step_progress::render_step_progress(steps, current_step)
    );
    println!();
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

// ── Option builders ────────────────────────────────────────────────────────

fn style_options() -> Vec<select::SelectOption> {
    vec![
        select::SelectOption {
            value: "ultrathink".into(),
            label: t("style.rainbow").into(),
            hint: None,
        },
        select::SelectOption {
            value: "cyan".into(),
            label: t("style.cyan").into(),
            hint: None,
        },
        select::SelectOption {
            value: "green".into(),
            label: t("style.green").into(),
            hint: None,
        },
        select::SelectOption {
            value: "blue".into(),
            label: t("style.blue").into(),
            hint: None,
        },
        select::SelectOption {
            value: "yellow".into(),
            label: t("style.yellow").into(),
            hint: None,
        },
        select::SelectOption {
            value: "magenta".into(),
            label: t("style.magenta").into(),
            hint: None,
        },
        select::SelectOption {
            value: "red".into(),
            label: t("style.red").into(),
            hint: None,
        },
        select::SelectOption {
            value: "white".into(),
            label: t("style.white").into(),
            hint: None,
        },
        select::SelectOption {
            value: "orange".into(),
            label: t("style.orange").into(),
            hint: None,
        },
        select::SelectOption {
            value: "pink".into(),
            label: t("style.pink").into(),
            hint: None,
        },
        select::SelectOption {
            value: "purple".into(),
            label: t("style.purple").into(),
            hint: None,
        },
    ]
}

fn bar_style_options() -> Vec<select::SelectOption> {
    vec![
        select::SelectOption {
            value: "ultrathink-gradient".into(),
            label: t("barStyle.gradient").into(),
            hint: None,
        },
        select::SelectOption {
            value: "semantic".into(),
            label: t("barStyle.trafficLight").into(),
            hint: None,
        },
        select::SelectOption {
            value: "cyan".into(),
            label: t("style.cyan").into(),
            hint: None,
        },
        select::SelectOption {
            value: "green".into(),
            label: t("style.green").into(),
            hint: None,
        },
        select::SelectOption {
            value: "blue".into(),
            label: t("style.blue").into(),
            hint: None,
        },
        select::SelectOption {
            value: "yellow".into(),
            label: t("style.yellow").into(),
            hint: None,
        },
        select::SelectOption {
            value: "magenta".into(),
            label: t("style.magenta").into(),
            hint: None,
        },
        select::SelectOption {
            value: "orange".into(),
            label: t("style.orange").into(),
            hint: None,
        },
        select::SelectOption {
            value: "pink".into(),
            label: t("style.pink").into(),
            hint: None,
        },
        select::SelectOption {
            value: "purple".into(),
            label: t("style.purple").into(),
            hint: None,
        },
    ]
}

fn bar_char_options() -> Vec<select::SelectOption> {
    vec![
        select::SelectOption {
            value: "shade".into(),
            label: t("char.shade").into(),
            hint: None,
        },
        select::SelectOption {
            value: "full-block".into(),
            label: t("char.fullBlock").into(),
            hint: None,
        },
        select::SelectOption {
            value: "rectangle".into(),
            label: t("char.rectangle").into(),
            hint: None,
        },
    ]
}

fn bar_length_options() -> Vec<select::SelectOption> {
    vec![
        select::SelectOption {
            value: "6".into(),
            label: format!("6 {}", t("unit.chars")),
            hint: None,
        },
        select::SelectOption {
            value: "8".into(),
            label: format!("8 {}", t("unit.chars")),
            hint: None,
        },
        select::SelectOption {
            value: "10".into(),
            label: format!("10 {}", t("unit.chars")),
            hint: None,
        },
        select::SelectOption {
            value: "12".into(),
            label: format!("12 {}", t("unit.chars")),
            hint: None,
        },
        select::SelectOption {
            value: "16".into(),
            label: format!("16 {}", t("unit.chars")),
            hint: None,
        },
    ]
}

fn refresh_interval_options() -> Vec<select::SelectOption> {
    vec![
        select::SelectOption {
            value: "30".into(),
            label: format!("30{}", t("unit.seconds")),
            hint: None,
        },
        select::SelectOption {
            value: "60".into(),
            label: format!("60{}", t("unit.seconds")),
            hint: None,
        },
        select::SelectOption {
            value: "120".into(),
            label: format!("120{}", t("unit.seconds")),
            hint: None,
        },
        select::SelectOption {
            value: "300".into(),
            label: format!("300{}", t("unit.seconds")),
            hint: None,
        },
    ]
}

fn max_length_options() -> Vec<select::SelectOption> {
    vec![
        select::SelectOption {
            value: "10".into(),
            label: format!("10 {}", t("unit.chars")),
            hint: None,
        },
        select::SelectOption {
            value: "15".into(),
            label: format!("15 {}", t("unit.chars")),
            hint: None,
        },
        select::SelectOption {
            value: "20".into(),
            label: format!("20 {}", t("unit.chars")),
            hint: None,
        },
        select::SelectOption {
            value: "30".into(),
            label: format!("30 {}", t("unit.chars")),
            hint: None,
        },
    ]
}

fn icon_options() -> Vec<select::SelectOption> {
    vec![
        select::SelectOption {
            value: "\u{1f525}".into(),
            label: "\u{1f525} Fire".into(),
            hint: None,
        },
        select::SelectOption {
            value: "\u{1f916}".into(),
            label: "\u{1f916} Robot".into(),
            hint: None,
        },
        select::SelectOption {
            value: "\u{2728}".into(),
            label: "\u{2728} Sparkles".into(),
            hint: None,
        },
        select::SelectOption {
            value: "\u{26a1}".into(),
            label: "\u{26a1} Lightning".into(),
            hint: None,
        },
        select::SelectOption {
            value: "".into(),
            label: "(none)".into(),
            hint: None,
        },
    ]
}

/// Returns the i18n label for a segment key.
fn seg_label(key: &str) -> &'static str {
    match key {
        "model" => t("seg.model"),
        "cost" => t("seg.cost"),
        "usage" => t("seg.usage"),
        "path" => t("seg.path"),
        "git" => t("seg.git"),
        "context" => t("seg.context"),
        "crypto" => t("seg.crypto"),
        _ => "?",
    }
}

/// Returns the sample hint for a segment (used in the segment multiselect).
fn seg_hint(key: &str) -> &'static str {
    match key {
        "model" => "Opus4.6",
        "cost" => "$0.42",
        "usage" => "25%",
        "path" => "~/Desktop",
        "git" => "main*",
        "context" => "60% 600K/1M",
        "crypto" => "BTC:$73748",
        _ => "",
    }
}

/// Return enabled segment keys in the order they appear in config.order.
fn enabled_keys(config: &Config) -> Vec<String> {
    config
        .order
        .iter()
        .filter(|k| is_seg_enabled(config, k))
        .cloned()
        .collect()
}

fn is_seg_enabled(config: &Config, key: &str) -> bool {
    match key {
        "model" => config.segments.model.enabled,
        "cost" => config.segments.cost.enabled,
        "usage" => config.segments.usage.enabled,
        "path" => config.segments.path.enabled,
        "git" => config.segments.git.enabled,
        "context" => config.segments.context.enabled,
        "crypto" => config.segments.crypto.enabled,
        _ => false,
    }
}

fn set_seg_enabled(config: &mut Config, key: &str, enabled: bool) {
    match key {
        "model" => config.segments.model.enabled = enabled,
        "cost" => config.segments.cost.enabled = enabled,
        "usage" => config.segments.usage.enabled = enabled,
        "path" => config.segments.path.enabled = enabled,
        "git" => config.segments.git.enabled = enabled,
        "context" => config.segments.context.enabled = enabled,
        "crypto" => config.segments.crypto.enabled = enabled,
        _ => {}
    }
}

// ── Step 1: Select segments ────────────────────────────────────────────────

fn step_select_segments(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
) -> StepResult {
    show_screen(config, steps, 0, t("step.segments"));

    let options: Vec<multiselect::MultiselectOption> = ALL_SEGMENTS
        .iter()
        .map(|&key| multiselect::MultiselectOption {
            value: key.into(),
            label: seg_label(key).into(),
            hint: Some(seg_hint(key).into()),
        })
        .collect();

    let initial: Vec<String> = ALL_SEGMENTS
        .iter()
        .filter(|&&k| is_seg_enabled(config, k))
        .map(|&k| k.into())
        .collect();

    let config_ptr = config as *mut Config;

    let result = multiselect::multiselect(
        t("prompt.selectSegments"),
        &options,
        &initial,
        true, // at least one required
        &mut |selected: &[String]| {
            let cfg = unsafe { &mut *config_ptr };
            for &key in ALL_SEGMENTS {
                set_seg_enabled(cfg, key, selected.contains(&key.to_string()));
            }
            preview::update_preview_in_place(cfg, PREVIEW_ROW);
        },
    );

    match result {
        multiselect::MultiselectResult::Selected(selected) => {
            for &key in ALL_SEGMENTS {
                set_seg_enabled(config, key, selected.contains(&key.to_string()));
            }
            // Update order: keep existing order for enabled segments, then add any new ones
            let mut new_order: Vec<String> = config
                .order
                .iter()
                .filter(|k| selected.contains(k))
                .cloned()
                .collect();
            for k in &selected {
                if !new_order.contains(k) {
                    new_order.push(k.clone());
                }
            }
            config.order = new_order;

            // Build summary
            let names: Vec<&str> = enabled_keys(config)
                .iter()
                .map(|k| seg_label(k))
                .collect();
            steps[0].summary = Some(tf("done.segments", &[&names.join(", ")]));

            StepResult::Next
        }
        multiselect::MultiselectResult::Back => StepResult::Back,
        multiselect::MultiselectResult::Cancelled => StepResult::Cancelled,
    }
}

// ── Step 2: Configure each segment ─────────────────────────────────────────

fn step_configure_segments(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
) -> StepResult {
    let keys = enabled_keys(config);
    let total = keys.len();
    let mut sub_idx: usize = 0;

    while sub_idx < total {
        let key = &keys[sub_idx];
        let sub_label = tf(
            "step.configSegment",
            &[&(sub_idx + 1).to_string(), &total.to_string()],
        );
        steps[1].label = sub_label.clone();

        let result = configure_segment(config, steps, key, &sub_label);

        match result {
            StepResult::Next => {
                sub_idx += 1;
            }
            StepResult::Back => {
                if sub_idx == 0 {
                    return StepResult::Back;
                }
                sub_idx -= 1;
            }
            StepResult::Cancelled => {
                return StepResult::Cancelled;
            }
        }
    }

    // Mark step as done
    steps[1].summary = Some("\u{2714}".to_string());

    StepResult::Next
}

fn configure_segment(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
    key: &str,
    step_label: &str,
) -> StepResult {
    match key {
        "model" => configure_model(config, steps, step_label),
        "cost" => configure_cost(config, steps, step_label),
        "usage" => configure_usage(config, steps, step_label),
        "path" => configure_path(config, steps, step_label),
        "git" => configure_git(config, steps, step_label),
        "context" => configure_context(config, steps, step_label),
        "crypto" => configure_crypto(config, steps, step_label),
        _ => StepResult::Next,
    }
}

// ── Model configuration ─────────────────────────────────────────────────

fn configure_model(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
    step_label: &str,
) -> StepResult {
    // Sub-step 0: style
    // Sub-step 1: icon
    let mut sub: usize = 0;
    loop {
        match sub {
            0 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("model"), t("prompt.style"));
                let opts = style_options();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&config.segments.model.style),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.model.style = v.to_string();
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        config.segments.model.style = v;
                        sub = 1;
                    }
                    select::SelectResult::Back => {
                        if sub == 0 {
                            return StepResult::Back;
                        }
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            1 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — Icon", seg_label("model"));
                let opts = icon_options();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&config.segments.model.icon),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.model.icon = v.to_string();
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        config.segments.model.icon = v;
                        return StepResult::Next;
                    }
                    select::SelectResult::Back => {
                        sub = 0;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            _ => return StepResult::Next,
        }
    }
}

// ── Cost configuration ──────────────────────────────────────────────────

fn configure_cost(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
    step_label: &str,
) -> StepResult {
    show_screen(config, steps, 1, step_label);
    let prompt = format!("{} — {}", seg_label("cost"), t("prompt.style"));
    let opts = style_options();
    let config_ptr = config as *mut Config;
    let result = select::select(
        &prompt,
        &opts,
        Some(&config.segments.cost.style),
        &mut |v: &str| {
            let cfg = unsafe { &mut *config_ptr };
            cfg.segments.cost.style = v.to_string();
            preview::update_preview_in_place(cfg, PREVIEW_ROW);
        },
    );
    match result {
        select::SelectResult::Selected(v) => {
            config.segments.cost.style = v;
            StepResult::Next
        }
        select::SelectResult::Back => StepResult::Back,
        select::SelectResult::Cancelled => StepResult::Cancelled,
    }
}

// ── Usage configuration ─────────────────────────────────────────────────

fn configure_usage(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
    step_label: &str,
) -> StepResult {
    // Sub-steps: 0=parts, 1=style, 2=barChar, 3=barLength, 4=refresh
    let mut sub: usize = 0;
    loop {
        match sub {
            0 => {
                // Parts multiselect
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("usage"), t("prompt.showParts"));
                let opts = vec![
                    multiselect::MultiselectOption {
                        value: "bar".into(),
                        label: t("part.bar").into(),
                        hint: None,
                    },
                    multiselect::MultiselectOption {
                        value: "percent".into(),
                        label: t("part.percent").into(),
                        hint: None,
                    },
                    multiselect::MultiselectOption {
                        value: "reset".into(),
                        label: t("part.reset").into(),
                        hint: None,
                    },
                ];
                let mut initial = Vec::new();
                if config.segments.usage.show_bar {
                    initial.push("bar".to_string());
                }
                if config.segments.usage.show_percent {
                    initial.push("percent".to_string());
                }
                if config.segments.usage.show_reset {
                    initial.push("reset".to_string());
                }
                let config_ptr = config as *mut Config;
                let result = multiselect::multiselect(
                    &prompt,
                    &opts,
                    &initial,
                    true,
                    &mut |selected: &[String]| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.usage.show_bar = selected.contains(&"bar".to_string());
                        cfg.segments.usage.show_percent =
                            selected.contains(&"percent".to_string());
                        cfg.segments.usage.show_reset = selected.contains(&"reset".to_string());
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    multiselect::MultiselectResult::Selected(selected) => {
                        config.segments.usage.show_bar = selected.contains(&"bar".to_string());
                        config.segments.usage.show_percent =
                            selected.contains(&"percent".to_string());
                        config.segments.usage.show_reset =
                            selected.contains(&"reset".to_string());
                        sub = 1;
                    }
                    multiselect::MultiselectResult::Back => return StepResult::Back,
                    multiselect::MultiselectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            1 => {
                // Bar style (only if bar is shown)
                if config.segments.usage.show_bar {
                    show_screen(config, steps, 1, step_label);
                    let prompt = format!("{} — {}", seg_label("usage"), t("prompt.barStyle"));
                    let opts = bar_style_options();
                    let config_ptr = config as *mut Config;
                    let result = select::select(
                        &prompt,
                        &opts,
                        Some(&config.segments.usage.style),
                        &mut |v: &str| {
                            let cfg = unsafe { &mut *config_ptr };
                            cfg.segments.usage.style = v.to_string();
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        },
                    );
                    match result {
                        select::SelectResult::Selected(v) => {
                            config.segments.usage.style = v;
                            sub = 2;
                        }
                        select::SelectResult::Back => {
                            sub = 0;
                        }
                        select::SelectResult::Cancelled => return StepResult::Cancelled,
                    }
                } else {
                    // Text style (no bar shown)
                    show_screen(config, steps, 1, step_label);
                    let prompt = format!("{} — {}", seg_label("usage"), t("prompt.textStyle"));
                    let opts = style_options();
                    let config_ptr = config as *mut Config;
                    let result = select::select(
                        &prompt,
                        &opts,
                        Some(&config.segments.usage.style),
                        &mut |v: &str| {
                            let cfg = unsafe { &mut *config_ptr };
                            cfg.segments.usage.style = v.to_string();
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        },
                    );
                    match result {
                        select::SelectResult::Selected(v) => {
                            config.segments.usage.style = v;
                            // Skip bar char + bar length since no bar
                            sub = 4;
                        }
                        select::SelectResult::Back => {
                            sub = 0;
                        }
                        select::SelectResult::Cancelled => return StepResult::Cancelled,
                    }
                }
            }
            2 => {
                // Bar char
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("usage"), t("prompt.barChar"));
                let opts = bar_char_options();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&config.segments.usage.bar_char),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.usage.bar_char = v.to_string();
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        config.segments.usage.bar_char = v;
                        sub = 3;
                    }
                    select::SelectResult::Back => {
                        sub = 1;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            3 => {
                // Bar length
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("usage"), t("prompt.barLength"));
                let opts = bar_length_options();
                let current = config.segments.usage.bar_length.to_string();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&current),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        if let Ok(n) = v.parse::<u32>() {
                            cfg.segments.usage.bar_length = n;
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        }
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        if let Ok(n) = v.parse::<u32>() {
                            config.segments.usage.bar_length = n;
                        }
                        sub = 4;
                    }
                    select::SelectResult::Back => {
                        sub = 2;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            4 => {
                // Refresh interval
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("usage"), t("prompt.refreshInterval"));
                let opts = refresh_interval_options();
                let current = config.segments.usage.refresh_interval.to_string();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&current),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        if let Ok(n) = v.parse::<u64>() {
                            cfg.segments.usage.refresh_interval = n;
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        }
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        if let Ok(n) = v.parse::<u64>() {
                            config.segments.usage.refresh_interval = n;
                        }
                        return StepResult::Next;
                    }
                    select::SelectResult::Back => {
                        if config.segments.usage.show_bar {
                            sub = 3;
                        } else {
                            sub = 1;
                        }
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            _ => return StepResult::Next,
        }
    }
}

// ── Path configuration ──────────────────────────────────────────────────

fn configure_path(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
    step_label: &str,
) -> StepResult {
    // Sub-steps: 0=style, 1=maxLength
    let mut sub: usize = 0;
    loop {
        match sub {
            0 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("path"), t("prompt.style"));
                let opts = style_options();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&config.segments.path.style),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.path.style = v.to_string();
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        config.segments.path.style = v;
                        sub = 1;
                    }
                    select::SelectResult::Back => return StepResult::Back,
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            1 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("path"), t("prompt.maxLength"));
                let opts = max_length_options();
                let current = config.segments.path.max_length.to_string();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&current),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        if let Ok(n) = v.parse::<u32>() {
                            cfg.segments.path.max_length = n;
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        }
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        if let Ok(n) = v.parse::<u32>() {
                            config.segments.path.max_length = n;
                        }
                        return StepResult::Next;
                    }
                    select::SelectResult::Back => {
                        sub = 0;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            _ => return StepResult::Next,
        }
    }
}

// ── Git configuration ───────────────────────────────────────────────────

fn configure_git(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
    step_label: &str,
) -> StepResult {
    // Sub-steps: 0=parts, 1=style
    let mut sub: usize = 0;
    loop {
        match sub {
            0 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("git"), t("prompt.showParts"));
                let opts = vec![
                    multiselect::MultiselectOption {
                        value: "dirty".into(),
                        label: t("part.dirty").into(),
                        hint: Some("*".into()),
                    },
                    multiselect::MultiselectOption {
                        value: "remote".into(),
                        label: t("part.remote").into(),
                        hint: Some("\u{2191}2\u{2193}1".into()),
                    },
                ];
                let mut initial = Vec::new();
                if config.segments.git.show_dirty {
                    initial.push("dirty".to_string());
                }
                if config.segments.git.show_remote {
                    initial.push("remote".to_string());
                }
                let config_ptr = config as *mut Config;
                let result = multiselect::multiselect(
                    &prompt,
                    &opts,
                    &initial,
                    false,
                    &mut |selected: &[String]| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.git.show_dirty = selected.contains(&"dirty".to_string());
                        cfg.segments.git.show_remote = selected.contains(&"remote".to_string());
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    multiselect::MultiselectResult::Selected(selected) => {
                        config.segments.git.show_dirty =
                            selected.contains(&"dirty".to_string());
                        config.segments.git.show_remote =
                            selected.contains(&"remote".to_string());
                        sub = 1;
                    }
                    multiselect::MultiselectResult::Back => return StepResult::Back,
                    multiselect::MultiselectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            1 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("git"), t("prompt.style"));
                let opts = style_options();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&config.segments.git.style),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.git.style = v.to_string();
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        config.segments.git.style = v;
                        return StepResult::Next;
                    }
                    select::SelectResult::Back => {
                        sub = 0;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            _ => return StepResult::Next,
        }
    }
}

// ── Context configuration ───────────────────────────────────────────────

fn configure_context(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
    step_label: &str,
) -> StepResult {
    // Sub-steps: 0=parts, 1=style, 2=barChar, 3=barLength
    let mut sub: usize = 0;
    loop {
        match sub {
            0 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("context"), t("prompt.showParts"));
                let opts = vec![
                    multiselect::MultiselectOption {
                        value: "bar".into(),
                        label: t("part.bar").into(),
                        hint: None,
                    },
                    multiselect::MultiselectOption {
                        value: "percent".into(),
                        label: t("part.percent").into(),
                        hint: None,
                    },
                    multiselect::MultiselectOption {
                        value: "size".into(),
                        label: t("part.size").into(),
                        hint: Some("600K/1M".into()),
                    },
                ];
                let mut initial = Vec::new();
                if config.segments.context.show_bar {
                    initial.push("bar".to_string());
                }
                if config.segments.context.show_percent {
                    initial.push("percent".to_string());
                }
                if config.segments.context.show_size {
                    initial.push("size".to_string());
                }
                let config_ptr = config as *mut Config;
                let result = multiselect::multiselect(
                    &prompt,
                    &opts,
                    &initial,
                    true,
                    &mut |selected: &[String]| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.context.show_bar = selected.contains(&"bar".to_string());
                        cfg.segments.context.show_percent =
                            selected.contains(&"percent".to_string());
                        cfg.segments.context.show_size = selected.contains(&"size".to_string());
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    multiselect::MultiselectResult::Selected(selected) => {
                        config.segments.context.show_bar =
                            selected.contains(&"bar".to_string());
                        config.segments.context.show_percent =
                            selected.contains(&"percent".to_string());
                        config.segments.context.show_size =
                            selected.contains(&"size".to_string());
                        sub = 1;
                    }
                    multiselect::MultiselectResult::Back => return StepResult::Back,
                    multiselect::MultiselectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            1 => {
                if config.segments.context.show_bar {
                    show_screen(config, steps, 1, step_label);
                    let prompt =
                        format!("{} — {}", seg_label("context"), t("prompt.barStyle"));
                    let opts = bar_style_options();
                    let config_ptr = config as *mut Config;
                    let result = select::select(
                        &prompt,
                        &opts,
                        Some(&config.segments.context.style),
                        &mut |v: &str| {
                            let cfg = unsafe { &mut *config_ptr };
                            cfg.segments.context.style = v.to_string();
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        },
                    );
                    match result {
                        select::SelectResult::Selected(v) => {
                            config.segments.context.style = v;
                            sub = 2;
                        }
                        select::SelectResult::Back => {
                            sub = 0;
                        }
                        select::SelectResult::Cancelled => return StepResult::Cancelled,
                    }
                } else {
                    show_screen(config, steps, 1, step_label);
                    let prompt =
                        format!("{} — {}", seg_label("context"), t("prompt.textStyle"));
                    let opts = style_options();
                    let config_ptr = config as *mut Config;
                    let result = select::select(
                        &prompt,
                        &opts,
                        Some(&config.segments.context.style),
                        &mut |v: &str| {
                            let cfg = unsafe { &mut *config_ptr };
                            cfg.segments.context.style = v.to_string();
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        },
                    );
                    match result {
                        select::SelectResult::Selected(v) => {
                            config.segments.context.style = v;
                            return StepResult::Next;
                        }
                        select::SelectResult::Back => {
                            sub = 0;
                        }
                        select::SelectResult::Cancelled => return StepResult::Cancelled,
                    }
                }
            }
            2 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("context"), t("prompt.barChar"));
                let opts = bar_char_options();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&config.segments.context.bar_char),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.context.bar_char = v.to_string();
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        config.segments.context.bar_char = v;
                        sub = 3;
                    }
                    select::SelectResult::Back => {
                        sub = 1;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            3 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("context"), t("prompt.barLength"));
                let opts = bar_length_options();
                let current = config.segments.context.bar_length.to_string();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&current),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        if let Ok(n) = v.parse::<u32>() {
                            cfg.segments.context.bar_length = n;
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        }
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        if let Ok(n) = v.parse::<u32>() {
                            config.segments.context.bar_length = n;
                        }
                        return StepResult::Next;
                    }
                    select::SelectResult::Back => {
                        sub = 2;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            _ => return StepResult::Next,
        }
    }
}

// ── Crypto configuration ────────────────────────────────────────────────

fn configure_crypto(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
    step_label: &str,
) -> StepResult {
    // Sub-steps: 0=coins, 1=style, 2=refresh
    let mut sub: usize = 0;
    loop {
        match sub {
            0 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("crypto"), t("prompt.selectCoins"));
                let opts: Vec<multiselect::MultiselectOption> = CRYPTO_LIST
                    .iter()
                    .map(|(sym, name, _pair)| multiselect::MultiselectOption {
                        value: sym.to_string(),
                        label: format!("{} {}", sym, name),
                        hint: None,
                    })
                    .collect();
                let initial = config.segments.crypto.coins.clone();
                let config_ptr = config as *mut Config;
                let result = multiselect::multiselect(
                    &prompt,
                    &opts,
                    &initial,
                    true,
                    &mut |selected: &[String]| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.crypto.coins = selected.to_vec();
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    multiselect::MultiselectResult::Selected(selected) => {
                        config.segments.crypto.coins = selected;
                        sub = 1;
                    }
                    multiselect::MultiselectResult::Back => return StepResult::Back,
                    multiselect::MultiselectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            1 => {
                show_screen(config, steps, 1, step_label);
                let prompt = format!("{} — {}", seg_label("crypto"), t("prompt.style"));
                let opts = style_options();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&config.segments.crypto.style),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        cfg.segments.crypto.style = v.to_string();
                        preview::update_preview_in_place(cfg, PREVIEW_ROW);
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        config.segments.crypto.style = v;
                        sub = 2;
                    }
                    select::SelectResult::Back => {
                        sub = 0;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            2 => {
                show_screen(config, steps, 1, step_label);
                let prompt =
                    format!("{} — {}", seg_label("crypto"), t("prompt.refreshInterval"));
                let opts = refresh_interval_options();
                let current = config.segments.crypto.refresh_interval.to_string();
                let config_ptr = config as *mut Config;
                let result = select::select(
                    &prompt,
                    &opts,
                    Some(&current),
                    &mut |v: &str| {
                        let cfg = unsafe { &mut *config_ptr };
                        if let Ok(n) = v.parse::<u64>() {
                            cfg.segments.crypto.refresh_interval = n;
                            preview::update_preview_in_place(cfg, PREVIEW_ROW);
                        }
                    },
                );
                match result {
                    select::SelectResult::Selected(v) => {
                        if let Ok(n) = v.parse::<u64>() {
                            config.segments.crypto.refresh_interval = n;
                        }
                        return StepResult::Next;
                    }
                    select::SelectResult::Back => {
                        sub = 1;
                    }
                    select::SelectResult::Cancelled => return StepResult::Cancelled,
                }
            }
            _ => return StepResult::Next,
        }
    }
}

// ── Step 3: Reorder ─────────────────────────────────────────────────────

fn step_reorder(
    config: &mut Config,
    steps: &mut [step_progress::StepInfo],
) -> StepResult {
    let keys = enabled_keys(config);
    if keys.len() <= 1 {
        // Only one segment — nothing to reorder
        steps[2].summary = Some(tf("done.order", &[&keys.join(", ")]));
        return StepResult::Next;
    }

    // Ask if user wants to reorder
    let current_labels: Vec<&str> = keys.iter().map(|k| seg_label(k)).collect();
    let reorder_prompt = tf("prompt.currentOrder", &[&current_labels.join(", ")]);

    show_screen(config, steps, 2, t("step.reorder"));
    let result = confirm::confirm(&reorder_prompt, false);

    match result {
        confirm::ConfirmResult::No => {
            // Keep current order
            let label_list: Vec<&str> = keys.iter().map(|k| seg_label(k)).collect();
            steps[2].summary = Some(tf("done.order", &[&label_list.join(", ")]));
            return StepResult::Next;
        }
        confirm::ConfirmResult::Yes => {
            // Continue to reorder
        }
        confirm::ConfirmResult::Back => return StepResult::Back,
        confirm::ConfirmResult::Cancelled => return StepResult::Cancelled,
    }

    // Sequential position picking
    let total = keys.len();
    let mut new_order: Vec<String> = Vec::new();
    let mut remaining = keys.clone();
    let mut pos: usize = 0;

    while pos < total {
        let pick_label = tf(
            "step.reorderN",
            &[&(pos + 1).to_string(), &total.to_string()],
        );
        show_screen(config, steps, 2, &pick_label);

        let prompt = tf("prompt.pickN", &[&(pos + 1).to_string()]);
        let opts: Vec<select::SelectOption> = remaining
            .iter()
            .map(|k| select::SelectOption {
                value: k.clone(),
                label: seg_label(k).into(),
                hint: None,
            })
            .collect();

        let result = select::select(&prompt, &opts, None, &mut |_| {});

        match result {
            select::SelectResult::Selected(v) => {
                new_order.push(v.clone());
                remaining.retain(|k| k != &v);
                pos += 1;
            }
            select::SelectResult::Back => {
                if pos == 0 {
                    return StepResult::Back;
                }
                // Undo last pick
                pos -= 1;
                let undone = new_order.pop().unwrap();
                remaining.insert(0, undone);
                // Re-sort remaining to match original order
                remaining.sort_by_key(|k| {
                    keys.iter().position(|orig| orig == k).unwrap_or(usize::MAX)
                });
            }
            select::SelectResult::Cancelled => return StepResult::Cancelled,
        }
    }

    config.order = new_order;
    let label_list: Vec<&str> = config.order.iter().map(|k| seg_label(k)).collect();
    steps[2].summary = Some(tf("done.order", &[&label_list.join(", ")]));

    StepResult::Next
}

// ── Step 4: Confirm ─────────────────────────────────────────────────────

fn step_confirm(
    config: &Config,
    steps: &[step_progress::StepInfo],
) -> StepResult {
    show_screen(config, steps, 3, t("step.confirm"));

    let result = confirm::confirm(t("prompt.save"), true);

    match result {
        confirm::ConfirmResult::Yes => StepResult::Next,
        confirm::ConfirmResult::No | confirm::ConfirmResult::Back => StepResult::Back,
        confirm::ConfirmResult::Cancelled => StepResult::Cancelled,
    }
}
