// ── Color codes (ANSI escape sequences) ──────────────────────────────────────

pub fn color_code(style: &str) -> &'static str {
    match style {
        "cyan" => "\x1b[0;36m",
        "green" => "\x1b[0;32m",
        "blue" => "\x1b[1;34m",
        "yellow" => "\x1b[0;33m",
        "magenta" => "\x1b[0;35m",
        "red" => "\x1b[0;31m",
        "white" => "\x1b[0;37m",
        "soft-green" => "\x1b[38;5;71m",
        "soft-yellow" => "\x1b[38;5;179m",
        "soft-red" => "\x1b[38;5;167m",
        "soft-blue" => "\x1b[38;5;75m",
        "soft-cyan" => "\x1b[38;5;80m",
        "soft-magenta" => "\x1b[38;5;176m",
        "orange" => "\x1b[38;5;208m",
        "pink" => "\x1b[38;5;212m",
        "purple" => "\x1b[38;5;141m",
        _ => "\x1b[0;37m", // fallback to white
    }
}

pub const RESET: &str = "\x1b[0m";
pub const DIM: &str = "\x1b[38;5;239m";

// ── Ultrathink rainbow palettes (7 colors each) ─────────────────────────────

pub const ULTRATHINK_MAIN: [(u8, u8, u8); 7] = [
    (235, 95, 87),
    (245, 139, 87),
    (250, 195, 95),
    (145, 200, 130),
    (130, 170, 220),
    (155, 130, 200),
    (200, 130, 180),
];

pub const ULTRATHINK_SHIMMER: [(u8, u8, u8); 7] = [
    (250, 155, 147),
    (255, 185, 137),
    (255, 225, 155),
    (185, 230, 180),
    (180, 205, 240),
    (195, 180, 230),
    (230, 180, 210),
];

// ── Bar characters ───────────────────────────────────────────────────────────

pub struct BarChars {
    pub filled: char,
    pub empty: char,
    pub empty_uses_dim: bool,
}

pub fn bar_chars(name: &str) -> BarChars {
    match name {
        "full-block" => BarChars {
            filled: '\u{2588}', // █
            empty: '\u{2588}',
            empty_uses_dim: true,
        },
        "rectangle" => BarChars {
            filled: '\u{25AC}', // ▬
            empty: '\u{25AC}',
            empty_uses_dim: true,
        },
        _ /* shade */ => BarChars {
            filled: '\u{2593}', // ▓
            empty: '\u{2591}', // ░
            empty_uses_dim: false,
        },
    }
}

// ── Semantic color helper ────────────────────────────────────────────────────

/// Returns the ANSI color code for a "traffic-light" semantic bar:
/// high ratio (>= 0.75) = soft-red, medium (>= 0.50) = soft-yellow, low = soft-green.
pub fn semantic_color(ratio: f64) -> &'static str {
    if ratio >= 0.75 {
        color_code("soft-red")
    } else if ratio >= 0.50 {
        color_code("soft-yellow")
    } else {
        color_code("soft-green")
    }
}

// ── Rendering helpers ────────────────────────────────────────────────────────

/// Produce an ANSI-colored rainbow string using per-character true-color sequences.
///
/// Each character at index `i` uses palette color `(i + offset) % 7`.
pub fn format_rainbow(text: &str, offset: usize, use_shimmer: bool) -> String {
    let palette = if use_shimmer {
        &ULTRATHINK_SHIMMER
    } else {
        &ULTRATHINK_MAIN
    };
    let mut out = String::new();
    for (i, ch) in text.chars().enumerate() {
        let (r, g, b) = palette[(i + offset) % 7];
        out.push_str(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, ch));
    }
    out.push_str(RESET);
    out
}

/// Wrap `text` in the appropriate color.
///
/// - If `style` is `"ultrathink"`, apply per-character rainbow coloring.
///   `timestamp % 2` selects main vs shimmer palette; `timestamp % 7` is the
///   color-rotation offset.
/// - Otherwise, wrap the whole text in `color_code(style)…RESET`.
pub fn format_colored(style: &str, text: &str, timestamp: u64) -> String {
    if style == "ultrathink" {
        let use_shimmer = timestamp % 2 == 1;
        let offset = (timestamp % 7) as usize;
        format_rainbow(text, offset, use_shimmer)
    } else {
        format!("{}{}{}", color_code(style), text, RESET)
    }
}

/// Render a progress bar of `length` characters at the given `ratio` (0.0–1.0).
///
/// Style determines coloring:
/// - `"ultrathink-gradient"`: smooth interpolation across the 7-color rainbow
///   for the filled portion.
/// - `"semantic"`: traffic-light coloring based on ratio.
/// - Anything else: solid color from `color_code(style)`.
///
/// The empty portion uses either DIM + same char (if `empty_uses_dim`) or
/// DIM + the dedicated empty char.
pub fn format_bar(
    style: &str,
    bar_char_name: &str,
    length: usize,
    ratio: f64,
    timestamp: u64,
) -> String {
    let filled = (length as f64 * ratio).round() as usize;
    let empty = length.saturating_sub(filled);
    let bc = bar_chars(bar_char_name);

    let mut out = String::new();

    // ── Filled portion ──────────────────────────────────────────────────
    if style == "ultrathink-gradient" {
        let use_shimmer = timestamp % 2 == 1;
        let palette = if use_shimmer {
            &ULTRATHINK_SHIMMER
        } else {
            &ULTRATHINK_MAIN
        };
        for i in 0..filled {
            let t = if filled <= 1 {
                0.0
            } else {
                i as f64 / (filled as f64 - 1.0)
            };
            let pos = t * 6.0;
            let idx = (pos.floor() as usize).min(5);
            let frac = pos - idx as f64;

            let (r1, g1, b1) = palette[idx];
            let (r2, g2, b2) = palette[(idx + 1).min(6)];

            let r = (r1 as f64 + (r2 as f64 - r1 as f64) * frac).round() as u8;
            let g = (g1 as f64 + (g2 as f64 - g1 as f64) * frac).round() as u8;
            let b = (b1 as f64 + (b2 as f64 - b1 as f64) * frac).round() as u8;

            out.push_str(&format!("\x1b[38;2;{};{};{}m{}", r, g, b, bc.filled));
        }
    } else if style == "semantic" {
        let color = semantic_color(ratio);
        out.push_str(color);
        for _ in 0..filled {
            out.push(bc.filled);
        }
    } else {
        out.push_str(color_code(style));
        for _ in 0..filled {
            out.push(bc.filled);
        }
    }

    // ── Empty portion ───────────────────────────────────────────────────
    out.push_str(DIM);
    let empty_char = if bc.empty_uses_dim {
        bc.filled
    } else {
        bc.empty
    };
    for _ in 0..empty {
        out.push(empty_char);
    }

    out.push_str(RESET);
    out
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_code() {
        assert_eq!(color_code("cyan"), "\x1b[0;36m");
        assert_eq!(color_code("green"), "\x1b[0;32m");
        assert_eq!(color_code("blue"), "\x1b[1;34m");
        assert_eq!(color_code("soft-green"), "\x1b[38;5;71m");
        assert_eq!(color_code("orange"), "\x1b[38;5;208m");
        assert_eq!(color_code("purple"), "\x1b[38;5;141m");
    }

    #[test]
    fn test_color_code_fallback() {
        // Unknown styles fall back to white
        assert_eq!(color_code("nonexistent"), "\x1b[0;37m");
        assert_eq!(color_code(""), "\x1b[0;37m");
        assert_eq!(color_code("neon"), "\x1b[0;37m");
    }

    #[test]
    fn test_bar_chars() {
        let shade = bar_chars("shade");
        assert_eq!(shade.filled, '▓');
        assert_eq!(shade.empty, '░');
        assert!(!shade.empty_uses_dim);

        let full = bar_chars("full-block");
        assert_eq!(full.filled, '█');
        assert_eq!(full.empty, '█');
        assert!(full.empty_uses_dim);

        let rect = bar_chars("rectangle");
        assert_eq!(rect.filled, '▬');
        assert_eq!(rect.empty, '▬');
        assert!(rect.empty_uses_dim);
    }

    #[test]
    fn test_format_colored_plain() {
        let result = format_colored("cyan", "hello", 0);
        assert!(result.starts_with("\x1b[0;36m"));
        assert!(result.contains("hello"));
        assert!(result.ends_with(RESET));
    }

    #[test]
    fn test_format_colored_rainbow() {
        let result = format_colored("ultrathink", "AB", 0);
        // Should contain per-character true-color sequences
        assert!(result.contains("\x1b[38;2;"));
        // Two characters → two separate color codes
        let count = result.matches("\x1b[38;2;").count();
        assert_eq!(count, 2);
        assert!(result.contains('A'));
        assert!(result.contains('B'));
        assert!(result.ends_with(RESET));
    }

    #[test]
    fn test_format_colored_rainbow_shimmer() {
        // timestamp=1 → shimmer, offset=1
        let result = format_colored("ultrathink", "X", 1);
        // Shimmer palette color at offset 1 = index (0+1)%7 = 1 → (255,185,137)
        assert!(result.contains("\x1b[38;2;255;185;137m"));
    }

    #[test]
    fn test_format_bar_length() {
        // Bar with length=10, ratio=0.5 → 5 filled + 5 empty = 10 visible chars
        let result = format_bar("cyan", "shade", 10, 0.5, 0);
        // Count the visible (non-escape) characters
        let visible: String = strip_ansi(&result);
        assert_eq!(visible.chars().count(), 10);
    }

    #[test]
    fn test_format_bar_semantic() {
        // ratio 0.3 → soft-green
        let low = format_bar("semantic", "full-block", 10, 0.3, 0);
        assert!(low.contains(color_code("soft-green")));

        // ratio 0.6 → soft-yellow
        let mid = format_bar("semantic", "full-block", 10, 0.6, 0);
        assert!(mid.contains(color_code("soft-yellow")));

        // ratio 0.9 → soft-red
        let high = format_bar("semantic", "full-block", 10, 0.9, 0);
        assert!(high.contains(color_code("soft-red")));
    }

    #[test]
    fn test_semantic_color() {
        assert_eq!(semantic_color(0.0), color_code("soft-green"));
        assert_eq!(semantic_color(0.49), color_code("soft-green"));
        assert_eq!(semantic_color(0.50), color_code("soft-yellow"));
        assert_eq!(semantic_color(0.74), color_code("soft-yellow"));
        assert_eq!(semantic_color(0.75), color_code("soft-red"));
        assert_eq!(semantic_color(1.0), color_code("soft-red"));
    }

    #[test]
    fn test_format_bar_gradient() {
        // ultrathink-gradient should produce per-char true-color for filled portion
        let result = format_bar("ultrathink-gradient", "full-block", 10, 0.7, 0);
        // 7 filled chars, each with its own true-color sequence
        let tc_count = result.matches("\x1b[38;2;").count();
        assert_eq!(tc_count, 7); // round(10*0.7) = 7
    }

    #[test]
    fn test_format_bar_empty_only() {
        // ratio = 0 → all empty
        let result = format_bar("cyan", "shade", 5, 0.0, 0);
        let visible = strip_ansi(&result);
        assert_eq!(visible.chars().count(), 5);
        // All chars should be the empty char '░'
        assert!(visible.chars().all(|c| c == '░'));
    }

    #[test]
    fn test_format_bar_full_only() {
        // ratio = 1 → all filled
        let result = format_bar("green", "full-block", 4, 1.0, 0);
        let visible = strip_ansi(&result);
        assert_eq!(visible.chars().count(), 4);
        assert!(visible.chars().all(|c| c == '█'));
    }

    // ── Helper: strip ANSI escape sequences ─────────────────────────────

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
}
