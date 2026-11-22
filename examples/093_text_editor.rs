//! 093 — Text Editor: a caret-driven scratch editor. Type to insert at the
//! caret, ENTER breaks lines, BACKSPACE deletes, arrows move the caret
//! (clamped per line), and the buffer is capped at 120 lines of 80 chars.
//! The caret blinks; a status line shows LINE/COL and the character count.
//! `C` clears the buffer (ignored while Ctrl is held).

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1080.0;
const H: f64 = 580.0;
const MAX_LINES: usize = 120;
const MAX_COLS: usize = 80;

struct Editor {
    lines: Vec<String>,
    row: usize,
    col: usize,
    scroll: usize,
}

impl Editor {
    fn new() -> Editor {
        Editor {
            lines: vec![
                "METAL-CPP-RS TEXT EDITOR".to_string(),
                String::new(),
                "TYPE TO INSERT. ENTER FOR A NEW LINE. BACKSPACE DELETES.".to_string(),
                "ARROWS MOVE THE CARET. C CLEARS THE BUFFER.".to_string(),
                String::new(),
            ],
            row: 4,
            col: 0,
            scroll: 0,
        }
    }

    fn line_len(&self, row: usize) -> usize {
        self.lines.get(row).map(|l| l.len()).unwrap_or(0)
    }

    fn char_count(&self) -> usize {
        self.lines.iter().map(|l| l.len()).sum::<usize>() + self.lines.len() - 1
    }

    fn insert_char(&mut self, ch: char) {
        if ch != ' ' && !ch.is_ascii_graphic() {
            return;
        }
        if self.col >= MAX_COLS || self.line_len(self.row) >= MAX_COLS {
            return;
        }
        let idx = self.col.min(self.lines[self.row].len());
        self.lines[self.row].insert(idx, ch);
        self.col += 1;
    }

    fn break_line(&mut self) {
        if self.lines.len() >= MAX_LINES {
            return;
        }
        let cut = self.col.min(self.lines[self.row].len());
        let rest = self.lines[self.row][cut..].to_string();
        self.lines[self.row].truncate(cut);
        self.lines.insert(self.row + 1, rest);
        self.row += 1;
        self.col = 0;
    }

    fn backspace(&mut self) {
        if self.col > 0 {
            let idx = (self.col - 1).min(self.lines[self.row].len());
            self.lines[self.row].remove(idx);
            self.col -= 1;
        } else if self.row > 0 {
            let prev_len = self.lines[self.row - 1].len();
            if prev_len + self.lines[self.row].len() <= MAX_COLS {
                let cur = self.lines.remove(self.row);
                self.row -= 1;
                self.lines[self.row].push_str(&cur);
                self.col = prev_len;
            }
        }
    }

    fn move_caret(&mut self, dx: i32, dy: i32) {
        if dx < 0 {
            if self.col > 0 {
                self.col -= 1;
            } else if self.row > 0 {
                self.row -= 1;
                self.col = self.line_len(self.row);
            }
        } else if dx > 0 {
            if self.col < self.line_len(self.row) {
                self.col += 1;
            } else if self.row + 1 < self.lines.len() {
                self.row += 1;
                self.col = 0;
            }
        }
        if dy < 0 {
            self.row = self.row.saturating_sub(1);
        } else if dy > 0 {
            self.row = (self.row + 1).min(self.lines.len() - 1);
        }
        self.col = self.col.min(self.line_len(self.row));
    }

    fn keep_visible(&mut self, visible: usize) {
        if visible == 0 {
            return;
        }
        if self.row < self.scroll {
            self.scroll = self.row;
        }
        if self.row >= self.scroll + visible.saturating_sub(1) {
            self.scroll = self.row + 1 - visible.saturating_sub(1).max(1);
        }
        self.scroll = self.scroll.min(self.lines.len().saturating_sub(1));
    }
}

fn main() {
    let mut editor = Editor::new();
    let app = gui::App::new("093 - Text Editor", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.09, 0.09, 0.12));
        let s = f.scale;

        // ------------------------------------------------------------- layout
        let margin = 24.0 * s;
        let status_h = 40.0 * s;
        let font_size = 2.0 * s;
        // font::LINE (9) in units of glyph size.
        let line_h = 9.0 * font_size;
        let text_x = margin + 52.0 * s;
        let text_y = margin + 26.0 * s;
        let text_h = f.height - text_y - status_h;
        let visible = ((text_h / line_h) as usize).max(1);

        // ------------------------------------------------------------- input
        for ch in f.input.text().chars() {
            editor.insert_char(ch);
        }
        if f.input.key_pressed(gui::key::ENTER) {
            editor.break_line();
        }
        if f.input.key_pressed(gui::key::BACKSPACE) {
            editor.backspace();
        }
        if f.input.key_pressed(gui::key::LEFT) {
            editor.move_caret(-1, 0);
        }
        if f.input.key_pressed(gui::key::RIGHT) {
            editor.move_caret(1, 0);
        }
        if f.input.key_pressed(gui::key::UP) {
            editor.move_caret(0, -1);
        }
        if f.input.key_pressed(gui::key::DOWN) {
            editor.move_caret(0, 1);
        }
        if f.input.key_pressed(gui::key::C) && !f.input.ctrl() {
            editor.lines = vec![String::new()];
            editor.row = 0;
            editor.col = 0;
            editor.scroll = 0;
        }
        editor.keep_visible(visible);

        // ------------------------------------------------------------- page
        f.rect(
            margin,
            margin,
            f.width - 2.0 * margin,
            f.height - 2.0 * margin - status_h * 0.4,
            gui::Color::rgb(0.13, 0.13, 0.17),
        );

        // Line numbers + text.
        for (i, line) in editor
            .lines
            .iter()
            .enumerate()
            .skip(editor.scroll)
            .take(visible)
        {
            let y = text_y + (i - editor.scroll) as f32 * line_h;
            let num = format!("{:>3}", i + 1);
            let num_color = if i == editor.row {
                gui::Color::rgb(0.5, 0.7, 1.0)
            } else {
                gui::Color::rgb(0.34, 0.36, 0.44)
            };
            f.text(margin + 6.0 * s, y, font_size, &num, num_color);
            f.text(
                text_x,
                y,
                font_size,
                line,
                gui::Color::rgb(0.88, 0.89, 0.94),
            );
        }

        // Blinking caret.
        if (f.time % 1.06) < 0.53
            && editor.row >= editor.scroll
            && editor.row < editor.scroll + visible
        {
            let caret_y = text_y + (editor.row - editor.scroll) as f32 * line_h;
            let caret_x = text_x + editor.col as f32 * font_size * 6.0;
            f.rect(
                caret_x,
                caret_y,
                font_size * 1.2,
                line_h - 2.0 * s,
                gui::Color::YELLOW,
            );
        }

        // ------------------------------------------------------------- status
        let status_y = f.height - status_h;
        f.rect(
            margin,
            status_y,
            f.width - 2.0 * margin,
            status_h - 8.0 * s,
            gui::Color::rgb(0.16, 0.16, 0.21),
        );
        let status = format!(
            "LINE {}  COL {}   {} CHARS   {} / {} LINES",
            editor.row + 1,
            editor.col,
            editor.char_count(),
            editor.lines.len(),
            MAX_LINES
        );
        f.text(
            margin + 10.0 * s,
            status_y + 12.0 * s,
            2.5 * s,
            &status,
            gui::Color::rgb(0.7, 0.72, 0.8),
        );
        let caps = format!("CAP {} COLS", MAX_COLS);
        f.text(
            f.width - margin - f.text_width(2.5 * s, &caps) - 10.0 * s,
            status_y + 12.0 * s,
            2.5 * s,
            &caps,
            gui::Color::GRAY,
        );

        f.text(
            margin,
            f.height - 16.0 * s,
            2.0 * s,
            "TYPE INSERTS - ENTER NEW LINE - BKSP DELETE - ARROWS - C CLEARS",
            gui::Color::GRAY,
        );
    });
}
