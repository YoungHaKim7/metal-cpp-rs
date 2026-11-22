//! 045 — Asteroids Field: thirty irregular polygons (5-8 vertices with
//! random radii) drift and tumble with wrap-around edges, drawn stroke-only
//! like a classic vector arcade. SPACE fires a brief boost that multiplies
//! every speed for a moment; R reseeds the whole field.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 860.0;
const H: f64 = 560.0;
const ROCKS: usize = 30;
const STARS: usize = 90;
const BOOST_TIME: f32 = 0.9;
const BOOST_MULT: f32 = 2.4;

struct Rock {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    rot: f32,
    spin: f32,
    size: f32,
    shape: Vec<(f32, f32)>,
    col: gui::Color,
}

struct Field {
    rocks: Vec<Rock>,
    stars: Vec<(f32, f32)>,
    boost: f32,
    rng: gui::Rng,
}

impl Field {
    fn new() -> Field {
        Field {
            rocks: Vec::new(),
            stars: Vec::new(),
            boost: 0.0,
            rng: gui::Rng::from_entropy(),
        }
    }

    fn seed(&mut self, w: f32, h: f32) {
        self.rocks.clear();
        for _ in 0..ROCKS {
            let n = self.rng.range(5, 8) as usize;
            let size = 18.0 + self.rng.f32() * 38.0;
            let mut shape = Vec::with_capacity(n);
            for k in 0..n {
                let a = k as f32 / n as f32 * std::f32::consts::TAU + (self.rng.f32() - 0.5) * 0.5;
                let r = size * (0.55 + self.rng.f32() * 0.45);
                shape.push((a.cos() * r, a.sin() * r));
            }
            let dir = self.rng.f32() * std::f32::consts::TAU;
            let spd = 14.0 + self.rng.f32() * 42.0;
            self.rocks.push(Rock {
                x: self.rng.f32() * w,
                y: self.rng.f32() * h,
                vx: dir.cos() * spd,
                vy: dir.sin() * spd,
                rot: self.rng.f32() * std::f32::consts::TAU,
                spin: (self.rng.f32() - 0.5) * 1.4,
                size,
                shape,
                col: gui::Color::lerp(
                    gui::Color::rgb(0.70, 0.76, 0.86),
                    gui::Color::rgb(0.45, 0.85, 1.0),
                    self.rng.f32() * 0.6,
                ),
            });
        }
        self.stars.clear();
        for _ in 0..STARS {
            self.stars.push((self.rng.f32() * w, self.rng.f32() * h));
        }
    }
}

fn main() {
    let mut field = Field::new();
    let app = gui::App::new("045 - Asteroids Field", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.02, 0.03, 0.06));

        if field.rocks.is_empty() || f.input.key_pressed(gui::key::R) {
            field.seed(f.width, f.height);
        }

        // Boost pulse on SPACE.
        if f.input.key_pressed(gui::key::SPACE) {
            field.boost = BOOST_TIME;
        }
        field.boost = (field.boost - f.dt).max(0.0);
        let b = field.boost / BOOST_TIME;
        let mult = 1.0 + BOOST_MULT * b * b;
        let spin_mult = 1.0 + 1.2 * b * b;

        // Drift + tumble + wrap.
        for r in &mut field.rocks {
            r.x += r.vx * mult * f.dt;
            r.y += r.vy * mult * f.dt;
            r.rot += r.spin * spin_mult * f.dt;
            let m = r.size + 10.0;
            if r.x < -m {
                r.x += f.width + 2.0 * m;
            }
            if r.x > f.width + m {
                r.x -= f.width + 2.0 * m;
            }
            if r.y < -m {
                r.y += f.height + 2.0 * m;
            }
            if r.y > f.height + m {
                r.y -= f.height + 2.0 * m;
            }
        }

        // Backdrop stars.
        for (x, y) in &field.stars {
            f.rect(*x, *y, 1.6, 1.6, gui::Color::rgba(0.75, 0.80, 0.95, 0.4));
        }

        // Stroke-only polygons.
        for r in &field.rocks {
            let (s, c) = (r.rot.sin(), r.rot.cos());
            let n = r.shape.len();
            for k in 0..n {
                let (ax, ay) = r.shape[k];
                let (bx, by) = r.shape[(k + 1) % n];
                let x0 = r.x + ax * c - ay * s;
                let y0 = r.y + ax * s + ay * c;
                let x1 = r.x + bx * c - by * s;
                let y1 = r.y + bx * s + by * c;
                f.line(x0, y0, x1, y1, 2.0, r.col);
            }
        }

        // Boost meter.
        if b > 0.01 {
            f.frame_rect(f.width - 100.0, 24.0, 80.0, 10.0, 1.5, gui::Color::GRAY);
            f.rect(f.width - 98.0, 26.0, 76.0 * b, 6.0, gui::Color::CYAN);
        }

        f.text_center(
            f.width * 0.5,
            20.0,
            4.0,
            "045 - ASTEROIDS FIELD",
            gui::Color::WHITE,
        );
        f.text_center(
            f.width * 0.5,
            f.height - 40.0,
            2.0,
            "30 TUMBLING POLYGONS - SPACE BOOST - R RESEED - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
