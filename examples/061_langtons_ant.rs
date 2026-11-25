//! 061 — Langton's Ant: the two-rule turmite. On white the ant turns right,
//! on black it turns left, flipping each square it leaves — after a few
//! thousand chaotic steps it starts building the famous highway. The 100x70
//! grid wraps at the edges; C resets grid and ant, SPACE pauses, and the
//! step counter lives in the header.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 1080.0;
const H: f64 = 830.0;
const COLS: usize = 100;
const ROWS: usize = 70;
const CELL: f32 = 10.0;
const STEP_EVERY: f32 = 1.0 / 30.0;
/// Ant steps per simulation tick (360 a second). The highway only emerges
/// after some ten thousand steps; at one step per tick the grid stayed
/// near-empty for minutes first.
const STEPS_PER_TICK: usize = 12;

/// Ant headings — up, right, down, left (screen y grows downward).
const DIRS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct Ant {
    grid: Vec<u8>,
    x: usize,
    y: usize,
    dir: usize,
    steps: u64,
    acc: f32,
    running: bool,
}

impl Ant {
    fn new() -> Ant {
        let mut ant = Ant {
            grid: vec![0u8; COLS * ROWS],
            x: COLS / 2,
            y: ROWS / 2,
            dir: 0,
            steps: 0,
            acc: 0.0,
            running: true,
        };
        ant.grid[ant.y * COLS + ant.x] = 1;
        ant
    }

    fn reset(&mut self) {
        self.grid.fill(0);
        self.x = COLS / 2;
        self.y = ROWS / 2;
        self.dir = 0;
        self.steps = 0;
        self.acc = 0.0;
        self.running = true;
        self.grid[self.y * COLS + self.x] = 1;
    }

    fn step(&mut self) {
        let i = self.y * COLS + self.x;
        self.dir = if self.grid[i] == 0 {
            (self.dir + 1) % 4
        } else {
            (self.dir + 3) % 4
        };
        self.grid[i] ^= 1;
        let (dx, dy) = DIRS[self.dir];
        self.x = (self.x as i32 + dx).rem_euclid(COLS as i32) as usize;
        self.y = (self.y as i32 + dy).rem_euclid(ROWS as i32) as usize;
        self.steps += 1;
    }
}

fn main() {
    let mut ant = Ant::new();
    let app = gui::App::new_points("061 - Langton's Ant", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.06, 0.06, 0.09));

        // Input + simulation.
        if f.input.key_pressed(gui::key::C) {
            ant.reset();
        }
        if f.input.key_pressed(gui::key::SPACE) {
            ant.running = !ant.running;
        }
        if ant.running {
            ant.acc += f.dt;
            while ant.acc >= STEP_EVERY {
                ant.acc -= STEP_EVERY;
                for _ in 0..STEPS_PER_TICK {
                    ant.step();
                }
            }
        }

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        let bx = ((f.width - bw) * 0.5).max(16.0);
        let by = 84.0;

        // Header.
        f.text(bx, 18.0, 4.0, "LANGTON'S ANT", gui::Color::WHITE);
        let status = format!(
            "STEP {}  {}",
            ant.steps,
            if ant.running { "RUNNING" } else { "PAUSED" }
        );
        f.text(
            bx + bw - f.text_width(2.5, &status),
            26.0,
            2.5,
            &status,
            if ant.running {
                gui::Color::GREEN
            } else {
                gui::Color::YELLOW
            },
        );

        // Board: flipped cells pale, the ant itself bright red.
        f.rect(bx, by, bw, bh, gui::Color::rgb(0.10, 0.10, 0.13));
        let pale = gui::Color::rgb(0.82, 0.80, 0.70);
        for y in 0..ROWS {
            for x in 0..COLS {
                if ant.grid[y * COLS + x] == 1 {
                    f.rect(
                        bx + x as f32 * CELL,
                        by + y as f32 * CELL,
                        CELL - 1.0,
                        CELL - 1.0,
                        pale,
                    );
                }
            }
        }
        f.rect(
            bx + ant.x as f32 * CELL,
            by + ant.y as f32 * CELL,
            CELL - 1.0,
            CELL - 1.0,
            gui::Color::RED,
        );
        f.frame_rect(bx, by, bw, bh, 2.0, gui::Color::rgb(0.3, 0.3, 0.4));

        f.text(
            bx,
            H as f32 - 40.0,
            2.0,
            "C RESET GRID AND ANT - SPACE PAUSE - RIGHT ON WHITE, LEFT ON BLACK - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
