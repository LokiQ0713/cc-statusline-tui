//! Installation logic: save config, copy binary, update Claude Code settings.
//!
//! Called at the end of the wizard after the user confirms. Performs three
//! steps atomically:
//! 1. Save `Config` to `~/.claude/statusline/config.json`
//! 2. Copy the running binary to `~/.claude/statusline/bin`
//! 3. Insert/update `statusLine` in `~/.claude/settings.json`
//!
//! Key function: `save_and_apply(config)` -- orchestrates all three steps.
//!
//! After installation, Claude Code picks up the new statusline command
//! on its next refresh cycle.

use std::fs;
use std::path::Path;

/// Save config, copy the running binary into `~/.claude/statusline/bin`,
/// and update `~/.claude/settings.json` so Claude uses it.
pub fn save_and_apply(config: &crate::config::Config) -> Result<(), String> {
    // 1. Save config
    crate::config::save_config(config).map_err(|e| e.to_string())?;

    // 2. Copy current binary to ~/.claude/statusline/bin
    let self_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let target = crate::config::bin_path();
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    // Only copy if source and target differ (avoid copying to self)
    if self_path.canonicalize().ok() != target.canonicalize().ok() {
        fs::copy(&self_path, &target).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&target, fs::Permissions::from_mode(0o755))
                .map_err(|e| e.to_string())?;
        }
    }

    // 3. Update ~/.claude/settings.json
    update_settings()?;

    Ok(())
}

// ---------------------------------------------------------------------------
// settings.json helpers
// ---------------------------------------------------------------------------

/// Update the real `~/.claude/settings.json`.
fn update_settings() -> Result<(), String> {
    let settings_path = dirs::home_dir()
        .ok_or("cannot find home directory")?
        .join(".claude")
        .join("settings.json");
    update_settings_at(&settings_path)
}

/// Update (or create) a `settings.json` at the given path, inserting or
/// replacing the `statusLine` key while preserving everything else.
fn update_settings_at(settings_path: &Path) -> Result<(), String> {
    if let Some(parent) = settings_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let mut settings: serde_json::Value = if settings_path.exists() {
        let raw = fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    settings["statusLine"] = serde_json::json!({
        "type": "command",
        "command": "~/.claude/statusline/bin/claude-statusline-config --render",
        "padding": 0
    });

    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(settings_path, json + "\n").map_err(|e| e.to_string())?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// Helper: read the settings file and parse it.
    fn read_settings(path: &Path) -> serde_json::Value {
        let raw = fs::read_to_string(path).expect("failed to read settings");
        serde_json::from_str(&raw).expect("invalid JSON")
    }

    /// Helper: build the expected statusLine value.
    fn expected_status_line() -> serde_json::Value {
        serde_json::json!({
            "type": "command",
            "command": "~/.claude/statusline/bin/claude-statusline-config --render",
            "padding": 0
        })
    }

    #[test]
    fn test_update_settings_creates_new() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let settings_path = dir.path().join("settings.json");

        // File must not exist yet.
        assert!(!settings_path.exists());

        update_settings_at(&settings_path).expect("update_settings_at failed");

        assert!(settings_path.exists());
        let v = read_settings(&settings_path);
        assert_eq!(v["statusLine"], expected_status_line());
    }

    #[test]
    fn test_update_settings_preserves_existing() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let settings_path = dir.path().join("settings.json");

        // Write an existing settings file with other fields.
        let existing = serde_json::json!({
            "theme": "dark",
            "verbose": true
        });
        fs::write(&settings_path, serde_json::to_string_pretty(&existing).unwrap())
            .expect("write failed");

        update_settings_at(&settings_path).expect("update_settings_at failed");

        let v = read_settings(&settings_path);
        // Original fields preserved.
        assert_eq!(v["theme"], "dark");
        assert_eq!(v["verbose"], true);
        // statusLine added.
        assert_eq!(v["statusLine"], expected_status_line());
    }

    #[test]
    fn test_update_settings_overwrites_old_statusline() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let settings_path = dir.path().join("settings.json");

        // Write an existing settings file with an outdated statusLine.
        let existing = serde_json::json!({
            "statusLine": {
                "type": "shell",
                "command": "/old/path --status",
                "padding": 2
            },
            "other": "value"
        });
        fs::write(&settings_path, serde_json::to_string_pretty(&existing).unwrap())
            .expect("write failed");

        update_settings_at(&settings_path).expect("update_settings_at failed");

        let v = read_settings(&settings_path);
        // statusLine must be the new value.
        assert_eq!(v["statusLine"], expected_status_line());
        // Other fields preserved.
        assert_eq!(v["other"], "value");
    }

    #[test]
    fn test_update_settings_content() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        let settings_path = dir.path().join("settings.json");

        update_settings_at(&settings_path).expect("update_settings_at failed");

        let v = read_settings(&settings_path);
        let sl = &v["statusLine"];

        // Verify exact structure.
        assert_eq!(sl["type"], "command");
        assert_eq!(sl["command"], "~/.claude/statusline/bin/claude-statusline-config --render");
        assert_eq!(sl["padding"], 0);

        // Ensure no extra keys in statusLine.
        let obj = sl.as_object().expect("statusLine should be an object");
        assert_eq!(obj.len(), 3, "statusLine should have exactly 3 keys");

        // File should end with a newline.
        let raw = fs::read_to_string(&settings_path).unwrap();
        assert!(raw.ends_with('\n'), "settings file should end with newline");
    }
}
