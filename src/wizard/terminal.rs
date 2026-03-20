use crossterm::{
    cursor, execute, queue,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style::Print,
    terminal,
};
use std::io::{Write, stdout};

// ── Terminal control ────────────────────────────────────────────────

pub fn enable_raw_mode() {
    terminal::enable_raw_mode().unwrap();
}

pub fn disable_raw_mode() {
    terminal::disable_raw_mode().unwrap();
}

pub fn clear_screen() {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}

pub fn move_to(row: u16, col: u16) {
    execute!(stdout(), cursor::MoveTo(col, row)).unwrap();
}

pub fn clear_line() {
    execute!(stdout(), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();
}

/// Print text at a specific row without moving the cursor permanently.
pub fn print_at(row: u16, text: &str) {
    let mut out = stdout();
    queue!(
        out,
        cursor::SavePosition,
        cursor::MoveTo(0, row),
        terminal::Clear(terminal::ClearType::CurrentLine),
        Print(text),
        cursor::RestorePosition
    )
    .unwrap();
    out.flush().unwrap();
}

pub fn hide_cursor() {
    execute!(stdout(), cursor::Hide).unwrap();
}

pub fn show_cursor() {
    execute!(stdout(), cursor::Show).unwrap();
}

pub fn flush() {
    stdout().flush().unwrap();
}

// ── Key abstraction ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Space,
    Char(char),
    Escape,
    CtrlC,
}

/// Block until the user presses a key and return the corresponding [`Key`].
///
/// **Note:** crossterm 0.28 on macOS fires both `KeyEventKind::Press` and
/// `KeyEventKind::Release` for every physical key press.  We filter to only
/// handle `Press` events so each key is processed exactly once.
pub fn read_key() -> Key {
    loop {
        if let Event::Key(KeyEvent {
            code,
            modifiers,
            kind,
            ..
        }) = event::read().unwrap()
        {
            // Only handle Press events — ignore Release/Repeat.
            if kind != crossterm::event::KeyEventKind::Press {
                continue;
            }
            return match code {
                KeyCode::Up => Key::Up,
                KeyCode::Down => Key::Down,
                KeyCode::Left => Key::Left,
                KeyCode::Right => Key::Right,
                KeyCode::Enter => Key::Enter,
                KeyCode::Char(' ') => Key::Space,
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => Key::CtrlC,
                KeyCode::Char(c) => Key::Char(c),
                KeyCode::Esc => Key::Escape,
                _ => continue,
            };
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_enum_equality() {
        assert_eq!(Key::Up, Key::Up);
        assert_eq!(Key::Down, Key::Down);
        assert_eq!(Key::Left, Key::Left);
        assert_eq!(Key::Right, Key::Right);
        assert_eq!(Key::Enter, Key::Enter);
        assert_eq!(Key::Space, Key::Space);
        assert_eq!(Key::Escape, Key::Escape);
        assert_eq!(Key::CtrlC, Key::CtrlC);
        assert_eq!(Key::Char('a'), Key::Char('a'));
        assert_ne!(Key::Char('a'), Key::Char('b'));
        assert_ne!(Key::Up, Key::Down);
    }

    #[test]
    fn test_key_debug() {
        assert_eq!(format!("{:?}", Key::Up), "Up");
        assert_eq!(format!("{:?}", Key::Down), "Down");
        assert_eq!(format!("{:?}", Key::Left), "Left");
        assert_eq!(format!("{:?}", Key::Right), "Right");
        assert_eq!(format!("{:?}", Key::Enter), "Enter");
        assert_eq!(format!("{:?}", Key::Space), "Space");
        assert_eq!(format!("{:?}", Key::Escape), "Escape");
        assert_eq!(format!("{:?}", Key::CtrlC), "CtrlC");
        assert_eq!(format!("{:?}", Key::Char('x')), "Char('x')");
    }

    #[test]
    fn test_key_clone() {
        let original = Key::Char('z');
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }
}
