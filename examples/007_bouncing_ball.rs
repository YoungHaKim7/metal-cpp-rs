//! 007 — Bouncing Ball: a ball crossing the window at constant speed,
//! bouncing off all four walls without energy loss. It leaves a fading trail
//! of its last 40 positions; R respawns it at the center with a random
//! velocity direction. (The stock pipeline has no alpha blending, so the
//! trail fades by lerping colors toward the background.)

use std::f32::consts::TAU;

#[path = "gui/mod.rs"]
mod gui;

const TRAIL_LEN: usize = 40;
const BALL_R: f32 = 14.0;
const BG: gui::Color = gui::Color::rgb(0.05, 0.05, 0.09);

struct Ball {
    pos: (f32, f32),
    vel: (f32, f32),
    trail: Vec<(f32, f32)>,
    rng: gui::Rng,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            pos: (0.0, 0.0),
            vel: (1.0, 0.0),
            trail: Vec::new(),
            rng: gui::Rng::from_entropy(),
        }
    }

    fn respawn(&mut self, w: f32, h: f32) {
        self.pos = (w * 0.5, h * 0.5);
        let a = self.rng.f32() * TAU;
        self.vel = (a.cos(), a.sin());
        self.trail.clear();
    }

    fn update(&mut self, f: &mut gui::Frame) {
        // Keep the speed constant (and window-relative) at all times.
        let speed = f.width.min(f.height) * 0.55;
        let vlen = (self.vel.0 * self.vel.0 + self.vel.1 * self.vel.1)
            .sqrt()
            .max(1e-5);
        self.vel = (self.vel.0 / vlen * speed, self.vel.1 / vlen * speed);

        self.pos.0 += self.vel.0 * f.dt;
        self.pos.1 += self.vel.1 * f.dt;
        if self.pos.0 < BALL_R {
            self.pos.0 = BALL_R;
            self.vel.0 = self.vel.0.abs();
        } else if self.pos.0 > f.width - BALL_R {
            self.pos.0 = f.width - BALL_R;
            self.vel.0 = -self.vel.0.abs();
        }
        if self.pos.1 < BALL_R {
            self.pos.1 = BALL_R;
            self.vel.1 = self.vel.1.abs();
        } else if self.pos.1 > f.height - BALL_R {
            self.pos.1 = f.height - BALL_R;
            self.vel.1 = -self.vel.1.abs();
        }

        self.trail.push(self.pos);
        if self.trail.len() > TRAIL_LEN {
            self.trail.remove(0);
        }
    }
}

fn main() {
    let mut ball = Ball::new();
    let mut started = false;
    let app = gui::App::new("007 - Bouncing Ball", 640.0, 400.0);
    app.run(move |f| {
        f.set_clear(BG);

        if !started {
            ball.respawn(f.width, f.height);
            started = true;
        }
        if f.input.key_pressed(gui::key::R) {
            ball.respawn(f.width, f.height);
        }
        ball.update(f);

        // Fading trail: older samples melt into the background color.
        let n = ball.trail.len();
        for (i, &(x, y)) in ball.trail.iter().enumerate() {
            let t = (i + 1) as f32 / n.max(1) as f32;
            let col = gui::Color::lerp(BG, gui::Color::rgb(0.30, 0.85, 1.0), t * 0.85);
            f.circle_segments(x, y, 2.0 + 8.0 * t, 10, col);
        }

        // The ball itself, with a thin bright rim.
        f.circle(
            ball.pos.0,
            ball.pos.1,
            BALL_R,
            gui::Color::rgb(0.35, 0.90, 1.0),
        );
        f.ring(
            ball.pos.0,
            ball.pos.1,
            BALL_R + 3.0,
            BALL_R,
            gui::Color::WHITE,
        );

        f.frame_rect(
            8.0,
            8.0,
            f.width - 16.0,
            f.height - 16.0,
            2.0,
            gui::Color::rgb(0.16, 0.18, 0.28),
        );

        f.text_center(f.width * 0.5, 14.0, 4.0, "BOUNCING BALL", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 38.0,
            2.0,
            "CONSTANT SPEED, NO ENERGY LOSS - R RESPAWNS FROM CENTER - ESC TO QUIT",
            gui::Color::GRAY,
        );
    });
}
