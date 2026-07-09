//! Statusline preview renderer using hardcoded sample data.
//!
//! Generates a preview string that looks like the real statusline but uses
//! fixed sample values (e.g., model="Opus4.6", cost="$0.42", usage=25%).
//! This lets the user see the fixed layout before saving.
//!
//! Key function:
//! - `render_sample_segment(key, config, now)` -- render one segment with
//!   sample data; used by the wizard's confirmation preview.

use crate::config::Config;
use crate::styles::{format_bar, format_colored};
#[cfg(test)]
use std::time::SystemTime;

// ── Public API ───────────────────────────────────────────────────────

/// Render each effective row to a joined preview string using sample data,
/// skipping rows whose segments all render to nothing. One entry per non-empty
/// row. Shared by the wizard's confirmation screen and `render_preview`.
pub fn render_rows(config: &Config, now: u64) -> Vec<String> {
    config
        .effective_rows()
        .iter()
        .filter_map(|row_keys| {
            let parts: Vec<String> = row_keys
                .iter()
                .filter_map(|key| render_sample_segment(key, config, now))
                .collect();
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(" "))
            }
        })
        .collect()
}

/// Render a full multi-row preview string (rows joined by newlines).
#[cfg(test)]
pub fn render_preview(config: &Config) -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    render_rows(config, now).join("\n")
}

// ── Per-segment sample renderers ─────────────────────────────────────

pub fn render_sample_segment(key: &str, config: &Config, now: u64) -> Option<String> {
    match key {
        "model" => {
            let seg = &config.segments.model;
            if !seg.enabled {
                return None;
            }
            // Sample includes a reasoning-effort suffix ("high") to show how
            // effort appears when the model reports one.
            let text = if seg.icon.is_empty() {
                "Opus4.6 high".to_string()
            } else {
                format!("{} Opus4.6 high", seg.icon)
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
            parts.push(format_colored(&seg.style, "5h:", now));
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
            if parts.len() <= 1 {
                None
            } else {
                Some(parts.join(" "))
            }
        }
        "usage_7d" => {
            let seg = &config.segments.usage_7d;
            if !seg.enabled {
                return None;
            }
            let ratio = 0.15;
            let mut parts = Vec::new();
            parts.push(format_colored(&seg.style, "7d:", now));
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
                parts.push(format_colored(&seg.style, "15%", now));
            }
            if seg.show_reset {
                parts.push(format_colored(&seg.style, "5d22h", now));
            }
            if parts.len() <= 1 {
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

    /// Every segment disabled, for isolating a single segment in a test.
    fn all_disabled() -> Segments {
        Segments {
            model: ModelSegment { enabled: false, ..Default::default() },
            cost: CostSegment { enabled: false, ..Default::default() },
            usage: UsageSegment { enabled: false, ..Default::default() },
            usage_7d: UsageSegment { enabled: false, ..Default::default() },
            path: PathSegment { enabled: false, ..Default::default() },
            git: GitSegment { enabled: false, ..Default::default() },
            context: ContextSegment { enabled: false, ..Default::default() },
        }
    }

    #[test]
    fn test_render_preview_with_defaults() {
        let config = Config::default();
        let result = render_preview(&config);
        assert!(!result.is_empty());
        assert!(result.contains("\x1b["));
    }

    #[test]
    fn test_render_preview_all_disabled() {
        let config = Config {
            rows: vec![vec![
                "model".into(),
                "cost".into(),
                "path".into(),
                "context".into(),
            ]],
            segments: all_disabled(),
            ..Default::default()
        };
        let result = render_preview(&config);
        assert!(result.is_empty());
    }

    #[test]
    fn test_render_preview_model_with_effort() {
        let config = Config {
            rows: vec![vec!["model".into()]],
            segments: Segments {
                model: ModelSegment { enabled: true, style: "cyan".into(), icon: "".into() },
                ..all_disabled()
            },
            ..Default::default()
        };
        let visible = strip_ansi(&render_preview(&config));
        assert!(visible.contains("Opus4.6"));
        assert!(visible.contains("high"));
    }

    #[test]
    fn test_render_preview_context_bar() {
        let config = Config {
            rows: vec![vec!["context".into()]],
            segments: Segments {
                context: ContextSegment {
                    enabled: true,
                    style: "semantic".into(),
                    bar_char: "shade".into(),
                    bar_length: 12,
                    show_bar: true,
                    show_percent: true,
                    show_size: true,
                },
                ..all_disabled()
            },
            ..Default::default()
        };
        let visible = strip_ansi(&render_preview(&config));
        assert!(visible.contains("60%"));
        assert!(visible.contains("600K/1M"));
    }

    #[test]
    fn test_render_preview_usage_no_bar() {
        // Fixed-layout 5h usage: percent + reset, no progress bar.
        let config = Config {
            rows: vec![vec!["usage".into()]],
            segments: Segments {
                usage: UsageSegment {
                    enabled: true,
                    style: "white".into(),
                    bar_char: "shade".into(),
                    bar_length: 8,
                    show_bar: false,
                    show_percent: true,
                    show_reset: true,
                    label: String::new(),
                },
                ..all_disabled()
            },
            ..Default::default()
        };
        let visible = strip_ansi(&render_preview(&config));
        assert!(visible.contains("5h:"));
        assert!(visible.contains("25%"));
        assert!(visible.contains("1h43m"));
    }
}
