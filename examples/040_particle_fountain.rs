//! 040 — Particle Fountain: a rainbow jet erupts from the bottom-center of
//! the window. Every frame six particles spawn with an upward velocity plus
//! horizontal spread, fall under gravity and fade out across a ~2.5 s
//! lifetime. Hue follows altitude (hot red at the nozzle, violet near the
//! top), the pool is capped at 1200 particles and anything that falls back
//! below the floor line dies.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 800.0;
const H: f64 = 600.0;
const CAP: usize = 1200;
const PER_FRAME: usize = 6;
const GRAVITY: f32 = 520.0;
const FLOOR_H: f32 = 26.0;

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    age: f32,
    life: f32,
}

struct Fountain {
    particles: Vec<Particle>,
    rng: gui::Rng,
}

impl Fountain {
    fn new() -> Fountain {
        Fountain {
            particles: Vec::with_capacity(CAP),
            rng: gui::Rng::from_entropy(),
        }
    }

    fn spawn(&mut self, w: f32, floor_y: f32) {
        if self.particles.len() >= CAP {
            return;
        }
        // The jet reaches most of the window height: v^2 / 2g tops out at
        // about 1120 units against GRAVITY.
        self.particles.push(Particle {
            x: w * 0.5 + (self.rng.f32() - 0.5) * 16.0,
            y: floor_y - 2.0,
            vx: (self.rng.f32() - 0.5) * 440.0,
            vy: -(520.0 + self.rng.f32() * 560.0),
            age: 0.0,
            life: 2.2 + self.rng.f32() * 0.6,
        });
    }
}

fn main() {
    let mut fountain = Fountain::new();
    let app = gui::App::new("040 - Particle Fountain", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.02, 0.05));

        let floor_y = f.height - FLOOR_H;
        for _ in 0..PER_FRAME {
            fountain.spawn(f.width, floor_y);
        }

        fountain.particles.retain_mut(|p| {
            p.age += f.dt;
            p.vy += GRAVITY * f.dt;
            p.vx *= (-0.25 * f.dt).exp();
            p.x += p.vx * f.dt;
            p.y += p.vy * f.dt;
            p.age < p.life && p.y < floor_y
        });

        // Floor line + nozzle glow.
        f.rect(
            0.0,
            floor_y,
            f.width,
            3.0,
            gui::Color::rgb(0.14, 0.15, 0.22),
        );
        f.circle(
            f.width * 0.5,
            floor_y - 1.0,
            7.0,
            gui::Color::rgba(1.0, 0.75, 0.35, 0.5),
        );

        for p in &fountain.particles {
            let t = (p.age / p.life).clamp(0.0, 1.0);
            let hue = (1.0 - p.y / f.height).clamp(0.0, 1.0) * 0.85;
            let c = gui::Color::hsv(hue, 0.85, 1.0);
            // 6..3 units: never below 1.5 device pixels across on a
            // non-Retina display, where the old 1.4-unit embers disappeared.
            let size = 6.0 - 3.0 * t;
            let a = (1.0 - t) * (1.0 - t);
            f.rect(
                p.x - size * 0.5,
                p.y - size * 0.5,
                size,
                size,
                gui::Color::rgba(c.r, c.g, c.b, a),
            );
        }

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "040 - PARTICLE FOUNTAIN",
            gui::Color::WHITE,
        );
        f.text(
            20.0,
            26.0,
            2.5,
            &format!("ALIVE: {}", fountain.particles.len()),
            gui::Color::GRAY,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "6 PARTICLES PER FRAME - CAP 1200 - HUE BY HEIGHT - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
