//! Configuration structs, JSON persistence, and path helpers.
//!
//! Defines the `Config` type (top-level) and per-segment structs (`ModelSegment`,
//! `CostSegment`, `UsageSegment`, `PathSegment`, `GitSegment`, `ContextSegment`,
//! `CryptoSegment`) that map to `~/.claude/statusline/config.json`.
//!
//! Key functions:
//! - `statusline_dir()` / `config_path()` / `bin_path()` / `log_path()` -- path helpers
//! - `load_config()` -- deserialize config from disk (falls back to defaults)
//! - `save_config()` -- serialize config to disk
//!
//! Used by every other module: the wizard writes configs, the render pipeline
//! reads them, and the install step references path helpers.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Crypto list constant
// ---------------------------------------------------------------------------

pub const CRYPTO_LIST: &[(&str, &str, &str)] = &[
    ("BTC", "Bitcoin", "BTCUSDT"),
    ("ETH", "Ethereum", "ETHUSDT"),
    ("BNB", "BNB", "BNBUSDT"),
    ("SOL", "Solana", "SOLUSDT"),
];

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

/// Returns `~/.claude/statusline/`
pub fn statusline_dir() -> PathBuf {
    dirs::home_dir()
        .expect("cannot determine home directory")
        .join(".claude")
        .join("statusline")
}

/// Returns `~/.claude/statusline/config.json`
pub fn config_path() -> PathBuf {
    statusline_dir().join("config.json")
}

/// Returns `~/.claude/statusline/bin/cc-statusline`
pub fn bin_path() -> PathBuf {
    statusline_dir().join("bin").join("cc-statusline")
}

/// Returns `~/.claude/statusline/statusline.log`
#[allow(dead_code)]
pub fn log_path() -> PathBuf {
    statusline_dir().join("statusline.log")
}

// ---------------------------------------------------------------------------
// Segment structs
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSegment {
    pub enabled: bool,
    pub style: String,
    pub icon: String,
}

impl Default for ModelSegment {
    fn default() -> Self {
        Self {
            enabled: true,
            style: "cyan".into(),
            icon: "\u{1f525}".into(), // fire emoji
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSegment {
    pub enabled: bool,
    pub style: String,
}

impl Default for CostSegment {
    fn default() -> Self {
        Self {
            enabled: true,
            style: "green".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageSegment {
    pub enabled: bool,
    pub style: String,
    pub bar_char: String,
    pub bar_length: u32,
    pub show_bar: bool,
    pub show_percent: bool,
    pub show_reset: bool,
    #[serde(default)]
    pub label: String,
}

impl Default for UsageSegment {
    fn default() -> Self {
        Self {
            enabled: true,
            style: "white".into(),
            bar_char: "shade".into(),
            bar_length: 8,
            show_bar: false,
            show_percent: true,
            show_reset: true,
            label: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathSegment {
    pub enabled: bool,
    pub style: String,
    pub max_length: u32,
}

impl Default for PathSegment {
    fn default() -> Self {
        Self {
            enabled: true,
            style: "ultrathink".into(),
            max_length: 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitSegment {
    pub enabled: bool,
    pub style: String,
    pub show_dirty: bool,
    pub show_remote: bool,
}

impl Default for GitSegment {
    fn default() -> Self {
        Self {
            enabled: true,
            style: "cyan".into(),
            show_dirty: true,
            show_remote: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContextSegment {
    pub enabled: bool,
    pub style: String,
    pub bar_char: String,
    pub bar_length: u32,
    pub show_bar: bool,
    pub show_percent: bool,
    pub show_size: bool,
}

impl Default for ContextSegment {
    fn default() -> Self {
        Self {
            enabled: true,
            style: "semantic".into(),
            bar_char: "shade".into(),
            bar_length: 12,
            show_bar: true,
            show_percent: true,
            show_size: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoSegment {
    pub enabled: bool,
    pub style: String,
    pub refresh_interval: u64,
    pub coins: Vec<String>,
}

impl Default for CryptoSegment {
    fn default() -> Self {
        Self {
            enabled: true,
            style: "green".into(),
            refresh_interval: 60,
            coins: vec!["BTC".into()],
        }
    }
}

// ---------------------------------------------------------------------------
// Segments container
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Segments {
    pub model: ModelSegment,
    pub cost: CostSegment,
    pub usage: UsageSegment,
    pub usage_7d: UsageSegment,
    pub path: PathSegment,
    pub git: GitSegment,
    pub context: ContextSegment,
    pub crypto: CryptoSegment,
}

// ---------------------------------------------------------------------------
// Top-level Config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub lang: String,

    /// Primary layout: each inner Vec is one row of segments.
    /// e.g. `[["model","cost","context"], ["usage","usage_7d"]]`
    #[serde(default)]
    pub rows: Vec<Vec<String>>,

    /// Legacy: single-row order (used when `rows` is empty).
    #[serde(default)]
    pub order: Vec<String>,

    /// Legacy: second row (used when `rows` is empty).
    #[serde(default, alias = "order_row2", rename = "orderRow2")]
    pub order_row2: Vec<String>,

    pub segments: Segments,
}

impl Config {
    /// Maximum number of status line rows.
    const MAX_ROWS: usize = 3;

    /// Return effective rows, preferring `rows` over legacy `order`+`order_row2`.
    /// Capped at [`MAX_ROWS`] rows.
    pub fn effective_rows(&self) -> Vec<&[String]> {
        let rows: Vec<&[String]> = if !self.rows.is_empty() {
            self.rows.iter().map(|r| r.as_slice()).collect()
        } else {
            let mut r = vec![self.order.as_slice()];
            if !self.order_row2.is_empty() {
                r.push(self.order_row2.as_slice());
            }
            r
        };
        rows.into_iter().take(Self::MAX_ROWS).collect()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: String::new(),
            rows: Vec::new(),
            order: vec![
                "model".into(),
                "cost".into(),
                "path".into(),
                "git".into(),
                "context".into(),
                "crypto".into(),
            ],
            order_row2: Vec::new(),
            segments: Segments::default(),
        }
    }
}

// ---------------------------------------------------------------------------
// Load / Save
// ---------------------------------------------------------------------------

/// Load config from `config_path()`. Returns `Config::default()` on any error.
pub fn load_config() -> Config {
    let path = config_path();
    match fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

/// Save config to `config_path()`. Creates the parent directory if needed.
pub fn save_config(config: &Config) -> std::io::Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(config)
        .map_err(std::io::Error::other)?;
    fs::write(&path, json)
}
