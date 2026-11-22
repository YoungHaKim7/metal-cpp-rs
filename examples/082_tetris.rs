//! 082 — Tetris: the falling-tetromino classic. Arrows steer (with key
//! auto-repeat), Up rotates with wall kicks, Space hard-drops, R restarts.
//! Score/lines/level live in the sidebar, next piece preview included.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 660.0;
const H: f64 = 600.0;
const COLS: usize = 10;
const ROWS: usize = 20;
const CELL: f32 = 26.0;
const BOARD_X: f32 = 40.0;
const BOARD_Y: f32 = 40.0;

/// Base cell layout and bounding-box size per tetromino
/// (I:0 O:1 T:2 S:3 Z:4 J:5 L:6).
const BASE: [([(i8, i8); 4], i8); 7] = [
    ([(0, 1), (1, 1), (2, 1), (3, 1)], 4), // I
    ([(0, 0), (1, 0), (0, 1), (1, 1)], 2), // O
    ([(1, 0), (0, 1), (1, 1), (2, 1)], 3), // T
    ([(1, 0), (2, 0), (0, 1), (1, 1)], 3), // S
    ([(0, 0), (1, 0), (1, 1), (2, 1)], 3), // Z
    ([(0, 0), (0, 1), (1, 1), (2, 1)], 3), // J
    ([(2, 0), (0, 1), (1, 1), (2, 1)], 3), // L
];

const COLORS: [gui::Color; 7] = [
    gui::Color::rgb(0.2, 0.9, 1.0),  // I cyan
    gui::Color::rgb(1.0, 0.85, 0.2), // O yellow
    gui::Color::rgb(0.9, 0.3, 0.95), // T magenta
    gui::Color::rgb(0.3, 0.9, 0.4),  // S green
    gui::Color::rgb(1.0, 0.3, 0.3),  // Z red
    gui::Color::rgb(0.3, 0.5, 1.0),  // J blue
    gui::Color::rgb(1.0, 0.6, 0.2),  // L orange
];

/// Rotate a piece `rot` quarter-turns clockwise inside its bounding box.
fn cells(kind: usize, rot: usize) -> [(i8, i8); 4] {
    let (base, n) = BASE[kind];
    let mut c = base;
    for _ in 0..(rot % 4) {
        c = c.map(|(x, y)| (n - 1 - y, x));
    }
    c
}

struct Tetris {
    board: [[u8; COLS]; ROWS],
    kind: usize,
    rot: usize,
    px: i8,
    py: i8,
    next: usize,
    rng: gui::Rng,
    score: u32,
    lines: u32,
    acc: f32,
    das: f32,
    soft: f32,
    over: bool,
}

impl Tetris {
    fn new() -> Tetris {
        let mut t = Tetris {
            board: [[0; COLS]; ROWS],
            kind: 0,
            rot: 0,
            px: 3,
            py: -2,
            next: 0,
            rng: gui::Rng::from_entropy(),
            score: 0,
            lines: 0,
            acc: 0.0,
            das: 0.0,
            soft: 0.0,
            over: false,
        };
        t.kind = t.rng.index(7);
        t.next = t.rng.index(7);
        t
    }

    fn level(&self) -> u32 {
        self.lines / 10 + 1
    }

    fn gravity_interval(&self) -> f32 {
        let level = self.level().min(10) as usize;
        [0.80, 0.65, 0.50, 0.40, 0.30, 0.22, 0.15, 0.11, 0.08, 0.06][level - 1]
    }

    fn collides(&self, cells: [(i8, i8); 4], px: i8, py: i8) -> bool {
        cells.iter().any(|(dx, dy)| {
            let x = px + dx;
            let y = py + dy;
            x < 0
                || x >= COLS as i8
                || y >= ROWS as i8
                || (y >= 0 && self.board[y as usize][x as usize] != 0)
        })
    }

    fn spawn(&mut self) {
        self.kind = self.next;
        self.next = self.rng.index(7);
        self.rot = 0;
        self.px = 3;
        self.py = -2;
        let c = cells(self.kind, self.rot);
        if self.collides(c, self.px, self.py) {
            self.over = true;
        }
    }

    fn lock(&mut self) {
        for (dx, dy) in cells(self.kind, self.rot) {
            let x = self.px + dx;
            let y = self.py + dy;
            if y < 0 {
                self.over = true;
                return;
            }
            self.board[y as usize][x as usize] = self.kind as u8 + 1;
        }
        let mut cleared = 0;
        let mut y = ROWS;
        while y > 0 {
            y -= 1;
            if self.board[y].iter().all(|&c| c != 0) {
                self.board.copy_within(0..y, 1);
                self.board[0] = [0; COLS];
                cleared += 1;
                y += 1;
            }
        }
        if cleared > 0 {
            self.lines += cleared;
            self.score += [0, 100, 300, 500, 800][cleared.min(4) as usize] * self.level();
        }
        self.spawn();
    }

    fn try_move(&mut self, dx: i8, dy: i8) -> bool {
        let c = cells(self.kind, self.rot);
        if !self.collides(c, self.px + dx, self.py + dy) {
            self.px += dx;
            self.py += dy;
            return true;
        }
        false
    }

    fn rotate(&mut self) {
        let new_rot = (self.rot + 1) % 4;
        for kick in [0i8, -1, 1, -2, 2] {
            let c = cells(self.kind, new_rot);
            if !self.collides(c, self.px + kick, self.py) {
                self.rot = new_rot;
                self.px += kick;
                return;
            }
        }
    }

    fn hard_drop(&mut self) {
        let mut dist = 0u32;
        while self.try_move(0, 1) {
            dist += 1;
        }
        self.score += dist * 2;
        self.lock();
    }

    fn update(&mut self, f: &mut gui::Frame) {
        if self.over {
            if f.input.key_pressed(gui::key::R) {
                *self = Tetris::new();
            }
            return;
        }

        // Horizontal movement with auto-repeat (initial delay 0.22 s, then 0.05 s).
        let left_held = f.input.key(gui::key::LEFT);
        let right_held = f.input.key(gui::key::RIGHT);
        if f.input.key_pressed(gui::key::LEFT) {
            self.try_move(-1, 0);
            self.das = 0.22;
        } else if left_held {
            self.das -= f.dt;
            if self.das <= 0.0 {
                self.try_move(-1, 0);
                self.das = 0.05;
            }
        } else if f.input.key_pressed(gui::key::RIGHT) {
            self.try_move(1, 0);
            self.das = 0.22;
        } else if right_held {
            self.das -= f.dt;
            if self.das <= 0.0 {
                self.try_move(1, 0);
                self.das = 0.05;
            }
        }

        if f.input.key_pressed(gui::key::UP) {
            self.rotate();
        }
        if f.input.key_pressed(gui::key::SPACE) {
            self.hard_drop();
            return;
        }

        // Soft drop.
        if f.input.key(gui::key::DOWN) {
            self.soft -= f.dt;
            if self.soft <= 0.0 {
                if self.try_move(0, 1) {
                    self.score += 1;
                }
                self.soft = 0.045;
            }
        }

        // Gravity.
        self.acc += f.dt;
        let interval = self.gravity_interval();
        while self.acc >= interval {
            self.acc -= interval;
            if !self.try_move(0, 1) {
                self.lock();
                break;
            }
        }
    }
}

fn draw_cell(f: &mut gui::Frame, x: f32, y: f32, color: gui::Color) {
    f.rect(x, y, CELL - 2.0, CELL - 2.0, color);
    f.rect(
        x + CELL - 2.0 - 5.0,
        y + 2.0,
        3.0,
        CELL - 2.0 - 4.0,
        gui::Color::lerp(color, gui::Color::BLACK, 0.35),
    );
}

fn draw_piece(f: &mut gui::Frame, kind: usize, px: i8, py: i8, ox: f32, oy: f32) {
    for (dx, dy) in cells(kind, 0) {
        let x = ox + (px + dx) as f32 * CELL;
        let y = oy + (py + dy) as f32 * CELL;
        if y >= oy {
            draw_cell(f, x, y, COLORS[kind]);
        }
    }
}

fn main() {
    let mut game = Tetris::new();
    let app = gui::App::new("082 - Tetris", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.07, 0.08, 0.11));
        game.update(f);

        // Board background + grid.
        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;
        f.rect(BOARD_X, BOARD_Y, bw, bh, gui::Color::rgb(0.10, 0.11, 0.15));
        for gx in 1..COLS {
            f.line(
                BOARD_X + gx as f32 * CELL,
                BOARD_Y,
                BOARD_X + gx as f32 * CELL,
                BOARD_Y + bh,
                1.0,
                gui::Color::rgb(0.15, 0.16, 0.21),
            );
        }
        for gy in 1..ROWS {
            f.line(
                BOARD_X,
                BOARD_Y + gy as f32 * CELL,
                BOARD_X + bw,
                BOARD_Y + gy as f32 * CELL,
                1.0,
                gui::Color::rgb(0.15, 0.16, 0.21),
            );
        }
        f.frame_rect(
            BOARD_X,
            BOARD_Y,
            bw,
            bh,
            2.0,
            gui::Color::rgb(0.3, 0.32, 0.45),
        );

        // Locked cells.
        for y in 0..ROWS {
            for x in 0..COLS {
                let c = game.board[y][x];
                if c != 0 {
                    draw_cell(
                        f,
                        BOARD_X + x as f32 * CELL,
                        BOARD_Y + y as f32 * CELL,
                        COLORS[(c - 1) as usize],
                    );
                }
            }
        }

        // Current piece.
        if !game.over {
            draw_piece(f, game.kind, game.px, game.py, BOARD_X, BOARD_Y);
        }

        // Sidebar.
        let sx = BOARD_X + bw + 28.0;
        f.text(sx, BOARD_Y, 4.0, "TETRIS", gui::Color::WHITE);
        f.text(sx, BOARD_Y + 56.0, 2.5, "SCORE", gui::Color::GRAY);
        f.text(
            sx,
            BOARD_Y + 78.0,
            4.0,
            &game.score.to_string(),
            gui::Color::YELLOW,
        );
        f.text(sx, BOARD_Y + 124.0, 2.5, "LINES", gui::Color::GRAY);
        f.text(
            sx,
            BOARD_Y + 146.0,
            4.0,
            &game.lines.to_string(),
            gui::Color::CYAN,
        );
        f.text(sx, BOARD_Y + 192.0, 2.5, "LEVEL", gui::Color::GRAY);
        f.text(
            sx,
            BOARD_Y + 214.0,
            4.0,
            &game.level().to_string(),
            gui::Color::GREEN,
        );
        f.text(sx, BOARD_Y + 260.0, 2.5, "NEXT", gui::Color::GRAY);
        f.rect(
            sx,
            BOARD_Y + 284.0,
            4.0 * CELL,
            2.0 * CELL,
            gui::Color::rgb(0.10, 0.11, 0.15),
        );
        {
            let (base, _) = BASE[game.next];
            let min_x = base.iter().map(|c| c.0).min().unwrap();
            let max_x = base.iter().map(|c| c.0).max().unwrap();
            let span = (max_x - min_x + 1) as f32;
            let bx = sx + (4.0 * CELL - span * CELL) * 0.5;
            for (dx, dy) in base {
                draw_cell(
                    f,
                    bx + dx as f32 * CELL,
                    BOARD_Y + 284.0 + dy as f32 * CELL,
                    COLORS[game.next],
                );
            }
        }

        f.text(sx, BOARD_Y + 380.0, 2.0, "ARROWS MOVE", gui::Color::GRAY);
        f.text(sx, BOARD_Y + 400.0, 2.0, "UP ROTATE", gui::Color::GRAY);
        f.text(sx, BOARD_Y + 420.0, 2.0, "SPACE DROP", gui::Color::GRAY);
        f.text(sx, BOARD_Y + 440.0, 2.0, "R RESTART", gui::Color::GRAY);

        if game.over {
            f.rect(
                BOARD_X,
                BOARD_Y + bh * 0.35,
                bw,
                90.0,
                gui::Color::rgba(0.0, 0.0, 0.0, 0.75),
            );
            f.text_center(
                BOARD_X + bw * 0.5,
                BOARD_Y + bh * 0.35 + 18.0,
                5.0,
                "GAME OVER",
                gui::Color::RED,
            );
            f.text_center(
                BOARD_X + bw * 0.5,
                BOARD_Y + bh * 0.35 + 60.0,
                2.5,
                "PRESS R TO RESTART",
                gui::Color::WHITE,
            );
        }
    });
}
