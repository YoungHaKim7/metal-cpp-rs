//! 080 — Calculator: a full GUI calculator. Mouse-click the button pad or
//! use the keyboard (digits, `+ - * / % .`, Enter for `=`, Backspace, `C`
//! to clear). The classic immediate-execution model: entry, operator,
//! previous value, repeated `=` is not chained.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 380.0;
const H: f64 = 540.0;

struct Calc {
    entry: String,
    prev: f64,
    op: Option<char>,
    fresh: bool,
}

fn apply(op: char, a: f64, b: f64) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0.0 {
                f64::NAN
            } else {
                a / b
            }
        }
        _ => b,
    }
}

fn fmt(v: f64) -> String {
    if v.is_nan() {
        "ERROR".to_string()
    } else if v == v.trunc() && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        format!("{v}")
    }
}

impl Calc {
    fn new() -> Calc {
        Calc {
            entry: String::new(),
            prev: 0.0,
            op: None,
            fresh: true,
        }
    }

    fn digit(&mut self, d: char) {
        if self.fresh {
            self.entry.clear();
            self.fresh = false;
        }
        if self.entry.len() < 12 {
            self.entry.push(d);
        }
    }

    fn dot(&mut self) {
        if self.fresh {
            self.entry.clear();
            self.fresh = false;
        }
        if !self.entry.contains('.') {
            if self.entry.is_empty() {
                self.entry.push('0');
            }
            self.entry.push('.');
        }
    }

    fn set_op(&mut self, op: char) {
        if !self.entry.is_empty() {
            let cur: f64 = self.entry.parse().unwrap_or(0.0);
            match self.op {
                Some(p) => {
                    self.prev = apply(p, self.prev, cur);
                    self.entry = fmt(self.prev);
                }
                None => self.prev = cur,
            }
        }
        self.op = Some(op);
        self.fresh = true;
    }

    fn equals(&mut self) {
        if let Some(p) = self.op {
            let cur: f64 = self.entry.parse().unwrap_or(0.0);
            self.prev = apply(p, self.prev, cur);
            self.entry = fmt(self.prev);
            self.op = None;
            self.fresh = true;
        }
    }

    fn negate(&mut self) {
        if let Ok(v) = self.entry.parse::<f64>() {
            self.entry = fmt(-v);
        }
    }

    fn percent(&mut self) {
        if let Ok(v) = self.entry.parse::<f64>() {
            self.entry = fmt(v / 100.0);
        }
    }

    fn backspace(&mut self) {
        if !self.fresh {
            self.entry.pop();
        }
    }

    fn clear(&mut self) {
        self.entry.clear();
        self.prev = 0.0;
        self.op = None;
        self.fresh = true;
    }

    fn display(&self) -> String {
        if self.entry.is_empty() {
            "0".to_string()
        } else {
            self.entry.clone()
        }
    }
}

/// (label, column, row, kind) — kind 0 digit, 1 op, 2 function, 3 equals.
const BUTTONS: [(&str, f32, f32, u32); 19] = [
    ("C", 0.0, 0.0, 2),
    ("+/-", 1.0, 0.0, 2),
    ("%", 2.0, 0.0, 2),
    ("\u{00F7}", 3.0, 0.0, 1),
    ("7", 0.0, 1.0, 0),
    ("8", 1.0, 1.0, 0),
    ("9", 2.0, 1.0, 0),
    ("\u{00D7}", 3.0, 1.0, 1),
    ("4", 0.0, 2.0, 0),
    ("5", 1.0, 2.0, 0),
    ("6", 2.0, 2.0, 0),
    ("\u{2212}", 3.0, 2.0, 1),
    ("1", 0.0, 3.0, 0),
    ("2", 1.0, 3.0, 0),
    ("3", 2.0, 3.0, 0),
    ("+", 3.0, 3.0, 1),
    ("0", 0.0, 4.0, 0),
    (".", 1.0, 4.0, 0),
    ("=", 2.0, 4.0, 3),
];

fn main() {
    let mut calc = Calc::new();
    let app = gui::App::new("080 - Calculator", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.09, 0.09, 0.12));

        let margin = 14.0;
        let cell_w = (W as f32 - 2.0 * margin - 3.0 * 8.0) / 4.0;
        let cell_h = 66.0;
        let top = 132.0;
        let gap = 8.0;

        // Display: the running expression and the current entry.
        f.frame_rect(
            margin,
            14.0,
            W as f32 - 2.0 * margin,
            104.0,
            2.0,
            gui::Color::rgb(0.2, 0.22, 0.3),
        );
        let expr = match (calc.prev, calc.op) {
            (p, Some(op)) if calc.fresh => format!("{} {}", fmt(p), op),
            (p, Some(op)) => format!("{} {} {}", fmt(p), op, calc.display()),
            _ => String::new(),
        };
        if !expr.is_empty() {
            f.text(
                W as f32 - margin - f.text_width(2.5, &expr) - 10.0,
                26.0,
                2.5,
                &expr,
                gui::Color::GRAY,
            );
        }
        let shown = calc.display();
        let dw = f.text_width(7.0, &shown);
        f.text(
            (W as f32 - margin - 10.0 - dw).max(margin + 10.0),
            66.0,
            7.0,
            &shown,
            gui::Color::WHITE,
        );

        // Button pad.
        for (label, col, row, kind) in BUTTONS {
            let x = margin + col * (cell_w + gap);
            let y = top + row * (cell_h + gap);
            let base = match kind {
                1 => gui::Color::rgb(0.85, 0.55, 0.15),
                2 => gui::Color::rgb(0.35, 0.37, 0.44),
                3 => gui::Color::rgb(0.20, 0.45, 0.95),
                _ => gui::Color::rgb(0.24, 0.25, 0.30),
            };
            let hovered = {
                let (mx, my) = f.mouse();
                mx >= x && mx <= x + cell_w && my >= y && my <= y + cell_h
            };
            let fill = if hovered {
                gui::Color::lerp(base, gui::Color::WHITE, 0.18)
            } else {
                base
            };
            f.rect(x, y, cell_w, cell_h, fill);
            f.text_center(
                x + cell_w * 0.5,
                y + cell_h * 0.5 - 3.5 * 4.0,
                4.0,
                label,
                gui::Color::WHITE,
            );

            if f.input.mouse_pressed() && hovered {
                match kind {
                    0 => {
                        if let Some(d) = label.chars().next() {
                            if d.is_ascii_digit() {
                                calc.digit(d);
                            }
                        }
                    }
                    1 => calc.set_op(match label {
                        "\u{00D7}" => '*',
                        "\u{00F7}" => '/',
                        "\u{2212}" => '-',
                        _ => '+',
                    }),
                    2 => match label {
                        "C" => calc.clear(),
                        "+/-" => calc.negate(),
                        _ => calc.percent(),
                    },
                    _ => calc.equals(),
                }
            }
        }

        // Keyboard input.
        for ch in f.input.text().chars() {
            match ch {
                '0'..='9' => calc.digit(ch),
                '.' => calc.dot(),
                '+' | '*' => calc.set_op(ch),
                '-' => calc.set_op('-'),
                '/' => calc.set_op('/'),
                '%' => calc.percent(),
                '=' | '\r' | '\n' => calc.equals(),
                _ => {}
            }
        }
        if f.input.key_pressed(gui::key::ENTER) {
            calc.equals();
        }
        if f.input.key_pressed(gui::key::BACKSPACE) {
            calc.backspace();
        }
        if f.input.key_pressed(gui::key::C) {
            calc.clear();
        }
        if f.input.key_pressed(gui::key::NUM_0) {
            calc.digit('0');
        }
        if f.input.key_pressed(gui::key::NUM_1) {
            calc.digit('1');
        }
        if f.input.key_pressed(gui::key::NUM_2) {
            calc.digit('2');
        }
        if f.input.key_pressed(gui::key::NUM_3) {
            calc.digit('3');
        }
        if f.input.key_pressed(gui::key::NUM_4) {
            calc.digit('4');
        }
        if f.input.key_pressed(gui::key::NUM_5) {
            calc.digit('5');
        }
        if f.input.key_pressed(gui::key::NUM_6) {
            calc.digit('6');
        }
        if f.input.key_pressed(gui::key::NUM_7) {
            calc.digit('7');
        }
        if f.input.key_pressed(gui::key::NUM_8) {
            calc.digit('8');
        }
        if f.input.key_pressed(gui::key::NUM_9) {
            calc.digit('9');
        }

        f.text(
            margin,
            H as f32 - 26.0,
            2.0,
            "KEYBOARD: 0-9 . + - * / % = ENTER BACKSPACE C",
            gui::Color::GRAY,
        );
    });
}
