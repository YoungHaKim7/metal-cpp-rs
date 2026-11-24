//! 064 — Wa-Tor: predator and prey on a torus. Fish (blue) drift and breed
//! every six ticks; sharks (red) hunt them, breed on their own clock and
//! starve after twelve hungry ticks. R reseeds the ecosystem with 500 fish
//! and 40 sharks; the live census sits in the header.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1040.0;
const H: f64 = 780.0;
const COLS: usize = 120;
const ROWS: usize = 80;
const CELL: f32 = 8.0;
const BOARD_Y: f32 = 84.0;
const STEP_EVERY: f32 = 1.0 / 12.0;
const FISH: u8 = 1;
const SHARK: u8 = 2;
const FISH_BREED: u8 = 6;
const SHARK_BREED: u8 = 8;
const SHARK_STARVE: u8 = 12;
const START_FISH: usize = 500;
const START_SHARKS: usize = 40;

struct Wator {
    grid: Vec<u8>,
    breed: Vec<u8>,
    hunger: Vec<u8>,
    moved: Vec<bool>,
    fish: usize,
    sharks: usize,
    acc: f32,
    rng: gui::Rng,
}

impl Wator {
    fn reseed(&mut self) {
        self.grid.fill(0);
        self.breed.fill(0);
        self.hunger.fill(0);
        let n = COLS * ROWS;
        for _ in 0..START_FISH {
            let i = self.rng.index(n);
            if self.grid[i] == 0 {
                self.grid[i] = FISH;
            }
        }
        for _ in 0..START_SHARKS {
            let i = self.rng.index(n);
            if self.grid[i] == 0 {
                self.grid[i] = SHARK;
            }
        }
        self.fish = self.grid.iter().filter(|&&c| c == FISH).count();
        self.sharks = self.grid.iter().filter(|&&c| c == SHARK).count();
    }

    /// The four wrapped neighbour indices of cell (x, y).
    fn around(&self, x: usize, y: usize) -> [usize; 4] {
        let xm = (x + COLS - 1) % COLS;
        let xp = (x + 1) % COLS;
        let ym = (y + ROWS - 1) % ROWS;
        let yp = (y + 1) % ROWS;
        [y * COLS + xm, y * COLS + xp, ym * COLS + x, yp * COLS + x]
    }

    fn move_fish(&mut self, x: usize, y: usize, i: usize) {
        self.breed[i] += 1;
        let free: Vec<usize> = self
            .around(x, y)
            .iter()
            .copied()
            .filter(|&n| self.grid[n] == 0)
            .collect();
        if free.is_empty() {
            return;
        }
        let t = free[self.rng.index(free.len())];
        self.moved[t] = true;
        if self.breed[i] >= FISH_BREED {
            self.breed[i] = 0;
            self.breed[t] = 0;
            self.moved[i] = true;
            self.grid[t] = FISH;
        } else {
            self.breed[t] = self.breed[i];
            self.breed[i] = 0;
            self.grid[t] = FISH;
            self.grid[i] = 0;
        }
    }

    fn move_shark(&mut self, x: usize, y: usize, i: usize) {
        self.hunger[i] += 1;
        let around = self.around(x, y);
        let prey: Vec<usize> = around
            .iter()
            .copied()
            .filter(|&n| self.grid[n] == FISH)
            .collect();
        if !prey.is_empty() {
            let t = prey[self.rng.index(prey.len())];
            self.breed[i] += 1;
            let newborn = self.breed[i] >= SHARK_BREED;
            self.grid[t] = SHARK;
            self.hunger[t] = 0;
            self.moved[t] = true;
            if newborn {
                self.breed[i] = 0;
                self.breed[t] = 0;
                self.hunger[i] = 0;
                self.moved[i] = true;
            } else {
                self.breed[t] = self.breed[i];
                self.breed[i] = 0;
                self.grid[i] = 0;
            }
            return;
        }
        if self.hunger[i] > SHARK_STARVE {
            self.grid[i] = 0;
            self.breed[i] = 0;
            self.hunger[i] = 0;
            return;
        }
        let free: Vec<usize> = around
            .iter()
            .copied()
            .filter(|&n| self.grid[n] == 0)
            .collect();
        if free.is_empty() {
            return;
        }
        let t = free[self.rng.index(free.len())];
        self.breed[i] += 1;
        let newborn = self.breed[i] >= SHARK_BREED;
        self.grid[t] = SHARK;
        self.hunger[t] = self.hunger[i];
        self.moved[t] = true;
        if newborn {
            self.breed[i] = 0;
            self.breed[t] = 0;
            self.hunger[i] = 0;
            self.moved[i] = true;
        } else {
            self.breed[t] = self.breed[i];
            self.breed[i] = 0;
            self.grid[i] = 0;
        }
    }

    fn step(&mut self) {
        self.moved.fill(false);
        for y in 0..ROWS {
            for x in 0..COLS {
                let i = y * COLS + x;
                if self.moved[i] {
                    continue;
                }
                match self.grid[i] {
                    FISH => self.move_fish(x, y, i),
                    SHARK => self.move_shark(x, y, i),
                    _ => {}
                }
            }
        }
        self.fish = self.grid.iter().filter(|&&c| c == FISH).count();
        self.sharks = self.grid.iter().filter(|&&c| c == SHARK).count();
    }
}

fn main() {
    let mut sea = Wator {
        grid: vec![0u8; COLS * ROWS],
        breed: vec![0u8; COLS * ROWS],
        hunger: vec![0u8; COLS * ROWS],
        moved: vec![false; COLS * ROWS],
        fish: 0,
        sharks: 0,
        acc: 0.0,
        rng: gui::Rng::from_entropy(),
    };
    sea.reseed();
    let app = gui::App::new_points("064 - Wa-Tor", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.06, 0.07, 0.10));

        // Input + simulation.
        if f.input.key_pressed(gui::key::R) {
            sea.reseed();
        }
        sea.acc += f.dt;
        if sea.acc >= STEP_EVERY {
            sea.acc = 0.0;
            sea.step();
        }

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let bx = ((f.width - bw) * 0.5).max(16.0);

        // Header with the live census.
        f.text(bx, 18.0, 4.0, "WA-TOR", gui::Color::WHITE);
        let fish_label = format!("FISH {}", sea.fish);
        let shark_label = format!("SHARKS {}", sea.sharks);
        let shark_x = bx + bw - f.text_width(2.5, &shark_label);
        f.text(shark_x, 26.0, 2.5, &shark_label, gui::Color::RED);
        f.text(
            shark_x - f.text_width(2.5, &fish_label) - 28.0,
            26.0,
            2.5,
            &fish_label,
            gui::Color::BLUE,
        );

        // Board: blue fish, red sharks, open ocean in between.
        f.rect(bx, BOARD_Y, bw, bh, gui::Color::rgb(0.04, 0.09, 0.12));
        for y in 0..ROWS {
            for x in 0..COLS {
                let ink = match sea.grid[y * COLS + x] {
                    FISH => gui::Color::BLUE,
                    SHARK => gui::Color::RED,
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
            "R RESEED - FISH BREED EVERY 6 - SHARKS STARVE AFTER 12 HUNGRY TICKS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
