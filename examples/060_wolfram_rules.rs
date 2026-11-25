//! 060 — Wolfram Rules: one-dimensional elementary cellular automata drawn
//! as a scrolling history — the newest generation appears at the top and
//! older rows slide downward. SPACE advances to the next rule in the cycle
//! (30, 45, 60, 90, 105, 110, 150); left alone the rule auto-advances every
//! six seconds and the pattern reseeds.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 960.0;
const H: f64 = 760.0;
const COLS: usize = 150;
const ROWS: usize = 100;
const STEP_EVERY: f32 = 0.1;
const AUTO_ADVANCE: f32 = 6.0;
const RULES: [u8; 7] = [30, 45, 60, 90, 105, 110, 150];
const RULE_NAMES: [&str; 7] = [
    "CHAOS",
    "CHAOS",
    "SIERPINSKI",
    "SIERPINSKI",
    "COMPLEX",
    "TURING",
    "SIERPINSKI",
];

struct Wolfram {
    history: Vec<Vec<u8>>,
    rule_ix: usize,
    acc: f32,
    rule_age: f32,
}

impl Wolfram {
    fn reseed(&mut self) {
        let mut row = vec![0u8; COLS];
        row[COLS / 2] = 1;
        self.history.clear();
        self.history.push(row);
    }

    fn step(&mut self) {
        let rule = RULES[self.rule_ix] as u32;
        let top = self.history[0].clone();
        let mut next = vec![0u8; COLS];
        for x in 0..COLS {
            let l = top[(x + COLS - 1) % COLS] as u32;
            let c = top[x] as u32;
            let r = top[(x + 1) % COLS] as u32;
            next[x] = ((rule >> (l * 4 + c * 2 + r)) & 1) as u8;
        }
        self.history.insert(0, next);
        if self.history.len() > ROWS {
            self.history.truncate(ROWS);
        }
    }

    fn next_rule(&mut self) {
        self.rule_ix = (self.rule_ix + 1) % RULES.len();
        self.rule_age = 0.0;
        self.reseed();
    }
}

fn main() {
    let mut ca = Wolfram {
        history: Vec::new(),
        rule_ix: 0,
        acc: 0.0,
        rule_age: 0.0,
    };
    ca.reseed();
    let app = gui::App::new("060 - Wolfram Rules", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.05, 0.05, 0.08));

        // Input: SPACE cycles the rule; the timer auto-advances every 6 s.
        if f.input.key_pressed(gui::key::SPACE) {
            ca.next_rule();
        }
        ca.rule_age += f.dt;
        if ca.rule_age >= AUTO_ADVANCE {
            ca.next_rule();
        }
        ca.acc += f.dt;
        if ca.acc >= STEP_EVERY {
            ca.acc = 0.0;
            ca.step();
        }

        // Header.
        f.text(
            30.0,
            18.0,
            4.0,
            "WOLFRAM ELEMENTARY RULES",
            gui::Color::WHITE,
        );
        let label = format!("RULE {} - {}", RULES[ca.rule_ix], RULE_NAMES[ca.rule_ix]);
        f.text(
            f.width - 30.0 - f.text_width(3.0, &label),
            22.0,
            3.0,
            &label,
            gui::Color::YELLOW,
        );

        // Board: newest generation on top, history scrolling downward. The
        // cell edge follows the window width so the board fills it.
        let cell = ((f.width * 0.84) / COLS as f32).floor().max(4.0);
        let bw = COLS as f32 * cell;
        let bh = ROWS as f32 * cell;
        let bx = (f.width - bw) * 0.5;
        let by = 96.0;
        f.rect(
            bx - 4.0,
            by - 4.0,
            bw + 8.0,
            bh + 8.0,
            gui::Color::rgb(0.09, 0.09, 0.13),
        );
        for (row, cells) in ca.history.iter().enumerate() {
            let fade = 1.0 - row as f32 / ROWS as f32;
            let ink = gui::Color::lerp(
                gui::Color::rgb(0.12, 0.35, 0.45),
                gui::Color::rgb(0.55, 0.95, 1.0),
                fade,
            );
            for x in 0..COLS {
                if cells[x] == 1 {
                    f.rect(
                        bx + x as f32 * cell,
                        by + row as f32 * cell,
                        cell - 1.0,
                        cell - 1.0,
                        ink,
                    );
                }
            }
        }
        f.frame_rect(
            bx - 4.0,
            by - 4.0,
            bw + 8.0,
            bh + 8.0,
            2.0,
            gui::Color::rgb(0.28, 0.28, 0.38),
        );

        f.text(
            bx,
            f.height - 40.0,
            2.0,
            "SPACE NEXT RULE - AUTO EVERY 6S - PATTERN RESEEDS ON CHANGE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
