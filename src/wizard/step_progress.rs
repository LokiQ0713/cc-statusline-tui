// ── Step progress indicator ──────────────────────────────────────────

/// Metadata for a single step in the wizard progress bar.
#[derive(Clone)]
pub struct StepInfo {
    pub label: String,
    pub summary: Option<String>,
}

// ── ANSI color constants ────────────────────────────────────────────

const GREEN: &str = "\x1b[32m";
const BRIGHT_CYAN: &str = "\x1b[1;36m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

// ── Public API ──────────────────────────────────────────────────────

/// Render a single step line with icon, color, and label.
///
/// Example output: `  ● 1/4 Select segments`
pub fn render_step_line(icon: &str, label: &str, color: &str) -> String {
    format!("  {}{} {}{}", color, icon, label, RESET)
}

/// Render the full vertical step progress indicator.
///
/// ```text
///   ● 1/4 Select segments        ← completed (dim green)
///   │   Segments: Model, Cost
///   │
///   ◆ 2/4 Configure (1/5)        ← current (bright cyan)
///   │
///   ○ 3/4 Reorder                ← pending (dim)
///   │
///   ○ 4/4 Confirm
/// ```
pub fn render_step_progress(steps: &[StepInfo], current_idx: usize) -> String {
    let total = steps.len();
    let mut lines: Vec<String> = Vec::new();

    for (i, step) in steps.iter().enumerate() {
        let step_num = format!("{}/{}", i + 1, total);

        if i < current_idx {
            // Completed step: dim green ●
            lines.push(render_step_line(
                "\u{25CF}",
                &format!("{} {}", step_num, step.label),
                GREEN,
            ));
            // Show summary if present
            if let Some(ref summary) = step.summary {
                lines.push(format!("  {}│   {}{}", DIM, summary, RESET));
            }
            // Connector line
            lines.push(format!("  {}│{}", DIM, RESET));
        } else if i == current_idx {
            // Current step: bright cyan ◆
            lines.push(render_step_line(
                "\u{25C6}",
                &format!("{} {}", step_num, step.label),
                BRIGHT_CYAN,
            ));
            // Connector line (unless last step)
            if i < total - 1 {
                lines.push(format!("  {}│{}", DIM, RESET));
            }
        } else {
            // Pending step: dim ○
            lines.push(render_step_line(
                "\u{25CB}",
                &format!("{} {}", step_num, step.label),
                DIM,
            ));
            // Connector line (unless last step)
            if i < total - 1 {
                lines.push(format!("  {}│{}", DIM, RESET));
            }
        }
    }

    lines.join("\n")
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_steps() -> Vec<StepInfo> {
        vec![
            StepInfo {
                label: "Select segments".into(),
                summary: None,
            },
            StepInfo {
                label: "Configure".into(),
                summary: None,
            },
            StepInfo {
                label: "Reorder".into(),
                summary: None,
            },
            StepInfo {
                label: "Confirm".into(),
                summary: None,
            },
        ]
    }

    #[test]
    fn test_step_info_creation() {
        let step = StepInfo {
            label: "Select segments".into(),
            summary: Some("Model, Cost".into()),
        };
        assert_eq!(step.label, "Select segments");
        assert_eq!(step.summary.as_deref(), Some("Model, Cost"));

        let step2 = step.clone();
        assert_eq!(step2.label, "Select segments");
    }

    #[test]
    fn test_render_step_progress_first() {
        let steps = make_steps();
        let result = render_step_progress(&steps, 0);

        // First step is current (◆)
        assert!(result.contains('\u{25C6}'));
        assert!(result.contains("1/4 Select segments"));

        // Remaining steps are pending (○)
        assert!(result.contains('\u{25CB}'));
        assert!(result.contains("2/4 Configure"));
        assert!(result.contains("3/4 Reorder"));
        assert!(result.contains("4/4 Confirm"));

        // No completed steps (●)
        assert!(!result.contains('\u{25CF}'));
    }

    #[test]
    fn test_render_step_progress_middle() {
        let mut steps = make_steps();
        steps[0].summary = Some("Segments: Model, Cost".into());
        steps[1].summary = Some("Done".into());
        let result = render_step_progress(&steps, 2);

        // First two are completed (●)
        let completed_count = result.matches('\u{25CF}').count();
        assert_eq!(completed_count, 2);

        // Third is current (◆)
        assert!(result.contains('\u{25C6}'));
        assert!(result.contains("3/4 Reorder"));

        // Fourth is pending (○)
        assert!(result.contains('\u{25CB}'));
        assert!(result.contains("4/4 Confirm"));
    }

    #[test]
    fn test_render_step_progress_with_summary() {
        let mut steps = make_steps();
        steps[0].summary = Some("Segments: Model, Cost".into());
        let result = render_step_progress(&steps, 1);

        // Completed step should show summary
        assert!(result.contains("Segments: Model, Cost"));
        // First step is completed (●)
        assert!(result.contains('\u{25CF}'));
        // Second step is current (◆)
        assert!(result.contains('\u{25C6}'));
        assert!(result.contains("2/4 Configure"));
    }

    #[test]
    fn test_render_step_progress_last() {
        let mut steps = make_steps();
        steps[0].summary = Some("Done".into());
        steps[1].summary = Some("Done".into());
        steps[2].summary = Some("Done".into());
        let result = render_step_progress(&steps, 3);

        // First three are completed (●)
        let completed_count = result.matches('\u{25CF}').count();
        assert_eq!(completed_count, 3);

        // Last is current (◆)
        assert!(result.contains('\u{25C6}'));
        assert!(result.contains("4/4 Confirm"));

        // No pending steps (○)
        assert!(!result.contains('\u{25CB}'));
    }
}
