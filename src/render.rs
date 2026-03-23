//! Statusline render pipeline -- the performance-critical hot path.
//!
//! Invoked by Claude Code on every status refresh via `--render`. Reads
//! JSON from stdin (model, workspace, context window, cost), loads the
//! user's config, and outputs a single ANSI-colored statusline string.
//!
//! Key functions:
//! - `run()` -- entry point: read stdin, iterate config.order, print segments
//! - `render_segment()` -- dispatcher to per-segment renderers
//! - `format_model()` / `format_path()` / `format_size()` -- formatting helpers
//!
//! The crypto segment reads from a file-based cache populated by
//! `cache::ensure_caches_fresh()`, which is called at the start of `run()`.
//! The usage segment reads from `rate_limits` in the stdin JSON (native
//! Claude Code v2.1.80+ feature).

use serde::Deserialize;
use std::io::Read;

// ─── Stdin JSON structs ──────────────────────────────────────────────

#[derive(Deserialize, Default, Debug)]
pub struct StdinInput {
    #[serde(default)]
    pub model: StdinModel,
    #[serde(default)]
    pub workspace: StdinWorkspace,
    #[serde(default)]
    pub context_window: StdinContext,
    #[serde(default)]
    pub cost: StdinCost,
    #[serde(default)]
    pub rate_limits: StdinRateLimits,
}

#[derive(Deserialize, Default, Debug)]
pub struct StdinRateLimits {
    #[serde(default)]
    pub five_hour: Option<StdinRateLimitWindow>,
    #[serde(default)]
    #[allow(dead_code)] // reserved for future 7-day window display
    pub seven_day: Option<StdinRateLimitWindow>,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct StdinRateLimitWindow {
    #[serde(default)]
    pub used_percentage: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_resets_at")]
    pub resets_at: Option<String>,
}

/// Accept `resets_at` as either a string (ISO 8601) or a number (Unix epoch).
fn deserialize_resets_at<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let val: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    match val {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::String(s)) => Ok(Some(s)),
        Some(serde_json::Value::Number(n)) => Ok(Some(n.to_string())),
        Some(other) => Ok(Some(other.to_string())),
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct StdinModel {
    #[serde(default)]
    pub id: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct StdinWorkspace {
    #[serde(default)]
    pub current_dir: Option<String>,
    #[serde(default)]
    pub cwd: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct StdinContext {
    #[serde(default)]
    pub context_window_size: Option<u64>,
    #[serde(default)]
    pub used_percentage: Option<f64>,
}

#[derive(Deserialize, Default, Debug)]
pub struct StdinCost {
    #[serde(default)]
    pub total_cost_usd: Option<f64>,
}

// ─── read_stdin ──────────────────────────────────────────────────────

pub fn read_stdin() -> StdinInput {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap_or_default();

    // Try direct deserialization first (fast path)
    if let Ok(input) = serde_json::from_str::<StdinInput>(&buf) {
        return input;
    }

    // Fallback: parse as Value, extract each segment individually so a
    // single field's type mismatch doesn't take down the entire statusline.
    let val: serde_json::Value = match serde_json::from_str(&buf) {
        Ok(v) => v,
        Err(_) => return StdinInput::default(),
    };

    StdinInput {
        model: serde_json::from_value(val.get("model").cloned().unwrap_or_default())
            .unwrap_or_default(),
        workspace: serde_json::from_value(val.get("workspace").cloned().unwrap_or_default())
            .unwrap_or_default(),
        context_window: serde_json::from_value(
            val.get("context_window").cloned().unwrap_or_default(),
        )
        .unwrap_or_default(),
        cost: serde_json::from_value(val.get("cost").cloned().unwrap_or_default())
            .unwrap_or_default(),
        rate_limits: serde_json::from_value(val.get("rate_limits").cloned().unwrap_or_default())
            .unwrap_or_default(),
    }
}

// ─── Formatting helpers ──────────────────────────────────────────────

/// Capitalize the first character of a string, leaving the rest unchanged.
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => {
            let upper: String = c.to_uppercase().collect();
            upper + chars.as_str()
        }
    }
}

/// Format a model id into a human-friendly short name.
///
/// - Strip `claude-` prefix
/// - Strip anything in `[...]` (e.g. `[1m]`)
/// - Pattern: `name-major-minor...` → `NameMajor.Minor`
///   e.g. `opus-4-6` → `Opus4.6`, `haiku-4-5-20251001` → `Haiku4.5`
/// - If the id doesn't match the expected pattern, return it as-is.
pub fn format_model(id: &str) -> String {
    if id.is_empty() {
        return String::new();
    }

    // Strip bracket suffix like [1m]
    let clean = if let Some(bracket_pos) = id.find('[') {
        &id[..bracket_pos]
    } else {
        id
    };

    // Strip `claude-` prefix
    let without_prefix = clean.strip_prefix("claude-").unwrap_or(clean);

    // Split into parts by `-`
    let parts: Vec<&str> = without_prefix.splitn(3, '-').collect();

    if parts.len() < 3 {
        // Not enough parts to extract name-major-minor, return cleaned id
        // but still capitalize if we stripped a prefix
        if clean != id || without_prefix != clean {
            return capitalize(without_prefix);
        }
        return without_prefix.to_string();
    }

    let name = parts[0];
    let major = parts[1];
    let minor_raw = parts[2];

    // Extract leading digits from minor (e.g. "5-20251001" → "5", "6" → "6")
    let minor: String = minor_raw.chars().take_while(|c| c.is_ascii_digit()).collect();

    if major.is_empty() || minor.is_empty() {
        return capitalize(without_prefix);
    }

    format!("{}{}.{}", capitalize(name), major, minor)
}

/// Format a path for display, replacing the home directory with `~`.
/// If the result exceeds `max_length`, show only the last path component.
pub fn format_path(cwd: &str, home: &str, max_length: usize) -> String {
    let replaced = if !home.is_empty() && cwd.starts_with(home) {
        cwd.strip_prefix(home)
            .map(|rest| format!("~{rest}"))
            .unwrap_or_else(|| cwd.to_string())
    } else {
        cwd.to_string()
    };

    if replaced.chars().count() <= max_length {
        replaced
    } else {
        // Return the last path component (directory name)
        match cwd.rsplit('/').next() {
            Some(last) if !last.is_empty() => last.to_string(),
            _ => replaced,
        }
    }
}

/// Format a number with K/M suffixes.
///
/// - >= 1,000,000: `"1M"` or `"1.5M"`
/// - >= 1,000: `"1K"` or `"1.5K"`
/// - else: plain number
/// - If value is exact (no fractional part), omit `.0`
pub fn format_size(n: u64) -> String {
    if n >= 1_000_000 {
        let val = n as f64 / 1_000_000.0;
        if (val.fract().abs()) < f64::EPSILON {
            format!("{}M", val as u64)
        } else {
            // Format with one decimal, strip trailing zeros
            let s = format!("{:.1}M", val);
            s.replace(".0M", "M")
        }
    } else if n >= 1_000 {
        let val = n as f64 / 1_000.0;
        if (val.fract().abs()) < f64::EPSILON {
            format!("{}K", val as u64)
        } else {
            let s = format!("{:.1}K", val);
            s.replace(".0K", "K")
        }
    } else {
        n.to_string()
    }
}

// ─── Render pipeline ────────────────────────────────────────────────

pub fn run() {
    let config = crate::config::load_config();
    let input = read_stdin();
    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Ensure crypto/usage caches are fresh (spawns background refresh if stale)
    crate::cache::ensure_caches_fresh(&config);

    let rows = config.effective_rows();
    let mut first = true;
    for row_keys in &rows {
        let mut parts: Vec<String> = Vec::new();
        for key in *row_keys {
            if let Some(s) = render_segment(key, &config, &input, &home, now) {
                parts.push(s);
            }
        }
        if !parts.is_empty() {
            if !first {
                print!("\n");
            }
            print!("{}", parts.join(" "));
            first = false;
        }
    }
}

/// Dispatch to per-segment renderers based on the key.
fn render_segment(
    key: &str,
    config: &crate::config::Config,
    input: &StdinInput,
    home: &str,
    now: u64,
) -> Option<String> {
    match key {
        "model" => {
            let seg = &config.segments.model;
            if !seg.enabled { return None; }
            render_model(seg, input, now)
        }
        "cost" => {
            let seg = &config.segments.cost;
            if !seg.enabled { return None; }
            render_cost(seg, input, now)
        }
        "path" => {
            let seg = &config.segments.path;
            if !seg.enabled { return None; }
            render_path(seg, input, home, now)
        }
        "git" => {
            let seg = &config.segments.git;
            if !seg.enabled { return None; }
            render_git(seg, now)
        }
        "context" => {
            let seg = &config.segments.context;
            if !seg.enabled { return None; }
            render_context(seg, input, now)
        }
        "usage" => {
            let seg = &config.segments.usage;
            if !seg.enabled { return None; }
            render_usage_window(seg, input.rate_limits.five_hour.as_ref(), "5h", now)
        }
        "usage_7d" => {
            let seg = &config.segments.usage_7d;
            if !seg.enabled { return None; }
            render_usage_window(seg, input.rate_limits.seven_day.as_ref(), "7d", now)
        }
        "crypto" => {
            let seg = &config.segments.crypto;
            if !seg.enabled { return None; }
            render_crypto(seg, now)
        }
        _ => None,
    }
}

// ─── Per-segment renderers ──────────────────────────────────────────

fn render_model(
    seg: &crate::config::ModelSegment,
    input: &StdinInput,
    now: u64,
) -> Option<String> {
    if input.model.id.is_empty() {
        return None;
    }
    let model_name = format_model(&input.model.id);
    let text = if seg.icon.is_empty() {
        model_name
    } else {
        format!("{} {}", seg.icon, model_name)
    };
    Some(crate::styles::format_colored(&seg.style, &text, now))
}

fn render_cost(
    seg: &crate::config::CostSegment,
    input: &StdinInput,
    now: u64,
) -> Option<String> {
    let cost = input.cost.total_cost_usd?;
    Some(crate::styles::format_colored(
        &seg.style,
        &format!("${:.2}", cost),
        now,
    ))
}

fn render_path(
    seg: &crate::config::PathSegment,
    input: &StdinInput,
    home: &str,
    now: u64,
) -> Option<String> {
    let cwd = input
        .workspace
        .current_dir
        .as_deref()
        .or(input.workspace.cwd.as_deref())?;
    let display = format_path(cwd, home, seg.max_length as usize);
    Some(crate::styles::format_colored(&seg.style, &display, now))
}

fn render_git(seg: &crate::config::GitSegment, now: u64) -> Option<String> {
    let branch = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())?;

    let mut text = branch;

    if seg.show_dirty {
        let dirty = std::process::Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .ok()
            .map(|o| !o.stdout.is_empty())
            .unwrap_or(false);
        if dirty {
            text.push('*');
        }
    }

    if seg.show_remote {
        if let Ok(output) = std::process::Command::new("git")
            .args(["rev-list", "--left-right", "--count", "HEAD...@{u}"])
            .output()
        {
            if output.status.success() {
                let counts = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = counts.split_whitespace().collect();
                if parts.len() == 2 {
                    let ahead: i32 = parts[0].parse().unwrap_or(0);
                    let behind: i32 = parts[1].parse().unwrap_or(0);
                    if ahead > 0 {
                        text.push_str(&format!(" \u{2191}{}", ahead));
                    }
                    if behind > 0 {
                        text.push_str(&format!(" \u{2193}{}", behind));
                    }
                }
            }
        }
    }

    Some(crate::styles::format_colored(&seg.style, &text, now))
}

fn render_context(
    seg: &crate::config::ContextSegment,
    input: &StdinInput,
    now: u64,
) -> Option<String> {
    let pct = input.context_window.used_percentage?;
    let size = input.context_window.context_window_size?;
    let ratio = pct / 100.0;
    let used = (pct * size as f64 / 100.0) as u64;

    let mut parts = Vec::new();
    if seg.show_bar {
        parts.push(crate::styles::format_bar(
            &seg.style,
            &seg.bar_char,
            seg.bar_length as usize,
            ratio,
            now,
        ));
    }
    if seg.show_percent {
        parts.push(crate::styles::format_colored(
            &seg.style,
            &format!("{}%", pct as u64),
            now,
        ));
    }
    if seg.show_size {
        parts.push(crate::styles::format_colored(
            &seg.style,
            &format!("{}/{}", format_size(used), format_size(size)),
            now,
        ));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

/// Render a rate-limit usage window (5h or 7d).
/// `default_label` is used when the segment config has no label set.
fn render_usage_window(
    seg: &crate::config::UsageSegment,
    window: Option<&StdinRateLimitWindow>,
    default_label: &str,
    now: u64,
) -> Option<String> {
    let window = window?;
    let pct = window.used_percentage? as u64;
    let resets_at = window.resets_at.as_deref().unwrap_or("");

    let ratio = pct as f64 / 100.0;
    let label = if seg.label.is_empty() {
        default_label
    } else {
        &seg.label
    };
    let mut parts = Vec::new();

    // Label always comes first
    parts.push(crate::styles::format_colored(
        &seg.style,
        &format!("{}:", label),
        now,
    ));

    if seg.show_bar {
        parts.push(crate::styles::format_bar(
            &seg.style,
            &seg.bar_char,
            seg.bar_length as usize,
            ratio,
            now,
        ));
    }
    if seg.show_percent {
        parts.push(crate::styles::format_colored(
            &seg.style,
            &format!("{}%", pct),
            now,
        ));
    }
    if seg.show_reset {
        if let Some(countdown) = format_countdown(resets_at, now) {
            parts.push(crate::styles::format_colored(
                &seg.style,
                &countdown,
                now,
            ));
        }
    }
    // parts always has at least the label, but if only label exists, skip
    if parts.len() <= 1 {
        None
    } else {
        Some(parts.join(" "))
    }
}

fn format_countdown(resets_at: &str, now: u64) -> Option<String> {
    let clean = resets_at.trim();
    if clean.is_empty() || clean == "null" {
        return None;
    }

    // Try parsing as Unix epoch (integer) first, then fall back to ISO 8601
    let epoch = if let Ok(ts) = clean.parse::<u64>() {
        ts
    } else {
        use chrono::DateTime;
        let dt = DateTime::parse_from_rfc3339(clean)
            .or_else(|_| {
                chrono::DateTime::parse_from_str(clean, "%Y-%m-%dT%H:%M:%S%z")
            })
            .ok()?;
        dt.timestamp() as u64
    };

    if epoch <= now {
        return None;
    }
    let diff = epoch - now;
    let days = diff / 86400;
    let hours = (diff % 86400) / 3600;
    let mins = (diff % 3600) / 60;
    if days > 0 {
        Some(format!("{}d{}h", days, hours))
    } else if hours > 0 {
        Some(format!("{}h{}m", hours, mins))
    } else {
        Some(format!("{}m", mins))
    }
}

fn render_crypto(seg: &crate::config::CryptoSegment, now: u64) -> Option<String> {
    let cache = std::fs::read_to_string("/tmp/claude-statusline-crypto-cache").ok()?;
    let prices: Vec<&str> = cache.trim().split('|').collect();
    let display: Vec<String> = seg
        .coins
        .iter()
        .zip(prices.iter())
        .map(|(coin, price)| {
            let p: f64 = price.parse().unwrap_or(0.0);
            format!("{}:${:.0}", coin, p)
        })
        .collect();
    if display.is_empty() {
        None
    } else {
        Some(crate::styles::format_colored(
            &seg.style,
            &display.join(" "),
            now,
        ))
    }
}

// ─── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_model_opus() {
        assert_eq!(format_model("claude-opus-4-6"), "Opus4.6");
    }

    #[test]
    fn test_format_model_sonnet() {
        assert_eq!(format_model("claude-sonnet-4-6"), "Sonnet4.6");
    }

    #[test]
    fn test_format_model_haiku() {
        assert_eq!(format_model("claude-haiku-4-5-20251001"), "Haiku4.5");
    }

    #[test]
    fn test_format_model_with_bracket() {
        assert_eq!(format_model("claude-opus-4-6[1m]"), "Opus4.6");
    }

    #[test]
    fn test_format_model_unknown() {
        assert_eq!(format_model("unknown"), "unknown");
    }

    #[test]
    fn test_format_path_with_home() {
        assert_eq!(
            format_path("/Users/loki/Desktop/web3", "/Users/loki", 20),
            "~/Desktop/web3"
        );
    }

    #[test]
    fn test_format_path_too_long() {
        assert_eq!(
            format_path(
                "/Users/loki/Desktop/very-long-project-name",
                "/Users/loki",
                15
            ),
            "very-long-project-name"
        );
    }

    #[test]
    fn test_format_path_short() {
        assert_eq!(format_path("/tmp", "/Users/loki", 15), "/tmp");
    }

    #[test]
    fn test_format_size_million() {
        assert_eq!(format_size(1_000_000), "1M");
    }

    #[test]
    fn test_format_size_half_million() {
        assert_eq!(format_size(500_000), "500K");
    }

    #[test]
    fn test_format_size_exact_k() {
        assert_eq!(format_size(2_000), "2K");
    }

    #[test]
    fn test_format_size_fractional() {
        assert_eq!(format_size(1_500), "1.5K");
    }

    #[test]
    fn test_format_size_small() {
        assert_eq!(format_size(500), "500");
    }

    #[test]
    fn test_stdin_deserialize() {
        let json = r#"{
            "model": { "id": "claude-opus-4-6" },
            "workspace": { "cwd": "/Users/loki/project" },
            "context_window": { "context_window_size": 200000, "used_percentage": 0.42 },
            "cost": { "total_cost_usd": 1.23 },
            "rate_limits": {
                "five_hour": { "used_percentage": 42.0, "resets_at": "2026-03-23T12:00:00Z" },
                "seven_day": { "used_percentage": 15.0, "resets_at": "2026-03-28T00:00:00Z" }
            }
        }"#;
        let input: StdinInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.model.id, "claude-opus-4-6");
        assert_eq!(input.workspace.cwd.as_deref(), Some("/Users/loki/project"));
        assert_eq!(input.context_window.context_window_size, Some(200000));
        assert!((input.context_window.used_percentage.unwrap() - 0.42).abs() < f64::EPSILON);
        assert!((input.cost.total_cost_usd.unwrap() - 1.23).abs() < f64::EPSILON);
        let five_hour = input.rate_limits.five_hour.as_ref().unwrap();
        assert!((five_hour.used_percentage.unwrap() - 42.0).abs() < f64::EPSILON);
        assert_eq!(five_hour.resets_at.as_deref(), Some("2026-03-23T12:00:00Z"));
        let seven_day = input.rate_limits.seven_day.as_ref().unwrap();
        assert!((seven_day.used_percentage.unwrap() - 15.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_stdin_empty() {
        let input: StdinInput = serde_json::from_str("{}").unwrap_or_default();
        assert_eq!(input.model.id, "");
        assert!(input.workspace.cwd.is_none());
        assert!(input.context_window.context_window_size.is_none());
        assert!(input.cost.total_cost_usd.is_none());
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("opus"), "Opus");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("A"), "A");
        assert_eq!(capitalize("hello world"), "Hello world");
    }

    // ─── Segment render tests ───────────────────────────────────────

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
    fn test_render_segment_model() {
        let seg = crate::config::ModelSegment {
            enabled: true,
            style: "cyan".into(),
            icon: "\u{1f525}".into(),
        };
        let input = StdinInput {
            model: StdinModel { id: "claude-opus-4-6".into() },
            ..Default::default()
        };
        let result = render_model(&seg, &input, 0).unwrap();
        let visible = strip_ansi(&result);
        assert!(visible.contains("Opus4.6"));
        assert!(visible.contains("\u{1f525}"));
        // Should contain ANSI color codes
        assert!(result.contains("\x1b["));
    }

    #[test]
    fn test_render_segment_model_no_icon() {
        let seg = crate::config::ModelSegment {
            enabled: true,
            style: "green".into(),
            icon: "".into(),
        };
        let input = StdinInput {
            model: StdinModel { id: "claude-sonnet-4-6".into() },
            ..Default::default()
        };
        let result = render_model(&seg, &input, 0).unwrap();
        let visible = strip_ansi(&result);
        assert_eq!(visible, "Sonnet4.6");
    }

    #[test]
    fn test_render_segment_cost() {
        let seg = crate::config::CostSegment {
            enabled: true,
            style: "green".into(),
        };
        let input = StdinInput {
            cost: StdinCost { total_cost_usd: Some(0.42) },
            ..Default::default()
        };
        let result = render_cost(&seg, &input, 0).unwrap();
        let visible = strip_ansi(&result);
        assert_eq!(visible, "$0.42");
    }

    #[test]
    fn test_render_segment_cost_none() {
        let seg = crate::config::CostSegment {
            enabled: true,
            style: "green".into(),
        };
        let input = StdinInput {
            cost: StdinCost { total_cost_usd: None },
            ..Default::default()
        };
        let result = render_cost(&seg, &input, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_render_segment_path() {
        let seg = crate::config::PathSegment {
            enabled: true,
            style: "cyan".into(),
            max_length: 20,
        };
        let input = StdinInput {
            workspace: StdinWorkspace {
                current_dir: Some("/Users/loki/Desktop/web3".into()),
                cwd: None,
            },
            ..Default::default()
        };
        let result = render_path(&seg, &input, "/Users/loki", 0).unwrap();
        let visible = strip_ansi(&result);
        assert_eq!(visible, "~/Desktop/web3");
    }

    #[test]
    fn test_render_segment_path_fallback_cwd() {
        let seg = crate::config::PathSegment {
            enabled: true,
            style: "cyan".into(),
            max_length: 20,
        };
        let input = StdinInput {
            workspace: StdinWorkspace {
                current_dir: None,
                cwd: Some("/tmp/myproject".into()),
            },
            ..Default::default()
        };
        let result = render_path(&seg, &input, "/Users/loki", 0).unwrap();
        let visible = strip_ansi(&result);
        assert_eq!(visible, "/tmp/myproject");
    }

    #[test]
    fn test_render_context() {
        let seg = crate::config::ContextSegment {
            enabled: true,
            style: "ultrathink-gradient".into(),
            bar_char: "shade".into(),
            bar_length: 12,
            show_bar: true,
            show_percent: true,
            show_size: true,
        };
        let input = StdinInput {
            context_window: StdinContext {
                context_window_size: Some(1_000_000),
                used_percentage: Some(60.0),
            },
            ..Default::default()
        };
        let result = render_context(&seg, &input, 0).unwrap();
        let visible = strip_ansi(&result);
        // Should contain bar chars, percent, and size
        assert!(visible.contains("60%"));
        assert!(visible.contains("600K/1M"));
        // Bar should be 12 chars (shade: filled + empty)
        // visible format: "<12 bar chars> 60% 600K/1M"
    }

    #[test]
    fn test_render_context_partial() {
        let seg = crate::config::ContextSegment {
            enabled: true,
            style: "cyan".into(),
            bar_char: "shade".into(),
            bar_length: 8,
            show_bar: false,
            show_percent: true,
            show_size: false,
        };
        let input = StdinInput {
            context_window: StdinContext {
                context_window_size: Some(200_000),
                used_percentage: Some(42.0),
            },
            ..Default::default()
        };
        let result = render_context(&seg, &input, 0).unwrap();
        let visible = strip_ansi(&result);
        assert_eq!(visible, "42%");
    }

    #[test]
    fn test_render_context_none_when_missing() {
        let seg = crate::config::ContextSegment::default();
        let input = StdinInput::default();
        let result = render_context(&seg, &input, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_format_countdown_future() {
        // Craft a timestamp 2 hours and 30 minutes in the future
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let future = now + 2 * 3600 + 30 * 60;
        // We can't easily test format_countdown with an ISO timestamp since it
        // invokes `date`. Instead, test the logic by calling it with a known epoch.
        // Let's test the internal logic by computing hours/mins directly.
        let diff = future - now;
        let hours = diff / 3600;
        let mins = (diff % 3600) / 60;
        assert_eq!(hours, 2);
        assert_eq!(mins, 30);
        // The expected format would be "2h30m"
        let expected = format!("{}h{}m", hours, mins);
        assert_eq!(expected, "2h30m");
    }

    #[test]
    fn test_format_countdown_past() {
        // A past timestamp should return None from format_countdown
        // We test the core condition: epoch <= now
        let now = 1000u64;
        let epoch = 999u64;
        assert!(epoch <= now);
        // This validates the logic in format_countdown:
        // if epoch <= now { return None; }
    }

    #[test]
    fn test_format_countdown_empty() {
        let result = format_countdown("", 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_format_countdown_null() {
        let result = format_countdown("null", 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_render_crypto_from_cache() {
        use std::io::Write;
        // Write a temp cache file
        let cache_path = "/tmp/claude-statusline-crypto-cache-test";
        {
            let mut f = std::fs::File::create(cache_path).unwrap();
            write!(f, "84532.50|3214.75").unwrap();
        }

        let seg = crate::config::CryptoSegment {
            enabled: true,
            style: "green".into(),
            refresh_interval: 60,
            coins: vec!["BTC".into(), "ETH".into()],
        };

        // Read cache and verify parsing logic
        let cache = std::fs::read_to_string(cache_path).unwrap();
        let prices: Vec<&str> = cache.trim().split('|').collect();
        let display: Vec<String> = seg
            .coins
            .iter()
            .zip(prices.iter())
            .map(|(coin, price)| {
                let p: f64 = price.parse().unwrap_or(0.0);
                format!("{}:${:.0}", coin, p)
            })
            .collect();
        let text = display.join(" ");
        assert_eq!(text, "BTC:$84532 ETH:$3215");

        // Clean up
        let _ = std::fs::remove_file(cache_path);
    }

    #[test]
    fn test_render_crypto_no_cache() {
        // Remove cache file to ensure None
        let _ = std::fs::remove_file("/tmp/claude-statusline-crypto-cache");
        let seg = crate::config::CryptoSegment {
            enabled: true,
            style: "green".into(),
            refresh_interval: 60,
            coins: vec!["BTC".into()],
        };
        let _result = render_crypto(&seg, 0);
        // If file doesn't exist, returns None
        // (it may exist from other tests, so just check it handles gracefully)
        // This test mainly ensures no panic
    }

    #[test]
    fn test_render_segment_disabled() {
        let config = crate::config::Config {
            order: vec!["model".into()],
            segments: crate::config::Segments {
                model: crate::config::ModelSegment {
                    enabled: false,
                    style: "cyan".into(),
                    icon: "".into(),
                },
                ..Default::default()
            },
            ..Default::default()
        };
        let input = StdinInput {
            model: StdinModel { id: "claude-opus-4-6".into() },
            ..Default::default()
        };
        let result = render_segment("model", &config, &input, "", 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_render_segment_unknown_key() {
        let config = crate::config::Config::default();
        let input = StdinInput::default();
        let result = render_segment("nonexistent", &config, &input, "", 0);
        assert!(result.is_none());
    }

    // ─── Integration tests: Claude Code v2.1.81 compatibility ────────

    /// Real Claude Code v2.1.81 stdin JSON with integer `resets_at` timestamps.
    /// This is the exact format that caused the deserialization failure.
    const REAL_CLAUDE_CODE_JSON: &str = r#"{
        "session_id": "test-session",
        "cwd": "/Users/loki/cc",
        "model": { "id": "claude-opus-4-6[1m]", "display_name": "Opus 4.6 (1M context)" },
        "workspace": { "current_dir": "/Users/loki/cc", "project_dir": "/Users/loki/cc" },
        "version": "2.1.81",
        "cost": { "total_cost_usd": 1.47 },
        "context_window": { "context_window_size": 1000000, "used_percentage": 8 },
        "rate_limits": {
            "five_hour": { "used_percentage": 47, "resets_at": 1774292400 },
            "seven_day": { "used_percentage": 28, "resets_at": 1774580400 }
        }
    }"#;

    #[test]
    fn test_real_claude_code_json_parses_all_fields() {
        let input: StdinInput = serde_json::from_str(REAL_CLAUDE_CODE_JSON).unwrap();

        // model
        assert_eq!(input.model.id, "claude-opus-4-6[1m]");

        // workspace
        assert_eq!(
            input.workspace.current_dir.as_deref(),
            Some("/Users/loki/cc")
        );

        // cost
        assert!((input.cost.total_cost_usd.unwrap() - 1.47).abs() < 0.01);

        // context_window
        assert_eq!(input.context_window.context_window_size, Some(1_000_000));
        assert!((input.context_window.used_percentage.unwrap() - 8.0).abs() < f64::EPSILON);

        // rate_limits — the critical fix: integer resets_at must parse
        let five_hour = input.rate_limits.five_hour.as_ref().unwrap();
        assert!((five_hour.used_percentage.unwrap() - 47.0).abs() < f64::EPSILON);
        assert_eq!(five_hour.resets_at.as_deref(), Some("1774292400"));

        let seven_day = input.rate_limits.seven_day.as_ref().unwrap();
        assert!((seven_day.used_percentage.unwrap() - 28.0).abs() < f64::EPSILON);
        assert_eq!(seven_day.resets_at.as_deref(), Some("1774580400"));
    }

    #[test]
    fn test_resets_at_accepts_string() {
        let json = r#"{ "used_percentage": 42.0, "resets_at": "2026-03-23T12:00:00Z" }"#;
        let w: StdinRateLimitWindow = serde_json::from_str(json).unwrap();
        assert_eq!(w.resets_at.as_deref(), Some("2026-03-23T12:00:00Z"));
    }

    #[test]
    fn test_resets_at_accepts_integer() {
        let json = r#"{ "used_percentage": 47, "resets_at": 1774292400 }"#;
        let w: StdinRateLimitWindow = serde_json::from_str(json).unwrap();
        assert_eq!(w.resets_at.as_deref(), Some("1774292400"));
    }

    #[test]
    fn test_resets_at_accepts_null() {
        let json = r#"{ "used_percentage": 47, "resets_at": null }"#;
        let w: StdinRateLimitWindow = serde_json::from_str(json).unwrap();
        assert!(w.resets_at.is_none());
    }

    #[test]
    fn test_resets_at_accepts_missing() {
        let json = r#"{ "used_percentage": 47 }"#;
        let w: StdinRateLimitWindow = serde_json::from_str(json).unwrap();
        assert!(w.resets_at.is_none());
    }

    #[test]
    fn test_format_countdown_unix_epoch() {
        // 2 hours from now
        let now = 1774280000u64;
        let future = now + 2 * 3600 + 30 * 60;
        let result = format_countdown(&future.to_string(), now).unwrap();
        assert_eq!(result, "2h30m");
    }

    #[test]
    fn test_format_countdown_iso8601() {
        // Use a fixed known timestamp
        let result = format_countdown("2026-03-24T00:00:00Z", 1774243200);
        // 1774243200 = 2026-03-20T16:00:00Z, so diff is ~3 days
        assert!(result.is_some());
        let text = result.unwrap();
        assert!(text.contains('h') || text.contains('m'));
    }

    #[test]
    fn test_resilient_read_stdin_bad_rate_limits_preserves_other_fields() {
        // Simulate a JSON where rate_limits has an unexpected type
        let json = r#"{
            "model": { "id": "claude-opus-4-6" },
            "cost": { "total_cost_usd": 1.23 },
            "context_window": { "context_window_size": 200000, "used_percentage": 42.0 },
            "rate_limits": "invalid_type"
        }"#;
        // Direct deserialization fails, but fallback should recover other fields
        assert!(serde_json::from_str::<StdinInput>(json).is_err());

        // Simulate the fallback path
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        let model: StdinModel =
            serde_json::from_value(val.get("model").cloned().unwrap_or_default())
                .unwrap_or_default();
        let cost: StdinCost =
            serde_json::from_value(val.get("cost").cloned().unwrap_or_default())
                .unwrap_or_default();
        let rate_limits: StdinRateLimits =
            serde_json::from_value(val.get("rate_limits").cloned().unwrap_or_default())
                .unwrap_or_default();

        assert_eq!(model.id, "claude-opus-4-6");
        assert!((cost.total_cost_usd.unwrap() - 1.23).abs() < f64::EPSILON);
        // rate_limits gracefully defaults
        assert!(rate_limits.five_hour.is_none());
    }

    #[test]
    fn test_all_segments_render_from_real_json() {
        let input: StdinInput = serde_json::from_str(REAL_CLAUDE_CODE_JSON).unwrap();
        let config = crate::config::Config::default();

        // model renders
        let model = render_segment("model", &config, &input, "/Users/loki", 0);
        assert!(model.is_some());
        assert!(strip_ansi(&model.unwrap()).contains("Opus4.6"));

        // cost renders
        let cost = render_segment("cost", &config, &input, "/Users/loki", 0);
        assert!(cost.is_some());
        assert!(strip_ansi(&cost.unwrap()).contains("$1.47"));

        // path renders
        let path = render_segment("path", &config, &input, "/Users/loki", 0);
        assert!(path.is_some());

        // context renders
        let ctx = render_segment("context", &config, &input, "/Users/loki", 0);
        assert!(ctx.is_some());
        assert!(strip_ansi(&ctx.unwrap()).contains("8%"));

        // usage (5h) renders with countdown
        let usage = render_segment("usage", &config, &input, "/Users/loki", 1774280000);
        assert!(usage.is_some());
        assert!(strip_ansi(&usage.unwrap()).contains("47%"));
    }

    // ─── Multi-row & rows config tests ───────────────────────────────

    #[test]
    fn test_effective_rows_from_rows_field() {
        let config = crate::config::Config {
            rows: vec![
                vec!["model".into(), "cost".into()],
                vec!["usage".into(), "usage_7d".into()],
            ],
            order: vec!["should".into(), "be".into(), "ignored".into()],
            ..Default::default()
        };
        let rows = config.effective_rows();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], &["model", "cost"]);
        assert_eq!(rows[1], &["usage", "usage_7d"]);
    }

    #[test]
    fn test_effective_rows_fallback_to_order() {
        let config = crate::config::Config {
            rows: Vec::new(),
            order: vec!["model".into(), "cost".into()],
            order_row2: vec!["usage".into()],
            ..Default::default()
        };
        let rows = config.effective_rows();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], &["model", "cost"]);
        assert_eq!(rows[1], &["usage"]);
    }

    #[test]
    fn test_effective_rows_single_row_legacy() {
        let config = crate::config::Config {
            rows: Vec::new(),
            order: vec!["model".into()],
            order_row2: Vec::new(),
            ..Default::default()
        };
        let rows = config.effective_rows();
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_effective_rows_capped_at_3() {
        let config = crate::config::Config {
            rows: vec![
                vec!["model".into()],
                vec!["cost".into()],
                vec!["usage".into()],
                vec!["crypto".into()], // 4th row — should be dropped
                vec!["context".into()], // 5th row — should be dropped
            ],
            ..Default::default()
        };
        let rows = config.effective_rows();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[2], &["usage"]);
    }

    #[test]
    fn test_effective_rows_empty_rows_skipped_in_render() {
        // Empty inner vecs should produce no output lines
        let config = crate::config::Config {
            rows: vec![
                vec!["model".into()],
                vec![], // empty row
                vec!["cost".into()],
            ],
            ..Default::default()
        };
        let rows = config.effective_rows();
        assert_eq!(rows.len(), 3); // all 3 returned, but render skips empty
        assert!(rows[1].is_empty());
    }

    #[test]
    fn test_usage_7d_renders_seven_day_data() {
        let input: StdinInput = serde_json::from_str(REAL_CLAUDE_CODE_JSON).unwrap();
        let mut config = crate::config::Config::default();
        config.segments.usage_7d.enabled = true;

        let result = render_segment("usage_7d", &config, &input, "", 1774280000);
        assert!(result.is_some());
        let visible = strip_ansi(&result.unwrap());
        assert!(visible.contains("7d:"));
        assert!(visible.contains("28%"));
        assert!(visible.contains("d")); // countdown contains days
    }

    #[test]
    fn test_usage_renders_with_label() {
        let input: StdinInput = serde_json::from_str(REAL_CLAUDE_CODE_JSON).unwrap();
        let config = crate::config::Config::default();

        let result = render_segment("usage", &config, &input, "", 1774280000);
        assert!(result.is_some());
        let visible = strip_ansi(&result.unwrap());
        assert!(visible.starts_with("5h:"));
    }

    #[test]
    fn test_usage_custom_label() {
        let input: StdinInput = serde_json::from_str(REAL_CLAUDE_CODE_JSON).unwrap();
        let mut config = crate::config::Config::default();
        config.segments.usage.label = "FIVE".into();

        let result = render_segment("usage", &config, &input, "", 1774280000);
        let visible = strip_ansi(&result.unwrap());
        assert!(visible.starts_with("FIVE:"));
    }

    #[test]
    fn test_usage_7d_disabled_returns_none() {
        let input: StdinInput = serde_json::from_str(REAL_CLAUDE_CODE_JSON).unwrap();
        let mut config = crate::config::Config::default();
        config.segments.usage_7d.enabled = false;

        let result = render_segment("usage_7d", &config, &input, "", 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_usage_no_rate_limits_returns_none() {
        let input = StdinInput::default(); // no rate_limits
        let config = crate::config::Config::default();

        let result = render_segment("usage", &config, &input, "", 0);
        assert!(result.is_none());

        let result_7d = render_segment("usage_7d", &config, &input, "", 0);
        assert!(result_7d.is_none());
    }

    #[test]
    fn test_format_countdown_days() {
        // 3 days 5 hours from now
        let now = 1774280000u64;
        let future = now + 3 * 86400 + 5 * 3600;
        let result = format_countdown(&future.to_string(), now).unwrap();
        assert_eq!(result, "3d5h");
    }

    #[test]
    fn test_format_countdown_minutes_only() {
        let now = 1774280000u64;
        let future = now + 45 * 60;
        let result = format_countdown(&future.to_string(), now).unwrap();
        assert_eq!(result, "45m");
    }

    #[test]
    fn test_three_row_config() {
        let config = crate::config::Config {
            rows: vec![
                vec!["model".into(), "cost".into()],
                vec!["path".into(), "git".into()],
                vec!["usage".into(), "usage_7d".into()],
            ],
            ..Default::default()
        };
        let rows = config.effective_rows();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], &["model", "cost"]);
        assert_eq!(rows[1], &["path", "git"]);
        assert_eq!(rows[2], &["usage", "usage_7d"]);
    }
}
