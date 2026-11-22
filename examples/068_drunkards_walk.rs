//! 068 — Drunkard's Walk: a lone walker carves tunnels out from the center
//! of the map, stumbling in the four cardinal directions with a 50% bias to
//! keep going straight. Freshly carved rock glows bright and cools to ember
//! darkness with age. R digs a new dungeon; the header counts steps carved.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1040.0;
const H: f64 = 720.0;
const COLS: usize = 110;
const ROWS: usize = 70;
const CELL: f32 = 8.0;
const BOARD_Y: f32 = 84.0;
const TICK: f32 = 1.0 / 45.0;
const STEPS_PER_TICK: usize = 2;
const MOMENTUM: f32 = 0.5;
const COOLING: f32 = 300.0;

/// Walker headings — up, right, down, left (screen y grows downward).
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Drunk {
    carved: Vec<bool>,
    age: Vec<u64>,
    x: usize,
    y: usize,
    dir: usize,
    steps: u64,
    count: usize,
    acc: f32,
    rng: gui::Rng,
}

impl Drunk {
    fn reset(&mut self) {
        self.carved.fill(false);
        self.x = COLS / 2;
        self.y = ROWS / 2;
        self.dir = self.rng.index(4);
        self.steps = 0;
        self.count = 0;
        self.acc = 0.0;
        let i = self.y * COLS + self.x;
        self.carved[i] = true;
        self.age[i] = 0;
        self.count = 1;
    }

    fn step(&mut self) {
        if self.rng.f32() >= MOMENTUM {
            self.dir = self.rng.index(4);
        }
        let (dx, dy) = DIRS[self.dir];
        self.x = (self.x as i32 + dx).clamp(0, COLS as i32 - 1) as usize;
        self.y = (self.y as i32 + dy).clamp(0, ROWS as i32 - 1) as usize;
        let i = self.y * COLS + self.x;
        if !self.carved[i] {
            self.carved[i] = true;
            self.age[i] = self.steps;
            self.count += 1;
        }
        self.steps += 1;
    }
}

fn main() {
    let mut drunk = Drunk {
        carved: vec![false; COLS * ROWS],
        age: vec![0; COLS * ROWS],
        x: COLS / 2,
        y: ROWS / 2,
        dir: 0,
        steps: 0,
        count: 0,
        acc: 0.0,
        rng: gui::Rng::from_entropy(),
    };
    drunk.reset();
    let app = gui::App::new("068 - Drunkard's Walk", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.06, 0.05, 0.05));

        // Input + simulation: 90 carving steps per second.
        if f.input.key_pressed(gui::key::R) {
            drunk.reset();
        }
        drunk.acc += f.dt;
        if drunk.acc >= TICK {
            drunk.acc = 0.0;
            for _ in 0..STEPS_PER_TICK {
                drunk.step();
            }
        }

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let bx = ((f.width - bw) * 0.5).max(16.0);

        // Header.
        f.text(bx, 18.0, 4.0, "DRUNKARD'S WALK", gui::Color::WHITE);
        let status = format!("STEPS {}  CARVED {}", drunk.steps, drunk.count);
        f.text(
            bx + bw - f.text_width(2.5, &status),
            26.0,
            2.5,
            &status,
            gui::Color::ORANGE,
        );

        // Board: carved rock glows warm and cools with age.
        f.rect(bx, BOARD_Y, bw, bh, gui::Color::rgb(0.10, 0.08, 0.08));
        for y in 0..ROWS {
            for x in 0..COLS {
                let i = y * COLS + x;
                if !drunk.carved[i] {
                    continue;
                }
                let age = (drunk.steps - drunk.age[i]) as f32;
                let glow = 0.22 + 0.78 * (-age / COOLING).exp();
                f.rect(
                    bx + x as f32 * CELL,
                    BOARD_Y + y as f32 * CELL,
                    CELL - 1.0,
                    CELL - 1.0,
                    gui::Color::hsv(0.06, 0.85, glow),
                );
            }
        }
        f.rect(
            bx + drunk.x as f32 * CELL,
            BOARD_Y + drunk.y as f32 * CELL,
            CELL - 1.0,
            CELL - 1.0,
            gui::Color::WHITE,
        );
        f.frame_rect(bx, BOARD_Y, bw, bh, 2.0, gui::Color::rgb(0.3, 0.3, 0.4));

        f.text(
            bx,
            H as f32 - 40.0,
            2.0,
            "R RESTART - THE DRUNK KEEPS STRAIGHT HALF THE TIME - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
