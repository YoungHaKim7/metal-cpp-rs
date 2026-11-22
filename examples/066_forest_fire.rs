//! 066 — Forest Fire: trees, fire and ash on a grid. Fire spreads to the
//! four neighbours for one tick, then burns out leaving ash, and the forest
//! slowly regrows into empty cells. Lightning ignites about two trees per
//! second; R replants a fresh 60% forest.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1040.0;
const H: f64 = 780.0;
const COLS: usize = 120;
const ROWS: usize = 80;
const CELL: f32 = 8.0;
const BOARD_Y: f32 = 84.0;
const STEP_EVERY: f32 = 0.1;
const STRIKES_PER_SECOND: f32 = 2.0;
const REGROW_P: f32 = 0.01;
const FOREST_DENSITY: f32 = 0.60;
const EMPTY: u8 = 0;
const TREE: u8 = 1;
const FIRE: u8 = 2;

struct Forest {
    grid: Vec<u8>,
    trees: usize,
    fires: usize,
    acc: f32,
    strike_acc: f32,
    rng: gui::Rng,
}

impl Forest {
    fn reseed(&mut self) {
        let mut rng = gui::Rng::from_entropy();
        for c in self.grid.iter_mut() {
            *c = if rng.f32() < FOREST_DENSITY {
                TREE
            } else {
                EMPTY
            };
        }
        self.count();
    }

    fn count(&mut self) {
        self.trees = self.grid.iter().filter(|&&c| c == TREE).count();
        self.fires = self.grid.iter().filter(|&&c| c == FIRE).count();
    }

    fn step(&mut self) {
        let old = self.grid.clone();
        let rng = &mut self.rng;
        for y in 0..ROWS {
            for x in 0..COLS {
                let i = y * COLS + x;
                self.grid[i] = match old[i] {
                    FIRE => EMPTY,
                    TREE => {
                        let burning = (x > 0 && old[i - 1] == FIRE)
                            || (x + 1 < COLS && old[i + 1] == FIRE)
                            || (y > 0 && old[i - COLS] == FIRE)
                            || (y + 1 < ROWS && old[i + COLS] == FIRE);
                        if burning { FIRE } else { TREE }
                    }
                    _ => {
                        if rng.f32() < REGROW_P {
                            TREE
                        } else {
                            EMPTY
                        }
                    }
                };
            }
        }
        self.count();
    }

    fn strike(&mut self) {
        let i = self.rng.index(COLS * ROWS);
        if self.grid[i] == TREE {
            self.grid[i] = FIRE;
        }
    }
}

fn main() {
    let mut forest = Forest {
        grid: vec![EMPTY; COLS * ROWS],
        trees: 0,
        fires: 0,
        acc: 0.0,
        strike_acc: 0.0,
        rng: gui::Rng::from_entropy(),
    };
    forest.reseed();
    let app = gui::App::new("066 - Forest Fire", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.08, 0.07, 0.06));

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let bx = ((f.width - bw) * 0.5).max(16.0);

        // Input + simulation: CA steps at 10/s, lightning at ~2 strikes/s.
        if f.input.key_pressed(gui::key::R) {
            forest.reseed();
        }
        forest.strike_acc += STRIKES_PER_SECOND * f.dt;
        while forest.strike_acc >= 1.0 {
            forest.strike_acc -= 1.0;
            forest.strike();
        }
        forest.acc += f.dt;
        if forest.acc >= STEP_EVERY {
            forest.acc = 0.0;
            forest.step();
        }

        // Header with the census.
        f.text(bx, 18.0, 4.0, "FOREST FIRE", gui::Color::WHITE);
        let census = format!("TREES {}  BURNING {}", forest.trees, forest.fires);
        f.text(
            bx + bw - f.text_width(2.5, &census),
            26.0,
            2.5,
            &census,
            if forest.fires > 0 {
                gui::Color::ORANGE
            } else {
                gui::Color::GREEN
            },
        );

        // Board: green canopy, orange fire, charcoal ash.
        f.rect(bx, BOARD_Y, bw, bh, gui::Color::rgb(0.13, 0.11, 0.10));
        for y in 0..ROWS {
            for x in 0..COLS {
                let ink = match forest.grid[y * COLS + x] {
                    TREE => gui::Color::GREEN,
                    FIRE => gui::Color::rgb(1.0, 0.55, 0.10),
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
            "R REFORESTS 60% - FIRE SPREADS 4-NEIGHBOURS FOR 1 TICK - LIGHTNING ~2/S - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
