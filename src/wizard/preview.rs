use crate::config::Config;
use crate::styles::{format_bar, format_colored};
use std::time::SystemTime;

// ── Sample data ──────────────────────────────────────────────────────

const SAMPLE_PRICES: &[(&str, u64)] = &[
    ("BTC", 73748),
    ("ETH", 2265),
    ("BNB", 612),
    ("SOL", 178),
];

// ── Public API ───────────────────────────────────────────────────────

/// Render a preview statusline bar using sample data for the given config.
///
/// Iterates through `config.order`, rendering each enabled segment with
/// hard-coded sample values so the user can see what the statusline will
/// look like before committing to the configuration.
pub fn render_preview(config: &Config) -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut parts: Vec<String> = Vec::new();

    for key in &config.order {
        if let Some(s) = render_sample_segment(key, config, now) {
            parts.push(s);
        }
    }

    parts.join(" ")
}

/// Update the preview line in-place at the given terminal row without
/// moving the main cursor.
pub fn update_preview_in_place(config: &Config, preview_row: u16) {
    let preview = render_preview(config);
    let text = format!("  \x1b[2mPreview:\x1b[0m {}", preview);
    super::terminal::print_at(preview_row, &text);
}

// ── Per-segment sample renderers ─────────────────────────────────────

fn render_sample_segment(key: &str, config: &Config, now: u64) -> Option<String> {
    match key {
        "model" => {
            let seg = &config.segments.model;
            if !seg.enabled {
                return None;
            }
            let text = if seg.icon.is_empty() {
                "Opus4.6".to_string()
            } else {
                format!("{} Opus4.6", seg.icon)
            };
            Some(format_colored(&seg.style, &text, now))
        }
        "cost" => {
            let seg = &config.segments.cost;
            if !seg.enabled {
                return None;
            }
            Some(format_colored(&seg.style, "$0.42", now))
        }
        "usage" => {
            let seg = &config.segments.usage;
            if !seg.enabled {
                return None;
            }
            let ratio = 0.25;
            let mut parts = Vec::new();
            if seg.show_bar {
                parts.push(format_bar(
                    &seg.style,
                    &seg.bar_char,
                    seg.bar_length as usize,
                    ratio,
                    now,
                ));
            }
            if seg.show_percent {
                parts.push(format_colored(&seg.style, "25%", now));
            }
            if seg.show_reset {
                parts.push(format_colored(&seg.style, "1h43m", now));
            }
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(" "))
            }
        }
        "path" => {
            let seg = &config.segments.path;
            if !seg.enabled {
                return None;
            }
            Some(format_colored(&seg.style, "~/Desktop/web3", now))
        }
        "git" => {
            let seg = &config.segments.git;
            if !seg.enabled {
                return None;
            }
            let mut text = "main".to_string();
            if seg.show_dirty {
                text.push('*');
            }
            if seg.show_remote {
                text.push_str(" \u{2191}2\u{2193}1");
            }
            Some(format_colored(&seg.style, &text, now))
        }
        "context" => {
            let seg = &config.segments.context;
            if !seg.enabled {
                return None;
            }
            let ratio = 0.6;
            let mut parts = Vec::new();
            if seg.show_bar {
                parts.push(format_bar(
                    &seg.style,
                    &seg.bar_char,
                    seg.bar_length as usize,
                    ratio,
                    now,
                ));
            }
            if seg.show_percent {
                parts.push(format_colored(&seg.style, "60%", now));
            }
            if seg.show_size {
                parts.push(format_colored(&seg.style, "600K/1M", now));
            }
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(" "))
            }
        }
        "crypto" => {
            let seg = &config.segments.crypto;
            if !seg.enabled {
                return None;
            }
            let display: Vec<String> = seg
                .coins
                .iter()
                .filter_map(|coin| {
                    SAMPLE_PRICES
                        .iter()
                        .find(|(sym, _)| *sym == coin.as_str())
                        .map(|(sym, price)| format!("{}:${}", sym, price))
                })
                .collect();
            if display.is_empty() {
                None
            } else {
                Some(format_colored(&seg.style, &display.join(" "), now))
            }
        }
        _ => None,
    }
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;

    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut in_escape = false;
        for ch in s.chars() {
            if ch == '\x1b' {
                in_escape = true;
                continue;
            }
            if in_escape {
                if ch == 'm' {
                    in_escape = false;
                }
                continue;
            }
            out.push(ch);
        }
        out
    }

    #[test]
    fn test_render_preview_with_defaults() {
        let config = Config::default();
        let result = render_preview(&config);
        assert!(!result.is_empty());
        // Should contain ANSI escape codes
        assert!(result.contains("\x1b["));
    }

    #[test]
    fn test_render_preview_all_disabled() {
        let config = Config {
            order: vec![
                "model".into(),
                "cost".into(),
                "usage".into(),
                "path".into(),
                "git".into(),
                "context".into(),
                "crypto".into(),
            ],
            segments: Segments {
                model: ModelSegment {
                    enabled: false,
                    ..Default::default()
                },
                cost: CostSegment {
                    enabled: false,
                    ..Default::default()
                },
                usage: UsageSegment {
                    enabled: false,
                    ..Default::default()
                },
                path: PathSegment {
                    enabled: false,
                    ..Default::default()
                },
                git: GitSegment {
                    enabled: false,
                    ..Default::default()
                },
                context: ContextSegment {
                    enabled: false,
                    ..Default::default()
                },
                crypto: CryptoSegment {
                    enabled: false,
                    ..Default::default()
                },
            },
            ..Default::default()
        };
        let result = render_preview(&config);
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_preview_model_only() {
        let config = Config {
            order: vec!["model".into()],
            segments: Segments {
                model: ModelSegment {
                    enabled: true,
                    style: "cyan".into(),
                    icon: "".into(),
                },
                cost: CostSegment {
                    enabled: false,
                    ..Default::default()
                },
                usage: UsageSegment {
                    enabled: false,
                    ..Default::default()
                },
                path: PathSegment {
                    enabled: false,
                    ..Default::default()
                },
                git: GitSegment {
                    enabled: false,
                    ..Default::default()
                },
                context: ContextSegment {
                    enabled: false,
                    ..Default::default()
                },
                crypto: CryptoSegment {
                    enabled: false,
                    ..Default::default()
                },
            },
            ..Default::default()
        };
        let result = render_preview(&config);
        let visible = strip_ansi(&result);
        assert!(visible.contains("Opus4.6"));
    }

    #[test]
    fn test_render_preview_context_bar() {
        let config = Config {
            order: vec!["context".into()],
            segments: Segments {
                model: ModelSegment {
                    enabled: false,
                    ..Default::default()
                },
                cost: CostSegment {
                    enabled: false,
                    ..Default::default()
                },
                usage: UsageSegment {
                    enabled: false,
                    ..Default::default()
                },
                path: PathSegment {
                    enabled: false,
                    ..Default::default()
                },
                git: GitSegment {
                    enabled: false,
                    ..Default::default()
                },
                context: ContextSegment {
                    enabled: true,
                    style: "ultrathink-gradient".into(),
                    bar_char: "shade".into(),
                    bar_length: 12,
                    show_bar: true,
                    show_percent: true,
                    show_size: true,
                },
                crypto: CryptoSegment {
                    enabled: false,
                    ..Default::default()
                },
            },
            ..Default::default()
        };
        let result = render_preview(&config);
        assert!(!result.is_empty());
        let visible = strip_ansi(&result);
        assert!(visible.contains("60%"));
        assert!(visible.contains("600K/1M"));
    }

    #[test]
    fn test_render_preview_usage_parts() {
        // bar only
        let config = Config {
            order: vec!["usage".into()],
            segments: Segments {
                model: ModelSegment {
                    enabled: false,
                    ..Default::default()
                },
                cost: CostSegment {
                    enabled: false,
                    ..Default::default()
                },
                usage: UsageSegment {
                    enabled: true,
                    style: "semantic".into(),
                    bar_char: "shade".into(),
                    bar_length: 8,
                    show_bar: true,
                    show_percent: false,
                    show_reset: false,
                    refresh_interval: 120,
                },
                path: PathSegment {
                    enabled: false,
                    ..Default::default()
                },
                git: GitSegment {
                    enabled: false,
                    ..Default::default()
                },
                context: ContextSegment {
                    enabled: false,
                    ..Default::default()
                },
                crypto: CryptoSegment {
                    enabled: false,
                    ..Default::default()
                },
            },
            ..Default::default()
        };
        let result = render_preview(&config);
        assert!(!result.is_empty());
        let visible = strip_ansi(&result);
        // Should have bar chars but not percent or reset
        assert!(!visible.contains("25%"));
        assert!(!visible.contains("1h43m"));

        // percent + reset only (no bar)
        let config2 = Config {
            order: vec!["usage".into()],
            segments: Segments {
                usage: UsageSegment {
                    enabled: true,
                    style: "green".into(),
                    bar_char: "shade".into(),
                    bar_length: 8,
                    show_bar: false,
                    show_percent: true,
                    show_reset: true,
                    refresh_interval: 120,
                },
                model: ModelSegment {
                    enabled: false,
                    ..Default::default()
                },
                cost: CostSegment {
                    enabled: false,
                    ..Default::default()
                },
                path: PathSegment {
                    enabled: false,
                    ..Default::default()
                },
                git: GitSegment {
                    enabled: false,
                    ..Default::default()
                },
                context: ContextSegment {
                    enabled: false,
                    ..Default::default()
                },
                crypto: CryptoSegment {
                    enabled: false,
                    ..Default::default()
                },
            },
            ..Default::default()
        };
        let result2 = render_preview(&config2);
        let visible2 = strip_ansi(&result2);
        assert!(visible2.contains("25%"));
        assert!(visible2.contains("1h43m"));
    }

    #[test]
    fn test_render_preview_crypto_coins() {
        // Single coin (BTC)
        let config = Config {
            order: vec!["crypto".into()],
            segments: Segments {
                model: ModelSegment {
                    enabled: false,
                    ..Default::default()
                },
                cost: CostSegment {
                    enabled: false,
                    ..Default::default()
                },
                usage: UsageSegment {
                    enabled: false,
                    ..Default::default()
                },
                path: PathSegment {
                    enabled: false,
                    ..Default::default()
                },
                git: GitSegment {
                    enabled: false,
                    ..Default::default()
                },
                context: ContextSegment {
                    enabled: false,
                    ..Default::default()
                },
                crypto: CryptoSegment {
                    enabled: true,
                    style: "green".into(),
                    refresh_interval: 60,
                    coins: vec!["BTC".into()],
                },
            },
            ..Default::default()
        };
        let result = render_preview(&config);
        let visible = strip_ansi(&result);
        assert_eq!(visible, "BTC:$73748");

        // Multiple coins
        let config2 = Config {
            order: vec!["crypto".into()],
            segments: Segments {
                model: ModelSegment {
                    enabled: false,
                    ..Default::default()
                },
                cost: CostSegment {
                    enabled: false,
                    ..Default::default()
                },
                usage: UsageSegment {
                    enabled: false,
                    ..Default::default()
                },
                path: PathSegment {
                    enabled: false,
                    ..Default::default()
                },
                git: GitSegment {
                    enabled: false,
                    ..Default::default()
                },
                context: ContextSegment {
                    enabled: false,
                    ..Default::default()
                },
                crypto: CryptoSegment {
                    enabled: true,
                    style: "green".into(),
                    refresh_interval: 60,
                    coins: vec!["BTC".into(), "ETH".into(), "SOL".into()],
                },
            },
            ..Default::default()
        };
        let result2 = render_preview(&config2);
        let visible2 = strip_ansi(&result2);
        assert!(visible2.contains("BTC:$73748"));
        assert!(visible2.contains("ETH:$2265"));
        assert!(visible2.contains("SOL:$178"));
    }
}
