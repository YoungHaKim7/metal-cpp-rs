//! 036 — Flow Field: 500 particles drift along the angle field
//! sin(x*0.012) + cos(y*0.012) with a slow time drift, wrapping at the
//! edges; each particle draws a 2px head rect plus a short 6-point trail;
//! R reseeds the swarm.

#[path = "gui/mod.rs"]
mod gui;

const W: f64 = 940.0;
const H: f64 = 620.0;
const N: usize = 500;
const TRAIL: usize = 6;
const SPEED: f32 = 80.0;

struct Particle {
    x: f32,
    y: f32,
    trail: Vec<(f32, f32)>,
    hue: f32,
}

struct Field {
    rng: gui::Rng,
    particles: Vec<Particle>,
}

impl Field {
    fn new() -> Field {
        Field {
            rng: gui::Rng::from_entropy(),
            particles: Vec::new(),
        }
    }

    fn reseed(&mut self, w: f32, h: f32) {
        self.particles.clear();
        for i in 0..N {
            self.particles.push(Particle {
                x: self.rng.f32() * w,
                y: self.rng.f32() * h,
                trail: Vec::with_capacity(TRAIL),
                hue: i as f32 / N as f32,
            });
        }
    }

    fn update(&mut self, f: &gui::Frame) {
        let drift = (f.time * 0.1).sin() * 0.8;
        for p in &mut self.particles {
            let a = (p.x * 0.012).sin() + (p.y * 0.012).cos() + drift;
            p.x += a.cos() * SPEED * f.dt;
            p.y += a.sin() * SPEED * f.dt;
            let mut wrapped = false;
            if p.x < 0.0 {
                p.x += f.width;
                wrapped = true;
            } else if p.x >= f.width {
                p.x -= f.width;
                wrapped = true;
            }
            if p.y < 0.0 {
                p.y += f.height;
                wrapped = true;
            } else if p.y >= f.height {
                p.y -= f.height;
                wrapped = true;
            }
            if wrapped {
                p.trail.clear();
            }
            p.trail.push((p.x, p.y));
            if p.trail.len() > TRAIL {
                p.trail.remove(0);
            }
        }
    }
}

fn main() {
    let mut field = Field::new();
    let app = gui::App::new("036 - Flow Field", W, H);
    app.run(move |f| {
        f.set_clear(gui::Color::rgb(0.03, 0.03, 0.06));
        if field.particles.is_empty() || f.input.key_pressed(gui::key::R) {
            field.reseed(f.width, f.height);
        }
        field.update(f);

        for p in &field.particles {
            let color = gui::Color::hsv(p.hue * 0.85 + 0.5, 0.7, 1.0);
            let n = p.trail.len();
            for (j, (tx, ty)) in p.trail.iter().enumerate() {
                let alpha = 0.45 * (j + 1) as f32 / n as f32;
                f.rect(
                    tx - 0.75,
                    ty - 0.75,
                    1.5,
                    1.5,
                    gui::Color::rgba(color.r, color.g, color.b, alpha),
                );
            }
            f.rect(p.x - 1.0, p.y - 1.0, 2.0, 2.0, color);
        }

        f.text(24.0, 20.0, 4.0, "036 - FLOW FIELD", gui::Color::WHITE);
        f.text(
            24.0,
            54.0,
            3.0,
            "ANGLE = SIN(X*0.012) + COS(Y*0.012) + DRIFT",
            gui::Color::CYAN,
        );
        f.text(
            24.0,
            f.height - 32.0,
            2.0,
            "500 PARTICLES WRAP AT EDGES - 6-POINT TRAILS - R RESEEDS - ESC QUITS",
            gui::Color::GRAY,
        );
    });
}
