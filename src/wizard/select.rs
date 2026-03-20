use super::terminal::{self, Key};
use std::io::{Write, stdout};

// ── Result & Option types ──────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
pub enum SelectResult {
    Selected(String), // Enter or Right arrow
    Back,             // Left arrow
    Cancelled,        // Esc or Ctrl+C
}

pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub hint: Option<String>,
}

// ── ANSI helpers ───────────────────────────────────────────────────────────

const RESET: &str = "\x1b[0m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";
const BAR: &str = "│";
const CORNER_TL: &str = "┌";
const CORNER_BL: &str = "└";

// ── Drawing ────────────────────────────────────────────────────────────────

/// Render the select UI into a byte buffer.
///
/// Layout (clack-style):
/// ```text
///   ┌ {message}
///   │  ● Option A
///   │  ○ Option B
///   │  ○ Option C (hint)
///   └
/// ```
pub fn draw_select(message: &str, options: &[SelectOption], selected: usize) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();

    // Top: ┌ message
    let _ = write!(buf, "  {CYAN}{CORNER_TL}{RESET} {message}\r\n");

    for (i, opt) in options.iter().enumerate() {
        if i == selected {
            // Selected: bright
            let hint_str = opt.hint.as_deref().map_or(String::new(), |h| format!(" ({h})"));
            let _ = write!(
                buf,
                "  {CYAN}{BAR}{RESET}  \x1b[1m● {}{}{RESET}\r\n",
                opt.label, hint_str
            );
        } else {
            // Not selected: dim
            let hint_str = opt.hint.as_deref().map_or(String::new(), |h| format!(" ({h})"));
            let _ = write!(
                buf,
                "  {CYAN}{BAR}{RESET}  {DIM}○ {}{}{RESET}\r\n",
                opt.label, hint_str
            );
        }
    }

    // Bottom: └
    let _ = write!(buf, "  {CYAN}{CORNER_BL}{RESET}");

    buf
}

/// Erase previous render and redraw.
fn redraw(message: &str, options: &[SelectOption], selected: usize) {
    let mut out = stdout();
    // Move cursor up by (options.len() + 2) lines to cover header + options + footer
    let lines_up = options.len() as u16 + 2;
    let _ = write!(out, "\x1b[{lines_up}A\r");
    // Clear from cursor to end of screen
    let _ = write!(out, "\x1b[J");
    let rendered = draw_select(message, options, selected);
    let _ = out.write_all(&rendered);
    let _ = out.flush();
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Single-select with live preview.
///
/// `on_change` is called on every arrow key for live preview updates.
/// Returns [`SelectResult`].
pub fn select(
    message: &str,
    options: &[SelectOption],
    initial: Option<&str>,
    on_change: &mut dyn FnMut(&str),
) -> SelectResult {
    if options.is_empty() {
        return SelectResult::Cancelled;
    }

    // 1. Find initial index from `initial` value, default to 0
    let mut idx: usize = initial
        .and_then(|v| options.iter().position(|o| o.value == v))
        .unwrap_or(0);

    // 2. Draw the select UI (initial render)
    let mut out = stdout();
    let rendered = draw_select(message, options, idx);
    let _ = out.write_all(&rendered);
    let _ = out.flush();

    // 3. Enter raw mode + hide cursor
    terminal::enable_raw_mode();
    terminal::hide_cursor();

    // 4. Key loop
    let result = loop {
        match terminal::read_key() {
            Key::Up => {
                if idx > 0 {
                    idx -= 1;
                    on_change(&options[idx].value);
                    redraw(message, options, idx);
                }
            }
            Key::Down => {
                if idx < options.len() - 1 {
                    idx += 1;
                    on_change(&options[idx].value);
                    redraw(message, options, idx);
                }
            }
            Key::Enter | Key::Right => {
                break SelectResult::Selected(options[idx].value.clone());
            }
            Key::Left => {
                break SelectResult::Back;
            }
            Key::Escape | Key::CtrlC => {
                break SelectResult::Cancelled;
            }
            _ => {}
        }
    };

    // 5. Restore terminal state
    terminal::show_cursor();
    terminal::disable_raw_mode();

    // Final newline so the caller starts on a clean line
    let _ = writeln!(stdout());

    result
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_result_variants() {
        // Selected variant holds a String
        let sel = SelectResult::Selected("foo".to_string());
        assert_eq!(sel, SelectResult::Selected("foo".to_string()));
        assert_ne!(sel, SelectResult::Back);
        assert_ne!(sel, SelectResult::Cancelled);

        // Back and Cancelled are distinct
        assert_eq!(SelectResult::Back, SelectResult::Back);
        assert_eq!(SelectResult::Cancelled, SelectResult::Cancelled);
        assert_ne!(SelectResult::Back, SelectResult::Cancelled);
    }

    #[test]
    fn test_select_result_debug() {
        assert_eq!(
            format!("{:?}", SelectResult::Selected("x".into())),
            "Selected(\"x\")"
        );
        assert_eq!(format!("{:?}", SelectResult::Back), "Back");
        assert_eq!(format!("{:?}", SelectResult::Cancelled), "Cancelled");
    }

    #[test]
    fn test_select_option_creation() {
        let opt = SelectOption {
            value: "val".to_string(),
            label: "Label".to_string(),
            hint: None,
        };
        assert_eq!(opt.value, "val");
        assert_eq!(opt.label, "Label");
        assert!(opt.hint.is_none());

        let opt_hint = SelectOption {
            value: "v2".to_string(),
            label: "Label 2".to_string(),
            hint: Some("a hint".to_string()),
        };
        assert_eq!(opt_hint.hint.as_deref(), Some("a hint"));
    }

    #[test]
    fn test_draw_select_output_structure() {
        let options = vec![
            SelectOption {
                value: "a".into(),
                label: "Alpha".into(),
                hint: None,
            },
            SelectOption {
                value: "b".into(),
                label: "Beta".into(),
                hint: Some("recommended".into()),
            },
            SelectOption {
                value: "c".into(),
                label: "Gamma".into(),
                hint: None,
            },
        ];

        let buf = draw_select("Pick one", &options, 0);
        let output = String::from_utf8(buf).unwrap();

        // Header line contains the message
        assert!(output.contains("Pick one"));

        // Selected marker for first option
        assert!(output.contains("● Alpha"));

        // Unselected markers
        assert!(output.contains("○ Beta"));
        assert!(output.contains("○ Gamma"));

        // Hint appears in parentheses
        assert!(output.contains("(recommended)"));

        // Box-drawing characters
        assert!(output.contains("┌"));
        assert!(output.contains("│"));
        assert!(output.contains("└"));
    }

    #[test]
    fn test_draw_select_different_selection() {
        let options = vec![
            SelectOption {
                value: "a".into(),
                label: "Alpha".into(),
                hint: None,
            },
            SelectOption {
                value: "b".into(),
                label: "Beta".into(),
                hint: None,
            },
        ];

        // Select index 1
        let buf = draw_select("Test", &options, 1);
        let output = String::from_utf8(buf).unwrap();

        // Alpha should be unselected (○), Beta should be selected (●)
        assert!(output.contains("○ Alpha"));
        assert!(output.contains("● Beta"));
    }

    #[test]
    fn test_draw_select_single_option() {
        let options = vec![SelectOption {
            value: "only".into(),
            label: "Only One".into(),
            hint: Some("solo".into()),
        }];

        let buf = draw_select("Single", &options, 0);
        let output = String::from_utf8(buf).unwrap();

        assert!(output.contains("● Only One (solo)"));
    }

    #[test]
    fn test_select_empty_options_returns_cancelled() {
        let options: Vec<SelectOption> = vec![];
        let result = select("Empty", &options, None, &mut |_| {});
        assert_eq!(result, SelectResult::Cancelled);
    }
}
