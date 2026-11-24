//! 083 — Snake: the grid classic. Arrows or WASD steer (turns are queued so
//! fast double-taps work), speed rises with every food eaten, R restarts
//! after a crash. Hitting a wall or your own tail ends the run.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 720.0;
const H: f64 = 560.0;
const COLS: i32 = 25;
const ROWS: i32 = 16;
const CELL: f32 = 26.0;
const BOARD_X: f32 = 35.0;
const BOARD_Y: f32 = 64.0;

struct Snake {
    body: Vec<(i32, i32)>,
    dir: (i32, i32),
    queue: Vec<(i32, i32)>,
    food: (i32, i32),
    rng: gui::Rng,
    acc: f32,
    score: u32,
    eaten: u32,
    over: bool,
}

impl Snake {
    fn new() -> Snake {
        let mut s = Snake {
            body: vec![(5, 8), (4, 8), (3, 8)],
            dir: (1, 0),
            queue: Vec::new(),
            food: (0, 0),
            rng: gui::Rng::from_entropy(),
            acc: 0.0,
            score: 0,
            eaten: 0,
            over: false,
        };
        s.place_food();
        s
    }

    fn interval(&self) -> f32 {
        (0.13 - self.eaten as f32 * 0.004).max(0.055)
    }

    fn place_food(&mut self) {
        loop {
            let x = self.rng.range(0, COLS as i64 - 1) as i32;
            let y = self.rng.range(0, ROWS as i64 - 1) as i32;
            if !self.body.contains(&(x, y)) {
                self.food = (x, y);
                return;
            }
        }
    }

    fn update(&mut self, f: &mut gui::Frame) {
        if self.over {
            if f.input.key_pressed(gui::key::R) {
                *self = Snake::new();
            }
            return;
        }

        // Queue direction changes (max 2) — never reverse into yourself.
        const UP: (i32, i32) = (0, -1);
        const DOWN: (i32, i32) = (0, 1);
        const LEFT: (i32, i32) = (-1, 0);
        const RIGHT: (i32, i32) = (1, 0);
        for (code, dir) in [
            (gui::key::UP, UP),
            (gui::key::W, UP),
            (gui::key::DOWN, DOWN),
            (gui::key::S, DOWN),
            (gui::key::LEFT, LEFT),
            (gui::key::A, LEFT),
            (gui::key::RIGHT, RIGHT),
            (gui::key::D, RIGHT),
        ] {
            if f.input.key_pressed(code) && self.queue.len() < 2 {
                let last = *self.queue.last().unwrap_or(&self.dir);
                if dir != last && (dir.0 + last.0 != 0 || dir.1 + last.1 != 0) {
                    self.queue.push(dir);
                }
            }
        }

        self.acc += f.dt;
        if self.acc < self.interval() {
            return;
        }
        self.acc = 0.0;

        if let Some(d) = self.queue.pop_front_marker() {
            self.dir = d;
        }
        let head = self.body[0];
        let next = (head.0 + self.dir.0, head.1 + self.dir.1);

        let hit_wall = next.0 < 0 || next.0 >= COLS || next.1 < 0 || next.1 >= ROWS;
        let hit_self = self.body[..self.body.len() - 1].contains(&next);
        if hit_wall || hit_self {
            self.over = true;
            return;
        }

        self.body.insert(0, next);
        if next == self.food {
            self.score += 10;
            self.eaten += 1;
            self.place_food();
        } else {
            self.body.pop();
        }
    }
}

/// Small local helper standing in for `VecDeque::pop_front`.
trait PopFront {
    fn pop_front_marker(&mut self) -> Option<(i32, i32)>;
}

impl PopFront for Vec<(i32, i32)> {
    fn pop_front_marker(&mut self) -> Option<(i32, i32)> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

fn main() {
    let mut game = Snake::new();
    let app = gui::App::new_points("083 - Snake", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.07, 0.09, 0.08));
        game.update(f);

        let bw = COLS as f32 * CELL;
        let bh = ROWS as f32 * CELL;

        // Header.
        f.text(BOARD_X, 20.0, 3.0, "SNAKE", gui::Color::GREEN);
        let score = format!("SCORE {}", game.score);
        f.text(
            BOARD_X + bw - f.text_width(3.0, &score),
            20.0,
            3.0,
            &score,
            gui::Color::WHITE,
        );

        // Board.
        f.rect(BOARD_X, BOARD_Y, bw, bh, gui::Color::rgb(0.10, 0.14, 0.11));
        f.frame_rect(
            BOARD_X,
            BOARD_Y,
            bw,
            bh,
            2.0,
            gui::Color::rgb(0.25, 0.45, 0.3),
        );

        // Food.
        let fx = BOARD_X + (game.food.0 as f32 + 0.5) * CELL;
        let fy = BOARD_Y + (game.food.1 as f32 + 0.5) * CELL;
        f.circle(fx, fy, CELL * 0.38, gui::Color::RED);

        // Snake body: bright head, tail fades out.
        let n = game.body.len();
        for (i, (x, y)) in game.body.iter().enumerate() {
            let t = if n > 1 {
                i as f32 / (n - 1) as f32
            } else {
                0.0
            };
            let color = if i == 0 {
                gui::Color::rgb(0.5, 1.0, 0.5)
            } else {
                gui::Color::lerp(
                    gui::Color::rgb(0.25, 0.85, 0.4),
                    gui::Color::rgb(0.1, 0.3, 0.18),
                    t,
                )
            };
            f.rect(
                BOARD_X + *x as f32 * CELL + 2.0,
                BOARD_Y + *y as f32 * CELL + 2.0,
                CELL - 4.0,
                CELL - 4.0,
                color,
            );
        }

        // Game over overlay.
        if game.over {
            f.rect(
                BOARD_X,
                BOARD_Y + bh * 0.3,
                bw,
                110.0,
                gui::Color::rgba(0.0, 0.0, 0.0, 0.75),
            );
            f.text_center(
                BOARD_X + bw * 0.5,
                BOARD_Y + bh * 0.3 + 24.0,
                5.0,
                "GAME OVER",
                gui::Color::RED,
            );
            f.text_center(
                BOARD_X + bw * 0.5,
                BOARD_Y + bh * 0.3 + 70.0,
                2.5,
                "PRESS R TO RESTART",
                gui::Color::WHITE,
            );
        }

        f.text(
            BOARD_X,
            H as f32 - 26.0,
            2.0,
            "ARROWS / WASD STEER - EAT THE RED DOT - R RESTART - ESC QUIT",
            gui::Color::GRAY,
        );
    });
}
