//! 063 — Cyclic Cellular Automaton: fourteen states chasing each other
//! around a torus. A cell advances to the next state in the cycle as soon
//! as any of its eight neighbours already holds that state; random noise
//! organizes itself into rotating spiral waves. R reseeds random states,
//! hue encodes state as hsv(state/14).

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1040.0;
const H: f64 = 780.0;
const COLS: usize = 120;
const ROWS: usize = 80;
const CELL: f32 = 8.0;
const BOARD_Y: f32 = 84.0;
const STEP_EVERY: f32 = 0.1;
const STATES: u8 = 14;

struct Cyclic {
    grid: Vec<u8>,
    acc: f32,
}

impl Cyclic {
    fn reseed(&mut self) {
        let mut rng = gui::Rng::from_entropy();
        for c in self.grid.iter_mut() {
            *c = rng.index(STATES as usize) as u8;
        }
    }

    fn step(&mut self) {
        let old = self.grid.clone();
        for y in 0..ROWS {
            for x in 0..COLS {
                let s = old[y * COLS + x];
                let target = (s + 1) % STATES;
                let mut advanced = false;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = (x as i32 + dx).rem_euclid(COLS as i32) as usize;
                        let ny = (y as i32 + dy).rem_euclid(ROWS as i32) as usize;
                        if old[ny * COLS + nx] == target {
                            advanced = true;
                        }
                    }
                }
                self.grid[y * COLS + x] = if advanced { target } else { s };
            }
        }
    }
}

fn main() {
    let mut ca = Cyclic {
        grid: vec![0u8; COLS * ROWS],
        acc: 0.0,
    };
    ca.reseed();
    let app = gui::App::new_points("063 - Cyclic CA", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.07, 0.07, 0.10));

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let bx = ((f.width - bw) * 0.5).max(16.0);

        // Input + simulation.
        if f.input.key_pressed(gui::key::R) {
            ca.reseed();
        }
        ca.acc += f.dt;
        if ca.acc >= STEP_EVERY {
            ca.acc = 0.0;
            ca.step();
        }

        // Header. The title is kept short so it cannot run into the
        // right-aligned status line.
        f.text(bx, 18.0, 4.0, "CYCLIC CA", gui::Color::WHITE);
        f.text(
            bx + bw - f.text_width(2.5, "14 STATES ON A TORUS - MOORE-8"),
            26.0,
            2.5,
            "14 STATES ON A TORUS - MOORE-8",
            gui::Color::GRAY,
        );

        // Board: every state painted with its hsv(state/14) hue.
        f.rect(bx, BOARD_Y, bw, bh, gui::Color::BLACK);
        for y in 0..ROWS {
            for x in 0..COLS {
                let s = ca.grid[y * COLS + x];
                f.rect(
                    bx + x as f32 * CELL,
                    BOARD_Y + y as f32 * CELL,
                    CELL - 1.0,
                    CELL - 1.0,
                    gui::Color::hsv(s as f32 / STATES as f32, 0.85, 0.95),
                );
            }
        }
        f.frame_rect(bx, BOARD_Y, bw, bh, 2.0, gui::Color::rgb(0.3, 0.3, 0.4));

        f.text(
            bx,
            H as f32 - 40.0,
            2.0,
            "R RESEEDS RANDOM STATES - RUNS AT 10 STEPS/S - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
