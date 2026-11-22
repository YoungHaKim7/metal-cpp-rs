//! 057 — Breakout: an 8x5 brick wall in five row colors, a paddle that
//! mirrors the mouse, and a ball whose rebound angle depends on where it
//! strikes the paddle. Each brick scores by row; three lives shown as dots
//! at the top-right; clearing the wall wins. R restarts.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 760.0;
const H: f64 = 600.0;
const COLS: usize = 8;
const ROWS: usize = 5;
const SIDE: f32 = 24.0;
const TOP: f32 = 78.0;
const BRICK_H: f32 = 24.0;
const GAP: f32 = 5.0;
const PAD_W: f32 = 96.0;
const PAD_H: f32 = 13.0;
const PAD_LIFT: f32 = 40.0;
const BALL_R: f32 = 7.5;
const SPEED: f32 = 350.0;
const LIVES: u32 = 3;

const ROW_COLORS: [gui::Color; ROWS] = [
    gui::Color::rgb(0.95, 0.30, 0.30),
    gui::Color::rgb(0.98, 0.60, 0.22),
    gui::Color::rgb(0.98, 0.86, 0.25),
    gui::Color::rgb(0.35, 0.85, 0.35),
    gui::Color::rgb(0.30, 0.75, 0.95),
];

#[derive(Clone, Copy, PartialEq)]
enum End {
    Playing,
    Won,
    Lost,
}

fn pad_y(h: f32) -> f32 {
    h - PAD_LIFT
}

fn brick_w(w: f32) -> f32 {
    (w - 2.0 * SIDE - (COLS - 1) as f32 * GAP) / COLS as f32
}

struct Breakout {
    alive: [bool; COLS * ROWS],
    px: f32,
    bx: f32,
    by: f32,
    bvx: f32,
    bvy: f32,
    stuck: bool,
    lives: u32,
    score: u32,
    end: End,
    rng: gui::Rng,
}

impl Breakout {
    fn new() -> Breakout {
        Breakout {
            alive: [true; COLS * ROWS],
            px: 0.0,
            bx: 0.0,
            by: 0.0,
            bvx: 0.0,
            bvy: 0.0,
            stuck: true,
            lives: LIVES,
            score: 0,
            end: End::Playing,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn update(&mut self, f: &mut gui::Frame) {
        if !matches!(self.end, End::Playing) {
            if f.input.key_pressed(gui::key::R) {
                *self = Breakout::new();
            }
            return;
        }

        // Paddle follows the mouse x.
        let (mx, _) = f.mouse();
        self.px = (mx - PAD_W * 0.5).clamp(SIDE, f.width - SIDE - PAD_W);
        let py = pad_y(f.height);

        // Ball parked on the paddle until launched.
        if self.stuck {
            self.bx = self.px + PAD_W * 0.5;
            self.by = py - BALL_R - 2.0;
            if f.input.mouse_pressed() || f.input.key_pressed(gui::key::SPACE) {
                self.stuck = false;
                let ang = (self.rng.f32() - 0.5) * 0.6;
                self.bvx = SPEED * ang.sin();
                self.bvy = -SPEED * ang.cos();
            }
            return;
        }

        self.bx += self.bvx * f.dt;
        self.by += self.bvy * f.dt;

        // Walls.
        if self.bx < SIDE + BALL_R {
            self.bx = SIDE + BALL_R;
            self.bvx = self.bvx.abs();
        }
        if self.bx > f.width - SIDE - BALL_R {
            self.bx = f.width - SIDE - BALL_R;
            self.bvx = -self.bvx.abs();
        }
        if self.by < 10.0 + BALL_R {
            self.by = 10.0 + BALL_R;
            self.bvy = self.bvy.abs();
        }

        // Paddle: rebound angle depends on the hit offset.
        if self.bvy > 0.0
            && self.by + BALL_R >= py
            && self.by - BALL_R < py + PAD_H
            && self.bx > self.px - BALL_R
            && self.bx < self.px + PAD_W + BALL_R
        {
            self.by = py - BALL_R;
            let off = ((self.bx - (self.px + PAD_W * 0.5)) / (PAD_W * 0.5)).clamp(-1.0, 1.0);
            let ang = off * 1.05;
            self.bvx = SPEED * ang.sin();
            self.bvy = -SPEED * ang.cos();
        }

        // Bricks: reflect along the axis of least penetration.
        let bw = brick_w(f.width);
        let mut hit = false;
        for idx in 0..COLS * ROWS {
            if hit || !self.alive[idx] {
                continue;
            }
            let col = idx % COLS;
            let row = idx / COLS;
            let bx0 = SIDE + col as f32 * (bw + GAP);
            let by0 = TOP + row as f32 * (BRICK_H + GAP);
            let nx = self.bx.clamp(bx0, bx0 + bw);
            let ny = self.by.clamp(by0, by0 + BRICK_H);
            let dx = self.bx - nx;
            let dy = self.by - ny;
            if dx * dx + dy * dy <= BALL_R * BALL_R {
                self.alive[idx] = false;
                self.score += (ROWS - row) as u32 * 10;
                let left = self.bx + BALL_R - bx0;
                let right = bx0 + bw - (self.bx - BALL_R);
                let top = self.by + BALL_R - by0;
                let bottom = by0 + BRICK_H - (self.by - BALL_R);
                let m = left.min(right).min(top).min(bottom);
                if m == top {
                    self.bvy = -self.bvy.abs();
                    self.by = by0 - BALL_R;
                } else if m == bottom {
                    self.bvy = self.bvy.abs();
                    self.by = by0 + BRICK_H + BALL_R;
                } else if m == left {
                    self.bvx = -self.bvx.abs();
                    self.bx = bx0 - BALL_R;
                } else {
                    self.bvx = self.bvx.abs();
                    self.bx = bx0 + bw + BALL_R;
                }
                hit = true;
            }
        }

        // Floor takes a life; no lives or no bricks ends the game.
        if self.by - BALL_R > f.height {
            self.lives -= 1;
            if self.lives == 0 {
                self.end = End::Lost;
            } else {
                self.stuck = true;
            }
        }
        if self.alive.iter().all(|&a| !a) {
            self.end = End::Won;
        }
    }
}

fn main() {
    let mut game = Breakout::new();
    let app = gui::App::new("057 - Breakout", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.04, 0.04, 0.07));
        game.update(f);
        let w = f.width;
        let h = f.height;

        // Court frame.
        f.frame_rect(
            SIDE - 6.0,
            6.0,
            w - 2.0 * (SIDE - 6.0),
            h - 12.0,
            3.0,
            gui::Color::rgb(0.20, 0.22, 0.30),
        );

        // Bricks.
        let bw = brick_w(w);
        for idx in 0..COLS * ROWS {
            if !game.alive[idx] {
                continue;
            }
            let col = idx % COLS;
            let row = idx / COLS;
            let x = SIDE + col as f32 * (bw + GAP);
            let y = TOP + row as f32 * (BRICK_H + GAP);
            f.rect(x, y, bw, BRICK_H, ROW_COLORS[row]);
            f.rect(
                x,
                y + BRICK_H - 4.0,
                bw,
                4.0,
                gui::Color::lerp(ROW_COLORS[row], gui::Color::BLACK, 0.35),
            );
        }

        // Paddle + ball.
        let py = pad_y(h);
        f.rect(game.px, py, PAD_W, PAD_H, gui::Color::rgb(0.85, 0.87, 0.92));
        f.rect(
            game.px + PAD_W * 0.5 - 8.0,
            py + 3.0,
            16.0,
            PAD_H - 6.0,
            gui::Color::rgb(0.30, 0.60, 0.95),
        );
        f.circle_segments(game.bx, game.by, BALL_R, 14, gui::Color::WHITE);

        // HUD: score left, life dots right.
        f.text(
            20.0,
            26.0,
            3.0,
            &format!("SCORE: {}", game.score),
            gui::Color::YELLOW,
        );
        for i in 0..LIVES {
            let cx = w - 26.0 - i as f32 * 22.0;
            if i < game.lives {
                f.circle(cx, 34.0, 6.0, gui::Color::rgb(0.95, 0.30, 0.30));
            } else {
                f.ring(cx, 34.0, 6.0, 4.0, gui::Color::rgb(0.35, 0.20, 0.20));
            }
        }
        f.text_center(w * 0.5, 30.0, 3.0, "BREAKOUT", gui::Color::WHITE);

        if game.stuck && game.end == End::Playing {
            let a = 0.5 + 0.4 * (f.time * 4.0).sin();
            f.text_center(
                w * 0.5,
                h * 0.62,
                2.5,
                "CLICK OR SPACE TO LAUNCH",
                gui::Color::rgba(1.0, 1.0, 1.0, a),
            );
        }

        match game.end {
            End::Won | End::Lost => {
                f.rect(
                    w * 0.5 - 220.0,
                    h * 0.36,
                    440.0,
                    120.0,
                    gui::Color::rgba(0.0, 0.0, 0.0, 0.78),
                );
                let (msg, col) = if game.end == End::Won {
                    ("YOU CLEARED THE WALL!", gui::Color::GREEN)
                } else {
                    ("GAME OVER", gui::Color::RED)
                };
                f.text_center(w * 0.5, h * 0.36 + 26.0, 6.0, msg, col);
                f.text_center(
                    w * 0.5,
                    h * 0.36 + 78.0,
                    2.5,
                    &format!("SCORE: {} - PRESS R TO RESTART", game.score),
                    gui::Color::WHITE,
                );
            }
            _ => {}
        }

        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "MOUSE MOVES THE PADDLE - REBOUND ANGLE DEPENDS ON HIT POINT - 3 LIVES - R RESTARTS",
            gui::Color::GRAY,
        );
    });
}
