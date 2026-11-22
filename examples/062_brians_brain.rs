//! 062 — Brian's Brain: a three-state cellular automaton. Firing cells cool
//! into a refractory state, refractory cells relax to off, and off cells
//! ignite only when exactly two of their eight neighbours are firing — the
//! sparks never settle. Click or drag paints firing cells, R reseeds, and a
//! few stray sparks per step keep the soup boiling forever.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1040.0;
const H: f64 = 780.0;
const COLS: usize = 120;
const ROWS: usize = 80;
const CELL: f32 = 8.0;
const BOARD_Y: f32 = 84.0;
const STEP_EVERY: f32 = 0.1;
const SPARKS_PER_STEP: usize = 3;
const OFF: u8 = 0;
const FIRING: u8 = 1;
const REFRACT: u8 = 2;

struct Brain {
    grid: Vec<u8>,
    rng: gui::Rng,
    acc: f32,
}

impl Brain {
    fn reseed(&mut self) {
        let mut rng = gui::Rng::from_entropy();
        for c in self.grid.iter_mut() {
            *c = if rng.f32() < 0.08 { FIRING } else { OFF };
        }
    }

    fn step(&mut self) {
        let old = self.grid.clone();
        for y in 0..ROWS {
            for x in 0..COLS {
                let mut n = 0u8;
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = (x as i32 + dx).rem_euclid(COLS as i32) as usize;
                        let ny = (y as i32 + dy).rem_euclid(ROWS as i32) as usize;
                        if old[ny * COLS + nx] == FIRING {
                            n += 1;
                        }
                    }
                }
                let i = y * COLS + x;
                self.grid[i] = match old[i] {
                    FIRING => REFRACT,
                    REFRACT => OFF,
                    _ => {
                        if n == 2 {
                            FIRING
                        } else {
                            OFF
                        }
                    }
                };
            }
        }
        for _ in 0..SPARKS_PER_STEP {
            let i = self.rng.index(COLS * ROWS);
            self.grid[i] = FIRING;
        }
    }

    fn paint(&mut self, cx: i32, cy: i32) {
        if cx >= 0 && cy >= 0 && (cx as usize) < COLS && (cy as usize) < ROWS {
            self.grid[cy as usize * COLS + cx as usize] = FIRING;
        }
    }
}

fn main() {
    let mut brain = Brain {
        grid: vec![OFF; COLS * ROWS],
        rng: gui::Rng::from_entropy(),
        acc: 0.0,
    };
    brain.reseed();
    let app = gui::App::new("062 - Brian's Brain", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.07, 0.07, 0.10));

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let bx = ((f.width - bw) * 0.5).max(16.0);

        // Input: paint firing cells, reseed.
        let (mx, my) = f.mouse();
        if mx >= bx && mx < bx + bw && my >= BOARD_Y && my < BOARD_Y + bh
            && (f.input.mouse_pressed() || f.input.mouse_down())
        {
            brain.paint(((mx - bx) / CELL) as i32, ((my - BOARD_Y) / CELL) as i32);
        }
        if f.input.key_pressed(gui::key::R) {
            brain.reseed();
        }
        brain.acc += f.dt;
        if brain.acc >= STEP_EVERY {
            brain.acc = 0.0;
            brain.step();
        }

        // Header.
        f.text(bx, 18.0, 4.0, "BRIAN'S BRAIN", gui::Color::WHITE);
        f.text(
            bx + bw - f.text_width(2.5, "OFF - FIRING - REFRACTORY"),
            26.0,
            2.5,
            "OFF - FIRING - REFRACTORY",
            gui::Color::GRAY,
        );

        // Board: white = firing, dim blue = refractory, dark = off.
        f.rect(bx, BOARD_Y, bw, bh, gui::Color::rgb(0.10, 0.10, 0.13));
        for y in 0..ROWS {
            for x in 0..COLS {
                let ink = match brain.grid[y * COLS + x] {
                    FIRING => gui::Color::WHITE,
                    REFRACT => gui::Color::rgb(0.18, 0.28, 0.55),
                    _ => continue,
                };
                f.rect(
                    bx + x as f32 * CELL,
                    BOARD_Y + y as f32 * CELL,
                    CELL - 1.0,
                    CELL - 1.0,
                    ink,
                );
            }
        }
        f.frame_rect(bx, BOARD_Y, bw, bh, 2.0, gui::Color::rgb(0.3, 0.3, 0.4));

        f.text(
            bx,
            H as f32 - 40.0,
            2.0,
            "CLICK/DRAG PAINTS FIRING CELLS - R RESEED - EXACTLY 2 FIRING NEIGHBOURS IGNITE - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
