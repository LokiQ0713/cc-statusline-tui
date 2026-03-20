use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_LOG_SIZE: u64 = 100_000; // 100KB

/// Log an error message to the statusline log file.
///
/// Best-effort: all IO errors are silently ignored.
pub fn error(msg: &str) {
    let path = crate::config::log_path();
    let _ = error_to_path(&path, msg);
}

/// Internal implementation that writes to an arbitrary path.
/// Returns `Result` so tests can verify behaviour, but the public
/// `error()` function discards any error.
fn error_to_path(path: &Path, msg: &str) -> std::io::Result<()> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // If the file already exists and exceeds MAX_LOG_SIZE, truncate it
    if let Ok(meta) = fs::metadata(path) {
        if meta.len() > MAX_LOG_SIZE {
            // Overwrite with empty content to truncate
            fs::write(path, b"")?;
        }
    }

    // Build the log line
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let line = format!("[{timestamp}] ERROR: {msg}\n");

    // Append to log file (create if it doesn't exist)
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    /// Helper: create a temp directory and return a log file path inside it.
    fn temp_log_path(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("claude_log_test_{name}_{}", std::process::id()));
        // Clean up from any previous run
        let _ = fs::remove_dir_all(&dir);
        dir.join("statusline.log")
    }

    #[test]
    fn test_error_writes_to_file() {
        let path = temp_log_path("write");
        error_to_path(&path, "something went wrong").unwrap();

        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("ERROR:"), "log line should contain ERROR:");
        assert!(
            contents.contains("something went wrong"),
            "log line should contain the message"
        );
        // Verify timestamp bracket format: [digits]
        assert!(
            contents.starts_with('['),
            "log line should start with timestamp bracket"
        );

        // Cleanup
        let _ = fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn test_error_truncates_large_file() {
        let path = temp_log_path("truncate");
        // Create parent dir and a file larger than MAX_LOG_SIZE
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let big_content = "x".repeat(MAX_LOG_SIZE as usize + 1000);
        fs::write(&path, &big_content).unwrap();

        // Verify it's actually big
        assert!(fs::metadata(&path).unwrap().len() > MAX_LOG_SIZE);

        // Write an error — should truncate first, then append
        error_to_path(&path, "after truncate").unwrap();

        let contents = fs::read_to_string(&path).unwrap();
        // File should now be small (just the one new log line)
        assert!(
            contents.len() < 200,
            "file should have been truncated, got {} bytes",
            contents.len()
        );
        assert!(
            contents.contains("after truncate"),
            "new entry should exist after truncation"
        );

        // Cleanup
        let _ = fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn test_error_creates_directory() {
        let path = temp_log_path("mkdir");
        let parent = path.parent().unwrap();

        // Make sure the directory does NOT exist
        let _ = fs::remove_dir_all(parent);
        assert!(!parent.exists(), "parent dir should not exist before test");

        // error_to_path should create it
        error_to_path(&path, "dir created").unwrap();

        assert!(parent.exists(), "parent directory should have been created");
        let contents = fs::read_to_string(&path).unwrap();
        assert!(contents.contains("dir created"));

        // Cleanup
        let _ = fs::remove_dir_all(parent);
    }
}
