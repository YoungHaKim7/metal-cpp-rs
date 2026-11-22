//! 056 — Pong: you take the left paddle (W/S or the arrow keys); the CPU
//! tracks on the right with a capped top speed. The ball gains 5% per
//! paddle hit and rebounds steeper the farther from paddle center it
//! strikes. First to 7 wins — WIN!/LOSE! banner and R restarts. The middle
//! line is dashed and the score lives at the top.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 900.0;
const H: f64 = 560.0;
const PAD_W: f32 = 13.0;
const PAD_H: f32 = 96.0;
const PAD_X: f32 = 30.0;
const BALL_R: f32 = 8.0;
const PLAYER_SPD: f32 = 460.0;
const AI_SPD: f32 = 265.0;
const SERVE_SPD: f32 = 350.0;
const SPEEDUP: f32 = 1.05;
const SPEED_CAP: f32 = 920.0;
const TARGET: u32 = 7;

#[derive(Clone, Copy, PartialEq)]
enum End {
    Playing,
    Won,
    Lost,
}

struct Pong {
    py: f32,
    ay: f32,
    bx: f32,
    by: f32,
    bvx: f32,
    bvy: f32,
    wait: f32,
    ps: u32,
    cs: u32,
    end: End,
    rng: gui::Rng,
}

impl Pong {
    fn new() -> Pong {
        Pong {
            py: 232.0,
            ay: 232.0,
            bx: W as f32 * 0.5,
            by: H as f32 * 0.5,
            bvx: 0.0,
            bvy: 0.0,
            wait: 1.0,
            ps: 0,
            cs: 0,
            end: End::Playing,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn after_score(&mut self, w: f32, h: f32) {
        self.bx = w * 0.5;
        self.by = h * 0.5;
        self.bvx = 0.0;
        self.bvy = 0.0;
        if self.ps >= TARGET {
            self.end = End::Won;
        } else if self.cs >= TARGET {
            self.end = End::Lost;
        } else {
            self.wait = 0.9;
        }
    }

    fn update(&mut self, f: &mut gui::Frame) {
        if !matches!(self.end, End::Playing) {
            if f.input.key_pressed(gui::key::R) {
                *self = Pong::new();
            }
            return;
        }
        let w = f.width;
        let h = f.height;

        // Player paddle.
        let mut pv = 0.0f32;
        if f.input.key(gui::key::W) || f.input.key(gui::key::UP) {
            pv -= PLAYER_SPD;
        }
        if f.input.key(gui::key::S) || f.input.key(gui::key::DOWN) {
            pv += PLAYER_SPD;
        }
        self.py = (self.py + pv * f.dt).clamp(10.0, h - 10.0 - PAD_H);

        // CPU paddle: capped tracking speed, rests toward center otherwise.
        let target = if self.bvx > 0.0 { self.by } else { h * 0.5 };
        let dy = target - (self.ay + PAD_H * 0.5);
        if dy.abs() > 8.0 {
            let step = dy.clamp(-AI_SPD * f.dt, AI_SPD * f.dt);
            self.ay = (self.ay + step).clamp(10.0, h - 10.0 - PAD_H);
        }

        // Serve countdown; the ball waits parked at center.
        if self.wait > 0.0 {
            self.bx = w * 0.5;
            self.by = h * 0.5;
            self.wait -= f.dt;
            if self.wait <= 0.0 {
                let ang = (self.rng.f32() - 0.5) * 0.9;
                let dir = if self.rng.f32() < 0.5 { -1.0 } else { 1.0 };
                self.bvx = dir * SERVE_SPD * ang.cos();
                self.bvy = SERVE_SPD * ang.sin();
            }
            return;
        }

        // Ball flight.
        self.bx += self.bvx * f.dt;
        self.by += self.bvy * f.dt;
        if self.by < BALL_R {
            self.by = BALL_R;
            self.bvy = self.bvy.abs();
        }
        if self.by > h - BALL_R {
            self.by = h - BALL_R;
            self.bvy = -self.bvy.abs();
        }

        // Player paddle collision (left).
        if self.bvx < 0.0
            && self.bx - BALL_R < PAD_X + PAD_W
            && self.bx > PAD_X - 10.0
            && self.by > self.py - BALL_R
            && self.by < self.py + PAD_H + BALL_R
        {
            self.bx = PAD_X + PAD_W + BALL_R;
            let off = ((self.by - (self.py + PAD_H * 0.5)) / (PAD_H * 0.5)).clamp(-1.0, 1.0);
            let spd = ((self.bvx * self.bvx + self.bvy * self.bvy).sqrt() * SPEEDUP).min(SPEED_CAP);
            let ang = off * 0.85;
            self.bvx = spd * ang.cos();
            self.bvy = spd * ang.sin();
        }
        // CPU paddle collision (right).
        let ax = w - PAD_X - PAD_W;
        if self.bvx > 0.0
            && self.bx + BALL_R > ax
            && self.bx < w - PAD_X + 10.0
            && self.by > self.ay - BALL_R
            && self.by < self.ay + PAD_H + BALL_R
        {
            self.bx = ax - BALL_R;
            let off = ((self.by - (self.ay + PAD_H * 0.5)) / (PAD_H * 0.5)).clamp(-1.0, 1.0);
            let spd = ((self.bvx * self.bvx + self.bvy * self.bvy).sqrt() * SPEEDUP).min(SPEED_CAP);
            let ang = off * 0.85;
            self.bvx = -spd * ang.cos();
            self.bvy = spd * ang.sin();
        }

        // Scoring.
        if self.bx < -24.0 {
            self.cs += 1;
            self.after_score(w, h);
        } else if self.bx > w + 24.0 {
            self.ps += 1;
            self.after_score(w, h);
        }
    }
}

fn main() {
    let mut game = Pong::new();
    let app = gui::App::new("056 - Pong", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.04, 0.05, 0.07));
        game.update(f);
        let w = f.width;
        let h = f.height;

        // Dashed center line.
        let mut y = 10.0f32;
        while y < h - 10.0 {
            f.rect(
                w * 0.5 - 2.0,
                y,
                4.0,
                14.0,
                gui::Color::rgb(0.18, 0.20, 0.28),
            );
            y += 30.0;
        }

        // Paddles + ball.
        f.rect(PAD_X, game.py, PAD_W, PAD_H, gui::Color::WHITE);
        f.rect(
            w - PAD_X - PAD_W,
            game.ay,
            PAD_W,
            PAD_H,
            gui::Color::rgb(1.0, 0.45, 0.45),
        );
        f.circle_segments(game.bx, game.by, BALL_R, 16, gui::Color::WHITE);

        // Score.
        f.text_center(
            w * 0.5 - 70.0,
            30.0,
            8.0,
            &game.ps.to_string(),
            gui::Color::WHITE,
        );
        f.text_center(
            w * 0.5 + 70.0,
            30.0,
            8.0,
            &game.cs.to_string(),
            gui::Color::rgb(1.0, 0.45, 0.45),
        );
        f.text_center(w * 0.5 - 70.0, 92.0, 2.5, "YOU", gui::Color::GRAY);
        f.text_center(w * 0.5 + 70.0, 92.0, 2.5, "CPU", gui::Color::GRAY);

        // End banner.
        match game.end {
            End::Won | End::Lost => {
                f.rect(
                    w * 0.5 - 210.0,
                    h * 0.34,
                    420.0,
                    120.0,
                    gui::Color::rgba(0.0, 0.0, 0.0, 0.75),
                );
                let (msg, col) = if game.end == End::Won {
                    ("YOU WIN!", gui::Color::GREEN)
                } else {
                    ("YOU LOSE!", gui::Color::RED)
                };
                f.text_center(w * 0.5, h * 0.34 + 26.0, 8.0, msg, col);
                f.text_center(
                    w * 0.5,
                    h * 0.34 + 82.0,
                    2.5,
                    "PRESS R TO RESTART",
                    gui::Color::WHITE,
                );
            }
            _ => {}
        }

        f.text_center(f.width * 0.5, 20.0, 4.0, "056 - PONG", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "W/S OR UP/DOWN MOVE - BALL SPEEDS UP 5% PER HIT - FIRST TO 7 - R RESTARTS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
