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
    pub refresh_interval: u64,
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
            refresh_interval: 120,
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
pub struct Segments {
    pub model: ModelSegment,
    pub cost: CostSegment,
    pub usage: UsageSegment,
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
    pub order: Vec<String>,
    pub segments: Segments,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lang: String::new(),
            order: vec![
                "model".into(),
                "cost".into(),
                "usage".into(),
                "path".into(),
                "git".into(),
                "context".into(),
                "crypto".into(),
            ],
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
