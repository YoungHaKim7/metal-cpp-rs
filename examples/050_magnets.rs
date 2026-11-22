//! 050 — Magnets: eight fixed poles alternate attract (+) and repel (-).
//! 400 charged particles feel an inverse-square pull from every pole
//! (clamped close in), wrap around the window edges, and SPACE flips every
//! polarity at once so the whole field turns inside out.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const POLES: usize = 8;
const CHARGES: usize = 400;
const FIELD_STRENGTH: f32 = 4.2e6;
const FORCE_CLAMP: f32 = 2600.0;
const MIN_R2: f32 = 900.0;
const SPEED_MAX: f32 = 400.0;
const POLE_R: f32 = 17.0;

struct Pole {
    nx: f32,
    ny: f32,
    positive: bool,
}

struct Charge {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

struct Lab {
    poles: Vec<Pole>,
    charges: Vec<Charge>,
    rng: gui::Rng,
}

impl Lab {
    fn new() -> Lab {
        let mut poles = Vec::with_capacity(POLES);
        for i in 0..POLES {
            poles.push(Pole {
                nx: 0.14 + 0.24 * (i % 4) as f32,
                ny: if i < 4 { 0.30 } else { 0.72 },
                positive: i % 2 == 0,
            });
        }
        Lab {
            poles,
            charges: Vec::new(),
            rng: gui::Rng::from_entropy(),
        }
    }

    fn seed(&mut self, w: f32, h: f32) {
        self.charges.clear();
        for _ in 0..CHARGES {
            let a = self.rng.f32() * std::f32::consts::TAU;
            self.charges.push(Charge {
                x: self.rng.f32() * w,
                y: self.rng.f32() * h,
                vx: a.cos() * 40.0,
                vy: a.sin() * 40.0,
            });
        }
    }
}

fn main() {
    let mut lab = Lab::new();
    let app = gui::App::new("050 - Magnets", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.05));

        if lab.charges.is_empty() {
            lab.seed(f.width, f.height);
        }

        if f.input.key_pressed(gui::key::SPACE) {
            for p in &mut lab.poles {
                p.positive = !p.positive;
            }
        }

        for c in &mut lab.charges {
            let mut fx = 0.0f32;
            let mut fy = 0.0f32;
            for p in &lab.poles {
                let px = p.nx * f.width;
                let py = p.ny * f.height;
                let dx = px - c.x;
                let dy = py - c.y;
                let d2 = (dx * dx + dy * dy).max(MIN_R2);
                let d = d2.sqrt();
                let a = (FIELD_STRENGTH / d2).min(FORCE_CLAMP);
                let s = if p.positive { 1.0 } else { -1.0 };
                fx += dx / d * a * s;
                fy += dy / d * a * s;
            }
            let damp = (-0.10 * f.dt).exp();
            c.vx = (c.vx + fx * f.dt) * damp;
            c.vy = (c.vy + fy * f.dt) * damp;
            let sp = (c.vx * c.vx + c.vy * c.vy).sqrt();
            if sp > SPEED_MAX {
                let k = SPEED_MAX / sp;
                c.vx *= k;
                c.vy *= k;
            }
            c.x += c.vx * f.dt;
            c.y += c.vy * f.dt;
            if c.x < -6.0 {
                c.x += f.width + 12.0;
            }
            if c.x > f.width + 6.0 {
                c.x -= f.width + 12.0;
            }
            if c.y < -6.0 {
                c.y += f.height + 12.0;
            }
            if c.y > f.height + 6.0 {
                c.y -= f.height + 12.0;
            }
        }

        // Charged particles, tinted by speed.
        for c in &lab.charges {
            let sp = (c.vx * c.vx + c.vy * c.vy).sqrt();
            let t = (sp / SPEED_MAX).clamp(0.0, 1.0);
            let col = gui::Color::lerp(gui::Color::CYAN, gui::Color::YELLOW, t);
            f.rect(c.x - 1.1, c.y - 1.1, 2.2, 2.2, col);
        }

        // Poles: dark disc, colored rim, +/- label.
        for p in &lab.poles {
            let px = p.nx * f.width;
            let py = p.ny * f.height;
            let pulse = 1.0 + 0.08 * (f.time * 3.0 + p.nx * 10.0).sin();
            let (rim, label) = if p.positive {
                (gui::Color::RED, "+")
            } else {
                (gui::Color::BLUE, "-")
            };
            f.circle(px, py, POLE_R * pulse, gui::Color::rgb(0.09, 0.09, 0.13));
            f.ring(px, py, POLE_R * pulse, POLE_R * pulse - 3.0, rim);
            f.text_center(px, py - 10.5, 3.0, label, gui::Color::WHITE);
        }

        f.text_center(f.width * 0.5, 20.0, 4.0, "050 - MAGNETS", gui::Color::WHITE);
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "8 POLES AT 1/R^2 (CLAMPED) - 400 CHARGES WRAP AT EDGES - SPACE FLIPS POLARITY - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
