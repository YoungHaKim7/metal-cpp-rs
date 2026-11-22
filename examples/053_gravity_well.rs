//! 053 — Gravity Well: 600 particles swing around attractor wells with an
//! inverse-square pull (clamped close in). Click to drop a well — five max,
//! the oldest pops off — and W removes the newest. Wells pulse; escapees
//! that fly off-screen respawn on the window edges with an inward push.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const PARTICLES: usize = 600;
const MAX_WELLS: usize = 5;
const PULL: f32 = 4.5e6;
const PULL_CLAMP: f32 = 2100.0;
const MIN_R2: f32 = 2400.0;
const SPEED_MAX: f32 = 540.0;

struct Well {
    x: f32,
    y: f32,
    birth: f32,
}

struct Mote {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

struct Field {
    wells: Vec<Well>,
    motes: Vec<Mote>,
    rng: gui::Rng,
}

impl Field {
    fn new() -> Field {
        Field {
            wells: Vec::new(),
            motes: Vec::new(),
            rng: gui::Rng::from_entropy(),
        }
    }

    /// One well at the center; particles start on rough circular orbits.
    fn seed(&mut self, w: f32, h: f32) {
        if self.wells.is_empty() {
            self.wells.push(Well {
                x: w * 0.5,
                y: h * 0.5,
                birth: 0.0,
            });
        }
        self.motes.clear();
        for _ in 0..PARTICLES {
            let ang = self.rng.f32() * std::f32::consts::TAU;
            let r = 60.0 + self.rng.f32() * (w.min(h) * 0.42);
            let x = (w * 0.5 + ang.cos() * r).clamp(20.0, w - 20.0);
            let y = (h * 0.5 + ang.sin() * r * 0.8).clamp(20.0, h - 20.0);
            let dx = w * 0.5 - x;
            let dy = h * 0.5 - y;
            let d = (dx * dx + dy * dy).sqrt().max(1.0);
            let v = (PULL / d).sqrt();
            self.motes.push(Mote {
                x,
                y,
                vx: -dy / d * v,
                vy: dx / d * v,
            });
        }
    }
}

fn main() {
    let mut field = Field::new();
    let app = gui::App::new("053 - Gravity Well", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.05));

        if field.motes.is_empty() {
            field.seed(f.width, f.height);
        }

        if f.input.mouse_pressed() {
            let (mx, my) = f.mouse();
            if field.wells.len() >= MAX_WELLS {
                field.wells.remove(0);
            }
            field.wells.push(Well {
                x: mx,
                y: my,
                birth: f.time,
            });
        }
        if f.input.key_pressed(gui::key::W) {
            field.wells.pop();
        }

        for m in &mut field.motes {
            let mut ax = 0.0f32;
            let mut ay = 0.0f32;
            for wl in &field.wells {
                let dx = wl.x - m.x;
                let dy = wl.y - m.y;
                let d2 = (dx * dx + dy * dy).max(MIN_R2);
                let d = d2.sqrt();
                let a = (PULL / d2).min(PULL_CLAMP);
                ax += dx / d * a;
                ay += dy / d * a;
            }
            let damp = (-0.02 * f.dt).exp();
            m.vx = (m.vx + ax * f.dt) * damp;
            m.vy = (m.vy + ay * f.dt) * damp;
            let sp = (m.vx * m.vx + m.vy * m.vy).sqrt();
            if sp > SPEED_MAX {
                let k = SPEED_MAX / sp;
                m.vx *= k;
                m.vy *= k;
            }
            m.x += m.vx * f.dt;
            m.y += m.vy * f.dt;
            if m.x < -30.0 || m.x > f.width + 30.0 || m.y < -30.0 || m.y > f.height + 30.0 {
                // Respawn on a random edge, pushed back toward the middle.
                let edge = field.rng.index(4);
                match edge {
                    0 => {
                        m.x = field.rng.f32() * f.width;
                        m.y = -10.0;
                    }
                    1 => {
                        m.x = f.width + 10.0;
                        m.y = field.rng.f32() * f.height;
                    }
                    2 => {
                        m.x = field.rng.f32() * f.width;
                        m.y = f.height + 10.0;
                    }
                    _ => {
                        m.x = -10.0;
                        m.y = field.rng.f32() * f.height;
                    }
                }
                let tx = f.width * 0.5 - m.x;
                let ty = f.height * 0.5 - m.y;
                let td = (tx * tx + ty * ty).sqrt().max(1.0);
                m.vx = tx / td * 130.0 + (field.rng.f32() - 0.5) * 60.0;
                m.vy = ty / td * 130.0 + (field.rng.f32() - 0.5) * 60.0;
            }
        }

        // Motes tinted by speed.
        for m in &field.motes {
            let sp = (m.vx * m.vx + m.vy * m.vy).sqrt();
            let t = (sp / SPEED_MAX).clamp(0.0, 1.0);
            let c = gui::Color::hsv(0.55 + 0.40 * t, 0.75, 1.0);
            f.rect(m.x - 1.1, m.y - 1.1, 2.2, 2.2, c);
        }

        // Pulsing wells.
        for (i, wl) in field.wells.iter().enumerate() {
            let c = gui::Color::hsv(0.5 + i as f32 * 0.155, 0.6, 1.0);
            let pulse = 1.0 + 0.18 * (f.time * 2.8 + wl.birth * 5.0).sin();
            let ro = 13.0 * pulse;
            f.ring(wl.x, wl.y, ro, ro - 3.5, c);
            f.circle(wl.x, wl.y, 3.6, c);
        }

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "053 - GRAVITY WELL",
            gui::Color::WHITE,
        );
        f.text(
            f.width - 130.0,
            26.0,
            2.5,
            &format!("WELLS: {}", field.wells.len()),
            gui::Color::GRAY,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "CLICK DROPS A WELL (MAX 5, OLDEST POPS) - W REMOVES THE LAST - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
