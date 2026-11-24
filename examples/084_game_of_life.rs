//! 084 — Game of Life: Conway's cellular automaton. Click/drag to draw or
//! erase cells, Space pauses/resumes, N steps one generation, R randomizes,
//! C clears, G seeds a Gosper glider gun, P an R-pentomino. Cell color
//! encodes age: newborns are yellow, elders fade to deep blue.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 830.0;
const H: f64 = 640.0;
const COLS: usize = 112;
const ROWS: usize = 68;
const CELL: f32 = 7.0;
const BOARD_X: f32 = 23.0;
const BOARD_Y: f32 = 104.0;

struct Life {
    grid: Vec<u8>,
    age: Vec<u32>,
    generation: u64,
    pop: usize,
    running: bool,
    acc: f32,
    paint: u8,
}

impl Life {
    fn new() -> Life {
        let mut life = Life {
            grid: vec![0; COLS * ROWS],
            age: vec![0; COLS * ROWS],
            generation: 0,
            pop: 0,
            running: true,
            acc: 0.0,
            paint: 1,
        };
        life.randomize();
        life
    }

    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= COLS as i32 || y >= ROWS as i32 {
            None
        } else {
            Some((y as usize) * COLS + x as usize)
        }
    }

    fn neighbors(&self, x: i32, y: i32) -> u8 {
        let mut n = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(i) = self.idx(x + dx, y + dy) {
                    n += self.grid[i];
                }
            }
        }
        n
    }

    fn step(&mut self) {
        let mut next = self.grid.clone();
        let mut next_age = self.age.clone();
        self.pop = 0;
        for y in 0..ROWS as i32 {
            for x in 0..COLS as i32 {
                let i = self.idx(x, y).unwrap();
                let n = self.neighbors(x, y);
                let alive = match (self.grid[i], n) {
                    (1, 2) | (1, 3) => 1,
                    (0, 3) => 1,
                    _ => 0,
                };
                next[i] = alive;
                next_age[i] = if alive == 1 { self.age[i].saturating_add(1) } else { 0 };
                self.pop += alive as usize;
            }
        }
        self.grid = next;
        self.age = next_age;
        self.generation += 1;
    }

    fn randomize(&mut self) {
        let mut rng = gui::Rng::from_entropy();
        self.grid
            .iter_mut()
            .for_each(|c| *c = (rng.f32() < 0.18) as u8);
        self.age = vec![0; COLS * ROWS];
        self.age
            .iter_mut()
            .zip(self.grid.iter())
            .for_each(|(a, &g)| *a = g as u32);
        self.generation = 0;
        self.pop = self.population();
    }

    /// Number of live cells. The sum is taken in `usize`: a board of a few
    /// thousand cells overflows a `u8` accumulator as soon as the colony
    /// passes 255 members.
    fn population(&self) -> usize {
        self.grid.iter().map(|&cell| cell as usize).sum()
    }

    fn clear(&mut self) {
        self.grid.fill(0);
        self.age.fill(0);
        self.generation = 0;
        self.pop = 0;
    }

    fn seed(&mut self, cells: &[(i32, i32)], ox: i32, oy: i32) {
        self.clear();
        for (x, y) in cells {
            if let Some(i) = self.idx(x + ox, y + oy) {
                self.grid[i] = 1;
            }
        }
        self.pop = self.population();
    }

    fn age_color(age: u32) -> gui::Color {
        match age {
            0 => gui::Color::rgb(0.11, 0.11, 0.15),
            1..=2 => gui::Color::YELLOW,
            3..=5 => gui::Color::ORANGE,
            6..=12 => gui::Color::RED,
            13..=40 => gui::Color::rgb(0.85, 0.3, 0.7),
            _ => gui::Color::rgb(0.25, 0.45, 0.95),
        }
    }
}

/// Gosper glider gun (standard 36-cell pattern).
const GOSPER_GUN: [(i32, i32); 36] = [
    (24, 0),
    (22, 1),
    (24, 1),
    (12, 2),
    (13, 2),
    (20, 2),
    (21, 2),
    (34, 2),
    (35, 2),
    (11, 3),
    (15, 3),
    (20, 3),
    (21, 3),
    (34, 3),
    (35, 3),
    (0, 4),
    (1, 4),
    (10, 4),
    (16, 4),
    (20, 4),
    (21, 4),
    (0, 5),
    (1, 5),
    (10, 5),
    (14, 5),
    (16, 5),
    (17, 5),
    (22, 5),
    (24, 5),
    (10, 6),
    (16, 6),
    (24, 6),
    (11, 7),
    (15, 7),
    (12, 8),
    (13, 8),
];

/// R-pentomino (the famous long-lived mini-pattern).
const R_PENTOMINO: [(i32, i32); 5] = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];

fn main() {
    let mut life = Life::new();
    let app = gui::App::new_points("084 - Game of Life", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.08, 0.08, 0.10));

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let board_rect = (BOARD_X, BOARD_Y, bw, bh);

        // --- Input: paint on the board ---
        let (mx, my) = f.mouse();
        let in_board = mx >= board_rect.0 && mx <= board_rect.0 + board_rect.2
            && my >= board_rect.1 && my <= board_rect.1 + board_rect.3;
        if f.input.mouse_pressed() && in_board {
            let cx = ((mx - board_rect.0) / CELL) as i32;
            let cy = ((my - board_rect.1) / CELL) as i32;
            if let Some(i) = life.idx(cx, cy) {
                life.paint = 1 - life.grid[i];
                life.grid[i] = life.paint;
                life.age[i] = life.paint as u32;
            }
        } else if f.input.mouse_down() && in_board {
            let cx = ((mx - board_rect.0) / CELL) as i32;
            let cy = ((my - board_rect.1) / CELL) as i32;
            if let Some(i) = life.idx(cx, cy) {
                life.grid[i] = life.paint;
                life.age[i] = life.paint as u32;
            }
        }

        // --- Keyboard + toolbar ---
        if f.input.key_pressed(gui::key::SPACE) {
            life.running = !life.running;
        }
        if f.input.key_pressed(gui::key::N) {
            life.step();
        }
        if f.input.key_pressed(gui::key::R) {
            life.randomize();
        }
        if f.input.key_pressed(gui::key::C) {
            life.clear();
        }
        if f.input.key_pressed(gui::key::G) {
            life.seed(&GOSPER_GUN, 30, 20);
        }
        if f.input.key_pressed(gui::key::P) {
            life.seed(&R_PENTOMINO, (COLS / 2) as i32, (ROWS / 2) as i32);
        }

        // --- Simulation ---
        if life.running {
            life.acc += f.dt;
            if life.acc >= 0.1 {
                life.acc = 0.0;
                life.step();
            }
        }

        // --- Header ---
        f.text(BOARD_X, 18.0, 4.0, "GAME OF LIFE", gui::Color::WHITE);
        let status = format!(
            "GEN {}  POP {}  {}",
            life.generation,
            life.pop,
            if life.running { "RUNNING" } else { "PAUSED" }
        );
        f.text(
            BOARD_X + bw - f.text_width(2.5, &status),
            26.0,
            2.5,
            &status,
            if life.running { gui::Color::GREEN } else { gui::Color::YELLOW },
        );

        // --- Board ---
        f.rect(board_rect.0, board_rect.1, board_rect.2, board_rect.3, gui::Color::rgb(0.11, 0.11, 0.15));
        for y in 0..ROWS {
            for x in 0..COLS {
                let i = y * COLS + x;
                if life.grid[i] == 1 {
                    f.rect(
                        board_rect.0 + x as f32 * CELL,
                        board_rect.1 + y as f32 * CELL,
                        CELL - 1.0,
                        CELL - 1.0,
                        Life::age_color(life.age[i]),
                    );
                }
            }
        }
        f.frame_rect(board_rect.0, board_rect.1, board_rect.2, board_rect.3, 2.0, gui::Color::rgb(0.3, 0.3, 0.4));

        // --- Footer hints ---
        f.text(
            BOARD_X,
            H as f32 - 40.0,
            2.0,
            "CLICK/DRAG DRAW - SPACE PAUSE - N STEP - R RANDOM - C CLEAR - G GOSPER GUN - P R-PENTOMINO",
            gui::Color::GRAY,
        );
    });
}
